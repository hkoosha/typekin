use crate::friendship::cfg::Friend;
use crate::integral::ValidationCfg;
use crate::runner;
use crate::runner::MkErr;
use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use std::collections::BTreeSet;
use std::fmt::{
    Debug,
    Formatter,
};
use syn::{
    Fields,
    ItemStruct,
    LitBool,
    LitStr,
    Path,
    Type,
    parse::{
        Parse,
        ParseStream,
    },
};

pub(crate) fn text(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let cfg = syn::parse_macro_input!(attr as TextCfg);
    let item = syn::parse_macro_input!(item as ItemStruct);

    return runner::catching(move || TextMaker::new(item, cfg)?.ekran());
}

#[derive(Default, Clone)]
pub(crate) struct TextCfg {
    pub(crate) konst: bool,
    pub(crate) callbacks: Vec<Path>,
    pub(crate) values: Vec<LitStr>,
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) display: bool,
}

impl Debug for TextCfg {
    fn fmt(
        &self,
        formatter: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return formatter
            .debug_struct("TextCfg")
            .field("konst", &self.konst)
            .field("callbacks", &self.callbacks.len())
            .field("values", &self.values.len())
            .field("friends", &self.friends)
            .field("display", &self.display)
            .finish();
    }
}

impl TextCfg {
    fn parse_values(input: ParseStream) -> syn::Result<Vec<LitStr>> {
        let values = runner::list::<LitStr>(input)?.collect::<Vec<_>>();
        if values.is_empty() {
            return input
                .span()
                .fail("text `in` requires at least one string literal");
        }

        let mut seen = BTreeSet::new();
        for value in &values {
            if !seen.insert(value.value()) {
                return value.span().fail("duplicated text literal");
            }
        }

        return Ok(values);
    }

    fn parse_display(
        &mut self,
        input: ParseStream,
        enabled: bool,
    ) -> syn::Result<()> {
        for flag in runner::list::<Ident>(input)? {
            if flag != "display" {
                return flag.fail("unknown text generation flag");
            }
            self.display = enabled;
        }

        return Ok(());
    }

    fn parse_friends(
        &mut self,
        input: ParseStream,
    ) -> syn::Result<()> {
        let friends = runner::list::<Friend>(input)?.collect::<BTreeSet<_>>();

        for friend in &friends {
            let Some(ty) = &friend.ty
            else {
                return input
                    .span()
                    .fail("text friends require a concrete type");
            };
            let mut has_make = false;

            for capability in &friend.capabilities {
                match capability.to_string().as_str() {
                    "Make" => has_make = true,
                    "Rel" => {
                        if !ty.is_ident("Self") {
                            return capability
                                .fail("text `Rel` is only available for Self");
                        }
                    }
                    _ => return capability.fail("unknown text capability"),
                }
            }

            if !has_make && friend.trusted {
                return input.span().fail("text `trusted` requires `Make`");
            }
            if has_make && friend.conv.is_none() {
                return input
                    .span()
                    .fail("text Make friend requires `conv = ...`");
            }
        }

        self.friends = friends;
        return Ok(());
    }

    fn has_validation(&self) -> bool {
        return !self.callbacks.is_empty() || !self.values.is_empty();
    }
}

impl Parse for TextCfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        runner::parse_inner_attributes(input, |attr, rest| {
            match attr {
                "konst" => this.konst = rest.parse::<LitBool>()?.value,
                "valid" => {
                    this.callbacks = ValidationCfg::parse_callbacks(rest)?
                }
                "in" => this.values = Self::parse_values(rest)?,
                "friends" => this.parse_friends(rest)?,
                "with" => this.parse_display(rest, true)?,
                "without" => this.parse_display(rest, false)?,
                _ => return Ok(false),
            }

            return Ok(true);
        })?;

        return Ok(this);
    }
}

struct TextMaker {
    item: ItemStruct,
    ty: Ident,
    cfg: TextCfg,
}

