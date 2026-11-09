//! Integer domains: the untyped zero probe, typing, per-width folding,
//! normalization, exact evaluation, structural closure, and emission. The
//! grammar and its parser live in `zz_domain`.

use std::collections::BTreeMap;

use proc_macro2::{
    Ident,
    Span,
    TokenStream,
};
use quote::quote;

use crate::zz_domain::{
    Atom,
    Const,
    ConstKind,
    Domain,
    Lens,
    MAX,
    MIN,
    Node,
    OperandKey,
    Test,
    Tri,
    collapse,
    group_in_order,
    kleene,
    listing,
};
use crate::zz_forward::{
    CORE_INT,
    CoreFwd,
    InDomain,
    domain_functions,
};
use crate::zz_lib::Named as _;
use crate::zz_runner::MkErr;
use crate::zz_values::{
    FnName,
    Int,
    IntTy,
    N,
    Width,
    by_width,
};

impl Domain {
    /// Whether the domain admits zero, before the value type is known.
    /// `Unknown` when the answer depends on the type (`count_zeros == 8`).
    pub(crate) fn admits_zero_untyped(&self) -> Tri {
        return Tri::from(Self::probe_node(&self.root));
    }

    fn probe_node(node: &Node) -> Option<bool> {
        return match node {
            Node::Or(items) => kleene(items.iter().map(Self::probe_node), true),
            Node::And(items) => {
                kleene(items.iter().map(Self::probe_node), false)
            }
            Node::Not(inner) => Self::probe_node(inner).map(|it| !it),
            Node::Atom(atom) => Self::probe_atom(atom),
        };
    }

    fn probe_atom(atom: &Atom) -> Option<bool> {
        let values = Width::ALL.map(|width| Self::probe_atom_at(atom, width));
        if values.iter().all(|it| *it == Some(true)) {
            return Some(true);
        }
        if values.iter().all(|it| *it == Some(false)) {
            return Some(false);
        }
        return None;
    }

    fn probe_atom_at(
        atom: &Atom,
        width: Width,
    ) -> Option<bool> {
        let operand = match &atom.lens {
            None => Int::NonNegative(0),
            Some(lens) => lens_at_zero_untyped(lens, width)?,
        };
        let value = |it: &Const| const_value_untyped(it, width);
        return match &atom.test {
            Test::Predicate => predicate_at_zero(atom.lens.as_ref()?.function),
            Test::Point {
                value: point,
                negated,
            } => Some((operand == value(point)?) != *negated),
            Test::Divisible { negated, .. } => Some(!*negated),
            Test::Range { lo, hi, inclusive } => {
                let above = match lo {
                    None => true,
                    Some(lo) => operand >= value(lo)?,
                };
                let below = match hi {
                    None => true,
                    Some(hi) if *inclusive => operand <= value(hi)?,
                    Some(hi) => operand < value(hi)?,
                };
                Some(above && below)
            }
        };
    }
}

fn const_value_untyped(
    constant: &Const,
    width: Width,
) -> Option<Int> {
    return match &constant.kind {
        ConstKind::Int { value, .. } => Some(*value),
        ConstKind::Bound { ty, max, negated } => {
            let (min, maximum) = ty.bounds_for_pointer_width(width);
            let value = if *max { maximum } else { min };
            Some(if *negated { value.negate() } else { value })
        }
        ConstKind::Float { .. } | ConstKind::FloatPath { .. } => None,
    };
}

fn predicate_at_zero(function: Option<FnName>) -> Option<bool> {
    return match function {
        Some(
            FnName::IsPowerOfTwo | FnName::IsPositive | FnName::IsNegative,
        ) => Some(false),
        _ => None,
    };
}

/// A lens applied to zero without knowing the operand type: exact except for
/// the functions whose result at zero is the type's bit width.
fn lens_at_zero_untyped(
    lens: &Lens,
    width: Width,
) -> Option<Int> {
    return match lens.function? {
        FnName::Abs
        | FnName::UnsignedAbs
        | FnName::Signum
        | FnName::CountOnes
        | FnName::LeadingOnes
        | FnName::TrailingOnes
        | FnName::RemEuclid
        | FnName::DivEuclid
        | FnName::Isqrt => Some(Int::NonNegative(0)),
        FnName::AbsDiff => {
            let arg = const_value_untyped(lens.args.first()?, width)?;
            Some(match arg {
                Int::Negative(value) | Int::NonNegative(value) => {
                    Int::NonNegative(value)
                }
            })
        }
        _ => None,
    };
}

// =============================================================================
// Typing

/// What an atom compares: the value itself or one lens application.
#[derive(Clone)]
pub(crate) struct Operand {
    /// `None` for the value; otherwise the method call suffix to emit.
    call: Option<TokenStream>,
    /// Grouping key for normalization: identical lens applications.
    pub(crate) key: OperandKey,
    ty: N,
    /// The value type the lens applies to; `ty` for the value itself.
    subject: N,
    lens: Option<LensKind>,
    /// Lens arguments per width (validated to fit).
    args: [Vec<Int>; Width::COUNT],
}

impl Operand {
    /// The value itself, not a lens application.
    pub(crate) fn is_subject(&self) -> bool {
        return self.key == OperandKey::Subject;
    }

    /// The lens applied to the value; `None` for the value itself.
    pub(crate) fn lens(&self) -> Option<LensKind> {
        return self.lens;
    }

