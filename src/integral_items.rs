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
    Translation,
    translate,
};
use crate::integral_compute::{
    Body,
    OpRef,
};
use crate::zz_friend::{
    Capability,
    Friend,
    any_friend_has,
};
use crate::zz_lib::{
    Check,
    FnAttrs,
    Konst,
    Konsts,
    Named as _,
    OnInvalid,
    Output,
    unwrap_or_panic,
};
use crate::zz_profile_integral::{
    RhsKind,
    Slot,
};
use crate::zz_runner::Merged;
use crate::zz_values::{
    F,
    N,
    Num,
    OPS,
    OpKind,
    OpSpec,
    Width,
    by_width_grouped,
};

/// Bit-access part types after the halves, in output order.
const BIT_PARTS: [N; 4] = [
    N::U008,
    N::U016,
    N::U032,
    N::U064,
];

impl Maker {
    /// `pub(self) #konst fn #name(&self, [rhs: #rhs,])`: binds the raw
    /// `lhs`, computes `body` from it (and `rhs`) and constructs the
    /// result. A helper's output is free: where a failure may not panic it
    /// is `_or_err`'s `Result`.
    fn validating_helper(
        &self,
        konst: &Option<TokenStream>,
        name: &Ident,
        rhs: Option<&TokenStream>,
        body: &Body,
    ) -> TokenStream {
        let panic_at = self.ctx.cfg.wide.panic_at;
        let Some(translation) = translate(body.fails, panic_at, false)
        else {
            return TokenStream::new();
        };
        let get_raw = &self.ctx.get_raw;
        let rhs = rhs.map(|rhs| quote! { rhs: #rhs, });
        let (output, returned) =
            match self.passed_failure(body.fails, translation) {
                Some(failure) => (
                    quote! { ::core::result::Result<Self, #failure> },
                    Output::MustUseType,
                ),
                None => (quote! { Self }, Output::Value),
            };
        let FnAttrs {
            must_use,
            track_caller,
        } = FnAttrs::of(self.form_panics(body.panics, translation), returned);
        let expression = &body.tokens;
        let result = self.constructed(body.fails, translation, quote! { it });
        return quote! {
            #must_use
            #[inline(always)]
            #track_caller
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

    /// The validating `_{method}` helper of a binary operator: the body of
    /// every impl of `slot`'s row. A shift takes its amount as `usize`. The
    /// arithmetic of `Wrapping` adds the non-const `_wrapping` twin, and a
    /// proof for same-type operands only adds the `_proven` sibling of the
    /// wrapper's own right-hand side.
    fn make_fn_math_binary(
        &self,
        operator: &OpSpec,
        slot: &Slot,
    ) -> TokenStream {
        let el = &self.el;
        let rhs_type = match operator.kind == OpKind::Shift {
            true => quote! { usize },
            false => quote! { #el },
        };
        let same = Slot {
            rhs: RhsKind::Same,
            ..slot.clone()
        };
        let mut stream = TokenStream::new();
        for konst in [
            true,
            false,
        ] {
            if !konst && !self.has_wrapping_helper(operator, slot) {
                continue;
            }
            let row = |slot| OpRef::Operator {
                spec: operator,
                slot,
                konst,
            };
            let helper_konst =
                Konst::konst(konst && self.ctx.cfg.helper_konst());
            let name = self.helper_name(operator, slot, konst);
            stream.extend(self.validating_helper(
                &helper_konst,
                &name,
                Some(&rhs_type),
                &self.compute(row(slot)),
            ));
            let sibling = self.helper_name(operator, &same, konst);
            if sibling != name {
                stream.extend(self.validating_helper(
                    &helper_konst,
                    &sibling,
                    Some(&rhs_type),
                    &self.compute(row(&same)),
                ));
            }
        }
        return stream;
    }

    /// The validating helpers that the operator and assignment impls call,
    /// in `OPS` order: those of each row with its `fn_math_*` flag on whose
    /// slots call a helper. Any raw value reaches a helper, so its body is
    /// that of the row's last slot (a friend's, or a shift's amount).
    pub(crate) fn make_fn_math_helpers(&self) -> TokenStream {
        let cfg = &self.ctx.cfg;
        return OPS
            .iter()
            .filter_map(|operator| {
                let slot = cfg.slots.of(operator.op).last()?;
                let helper = IntegralFlag::of_op(operator.op).helper;
                if slot.is_facet() || !cfg.flags.has(helper) {
                    return None;
                }
                return Some(match operator.kind {
                    OpKind::Arith | OpKind::Bitwise | OpKind::Shift => {
                        self.make_fn_math_binary(operator, slot)
                    }
                    OpKind::Unary => self.make_fn_math_unary(operator, slot),
                });
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

        // `of` returns the type itself: where a friend's invalid value may
        // not panic, it gives way to `try_of`.
        if self.fixed_form().is_none() {
            let el = &self.el;
            let fn_try_of = self.ctx.names.get(IntegralName::TryOfFn);
            let FnAttrs { track_caller, .. } =
                FnAttrs::of(self.of_panics(), Output::MustUseType);
            return quote! {
                #[inline(always)]
                #track_caller
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
        let FnAttrs {
            must_use,
            track_caller,
        } = FnAttrs::of(self.of_panics(), Output::Value);

        return quote! {
            #must_use
            #[inline(always)]
            #track_caller
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
        let FnAttrs {
            must_use,
            track_caller,
        } = FnAttrs::of(self.validation_panics(), Output::Value);
        return quote! {
            #must_use
            #[inline(always)]
            #track_caller
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

    /// `try_map`, and `map` where an invalid mapped value may panic: `map`
    /// returns `Self`, and ingress keeps its own panic message.
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
        let map = self.fixed_form().map(|_| {
            let mapped = unwrap_or_panic(
                quote! { self.try_map(function) },
                "mapping operation produced an invalid value",
            );
            let FnAttrs {
                must_use,
                track_caller,
            } = FnAttrs::of(self.validation_panics(), Output::Value);
            quote! {
                #must_use
                #[inline(always)]
                #track_caller
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
        let FnAttrs { track_caller, .. } =
            FnAttrs::of(self.storage.write_panics(), Output::MustUseType);
        return quote! {
            #[inline(always)]
            #track_caller
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

    /// The validating `_neg` or `_not` helper, which only its operator impl
    /// calls. A result that cannot fail runs no validation, so its helper
    /// is `const` whatever the configuration.
    fn make_fn_math_unary(
        &self,
        operator: &OpSpec,
        slot: &Slot,
    ) -> TokenStream {
        let body = self.compute(OpRef::Operator {
            spec: operator,
            slot,
            konst: true,
        });
        let konst = Konst::konst(!body.fails || self.ctx.cfg.helper_konst());
        return self.validating_helper(
            &konst,
            &format_ident!("_{}", operator.method),
            None,
            &body,
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
            let FnAttrs { track_caller, .. } =
                FnAttrs::of(self.validation_panics(), Output::MustUseType);
            return quote! {
                #[inline(always)]
                #track_caller
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
        let track_caller = self.track_write();
        return quote! {
            #[inline(always)]
            #track_caller
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

    /// The constructor a failure panics in (`_or_panic`): it validates and
    /// panics with the translation's message, or just constructs where
    /// there is nothing to validate. Its output is fixed, so a validated
    /// type has none where a failure may not panic: `_or_err` validates
    /// instead.
    pub(crate) fn make_or_panic(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::FnOrPanic) {
            return TokenStream::new();
        }

        let el = &self.el;
        let or_panic = self.ctx.names.get(IntegralName::OrPanic);
        let konst = Konst::konst(self.ctx.cfg.helper_konst());
        let fails = self.ctx.cfg.validation.has_validation();
        let Some(translation) =
            translate(fails, self.ctx.cfg.wide.panic_at, true)
        else {
            return TokenStream::new();
        };
        let body = match translation {
            Translation::Unwrap(message) => self.construction(
                quote! { it },
                Check::Validate,
                OnInvalid::Panic(message),
            ),
            Translation::Pass => {
                let constructed = self.construct_self(quote! { it });
                quote! { return #constructed; }
            }
        };
        let FnAttrs {
            must_use,
            track_caller,
        } = FnAttrs::of(self.form_panics(false, translation), Output::Value);
        return quote! {
            #must_use
            #[inline(always)]
            #track_caller
            pub(self) #konst fn #or_panic(it: #el) -> Self {
                #body
            }
        };
    }

    /// `_or_err`: an operation's `Option` of a raw result, validated. Its
    /// error is `Some(raw)` for an invalid result and `None` when there is
    /// none. Only emitted where a failing operator passes its `Result` on.
    pub(crate) fn make_or_err(&self) -> TokenStream {
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
        let track_caller = self.track_write();
        return quote! {
            #[inline(always)]
            #track_caller
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
    pub(crate) fn make_proven(&self) -> TokenStream {
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
        let track_caller = self.track_write();
        return quote! {
            /// Builds a result that an expansion-time closure proof
            /// shows valid; debug builds still check it.
            #[must_use]
            #[inline(always)]
            #track_caller
            pub(self) const fn _proven(it: #el) -> Self {
                #body
            }
        };
    }

    /// The unvalidated constructor (`_blind_trust`), for `with_blind_trust`
    /// or an `unsafe` friend.
    pub(crate) fn make_blind_trust(&self) -> TokenStream {
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
        let track_caller = self.track_write();
        return quote! {
            /// Builds the wrapper **without validation**; the caller
            /// vouches for `raw`. Zero still cannot enter `NonZero`
            /// storage.
            #[doc(hidden)]
            #[must_use]
            #[inline(always)]
            #track_caller
            #visibility const fn #blind_trust(raw: #el) -> Self {
                return #constructed;
            }
        };
    }
}