impl TextMaker {
    fn new(
        item: ItemStruct,
        cfg: TextCfg,
    ) -> syn::Result<Self> {
        if !item.generics.params.is_empty()
            || item.generics.where_clause.is_some()
        {
            return item.fail("text does not support generic structs");
        }
        if !runner::find_repr_transparent(&item.attrs)? {
            return item.fail("expecting #[repr(transparent, ...)]");
        }
        if !matches!(
            &item.fields,
            Fields::Unnamed(fields)
                if fields.unnamed.len() == 1
                    && matches!(
                        &fields.unnamed[0].ty,
                        Type::Path(path) if path.qself.is_none() && path.path.is_ident("String")
                    )
        ) {
            return item
                .fail("expected a tuple struct with exactly one String field");
        }

        return Ok(Self {
            ty: item.ident.clone(),
            item,
            cfg,
        });
    }

    fn validation_condition(&self) -> TokenStream {
        let callbacks = self
            .cfg
            .callbacks
            .iter()
            .map(|callback| quote! { #callback(value.as_str()) })
            .reduce(|left, right| quote! { (#left) && (#right) });
        let values = self
            .cfg
            .values
            .iter()
            .map(|value| quote! { value.as_str() == #value })
            .reduce(|left, right| quote! { (#left) || (#right) });

        return match (callbacks, values) {
            (Some(callbacks), Some(values)) => {
                quote! { (#callbacks) && (#values) }
            }
            (Some(callbacks), None) => callbacks,
            (None, Some(values)) => values,
            (None, None) => quote! { true },
        };
    }

    fn make_friendship(&self) -> TokenStream {
        let ty = &self.ty;
        let konst = runner::konst(self.cfg.konst);
        let bonst = runner::bonst(self.cfg.konst);
        let destruct = runner::destruct(self.cfg.konst);
        let make = format_ident!("Make");
        let seal = format_ident!("TextMake");
        let friends = self
            .cfg
            .friends
            .iter()
            .filter(|friend| {
                friend.capabilities.contains(&make)
            })
            .map(|friend| {
                let friend_ty = friend.ty.as_ref().expect("text friend type missing");
                let friend_ty = match friend_ty.is_ident("Self") {
                    true => quote! { #ty },
                    false => quote! { #friend_ty },
                };
                let conv = friend.conv.as_ref().expect("text Make conversion missing");
                let make = if friend.trusted {
                    quote! {
                        return #ty(#conv(self));
                    }
                }
                else {
                    quote! {
                        return match #ty::try_make(#conv(self)) {
                            ::core::result::Result::Ok(value) => value,
                            ::core::result::Result::Err(()) => {
                                ::core::panic!("invalid value from untrusted text friend")
                            }
                        };
                    }
                };

                return quote! {
                    #konst impl #seal for #friend_ty {
                        #[inline(always)]
                        fn make_text(self) -> #ty {
                            #make
                        }
                    }
                };
            });

        if self
            .cfg
            .friends
            .iter()
            .all(|friend| !friend.capabilities.contains(&make))
        {
            return TokenStream::new();
        }

        return quote! {
            #konst trait #seal {
                fn make_text(self) -> #ty;
            }

            #(#friends)*

            impl #ty {
                #[allow(private_bounds)]
                #[inline(always)]
                pub #konst fn of<T>(value: T) -> Self
                where
                    T: #bonst #seal #destruct,
                {
                    return #seal::make_text(value);
                }
            }
        };
    }

    fn ekran(self) -> syn::Result<TokenStream> {
        let item = &self.item;
        let ty = &self.ty;
        let konst = runner::konst(self.cfg.konst);
        let validation = self.validation_condition();
        let display = self.cfg.display.then(|| {
            quote! {
                impl ::core::fmt::Display for #ty {
                    #[inline(always)]
                    fn fmt(
                        &self,
                        formatter: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        return ::core::fmt::Display::fmt(self.0.as_str(), formatter);
                    }
                }
            }
        });
        let infallible_from = (!self.cfg.has_validation()).then(|| {
            quote! {
                #konst impl ::core::convert::From<::alloc::string::String> for #ty {
                    #[inline(always)]
                    fn from(value: ::alloc::string::String) -> Self {
                        return Self(value);
                    }
                }

                #konst impl ::core::convert::From<&str> for #ty {
                    #[inline(always)]
                    fn from(value: &str) -> Self {
                        return Self(::alloc::string::String::from(value));
                    }
                }
            }
        });
        let friendship = self.make_friendship();

        let generated = quote! {
            impl #ty {
                #[inline(always)]
                #konst fn is_valid(value: &::alloc::string::String) -> bool {
                    #validation
                }

                #[inline(always)]
                pub #konst fn try_make(
                    value: ::alloc::string::String,
                ) -> ::core::result::Result<Self, ()> {
                    return if Self::is_valid(&value) {
                        ::core::result::Result::Ok(Self(value))
                    }
                    else {
                        ::core::result::Result::Err(())
                    };
                }

                #[inline(always)]
                pub #konst fn try_from_str(
                    value: &str,
                ) -> ::core::result::Result<Self, ()> {
                    return Self::try_make(::alloc::string::String::from(value));
                }

                #[inline(always)]
                pub #konst fn map<F>(
                    self,
                    function: F,
                ) -> ::core::result::Result<Self, ()>
                where
                    F: ::core::ops::FnOnce(&mut ::alloc::string::String),
                {
                    let mut value = self.0;
                    function(&mut value);
                    return Self::try_make(value);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn into_inner(self) -> ::alloc::string::String {
                    return self.0;
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn into_bytes(self) -> ::alloc::vec::Vec<u8> {
                    return self.0.into_bytes();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn as_str(&self) -> &str {
                    return self.0.as_str();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn as_bytes(&self) -> &[u8] {
                    return self.0.as_bytes();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn len(&self) -> usize {
                    return self.0.len();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn is_empty(&self) -> bool {
                    return self.0.is_empty();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn capacity(&self) -> usize {
                    return self.0.capacity();
                }
            }

            impl ::core::convert::AsRef<str> for #ty {
                #[inline(always)]
                fn as_ref(&self) -> &str {
                    return self.as_str();
                }
            }

            impl ::core::convert::AsRef<[u8]> for #ty {
                #[inline(always)]
                fn as_ref(&self) -> &[u8] {
                    return self.as_bytes();
                }
            }

            impl ::core::borrow::Borrow<str> for #ty {
                #[inline(always)]
                fn borrow(&self) -> &str {
                    return self.as_str();
                }
            }

            impl ::core::ops::Deref for #ty {
                type Target = str;

                #[inline(always)]
                fn deref(&self) -> &Self::Target {
                    return self.as_str();
                }
            }

            impl ::core::fmt::Debug for #ty {
                #[inline(always)]
                fn fmt(
                    &self,
                    formatter: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    return ::core::fmt::Debug::fmt(self.as_str(), formatter);
                }
            }

            impl ::core::hash::Hash for #ty {
                #[inline(always)]
                fn hash<H>(
                    &self,
                    state: &mut H,
                )
                where
                    H: ::core::hash::Hasher,
                {
                    return ::core::hash::Hash::hash(self.as_str(), state);
                }
            }

            impl ::core::cmp::PartialEq for #ty {
                #[inline(always)]
                fn eq(
                    &self,
                    other: &Self,
                ) -> bool {
                    return self.as_str() == other.as_str();
                }
            }

            impl ::core::cmp::Eq for #ty {}

            impl ::core::cmp::PartialOrd for #ty {
                #[inline(always)]
                fn partial_cmp(
                    &self,
                    other: &Self,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    return ::core::option::Option::Some(self.cmp(other));
                }
            }

            impl ::core::cmp::Ord for #ty {
                #[inline(always)]
                fn cmp(
                    &self,
                    other: &Self,
                ) -> ::core::cmp::Ordering {
                    return self.as_str().cmp(other.as_str());
                }
            }

            impl ::core::convert::From<#ty> for ::alloc::string::String {
                #[inline(always)]
                fn from(value: #ty) -> Self {
                    return value.into_inner();
                }
            }

            #infallible_from
            #display
            #friendship
        };

        return Ok(quote! {
            #item

            #[allow(dead_code)]
            #[allow(unused_qualifications)]
            const _: () = {
                #generated
            };
        });
    }
}
