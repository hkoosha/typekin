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
use crate::integral_cfg::{
    IntegralFlag,
    IntegralName,
};
use crate::zz_forward::{
    CoreFwd,
    InDomain,
};

const CORE_FLOAT_CONSTS: &[(&str, &str)] = &[
    ("RADIX", "u32"),
    ("MANTISSA_DIGITS", "u32"),
    ("DIGITS", "u32"),
    ("MIN_EXP", "i32"),
    ("MAX_EXP", "i32"),
    ("MIN_10_EXP", "i32"),
    ("MAX_10_EXP", "i32"),
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

    /// A `core_float` query that every valid value answers alike returns
    /// that answer: its doc line and body.
    pub(crate) fn decided_query(
        &self,
        row: &CoreFwd,
    ) -> Option<(TokenStream, TokenStream)> {
        // Only a private field keeps every live value valid.
        if row.lens != InDomain::Predicate || self.ctx.cfg.visible_value {
            return None;
        }
        let validation = &self.ctx.cfg.validation;
        let answer = validation.float_domain()?.query(row.name)?;
        let doc = format!(
            " Always `{answer}`: every value in the domain `{}` answers \
            alike (decided by typekin at expansion).",
            validation.domain_text(),
        );
        return Some((quote! { #[doc = #doc] }, quote! { return #answer; }));
    }

    pub(crate) fn make_fmt_exp(&self) -> TokenStream {
        return self.make_fmt_traits(&[
            (IntegralFlag::FmtExpLower, "LowerExp"),
            (IntegralFlag::FmtExpUpper, "UpperExp"),
        ]);
    }

    /// `Sum` and `Product` over values and references: the raw fold of
    /// `core`'s float implementation, then one validation of the result.
    /// Operations that report failure fold into a `Result` instead.
    pub(crate) fn make_iterator_folds(&self) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let el = &self.el;
        let get_raw = &self.ctx.get_raw;
        let iterator = self.target.fresh_ident("I");
        let lifetime = Lifetime::new("'__typekin_item", Span::call_site());
        let lifetime_param =
            GenericParam::Lifetime(LifetimeParam::new(lifetime.clone()));

        let mut stream = TokenStream::new();

        for (flag, name, method, symbol) in [
            (IntegralFlag::Sum, "Sum", "sum", quote! { + }),
            (IntegralFlag::Product, "Product", "product", quote! { * }),
        ] {
            if !self.ctx.cfg.flags.has(flag) {
                continue;
            }
            let fold = format_ident!("{}", name);
            let method = Ident::new(method, Span::call_site());
            if let Some(failure) = &self.ctx.failure {
                let (ref_impl_generics, ref_where_clause) = self
                    .target
                    .impl_header(std::slice::from_ref(&lifetime_param), &[]);
                let or_err = self.ctx.names.get(IntegralName::OrErr);
                // `Self` is the `Result` here: qualify by the target.
                let get_raw = self.target.qualify_path(get_raw);
                let output = quote! {
                    ::core::result::Result<#ty, #failure>
                };
                let body = |item: TokenStream| {
                    return quote! {
                        let mut iter = iter;
                        let mut folded = match iter.next() {
                            ::core::option::Option::Some(it) => #item,
                            ::core::option::Option::None => {
                                let empty = ::core::iter::empty::<#el>();
                                return <#ty>::#or_err(
                                    ::core::option::Option::Some(
                                        empty.#method::<#el>(),
                                    ),
                                );
                            }
                        };
                        for it in iter {
                            let value = #get_raw(folded) #symbol #get_raw(#item);
                            folded = <#ty>::#or_err(
                                ::core::option::Option::Some(value),
                            )?;
                        }
                        return ::core::result::Result::Ok(folded);
                    };
                };
                let by_value = body(quote! { it });
                let by_reference = body(quote! { *it });
                stream.extend(quote! {
                    impl #impl_generics ::core::iter::#fold<#ty> for #output
                    #where_clause {
                        #[inline(always)]
                        fn #method<#iterator>(iter: #iterator) -> Self
                        where
                            #iterator: ::core::iter::Iterator<Item = #ty>,
                        {
                            #by_value
                        }
                    }

                    impl #ref_impl_generics ::core::iter::#fold<&#lifetime #ty>
                        for #output
                    #ref_where_clause
                    {
                        #[inline(always)]
                        fn #method<#iterator>(iter: #iterator) -> Self
                        where
                            #iterator:
                                ::core::iter::Iterator<Item = &#lifetime #ty>,
                        {
                            #by_reference
                        }
                    }
                });
                continue;
            }
            stream.extend(self.impl_trait(
                false,
                quote! { ::core::iter::#fold },
                quote! {
                    #[inline(always)]
                    #[track_caller]
                    fn #method<#iterator>(iter: #iterator) -> Self
                    where
                        #iterator: ::core::iter::Iterator<Item = Self>,
                    {
                        let raw = iter.map(|it| #get_raw(it)).#method::<#el>();
                        return Self::_result_or_panic(raw);
                    }
                },
            ));
            stream.extend(self.impl_trait_with(
                false,
                std::slice::from_ref(&lifetime_param),
                &[],
                quote! { ::core::iter::#fold<&#lifetime #ty> },
                quote! {
                    #[inline(always)]
                    #[track_caller]
                    fn #method<#iterator>(iter: #iterator) -> Self
                    where
                        #iterator: ::core::iter::Iterator<Item = &#lifetime Self>,
                    {
                        let raw = iter.map(|it| #get_raw(*it)).#method::<#el>();
                        return Self::_result_or_panic(raw);
                    }
                },
            ));
        }

        return stream;
    }
}
