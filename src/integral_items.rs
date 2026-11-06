use proc_macro2::{
    Ident,
    Literal,
    Span,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::LitInt;

use crate::integral::{
    Maker,
    pointer_width_cfg,
};
use crate::integral_cfg::{
    IntegralFlag,
    IntegralName,
};
use crate::integral_impls::SHIFTS;
use crate::integral_proof::OperatorProof;
use crate::zz_friend::{
    Capability,
    Friend,
    any_friend_has,
};
use crate::zz_lib::{
    Konst,
    Konsts,
    validated_result,
};
use crate::zz_profile_integral::Arithmetic;
use crate::zz_runner::Merged;
use crate::zz_values::{
    ArithmeticOp,
    F,
    N,
    Num,
    Operator,
    POINTER_WIDTHS,
};

/// The inherent items, in output order: conversions, the math helpers,
/// `precedes`, comparisons, and construction.
const ITEM_EMITTERS: &[fn(&Maker) -> TokenStream] = &[
    Maker::make_fn_conv_of,
    Maker::make_auto_of_raw,
    Maker::make_fn_conv_raw,
    Maker::make_fn_conv_into,
    Maker::make_map,
    Maker::make_fn_conv_try_into_lossless,
    Maker::make_fn_conv_try_into_lossy,
    Maker::make_fn_math_helpers,
    Maker::make_fn_math_not,
    Maker::make_fn_math_neg,
    Maker::make_precedes,
    Maker::make_fn_op_eq,
    Maker::make_fn_op_cmp,
    Maker::make_fn_make_try,
    Maker::make_or_panic,
    Maker::make_or_err,
    Maker::make_proven,
    Maker::make_blind_trust,
    Maker::make_within,
    Maker::make_within_proofs,
    Maker::make_within_source,
];

/// Bit-access part types after the halves, in output order.
const BIT_PARTS: [N; 4] = [
    N::U008,
    N::U016,
    N::U032,
    N::U064,
];

/// A validating binary math helper (`make_fn_math_*`).
#[derive(Clone, Copy)]
enum MathHelper {
    Arithmetic(ArithmeticOp),
    Bitwise(Operator),
}

/// The binary math helpers, in emission order; Shr/Shl take `usize`.
const FN_MATH_HELPERS: [(IntegralFlag, MathHelper); 10] = [
    (
        IntegralFlag::FnMathAdd,
        MathHelper::Arithmetic(ArithmeticOp::Add),
    ),
    (
        IntegralFlag::FnMathSub,
        MathHelper::Arithmetic(ArithmeticOp::Sub),
    ),
    (
        IntegralFlag::FnMathMul,
        MathHelper::Arithmetic(ArithmeticOp::Mul),
    ),
    (
        IntegralFlag::FnMathDiv,
        MathHelper::Arithmetic(ArithmeticOp::Div),
    ),
    (
        IntegralFlag::FnMathRem,
        MathHelper::Arithmetic(ArithmeticOp::Rem),
    ),
    (
        IntegralFlag::FnMathXor,
        MathHelper::Bitwise(Operator::BitXor),
    ),
    (
        IntegralFlag::FnMathAnd,
        MathHelper::Bitwise(Operator::BitAnd),
    ),
    (IntegralFlag::FnMathOr, MathHelper::Bitwise(Operator::BitOr)),
    (
        IntegralFlag::FnMathShr,
        MathHelper::Arithmetic(ArithmeticOp::Shr),
    ),
    (
        IntegralFlag::FnMathShl,
        MathHelper::Arithmetic(ArithmeticOp::Shl),
    ),
];

impl Maker {
    /// The inherent items of `ITEM_EMITTERS`, in output order.
    pub(crate) fn item_fns(&self) -> TokenStream {
        return ITEM_EMITTERS.iter().map(|emit| emit(self)).merged();
    }

    /// `pub(self) #konst fn #name(&self, [rhs: #rhs,]) -> Self`: applies
    /// `expression` to the raw `lhs` (and `rhs`) and returns `result`. When
    /// operations report failure, `expression` is an `Option` of the raw
    /// result and the helper returns `_or_err`'s `Result`.
    fn validating_helper(
        &self,
        konst: &Option<TokenStream>,
        name: &Ident,
        rhs: Option<&TokenStream>,
        expression: &TokenStream,
        result: &TokenStream,
    ) -> TokenStream {
        let get_raw = &self.ctx.get_raw;
        let rhs = rhs.map(|rhs| quote! { rhs: #rhs, });
        let output = match &self.ctx.failure {
            Some(failure) => quote! { ::core::result::Result<Self, #failure> },
            None => quote! { Self },
        };
        return quote! {
            #[inline(always)]
            pub(self) #konst fn #name(
                &self,
                #rhs
            ) -> #output {
                let lhs = #get_raw(*self);
                let it = #expression;
                return #result;
            }
        };
    }

    /// The validating helper `name` (a proven result skips validation), plus
    /// a `_proven` sibling when only same-type operands are proven
    /// (`same_type_helper`). Reporting failure, every result goes through
    /// `_or_err` instead.
    fn proven_helpers(
        &self,
        konst: &Option<TokenStream>,
        name: &Ident,
        rhs: &TokenStream,
        expression: &TokenStream,
        proof: OperatorProof,
    ) -> TokenStream {
        if self.ctx.failure.is_some() {
            let or_err = self.ctx.names.get(IntegralName::OrErr);
            return self.validating_helper(
                konst,
                name,
                Some(rhs),
                expression,
                &quote! { Self::#or_err(it) },
            );
        }
        let fp_or_panic = &self.ctx.fp_or_panic;
        let helper = |name: &Ident, proven: bool| {
            let constructor =
                self.proven_constructor(proven, quote! { #fp_or_panic });
            let result = quote! { #constructor(it) };
            return self.validating_helper(
                konst,
                name,
                Some(rhs),
                expression,
                &result,
            );
        };
        let mut stream = helper(name, proof.full);
        let same = Self::same_type_helper(proof, name);
        if same != *name {
            stream.extend(helper(&same, true));
        }
        return stream;
    }

    /// `operation` on the raw operands: panicking, or an `Option` of the
    /// result when operations report failure.
    fn arithmetic_expr(
        &self,
        operation: ArithmeticOp,
        is_konst: bool,
    ) -> TokenStream {
        let arithmetic = self.ctx.cfg.profile.arithmetic();
        if self.ctx.failure.is_some() {
            return arithmetic.make_fallible_arithmetic(
                operation,
                self.ctx.cfg.panic_at,
                matches!(self.num, Num::Float(_)),
                is_konst,
            );
        }
        return match is_konst {
            true => arithmetic.make_const_expr(operation),
            false => arithmetic.make_expr(operation),
        };
    }

    /// The validating `_{operation}` helper (and its non-const `_wrapping`
    /// twin for `Wrapping` storage).
    fn ekran_arithmetic_helper(
        &self,
        operation: ArithmeticOp,
        rhs_type: TokenStream,
    ) -> TokenStream {
        let arithmetic = self.ctx.cfg.profile.arithmetic();
        let proof = self.arithmetic_proof(operation);
        let mut stream = self.proven_helpers(
            &Konst::konst(self.ctx.cfg.helper_konst()),
            &self.arithmetic_method_name(operation, true),
            &rhs_type,
            &self.arithmetic_expr(operation, true),
            proof,
        );
        if arithmetic != Arithmetic::WrapperWrapping {
            return stream;
        }
        stream.extend(self.proven_helpers(
            &Konst::konst(false),
            &self.arithmetic_method_name(operation, false),
            &rhs_type,
            &self.arithmetic_expr(operation, false),
            proof,
        ));
        return stream;
    }

    /// The validating `_bit{and,or,xor}` helper.
    fn make_fn_math_bitwise(
        &self,
        operator: Operator,
    ) -> TokenStream {
        let el = &self.el;
        let symbol = operator.symbol();
        let expression = match self.ctx.failure {
            Some(_) => quote! { ::core::option::Option::Some(lhs #symbol rhs) },
            None => quote! { lhs #symbol rhs },
        };
        return self.proven_helpers(
            &Konst::konst(self.ctx.cfg.helper_konst()),
            &format_ident!("_{}", operator.method_name()),
            &quote! { #el },
            &expression,
            self.operator_proof(operator),
        );
    }

    /// The enabled `FN_MATH_HELPERS` an emitted operator calls, in table
    /// order.
    fn make_fn_math_helpers(&self) -> TokenStream {
        let el = &self.el;
        return FN_MATH_HELPERS
            .into_iter()
            .filter(|(flag, helper)| {
                return self.ctx.cfg.flags.has(*flag)
                    && self.calls_math_helper(*helper);
            })
            .map(|(_, helper)| {
                return match helper {
                    MathHelper::Arithmetic(
                        operation @ (ArithmeticOp::Shr | ArithmeticOp::Shl),
                    ) => self
                        .ekran_arithmetic_helper(operation, quote! { usize }),
                    MathHelper::Arithmetic(operation) => {
                        self.ekran_arithmetic_helper(operation, quote! { #el })
                    }
                    MathHelper::Bitwise(operator) => {
                        self.make_fn_math_bitwise(operator)
                    }
                };
            })
            .merged();
    }

    /// Whether an emitted operator impl calls `helper`. Shifts by an amount
    /// are emitted with or without facets; the other binary operators and
    /// their assignments only without, since facet dispatch inlines its
    /// arithmetic.
    fn calls_math_helper(
        &self,
        helper: MathHelper,
    ) -> bool {
        let flags = &self.ctx.cfg.flags;
        let operator = match helper {
            MathHelper::Arithmetic(operation) => {
                if let Some(shift) =
                    SHIFTS.iter().find(|shift| shift.operation == operation)
                {
                    return flags.has(shift.math) || flags.has(shift.assign);
                }
                operation
                    .operator()
                    .expect("non-shift arithmetic has a binary operator")
            }
            MathHelper::Bitwise(operator) => operator,
        };
        let assign =
            IntegralFlag::assign(operator).is_some_and(|flag| flags.has(flag));
        return self.with_global_operators()
            && (flags.has(IntegralFlag::math(operator)) || assign);
    }

    pub(crate) fn ekran_bit_access_items(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::BitAccess)
            || self.repr().is_signed()
        {
            return TokenStream::new();
        }

        let mut stream = self.bit_access_halves();
        stream.extend(BIT_PARTS.into_iter().map(|part| {
            return self.bit_access_parts(part);
        }));
        return stream;
    }

    /// Every `part`-sized slice of the raw value, low to high, each behind
    /// the pointer widths that have it.
    fn bit_access_parts(
        &self,
        part: N,
    ) -> TokenStream {
        let get_raw = &self.ctx.get_raw;
        let el = &self.el;
        let repr = self.repr();
        let bits = part.bits_for_pointer_width(64);
        let ty = format_ident!("{}", part.rust_name());
        let shift = Literal::usize_unsuffixed(bits);
        let mask = LitInt::new(
            &format!("0x{}", "F".repeat(bits / 4)),
            Span::call_site(),
        );
        return (0..repr.bits_for_pointer_width(64) / bits)
            .filter_map(|i| {
                let cfg = pointer_width_cfg(|width| {
                    return repr.bits_for_pointer_width(width) >= bits * (i + 1);
                })?;
                let name = self.ctx.names.get(IntegralName::BitPart(part, i));
                return Some(quote! {
                    #cfg
                    #[must_use]
                    #[inline(always)]
                    pub const fn #name(self) -> #ty {
                        return ((#get_raw(self) >> (#shift * #i)) & (#mask as #el)) as #ty;
                    }
                });
            })
            .merged();
    }

    /// The low and high halves, from the `BIT_PARTS` row of half the width;
    /// none for `usize`.
    fn bit_access_halves(&self) -> TokenStream {
        let repr = self.repr();
        let half = BIT_PARTS.into_iter().find(|part| {
            return repr != N::USIZ
                && 2 * part.bits_for_pointer_width(64)
                    == repr.bits_for_pointer_width(64);
        });
        let Some(half) = half
        else {
            return TokenStream::new();
        };

        let names = &self.ctx.names;
        let ty = format_ident!("{}", half.rust_name());
        let low = names.get(IntegralName::LowHalf(half));
        let high = names.get(IntegralName::HighHalf(half));
        let low_part = names.get(IntegralName::BitPart(half, 0));
        let high_part = names.get(IntegralName::BitPart(half, 1));
        return quote! {
            #[must_use]
            #[inline(always)]
            pub const fn #low(self) -> #ty {
                return self.#low_part();
            }

            #[must_use]
            #[inline(always)]
            pub const fn #high(self) -> #ty {
                return self.#high_part();
            }
        };
    }

    pub(crate) fn make_fn_conv_of(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::FnConvOf) {
            return TokenStream::new();
        }

        let ty = &self.ctx.ty;
        let arguments = &self.ctx.arguments;
        let is_konst = self.ctx.cfg.konst_for(&[
            Konsts::Of,
            Konsts::Conversion,
        ]);
        let konst = Konst::konst(is_konst);
        let what = self
            .ctx
            .friends
            .of(Capability::Make)
            .expect("integral capabilities have protocol traits");
        let bound = Konst::bound(is_konst, quote! { #what #arguments });
        let parameter = self.target.fresh_ident("T");

        if self.ctx.failure.is_some() {
            let el = &self.el;
            let fn_try_of = self.ctx.names.get(IntegralName::TryOfFn);
            return quote! {
                #[inline(always)]
                #[allow(private_bounds)]
                pub #konst fn #fn_try_of<#parameter>(
                    it: #parameter,
                ) -> ::core::result::Result<#ty, #el>
                where
                    #parameter: #bound,
                {
                    return <#parameter as #what #arguments>::try_make(it);
                }
            };
        }

        let fn_of = self.ctx.names.get(IntegralName::OfFn);

        return quote! {
            #[inline(always)]
            #[allow(private_bounds)]
            pub #konst fn #fn_of<#parameter>(
                it: #parameter,
            ) -> #ty
            where
                #parameter: #bound,
            {
                return <#parameter as #what #arguments>::make(it);
            }
        };
    }

    /// `make(raw)`, which exists only for unvalidated types and only where
    /// the primitive's `Make` friend does (`without = self_friend_make` or
    /// validation leave it out). It skips the friend protocol: the friend
    /// only casts the primitive to itself and builds through an unvalidated
    /// constructor, so `make` calls `_or_panic` directly, `const` like
    /// unvalidated `try_make`.
    pub(crate) fn make_auto_of_raw(&self) -> TokenStream {
        let ty = &self.ctx.ty;
        let el = &self.el;
        let el_makes = self
            .ctx
            .cfg
            .friends
            .get(&Friend::from(el))
            .is_some_and(|it| it.has(Capability::Make));
        if !self.ctx.cfg.flags.has(IntegralFlag::AutoOfRaw) || !el_makes {
            return TokenStream::new();
        }

        let fn_make = self.ctx.names.get(IntegralName::MakeFn);
        let fp_or_panic = &self.ctx.fp_or_panic;
        return quote! {
            #[inline(always)]
            pub const fn #fn_make(it: #el) -> #ty {
                return #fp_or_panic(it);
            }
        };
    }

    pub(crate) fn make_fn_conv_raw(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::FnConvRaw) {
            return TokenStream::new();
        }

        let el = &self.el;
        let raw = &self.ctx.self_raw;
        let get_raw_ident = &self.ctx.get_raw.segments.last().unwrap().ident;

        return quote! {
            #[must_use]
            #[inline(always)]
            pub const fn #get_raw_ident(self) -> #el {
                return #raw;
            }
        };
    }

    pub(crate) fn make_fn_conv_into(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::FnConvInto) {
            return TokenStream::new();
        }

        let get_raw = &self.ctx.get_raw;
        return self
            .cast_targets(true)
            .map(|(it, cfg)| {
                let target = format_ident!("{}", it.rust_name());
                let conv = self.ctx.names.get(IntegralName::Into(it));
                return quote! {
                    #cfg
                    #[must_use]
                    #[inline(always)]
                    #[allow(clippy::unnecessary_cast)]
                    pub const fn #conv(self) -> #target {
                        let it = #get_raw(self);
                        return it as #target;
                    }
                };
            })
            .merged();
    }

    /// `try_map`, and `map` unless construction reports failure: `map`
    /// would then return what `try_map` does.
    pub(crate) fn make_map(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::Map) {
            return TokenStream::new();
        }

        let el = &self.el;
        let get_raw = &self.ctx.get_raw;
        let is_konst = self.ctx.cfg.konst_for(&[Konsts::Map])
            || self.ctx.cfg.flags.has(IntegralFlag::WithConstMapFn);
        let konst = Konst::konst(is_konst);
        let bound =
            Konst::bound(is_konst, quote! { ::core::ops::FnOnce(#el) -> #el });
        let callback = self.target.fresh_ident("F");
        let constructed = self.construct_self(quote! { value });
        let result = validated_result(
            &self.ctx.validation,
            constructed,
            quote! { value },
        );
        let map = self.ctx.failure.is_none().then(|| {
            quote! {
                #[inline(always)]
                #[track_caller]
                pub #konst fn map<#callback>(
                    self,
                    function: #callback,
                ) -> Self
                where
                    #callback: #bound,
                {
                    return match self.try_map(function) {
                        ::core::result::Result::Ok(value) => value,
                        ::core::result::Result::Err(_) => {
                            ::core::panic!(
                                "mapping operation produced an invalid value"
                            )
                        }
                    };
                }
            }
        });
        return quote! {
            #[inline(always)]
            pub #konst fn try_map<#callback>(
                self,
                function: #callback,
            ) -> ::core::result::Result<Self, #el>
            where
                #callback: #bound,
            {
                let value = function(#get_raw(self));
                #result
            }

            #map
        };
    }

    pub(crate) fn make_fn_conv_try_into_lossless(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::FnConvTryIntoLossless) {
            return TokenStream::new();
        }

        let get_raw = &self.ctx.get_raw;
        return self.cast_targets(true)
            .map(|(it, cfg)| {
                let target = format_ident!("{}", it.rust_name());
                let conv = self.ctx.names.get(IntegralName::TryInto(it));
                return quote! {
                    #cfg
                    #[inline(always)]
                    #[allow(clippy::unnecessary_cast)]
                    pub const fn #conv(self) -> ::core::result::Result<#target, ()> {
                        return ::core::result::Result::Ok(
                            #get_raw(self) as #target
                        );
                    }
                };
            })
            .merged();
    }

    pub(crate) fn make_fn_conv_try_into_lossy(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::FnConvTryIntoLossy) {
            return TokenStream::new();
        }

        let get_raw = &self.ctx.get_raw;
        let source = format_ident!("{}", self.num.rust_name());
        return self.cast_targets(false)
            .flat_map(|(it, cfg)| {
                let target = format_ident!("{}", it.rust_name());
                let name = self.ctx.names.get(IntegralName::TryInto(it));
                let bodies = self.lossy_bodies(it, cfg, &source, &target);
                return bodies.into_iter().map(move |(cfg, body)| {
                    return quote! {
                        #cfg
                        #[inline(always)]
                        #[allow(clippy::unnecessary_cast)]
                        pub const fn #name(self) -> ::core::result::Result<#target, ()> {
                            let r = #get_raw(self);
                            #body
                        }
                    };
                });
            })
            .merged();
    }

    /// The bodies of the lossy conversion to `it`, each with its `cfg`;
    /// `cfg` is the conversion's own, which the float cases replace.
    fn lossy_bodies(
        &self,
        it: Num,
        cfg: TokenStream,
        source: &Ident,
        target: &Ident,
    ) -> Vec<(TokenStream, TokenStream)> {
        return match (self.num, it) {
            (Num::Int(repr), Num::Int(it)) => {
                vec![
                    (
                        cfg,
                        Self::lossy_int_to_int_body(repr, it, source, target),
                    ),
                ]
            }
            (Num::Float(_), Num::Float(_)) => {
                vec![(cfg, Self::lossy_float_to_float_body(source, target))]
            }
            (Num::Float(float), Num::Int(it)) => {
                assert!(
                    cfg.is_empty(),
                    "float to integer conversions are lossy at every width",
                );
                Self::float_to_int_bodies(float, it, source, target)
            }
            (Num::Int(repr), Num::Float(float)) => {
                Self::int_to_float_bodies(repr, float, source, target)
            }
        };
    }

    /// `#prelude return if #condition { Ok(t) } else { Err(()) };`
    fn cast_result(
        prelude: TokenStream,
        condition: TokenStream,
    ) -> TokenStream {
        return quote! {
            #prelude

            return if #condition {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        };
    }

    /// The round trip, plus the sign where only one side is signed.
    fn lossy_int_to_int_body(
        repr: N,
        it: N,
        source: &Ident,
        target: &Ident,
    ) -> TokenStream {
        let sign_check = match (repr.is_signed(), it.is_signed()) {
            (false, true) => quote! { t >= 0 },
            (true, false) => quote! { r >= 0 },
            _ => quote! { true },
        };
        return Self::cast_result(
            quote! {
                let t = r as #target;
                let s = t as #source;
            },
            quote! { s == r && #sign_check },
        );
    }

    /// Narrowing keeps NaN (D3) and infinities; a finite value must survive
    /// the round trip.
    fn lossy_float_to_float_body(
        source: &Ident,
        target: &Ident,
    ) -> TokenStream {
        return Self::cast_result(
            quote! { let t = r as #target; },
            quote! { t as #source == r || r.is_nan() },
        );
    }

    /// `as` saturates to `MIN`/`MAX` and maps NaN to zero. When `MAX` is
    /// exactly representable in the float, the round trip alone rejects
    /// every out-of-range value, NaN, and fractions (a saturated result only
    /// equals `r` when `r` is `MIN` or `MAX` itself). Otherwise `MAX` rounds
    /// up to `2^bits` (`2^(bits-1)` signed), which also needs the exclusive
    /// upper bound; `MIN` is a power of two and always exact. `usize` and
    /// `isize` get one variant per pointer width.
    fn float_to_int_bodies(
        float: F,
        it: N,
        source: &Ident,
        target: &Ident,
    ) -> Vec<(TokenStream, TokenStream)> {
        let exact = |width: u32| {
            return it.magnitude_bits_for_pointer_width(width)
                <= float.mantissa_digits();
        };
        let round_trip = Self::cast_result(
            quote! { let t = r as #target; },
            quote! { t as #source == r },
        );
        let bounded = Self::upper_bounds(it, float, |width| !exact(width))
            .into_iter()
            .map(|(cfg, hi)| {
                return (
                    cfg,
                    Self::cast_result(
                        quote! { let t = r as #target; },
                        quote! { t as #source == r && r < #hi },
                    ),
                );
            });
        return pointer_width_cfg(exact)
            .map(|cfg| (cfg, round_trip))
            .into_iter()
            .chain(bounded)
            .collect();
    }

    /// A round trip catches rounding, except where `MAX` rounds up to
    /// `2^bits` (`2^(bits-1)` signed) and saturates back to `MAX`.
    fn int_to_float_bodies(
        repr: N,
        float: F,
        source: &Ident,
        target: &Ident,
    ) -> Vec<(TokenStream, TokenStream)> {
        let lossy = |width: u32| {
            return !Num::Int(repr).can_safe_cast_to(Num::Float(float), width);
        };
        return Self::upper_bounds(repr, float, lossy)
            .into_iter()
            .map(|(cfg, hi)| {
                return (
                    cfg,
                    Self::cast_result(
                        quote! { let t = r as #target; },
                        quote! { t as #source == r && t < #hi },
                    ),
                );
            })
            .collect();
    }

    /// The exclusive upper bound `2^bits` (`2^(bits-1)` signed) of `it` as a
    /// `float` literal, one entry per distinct value over the widths where
    /// `applies`. `2^128` overflows `f32` to infinity, which bounds every
    /// finite `f32`.
    fn upper_bounds(
        it: N,
        float: F,
        applies: impl Fn(u32) -> bool,
    ) -> Vec<(TokenStream, TokenStream)> {
        let literal = |width: u32| {
            let exponent = it.magnitude_bits_for_pointer_width(width);
            return match (float, exponent) {
                (F::F032, 128..) => None,
                _ => Some(crate::zz_values::pow2_literal(exponent)),
            };
        };
        let mut bounds = Vec::new();
        let mut seen = Vec::new();
        for width in POINTER_WIDTHS {
            if !applies(width) || seen.contains(&literal(width)) {
                continue;
            }
            let value = literal(width);
            seen.push(value.clone());
            let cfg = pointer_width_cfg(|other| {
                return applies(other) && literal(other) == value;
            })
            .expect("the width itself matches");
            let hi = match value {
                Some(value) => {
                    let value = syn::LitFloat::new(&value, Span::call_site());
                    quote! { #value }
                }
                None => quote! { ::core::primitive::f32::INFINITY },
            };
            bounds.push((cfg, hi));
        }
        return bounds;
    }

    /// `_not`, which only the `Not` impl calls.
    pub(crate) fn make_fn_math_not(&self) -> TokenStream {
        let flags = &self.ctx.cfg.flags;
        if !flags.has(IntegralFlag::FnMathNot)
            || !flags.has(IntegralFlag::MathNot)
        {
            return TokenStream::new();
        }

        let some = self.ctx.failure.is_some();
        return self.make_fn_math_unary(
            format_ident!("_not"),
            match some {
                true => quote! { ::core::option::Option::Some(!lhs) },
                false => quote! { !lhs },
            },
            self.not_proven(),
        );
    }

    /// `_neg`, which only the `Neg` impl calls. Floats and `Primitive`
    /// integers negate like the raw type; the other integer policies wrap or
    /// check like their binary operators.
    pub(crate) fn make_fn_math_neg(&self) -> TokenStream {
        let flags = &self.ctx.cfg.flags;
        if !flags.has(IntegralFlag::FnMathNeg)
            || !flags.has(IntegralFlag::MathNeg)
        {
            return TokenStream::new();
        }

        let expression = match self.ctx.failure {
            Some(_) => self.fallible_neg(),
            None => self.panicking_neg(),
        };
        return self.make_fn_math_unary(
            format_ident!("_neg"),
            expression,
            self.neg_proven(),
        );
    }

    /// `-lhs` under the profile's arithmetic, panicking where it may.
    fn panicking_neg(&self) -> TokenStream {
        return match (self.num, self.ctx.cfg.profile.arithmetic()) {
            (Num::Float(_), _) | (Num::Int(_), Arithmetic::Primitive) => {
                quote! { -lhs }
            }
            (
                Num::Int(_),
                Arithmetic::PrimitiveWrapping | Arithmetic::WrapperWrapping,
            ) => quote! { lhs.wrapping_neg() },
            (Num::Int(_), Arithmetic::Checked) => quote! {
                match lhs.checked_neg() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panic!("operation overflowed");
                    }
                }
            },
        };
    }

    /// `-lhs` as an `Option`: `sequential` always checks, `int` checks at
    /// `nothing`, and wrapping or float negation cannot fail.
    fn fallible_neg(&self) -> TokenStream {
        let checked = match (self.num, self.ctx.cfg.profile.arithmetic()) {
            (Num::Float(_), _) => false,
            (Num::Int(_), Arithmetic::Checked) => true,
            (Num::Int(_), Arithmetic::Primitive) => {
                self.ctx.cfg.panic_at.checks_overflow()
            }
            (
                Num::Int(_),
                Arithmetic::PrimitiveWrapping | Arithmetic::WrapperWrapping,
            ) => {
                return quote! {
                    ::core::option::Option::Some(lhs.wrapping_neg())
                };
            }
        };
        return match checked {
            true => quote! { lhs.checked_neg() },
            false => quote! { ::core::option::Option::Some(-lhs) },
        };
    }

    /// `closed`: the result of `expression` on a valid value is always valid
    /// (proven from the domain), so it is constructed without the check.
    fn make_fn_math_unary(
        &self,
        name: Ident,
        expression: TokenStream,
        closed: bool,
    ) -> TokenStream {
        let (konst, result) =
            match (&self.ctx.failure, self.closed_result(closed)) {
                (Some(_), _) => {
                    let or_err = self.ctx.names.get(IntegralName::OrErr);
                    (
                        Konst::konst(self.ctx.cfg.helper_konst()),
                        quote! { Self::#or_err(it) },
                    )
                }
                (None, Some(constructed)) => (Konst::konst(true), constructed),
                (None, None) => {
                    let fp_or_panic = &self.ctx.fp_or_panic;
                    (
                        Konst::konst(self.ctx.cfg.helper_konst()),
                        quote! { #fp_or_panic(it) },
                    )
                }
            };
        return self.validating_helper(
            &konst,
            &name,
            None,
            &expression,
            &result,
        );
    }

    pub(crate) fn make_precedes(&self) -> TokenStream {
        if !self.ctx.cfg.profile.emits_precedes() {
            return TokenStream::new();
        }

        let get_raw = &self.ctx.get_raw;
        let unsigned = format_ident!("{}", self.repr().unsigned_rust_name());
        let konst = Konst::konst(self.ctx.cfg.konst_for(&[Konsts::Value]));
        return quote! {
            #[must_use]
            #[inline(always)]
            pub #konst fn precedes(self, that: Self) -> bool {
                let lhs = #get_raw(self) as #unsigned;
                let rhs = #get_raw(that) as #unsigned;
                let difference = rhs.wrapping_sub(lhs);
                let half = (1 as #unsigned) << (#unsigned::BITS - 1);
                return difference != 0 && difference < half;
            }
        };
    }

    pub(crate) fn make_fn_op_eq(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::FnOpEq) {
            return TokenStream::new();
        }

        let el = &self.el;
        let get_raw = &self.ctx.get_raw;
        return quote! {
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _eq(
                self,
                rhs: #el,
            ) -> bool {
                let lhs = #get_raw(self);
                return lhs == rhs;
            }
        };
    }

    pub(crate) fn make_fn_op_cmp(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::FnOpCmp) {
            return TokenStream::new();
        }

        let el = &self.el;
        let get_raw = &self.ctx.get_raw;
        return match self.num {
            Num::Int(_) => quote! {
                #[must_use]
                #[inline(always)]
                #[doc(hidden)]
                pub(self) fn _cmp(
                    self,
                    rhs: #el,
                ) -> ::core::cmp::Ordering {
                    let lhs = #get_raw(self);
                    return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs)
                        .unwrap();
                }
            },
            Num::Float(_) => {
                let compared = Self::float_cmp();
                quote! {
                    #[must_use]
                    #[inline(always)]
                    #[doc(hidden)]
                    pub(self) const fn _cmp(
                        self,
                        rhs: #el,
                    ) -> ::core::cmp::Ordering {
                        let lhs = #get_raw(self);
                        #compared
                    }
                }
            }
        };
    }

    /// Total order of two non-NaN floats `lhs` and `rhs` (`-0.0 == +0.0`).
    pub(crate) fn float_cmp() -> TokenStream {
        return quote! {
            return if lhs < rhs {
                ::core::cmp::Ordering::Less
            }
            else if lhs > rhs {
                ::core::cmp::Ordering::Greater
            }
            else {
                ::core::cmp::Ordering::Equal
            };
        };
    }

    pub(crate) fn make_fn_make_try(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::FnTryMake) {
            return TokenStream::new();
        }

        let el = &self.el;
        if !self.ctx.cfg.validation.has_validation() {
            let fp_or_panic = &self.ctx.fp_or_panic;
            return quote! {
                #[inline(always)]
                pub const fn try_make(it: #el) -> Result<Self, #el> {
                    return Ok(#fp_or_panic(it));
                }
            };
        }

        let konst = Konst::konst(self.ctx.cfg.konst_for(&[Konsts::TryMake]));
        let value = quote! { it };
        let validation = self.ctx.cfg.validation.emit(&value);
        let constructed = self.construct_self(value);
        return quote! {
            #[inline(always)]
            pub #konst fn try_make(it: #el) -> Result<Self, #el> {
                return if #validation {
                    return ::core::result::Result::Ok(#constructed);
                }
                else {
                    ::core::result::Result::Err(it)
                };
            }
        };
    }

    /// Whether any friend holds `capability`.
    fn has_friend_with(
        &self,
        capability: Capability,
    ) -> bool {
        return any_friend_has(&self.ctx.cfg.friends, capability);
    }

    /// The validating, panicking constructor (`_or_panic`). Reporting
    /// failure, a validated type has none: `_or_err` validates instead.
    fn make_or_panic(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::FnOrPanic)
            || (self.ctx.failure.is_some()
                && self.ctx.cfg.validation.has_validation())
        {
            return TokenStream::new();
        }

        let el = &self.el;
        let or_panic = self.ctx.names.get(IntegralName::OrPanic);
        if !self.ctx.cfg.validation.has_validation() {
            let constructed = self.construct_self(quote! { it });
            return quote! {
                #[must_use]
                #[inline(always)]
                pub(self) const fn #or_panic(it: #el) -> Self {
                    return #constructed;
                }
            };
        }

        let konst = Konst::konst(self.ctx.cfg.helper_konst());
        let value = quote! { it };
        let validation = self.ctx.cfg.validation.emit(&value);
        let constructed = self.construct_self(value);
        return quote! {
            #[must_use]
            #[inline(always)]
            pub(self) #konst fn #or_panic(it: #el) -> Self {
                if #validation {
                    return #constructed;
                }
                else {
                    ::core::panic!("invalid value");
                };
            }
        };
    }

    /// `_or_err`: an operation's `Option` of a raw result, validated. Its
    /// error is `Some(raw)` for an invalid result and `None` when there is
    /// none. Only emitted when operations report failure.
    fn make_or_err(&self) -> TokenStream {
        let Some(failure) = &self.ctx.failure
        else {
            return TokenStream::new();
        };
        let el = &self.el;
        let or_err = self.ctx.names.get(IntegralName::OrErr);
        let konst = Konst::konst(self.ctx.cfg.helper_konst());
        let value = quote! { it };
        let guard = self.ctx.cfg.validation.has_validation().then(|| {
            let validation = self.ctx.cfg.validation.emit(&value);
            return quote! { if #validation };
        });
        let constructed = self.construct_self(value);
        return quote! {
            #[inline(always)]
            pub(self) #konst fn #or_err(
                it: ::core::option::Option<#el>,
            ) -> ::core::result::Result<Self, #failure> {
                return match it {
                    ::core::option::Option::Some(it) #guard => {
                        ::core::result::Result::Ok(#constructed)
                    }
                    it => ::core::result::Result::Err(it),
                };
            }
        };
    }

    /// The constructor of results a closure proof shows valid (`_proven`).
    fn make_proven(&self) -> TokenStream {
        if !self.proofs_available() {
            return TokenStream::new();
        }

        let el = &self.el;
        let value = quote! { it };
        let validation = self.ctx.cfg.validation.emit(&value);
        let constructed = self.construct_self(value);
        return quote! {
            /// Builds a result that an expansion-time closure proof
            /// shows valid; debug builds still check it.
            #[must_use]
            #[inline(always)]
            pub(self) const fn _proven(it: #el) -> Self {
                ::core::debug_assert!(
                    #validation,
                    "typekin closure proof violated",
                );
                return #constructed;
            }
        };
    }

    /// The unvalidated constructor (`_blind_trust`), for `with_blind_trust`
    /// or an `unsafe` friend.
    fn make_blind_trust(&self) -> TokenStream {
        let with_blind_trust =
            self.ctx.cfg.flags.has(IntegralFlag::WithBlindTrust);
        if !with_blind_trust && !self.has_friend_with(Capability::Unsafe) {
            return TokenStream::new();
        }

        let el = &self.el;
        let blind_trust = self.ctx.names.get(IntegralName::BlindTrust);
        let constructed = self.target.construct(
            &self.storage,
            quote! { Self },
            quote! { raw },
            self.ctx.cfg.flags.has(IntegralFlag::Assertions),
        );
        let visibility = crate::zz_hide::blind_trust_visibility(
            &self.ctx.cfg.hide_in,
            with_blind_trust,
        );
        return quote! {
            /// Builds the wrapper **without validation**; the caller
            /// vouches for `raw`. Zero still cannot enter `NonZero`
            /// storage.
            #[doc(hidden)]
            #[must_use]
            #[inline(always)]
            #visibility const fn #blind_trust(raw: #el) -> Self {
                return #constructed;
            }
        };
    }
}
