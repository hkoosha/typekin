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
    Path,
};

use crate::integral::Maker;
use crate::integral_cfg::{
    IntegralFlag,
    Translation,
    translate,
};
use crate::integral_compute::{
    Body,
    OpRef,
};
use crate::zz_forward::{
    CoreFwd,
    InDomain,
};
use crate::zz_lib::{
    FnAttrs,
    Output,
};
use crate::zz_values::{
    Op,
    OpSpec,
};

/// `Sum` and `Product`: each one's flag, the operator it folds, its
/// `core::iter` trait and that trait's method.
const FOLDS: [(IntegralFlag, Op, &str, &str); 2] = [
    (IntegralFlag::Sum, Op::Add, "Sum", "sum"),
    (IntegralFlag::Product, Op::Mul, "Product", "product"),
];

/// One impl of `Sum` or `Product`.
struct Fold<'a> {
    /// The operator folded: `+` or `*`.
    spec: &'a OpSpec,
    /// `Sum` or `Product`.
    name: Ident,
    /// `sum` or `product`.
    method: Ident,
    /// The lifetime of items taken by reference; `None` takes them by
    /// value.
    lifetime: Option<&'a Lifetime>,
}

impl Fold<'_> {
    /// The wrapper value of the item `it`.
    fn item(&self) -> TokenStream {
        return match self.lifetime {
            Some(_) => quote! { *it },
            None => quote! { it },
        };
    }

    /// What stands before the type of an item taken by reference.
    fn borrow(&self) -> Option<TokenStream> {
        return self.lifetime.map(|it| quote! { &#it });
    }

    /// The generic parameter the impl adds: the items' lifetime.
    fn parameters(&self) -> Vec<GenericParam> {
        return self
            .lifetime
            .map(|it| GenericParam::Lifetime(LifetimeParam::new(it.clone())))
            .into_iter()
            .collect();
    }
}

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

    /// `Sum` and `Product` over values and references.
    pub(crate) fn make_iterator_folds(&self) -> TokenStream {
        let lifetime = Lifetime::new("'__typekin_item", Span::call_site());
        let mut stream = TokenStream::new();
        for (flag, op, name, method) in FOLDS {
            if !self.ctx.cfg.flags.has(flag) {
                continue;
            }
            for lifetime in [
                None,
                Some(&lifetime),
            ] {
                stream.extend(self.fold_impl(&Fold {
                    spec: op.spec(),
                    name: format_ident!("{}", name),
                    method: format_ident!("{}", method),
                    lifetime,
                }));
            }
        }
        return stream;
    }

    /// One impl of a fold. Its output is free: where a failure panics,
    /// the impl is the target's; where it is passed on, the `Result`'s.
    fn fold_impl(
        &self,
        fold: &Fold,
    ) -> TokenStream {
        let get_raw = &self.ctx.get_raw;
        let item = fold.item();
        let items = quote! { iter.map(|it| #get_raw(#item)) };
        let body = self.compute(OpRef::Fold {
            method: &fold.method,
            items: &items,
        });
        let panic_at = self.ctx.cfg.wide.panic_at;
        let Some(translation) = translate(body.fails, panic_at, false)
        else {
            return TokenStream::new();
        };
        let FnAttrs { track_caller, .. } = FnAttrs::of(
            self.form_panics(body.panics, translation),
            Output::Unmarked,
        );
        return match self.passed_failure(body.fails, translation) {
            Some(failure) => {
                let constructor = self
                    .result_constructor(body.fails, translation)
                    .expect("a failure is passed on by its constructor");
                self.fold_passing(fold, failure, &constructor, &track_caller)
            }
            None => {
                self.fold_panicking(fold, &body, translation, &track_caller)
            }
        };
    }

    /// A fold as the target's impl: the raw fold of `core`'s float
    /// implementation (`body`), then one construction of the result.
    fn fold_panicking(
        &self,
        fold: &Fold,
        body: &Body,
        translation: Translation,
        track_caller: &Option<TokenStream>,
    ) -> TokenStream {
        let ty = &self.ctx.ty;
        let iterator = self.target.fresh_ident("I");
        let (name, method) = (&fold.name, &fold.method);
        let borrow = fold.borrow();
        // By value the trait's parameter is its default, `Self`.
        let items = fold.lifetime.map(|_| quote! { <#borrow #ty> });
        let raw = &body.tokens;
        let result =
            self.result_constructed(body.fails, translation, quote! { raw });
        return self.impl_trait_with(
            false,
            &fold.parameters(),
            &[],
            quote! { ::core::iter::#name #items },
            quote! {
                #[inline(always)]
                #track_caller
                fn #method<#iterator>(iter: #iterator) -> Self
                where
                    #iterator: ::core::iter::Iterator<Item = #borrow Self>,
                {
                    let raw = #raw;
                    return #result;
                }
            },
        );
    }

    /// A fold as an impl of the `Result` that passes `failure` on: every
    /// step goes through `constructor`, and so does `core`'s fold of no
    /// items, the value of an empty iterator.
    fn fold_passing(
        &self,
        fold: &Fold,
        failure: &TokenStream,
        constructor: &Path,
        track_caller: &Option<TokenStream>,
    ) -> TokenStream {
        let ty = &self.ctx.ty;
        let el = &self.el;
        let iterator = self.target.fresh_ident("I");
        let (name, method) = (&fold.name, &fold.method);
        let borrow = fold.borrow();
        let (impl_generics, where_clause) =
            self.target.impl_header(&fold.parameters(), &[]);
        // `Self` is the `Result` here: the constructor is the target's.
        let constructor = constructor.segments.iter().skip(1);
        let constructor = quote! { <#ty>::#(#constructor)::* };
        let item = fold.item();
        let empty = self.compute(OpRef::Fold {
            method,
            items: &quote! { empty },
        });
        let step = self.compute(OpRef::FoldStep {
            spec: fold.spec,
            item: &item,
        });
        let (empty, step) = (empty.tokens, step.tokens);
        return quote! {
            impl #impl_generics ::core::iter::#name<#borrow #ty>
                for ::core::result::Result<#ty, #failure>
            #where_clause {
                #[inline(always)]
                #track_caller
                fn #method<#iterator>(iter: #iterator) -> Self
                where
                    #iterator: ::core::iter::Iterator<Item = #borrow #ty>,
                {
                    let mut iter = iter;
                    let mut folded = match iter.next() {
                        ::core::option::Option::Some(it) => #item,
                        ::core::option::Option::None => {
                            let empty = ::core::iter::empty::<#el>();
                            return #constructor(
                                ::core::option::Option::Some(#empty),
                            );
                        }
                    };
                    for it in iter {
                        let value = #step;
                        folded = #constructor(
                            ::core::option::Option::Some(value),
                        )?;
                    }
                    return ::core::result::Result::Ok(folded);
                }
            }
        };
    }
}
