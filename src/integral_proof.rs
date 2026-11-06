use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    ToTokens,
    format_ident,
    quote,
};
use syn::Path;
use syn::parse::{
    Parse,
    ParseStream,
};

use crate::integral::Maker;
use crate::integral_cfg::IntegralFlag;
use crate::integral_forward::CoreRhs;
use crate::zz_domain_closure::{
    Closure,
    ClosureOp,
    Policy,
    Rhs,
};
use crate::zz_flags::Flag;
use crate::zz_friend::Friend;
use crate::zz_lib::ValidationCfg;
use crate::zz_profile_integral::Arithmetic;
use crate::zz_runner::MkErr;
use crate::zz_values::{
    ArithmeticOp,
    FnName,
    Num,
    Operator,
};

impl Maker {
    /// `!x` of a valid value is always valid.
    pub(crate) fn not_proven(&self) -> bool {
        let (op, policy, rhs) = ProveOp::Not.closure(self.arithmetic_policy());
        return self.proven(op, rhs.closure(), policy);
    }

    /// `-x` of a valid value is always valid: by the domain's symmetry, or
    /// by a closure proof under the profile's arithmetic.
    pub(crate) fn neg_proven(&self) -> bool {
        let (op, policy, _) = ProveOp::Neg.closure(self.arithmetic_policy());
        return self.validated_closed(
            self.ctx.cfg.validation.neg_closed(),
            op,
            policy,
        );
    }

    /// A unary `op` of a valid value is valid, by the domain's shape
    /// (`structural`) or by a closure proof, and validation that a proof
    /// may skip is configured: a domain and the generated `_or_panic`.
    pub(crate) fn validated_closed(
        &self,
        structural: bool,
        op: ClosureOp,
        policy: Policy,
    ) -> bool {
        let cfg = &self.ctx.cfg;
        return cfg.validation.has_validation()
            && cfg.flags.has(IntegralFlag::FnOrPanic)
            && (structural || self.proven(op, Rhs::Unary, policy));
    }

    /// Construction from `it` when validation is configured, the generated
    /// `_or_panic` is in use, and `closed` proves the result valid: through
    /// `_proven` for integers, directly for floats.
    pub(crate) fn closed_result(
        &self,
        closed: bool,
    ) -> Option<TokenStream> {
        if !closed
            || !self.ctx.cfg.validation.has_validation()
            || !self.ctx.cfg.flags.has(IntegralFlag::FnOrPanic)
        {
            return None;
        }
        if self.proofs_available() {
            return Some(quote! { Self::_proven(it) });
        }
        return Some(self.construct_self(quote! { it }));
    }

    /// `closed_result`, else `it` validated by `_result_or_panic`.
    pub(crate) fn closed_or_validated(
        &self,
        closed: bool,
    ) -> TokenStream {
        return self
            .closed_result(closed)
            .unwrap_or_else(|| quote! { Self::_result_or_panic(it) });
    }

    /// Integer wrappers validated by their domain alone can construct
    /// results that a closure proof (`zz_domain_closure`) shows valid
    /// through `_proven`. Callbacks are opaque to proofs, and operations
    /// that report failure validate every result instead.
    pub(crate) fn proofs_available(&self) -> bool {
        let cfg = &self.ctx.cfg;
        return matches!(self.num, Num::Int(_))
            && cfg.flags.has(IntegralFlag::FnOrPanic)
            && cfg.validation.has_validation()
            && cfg.validation.callbacks.is_empty()
            && !cfg.panic_at.fallible();
    }

    /// Every result of `op` on valid operands is valid at every width, by a
    /// closure proof the `proven` flag enables.
    pub(crate) fn proven(
        &self,
        op: ClosureOp,
        rhs: Rhs,
        policy: Policy,
    ) -> bool {
        return self.proofs_available()
            && self.ctx.cfg.flags.has(IntegralFlag::Proven)
            && self.ctx.cfg.validation.resolved().is_some_and(|it| {
                return it.closure(op, rhs, policy) == Closure::Proven;
            });
    }

    /// The overflow behavior of an arithmetic policy. A primitive operator
    /// panics (debug) or wraps (release); the wrapped results cover both.
    fn closure_policy(arithmetic: Arithmetic) -> Policy {
        return match arithmetic {
            Arithmetic::Checked => Policy::Checked,
            Arithmetic::Primitive
            | Arithmetic::PrimitiveWrapping
            | Arithmetic::WrapperWrapping => Policy::Wrapping,
        };
    }

    /// `closure_policy` of the root profile's arithmetic.
    fn arithmetic_policy(&self) -> Policy {
        return Self::closure_policy(self.ctx.cfg.profile.arithmetic());
    }

