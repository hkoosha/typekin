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
};
use crate::integral_proof::OperatorProof;
use crate::zz_friend::Capability;
use crate::zz_lib::{
    Check,
    Konst,
    Konsts,
    OnInvalid,
};
use crate::zz_runner::Merged;
use crate::zz_values::{
    Num,
    OPS,
    OpKind,
    OpSpec,
};

/// The right-hand side of a binary operator impl.
#[derive(Clone, Copy)]
enum OpRhs {
    /// `Trait for Ty`, `rhs: Self`, operand `rhs_raw`.
    Same,
    /// `Trait<T> for Ty` where `T` has the capability's friend bound and the
    /// seal bound; the operand is `let it = {conversion}(rhs);`.
    Friend(Capability),
    /// `Trait<usize> for Ty`; parameter `rhs` (value) or `that` (assign).
    Amount,
}

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

/// The integral `core::fmt` traits, in output order.
const FMT_TRAITS: [(IntegralFlag, &str); 5] = [
    (IntegralFlag::Display, "Display"),
    (IntegralFlag::FmtBinary, "Binary"),
    (IntegralFlag::FmtOctal, "Octal"),
    (IntegralFlag::FmtHexLower, "LowerHex"),
    (IntegralFlag::FmtHexUpper, "UpperHex"),
];

impl Maker {
    /// The impls before the `core` forwards: `AsRef`, `FromStr`, serde.
    pub(crate) fn impls_head(&self) -> TokenStream {
        let mut stream = TokenStream::new();
        stream.extend(self.make_as_ref());
        stream.extend(self.make_from_str());
        stream.extend(self.make_serde());
        return stream;
    }

