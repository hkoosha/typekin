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
use crate::integral_proof::OperatorProof;
use crate::zz_friend::{
    Capability,
    Friend,
    any_friend_has,
};
use crate::zz_lib::{
    Check,
    Konst,
    Konsts,
    Named as _,
    OnInvalid,
    unwrap_or_panic,
};
use crate::zz_profile_integral::Arithmetic;
use crate::zz_runner::Merged;
use crate::zz_values::{
    F,
    N,
    Num,
    OPS,
    Op,
    OpKind,
    OpSpec,
    Width,
    by_width_grouped,
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

    /// `operator` on the raw operands: panicking, or an `Option` of the
    /// result when operations report failure.
    fn operator_expr(
        &self,
        operator: &OpSpec,
        is_konst: bool,
    ) -> TokenStream {
        let arithmetic = self.ctx.cfg.profile.arithmetic();
        if self.ctx.failure.is_some() {
            return arithmetic.make_fallible_expr(
                operator,
                self.ctx.cfg.wide.panic_at,
                matches!(self.num, Num::Float(_)),
                is_konst,
            );
        }
        return arithmetic.make_operator_expr(operator, is_konst);
    }

    /// The validating `_{method}` helper of a binary operator, and its
    /// non-const `_wrapping` twin where it has one. A shift takes its amount
    /// as `usize`.
    fn make_fn_math_binary(
        &self,
        operator: &OpSpec,
    ) -> TokenStream {
        let el = &self.el;
        let rhs_type = match operator.kind == OpKind::Shift {
            true => quote! { usize },
            false => quote! { #el },
        };
        let proof = self.operator_proof(operator);
        let mut stream = self.proven_helpers(
            &Konst::konst(self.ctx.cfg.helper_konst()),
            &self.arithmetic_method_name(operator, true),
            &rhs_type,
            &self.operator_expr(operator, true),
            proof,
        );
        if !self.has_wrapping_helper(operator) {
            return stream;
        }
        stream.extend(self.proven_helpers(
            &Konst::konst(false),
            &self.arithmetic_method_name(operator, false),
            &rhs_type,
            &self.operator_expr(operator, false),
            proof,
        ));
        return stream;
    }

    /// The enabled validating helpers that an emitted operator or
    /// assignment impl calls, in `OPS` order.
    fn make_fn_math_helpers(&self) -> TokenStream {
        let flags = &self.ctx.cfg.flags;
        return OPS
            .iter()
            .filter(|operator| {
                return flags.has(IntegralFlag::of_op(operator.op).helper)
                    && (self.with_operator(operator)
                        || self.with_assignment(operator));
            })
            .map(|operator| {
                return match operator.kind {
                    OpKind::Arith | OpKind::Bitwise | OpKind::Shift => {
                        self.make_fn_math_binary(operator)
                    }
                    OpKind::Unary => self.make_fn_math_unary(operator),
                };
            })
            .merged();
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
        let bits = part.bits_for_pointer_width(Width::W64);
        let ty = format_ident!("{}", part.name());
        let shift = Literal::usize_unsuffixed(bits);
        let mask = LitInt::new(
            &format!("0x{}", "F".repeat(bits / 4)),
            Span::call_site(),
        );
        return (0..repr.bits_for_pointer_width(Width::W64) / bits)
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
                && 2 * part.bits_for_pointer_width(Width::W64)
                    == repr.bits_for_pointer_width(Width::W64);
        });
        let Some(half) = half
        else {
            return TokenStream::new();
        };

        let names = &self.ctx.names;
        let ty = format_ident!("{}", half.name());
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
                let target = format_ident!("{}", it.name());
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
        let result = self.construction(
            quote! { value },
            Check::Validate,
            OnInvalid::Err(quote! { value }),
        );
        let map = self.ctx.failure.is_none().then(|| {
            let mapped = unwrap_or_panic(
                quote! { self.try_map(function) },
                "mapping operation produced an invalid value",
            );
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
                    return #mapped;
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
                let target = format_ident!("{}", it.name());
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
        let source = format_ident!("{}", self.num.name());
        return self.cast_targets(false)
            .flat_map(|(it, cfg)| {
                let target = format_ident!("{}", it.name());
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
        let exact = |width: Width| {
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
        let lossy = |width: Width| {
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
        applies: impl Fn(Width) -> bool,
    ) -> Vec<(TokenStream, TokenStream)> {
        let literal = |width: Width| {
            let exponent = it.magnitude_bits_for_pointer_width(width);
            return match (float, exponent) {
                (F::F032, 128..) => None,
                _ => Some(crate::zz_values::pow2_literal(exponent)),
            };
        };
        let values =
            Width::ALL.map(|width| applies(width).then(|| literal(width)));
        return by_width_grouped(values)
            .into_iter()
            .filter_map(|(cfg, value)| {
                let hi = match value? {
                    Some(value) => {
                        let value =
                            syn::LitFloat::new(&value, Span::call_site());
                        quote! { #value }
                    }
                    None => quote! { ::core::primitive::f32::INFINITY },
                };
                return Some((cfg, hi));
            })
            .collect();
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
                self.ctx.cfg.wide.panic_at.checks_overflow()
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

    /// The validating `_neg` or `_not` helper, which only its operator impl
    /// calls. Floats and `Primitive` integers negate like the raw type; the
    /// other integer policies wrap or check like their binary operators. A
    /// result the domain proves valid is constructed without the check.
    fn make_fn_math_unary(
        &self,
        operator: &OpSpec,
    ) -> TokenStream {
        let symbol = operator.token();
        let expression = match (operator.op, &self.ctx.failure) {
            (Op::Neg, Some(_)) => self.fallible_neg(),
            (Op::Neg, None) => self.panicking_neg(),
            (_, Some(_)) => {
                quote! { ::core::option::Option::Some(#symbol lhs) }
            }
            (_, None) => quote! { #symbol lhs },
        };
        let closed = self.unary_proven(operator);
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
            &format_ident!("_{}", operator.method),
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
        let unsigned = format_ident!("{}", self.repr().unsigned().name());
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
        let body = self.construction(
            quote! { it },
            Check::Validate,
            OnInvalid::Err(quote! { it }),
        );
        return quote! {
            #[inline(always)]
            pub #konst fn try_make(it: #el) -> Result<Self, #el> {
                #body
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
        let konst = Konst::konst(self.ctx.cfg.helper_konst());
        let body = match self.ctx.cfg.validation.has_validation() {
            true => self.construction(
                quote! { it },
                Check::Validate,
                OnInvalid::Panic("invalid value"),
            ),
            false => {
                let constructed = self.construct_self(quote! { it });
                quote! { return #constructed; }
            }
        };
        return quote! {
            #[must_use]
            #[inline(always)]
            pub(self) #konst fn #or_panic(it: #el) -> Self {
                #body
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
        let none = quote! { ::core::option::Option::None };
        let body = self.construction(
            quote! { it },
            Check::Validate,
            OnInvalid::Err(quote! { ::core::option::Option::Some(it) }),
        );
        return quote! {
            #[inline(always)]
            pub(self) #konst fn #or_err(
                it: ::core::option::Option<#el>,
            ) -> ::core::result::Result<Self, #failure> {
                let ::core::option::Option::Some(it) = it
                else {
                    return ::core::result::Result::Err(#none);
                };
                #body
            }
        };
    }

    /// The constructor of results a closure proof shows valid (`_proven`).
    fn make_proven(&self) -> TokenStream {
        if !self.proofs_available() {
            return TokenStream::new();
        }

        let el = &self.el;
        let violated = "typekin closure proof violated";
        let body = self.construction(
            quote! { it },
            Check::DebugOnly(violated),
            OnInvalid::Panic(violated),
        );
        return quote! {
            /// Builds a result that an expansion-time closure proof
            /// shows valid; debug builds still check it.
            #[must_use]
            #[inline(always)]
            pub(self) const fn _proven(it: #el) -> Self {
                #body
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
        let constructed = self.construct_self(quote! { raw });
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