    /// The lens argument at `width`: the `a` of `abs_diff(a)`, the divisor
    /// of `rem_euclid` and `div_euclid`.
    pub(crate) fn arg(
        &self,
        width: Width,
    ) -> Int {
        return self.args[width.index()][0];
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LensKind {
    UnsignedAbs,
    AbsDiff,
    Signum,
    CountOnes,
    CountZeros,
    LeadingZeros,
    TrailingZeros,
    LeadingOnes,
    TrailingOnes,
    RemEuclid,
    DivEuclid,
    Isqrt,
}

impl LensKind {
    /// The primitive method the lens calls.
    pub(crate) fn function(self) -> FnName {
        return match self {
            Self::UnsignedAbs => FnName::UnsignedAbs,
            Self::AbsDiff => FnName::AbsDiff,
            Self::Signum => FnName::Signum,
            Self::CountOnes => FnName::CountOnes,
            Self::CountZeros => FnName::CountZeros,
            Self::LeadingZeros => FnName::LeadingZeros,
            Self::TrailingZeros => FnName::TrailingZeros,
            Self::LeadingOnes => FnName::LeadingOnes,
            Self::TrailingOnes => FnName::TrailingOnes,
            Self::RemEuclid => FnName::RemEuclid,
            Self::DivEuclid => FnName::DivEuclid,
            Self::Isqrt => FnName::Isqrt,
        };
    }

    /// `subject.unsigned()`, `N::U032`, or `subject`.
    fn result(
        self,
        subject: N,
    ) -> N {
        return match self {
            Self::UnsignedAbs | Self::AbsDiff => subject.unsigned(),
            Self::CountOnes
            | Self::CountZeros
            | Self::LeadingZeros
            | Self::TrailingZeros
            | Self::LeadingOnes
            | Self::TrailingOnes => N::U032,
            Self::Signum | Self::RemEuclid | Self::DivEuclid | Self::Isqrt => {
                subject
            }
        };
    }
}

/// The `core_int` row `lens` names and the lens it reads; `abs` and
/// `unsigned_abs` both read `UnsignedAbs`.
fn lens_row(lens: &Lens) -> Option<(&'static CoreFwd, LensKind)> {
    let row = CoreFwd::of(CORE_INT, lens.function?)?;
    return match row.lens {
        InDomain::Int(kind) => Some((row, kind)),
        _ => None,
    };
}

/// "unknown domain function …", listing every integer function.
fn unknown_function(
    lens: &Lens,
    subject: N,
) -> syn::Error {
    return lens.name.errorful(format!(
        "unknown domain function {}; available for {}: {}, {}",
        lens.name,
        subject.name(),
        listing(&domain_functions(CORE_INT, false)),
        listing(&domain_functions(CORE_INT, true))
    ));
}

fn type_lens(
    lens: &Lens,
    subject: N,
) -> syn::Result<Operand> {
    let Some((row, kind)) = lens_row(lens)
    else {
        return Err(unknown_function(lens, subject));
    };
    check_sign(lens, row.name, subject)?;

    let arity = row.args.len();
    if lens.args.len() != arity {
        return lens.name.fail(format!(
            "{} takes {arity} argument{}",
            lens.name,
            if arity == 1 { "" } else { "s" }
        ));
    }

    let args = type_lens_args(lens, subject)?;
    if matches!(kind, LensKind::RemEuclid | LensKind::DivEuclid) {
        check_divisor(&lens.args[0], args[0][0])?;
    }

    let method = Ident::new(kind.function().name(), lens.name.span());
    let arg_tokens = lens.args.iter().map(|it| {
        let tokens = &it.tokens;
        return quote! { (#tokens) };
    });
    let call = quote! { .#method(#(#arg_tokens),*) };

    return Ok(Operand {
        call: Some(call),
        key: OperandKey::of(kind.function(), &lens.args),
        ty: kind.result(subject),
        subject,
        lens: Some(kind),
        args,
    });
}

/// The sign rule, by written name: `abs` on unsigned is "the identity",
/// `unsigned_abs`/`signum` need signed, `isqrt` unsigned.
fn check_sign(
    lens: &Lens,
    function: FnName,
    subject: N,
) -> syn::Result<()> {
    let name = &lens.name;
    let signed = subject.is_signed();
    return match function {
        FnName::Abs if !signed => name.fail(format!(
            "abs on {} is the identity; remove it",
            subject.name()
        )),
        FnName::UnsignedAbs | FnName::Signum if !signed => {
            name.fail(format!("{name} requires a signed operand"))
        }
        FnName::Isqrt if signed => {
            name.fail(format!("{name} requires an unsigned operand"))
        }
        _ => Ok(()),
    };
}

/// The lens arguments at each pointer width, checked to fit `subject`.
fn type_lens_args(
    lens: &Lens,
    subject: N,
) -> syn::Result<[Vec<Int>; Width::COUNT]> {
    let mut args: [Vec<Int>; Width::COUNT] = Default::default();
    for arg in &lens.args {
        let values = type_const(arg, subject, None)?;
        for (width, value) in Width::ALL.into_iter().zip(values) {
            let (min, max) = subject.bounds_for_pointer_width(width);
            if value < min || value > max {
                return arg.span.fail(format!(
                    "{} does not fit {width}-bit {}",
                    arg.tokens,
                    subject.name()
                ));
            }
            args[width.index()].push(value);
        }
    }
    return Ok(args);
}

/// The predicate `lens` names, if it applies to `subject`.
fn predicate_operand_check(
    lens: &Lens,
    subject: N,
) -> syn::Result<FnName> {
    let name = lens.name.to_string();
    if !lens.args.is_empty() {
        return lens.name.fail(format!("{name} takes no arguments"));
    }
    let row = lens.function.and_then(|it| CoreFwd::of(CORE_INT, it));
    return match row.map(|it| (it.name, it.lens)) {
        Some((FnName::IsPowerOfTwo, _)) if subject.is_signed() => lens
            .name
            .fail("is_power_of_two requires an unsigned operand"),
        Some((FnName::IsPositive | FnName::IsNegative, _))
            if !subject.is_signed() =>
        {
            lens.name.fail(format!("{name} requires a signed operand"))
        }
        Some((function, InDomain::Predicate)) => Ok(function),
        Some((_, InDomain::Int(_))) => lens.name.fail(format!(
            "{name} needs `in` and a set of its results; for example \
            {name} in ..=3"
        )),
        _ => Err(unknown_function(lens, subject)),
    };
}

/// Values of a constant at each pointer width, typed against `ty`. `lens`
/// names the function whose result `ty` is, for diagnostics.
fn type_const(
    constant: &Const,
    ty: N,
    lens: Option<&str>,
) -> syn::Result<[Int; Width::COUNT]> {
    let mismatch = |found: &str| -> syn::Error {
        let expected = ty.name();
        return match lens {
            None => constant.span.errorful(format!(
                "{found} does not match the {expected} operand; write \
                {expected}::MAX or a literal"
            )),
            Some(lens) => constant.span.errorful(format!(
                "{lens} yields {expected} here; write {expected}::MAX or a \
                literal"
            )),
        };
    };

    return match &constant.kind {
        ConstKind::Int { value, suffix } => {
            if !suffix.is_empty() && suffix != ty.name() {
                return Err(constant.span.errorful(format!(
                    "{suffix} suffix does not match the {} operand",
                    ty.name()
                )));
            }
            Ok([*value; Width::COUNT])
        }
        ConstKind::Bound {
            ty: bound,
            max,
            negated,
        } => {
            if *bound != ty {
                return Err(mismatch(&format!(
                    "{}::{}",
                    bound.name(),
                    if *max { MAX } else { MIN }
                )));
            }
            Ok(Width::ALL.map(|width| {
                let (min, maximum) = bound.bounds_for_pointer_width(width);
                let value = if *max { maximum } else { min };
                if *negated { value.negate() } else { value }
            }))
        }
        ConstKind::Float { .. } | ConstKind::FloatPath { .. } => constant
            .span
            .fail("integer domains need integer literals; write 1"),
    };
}

fn check_divisor(
    divisor: &Const,
    value: Int,
) -> syn::Result<()> {
    return match value {
        Int::NonNegative(0) => divisor.span.fail("division by zero"),
        Int::NonNegative(1) => {
            divisor.span.fail("every integer is divisible by 1")
        }
        Int::Negative(_) => {
            divisor.span.fail("negative divisors are not supported")
        }
        Int::NonNegative(_) => Ok(()),
    };
}

// =============================================================================
// Resolved form (one per pointer width)

#[derive(Clone)]
pub(crate) struct Bound {
    /// Inclusive value: `> 4` is stored as 5.
    pub(crate) value: Int,
    pub(crate) tokens: TokenStream,
    /// `tokens` are emitted with `>=`/`<=`, otherwise with `>`/`<`.
    pub(crate) inclusive: bool,
}

#[derive(Clone)]
pub(crate) enum RNode {
    Const(bool),
    Or(Vec<RNode>),
    And(Vec<RNode>),
    Not(Box<RNode>),
    Interval {
        operand: Operand,
        lo: Option<Bound>,
        hi: Option<Bound>,
    },
    Ne {
        operand: Operand,
        value: Int,
        tokens: TokenStream,
    },
    Divisible {
        operand: Operand,
        divisor: Int,
        tokens: TokenStream,
        negated: bool,
    },
    Predicate {
        operand: Operand,
        name: Ident,
        function: FnName,
    },
}

/// A domain resolved for one value type: per-width normalized expressions
/// and exact zero membership.
#[derive(Clone)]
pub(crate) struct ResolvedDomain {
    pub(crate) subject: N,
    pub(crate) per_width: [RNode; Width::COUNT],
}

impl Domain {
    /// Types the domain for `subject`, optionally ANDing `!= 0`
    /// (`with = non_zero`), then folds, normalizes, and checks it.
    pub(crate) fn resolve(
        &self,
        subject: N,
        non_zero: bool,
    ) -> syn::Result<ResolvedDomain> {
        let typed = type_node(&self.root, subject)?;
        let per_width =
            Width::ALL.map(|width| normalize(fold(&typed, width), width));

        if per_width.iter().all(|it| matches!(it, RNode::Const(true))) {
            return self.span.fail(format!(
                "domain admits every {} value; remove the domain",
                subject.name()
            ));
        }
        if per_width.iter().all(|it| matches!(it, RNode::Const(false))) {
            return self
                .span
                .fail(format!("domain admits no {} value", subject.name()));
        }

        let per_width = match non_zero {
            false => per_width,
            // A second normalize over the folded tree, so N8 sees the same
            // input.
            true => {
                let mut folded = per_width.into_iter();
                Width::ALL.map(|width| {
                    let it = folded.next().expect("one tree per width");
                    return normalize(
                        RNode::And(vec![
                            it,
                            ne_zero(subject),
                        ]),
                        width,
                    );
                })
            }
        };

        return Ok(ResolvedDomain { subject, per_width });
    }

    /// `with = non_zero` without a domain: exactly `!= 0`.
    pub(crate) fn non_zero_only(subject: N) -> ResolvedDomain {
        return ResolvedDomain {
            subject,
            per_width: std::array::from_fn(|_| ne_zero(subject)),
        };
    }
}

fn subject_operand(subject: N) -> Operand {
    return Operand {
        call: None,
        key: OperandKey::Subject,
        ty: subject,
        subject,
        lens: None,
        args: Default::default(),
    };
}

fn ne_zero(subject: N) -> RNode {
    return RNode::Ne {
        operand: subject_operand(subject),
        value: Int::NonNegative(0),
        tokens: quote! { 0 },
    };
}

/// Atoms typed once, with per-width constant values.
#[allow(clippy::large_enum_variant)]
enum TNode {
    Or(Vec<TNode>),
    And(Vec<TNode>),
    Not(Box<TNode>),
    Atom(TAtom),
}

struct TAtom {
    operand: Operand,
    test: TTest,
}

enum TTest {
    Range {
        lo: Option<([Int; Width::COUNT], TokenStream)>,
        hi: Option<([Int; Width::COUNT], TokenStream)>,
        inclusive: bool,
    },
    Point {
        values: [Int; Width::COUNT],
        tokens: TokenStream,
        negated: bool,
    },
    Divisible {
        divisor: [Int; Width::COUNT],
        tokens: TokenStream,
        negated: bool,
    },
    Predicate(Ident, FnName),
}

fn type_node(
    node: &Node,
    subject: N,
) -> syn::Result<TNode> {
    return Ok(match node {
        Node::Or(items) => TNode::Or(
            items
                .iter()
                .map(|it| type_node(it, subject))
                .collect::<syn::Result<_>>()?,
        ),
        Node::And(items) => TNode::And(
            items
                .iter()
                .map(|it| type_node(it, subject))
                .collect::<syn::Result<_>>()?,
        ),
        Node::Not(inner) => TNode::Not(Box::new(type_node(inner, subject)?)),
        Node::Atom(atom) => TNode::Atom(type_atom(atom, subject)?),
    });
}

fn type_atom(
    atom: &Atom,
    subject: N,
) -> syn::Result<TAtom> {
    if let Test::Predicate = atom.test {
        let lens = atom.lens.as_ref().expect("predicates have a name");
        return type_predicate(lens, subject);
    }

    let operand = match &atom.lens {
        None => subject_operand(subject),
        Some(lens) => type_lens(lens, subject)?,
    };
    let lens_name = atom.lens.as_ref().map(|it| it.name.to_string());
    let typed =
        |it: &Const| -> syn::Result<([Int; Width::COUNT], TokenStream)> {
            return Ok((
                type_const(it, operand.ty, lens_name.as_deref())?,
                it.tokens.clone(),
            ));
        };

    let test = match &atom.test {
        Test::Predicate => unreachable!("handled above"),
        Test::Point { value, negated } => {
            let (values, tokens) = typed(value)?;
            TTest::Point {
                values,
                tokens,
                negated: *negated,
            }
        }
        Test::Divisible { divisor, negated } => {
            let (values, tokens) = typed(divisor)?;
            check_divisor(divisor, values[0])?;
            TTest::Divisible {
                divisor: values,
                tokens,
                negated: *negated,
            }
        }
        Test::Range { lo, hi, inclusive } => {
            let lo = lo.as_ref().map(typed).transpose()?;
            let hi = hi.as_ref().map(typed).transpose()?;
            check_empty_range(atom, &lo, &hi, *inclusive)?;
            TTest::Range {
                lo,
                hi,
                inclusive: *inclusive,
            }
        }
    };

    return Ok(TAtom { operand, test });
}

fn type_predicate(
    lens: &Lens,
    subject: N,
) -> syn::Result<TAtom> {
    let function = predicate_operand_check(lens, subject)?;
    return Ok(TAtom {
        operand: subject_operand(subject),
        test: TTest::Predicate(lens.name.clone(), function),
    });
}

/// A range empty at every pointer width is an error.
fn check_empty_range(
    atom: &Atom,
    lo: &Option<([Int; Width::COUNT], TokenStream)>,
    hi: &Option<([Int; Width::COUNT], TokenStream)>,
    inclusive: bool,
) -> syn::Result<()> {
    let (Some((lo_values, _)), Some((hi_values, hi_tokens))) = (lo, hi)
    else {
        return Ok(());
    };
    let empty = lo_values.iter().zip(hi_values).all(|(lo, hi)| {
        return match inclusive {
            true => lo > hi,
            false => lo >= hi,
        };
    });
    if empty {
        return atom.span.fail(format!(
            "this range is empty; its upper bound {hi_tokens} is not above \
            its lower bound"
        ));
    }
    return Ok(());
}

// -----------------------------------------------------------------------------
// Per-width folding: constants outside the operand type become `true` or
// `false`, so no out-of-range literal is ever emitted.

fn fold(
    node: &TNode,
    width: Width,
) -> RNode {
    return match node {
        TNode::Or(items) => {
            RNode::Or(items.iter().map(|it| fold(it, width)).collect())
        }
        TNode::And(items) => {
            RNode::And(items.iter().map(|it| fold(it, width)).collect())
        }
        TNode::Not(inner) => RNode::Not(Box::new(fold(inner, width))),
        TNode::Atom(atom) => fold_atom(atom, width),
    };
}

fn closed_bound(
    value: Int,
    tokens: &TokenStream,
) -> Bound {
    return Bound {
        value,
        tokens: tokens.clone(),
        inclusive: true,
    };
}

/// An upper end: none, below every integer (the atom is empty), or a bound.
enum Edge {
    Open,
    Below,
    At(Bound),
}

fn hi_edge(
    value: Int,
    tokens: &TokenStream,
    inclusive: bool,
) -> Edge {
    let value = match inclusive {
        true => Some(value),
        false => value.predecessor(),
    };
    return match value {
        None => Edge::Below,
        Some(value) => Edge::At(Bound {
            value,
            tokens: tokens.clone(),
            inclusive,
        }),
    };
}

fn fold_atom(
    atom: &TAtom,
    width: Width,
) -> RNode {
    let operand = &atom.operand;
    let index = width.index();
    return match &atom.test {
        TTest::Predicate(name, function) => RNode::Predicate {
            operand: operand.clone(),
            name: name.clone(),
            function: *function,
        },
        TTest::Range { lo, hi, inclusive } => fold_interval(
            operand,
            lo.as_ref()
                .map(|(values, tokens)| closed_bound(values[index], tokens)),
            match hi {
                None => Edge::Open,
                Some((values, tokens)) => {
                    hi_edge(values[index], tokens, *inclusive)
                }
            },
            width,
        ),
        TTest::Point {
            values,
            tokens,
            negated: false,
        } => fold_interval(
            operand,
            Some(closed_bound(values[index], tokens)),
            Edge::At(closed_bound(values[index], tokens)),
            width,
        ),
        TTest::Point {
            values,
            tokens,
            negated: true,
        } => fold_ne(operand, values[index], tokens, width),
        TTest::Divisible {
            divisor,
            tokens,
            negated,
        } => fold_divisible(operand, divisor[index], tokens, *negated, width),
    };
}

fn fold_interval(
    operand: &Operand,
    lo: Option<Bound>,
    hi: Edge,
    width: Width,
) -> RNode {
    let hi = match hi {
        Edge::Open => None,
        Edge::Below => return RNode::Const(false),
        Edge::At(it) => Some(it),
    };
    return clamp(
        RNode::Interval {
            operand: operand.clone(),
            lo,
            hi,
        },
        width,
    );
}

fn fold_ne(
    operand: &Operand,
    value: Int,
    tokens: &TokenStream,
    width: Width,
) -> RNode {
    let (min, max) = operand.ty.bounds_for_pointer_width(width);
    return match value < min || value > max {
        true => RNode::Const(true),
        false => RNode::Ne {
            operand: operand.clone(),
            value,
            tokens: tokens.clone(),
        },
    };
}

fn fold_divisible(
    operand: &Operand,
    divisor: Int,
    tokens: &TokenStream,
    negated: bool,
    width: Width,
) -> RNode {
    let (min, max) = operand.ty.bounds_for_pointer_width(width);
    if divisor <= max {
        return RNode::Divisible {
            operand: operand.clone(),
            divisor,
            tokens: tokens.clone(),
            negated,
        };
    }
    let node = beyond_divisor(operand, divisor, min);
    return match negated {
        true => RNode::Not(Box::new(node)),
        false => node,
    };
}

/// `mul(d)` above the type's maximum: `{0}`, or `{0} | {MIN}` when -MIN == d.
/// Every value has a smaller magnitude than the divisor except possibly the
/// signed minimum.
fn beyond_divisor(
    operand: &Operand,
    divisor: Int,
    min: Int,
) -> RNode {
    let zero = RNode::Interval {
        operand: operand.clone(),
        lo: Some(Bound {
            value: Int::NonNegative(0),
            tokens: quote! { 0 },
            inclusive: true,
        }),
        hi: Some(Bound {
            value: Int::NonNegative(0),
            tokens: quote! { 0 },
            inclusive: true,
        }),
    };
    if min.negate() != divisor {
        return zero;
    }
    let ty = Ident::new(operand.ty.name(), Span::call_site());
    let minimum = Bound {
        value: min,
        tokens: quote! { #ty::MIN },
        inclusive: true,
    };
    return RNode::Or(vec![
        zero,
        RNode::Interval {
            operand: operand.clone(),
            lo: Some(minimum.clone()),
            hi: Some(minimum),
        },
    ]);
}

/// N5: drops bounds at or beyond the operand type's limits; an interval that
/// is empty or covers the whole type becomes a constant.
fn clamp(
    node: RNode,
    width: Width,
) -> RNode {
    let RNode::Interval { operand, lo, hi } = node
    else {
        return node;
    };
    let (min, max) = operand.ty.bounds_for_pointer_width(width);
    let lo = lo.filter(|it| it.value > min);
    let hi = hi.filter(|it| it.value < max);
    if lo.as_ref().is_some_and(|it| it.value > max)
        || hi.as_ref().is_some_and(|it| it.value < min)
    {
        return RNode::Const(false);
    }
    if let (Some(lo), Some(hi)) = (&lo, &hi)
        && lo.value > hi.value
    {
        return RNode::Const(false);
    }
    if lo.is_none() && hi.is_none() {
        return RNode::Const(true);
    }
    return RNode::Interval { operand, lo, hi };
}

// -----------------------------------------------------------------------------
// Normalization (N1–N7). Never adds a comparison.

fn normalize(
    node: RNode,
    width: Width,
) -> RNode {
    return match node {
        RNode::Or(items) => normalize_or(items, width),
        RNode::And(items) => normalize_and(items, width),
        RNode::Not(inner) => match normalize(*inner, width) {
            RNode::Const(value) => RNode::Const(!value),
            other => RNode::Not(Box::new(other)),
        },
        other => other,
    };
}

fn flatten(
    items: Vec<RNode>,
    width: Width,
    or: bool,
) -> Vec<RNode> {
    let mut result = Vec::with_capacity(items.len());
    for item in items {
        match (normalize(item, width), or) {
            (RNode::Or(inner), true) | (RNode::And(inner), false) => {
                result.extend(inner);
            }
            (other, _) => result.push(other),
        }
    }
    return result;
}

/// The grouping key of an interval; other nodes are not grouped.
fn interval_key(node: &RNode) -> Option<&OperandKey> {
    return match node {
        RNode::Interval { operand, .. } => Some(&operand.key),
        _ => None,
    };
}

fn normalize_or(
    items: Vec<RNode>,
    width: Width,
) -> RNode {
    let items = flatten(items, width, true);
    if items.iter().any(|it| matches!(it, RNode::Const(true))) {
        return RNode::Const(true);
    }
    let items = items
        .into_iter()
        .filter(|it| !matches!(it, RNode::Const(false)))
        .collect::<Vec<_>>();

    // N3: per operand, sort intervals by lower bound and merge overlapping or
    // adjacent ones; the merged list takes the first interval's position.
    let (order, mut groups) = group_in_order(items, interval_key);

    let mut result = Vec::new();
    for entry in order {
        match entry {
            Err(item) => result.push(item),
            Ok(key) => {
                let merged = merge_union(groups.remove(&key).unwrap(), width);
                if merged.iter().any(|it| matches!(it, RNode::Const(true))) {
                    return RNode::Const(true);
                }
                result.extend(merged);
            }
        }
    }

    return collapse(result, RNode::Const(false), RNode::Or);
}

/// One operand's intersected interval: the operand and its two ends.
type Ends = (Operand, Option<Bound>, Option<Bound>);

fn interval_parts(node: RNode) -> Ends {
    let RNode::Interval { operand, lo, hi } = node
    else {
        unreachable!("only intervals are grouped");
    };
    return (operand, lo, hi);
}

fn merge_union(
    intervals: Vec<RNode>,
    width: Width,
) -> Vec<RNode> {
    let mut parts = intervals
        .into_iter()
        .map(interval_parts)
        .collect::<Vec<_>>();
    parts.sort_by(|left, right| {
        return match (&left.1, &right.1) {
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (Some(left), Some(right)) => left.value.cmp(&right.value),
        };
    });

    let mut merged = Vec::<(Operand, Option<Bound>, Option<Bound>)>::new();
    for next in parts {
        let Some(current) = merged.last_mut()
        else {
            merged.push(next);
            continue;
        };
        let connected = match (&current.2, &next.1) {
            (None, _) | (_, None) => true,
            (Some(hi), Some(lo)) => {
                lo.value <= hi.value
                    || hi.value.successor().is_some_and(|it| it == lo.value)
            }
        };
        if !connected {
            merged.push(next);
            continue;
        }
        current.2 = match (current.2.take(), next.2) {
            (None, _) | (_, None) => None,
            (Some(left), Some(right)) => Some(match right.value > left.value {
                true => right,
                false => left,
            }),
        };
    }

    return merged
        .into_iter()
        .map(|(operand, lo, hi)| {
            clamp(RNode::Interval { operand, lo, hi }, width)
        })
        .collect();
}

fn normalize_and(
    items: Vec<RNode>,
    width: Width,
) -> RNode {
    let items = flatten(items, width, false);
    if items.iter().any(|it| matches!(it, RNode::Const(false))) {
        return RNode::Const(false);
    }
    let items = items
        .into_iter()
        .filter(|it| !matches!(it, RNode::Const(true)))
        .collect::<Vec<_>>();

    // N4: per operand, intersect intervals at the first one's position.
    let (mut order, groups) = group_in_order(items, interval_key);
    let mut ends = BTreeMap::<OperandKey, Ends>::new();
    for (key, group) in groups {
        let mut parts = group.into_iter().map(interval_parts);
        let mut current = parts.next().expect("a group has an interval");
        for (_, lo, hi) in parts {
            intersect_ends(&mut current, lo, hi);
        }
        ends.insert(key, current);
    }

    if exclude_ends(&mut order, &mut ends).is_none() {
        return RNode::Const(false);
    }

    let mut result = Vec::new();
    for entry in order {
        match entry {
            Err(item) => result.push(item),
            Ok(key) => {
                let (operand, lo, hi) = ends[&key].clone();
                match clamp(RNode::Interval { operand, lo, hi }, width) {
                    RNode::Const(false) => return RNode::Const(false),
                    RNode::Const(true) => {}
                    other => result.push(other),
                }
            }
        }
    }

    let result = result
        .into_iter()
        .filter(|item| !redundant_ne(item, &ends))
        .collect::<Vec<_>>();

    return collapse(result, RNode::Const(true), RNode::And);
}

/// Keeps the greater lower and the smaller upper end; the left one on equal
/// values.
fn intersect_ends(
    ends: &mut Ends,
    lo: Option<Bound>,
    hi: Option<Bound>,
) {
    ends.1 = match (ends.1.take(), lo) {
        (None, other) | (other, None) => other,
        (Some(left), Some(right)) => Some(match right.value > left.value {
            true => right,
            false => left,
        }),
    };
    ends.2 = match (ends.2.take(), hi) {
        (None, other) | (other, None) => other,
        (Some(left), Some(right)) => Some(match right.value < left.value {
            true => right,
            false => left,
        }),
    };
}

/// N8: `!{c}` at an end of an AND-sibling interval on the same operand
/// moves that end past `c`, written as `> c` or `< c`. `None` when an end
/// moves past the type (the AND is `false`).
fn exclude_ends(
    order: &mut Vec<Result<OperandKey, RNode>>,
    ends: &mut BTreeMap<OperandKey, Ends>,
) -> Option<()> {
    while let Some(position) = order.iter().position(|entry| {
        let Err(RNode::Ne { operand, value, .. }) = entry
        else {
            return false;
        };
        return ends.get(&operand.key).is_some_and(|(_, lo, hi)| {
            return lo.as_ref().is_some_and(|it| it.value == *value)
                || hi.as_ref().is_some_and(|it| it.value == *value);
        });
    }) {
        let Err(RNode::Ne {
            operand,
            value,
            tokens,
        }) = order.remove(position)
        else {
            unreachable!("the position holds a `!{{c}}`");
        };
        let (_, lo, hi) = ends
            .get_mut(&operand.key)
            .expect("the operand has an interval");
        if lo.as_ref().is_some_and(|it| it.value == value) {
            *lo = Some(Bound {
                value: value.successor()?,
                tokens,
                inclusive: false,
            });
        }
        else {
            *hi = Some(Bound {
                value: value.predecessor()?,
                tokens,
                inclusive: false,
            });
        }
    }
    return Some(());
}

/// N7: `!= c` is redundant when an AND-sibling interval on the same operand
/// already excludes `c`.
fn redundant_ne(
    item: &RNode,
    ends: &BTreeMap<OperandKey, Ends>,
) -> bool {
    let RNode::Ne { operand, value, .. } = item
    else {
        return false;
    };
    return ends.get(&operand.key).is_some_and(|(_, lo, hi)| {
        lo.as_ref().is_some_and(|lo| *value < lo.value)
            || hi.as_ref().is_some_and(|hi| *value > hi.value)
    });
}

// -----------------------------------------------------------------------------
// Exact evaluation (zero probes, `Within` forms, tests)

/// The operand's value at `value`: the value itself, or the lens result,
/// exactly as the primitive method computes it at `width`.
pub(crate) fn lens_value(
    operand: &Operand,
    value: Int,
    width: Width,
) -> Int {
    let Some(kind) = operand.lens
    else {
        return value;
    };
    return match kind {
        LensKind::UnsignedAbs => Int::NonNegative(value.magnitude()),
        LensKind::AbsDiff => {
            Int::NonNegative(match (value, operand.arg(width)) {
                (Int::NonNegative(a), Int::NonNegative(b))
                | (Int::Negative(a), Int::Negative(b)) => a.abs_diff(b),
                (Int::NonNegative(a), Int::Negative(b))
                | (Int::Negative(a), Int::NonNegative(b)) => a + b,
            })
        }
        LensKind::Signum => match value {
            Int::NonNegative(0) => Int::NonNegative(0),
            Int::NonNegative(_) => Int::NonNegative(1),
            Int::Negative(_) => Int::Negative(1),
        },
        // Divisors are at least 2 (`check_divisor`), so neither overflows.
        LensKind::RemEuclid => value
            .checked_rem_euclid(operand.arg(width))
            .expect("a nonzero divisor"),
        LensKind::DivEuclid => value
            .checked_div_euclid(operand.arg(width))
            .expect("a nonzero divisor"),
        LensKind::Isqrt => value.isqrt().expect("isqrt takes unsigned values"),
        LensKind::CountOnes
        | LensKind::CountZeros
        | LensKind::LeadingZeros
        | LensKind::TrailingZeros
        | LensKind::LeadingOnes
        | LensKind::TrailingOnes => Int::NonNegative(u128::from(bit_count(
            kind,
            value,
            operand.subject.at(width),
        ))),
    };
}

/// A bit-counting lens over the two's-complement pattern of `value` in
/// `ty`: `count_ones(0) == 0`, `leading_zeros(0) == bits`.
fn bit_count(
    kind: LensKind,
    value: Int,
    ty: IntTy,
) -> u32 {
    let bits = ty.bits;
    let ones = value.pattern() & ty.mask();
    let zeros = !ones & ty.mask();
    let leading = |it: u128| bits - (u128::BITS - it.leading_zeros());
    let trailing = |it: u128| {
        return match it {
            0 => bits,
            it => it.trailing_zeros(),
        };
    };
    return match kind {
        LensKind::CountOnes => ones.count_ones(),
        LensKind::CountZeros => zeros.count_ones(),
        LensKind::LeadingZeros => leading(ones),
        LensKind::LeadingOnes => leading(zeros),
        LensKind::TrailingZeros => trailing(ones),
        LensKind::TrailingOnes => trailing(zeros),
        _ => unreachable!("{kind:?} does not count bits"),
    };
}

/// Whether a comparison leaf (`Interval`, `Ne`, `Divisible`) holds for the
/// operand value `it`.
pub(crate) fn test_holds(
    node: &RNode,
    it: Int,
) -> bool {
    return match node {
        RNode::Interval { lo, hi, .. } => {
            lo.as_ref().is_none_or(|lo| it >= lo.value)
                && hi.as_ref().is_none_or(|hi| it <= hi.value)
        }
        RNode::Ne { value, .. } => it != *value,
        RNode::Divisible {
            divisor, negated, ..
        } => it.magnitude().is_multiple_of(divisor.magnitude()) != *negated,
        _ => unreachable!("only comparisons test an operand value"),
    };
}

/// Whether the predicate `function` holds for `value`.
fn predicate_holds(
    function: FnName,
    value: Int,
) -> bool {
    return match function {
        FnName::IsPowerOfTwo => {
            matches!(value, Int::NonNegative(it) if it.is_power_of_two())
        }
        FnName::IsPositive => matches!(value, Int::NonNegative(it) if it > 0),
        FnName::IsNegative => value.is_negative(),
        _ => unreachable!("{function:?} is not a domain predicate"),
    };
}

/// Exact membership of `value` in `node` at `width`.
pub(crate) fn eval(
    node: &RNode,
    value: Int,
    width: Width,
) -> bool {
    return match node {
        RNode::Const(it) => *it,
        RNode::Or(items) => items.iter().any(|it| eval(it, value, width)),
        RNode::And(items) => items.iter().all(|it| eval(it, value, width)),
        RNode::Not(inner) => !eval(inner, value, width),
        RNode::Interval { operand, .. }
        | RNode::Ne { operand, .. }
        | RNode::Divisible { operand, .. } => {
            test_holds(node, lens_value(operand, value, width))
        }
        RNode::Predicate { function, .. } => predicate_holds(*function, value),
    };
}

impl ResolvedDomain {
    /// Exact membership of `value` at `width`.
    pub(crate) fn contains(
        &self,
        value: Int,
        width: Width,
    ) -> bool {
        return eval(&self.per_width[width.index()], value, width);
    }

    /// `at` is rejected at every pointer width.
    pub(crate) fn excludes(
        &self,
        at: Int,
    ) -> bool {
        return Width::ALL.into_iter().all(|it| !self.contains(at, it));
    }

    /// Every valid `x` has a valid `-x` at every width. Proven structurally:
    /// each atom treats `x` and `-x` alike, which `!`, `&&`, and `||` keep.
    /// `-MIN` wraps to `MIN` itself, which is then valid too.
    pub(crate) fn neg_closed(&self) -> bool {
        return self.per_width.iter().all(symmetric);
    }

    /// Every valid `x` has a valid `abs(x)` at every width: symmetric, or
    /// only non-negative values pass where it is not.
    pub(crate) fn abs_closed(&self) -> bool {
        return self.per_width.iter().all(abs_closed);
    }

    /// The validation condition over `value`: one expression, or a per-width
    /// `cfg` block when widths normalize differently.
    pub(crate) fn emit(
        &self,
        value: &TokenStream,
    ) -> TokenStream {
        let emitted =
            by_width(self.per_width.clone().map(|it| emit(&it, value)));
        if let [(_, condition)] = &emitted[..] {
            return condition.clone();
        }

        let variants = emitted.into_iter().map(|(cfg, condition)| {
            return quote! {
                #cfg
                let __typekin_domain_valid = #condition;
            };
        });
        return quote! {
            {
                #(#variants)*
                __typekin_domain_valid
            }
        };
    }
}

fn symmetric(node: &RNode) -> bool {
    return match node {
        RNode::Const(_) => true,
        RNode::Or(items) | RNode::And(items) => items.iter().all(symmetric),
        RNode::Not(inner) => symmetric(inner),
        RNode::Interval { operand, .. }
        | RNode::Ne { operand, .. }
        | RNode::Divisible { operand, .. }
        | RNode::Predicate { operand, .. }
            if operand.lens == Some(LensKind::UnsignedAbs) =>
        {
            true
        }
        RNode::Interval { operand, lo, hi } if operand.is_subject() => {
            match (lo, hi) {
                (None, None) => true,
                (Some(lo), Some(hi)) => lo.value == hi.value.negate(),
                _ => false,
            }
        }
        RNode::Ne { operand, value, .. } if operand.is_subject() => {
            *value == Int::NonNegative(0)
        }
        RNode::Divisible { operand, .. } => operand.is_subject(),
        _ => false,
    };
}

fn abs_closed(node: &RNode) -> bool {
    if symmetric(node) {
        return true;
    }
    return match node {
        RNode::Or(items) | RNode::And(items) => items.iter().all(abs_closed),
        RNode::Interval {
            operand,
            lo: Some(lo),
            ..
        } if operand.is_subject() => matches!(lo.value, Int::NonNegative(_)),
        RNode::Predicate {
            operand, function, ..
        } if operand.is_subject() => {
            matches!(function, FnName::IsPositive | FnName::IsPowerOfTwo)
        }
        _ => false,
    };
}

fn subject_tokens(
    operand: &Operand,
    value: &TokenStream,
) -> TokenStream {
    return match &operand.call {
        None => quote! { #value },
        Some(call) => quote! { (#value)#call },
    };
}
/// Atoms and negations emit their own parentheses; `||`/`&&` lists get them
/// here when nested.
fn emit_operand(
    node: &RNode,
    value: &TokenStream,
) -> TokenStream {
    let it = emit(node, value);
    return match node {
        RNode::Or(_) | RNode::And(_) => quote! { (#it) },
        _ => it,
    };
}

fn emit(
    node: &RNode,
    value: &TokenStream,
) -> TokenStream {
    return match node {
        RNode::Const(true) => quote! { true },
        RNode::Const(false) => quote! { false },
        RNode::Or(items) => {
            let items = items.iter().map(|it| emit_operand(it, value));
            quote! { #(#items)||* }
        }
        RNode::And(items) => {
            let items = items.iter().map(|it| emit_operand(it, value));
            quote! { #(#items)&&* }
        }
        RNode::Not(inner) => {
            let inner = emit_operand(inner, value);
            quote! { (!#inner) }
        }
        RNode::Interval { operand, lo, hi } => {
            let it = subject_tokens(operand, value);
            if let (Some(lo), Some(hi)) = (lo, hi)
                && lo.value == hi.value
                && (lo.inclusive || hi.inclusive)
            {
                let tokens = match lo.inclusive {
                    true => &lo.tokens,
                    false => &hi.tokens,
                };
                return quote! { (#it == (#tokens)) };
            }
            let lo = lo.as_ref().map(|lo| {
                let tokens = &lo.tokens;
                let op = match lo.inclusive {
                    true => quote! { >= },
                    false => quote! { > },
                };
                quote! { (#it #op (#tokens)) }
            });
            let hi = hi.as_ref().map(|hi| {
                let tokens = &hi.tokens;
                let op = match hi.inclusive {
                    true => quote! { <= },
                    false => quote! { < },
                };
                quote! { (#it #op (#tokens)) }
            });
            match (lo, hi) {
                (Some(lo), Some(hi)) => quote! { (#lo && #hi) },
                (Some(it), None) | (None, Some(it)) => it,
                (None, None) => quote! { true },
            }
        }
        RNode::Ne {
            operand, tokens, ..
        } => {
            let it = subject_tokens(operand, value);
            quote! { (#it != (#tokens)) }
        }
        RNode::Divisible {
            operand,
            tokens,
            negated,
            ..
        } => {
            let it = subject_tokens(operand, value);
            match negated {
                false => quote! { ((#it % (#tokens)) == 0) },
                true => quote! { ((#it % (#tokens)) != 0) },
            }
        }
        RNode::Predicate { operand, name, .. } => {
            let it = subject_tokens(operand, value);
            quote! { ((#it).#name()) }
        }
    };
}
