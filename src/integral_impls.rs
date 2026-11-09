use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::parse_quote;

use crate::integral::Maker;
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
use crate::zz_friend::Capability;
use crate::zz_lib::{
    Check,
    Konst,
    Konsts,
    OnInvalid,
};
use crate::zz_profile_integral::{
    RhsKind,
    Slot,
};
use crate::zz_runner::Merged;
use crate::zz_values::{
    Num,
    OPS,
    OpSpec,
};

/// A binary operator impl by value (`type Output`) or as an assignment.
#[derive(Clone, Copy)]
enum OpForm {
    Value,
    Assign,
}

/// What one binary operator impl calls and how it is marked.
struct OpImpl {
    /// `Add`, `AddAssign`, `Shr`, `BitXorAssign`, …
    trait_name: Ident,
    /// `add`, `add_assign`, …
    method: Ident,
    /// `konst_for` the operator's flags in its form.
    is_konst: bool,
    /// The resolved helper: `_add`, `_add_wrapping`, `_bitor_proven`, …
    helper: Ident,
    /// The proof doc, or empty.
    doc: TokenStream,
}

/// How a binary operator impl that calls a helper takes its right-hand
/// side.
struct OpRhs {
    /// The parameter's type.
    ty: TokenStream,
    /// The statement that binds a converted operand, or empty.
    prelude: TokenStream,
    /// The operand the helper takes.
    operand: TokenStream,
    /// The trait's `<Rhs>`; empty for the wrapper itself.
    generic: TokenStream,
}

/// The integral `core::fmt` traits, in output order.
const FMT_TRAITS: [(IntegralFlag, &str); 5] = [
    (IntegralFlag::Display, "Display"),
    (IntegralFlag::FmtBinary, "Binary"),
    (IntegralFlag::FmtOctal, "Octal"),
    (IntegralFlag::FmtHexLower, "LowerHex"),
    (IntegralFlag::FmtHexUpper, "UpperHex"),
];

impl Maker {
    /// The integral `core::fmt` traits.
    pub(crate) fn make_fmt(&self) -> TokenStream {
        return self.make_fmt_traits(&FMT_TRAITS);
    }

    /// The seal bound of a friend right-hand side, `[const]` under `is_konst`.
    fn seal_bound(
        &self,
        is_konst: bool,
    ) -> TokenStream {
        return Konst::bound(is_konst, self.ctx.friends.seal_with_arguments());
    }

