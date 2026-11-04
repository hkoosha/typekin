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
    Konst,
    Konsts,
    validated_result,
};
use crate::zz_runner::Merged;
use crate::zz_values::{
    ArithmeticOp,
    Num,
    Operator,
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

/// A shift operator: its helper operation, trait, flags and `konst_*` flags.
struct Shift {
    operation: ArithmeticOp,
    trait_name: &'static str,
    math: IntegralFlag,
    assign: IntegralFlag,
    konst: Konsts,
    konst_assign: Konsts,
}

/// The shifts, in output order.
const SHIFTS: [Shift; 2] = [
    Shift {
        operation: ArithmeticOp::Shr,
        trait_name: "Shr",
        math: IntegralFlag::MathShr,
        assign: IntegralFlag::AssignShr,
        konst: Konsts::Shr,
        konst_assign: Konsts::ShrAssign,
    },
    Shift {
        operation: ArithmeticOp::Shl,
        trait_name: "Shl",
        math: IntegralFlag::MathShl,
        assign: IntegralFlag::AssignShl,
        konst: Konsts::Shl,
        konst_assign: Konsts::ShlAssign,
    },
];

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

    /// The impls after the forwards, in output order: conversions, shifts,
    /// assignments, the derive-like traits, operators, and formatting.
    pub(crate) fn impls_body(&self) -> TokenStream {
        let mut stream = TokenStream::new();
        stream.extend(self.make_cast_impls(true));
        stream.extend(self.make_cast_impls(false));
        stream.extend(self.make_math_shifts());
        stream.extend(self.make_assign_shifts());
        stream.extend(self.make_assign(Operator::BitAnd));
        for &operator in Operator::math_items() {
            stream.extend(self.make_assign(operator));
        }
        stream.extend(self.make_assign(Operator::BitOr));
        stream.extend(self.make_range());
        stream.extend(self.make_debug());
        stream.extend(self.make_hash());
        stream.extend(self.make_eq());
        stream.extend(self.make_ord());
        stream.extend(self.make_assertions());
        for &operator in Operator::math_items() {
            stream.extend(self.make_math_binary(operator));
        }
        stream.extend(self.make_math_binary(Operator::BitAnd));
        stream.extend(self.make_math_binary(Operator::BitOr));
        stream.extend(self.make_math_xor());
        stream.extend(self.make_math_not());
        stream.extend(self.make_math_neg());
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

    pub(crate) fn make_range(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::Range) {
            return TokenStream::new();
        }

        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let el = &self.el;
        let konst = Konst::konst(self.ctx.cfg.konst_for(&[Konsts::Iterator]));
        let start = self.storage.extract(quote! { start.0 });
        let end = self.storage.extract(quote! { end.0 });
        let forward = self.step_direction("forward", &start);
        let backward = self.step_direction("backward", &start);

        return quote! {
            #konst impl #impl_generics ::core::iter::Step for #ty #where_clause {
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
            }
        };
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

        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let el = &self.el;
        let self_raw_ref = self.storage.extract_ref(quote! { self.0 });
        return quote! {
            impl #impl_generics ::core::convert::AsRef<#el> for #ty
            #where_clause {
                #[inline(always)]
                fn as_ref(&self) -> &#el {
                    return #self_raw_ref;
                }
            }
        };
    }

    pub(crate) fn make_from_str(&self) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let el = &self.el;
        let validation = &self.ctx.validation;

        let mut stream = TokenStream::new();

        if self.ctx.cfg.flags.has(IntegralFlag::FromStr) {
            let constructed = self.construct_self(quote! { value });
            let result =
                validated_result(validation, constructed, quote! { () });
            stream.extend(quote! {
                impl #impl_generics ::core::str::FromStr for #ty
                #where_clause {
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
                }
            });
        }

        return stream;
    }

    pub(crate) fn make_serde(&self) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let el = &self.el;
        let validation = &self.ctx.validation;
        let get_raw = &self.ctx.get_raw;

        let mut stream = TokenStream::new();

        if self.ctx.cfg.flags.has(IntegralFlag::Serde) {
            let constructed = self.construct_self(quote! { value });
            let serializer = self.target.fresh_ident("S");
            let deserializer = self.target.fresh_ident("D");
            let de = self.target.fresh_lifetime("de");
            let result = validated_result(
                validation,
                constructed,
                quote! {
                    <#deserializer::Error as ::serde::de::Error>::custom(
                        "invalid value",
                    )
                },
            );
            let (de_generics, de_where_clause) =
                self.target.impl_header(&[parse_quote! { #de }], &[]);
            stream.extend(quote! {
                impl #impl_generics ::serde::Serialize for #ty
                #where_clause {
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
                }

                impl #de_generics ::serde::Deserialize<#de> for #ty
                #de_where_clause {
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
                }
            });
        }

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

        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let konst = Konst::konst(self.ctx.cfg.konst_for(&konsts));

        return self
            .cast_targets(safe)
            .map(|(it, cfg)| {
                let target = format_ident!("{}", it.rust_name());
                let cast = match safe {
                    true => {
                        let conv = self.ctx.names.get(IntegralName::Into(it));
                        quote! {
                            ::core::convert::Into<#target> for #ty
                            #where_clause {
                                #[inline(always)]
                                fn into(self) -> #target {
                                    return Self::#conv(self);
                                }
                            }
                        }
                    }
                    false => {
                        let conv =
                            self.ctx.names.get(IntegralName::TryInto(it));
                        quote! {
                            ::core::convert::TryInto<#target> for #ty
                            #where_clause {
                                type Error = ();

                                #[inline(always)]
                                fn try_into(
                                    self,
                                ) -> ::core::result::Result<#target, Self::Error> {
                                    return Self::#conv(self);
                                }
                            }
                        }
                    }
                };
                return quote! {
                    #cfg
                    #konst impl #impl_generics #cast
                };
            })
            .merged();
    }

    /// The shift operators by an amount; the helper takes any amount, so the
    /// doc reflects its full proof.
    pub(crate) fn make_math_shifts(&self) -> TokenStream {
        let mut stream = TokenStream::new();
        for shift in &SHIFTS {
            if !self.ctx.cfg.flags.has(shift.math) {
                continue;
            }
            let is_konst = self.ctx.cfg.konst_for(&[
                shift.konst,
                Konsts::Math,
            ]);
            let it = OpImpl {
                trait_name: format_ident!("{}", shift.trait_name),
                method: format_ident!("{}", shift.operation.name()),
                is_konst,
                helper: self.arithmetic_method_name(shift.operation, is_konst),
                doc: self.shift_doc(shift.operation),
            };
            stream.extend(self.make_op_impl(&it, OpRhs::Amount, OpForm::Value));
        }
        return stream;
    }

    /// The shift-assign operators by an amount `that`.
    pub(crate) fn make_assign_shifts(&self) -> TokenStream {
        let mut stream = TokenStream::new();
        for shift in &SHIFTS {
            if !self.ctx.cfg.flags.has(shift.assign) {
                continue;
            }
            let is_konst = self.ctx.cfg.konst_for(&[
                shift.konst_assign,
                shift.konst,
                Konsts::Assign,
                Konsts::Math,
            ]);
            let it = OpImpl {
                trait_name: format_ident!("{}Assign", shift.trait_name),
                method: format_ident!("{}_assign", shift.operation.name()),
                is_konst,
                helper: self.arithmetic_method_name(shift.operation, is_konst),
                doc: self.shift_doc(shift.operation),
            };
            stream.extend(self.make_op_impl(
                &it,
                OpRhs::Amount,
                OpForm::Assign,
            ));
        }
        return stream;
    }

    /// The proof doc of a shift impl: its helper (`_shr`, `_shl`, or their
    /// `_wrapping` twins) is proven for every amount or not at all.
    fn shift_doc(
        &self,
        operation: ArithmeticOp,
    ) -> TokenStream {
        return self.proof_doc(self.arithmetic_proof(operation).full);
    }

    pub(crate) fn make_assign(
        &self,
        operator: Operator,
    ) -> TokenStream {
        let enabled = IntegralFlag::assign(operator)
            .is_some_and(|flag| self.ctx.cfg.flags.has(flag));
        if !enabled || !self.with_global_operators() {
            return TokenStream::new();
        }
        return self.make_binary_ops(operator, OpForm::Assign);
    }

    pub(crate) fn make_debug(&self) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let self_raw = &self.ctx.self_raw;

        let mut stream = TokenStream::new();

        if self.ctx.cfg.flags.has(IntegralFlag::Debug) {
            let name = self.target.ident.to_string();
            let it = quote! {
                impl #impl_generics ::core::fmt::Debug for #ty #where_clause {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        return f
                            .debug_tuple(#name)
                            .field(&#self_raw)
                            .finish();
                    }
                }
            };
            stream.extend(it);
        }

        return stream;
    }

    pub(crate) fn make_hash(&self) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let self_raw = &self.ctx.self_raw;

        let mut stream = TokenStream::new();

        if self.ctx.cfg.flags.has(IntegralFlag::Hash) {
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
            let it = quote! {
                impl #impl_generics ::core::hash::Hash for #ty #where_clause {
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
                }
            };
            stream.extend(it);
        }

        return stream;
    }

    pub(crate) fn make_eq(&self) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;

        let mut stream = TokenStream::new();

        if self.ctx.cfg.flags.has(IntegralFlag::Eq) {
            let konst = Konst::konst(self.ctx.cfg.konst_for(&[
                Konsts::Eq,
                Konsts::Comparison,
            ]));
            // Not a derive: a derive cannot emit `const impl` under `konst`,
            // and it would bound the generic parameters.
            let it = quote! {
                #konst impl #impl_generics ::core::cmp::Eq for #ty #where_clause {}
            };
            stream.extend(it);
        }

        return stream;
    }

    pub(crate) fn make_ord(&self) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;

        let mut stream = TokenStream::new();

        if self.ctx.cfg.flags.has(IntegralFlag::Ord) {
            let konst = Konst::konst(self.ctx.cfg.konst_for(&[
                Konsts::Ord,
                Konsts::Comparison,
            ]));
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
            let it = quote! {
                #konst impl #impl_generics ::core::cmp::Ord for #ty #where_clause {
                    #[inline(always)]
                    fn cmp(&self, that: &Self) -> ::core::cmp::Ordering {
                        #body
                    }
                }
            };

            stream.extend(it);
        }

        return stream;
    }

    pub(crate) fn make_assertions(&self) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;

        let mut stream = TokenStream::new();

        if self.ctx.cfg.flags.has(IntegralFlag::Assertions) {
            let assertions = self.storage.layout_assertions(quote! { #ty });
            if self.target.has_generics() {
                let name = self.target.layout_assertion();
                stream.extend(quote! {
                    impl #impl_generics #ty #where_clause {
                        const #name: () = {
                            #assertions
                        };
                    }
                });
            }
            else {
                stream.extend(assertions);
            }
        }

        return stream;
    }

    /// A binary arithmetic or bitwise operator other than xor.
    pub(crate) fn make_math_binary(
        &self,
        operator: Operator,
    ) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::math(operator))
            || !self.with_global_operators()
        {
            return TokenStream::new();
        }
        return self.make_binary_ops(operator, OpForm::Value);
    }

    /// Xor and xor-assign, interleaved: same-type, then friend. The assigns
    /// need a `Self` output (xor has no assign flag).
    pub(crate) fn make_math_xor(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::MathXor)
            || !self.with_global_operators()
        {
            return TokenStream::new();
        }

        let operator = Operator::BitXor;
        let proof = self.operator_proof(operator);
        let friend = OpRhs::Friend(Capability::Bit);
        let with_assign = self.ctx.cfg.ty_out.is_self();
        let mut stream = TokenStream::new();

        if self.with_self_bit() {
            let it =
                self.binary_op(operator, proof, OpRhs::Same, OpForm::Value);
            stream.extend(self.make_op_impl(&it, OpRhs::Same, OpForm::Value));
            if with_assign {
                let it = self.binary_op(
                    operator,
                    proof,
                    OpRhs::Same,
                    OpForm::Assign,
                );
                stream.extend(self.make_op_impl(
                    &it,
                    OpRhs::Same,
                    OpForm::Assign,
                ));
            }
        }

        let it = self.binary_op(operator, proof, friend, OpForm::Value);
        stream.extend(self.make_op_impl(&it, friend, OpForm::Value));
        if with_assign {
            let it = self.binary_op(operator, proof, friend, OpForm::Assign);
            stream.extend(self.make_op_impl(&it, friend, OpForm::Assign));
        }

        return stream;
    }

    /// `operator`'s same-type impl (when enabled), then its friend impl.
    fn make_binary_ops(
        &self,
        operator: Operator,
        form: OpForm,
    ) -> TokenStream {
        let (with_self, capability) = match operator.arithmetic_op() {
            Some(_) => (self.with_self_math(), Capability::Math),
            None => (self.with_self_bit(), Capability::Bit),
        };
        let proof = self.operator_proof(operator);
        let friend = OpRhs::Friend(capability);
        let mut stream = TokenStream::new();

        if with_self {
            let it = self.binary_op(operator, proof, OpRhs::Same, form);
            stream.extend(self.make_op_impl(&it, OpRhs::Same, form));
        }

        let it = self.binary_op(operator, proof, friend, form);
        stream.extend(self.make_op_impl(&it, friend, form));
        return stream;
    }

    /// `operator`'s impl in `form` for `rhs`. Its `konst_*` flags are the
    /// operator's, with an assignment's as `[XAssign, op, Assign, family]`;
    /// a same-type operand calls the `_proven` helper when only same-type
    /// operands are proven.
    fn binary_op(
        &self,
        operator: Operator,
        proof: OperatorProof,
        rhs: OpRhs,
        form: OpForm,
    ) -> OpImpl {
        let [
            operator_flag,
            family_flag,
        ] = operator.is_konst_flags();
        let (trait_name, method, is_konst) = match form {
            OpForm::Value => (
                format_ident!("{}", operator.name()),
                format_ident!("{}", operator.method_name()),
                self.ctx.cfg.konst_for(&[
                    operator_flag,
                    family_flag,
                ]),
            ),
            OpForm::Assign => (
                format_ident!("{}Assign", operator.name()),
                format_ident!("{}_assign", operator.method_name()),
                self.ctx.cfg.konst_for(&[
                    operator.assign_konst(),
                    operator_flag,
                    Konsts::Assign,
                    family_flag,
                ]),
            ),
        };
        let helper = match operator.arithmetic_op() {
            Some(arithmetic) => {
                self.arithmetic_method_name(arithmetic, is_konst)
            }
            None => format_ident!("_{}", operator.method_name()),
        };
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

    /// `#konst impl … for Ty …` of one binary operator impl: no `<..>` for
    /// `Same`, `<T>` with the friend and seal bounds for `Friend`, `<usize>`
    /// for `Amount`.
    fn op_header(
        &self,
        it: &OpImpl,
        rhs: OpRhs,
    ) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let trait_name = &it.trait_name;
        let konst = Konst::konst(it.is_konst);
        return match rhs {
            OpRhs::Same => quote! {
                #konst impl #impl_generics ::core::ops::#trait_name for #ty
                #where_clause
            },
            OpRhs::Friend(capability) => {
                let rhs_ty = &self.ctx.rhs_ty;
                let rhs_impl_generics = &self.ctx.rhs_impl_generics;
                let friend = self.ctx.friends.bound(capability);
                let seal = self.seal_bound(it.is_konst);
                let rhs_where = self.rhs_where(quote! { #friend + #seal });
                quote! {
                    #konst impl #rhs_impl_generics
                    ::core::ops::#trait_name<#rhs_ty> for #ty #rhs_where
                }
            }
            OpRhs::Amount => quote! {
                #konst impl #impl_generics ::core::ops::#trait_name<usize>
                for #ty #where_clause
            },
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
        let header = self.op_header(it, rhs);
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

        return match form {
            OpForm::Value => {
                let output_ty = self.ctx.output.ty();
                let result = self.ctx.output.convert(call);
                quote! {
                    #header {
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
            }
            OpForm::Assign => quote! {
                #header {
                    #doc
                    #[inline(always)]
                    fn #method(
                        &mut self,
                        #param: #param_ty,
                    ) {
                        #prelude
                        *self = #call;
                    }
                }
            },
        };
    }

    pub(crate) fn make_math_not(&self) -> TokenStream {
        return self.make_math_unary(
            self.ctx.cfg.flags.has(IntegralFlag::MathNot),
            &[
                Konsts::Not,
                Konsts::Bit,
            ],
            quote! { ::core::ops::Not },
            format_ident!("not"),
            self.not_proven(),
        );
    }

    pub(crate) fn make_math_neg(&self) -> TokenStream {
        return self.make_math_unary(
            self.ctx.cfg.flags.has(IntegralFlag::MathNeg),
            &[
                Konsts::Neg,
                Konsts::Math,
            ],
            quote! { ::core::ops::Neg },
            format_ident!("neg"),
            self.neg_proven(),
        );
    }

    /// A unary operator trait forwarding to its validating `_{method}`
    /// helper.
    fn make_math_unary(
        &self,
        enabled: bool,
        konsts: &[Konsts],
        operator: TokenStream,
        method: Ident,
        proven: bool,
    ) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let output = &self.ctx.output;

        let mut stream = TokenStream::new();

        if enabled {
            let konst = Konst::konst(self.ctx.cfg.konst_for(konsts));
            let output_ty = output.ty();
            let helper = format_ident!("_{}", method);
            let result = output.convert(quote! { self.#helper() });
            let doc = self.proof_doc(proven);
            let it = quote! {
                #konst impl #impl_generics #operator for #ty #where_clause {
                    type Output = #output_ty;

                    #doc
                    #[inline(always)]
                    fn #method(self) -> Self::Output {
                        return #result;
                    }
                }
            };
            stream.extend(it);
        }

        return stream;
    }

    /// The `core::fmt` traits whose flag is on, each formatting the raw
    /// value: `(flag, trait name)` rows.
    pub(crate) fn make_fmt_traits(
        &self,
        traits: &[(IntegralFlag, &str)],
    ) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let get_raw = &self.ctx.get_raw;

        let mut stream = TokenStream::new();

        for &(flag, name) in traits {
            if !self.ctx.cfg.flags.has(flag) {
                continue;
            }
            let format = format_ident!("{}", name);
            stream.extend(quote! {
                impl #impl_generics ::core::fmt::#format for #ty #where_clause {
                    #[inline(always)]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let raw = #get_raw(*self);
                        return ::core::fmt::#format::fmt(&raw, f);
                    }
                }
            });
        }

        return stream;
    }

    pub(crate) fn make_partial_eq(&self) -> TokenStream {
        let ty = &self.ctx.ty;
        let rhs_ty = &self.ctx.rhs_ty;
        let rhs_impl_generics = &self.ctx.rhs_impl_generics;
        let fp_friend_conv = self.ctx.friends.conversion_path();
        let get_raw = &self.ctx.get_raw;
        let trait_friend_rel = self.ctx.friends.bound(Capability::Rel);

        let mut stream = TokenStream::new();

        if self.ctx.cfg.flags.has(IntegralFlag::PartialEq) {
            let flags = [
                Konsts::PartialEq,
                Konsts::Comparison,
            ];
            let is_konst = self.ctx.cfg.konst_for(&flags);
            let konst = Konst::konst(is_konst);
            let bounds = self.seal_bound(is_konst);
            let rhs_where = self.rhs_where(quote! {
                #trait_friend_rel + #bounds + ::core::marker::Copy
            });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::cmp::PartialEq<#rhs_ty> for #ty
                #rhs_where
                {
                    #[inline(always)]
                    fn eq(
                        &self,
                        rhs: &#rhs_ty,
                    ) -> bool {
                        let lhs = #get_raw(*self);
                        let rhs = #fp_friend_conv(*rhs);
                        return lhs == rhs;
                    }
                }
            };
            stream.extend(it);
        }

        return stream;
    }

    pub(crate) fn make_partial_ord(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::PartialOrd) {
            return TokenStream::new();
        }

        let ty = &self.ctx.ty;
        let rhs_ty = &self.ctx.rhs_ty;
        let rhs_impl_generics = &self.ctx.rhs_impl_generics;
        let fp_friend_conv = self.ctx.friends.conversion_path();
        let get_raw = &self.ctx.get_raw;
        let trait_friend_rel = self.ctx.friends.bound(Capability::Rel);
        let is_konst = self.ctx.cfg.konst_for(&[
            Konsts::PartialOrd,
            Konsts::Comparison,
        ]);
        let konst = Konst::konst(is_konst);
        let bounds = self.seal_bound(is_konst);
        let rhs_where = self.rhs_where(quote! {
            #trait_friend_rel + #bounds + ::core::marker::Copy
        });
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

        return quote! {
            #konst impl #rhs_impl_generics ::core::cmp::PartialOrd<#rhs_ty> for #ty
            #rhs_where
            {
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
            }
        };
    }
}