    /// The impls after the forwards, in output order: conversions, the
    /// derive-like traits, operators, and formatting.
    pub(crate) fn impls_body(&self) -> TokenStream {
        let mut stream = TokenStream::new();
        stream.extend(self.make_cast_impls(true));
        stream.extend(self.make_cast_impls(false));
        stream.extend(self.make_range());
        stream.extend(self.make_debug());
        stream.extend(self.make_hash());
        stream.extend(self.make_eq());
        stream.extend(self.make_ord());
        stream.extend(self.make_assertions());
        stream.extend(self.make_operators());
        stream.extend(self.make_fmt_traits(&FMT_TRAITS));
        stream.extend(self.make_partial_eq());
        stream.extend(self.make_partial_ord());
        return stream;
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

    pub(crate) fn make_range(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::Range) {
            return TokenStream::new();
        }

        let el = &self.el;
        let is_konst = self.ctx.cfg.konst_for(&[Konsts::Iterator]);
        let start = self.storage.extract(quote! { start.0 });
        let end = self.storage.extract(quote! { end.0 });
        let forward = self.step_direction("forward", &start);
        let backward = self.step_direction("backward", &start);

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
    /// or `backward`), stepping the raw `start`.
    fn step_direction(
        &self,
        direction: &str,
        start: &TokenStream,
    ) -> TokenStream {
        let el = &self.el;
        let or_panic = &self.ctx.fp_or_panic;
        let checked = format_ident!("{}_checked", direction);
        let overflowing = format_ident!("{}_overflowing", direction);

        return quote! {
            #[inline(always)]
            fn #checked(
                start: Self,
                count: usize,
            ) -> ::core::option::Option<Self> {
                return match <#el as ::core::iter::Step>::#checked(
                    #start,
                    count,
                ) {
                    ::core::option::Option::Some(value) => {
                        ::core::option::Option::Some(#or_panic(value))
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
                let (value, overflowed) =
                    <#el as ::core::iter::Step>::#overflowing(
                        #start,
                        count,
                    );
                return (#or_panic(value), overflowed);
            }
        };
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

    /// The operator impls, in `OPS` order.
    pub(crate) fn make_operators(&self) -> TokenStream {
        return OPS
            .iter()
            .map(|operator| self.make_operator(operator))
            .merged();
    }

    /// `operator`'s impls are generated: its `math_*` flag is on and no
    /// facet dispatches it instead.
    pub(crate) fn with_operator(
        &self,
        operator: &OpSpec,
    ) -> bool {
        let flags = IntegralFlag::of_op(operator.op);
        return self.ctx.cfg.flags.has(flags.math)
            && !self.facet_dispatched(operator);
    }

    /// `operator`'s assignment impls are generated: its `assign_*` flag is
    /// on and no facet dispatches it instead. Configuration turns the flag
    /// off for a non-`Self` output and when operations report failure.
    pub(crate) fn with_assignment(
        &self,
        operator: &OpSpec,
    ) -> bool {
        let flags = IntegralFlag::of_op(operator.op);
        return flags
            .assign
            .is_some_and(|flag| self.ctx.cfg.flags.has(flag))
            && !self.facet_dispatched(operator);
    }

    /// Facets, when present, dispatch the operators that take a right-hand
    /// value; shifts and unary operators are generated either way.
    fn facet_dispatched(
        &self,
        operator: &OpSpec,
    ) -> bool {
        return operator.kind.has_operand() && !self.with_global_operators();
    }

    /// One operator's impls: for each right-hand side, the operator and
    /// then its assignment.
    fn make_operator(
        &self,
        operator: &OpSpec,
    ) -> TokenStream {
        if operator.kind == OpKind::Unary {
            return self.make_math_unary(operator);
        }
        let forms = [
            self.with_operator(operator).then_some(OpForm::Value),
            self.with_assignment(operator).then_some(OpForm::Assign),
        ];
        if forms.iter().all(Option::is_none) {
            return TokenStream::new();
        }

        let proof = self.operator_proof(operator);
        let mut stream = TokenStream::new();
        for rhs in self.operator_rhs(operator).into_iter().flatten() {
            for form in forms.into_iter().flatten() {
                let it = self.binary_op(operator, proof, rhs, form);
                stream.extend(self.make_op_impl(&it, rhs, form));
            }
        }
        return stream;
    }

    /// The right-hand sides of a binary operator's impls, in output order:
    /// the same type (when enabled) and then the capability's friends, or a
    /// shift amount.
    fn operator_rhs(
        &self,
        operator: &OpSpec,
    ) -> [Option<OpRhs>; 2] {
        return match operator.kind {
            OpKind::Arith => [
                self.with_self_math().then_some(OpRhs::Same),
                Some(OpRhs::Friend(Capability::Math)),
            ],
            OpKind::Bitwise => [
                self.with_self_bit().then_some(OpRhs::Same),
                Some(OpRhs::Friend(Capability::Bit)),
            ],
            OpKind::Shift => [
                None,
                Some(OpRhs::Amount),
            ],
            OpKind::Unary => [
                None,
                None,
            ],
        };
    }

    /// `operator`'s impl in `form` for `rhs`. Its `konst_*` flags are the
    /// operator's, with an assignment's as `[XAssign, op, Assign, family]`;
    /// a same-type operand calls the `_proven` helper when only same-type
    /// operands are proven. A shift's helper takes any amount, so its doc
    /// reflects the full proof.
    fn binary_op(
        &self,
        operator: &OpSpec,
        proof: OperatorProof,
        rhs: OpRhs,
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
        let helper = self.arithmetic_method_name(operator, is_konst);
        let (helper, proven) = match rhs {
            OpRhs::Same => (Self::same_type_helper(proof, &helper), proof.same),
            OpRhs::Friend(_) | OpRhs::Amount => (helper, proof.full),
        };
        return OpImpl {
            trait_name,
            method,
            is_konst,
            helper,
            doc: self.proof_doc(proven),
        };
    }

    /// `body` as one binary operator impl: no `<..>` for `Same`, `<T>` with
    /// the friend and seal bounds for `Friend`, `<usize>` for `Amount`.
    fn impl_op(
        &self,
        it: &OpImpl,
        rhs: OpRhs,
        body: TokenStream,
    ) -> TokenStream {
        let trait_name = &it.trait_name;
        return match rhs {
            OpRhs::Same => self.impl_trait(
                it.is_konst,
                quote! { ::core::ops::#trait_name },
                body,
            ),
            OpRhs::Friend(capability) => {
                let rhs_ty = &self.ctx.rhs_ty;
                let friend = self.ctx.friends.bound(capability);
                let seal = self.seal_bound(it.is_konst);
                self.impl_rhs_trait(
                    it.is_konst,
                    quote! { #friend + #seal },
                    quote! { ::core::ops::#trait_name<#rhs_ty> },
                    body,
                )
            }
            OpRhs::Amount => self.impl_trait(
                it.is_konst,
                quote! { ::core::ops::#trait_name<usize> },
                body,
            ),
        };
    }

    /// One binary operator impl: `type Output` (value form), the doc, then
    /// the method, which binds a friend operand as `it` first.
    fn make_op_impl(
        &self,
        it: &OpImpl,
        rhs: OpRhs,
        form: OpForm,
    ) -> TokenStream {
        let method = &it.method;
        let helper = &it.helper;
        let doc = &it.doc;
        let param = match (rhs, form) {
            (OpRhs::Amount, OpForm::Assign) => format_ident!("that"),
            _ => format_ident!("rhs"),
        };
        let (param_ty, prelude, call) = match rhs {
            OpRhs::Same => {
                let rhs_raw = &self.ctx.rhs_raw;
                (
                    quote! { Self },
                    TokenStream::new(),
                    quote! { self.#helper(#rhs_raw) },
                )
            }
            OpRhs::Friend(_) => {
                let rhs_ty = &self.ctx.rhs_ty;
                let conversion = self.ctx.friends.conversion_path();
                (
                    quote! { #rhs_ty },
                    quote! { let it = #conversion(rhs); },
                    quote! { self.#helper(it) },
                )
            }
            OpRhs::Amount => (
                quote! { usize },
                TokenStream::new(),
                quote! { self.#helper(#param) },
            ),
        };

        let body = match form {
            OpForm::Value => {
                let output_ty = self.ctx.output.ty();
                let result = self.ctx.output.convert(call);
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
        return self.impl_op(it, rhs, body);
    }

    /// A unary operator trait forwarding to its validating `_{method}`
    /// helper.
    fn make_math_unary(
        &self,
        operator: &OpSpec,
    ) -> TokenStream {
        if !self.with_operator(operator) {
            return TokenStream::new();
        }

        let output = &self.ctx.output;
        let is_konst = self.ctx.cfg.konst_for(&operator.konsts());
        let output_ty = output.ty();
        let trait_name = format_ident!("{}", operator.trait_name);
        let method = format_ident!("{}", operator.method);
        let helper = format_ident!("_{}", method);
        let result = output.convert(quote! { self.#helper() });
        let doc = self.proof_doc(self.unary_proven(operator));
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
