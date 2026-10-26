use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::fmt::{
    Debug,
    Formatter,
};

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::spanned::Spanned;
use syn::{
    Fields,
    ItemStruct,
    LitBool,
    LitStr,
    Path,
    Type,
};

use crate::friendship::cfg::Friend;
use crate::zz::{
    Konst,
    Konsts,
};
use crate::zz_runner::{
    self,
    MkErr,
    ToCollection,
    mk_flags,
};
use crate::{
    marker_tree,
    zz,
};

pub(crate) fn text(
    attr: Cfg,
    item: ItemStruct,
) -> proc_macro::TokenStream {
    return zz_runner::catching(move || {
        let repr = attr.flags.repr.then(|| quote! { #[repr(transparent)] });
        let derive_clone =
            attr.flags.clone.then(|| quote! { #[derive(Clone)] });
        let stream = TextMaker::new(item, attr)?.ekran()?;

        return Ok(quote! {
            #repr
            #derive_clone
            #stream
        });
    })
    .into();
}

mk_flags! {
    #[flag_default(bool=true, str="")]
    #[derive(Debug, Clone)]
    pub(crate) struct TextFlags {
        pub clone: bool,
        pub debug: bool,
        pub display: bool,
        pub hash: bool,
        pub repr: bool,
        pub serde: bool = false,
    }
}

#[derive(Clone)]
pub(crate) struct Cfg {
    pub(crate) konst: Konst,
    konst_flags: BTreeMap<Konsts, bool>,
    seen_flags: BTreeMap<String, bool>,
    pub(crate) std: bool,
    pub(crate) tree: Option<marker_tree::Cfg>,
    pub(crate) callbacks: Vec<Path>,
    pub(crate) values: Option<Vec<LitStr>>,
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) flags: TextFlags,
}

impl Cfg {
    fn parse_flags(
        &mut self,
        input: ParseStream,
        value_if_seen: bool,
    ) -> syn::Result<()> {
        let attrs = zz::one_or_list::<Ident>(input)?;
        let mut duplicated = BTreeMap::new();

        for attr in attrs {
            let name = attr.to_string();
            let span = attr.span();

            if duplicated.insert(name.clone(), span).is_some() {
                return Err(syn::Error::new(span, "duplicated flags"));
            }

            if self
                .seen_flags
                .get(&name)
                .is_some_and(|value| *value != value_if_seen)
            {
                return Err(syn::Error::new(
                    span,
                    "flag specified in both `with` and `without`",
                ));
            }

            if name.starts_with("konst_") {
                let Some(flag) = Self::parse_konst_flags(&name)
                else {
                    return Err(syn::Error::new(span, "unknown flag"));
                };

                self.konst_flags.insert(flag, value_if_seen);
            }
            else if self.flags.set_bool(&name, value_if_seen).is_err() {
                return Err(syn::Error::new(span, "unknown flag"));
            }

            self.seen_flags.insert(name, value_if_seen);
        }

        return Ok(());
    }

    pub(crate) fn konst_for(
        &self,
        flags: &[Konsts],
    ) -> bool {
        for flag in flags {
            if let Some(value) = self.konst_flags.get(flag) {
                return *value;
            }
        }

        return self.konst.enables(flags);
    }

    fn parse_values(input: ParseStream) -> syn::Result<Vec<LitStr>> {
        let values = zz::list::<LitStr>(input)?.vec();

        let mut seen = BTreeSet::new();
        for value in &values {
            if !seen.insert(value.value()) {
                return value.fail("duplicated text literal");
            }
        }

        return Ok(values);
    }

    fn parse_friends(
        &mut self,
        input: ParseStream,
    ) -> syn::Result<()> {
        let mut friends = BTreeSet::new();

        for friend in zz::one_or_list::<Friend>(input)? {
            let mut has_make = false;

            for capability in &friend.capabilities {
                match capability.to_string().as_str() {
                    "Make" => has_make = true,
                    "UnsafeTrust" => {}
                    "Rel" => {
                        if !friend.ty.is_ident("Self") {
                            return capability
                                .fail("text `Rel` is only available for Self");
                        }
                    }
                    _ => return capability.fail("unknown text capability"),
                }
            }

            if has_make && friend.conv.is_none() {
                return input.span().fail(
                    "text Make friend requires `conversion(Type) -> Make`",
                );
            }
            let span = friend.ty.span();
            if !friends.insert(friend) {
                return span.fail("duplicated text friend");
            }
        }

        self.friends = friends;
        return Ok(());
    }

    fn has_validation(&self) -> bool {
        return !self.callbacks.is_empty() || self.values.is_some();
    }

    pub(crate) fn new(
        konst: Konst,
        std: bool,
    ) -> Self {
        return Self {
            konst,
            konst_flags: Default::default(),
            seen_flags: Default::default(),
            std,
            tree: None,
            callbacks: vec![],
            values: None,
            friends: BTreeSet::new(),
            flags: Default::default(),
        };
    }

    fn parse_konst_flags(name: &str) -> Option<Konsts> {
        return Some(match name {
            "konst_friendship" => Konsts::Friendship,
            "konst_from" => Konsts::From,
            "konst_text" => Konsts::Text,
            "konst_text_access" => Konsts::TextAccess,
            "konst_text_friendship" => Konsts::TextFriendship,
            "konst_text_from" => Konsts::TextFrom,
            _ => return None,
        });
    }
}

impl Debug for Cfg {
    fn fmt(
        &self,
        formatter: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return formatter
            .debug_struct("TextCfg")
            .field("konst", &self.konst)
            .field("std", &self.std)
            .field("callbacks", &self.callbacks.len())
            .field("tree", &self.tree.is_some())
            .field("values", &self.values.as_ref().map(Vec::len))
            .field("friends", &self.friends)
            .field("flags", &self.flags)
            .finish();
    }
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::new(Konst::default(), true);

        zz::parse_inner(input, |attr, rest| {
            match attr {
                "std" => this.std = rest.parse::<LitBool>()?.value,
                "konst" => {
                    this.konst = rest.parse()?;
                }
                "error" => zz::parse_unit_error(rest)?,
                "valid" => {
                    if this.values.is_some() {
                        rest.span().fail(
                            "`valid` and `domain` cannot be used together",
                        )?;
                    }
                    this.callbacks = zz::parse_callbacks(rest)?;
                }
                "domain" => {
                    if !this.callbacks.is_empty() {
                        rest.span().fail(
                            "`valid` and `domain` cannot be used together",
                        )?;
                    }
                    this.values = Some(Self::parse_values(rest)?);
                }
                "in" => {
                    rest.span().fail("`in` was renamed to `domain`")?;
                }
                "friend" => this.parse_friends(rest)?,
                "tree" => {
                    this.tree = Some(marker_tree::Cfg::parse_attribute(rest)?);
                }
                "with" => this.parse_flags(rest, true)?,
                "without" => this.parse_flags(rest, false)?,
                _ => return Ok(false),
            }

            return Ok(true);
        })?;

        return Ok(this);
    }
}

pub(crate) struct TextMaker {
    item: ItemStruct,
    ty: Ident,
    cfg: Cfg,
}

impl TextMaker {
    pub(crate) fn new(
        item: ItemStruct,
        cfg: Cfg,
    ) -> syn::Result<Self> {
        if !item.generics.params.is_empty() {
            return item
                .generics
                .params
                .fail("text does not support generic structs");
        }
        if let Some(clause) = &item.generics.where_clause {
            return clause.fail("text does not support generic structs");
        }
        let shape_error =
            "expected a tuple struct with exactly one String field";
        let field = match &item.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                &fields.unnamed[0]
            }
            Fields::Unit => return item.ident.fail(shape_error),
            fields => return fields.fail(shape_error),
        };
        if !matches!(
            &field.ty,
            Type::Path(path) if path.qself.is_none() && path.path.is_ident("String")
        ) {
            return field.ty.fail(shape_error);
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
        let values = self.cfg.values.as_ref().map(|values| {
            return values
                .iter()
                .map(|value| quote! { value.as_str() == #value })
                .reduce(|left, right| quote! { (#left) || (#right) })
                .unwrap_or_else(|| quote! { false });
        });

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
        let is_konst = self.cfg.konst_for(&[
            Konsts::TextFriendship,
            Konsts::Friendship,
            Konsts::Text,
        ]);
        let konst = zz_runner::konst(is_konst);
        let bonst = zz_runner::bonst(is_konst);
        let destruct = zz_runner::destruct(is_konst);
        let make = format_ident!("Make");
        let trusted = crate::friendship::cfg::trust_capability();
        let seal = format_ident!("TextMake");
        let friends = self
            .cfg
            .friends
            .iter()
            .filter(|friend| {
                friend.capabilities.contains(&make)
            })
            .map(|friend| {
                let friend_ty = &friend.ty;
                let friend_ty = match friend.ty.is_ident("Self") {
                    true => quote! { #ty },
                    false => quote! { #friend_ty },
                };
                let conv = friend.conv.as_ref().expect("text Make conversion missing");
                let make = if friend.capabilities.contains(&trusted) {
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
        let tree = self
            .cfg
            .tree
            .as_ref()
            .map(|tree| tree.expand_embedded(&item.ident, &item.vis))
            .transpose()?;
        let ty = &self.ty;
        let ty_string = match self.cfg.std {
            true => quote! { ::std::string::String },
            false => quote! { ::alloc::string::String },
        };
        let ty_vec = match self.cfg.std {
            true => quote! { ::std::vec::Vec },
            false => quote! { ::alloc::vec::Vec },
        };
        let is_konst = self.cfg.konst_for(&[Konsts::Text]);
        let konst = zz_runner::konst(is_konst);
        let bonst = TokenStream::new();
        let destruct = TokenStream::new();
        let access_konst = zz_runner::konst(self.cfg.konst_for(&[
            Konsts::TextAccess,
            Konsts::Text,
        ]));
        let validation = self.validation_condition();
        let display = self.cfg.flags.display.then(|| {
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
        let debug = self.cfg.flags.debug.then(|| {
            let name = ty.to_string();
            quote! {
                impl ::core::fmt::Debug for #ty {
                    #[inline(always)]
                    fn fmt(
                        &self,
                        formatter: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        return formatter
                            .debug_tuple(#name)
                            .field(&self.as_str())
                            .finish();
                    }
                }
            }
        });
        let hash = self.cfg.flags.hash.then(|| {
            quote! {
                impl ::core::hash::Hash for #ty {
                    #[inline(always)]
                    fn hash<H>(
                        &self,
                        state: &mut H,
                    )
                    where
                        H: ::core::hash::Hasher,
                    {
                        return ::core::hash::Hash::hash(
                            self.as_str(),
                            state,
                        );
                    }
                }
            }
        });
        let infallible_from = (!self.cfg.has_validation()).then(|| {
            let konst = zz_runner::konst(self.cfg.konst_for(&[
                Konsts::TextFrom,
                Konsts::From,
                Konsts::Text,
            ]));
            quote! {
                #konst impl ::core::convert::From<#ty_string> for #ty {
                    #[inline(always)]
                    fn from(value: #ty_string) -> Self {
                        return Self(value);
                    }
                }

                impl ::core::convert::From<&str> for #ty {
                    #[inline(always)]
                    fn from(value: &str) -> Self {
                        return Self(#ty_string::from(value));
                    }
                }
            }
        });
        let friendship = self.make_friendship();
        let serde = self.cfg.flags.serde.then(|| {
            quote! {
                impl ::serde::Serialize for #ty {
                    #[inline(always)]
                    fn serialize<S>(
                        &self,
                        serializer: S,
                    ) -> ::core::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::Serializer,
                    {
                        return <str as ::serde::Serialize>::serialize(
                            self.as_str(),
                            serializer,
                        );
                    }
                }

                impl<'de> ::serde::Deserialize<'de> for #ty {
                    #[inline(always)]
                    fn deserialize<D>(
                        deserializer: D,
                    ) -> ::core::result::Result<Self, D::Error>
                    where
                        D: ::serde::Deserializer<'de>,
                    {
                        let value =
                            <#ty_string as ::serde::Deserialize<'de>>::
                                deserialize(deserializer)?;

                        return Self::try_make(value).map_err(|_| {
                            <D::Error as ::serde::de::Error>::custom(
                                "invalid value",
                            )
                        });
                    }
                }
            }
        });

        let generated = quote! {
            impl #ty {
                #[inline(always)]
                #konst fn is_valid(value: &#ty_string) -> bool {
                    #validation
                }

                #[inline(always)]
                pub #konst fn try_make(
                    value: #ty_string,
                ) -> ::core::result::Result<Self, ()> {
                    return if Self::is_valid(&value) {
                        ::core::result::Result::Ok(Self(value))
                    }
                    else {
                        ::core::result::Result::Err(())
                    };
                }

                #[inline(always)]
                pub fn try_from_str(
                    value: &str,
                ) -> ::core::result::Result<Self, ()> {
                    return Self::try_make(#ty_string::from(value));
                }

                #[inline(always)]
                pub fn map<F>(
                    self,
                    function: F,
                ) -> ::core::result::Result<Self, ()>
                where
                    F: #bonst ::core::ops::FnOnce(&mut #ty_string) #destruct,
                {
                    let mut value = self.0;
                    function(&mut value);
                    return Self::try_make(value);
                }

                #[inline(always)]
                pub fn try_push(self, character: char) -> ::core::result::Result<Self, ()> {
                    return self.map(|value| value.push(character));
                }

                #[inline(always)]
                pub fn try_push_str(self, text: &str) -> ::core::result::Result<Self, ()> {
                    return self.map(|value| value.push_str(text));
                }

                #[inline(always)]
                pub fn try_insert(self, index: usize, character: char) -> ::core::result::Result<Self, ()> {
                    return self.map(|value| value.insert(index, character));
                }

                #[inline(always)]
                pub fn try_insert_str(self, index: usize, text: &str) -> ::core::result::Result<Self, ()> {
                    return self.map(|value| value.insert_str(index, text));
                }

                #[inline(always)]
                pub fn try_replace_range<R>(self, range: R, text: &str) -> ::core::result::Result<Self, ()>
                where
                    R: #bonst ::core::ops::RangeBounds<usize> #destruct,
                {
                    return self.map(|value| value.replace_range(range, text));
                }

                #[inline(always)]
                pub fn try_truncate(self, length: usize) -> ::core::result::Result<Self, ()> {
                    return self.map(|value| value.truncate(length));
                }

                #[inline(always)]
                pub fn try_clear(self) -> ::core::result::Result<Self, ()> {
                    return self.map(|value| value.clear());
                }

                #[inline(always)]
                pub fn try_retain<F>(self, mut predicate: F) -> ::core::result::Result<Self, ()>
                where
                    F: #bonst ::core::ops::FnMut(char) -> bool #destruct,
                {
                    return self.map(|value| value.retain(|character| predicate(character)));
                }

                #[inline(always)]
                pub fn try_remove(self, index: usize) -> ::core::result::Result<(Self, char), ()> {
                    let mut value = self.0;
                    let removed = value.remove(index);
                    return match Self::try_make(value) {
                        ::core::result::Result::Ok(value) => ::core::result::Result::Ok((value, removed)),
                        ::core::result::Result::Err(()) => ::core::result::Result::Err(()),
                    };
                }

                #[inline(always)]
                pub fn try_pop(self) -> ::core::result::Result<(Self, ::core::option::Option<char>), ()> {
                    let mut value = self.0;
                    let popped = value.pop();
                    return match Self::try_make(value) {
                        ::core::result::Result::Ok(value) => ::core::result::Result::Ok((value, popped)),
                        ::core::result::Result::Err(()) => ::core::result::Result::Err(()),
                    };
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn into_inner(self) -> #ty_string {
                    return self.0;
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn into_bytes(self) -> #ty_vec<u8> {
                    return self.0.into_bytes();
                }

                #[must_use]
                #[inline(always)]
                pub #access_konst fn as_str(&self) -> &str {
                    return self.0.as_str();
                }

                #[must_use]
                #[inline(always)]
                pub #access_konst fn as_bytes(&self) -> &[u8] {
                    return self.0.as_bytes();
                }

                #[must_use]
                #[inline(always)]
                pub #access_konst fn len(&self) -> usize {
                    return self.0.len();
                }

                #[must_use]
                #[inline(always)]
                pub #access_konst fn is_empty(&self) -> bool {
                    return self.0.is_empty();
                }

                #[must_use]
                #[inline(always)]
                pub #access_konst fn capacity(&self) -> usize {
                    return self.0.capacity();
                }
            }

            impl ::core::ops::Add<&str> for #ty {
                type Output = Self;

                #[inline(always)]
                fn add(self, text: &str) -> Self {
                    return match self.try_push_str(text) {
                        ::core::result::Result::Ok(value) => value,
                        ::core::result::Result::Err(()) => ::core::panic!("invalid value from text addition"),
                    };
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

            #debug

            #hash

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

            impl ::core::convert::From<#ty> for #ty_string {
                #[inline(always)]
                fn from(value: #ty) -> Self {
                    return value.into_inner();
                }
            }

            #infallible_from
            #display
            #friendship
            #serde
        };

        return Ok(quote! {
            #item
            #tree


            #[allow(dead_code)]
            #[allow(unused_qualifications)]
            const _: () = {
                #generated
            };
        });
    }
}