    /// An impl of `path` over the friend right-hand side `T: #bounds`.
    fn impl_rhs_trait(
        &self,
        is_konst: bool,
        bounds: TokenStream,
        path: TokenStream,
        body: TokenStream,
    ) -> TokenStream {
        let rhs_ty = &self.ctx.rhs_ty;
        return self.impl_trait_with(
            is_konst,
            &[parse_quote! { #rhs_ty }],
            &[parse_quote! { #rhs_ty: #bounds }],
            path,
            body,
        );
    }

    /// `Step`, whose methods step the raw value by a count.
    pub(crate) fn make_range(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::Range) {
            return TokenStream::new();
        }

        let el = &self.el;
        let is_konst = self.ctx.cfg.konst_for(&[Konsts::Iterator]);
        let start = self.storage.extract(quote! { start.0 });
        let end = self.storage.extract(quote! { end.0 });
        let count = Slot::count(self.ctx.cfg.profile.arithmetic());
        let (Some(forward), Some(backward)) = (
            self.step_direction("forward", &count),
            self.step_direction("backward", &count),
        )
        else {
            return TokenStream::new();
        };

        return self.impl_trait(
            is_konst,
            quote! { ::core::iter::Step },
            quote! {
                #[inline(always)]
                fn steps_between(
                    start: &Self,
                    end: &Self,
                ) -> (usize, ::core::option::Option<usize>) {
                    return <#el as ::core::iter::Step>::steps_between(
                        &#start,
                        &#end,
                    );
                }

                #forward
                #backward
            },
        );
    }

    /// `Step`'s checked and overflowing methods in `direction` (`forward`
    /// or `backward`), each a step by `count`. `Step` fixes their outputs:
    /// `None` where a step could fail and may not panic.
    fn step_direction(
        &self,
        direction: &str,
        count: &Slot,
    ) -> Option<TokenStream> {
        let checked = format_ident!("{}_checked", direction);
        let overflowing = format_ident!("{}_overflowing", direction);
        let stepped = self.compute(OpRef::Step(&checked), count);
        let wrapped = self.compute(OpRef::Step(&overflowing), count);
        let translation =
            translate(stepped.fails, self.ctx.cfg.wide.panic_at, true)?;
        let constructed =
            self.constructed(stepped.fails, translation, quote! { value });
        let (stepped, wrapped) = (stepped.tokens, wrapped.tokens);

        return Some(quote! {
            #[inline(always)]
            fn #checked(
                start: Self,
                count: usize,
            ) -> ::core::option::Option<Self> {
                return match #stepped {
                    ::core::option::Option::Some(value) => {
                        ::core::option::Option::Some(#constructed)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }

            #[inline(always)]
            fn #overflowing(
                start: Self,
                count: usize,
            ) -> (Self, bool) {
                let (value, overflowed) = #wrapped;
                return (#constructed, overflowed);
            }
        });
    }

    /// `AsRef<raw>`, which `NonZero` storage lacks (`Cfg::settle`).
    pub(crate) fn make_as_ref(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::AsRef) {
            return TokenStream::new();
        }

        let el = &self.el;
        let self_raw_ref = self.storage.extract_ref(quote! { self.0 });
        let path = quote! { ::core::convert::AsRef<#el> };
        return self.impl_trait(
            false,
            path,
            quote! {
                #[inline(always)]
                fn as_ref(&self) -> &#el {
                    return #self_raw_ref;
                }
            },
        );
    }

    pub(crate) fn make_from_str(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::FromStr) {
            return TokenStream::new();
        }

        let el = &self.el;
        let result = self.construction(
            quote! { value },
            Check::Validate,
            OnInvalid::Err(quote! { () }),
        );
        return self.impl_trait(
            false,
            quote! { ::core::str::FromStr },
            quote! {
                type Err = ();

                #[inline(always)]
                fn from_str(
                    source: &str,
                ) -> ::core::result::Result<Self, Self::Err> {
                    let value =
                        match <#el as ::core::str::FromStr>::from_str(source) {
                            ::core::result::Result::Ok(value) => value,
                            ::core::result::Result::Err(_) => {
                                return ::core::result::Result::Err(());
                            }
                        };

                    #result
                }
            },
        );
    }

    pub(crate) fn make_serde(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::Serde) {
            return TokenStream::new();
        }

        let el = &self.el;
        let get_raw = &self.ctx.get_raw;
        let serializer = self.target.fresh_ident("S");
        let deserializer = self.target.fresh_ident("D");
        let de = self.target.fresh_lifetime("de");
        let result = self.construction(
            quote! { value },
            Check::Validate,
            OnInvalid::Err(quote! {
                <#deserializer::Error as ::serde::de::Error>::custom(
                    "invalid value",
                )
            }),
        );
        let serialize = quote! {
            #[inline(always)]
            fn serialize<#serializer>(
                &self,
                serializer: #serializer,
            ) -> ::core::result::Result<#serializer::Ok, #serializer::Error>
            where
                #serializer: ::serde::Serializer,
            {
                let value = #get_raw(*self);
                return <#el as ::serde::Serialize>::serialize(
                    &value,
                    serializer,
                );
            }
        };
        let deserialize = quote! {
            #[inline(always)]
            fn deserialize<#deserializer>(
                deserializer: #deserializer,
            ) -> ::core::result::Result<Self, #deserializer::Error>
            where
                #deserializer: ::serde::Deserializer<#de>,
            {
                let value =
                    <#el as ::serde::Deserialize<#de>>::deserialize(
                        deserializer,
                    )?;

                #result
            }
        };

        let mut stream =
            self.impl_trait(false, quote! { ::serde::Serialize }, serialize);
        stream.extend(self.impl_trait_with(
            false,
            &[parse_quote! { #de }],
            &[],
            quote! { ::serde::Deserialize<#de> },
            deserialize,
        ));
        return stream;
    }

    /// `Into` (`safe`, the lossless targets) or `TryInto` (the lossy ones)
    /// for each primitive cast target, under its pointer-width `#cfg`.
    pub(crate) fn make_cast_impls(
        &self,
        safe: bool,
    ) -> TokenStream {
        let (flag, konsts) = match safe {
            true => (
                IntegralFlag::Into,
                [
                    Konsts::Into,
                    Konsts::Conversion,
                ],
            ),
            false => (
                IntegralFlag::TryInto,
                [
                    Konsts::TryInto,
                    Konsts::Conversion,
                ],
            ),
        };
        if !self.ctx.cfg.flags.has(flag) {
            return TokenStream::new();
        }

        let is_konst = self.ctx.cfg.konst_for(&konsts);

        return self
            .cast_targets(safe)
            .map(|(it, cfg)| {
                let target = format_ident!("{}", it.name());
                let (path, body) = match safe {
                    true => {
                        let conv = self.ctx.names.get(IntegralName::Into(it));
                        (
                            quote! { ::core::convert::Into<#target> },
                            quote! {
                                #[inline(always)]
                                fn into(self) -> #target {
                                    return Self::#conv(self);
                                }
                            },
                        )
                    }
                    false => {
                        let conv =
                            self.ctx.names.get(IntegralName::TryInto(it));
                        (
                            quote! { ::core::convert::TryInto<#target> },
                            quote! {
                                type Error = ();

                                #[inline(always)]
                                fn try_into(
                                    self,
                                ) -> ::core::result::Result<#target, Self::Error> {
                                    return Self::#conv(self);
                                }
                            },
                        )
                    }
                };
                let cast = self.impl_trait(is_konst, path, body);
                return quote! {
                    #cfg
                    #cast
                };
            })
            .merged();
    }

    pub(crate) fn make_debug(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::Debug) {
            return TokenStream::new();
        }

        let self_raw = &self.ctx.self_raw;
        let name = self.target.ident.to_string();
        return self.impl_trait(
            false,
            quote! { ::core::fmt::Debug },
            quote! {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    return f
                        .debug_tuple(#name)
                        .field(&#self_raw)
                        .finish();
                }
            },
        );
    }

    pub(crate) fn make_hash(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::Hash) {
            return TokenStream::new();
        }

        let self_raw = &self.ctx.self_raw;
        let hasher = self.target.fresh_ident("H");
        // Equal floats hash alike: `-0.0 + 0.0` is `+0.0`. The addition is
        // left out when the domain admits at most one zero.
        let hashed = match self.num {
            Num::Int(_) => quote! { #self_raw },
            Num::Float(_) => match self
                .ctx
                .cfg
                .validation
                .float_domain()
                .is_none_or(|it| it.admits_both_zeros())
            {
                true => quote! { (#self_raw + 0.0).to_bits() },
                false => quote! { #self_raw.to_bits() },
            },
        };
        return self.impl_trait(
            false,
            quote! { ::core::hash::Hash },
            quote! {
                #[inline(always)]
                fn hash<#hasher>(
                    &self,
                    state: &mut #hasher,
                )
                where
                    #hasher: ::core::hash::Hasher,
                {
                    return ::core::hash::Hash::hash(&#hashed, state);
                }
            },
        );
    }

    pub(crate) fn make_eq(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::Eq) {
            return TokenStream::new();
        }

        let is_konst = self.ctx.cfg.konst_for(&[
            Konsts::Eq,
            Konsts::Comparison,
        ]);
        // Not a derive: a derive cannot emit `const impl` under `konst`,
        // and it would bound the generic parameters.
        return self.impl_trait(
            is_konst,
            quote! { ::core::cmp::Eq },
            TokenStream::new(),
        );
    }

    pub(crate) fn make_ord(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::Ord) {
            return TokenStream::new();
        }

        let is_konst = self.ctx.cfg.konst_for(&[
            Konsts::Ord,
            Konsts::Comparison,
        ]);
        let body = match self.num {
            Num::Int(_) => quote! {
                return self.partial_cmp(that).unwrap();
            },
            // NaN-free: no panic path, const on every konst mode.
            Num::Float(_) => {
                let get_raw = &self.ctx.get_raw;
                let compared = Self::float_cmp();
                quote! {
                    let lhs = #get_raw(*self);
                    let rhs = #get_raw(*that);
                    #compared
                }
            }
        };
        return self.impl_trait(
            is_konst,
            quote! { ::core::cmp::Ord },
            quote! {
                #[inline(always)]
                fn cmp(&self, that: &Self) -> ::core::cmp::Ordering {
                    #body
                }
            },
        );
    }

    pub(crate) fn make_assertions(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::Assertions) {
            return TokenStream::new();
        }

        let ty = &self.ctx.ty;
        let assertions = self.storage.layout_assertions(quote! { #ty });
        if !self.target.has_generics() {
            return assertions;
        }
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let name = self.target.layout_assertion();
        return quote! {
            impl #impl_generics #ty #where_clause {
                const #name: () = {
                    #assertions
                };
            }
        };
    }

    /// The operator impls: rows in `OPS` order, each row's slots in slot
    /// order, each slot's forms in form order.
    pub(crate) fn make_operators(&self) -> TokenStream {
        return OPS
            .iter()
            .map(|operator| self.make_operator(operator))
            .merged();
    }

    /// One operator's impls: for each slot, the operator and then its
    /// assignment. A row that facets dispatch starts with the `core::ops`
    /// impl forwarding to its `{Op}Rhs` trait.
    fn make_operator(
        &self,
        operator: &OpSpec,
    ) -> TokenStream {
        let slots = self.ctx.cfg.slots.of(operator.op);
        let row = OpRef::Operator {
            spec: operator,
            konst: self.ctx.cfg.konst_for(&operator.konsts()),
        };
        let mut stream = self.facet_operator_impl(operator, slots);
        for slot in slots {
            let body = self.compute(row, slot);
            let forms = self.slot_forms(slot, body.fails);
            for (form, translation) in forms.into_iter().flatten() {
                stream.extend(self.make_form(
                    operator,
                    slot,
                    form,
                    translation,
                    &body,
                ));
            }
        }
        return stream;
    }

    /// The forms `slot` gets, in form order, each with what it does with a
    /// failing body: the operator, whose `Output` is free, then the
    /// assignment, whose output is fixed.
    fn slot_forms(
        &self,
        slot: &Slot,
        fails: bool,
    ) -> [Option<(OpForm, Translation)>; 2] {
        let panic_at = self.ctx.cfg.wide.panic_at;
        let form = |on: bool, form: OpForm, fixed_output: bool| {
            let translation = translate(fails, panic_at, fixed_output)?;
            return on.then_some((form, translation));
        };
        return [
            form(slot.regular, OpForm::Value, false),
            form(slot.assign, OpForm::Assign, true),
        ];
    }

    /// One form of `operator` for `slot`. A facet slot's `{Op}Rhs` impl
    /// holds the body and its assignment forwards to that impl; every other
    /// impl calls the operator's helper, which holds the body.
    fn make_form(
        &self,
        operator: &OpSpec,
        slot: &Slot,
        form: OpForm,
        translation: Translation,
        body: &Body,
    ) -> TokenStream {
        return match (&slot.rhs, form) {
            (RhsKind::Facet { .. }, OpForm::Value) => {
                self.facet_rhs_impl(operator, slot, translation, body)
            }
            (RhsKind::Facet { .. }, OpForm::Assign) => {
                self.facet_assign_impl(operator, slot)
            }
            (RhsKind::Unary, _) => {
                self.make_math_unary(operator, slot, translation, body.fails)
            }
            (RhsKind::Same | RhsKind::Friend(_) | RhsKind::Amount, _) => {
                self.make_op_impl(operator, slot, form, translation, body.fails)
            }
        };
    }

    /// `operator`'s impl in `form` for `slot`. Its `konst_*` flags are the
    /// operator's, with an assignment's as `[XAssign, op, Assign, family]`;
    /// the helper and the doc follow the slot's proof. A shift's helper
    /// takes any amount, so its doc reflects the full proof.
    fn binary_op(
        &self,
        operator: &OpSpec,
        slot: &Slot,
        form: OpForm,
    ) -> OpImpl {
        let (trait_name, method, is_konst) = match form {
            OpForm::Value => (
                format_ident!("{}", operator.trait_name),
                format_ident!("{}", operator.method),
                self.ctx.cfg.konst_for(&operator.konsts()),
            ),
            OpForm::Assign => (
                format_ident!("{}Assign", operator.trait_name),
                format_ident!("{}_assign", operator.method),
                self.ctx.cfg.konst_for(&operator.assign_konsts()),
            ),
        };
        return OpImpl {
            trait_name,
            method,
            is_konst,
            helper: self.helper_name(operator, slot, is_konst),
            doc: self.proof_doc(self.slot_proven(operator, slot)),
        };
    }

    /// How the impls of `slot` take the right-hand side named `param`:
    /// `Trait for Ty` and `rhs: Self` for the wrapper itself; `Trait<T> for
    /// Ty` for a friend, bound as `it` first; `Trait<usize> for Ty` for a
    /// shift. `None` for a facet slot, which computes in its `{Op}Rhs`
    /// impl, and for a unary operator.
    fn op_rhs(
        &self,
        slot: &Slot,
        param: &Ident,
    ) -> Option<OpRhs> {
        return match &slot.rhs {
            RhsKind::Same => Some(OpRhs {
                ty: quote! { Self },
                prelude: TokenStream::new(),
                operand: self.ctx.rhs_raw.clone(),
                generic: TokenStream::new(),
            }),
            RhsKind::Friend(_) => {
                let rhs_ty = &self.ctx.rhs_ty;
                let conversion = self.ctx.friends.conversion_path();
                Some(OpRhs {
                    ty: quote! { #rhs_ty },
                    prelude: quote! { let it = #conversion(rhs); },
                    operand: quote! { it },
                    generic: quote! { <#rhs_ty> },
                })
            }
            RhsKind::Amount => Some(OpRhs {
                ty: quote! { usize },
                prelude: TokenStream::new(),
                operand: quote! { #param },
                generic: quote! { <usize> },
            }),
            RhsKind::Facet { .. } | RhsKind::Unary => None,
        };
    }

    /// One binary operator impl that calls the operator's helper: `type
    /// Output` (value form), the doc, then the method. A shift's parameter
    /// is `rhs` (value) or `that` (assignment).
    fn make_op_impl(
        &self,
        operator: &OpSpec,
        slot: &Slot,
        form: OpForm,
        translation: Translation,
        fails: bool,
    ) -> TokenStream {
        let param = match (&slot.rhs, form) {
            (RhsKind::Amount, OpForm::Assign) => format_ident!("that"),
            _ => format_ident!("rhs"),
        };
        let Some(rhs) = self.op_rhs(slot, &param)
        else {
            return TokenStream::new();
        };
        let it = self.binary_op(operator, slot, form);
        let (method, helper, doc) = (&it.method, &it.helper, &it.doc);
        let (param_ty, prelude, operand) =
            (&rhs.ty, &rhs.prelude, &rhs.operand);
        let call = quote! { self.#helper(#operand) };

        let body = match form {
            OpForm::Value => {
                let output = self.output_of(slot, false, fails, translation);
                let output_ty = output.ty();
                let result = output.convert(call);
                quote! {
                    type Output = #output_ty;

                    #doc
                    #[inline(always)]
                    fn #method(
                        self,
                        #param: #param_ty,
                    ) -> Self::Output {
                        #prelude
                        return #result;
                    }
                }
            }
            OpForm::Assign => quote! {
                #doc
                #[inline(always)]
                fn #method(
                    &mut self,
                    #param: #param_ty,
                ) {
                    #prelude
                    *self = #call;
                }
            },
        };
        return self.impl_op(&it, slot, &rhs, body);
    }

    /// `body` as one binary operator impl; a friend right-hand side adds
    /// its capability's bound and the seal bound.
    fn impl_op(
        &self,
        it: &OpImpl,
        slot: &Slot,
        rhs: &OpRhs,
        body: TokenStream,
    ) -> TokenStream {
        let trait_name = &it.trait_name;
        let generic = &rhs.generic;
        let path = quote! { ::core::ops::#trait_name #generic };
        let RhsKind::Friend(capability) = slot.rhs
        else {
            return self.impl_trait(it.is_konst, path, body);
        };
        let friend = self.ctx.friends.bound(capability);
        let seal = self.seal_bound(it.is_konst);
        return self.impl_rhs_trait(
            it.is_konst,
            quote! { #friend + #seal },
            path,
            body,
        );
    }

    /// A unary operator trait forwarding to its validating `_{method}`
    /// helper.
    fn make_math_unary(
        &self,
        operator: &OpSpec,
        slot: &Slot,
        translation: Translation,
        fails: bool,
    ) -> TokenStream {
        let output = self.output_of(slot, false, fails, translation);
        let is_konst = self.ctx.cfg.konst_for(&operator.konsts());
        let output_ty = output.ty();
        let trait_name = format_ident!("{}", operator.trait_name);
        let method = format_ident!("{}", operator.method);
        let helper = format_ident!("_{}", method);
        let result = output.convert(quote! { self.#helper() });
        let doc = self.proof_doc(self.slot_proven(operator, slot));
        return self.impl_trait(
            is_konst,
            quote! { ::core::ops::#trait_name },
            quote! {
                type Output = #output_ty;

                #doc
                #[inline(always)]
                fn #method(self) -> Self::Output {
                    return #result;
                }
            },
        );
    }

    /// The `core::fmt` traits whose flag is on, each formatting the raw
    /// value: `(flag, trait name)` rows.
    pub(crate) fn make_fmt_traits(
        &self,
        traits: &[(IntegralFlag, &str)],
    ) -> TokenStream {
        let get_raw = &self.ctx.get_raw;

        let mut stream = TokenStream::new();

        for &(flag, name) in traits {
            if !self.ctx.cfg.flags.has(flag) {
                continue;
            }
            let format = format_ident!("{}", name);
            let path = quote! { ::core::fmt::#format };
            stream.extend(self.impl_trait(
                false,
                path,
                quote! {
                    #[inline(always)]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let raw = #get_raw(*self);
                        return ::core::fmt::#format::fmt(&raw, f);
                    }
                },
            ));
        }

        return stream;
    }

    pub(crate) fn make_partial_eq(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::PartialEq) {
            return TokenStream::new();
        }

        let rhs_ty = &self.ctx.rhs_ty;
        let fp_friend_conv = self.ctx.friends.conversion_path();
        let get_raw = &self.ctx.get_raw;
        let trait_friend_rel = self.ctx.friends.bound(Capability::Rel);
        let is_konst = self.ctx.cfg.konst_for(&[
            Konsts::PartialEq,
            Konsts::Comparison,
        ]);
        let bounds = self.seal_bound(is_konst);
        return self.impl_rhs_trait(
            is_konst,
            quote! { #trait_friend_rel + #bounds + ::core::marker::Copy },
            quote! { ::core::cmp::PartialEq<#rhs_ty> },
            quote! {
                #[inline(always)]
                fn eq(
                    &self,
                    rhs: &#rhs_ty,
                ) -> bool {
                    let lhs = #get_raw(*self);
                    let rhs = #fp_friend_conv(*rhs);
                    return lhs == rhs;
                }
            },
        );
    }

    pub(crate) fn make_partial_ord(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::PartialOrd) {
            return TokenStream::new();
        }

        let rhs_ty = &self.ctx.rhs_ty;
        let fp_friend_conv = self.ctx.friends.conversion_path();
        let get_raw = &self.ctx.get_raw;
        let trait_friend_rel = self.ctx.friends.bound(Capability::Rel);
        let is_konst = self.ctx.cfg.konst_for(&[
            Konsts::PartialOrd,
            Konsts::Comparison,
        ]);
        let bounds = self.seal_bound(is_konst);
        let comparisons = [
            // The defaults go through `partial_cmp`; for floats LLVM does not
            // fold that back into one comparison.
            ("lt", quote! { < }),
            ("le", quote! { <= }),
            ("gt", quote! { > }),
            ("ge", quote! { >= }),
        ]
        .into_iter()
        .map(|(name, symbol)| {
            let method = format_ident!("{}", name);
            return quote! {
                #[inline(always)]
                fn #method(
                    &self,
                    rhs: &#rhs_ty,
                ) -> bool {
                    return #get_raw(*self) #symbol #fp_friend_conv(*rhs);
                }
            };
        })
        .merged();

        return self.impl_rhs_trait(
            is_konst,
            quote! { #trait_friend_rel + #bounds + ::core::marker::Copy },
            quote! { ::core::cmp::PartialOrd<#rhs_ty> },
            quote! {
                #[inline(always)]
                fn partial_cmp(
                    &self,
                    rhs: &#rhs_ty,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let lhs = #get_raw(*self);
                    let rhs = #fp_friend_conv(*rhs);
                    return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
                }

                #comparisons
            },
        );
    }
}