    /// The proof of a binary operator under the root profile, for a raw or
    /// friend right-hand side and for a same-type one.
    pub(crate) fn operator_proof(
        &self,
        operator: Operator,
    ) -> OperatorProof {
        return self
            .operator_proof_with(operator, self.ctx.cfg.profile.arithmetic());
    }

    /// `operator_proof` under a given arithmetic (a facet's).
    pub(crate) fn operator_proof_with(
        &self,
        operator: Operator,
        arithmetic: Arithmetic,
    ) -> OperatorProof {
        let (op, policy, _) =
            ProveOp::Binary(operator).closure(Self::closure_policy(arithmetic));
        let full = self.proven(op, Rhs::Full, policy);
        let same = full || self.proven(op, Rhs::SameDomain, policy);
        return OperatorProof { full, same };
    }

    /// `operator_proof` for an arithmetic helper; shifts take an amount, and
    /// left shifts are never proven.
    pub(crate) fn arithmetic_proof(
        &self,
        operation: ArithmeticOp,
    ) -> OperatorProof {
        if let Some(operator) = operation.operator() {
            return self.operator_proof(operator);
        }
        return match operation {
            ArithmeticOp::Shr => {
                let (op, policy, rhs) =
                    ProveOp::Shr.closure(self.arithmetic_policy());
                let full = self.proven(op, rhs.closure(), policy);
                OperatorProof { full, same: full }
            }
            _ => OperatorProof::default(),
        };
    }

    /// The constructor of a raw result: `Self::_proven` when `proven`,
    /// else the validating `fallback`.
    pub(crate) fn proven_constructor(
        &self,
        proven: bool,
        fallback: TokenStream,
    ) -> TokenStream {
        return match proven {
            true => quote! { Self::_proven },
            false => fallback,
        };
    }

    /// The helper a same-type operand calls: `{helper}_proven` when only the
    /// same-type form is proven.
    pub(crate) fn same_type_helper(
        proof: OperatorProof,
        helper: &Ident,
    ) -> Ident {
        return match proof.same && !proof.full {
            true => format_ident!("{}_proven", helper),
            false => helper.clone(),
        };
    }

