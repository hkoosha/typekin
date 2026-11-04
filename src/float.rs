use proc_macro2::{
    Ident,
    Span,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::{
    GenericParam,
    Lifetime,
    LifetimeParam,
};

use crate::integral::Maker;
use crate::integral_cfg::IntegralFlag;
use crate::integral_forward::{
    Param,
    params,
};
use crate::zz_domain_float::MathProvider;
use crate::zz_lib::Forward;
use crate::zz_values::{
    F,
    FnName,
    N,
};

/// `(name, parameters)` for float methods that return `Self`.
const CORE_FLOAT_OPS: &[(FnName, &[Param])] = &[
    (FnName::Abs, &[]),
    (FnName::Signum, &[]),
    (FnName::Copysign, &[Param::Wrapped("sign")]),
    (FnName::Recip, &[]),
    (FnName::ToDegrees, &[]),
    (FnName::ToRadians, &[]),
    (FnName::NextUp, &[]),
    (FnName::NextDown, &[]),
    (FnName::Midpoint, &[Param::Wrapped("that")]),
];

/// `min`, `max`, and `clamp` for types without `Ord`; `Ord`'s versions
/// select valid values and need no validation.
const CORE_FLOAT_SELECTIONS: &[(FnName, &[Param])] = &[
    (FnName::Min, &[Param::Wrapped("that")]),
    (FnName::Max, &[Param::Wrapped("that")]),
    (
        FnName::Clamp,
        &[
            Param::Wrapped("min"),
            Param::Wrapped("max"),
        ],
    ),
];

const CORE_FLOAT_QUERIES: &[FnName] = &[
    FnName::IsNan,
    FnName::IsInfinite,
    FnName::IsFinite,
    FnName::IsSubnormal,
    FnName::IsNormal,
    FnName::IsSignPositive,
    FnName::IsSignNegative,
];

const CORE_FLOAT_CONSTS: &[(&str, &str)] = &[
    ("RADIX", "u32"),
    ("MANTISSA_DIGITS", "u32"),
    ("DIGITS", "u32"),
    ("MIN_EXP", "i32"),
    ("MAX_EXP", "i32"),
    ("MIN_10_EXP", "i32"),
    ("MAX_10_EXP", "i32"),
];

/// A value of the wrapper, passed on as its raw float.
const A: Param = Param::Wrapped("a");
const B: Param = Param::Wrapped("b");
/// An `i32` exponent.
const EXPONENT: Param = Param::Plain("a", N::I032);

/// `(name, parameters, const with the provider)`.
const STD_MATH: &[(FnName, &[Param], bool)] = &[
    (FnName::Floor, &[], true),
    (FnName::Ceil, &[], true),
    (FnName::Round, &[], true),
    (FnName::RoundTiesEven, &[], true),
    (FnName::Trunc, &[], true),
    (FnName::Fract, &[], true),
    (
        FnName::MulAdd,
        &[
            A,
            B,
        ],
        true,
    ),
    (FnName::DivEuclid, &[A], false),
    (FnName::RemEuclid, &[A], false),
    (FnName::Powi, &[EXPONENT], false),
    (FnName::Powf, &[A], false),
    (FnName::Sqrt, &[], false),
    (FnName::Cbrt, &[], false),
    (FnName::Exp, &[], false),
    (FnName::Exp2, &[], false),
    (FnName::ExpM1, &[], false),
    (FnName::Ln, &[], false),
    (FnName::Ln1p, &[], false),
    (FnName::Log, &[A], false),
    (FnName::Log2, &[], false),
    (FnName::Log10, &[], false),
    (FnName::Hypot, &[A], false),
    (FnName::Sin, &[], false),
    (FnName::Cos, &[], false),
    (FnName::Tan, &[], false),
    (FnName::Asin, &[], false),
    (FnName::Acos, &[], false),
    (FnName::Atan, &[], false),
    (FnName::Atan2, &[A], false),
    (FnName::Sinh, &[], false),
    (FnName::Cosh, &[], false),
    (FnName::Tanh, &[], false),
    (FnName::Asinh, &[], false),
    (FnName::Acosh, &[], false),
    (FnName::Atanh, &[], false),
];

const NIGHTLY_MATH: &[(FnName, &[Param], bool)] = &[
    (FnName::Floor, &[], true),
    (FnName::Ceil, &[], true),
    (FnName::Round, &[], true),
    (FnName::RoundTiesEven, &[], true),
    (FnName::Trunc, &[], true),
    (FnName::Fract, &[], true),
    (
        FnName::MulAdd,
        &[
            A,
            B,
        ],
        true,
    ),
    (FnName::DivEuclid, &[A], false),
    (FnName::RemEuclid, &[A], false),
    (FnName::Powi, &[EXPONENT], false),
    (FnName::Sqrt, &[], false),
    (FnName::Cbrt, &[], false),
];

impl Maker {
    /// `core_float` items that are not functions: the float's constants.
    pub(crate) fn make_core_float(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::CoreFloat) {
            return TokenStream::new();
        }

        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let el = &self.el;
        let consts = CORE_FLOAT_CONSTS.iter().map(|(name, raw)| {
            let name = format_ident!("{}", name);
            let raw = format_ident!("{}", raw);
            return quote! {
                pub const #name: #raw = #el::#name;
            };
        });

        return quote! {
            impl #impl_generics #ty #where_clause {
                #(#consts)*
            }
        };
    }

    /// The `core_float` methods: each calls the float's method of the same
    /// name and validates a wrapper result.
    pub(crate) fn core_float_forwards(&self) -> Vec<Forward> {
        if !self.ctx.cfg.flags.has(IntegralFlag::CoreFloat) {
            return Vec::new();
        }
        let Some(float) = self.num.float()
        else {
            return Vec::new();
        };

        let mut forwards = self.core_float_query_forwards();
        forwards.extend(self.core_float_bit_forwards(float));
        forwards.extend(self.core_float_op_forwards());
        return forwards;
    }

    /// The `is_*` queries, constant where the domain decides them, and
    /// `classify`.
    fn core_float_query_forwards(&self) -> Vec<Forward> {
        let get_raw = &self.ctx.get_raw;
        // Only a private field keeps every live value valid.
        let domain = match self.ctx.cfg.visible_value {
            true => None,
            false => self.ctx.cfg.validation.float_domain(),
        };
        let mut forwards = CORE_FLOAT_QUERIES
            .iter()
            .map(|name| {
                let answer = domain.and_then(|it| it.query(*name));
                return Forward::new(*name, &self.ctx.names, |name| {
                    let Some(answer) = answer
                    else {
                        return quote! {
                            #[must_use]
                            #[inline(always)]
                            pub const fn #name(self) -> bool {
                                return #get_raw(self).#name();
                            }
                        };
                    };
                    let doc = format!(
                        " Always `{answer}`: every value in the domain `{}` \
                        answers alike (decided by typekin at expansion).",
                        self.ctx.cfg.validation.domain_text(),
                    );
                    return quote! {
                        #[doc = #doc]
                        #[must_use]
                        #[inline(always)]
                        pub const fn #name(self) -> bool {
                            return #answer;
                        }
                    };
                });
            })
            .collect::<Vec<_>>();
        forwards.push(Forward::new(
            FnName::Classify,
            &self.ctx.names,
            |name| {
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn #name(self) -> ::core::num::FpCategory {
                        return #get_raw(self).#name();
                    }
                };
            },
        ));
        return forwards;
    }

    /// The bit and byte conversions and `total_cmp`.
    fn core_float_bit_forwards(
        &self,
        float: F,
    ) -> Vec<Forward> {
        let bits = format_ident!("u{}", float.bits());
        let el = &self.el;
        let get_raw = &self.ctx.get_raw;
        let konst = self.core_konst();
        let bytes = quote! { [u8; ::core::mem::size_of::<#el>()] };

        let mut forwards = vec![
            Forward::new(FnName::ToBits, &self.ctx.names, |name| {
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn #name(self) -> #bits {
                        return #get_raw(self).#name();
                    }
                };
            }),
            Forward::new(FnName::FromBits, &self.ctx.names, |name| {
                return quote! {
                    #[inline(always)]
                    #[track_caller]
                    pub #konst fn #name(bits: #bits) -> Self {
                        return Self::_result_or_panic(#el::#name(bits));
                    }
                };
            }),
        ];
        for (to, from) in [
            (FnName::ToNeBytes, FnName::FromNeBytes),
            (FnName::ToBeBytes, FnName::FromBeBytes),
            (FnName::ToLeBytes, FnName::FromLeBytes),
        ] {
            forwards.push(Forward::new(to, &self.ctx.names, |name| {
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn #name(self) -> #bytes {
                        return #get_raw(self).#name();
                    }
                };
            }));
            forwards.push(Forward::new(from, &self.ctx.names, |name| {
                return quote! {
                    #[inline(always)]
                    #[track_caller]
                    pub #konst fn #name(bytes: #bytes) -> Self {
                        return Self::_result_or_panic(#el::#name(bytes));
                    }
                };
            }));
        }
        forwards.push(Forward::new(
            FnName::TotalCmp,
            &self.ctx.names,
            |name| {
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub fn #name(
                        &self,
                        that: &Self,
                    ) -> ::core::cmp::Ordering {
                        return #get_raw(*self).#name(&#get_raw(*that));
                    }
                };
            },
        ));
        return forwards;
    }

    /// The methods returning `Self`, then the selections a type without
    /// `Ord` gets.
    fn core_float_op_forwards(&self) -> Vec<Forward> {
        let get_raw = &self.ctx.get_raw;
        let konst = self.core_konst();
        let selections = match self.ctx.cfg.flags.has(IntegralFlag::Ord) {
            true => &[],
            false => CORE_FLOAT_SELECTIONS,
        };
        return CORE_FLOAT_OPS
            .iter()
            .chain(selections)
            .map(|(name, parameters)| {
                let (declared, passed) = params(parameters, get_raw);
                let result = match *name == FnName::Abs {
                    true => self.closed_or_validated(
                        self.ctx.cfg.validation.abs_closed(),
                    ),
                    false => quote! { Self::_result_or_panic(it) },
                };
                return Forward::new(*name, &self.ctx.names, |name| {
                    return quote! {
                        #[must_use]
                        #[inline(always)]
                        #[track_caller]
                        pub #konst fn #name(self #(, #declared)*) -> Self {
                            let it = #get_raw(self).#name(#(#passed),*);
                            return #result;
                        }
                    };
                });
            })
            .collect();
    }

    /// The `float_math` methods, from the `math` provider.
    pub(crate) fn float_math_forwards(&self) -> Vec<Forward> {
        if !self.ctx.cfg.flags.has(IntegralFlag::FloatMath) {
            return Vec::new();
        }

        let el = &self.el;
        let get_raw = &self.ctx.get_raw;
        let konst = self.core_konst();
        let functions = match self.ctx.cfg.math {
            MathProvider::Std => STD_MATH,
            MathProvider::Nightly => NIGHTLY_MATH,
            MathProvider::Core => {
                unreachable!("float_math is rejected without a provider")
            }
        };

        let mut forwards = functions
            .iter()
            .map(|(name, parameters, is_const)| {
                let konst = konst.as_ref().filter(|_| *is_const);
                let (declared, passed) = params(parameters, get_raw);
                return Forward::new(*name, &self.ctx.names, |name| {
                    let call = match self.ctx.cfg.math {
                        MathProvider::Nightly => quote! {
                            ::core::#el::math::#name(#get_raw(self) #(, #passed)*)
                        },
                        _ => quote! { #get_raw(self).#name(#(#passed),*) },
                    };
                    return quote! {
                        #[must_use]
                        #[inline(always)]
                        #[track_caller]
                        pub #konst fn #name(self #(, #declared)*) -> Self {
                            return Self::_result_or_panic(#call);
                        }
                    };
                });
            })
            .collect::<Vec<_>>();

        // Only `std` has `sin_cos`.
        if self.ctx.cfg.math == MathProvider::Std {
            forwards.push(Forward::new(
                FnName::SinCos,
                &self.ctx.names,
                |name| {
                    return quote! {
                        #[must_use]
                        #[inline(always)]
                        #[track_caller]
                        pub fn #name(self) -> (Self, Self) {
                            let (sin, cos) = #get_raw(self).#name();
                            return (
                                Self::_result_or_panic(sin),
                                Self::_result_or_panic(cos),
                            );
                        }
                    };
                },
            ));
        }

        return forwards;
    }

    pub(crate) fn make_fmt_exp(&self) -> TokenStream {
        return self.make_fmt_traits(&[
            (IntegralFlag::FmtExpLower, "LowerExp"),
            (IntegralFlag::FmtExpUpper, "UpperExp"),
        ]);
    }

    /// `Sum` and `Product` over values and references: the raw fold of
    /// `core`'s float implementation, then one validation of the result.
    pub(crate) fn make_iterator_folds(&self) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let el = &self.el;
        let get_raw = &self.ctx.get_raw;
        let iterator = self.target.fresh_ident("I");
        let lifetime = Lifetime::new("'__typekin_item", Span::call_site());
        let (ref_impl_generics, ref_where_clause) = self.target.impl_header(
            &[GenericParam::Lifetime(LifetimeParam::new(lifetime.clone()))],
            &[],
        );

        let mut stream = TokenStream::new();

        for (flag, name, method) in [
            (IntegralFlag::Sum, "Sum", "sum"),
            (IntegralFlag::Product, "Product", "product"),
        ] {
            if !self.ctx.cfg.flags.has(flag) {
                continue;
            }
            let fold = format_ident!("{}", name);
            let method = Ident::new(method, Span::call_site());
            stream.extend(quote! {
                impl #impl_generics ::core::iter::#fold for #ty #where_clause {
                    #[inline(always)]
                    #[track_caller]
                    fn #method<#iterator>(iter: #iterator) -> Self
                    where
                        #iterator: ::core::iter::Iterator<Item = Self>,
                    {
                        let raw = iter.map(|it| #get_raw(it)).#method::<#el>();
                        return Self::_result_or_panic(raw);
                    }
                }

                impl #ref_impl_generics ::core::iter::#fold<&#lifetime #ty> for #ty
                #ref_where_clause
                {
                    #[inline(always)]
                    #[track_caller]
                    fn #method<#iterator>(iter: #iterator) -> Self
                    where
                        #iterator: ::core::iter::Iterator<Item = &#lifetime Self>,
                    {
                        let raw = iter.map(|it| #get_raw(*it)).#method::<#el>();
                        return Self::_result_or_panic(raw);
                    }
                }
            });
        }

        return stream;
    }
}