    /// The doc line of a generated item whose result cannot fail validation.
    pub(crate) fn proof_doc(
        &self,
        proven: bool,
    ) -> TokenStream {
        if !proven {
            return TokenStream::new();
        }
        let doc = format!(
            " Cannot fail validation: every result lies in the domain `{}` \
            at every pointer width (proven by typekin at expansion).",
            self.ctx.cfg.validation.domain_text(),
        );
        return quote! { #[doc = #doc] };
    }

    /// Whether a `core_int` method's result is proven; `None` operations
    /// (shifts left, powers) are never proven.
    pub(crate) fn core_int_proven(
        &self,
        op: Option<ClosureOp>,
        rhs: CoreRhs,
        policy: Policy,
    ) -> bool {
        return op.is_some_and(|op| self.proven(op, rhs.closure(), policy));
    }

    /// `prove = [...]`: every listed operation must be proven closed, or
    /// expansion fails with a counterexample.
    pub(crate) fn check_prove(&self) -> syn::Result<()> {
        for item in &self.ctx.cfg.prove {
            self.check_prove_item(item)?;
        }
        return Ok(());
    }

    /// One `prove` entry: the operation, its right-hand side, the
    /// prerequisites, then the structural shortcut and the closure proof.
    fn check_prove_item(
        &self,
        item: &ProveItem,
    ) -> syn::Result<()> {
        let name = &item.name;
        if !matches!(self.num, Num::Int(_)) {
            return name.fail("closure proofs support integer wrappers only");
        }
        let Some(target) = self.prove_target(name)
        else {
            return name
                .fail(format!("unknown operation `{name}` for `prove`"));
        };
        if !target.enabled {
            return name.fail(format!(
                "`prove` lists `{name}`, which is not generated"
            ));
        }
        let (rhs, rhs_text) = self.prove_rhs(item, &target)?;
        let expression =
            prove_expression(&self.target.ident, name, &target, &rhs_text);
        self.check_prove_prerequisites(item)?;

        let validation = &self.ctx.cfg.validation;
        if target.structural.is_some_and(|it| it.closed(validation)) {
            return Ok(());
        }
        if !validation.callbacks.is_empty() {
            return name.fail(
                "`prove` needs an interval domain; `valid` callbacks are \
                opaque to closure proofs",
            );
        }
        let Some(resolved) = validation.resolved()
        else {
            unreachable!("an integer domain is resolved before `prove`");
        };
        let domain = validation.domain_text();
        let closure = resolved.closure(target.op, rhs, target.policy);
        return closure_failure(
            item,
            &expression,
            target.symbol,
            &domain,
            closure,
        );
    }

    /// The question `prove` asks for `name`: an operator, whose flag
    /// generates it, or a proven `core_int` method.
    fn prove_target(
        &self,
        name: &Ident,
    ) -> Option<ProveTarget> {
        let text = name.to_string();
        let operator = IntegralFlag::of(&text).and_then(|flag| {
            return PROVE_OPERATORS.iter().find(|(it, _, _)| *it == flag);
        });
        if let Some(&(flag, operator, symbol)) = operator {
            let (op, policy, rhs) = operator.closure(self.arithmetic_policy());
            return Some(ProveTarget {
                op,
                policy,
                rhs,
                enabled: self.ctx.cfg.flags.has(flag),
                symbol: Some(symbol),
                structural: matches!(operator, ProveOp::Neg)
                    .then_some(Structural::Neg),
            });
        }

        let function = FnName::of(&text)?;
        let forward = self.core_forward(function)?;
        let (op, policy) = forward.proof?;
        return Some(ProveTarget {
            op,
            policy,
            rhs: forward.rhs(),
            enabled: self.forward_generated(function),
            symbol: None,
            structural: (function == FnName::Abs).then_some(Structural::Abs),
        });
    }

    /// The right-hand side `prove` checks and its text: the wrapper itself,
    /// or the type an operator entry names (a primitive integer or a
    /// friend).
    fn prove_rhs(
        &self,
        item: &ProveItem,
        target: &ProveTarget,
    ) -> syn::Result<(Rhs, String)> {
        let ty = &self.target.ident;
        let Some(path) = &item.rhs
        else {
            return Ok((target.rhs.closure(), ty.to_string()));
        };
        if !matches!(target.rhs, CoreRhs::Same) || target.symbol.is_none() {
            return path.fail(format!(
                "`{}` takes no right-hand side type in `prove`",
                item.name,
            ));
        }
        let primitive = path
            .get_ident()
            .and_then(|it| Num::of(it.to_string()))
            .is_some_and(|it| matches!(it, Num::Int(_)));
        if !primitive && !self.ctx.cfg.friends.contains(&Friend::from(path)) {
            return path.fail(format!(
                "`{}` is not a primitive integer or a friend of `{ty}`",
                path.to_token_stream(),
            ));
        }
        return Ok((Rhs::Full, path.to_token_stream().to_string()));
    }

    /// A proof skips validation through the generated `_or_panic`, so
    /// `prove` needs both a domain and that function.
    fn check_prove_prerequisites(
        &self,
        item: &ProveItem,
    ) -> syn::Result<()> {
        if !self.ctx.cfg.validation.has_validation() {
            return item.name.fail("`prove` needs a domain");
        }
        if !self.ctx.cfg.flags.has(IntegralFlag::FnOrPanic) {
            return item.name.fail(
                "`prove` needs the generated `_or_panic`; remove \
                `without = fn_or_panic`",
            );
        }
        return Ok(());
    }
}

/// An operator `prove` accepts.
#[derive(Clone, Copy)]
enum ProveOp {
    Binary(Operator),
    Shr,
    Neg,
    Not,
}

impl ProveOp {
    /// The closure question of the operator under the profile's
    /// `arithmetic` policy, and its right-hand side. Shared by the
    /// emitters' proofs and `prove`, so both ask the same question.
    fn closure(
        self,
        arithmetic: Policy,
    ) -> (ClosureOp, Policy, CoreRhs) {
        return match self {
            Self::Binary(operator) => {
                let (op, policy) = match operator {
                    Operator::Add => (ClosureOp::Add, arithmetic),
                    Operator::Sub => (ClosureOp::Sub, arithmetic),
                    Operator::Mul => (ClosureOp::Mul, arithmetic),
                    Operator::Div => (ClosureOp::Div, arithmetic),
                    Operator::Rem => (ClosureOp::Rem, arithmetic),
                    Operator::BitAnd => (ClosureOp::BitAnd, Policy::Wrapping),
                    Operator::BitOr => (ClosureOp::BitOr, Policy::Wrapping),
                    Operator::BitXor => (ClosureOp::BitXor, Policy::Wrapping),
                };
                (op, policy, CoreRhs::Same)
            }
            Self::Shr => (ClosureOp::Shr, arithmetic, CoreRhs::Amount("rhs")),
            Self::Neg => (ClosureOp::Neg, arithmetic, CoreRhs::Unary),
            Self::Not => (ClosureOp::Not, Policy::Wrapping, CoreRhs::Unary),
        };
    }
}

/// The flag that generates each `prove` operator, and its symbol.
const PROVE_OPERATORS: [(IntegralFlag, ProveOp, &str); 11] = [
    (IntegralFlag::MathAdd, ProveOp::Binary(Operator::Add), "+"),
    (IntegralFlag::MathSub, ProveOp::Binary(Operator::Sub), "-"),
    (IntegralFlag::MathMul, ProveOp::Binary(Operator::Mul), "*"),
    (IntegralFlag::MathDiv, ProveOp::Binary(Operator::Div), "/"),
    (IntegralFlag::MathRem, ProveOp::Binary(Operator::Rem), "%"),
    (
        IntegralFlag::MathAnd,
        ProveOp::Binary(Operator::BitAnd),
        "&",
    ),
    (IntegralFlag::MathOr, ProveOp::Binary(Operator::BitOr), "|"),
    (
        IntegralFlag::MathXor,
        ProveOp::Binary(Operator::BitXor),
        "^",
    ),
    (IntegralFlag::MathShr, ProveOp::Shr, ">>"),
    (IntegralFlag::MathNeg, ProveOp::Neg, "-"),
    (IntegralFlag::MathNot, ProveOp::Not, "!"),
];

/// A domain shape that makes an operation closed without a closure proof.
#[derive(Clone, Copy)]
enum Structural {
    /// The `-` operator on a symmetric domain.
    Neg,
    /// Plain `abs`.
    Abs,
}

impl Structural {
    fn closed(
        self,
        validation: &ValidationCfg,
    ) -> bool {
        return match self {
            Self::Neg => validation.neg_closed(),
            Self::Abs => validation.abs_closed(),
        };
    }
}

/// What `prove` checks for one entry.
struct ProveTarget {
    op: ClosureOp,
    policy: Policy,
    rhs: CoreRhs,
    /// The configuration generates the operation.
    enabled: bool,
    /// Operators only.
    symbol: Option<&'static str>,
    structural: Option<Structural>,
}

/// The operation as `prove` reports it: `-T`, `T >> usize`, `T + rhs` or
/// `T::name`.
fn prove_expression(
    ty: &Ident,
    name: &Ident,
    target: &ProveTarget,
    rhs: &str,
) -> String {
    return match (target.symbol, target.rhs) {
        (Some(symbol), CoreRhs::Unary) => format!("{symbol}{ty}"),
        (Some(symbol), CoreRhs::Amount(_)) => format!("{ty} {symbol} usize"),
        (Some(symbol), CoreRhs::Same) => format!("{ty} {symbol} {rhs}"),
        (None, _) => format!("{ty}::{name}"),
    };
}

/// `Ok` for a proven `closure`, else the reason `expression` is not proven
/// closed over `domain`.
fn closure_failure(
    item: &ProveItem,
    expression: &str,
    symbol: Option<&str>,
    domain: &str,
    closure: Closure,
) -> syn::Result<()> {
    let name = &item.name;
    return match closure {
        Closure::Proven => Ok(()),
        Closure::Ineligible(atom) => name.fail(format!(
            "`prove` needs an interval domain; `{atom}` is opaque to \
            closure proofs"
        )),
        Closure::Refuted {
            lhs,
            rhs: value,
            result,
            width,
            all_widths,
        } => {
            let example = match (symbol, value) {
                (Some(symbol), None) => format!("{symbol}{lhs}"),
                (Some(symbol), Some(value)) => {
                    format!("{lhs} {symbol} {value}")
                }
                (None, None) => format!("{name}({lhs})"),
                (None, Some(value)) => format!("{name}({lhs}, {value})"),
            };
            let at = match all_widths {
                true => String::new(),
                false => format!(" at {width}-bit pointer width"),
            };
            name.fail(format!(
                "`{expression}` is not closed: {example} = {result} is \
                outside `{domain}`{at}"
            ))
        }
        Closure::Unknown { lo, hi, width: _ } => name.fail(format!(
            "cannot prove `{expression}`: results may reach {lo}..={hi}, \
            outside `{domain}`"
        )),
    };
}

/// A closure proof of one binary operator: `full` for a raw or friend
/// right-hand side (which implies `same`), `same` for a same-type one.
#[derive(Clone, Copy, Default)]
pub(crate) struct OperatorProof {
    pub(crate) full: bool,
    pub(crate) same: bool,
}

/// One `prove` entry: `math_div` (same-type right-hand side), `math_div(u8)`
/// (a raw or friend right-hand side), or a `core_int` method name.
pub(crate) struct ProveItem {
    pub(crate) name: Ident,
    pub(crate) rhs: Option<Path>,
}

impl Parse for ProveItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let rhs = match input.peek(syn::token::Paren) {
            true => {
                let content;
                syn::parenthesized!(content in input);
                Some(content.parse::<Path>()?)
            }
            false => None,
        };
        return Ok(Self { name, rhs });
    }
}
