use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::str::FromStr;

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
    ItemEnum,
    bracketed,
    parse_quote,
};

use crate::friendship::cfg::Friend;
use crate::friendship::{
    Protocol,
    ProtocolFriend,
};
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
use crate::zz_value_type::N;
use crate::{
    integral,
    marker_tree,
    zz,
};

pub(crate) fn bitflag(
    attr: Box<Cfg>,
    item: ItemEnum,
) -> proc_macro::TokenStream {
    let derive_clone = attr.bit.clone.then(|| match attr.konst {
        Konst::Nightly => quote! { #[derive_const(Clone)] },
        Konst::None | Konst::Stable => quote! { #[derive(Clone)] },
    });
    let derive_comparison = match attr.konst {
        Konst::Nightly => {
            quote! { #[derive_const(Eq, PartialEq, Ord, PartialOrd)] }
        }
        Konst::None | Konst::Stable => {
            quote! { #[derive(Eq, PartialEq, Ord, PartialOrd)] }
        }
    };
    let derive_copy = attr.bit.copy.then(|| quote! { #[derive(Copy)] });
    let derive_hash = attr.bit.hash.then(|| quote! { #[derive(Hash)] });
    let derive_debug = attr.bit.debug.then(|| quote! { #[derive(Debug)] });
    let attributes = quote! {
        #derive_clone
        #derive_comparison
        #derive_copy
        #derive_hash
        #derive_debug
    };

    return ekran(attr, item, attributes);
}

fn ekran(
    attr: Box<Cfg>,
    item: ItemEnum,
    attributes: TokenStream,
) -> proc_macro::TokenStream {
    return zz_runner::catching(move || {
        let attrs = &item.attrs;
        let span = item.span();
        let item_repr = zz::find_repr_n_optional(attrs, &span)?;
        let repr = resolve_repr(attr.repr, item_repr, span)?;
        let repr_attr = attr.repr.map(|it| {
            let it = format_ident!("{}", it.rust_name());
            quote! { #[repr(#it)] }
        });
        let ty = item.ident.clone();
        let items = item.variants.iter().map(|it| it.ident.clone()).vec();

        if let Some(bad) = item
            .variants
            .iter()
            .find(|it| !matches!(it.fields, Fields::Unit))
        {
            return bad.fail("only unit variants are supported");
        }

        let tree = attr
            .tree
            .as_ref()
            .map(|tree| tree.expand_embedded(&item.ident, &item.vis))
            .transpose()?;

        let stream = Maker::new(ty, repr, attr, items).ekran()?;
        return Ok(quote! { #attributes #repr_attr #item #tree #stream });
    })
    .into();
}

pub(crate) fn resolve_repr(
    attr_repr: Option<N>,
    item_repr: Option<N>,
    span: impl Spanned,
) -> syn::Result<N> {
    if attr_repr.is_some() && item_repr.is_some() {
        return span
            .fail("repr cannot be specified on both bitflag and the enum");
    }

    return match attr_repr.or(item_repr) {
        Some(repr) => Ok(repr),
        None => span.fail(
            "bitflag requires `repr = <numeric type>` or \
             `#[repr(<numeric type>)]` on the enum",
        ),
    };
}

pub(crate) struct Maker {
    repr: N,
    el: Ident,
    ty: Ident,
    vl: Ident,
    cfg: Box<Cfg>,
    items: Vec<Ident>,
}

impl Maker {
    pub(crate) fn new(
        ty: Ident,
        repr: N,
        cfg: Box<Cfg>,
        items: Vec<Ident>,
    ) -> Self {
        let mut this = Self {
            vl: match (cfg.bit.value_name.as_str(), cfg.bit.suffix.as_str()) {
                ("", "") => format_ident!("{}Value", ty),
                ("", suffix) => format_ident!("{}{}", ty, suffix),
                (prefix, "") => format_ident!("{}", prefix),
                (prefix, suffix) => format_ident!("{}{}", prefix, suffix),
            },
            el: format_ident!("{}", repr.rust_name()),
            repr,
            ty: ty.clone(),
            cfg,
            items,
        };

        this.cfg.int.flags.math_not = false;
        this.fix_friendship();

        return this;
    }

    fn fix_friendship(&mut self) {
        let ty = &self.ty;
        let vl = &self.vl;

        // enum :: enum, Make & Bit & Cmp.
        {
            let req = Friend::from(ty);
            let mut req = self.cfg.friends.take(&req).unwrap_or(req);
            req.capabilities.insert(format_ident!("Cmp"));
            req.capabilities.insert(format_ident!("Bit"));
            req.capabilities.insert(format_ident!("Make"));
            req.conv = Some(parse_quote! { #ty::raw });
            assert!(self.cfg.friends.insert(req));
        }

        // enum :: value, Make & Bit.
        {
            let req = Friend::from(vl);
            let mut req = self.cfg.friends.take(&req).unwrap_or(req);
            req.capabilities.insert(format_ident!("Bit"));
            req.capabilities.insert(format_ident!("Make"));
            req.conv = Some(parse_quote! { #vl::raw });
            assert!(self.cfg.friends.insert(req));
        }

        // value :: enum, Make & Bit.
        self.cfg.int.add_friend(
            ty,
            [
                format_ident!("Bit"),
                format_ident!("Make"),
            ],
            parse_quote! { #ty::raw },
        );
    }

    pub(crate) fn ekran(mut self) -> syn::Result<TokenStream> {
        let vl = self.ekran_vl();
        let vl_impl = self.ekran_vl_impl();
        let vl_impls = self.ekran_vl_impls();
        let ty_delegate = self.ekran_ty_delegate();
        let ty_impl = self.ekran_ty_impl();
        let ty_impls = self.ekran_ty_impls();
        let iter_impl = self.ekran_iter();
        let friends = self.ekran_friendship()?;
        let value_display = self.cfg.int.flags.display || self.cfg.bit.display;
        let value_from_str = self.cfg.int.flags.from_str;
        let text_impls = self.ekran_text_impls(value_display, value_from_str);
        let serde_impls = self.ekran_serde();
        self.cfg.int.flags.display = false;
        self.cfg.int.flags.from_str = false;
        if self.cfg.bit.serde {
            self.cfg.int.flags.serde = false;
        }

        let impl_int = match self.cfg.bit.impl_value {
            false => TokenStream::new(),
            true => {
                let raw = N::of(self.el.to_string()).ok_or_else(|| {
                    syn::Error::new(self.el.span(), "unknown integral type")
                })?;
                integral::ekran_with(
                    self.vl,
                    self.el,
                    self.cfg.int,
                    integral::StoragePlan::primitive(raw),
                )?
            }
        };

        let it = quote::quote! {
            #vl

            #[allow(dead_code)]
            #[allow(unused_qualifications)]
            #[allow(clippy::unnecessary_cast)]
            const _: () = {
                #impl_int

                #vl_impl

                #vl_impls

                #ty_impl

                #ty_delegate

                #ty_impls

                #iter_impl

                #friends

                #text_impls

                #serde_impls
            };
        };

        return Ok(it);
    }

    // ---------------------------------

    fn ekran_vl(&self) -> TokenStream {
        let ty_value = &self.vl;
        let el = &self.el;
        let derive_clone = if matches!(self.cfg.int.konst, Konst::Nightly) {
            quote! { #[derive_const(Clone)] }
        }
        else {
            quote! { #[derive(Clone)] }
        };
        let repr = self.cfg.bit.repr.then(|| quote! { #[repr(transparent)] });

        return match self.cfg.bit.make_value {
            true => quote! {
                #derive_clone
                #[derive(Copy)]
                #repr
                pub struct #ty_value(#el);
            },
            false => TokenStream::new(),
        };
    }

    fn ekran_vl_impl(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let el = &self.el;
        let konst = zz_runner::konst(self.cfg.int.konst_for(&[
            Konsts::Value,
            Konsts::Bit,
        ]));
        let from_konst =
            zz_runner::konst(self.cfg.konst_for_nightly(&[Konsts::Conversion]));
        let iterator_konst =
            zz_runner::konst(self.cfg.konst_for_nightly(&[Konsts::Iterator]));

        let raw = self
            .cfg
            .int
            .get_raw
            .as_ref()
            .map(|it| quote! { #it })
            .unwrap_or_else(|| quote! { #vl::raw });

        let self1 = &self.cfg.int;
        let from_bits = if self1.validation.has_validation() {
            quote! {
                match Self::try_make(bits) {
                    ::core::result::Result::Ok(value) => value,
                    ::core::result::Result::Err(_) => {
                        ::core::panic!("invalid value")
                    }
                }
            }
        }
        else {
            quote! { Self::_unchecked(bits) }
        };

        let named_values = self.items.iter().map(|it| {
            return quote! {
                #[allow(non_upper_case_globals)]
                pub const #it: Self = Self::_unchecked(#ty::#it as #el);
            };
        });

        return quote! {
            impl #vl {
                #(#named_values)*

                #[must_use]
                #[inline(always)]
                pub #konst fn bits(self) -> #el {
                    return #raw(self);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn from_bits_retain(bits: #el) -> Self {
                    return #from_bits;
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn empty() -> Self {
                    return Self::from_bits_retain(0);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all() -> Self {
                    return #ty::all();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all_named() -> Self {
                    return Self::all();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all_known() -> Self {
                    return Self::all();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all_unknown() -> Self {
                    return Self::from_bits_retain(!Self::all().bits());
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn known_bits(self) -> #el {
                    return self.bits() & Self::all().bits();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn unknown_bits(self) -> #el {
                    return self.bits() & !Self::all().bits();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn into_known_bits(self) -> Self {
                    return Self::from_bits_retain(self.known_bits());
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn into_unknown_bits(self) -> Self {
                    return Self::from_bits_retain(self.unknown_bits());
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn contains_unknown_bits(self) -> bool {
                    return self.unknown_bits() != Self::empty().bits();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn from_bits(bits: #el) -> Option<Self> {
                    let value = Self::from_bits_retain(bits);
                    return if value.contains_unknown_bits() {
                        None
                    }
                    else {
                        Some(value)
                    };
                }

                #[inline(always)]
                pub #konst fn try_as_known_bits_only(self) -> Result<Self, Self> {
                    return if self.contains_unknown_bits() {
                        Err(self)
                    }
                    else {
                        Ok(self)
                    };
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn from_bits_truncate(bits: #el) -> Self {
                    return Self::from_bits_retain(bits).truncated();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn truncated_into_known_bits(self) -> Self {
                    return self.truncated();
                }

                #[inline(always)]
                pub #konst fn truncate_into_known_bits(&mut self) {
                    self.truncate();
                }

                #[must_use]
                #[inline(always)]
                pub #from_konst fn from_name(name: &str) -> Option<Self> {
                    return match #ty::from_name(name) {
                        Some(flag) => Some(flag.into_value()),
                        None => None,
                    };
                }

                #[inline(always)]
                pub #konst fn into_flag(self) -> Result<#ty, Self> {
                    let items = #ty::items();
                    let max = items.len();
                    let mut i = 0;

                    while i < max {
                        let flag = items[i];
                        if flag.into_value().bits() == self.bits() {
                            return Ok(flag);
                        }
                        i += 1;
                    }

                    return Err(self);
                }

                #[must_use]
                #[inline(always)]
                pub #iterator_konst fn iter_known_flags(self) -> impl Iterator<Item = #ty> {
                    return IterFlags {
                        value: self,
                        index: 0,
                    };
                }

                #[must_use]
                #[inline(always)]
                pub #iterator_konst fn iter(self) -> impl Iterator<Item = Self> {
                    return IterValues {
                        value: self,
                        index: 0,
                    };
                }

                #[must_use]
                #[inline(always)]
                pub fn iter_names(self) -> impl Iterator<Item = (&'static str, Self)> {
                    return self
                        .iter_known_flags()
                        .map(|flag| (flag.name(), flag.into_value()));
                }

                #[must_use]
                #[inline(always)]
                pub fn iter_defined_names() -> impl Iterator<Item = (&'static str, Self)> {
                    return #ty::iter().map(|flag| (flag.name(), flag.into_value()));
                }

                #[must_use]
                #[inline(always)]
                pub fn iter_equal_names(self) -> impl Iterator<Item = &'static str> {
                    return #ty::iter()
                        .filter(move |flag| flag.into_value() == self)
                        .map(|flag| flag.name());
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn is_empty(self) -> bool {
                    return self.bits() == 0;
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn is_all(self) -> bool {
                    return self.contains_all(Self::all());
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn is_exactly_all_known_bits(self) -> bool {
                    return self.bits() == Self::all().bits();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn intersects(self, other: Self) -> bool {
                    return self.bits() & other.bits() != 0;
                }

                #[must_use]
                #[inline(always)]
                pub fn contains<T>(self, other: T) -> bool
                where
                    T: Into<Self>,
                {
                    return self.contains_all(other.into());
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn contains_all(self, other: Self) -> bool {
                    return self.bits() & other.bits() == other.bits();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn contains_any(self, other: Self) -> bool {
                    return self.intersects(other);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn truncated(self) -> Self {
                    return Self::from_bits_retain(self.known_bits());
                }

                #[inline(always)]
                pub #konst fn truncate(&mut self) {
                    *self = self.truncated();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn inserted(self, other: Self) -> Self {
                    return Self::from_bits_retain(self.bits() | other.bits());
                }

                #[inline(always)]
                pub #konst fn insert(&mut self, other: Self) {
                    *self = self.inserted(other);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn removed(self, other: Self) -> Self {
                    return Self::from_bits_retain(self.bits() & !other.bits());
                }

                #[inline(always)]
                pub #konst fn remove(&mut self, other: Self) {
                    *self = self.removed(other);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn toggled(self, other: Self) -> Self {
                    return Self::from_bits_retain(self.bits() ^ other.bits());
                }

                #[inline(always)]
                pub #konst fn toggle(&mut self, other: Self) {
                    *self = self.toggled(other);
                }

                #[inline(always)]
                pub #konst fn set(&mut self, other: Self, value: bool) {
                    if value {
                        self.insert(other);
                    }
                    else {
                        self.remove(other);
                    }
                }

                #[inline(always)]
                pub #konst fn clear(&mut self) {
                    *self = Self::empty();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn with(self, other: Self) -> Self {
                    return self.inserted(other);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn without(self, other: Self) -> Self {
                    return self.removed(other);
                }

                #[inline(always)]
                pub #konst fn unset(&mut self, other: Self) {
                    self.remove(other);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn intersection(self, other: Self) -> Self {
                    return Self::from_bits_retain(self.bits() & other.bits());
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn union(self, other: Self) -> Self {
                    return self.inserted(other);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn difference(self, other: Self) -> Self {
                    return self.removed(other);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn symmetric_difference(self, other: Self) -> Self {
                    return self.toggled(other);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn complemented(self) -> Self {
                    return Self::from_bits_retain(!self.bits() & Self::all().bits());
                }

                #[inline(always)]
                pub #konst fn complement(&mut self) {
                    *self = self.complemented();
                }
            }
        };
    }

    fn ekran_text_impls(
        &self,
        display: bool,
        from_str: bool,
    ) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let el = &self.el;
        let unsigned = format_ident!("{}", self.repr.unsigned_rust_name());
        let value = quote! { bits };
        let validation = &self.cfg.int.validation.emit(&value);

        let display = display.then(|| {
            quote! {
                impl ::core::fmt::Display for #vl {
                    #[inline]
                    fn fmt(
                        &self,
                        formatter: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let mut remaining = self.bits();
                        let mut first = true;
                        let items = #ty::items();
                        let mut index = 0;

                        while index < items.len() {
                            let flag = items[index];
                            let bits = flag.raw();
                            index += 1;

                            if bits != 0 && remaining & bits == bits {
                                if !first {
                                    formatter.write_str(" | ")?;
                                }
                                formatter.write_str(flag.name())?;
                                first = false;
                                remaining &= !bits;
                            }
                        }

                        if remaining != 0 {
                            if !first {
                                formatter.write_str(" | ")?;
                            }
                            return ::core::fmt::Write::write_fmt(
                                formatter,
                                ::core::format_args!(
                                    "0x{:x}",
                                    remaining as #unsigned,
                                ),
                            );
                        }

                        return ::core::result::Result::Ok(());
                    }
                }
            }
        });

        let from_str = from_str.then(|| {
            quote! {
                impl ::core::str::FromStr for #ty {
                    type Err = ();

                    #[inline]
                    fn from_str(
                        source: &str,
                    ) -> ::core::result::Result<Self, Self::Err> {
                        return Self::from_name(source.trim()).ok_or(());
                    }
                }

                impl ::core::str::FromStr for #vl {
                    type Err = ();

                    #[inline]
                    fn from_str(
                        source: &str,
                    ) -> ::core::result::Result<Self, Self::Err> {
                        let source = source.trim();
                        let mut bits: #el = 0;

                        if !source.is_empty() {
                            for part in source.split('|') {
                                let part = part.trim();
                                if part.is_empty() {
                                    return ::core::result::Result::Err(());
                                }

                                let value = if let Some(hex) =
                                    part.strip_prefix("0x")
                                {
                                    if hex.is_empty() {
                                        return ::core::result::Result::Err(());
                                    }
                                    match <#unsigned>::from_str_radix(hex, 16) {
                                        ::core::result::Result::Ok(value) => {
                                            value as #el
                                        }
                                        ::core::result::Result::Err(_) => {
                                            return ::core::result::Result::Err(());
                                        }
                                    }
                                }
                                else {
                                    match #ty::from_name(part) {
                                        ::core::option::Option::Some(flag) => {
                                            flag.raw()
                                        }
                                        ::core::option::Option::None => {
                                            return ::core::result::Result::Err(());
                                        }
                                    }
                                };

                                bits |= value;
                            }
                        }

                        return if #validation {
                            ::core::result::Result::Ok(Self(bits))
                        }
                        else {
                            ::core::result::Result::Err(())
                        };
                    }
                }
            }
        });

        return quote! {
            #display
            #from_str
        };
    }

    fn ekran_serde(&self) -> TokenStream {
        if !self.cfg.bit.serde {
            return TokenStream::new();
        }

        let ty = &self.ty;
        let vl = &self.vl;
        let el = &self.el;
        let unsigned = format_ident!("{}", self.repr.unsigned_rust_name());
        let bits = quote! { bits };
        let validation = &self.cfg.int.validation.emit(&bits);
        let serialize_as_name = if self.cfg.bit.serde_with_name {
            quote! { true }
        }
        else if self.cfg.bit.serde_with_value {
            quote! { false }
        }
        else {
            quote! { serializer.is_human_readable() }
        };
        let deserialize_as_name = if self.cfg.bit.serde_with_name {
            quote! { true }
        }
        else if self.cfg.bit.serde_with_value {
            quote! { false }
        }
        else {
            quote! { deserializer.is_human_readable() }
        };

        return quote! {
            struct SerdeFlagValue(#vl);

            impl ::core::fmt::Display for SerdeFlagValue {
                fn fmt(
                    &self,
                    formatter: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    let mut remaining = self.0.bits();
                    let mut first = true;
                    let items = #ty::items();
                    let mut index = 0;

                    while index < items.len() {
                        let flag = items[index];
                        let bits = flag.raw();
                        index += 1;

                        if bits != 0 && remaining & bits == bits {
                            if !first {
                                formatter.write_str(" | ")?;
                            }
                            formatter.write_str(flag.name())?;
                            first = false;
                            remaining &= !bits;
                        }
                    }

                    if remaining != 0 {
                        if !first {
                            formatter.write_str(" | ")?;
                        }
                        return ::core::fmt::Write::write_fmt(
                            formatter,
                            ::core::format_args!(
                                "0x{:x}",
                                remaining as #unsigned,
                            ),
                        );
                    }

                    return ::core::result::Result::Ok(());
                }
            }

            struct SerdeFlagVisitor;

            impl<'de> ::serde::de::Visitor<'de> for SerdeFlagVisitor {
                type Value = #ty;

                fn expecting(
                    &self,
                    formatter: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    return formatter.write_str("a declared flag name");
                }

                fn visit_str<E>(
                    self,
                    value: &str,
                ) -> ::core::result::Result<Self::Value, E>
                where
                    E: ::serde::de::Error,
                {
                    return #ty::from_name(value).ok_or_else(|| {
                        E::custom("unknown flag name")
                    });
                }
            }

            struct SerdeFlagValueVisitor;

            impl<'de> ::serde::de::Visitor<'de> for SerdeFlagValueVisitor {
                type Value = #vl;

                fn expecting(
                    &self,
                    formatter: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    return formatter.write_str(
                        "a `|` separated string of flag names and hexadecimal bits",
                    );
                }

                fn visit_str<E>(
                    self,
                    source: &str,
                ) -> ::core::result::Result<Self::Value, E>
                where
                    E: ::serde::de::Error,
                {
                    let source = source.trim();
                    let mut bits: #el = 0;

                    if !source.is_empty() {
                        for part in source.split('|') {
                            let part = part.trim();
                            if part.is_empty() {
                                return ::core::result::Result::Err(
                                    E::custom("empty flag"),
                                );
                            }

                            let value = if let Some(hex) =
                                part.strip_prefix("0x")
                            {
                                if hex.is_empty() {
                                    return ::core::result::Result::Err(
                                        E::custom("invalid hexadecimal flag bits"),
                                    );
                                }
                                match <#unsigned>::from_str_radix(hex, 16) {
                                    ::core::result::Result::Ok(value) => {
                                        value as #el
                                    }
                                    ::core::result::Result::Err(_) => {
                                        return ::core::result::Result::Err(
                                            E::custom(
                                                "invalid hexadecimal flag bits",
                                            ),
                                        );
                                    }
                                }
                            }
                            else {
                                match #ty::from_name(part) {
                                    ::core::option::Option::Some(flag) => {
                                        flag.raw()
                                    }
                                    ::core::option::Option::None => {
                                        return ::core::result::Result::Err(
                                            E::custom("unknown flag name"),
                                        );
                                    }
                                }
                            };

                            bits |= value;
                        }
                    }

                    return if #validation {
                        ::core::result::Result::Ok(#vl(bits))
                    }
                    else {
                        ::core::result::Result::Err(
                            E::custom("invalid value"),
                        )
                    };
                }
            }

            impl ::serde::Serialize for #ty {
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    if #serialize_as_name {
                        return serializer.serialize_str(self.name());
                    }
                    return <#el as ::serde::Serialize>::serialize(
                        &self.raw(),
                        serializer,
                    );
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #ty {
                fn deserialize<D>(
                    deserializer: D,
                ) -> ::core::result::Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    if #deserialize_as_name {
                        return deserializer.deserialize_str(SerdeFlagVisitor);
                    }

                    let bits =
                        <#el as ::serde::Deserialize<'de>>::deserialize(
                            deserializer,
                        )?;
                    let items = Self::items();
                    let mut index = 0;
                    while index < items.len() {
                        let flag = items[index];
                        if flag.raw() == bits {
                            return ::core::result::Result::Ok(flag);
                        }
                        index += 1;
                    }
                    return ::core::result::Result::Err(
                        <D::Error as ::serde::de::Error>::custom(
                            "raw value is not a declared flag",
                        ),
                    );
                }
            }

            impl ::serde::Serialize for #vl {
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    if #serialize_as_name {
                        return serializer.collect_str(&SerdeFlagValue(*self));
                    }
                    return <#el as ::serde::Serialize>::serialize(
                        &self.bits(),
                        serializer,
                    );
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #vl {
                fn deserialize<D>(
                    deserializer: D,
                ) -> ::core::result::Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    if #deserialize_as_name {
                        return deserializer.deserialize_str(
                            SerdeFlagValueVisitor,
                        );
                    }

                    let bits =
                        <#el as ::serde::Deserialize<'de>>::deserialize(
                            deserializer,
                        )?;
                    return if #validation {
                        ::core::result::Result::Ok(Self(bits))
                    }
                    else {
                        ::core::result::Result::Err(
                            <D::Error as ::serde::de::Error>::custom(
                                "invalid value",
                            ),
                        )
                    };
                }
            }
        };
    }

    fn ekran_vl_impls(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let konst = zz_runner::konst(self.cfg.int.konst_for(&[
            Konsts::Not,
            Konsts::Bit,
        ]));

        return quote! {
            impl ::core::convert::From<#ty> for #vl {
                #[inline(always)]
                fn from(flag: #ty) -> Self {
                    return flag.into_value();
                }
            }

            #konst impl ::core::ops::Not for #vl {
                type Output = Self;

                #[inline(always)]
                fn not(self) -> Self::Output {
                    return self.complemented();
                }
            }
        };
    }

    // ---------------------------------

    fn ekran_ty_delegate(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let el = &self.el;
        let konst = zz_runner::konst(self.cfg.int.konst_for(&[
            Konsts::Value,
            Konsts::Bit,
        ]));

        return quote! {
            impl #ty {
                #[must_use]
                #[inline(always)]
                pub #konst fn from_bits_retain(bits: #el) -> #vl {
                    return #vl::from_bits_retain(bits);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn empty() -> #vl {
                    return #vl::empty();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all_named() -> #vl {
                    return #vl::all();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all_known() -> #vl {
                    return #vl::all();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all_unknown() -> #vl {
                    return #vl::from_bits_retain(!#vl::all().bits());
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn from_bits(bits: #el) -> Option<#vl> {
                    return #vl::from_bits(bits);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn from_bits_truncate(bits: #el) -> #vl {
                    return #vl::from_bits_truncate(bits);
                }

                #[must_use]
                #[inline(always)]
                pub fn iter_defined_names() -> impl Iterator<Item = (&'static str, #vl)> {
                    return #vl::iter_defined_names();
                }
            }
        };
    }

    fn ekran_ty_impl(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let el = &self.el;
        let all_bits = self.items.iter();
        let konst = zz_runner::konst(self.cfg.int.konst_for(&[
            Konsts::Value,
            Konsts::Bit,
        ]));
        let from_konst =
            zz_runner::konst(self.cfg.konst_for_nightly(&[Konsts::Conversion]));
        let iterator_konst =
            zz_runner::konst(self.cfg.konst_for_nightly(&[Konsts::Iterator]));

        let name_arms = self
            .items
            .iter()
            .map(|it| {
                let name = it.to_string();
                return quote! { #ty::#it => #name, };
            })
            .vec();

        let from_name_arms = self
            .items
            .iter()
            .map(|it| {
                let name = it.to_string();
                return quote! { #name => Some(#ty::#it), };
            })
            .vec();

        let item_items = self.items.iter().map(|it| quote! { #ty::#it }).vec();

        return quote! {
            impl #ty {
                #[inline(always)]
                #[must_use]
                pub #konst fn name(self) -> &'static str {
                    return match self {
                        #(#name_arms)*
                    };
                }

                #[inline(always)]
                #[must_use]
                pub #konst fn items() -> &'static [Self] {
                    const ITEMS: &'static [#ty] = &[#(#item_items),*];

                    return ITEMS;
                }

                #[inline]
                #[must_use]
                pub #from_konst fn from_name(name: &str) -> Option<Self> {
                    return match name {
                        #(#from_name_arms)*

                        _ => ::core::option::Option::None,
                    };
                }

                #[inline(always)]
                #[must_use]
                pub #konst fn raw(self) -> #el {
                    return self as #el;
                }

                #[inline(always)]
                #[must_use]
                pub #konst fn into_value(self) -> #vl {
                    return #vl::from_bits_retain(self.raw());
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all() -> #vl {
                    #[allow(clippy::unnecessary_cast)]
                    return #vl::from_bits_retain(0 as #el #(| (#ty::#all_bits as #el))*);
                }

                #[must_use]
                #[inline(always)]
                pub #iterator_konst fn iter() -> impl Iterator<Item = Self> {
                    return IterItems { index: 0 };
                }

                #[must_use]
                #[inline(always)]
                pub #iterator_konst fn iter_values() -> impl Iterator<Item = #vl> {
                    return #vl::all().iter();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn inserted(self, other: Self) -> #vl {
                    return self.into_value().inserted(other.into_value());
                }
            }
        };
    }

    fn ekran_ty_impls(&self) -> TokenStream {
        let ty = &self.ty;
        let value = &self.vl;
        let seal = format_ident!("FlagSeal");
        let bit_capability = format_ident!("FlagBit");
        let fn_conv = format_ident!(
            "conv_{}",
            zz_runner::snake_case_of(&self.ty.to_string())
        );
        let bounds_for = |flags: &[Konsts]| {
            let is_konst = self.cfg.int.konst_for(flags);
            let bonst = zz_runner::bonst(is_konst);
            let destruct = zz_runner::destruct(is_konst);
            return quote! { #bonst #seal #destruct };
        };

        let mut stream = TokenStream::new();

        if self.cfg.bit.display {
            stream.extend(quote! {
                impl ::core::fmt::Display for #ty {
                    #[inline(always)]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        return ::core::fmt::Display::fmt(self.name(), f);
                    }
                }
            });
        }

        if self.cfg.int.flags.math_shr {
            let konst = zz_runner::konst(self.cfg.int.konst_for(&[
                Konsts::Shr,
                Konsts::Math,
            ]));
            stream.extend(quote! {
                #konst impl ::core::ops::Shr<usize> for #ty {
                    type Output = #value;

                    #[inline(always)]
                    fn shr(self, rhs: usize) -> Self::Output {
                        return self.into_value() >> rhs;
                    }
                }
            });
        }

        if self.cfg.int.flags.math_shl {
            let konst = zz_runner::konst(self.cfg.int.konst_for(&[
                Konsts::Shl,
                Konsts::Math,
            ]));
            stream.extend(quote! {
                #konst impl ::core::ops::Shl<usize> for #ty {
                    type Output = #value;

                    #[inline(always)]
                    fn shl(self, rhs: usize) -> Self::Output {
                        return self.into_value() << rhs;
                    }
                }
            });
        }

        if self.cfg.int.flags.math_and {
            let flags = [
                Konsts::And,
                Konsts::Bit,
            ];
            let konst = zz_runner::konst(self.cfg.int.konst_for(&flags));
            let bounds = bounds_for(&flags);
            stream.extend(quote! {
                #konst impl<T> ::core::ops::BitAnd<T> for #ty
                where
                    T: #bit_capability + #bounds,
                {
                    type Output = #value;

                    #[inline(always)]
                    fn bitand(self, rhs: T) -> Self::Output {
                        let rhs = #seal::#fn_conv(rhs);
                        return self.into_value().intersection(
                            #value::from_bits_retain(rhs)
                        );
                    }
                }
            });
        }

        if self.cfg.int.flags.math_or {
            let flags = [
                Konsts::Or,
                Konsts::Bit,
            ];
            let konst = zz_runner::konst(self.cfg.int.konst_for(&flags));
            let bounds = bounds_for(&flags);
            stream.extend(quote! {
                #konst impl<T> ::core::ops::BitOr<T> for #ty
                where
                    T: #bit_capability + #bounds,
                {
                    type Output = #value;

                    #[inline(always)]
                    fn bitor(self, rhs: T) -> Self::Output {
                        let rhs = #seal::#fn_conv(rhs);
                        return self.into_value().union(
                            #value::from_bits_retain(rhs)
                        );
                    }
                }
            });
        }

        if self.cfg.int.flags.math_xor {
            let flags = [
                Konsts::Xor,
                Konsts::Bit,
            ];
            let konst = zz_runner::konst(self.cfg.int.konst_for(&flags));
            let bounds = bounds_for(&flags);
            stream.extend(quote! {
                #konst impl<T> ::core::ops::BitXor<T> for #ty
                where
                    T: #bit_capability + #bounds,
                {
                    type Output = #value;

                    #[inline(always)]
                    fn bitxor(self, rhs: T) -> Self::Output {
                        let rhs = #seal::#fn_conv(rhs);
                        return self.into_value().symmetric_difference(
                            #value::from_bits_retain(rhs)
                        );
                    }
                }
            });
        }

        if self.cfg.int.flags.partial_eq {
            let konst = zz_runner::konst(self.cfg.int.konst_for(&[
                Konsts::PartialEq,
                Konsts::Comparison,
            ]));
            stream.extend(quote! {
                #konst impl ::core::cmp::PartialEq<#value> for #ty {
                    #[inline(always)]
                    fn eq(&self, rhs: &#value) -> bool {
                        return self.raw() == rhs.raw();
                    }
                }
            });
        }

        if self.cfg.int.flags.partial_ord {
            let konst = zz_runner::konst(self.cfg.int.konst_for(&[
                Konsts::PartialOrd,
                Konsts::Comparison,
            ]));
            stream.extend(quote! {
                #konst impl ::core::cmp::PartialOrd<#value> for #ty {
                    #[inline(always)]
                    fn partial_cmp(
                        &self,
                        rhs: &#value,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        return self.raw().partial_cmp(&rhs.raw());
                    }
                }
            });
        }

        let konst = zz_runner::konst(self.cfg.int.konst_for(&[
            Konsts::Not,
            Konsts::Bit,
        ]));
        stream.extend(quote! {
            #konst impl ::core::ops::Not for #ty {
                type Output = #value;

                #[inline(always)]
                fn not(self) -> Self::Output {
                    return self.into_value().complemented();
                }
            }
        });

        return stream;
    }

    // ---------------------------------

    fn ekran_iter(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let konst =
            zz_runner::konst(self.cfg.int.konst_for(&[Konsts::Iterator]));
        let raw = self
            .cfg
            .int
            .get_raw
            .as_ref()
            .map(|it| quote! { #it })
            .unwrap_or_else(|| quote! { #vl::raw });

        return quote! {
            struct IterItems {
                index: usize,
            }

            #konst impl core::iter::Iterator for IterItems {
                type Item = #ty;

                fn next(&mut self) -> Option<Self::Item> {
                    let items = #ty::items();
                    let max = items.len();

                    while self.index < max {
                        let next = items[self.index];
                        self.index += 1;
                        return Some(next);
                    }

                    return None;
                }

                #[inline(always)]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    let max = #ty::items().len();

                    return (max, Some(max));
                }
            }

            struct IterFlags {
                value: #vl,
                index: usize,
            }

            #konst impl core::iter::Iterator for IterFlags {
                type Item = #ty;

                fn next(&mut self) -> Option<Self::Item> {
                    let items = #ty::items();
                    let max = items.len();

                    while self.index < max {
                        let next = items[self.index];
                        self.index += 1;
                        if self.value.contains_all(next.into_value()) {
                            self.value = self.value.without(next.into_value());
                            return Some(next);
                        }
                    }

                    return None;
                }

                #[inline]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    let bound = #raw(self.value).count_ones() as usize;
                    return (bound, Some(bound));
                }
            }

            pub struct IterValues {
                value: #vl,
                index: usize,
            }

            #konst impl Iterator for IterValues {
                type Item = #vl;

                fn next(&mut self) -> Option<Self::Item> {
                    let items = #ty::items();
                    let max = items.len();

                    while self.index < max {
                        let next = items[self.index].into_value();
                        self.index += 1;
                        if self.value.contains_all(next) {
                            self.value = self.value.without(next);
                            return Some(next);
                        }
                    }

                    if !self.value.is_empty() {
                        let it = self.value;
                        self.value = Self::Item::empty();
                        return Some(it);
                    }

                    return None;
                }

                #[inline]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    let bound = (#raw(self.value).count_ones() + 1) as usize;
                    return (bound, Some(bound));
                }
            }
        };
    }

    fn ekran_friendship(&self) -> syn::Result<TokenStream> {
        let relation_ident = &self.el;
        let relation: syn::Type = parse_quote! { #relation_ident };
        let friends = self
            .cfg
            .friends
            .iter()
            .map(|friend| {
                let capabilities = friend
                    .capabilities
                    .iter()
                    .map(|it| format_ident!("Flag{}", it))
                    .set();
                let relation = &self.el;
                let conversion = match &friend.conv {
                    Some(conv) if conv.is_ident("self") => quote! {
                        let relation: #relation = self;
                        return relation;
                    },
                    Some(conv) => quote! {
                        return #conv(self);
                    },
                    None => quote! {
                        let relation: #relation = self.into();
                        return relation;
                    },
                };

                let ty = &friend.ty;
                return Ok(Some(ProtocolFriend {
                    ty: syn::parse_quote! { #ty },
                    capabilities,
                    conversion: Some(conversion),
                }));
            })
            .collect::<syn::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .vec();
        let target = &self.ty;

        return Ok(crate::friendship::emit_protocol(Protocol {
            target: quote! { #target },
            generics: syn::Generics::default(),
            relation,
            seal: format_ident!("FlagSeal"),
            conversion: format_ident!(
                "conv_{}",
                zz_runner::snake_case_of(&self.ty.to_string())
            ),
            capabilities: vec![
                format_ident!("FlagMake"),
                format_ident!("FlagMath"),
                format_ident!("FlagBit"),
                format_ident!("FlagCmp"),
                format_ident!("FlagUnsafe"),
            ],
            friends,
            emit_seal: true,
            target_conversion: None,
            construction: None,
            konst: self.cfg.int.konst_friendship(),
        }));
    }
}

// =============================================================================

#[derive(Debug)]
pub(crate) struct Cfg {
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) bit: Box<BitFlags>,
    pub(crate) int: Box<integral::Cfg>,
    pub(crate) tree: Option<marker_tree::Cfg>,
    repr: Option<N>,
    konst: Konst,
    konst_flags: BTreeMap<Konsts, bool>,
    seen_flags: BTreeMap<String, bool>,
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self {
            friends: Default::default(),
            bit: Box::default(),
            int: Box::new(integral::Cfg::without_konst()),
            tree: None,
            repr: None,
            konst: Konst::default(),
            konst_flags: Default::default(),
            seen_flags: Default::default(),
        };

        zz::parse_inner(input, |attr, rest| this.parse_attr(attr, rest))?;

        let serde_modes = [
            this.bit.serde_with_auto,
            this.bit.serde_with_name,
            this.bit.serde_with_value,
        ]
        .into_iter()
        .filter(|enabled| *enabled)
        .count();
        if serde_modes > 1 {
            return Err(syn::Error::new(
                input.span(),
                "bitflag serde representation flags are mutually exclusive",
            ));
        }
        if serde_modes == 1 {
            this.bit.serde = true;
        }

        this.apply_konst()?;

        return Ok(this);
    }
}

impl Cfg {
    fn parse_flags(
        &mut self,
        input: ParseStream,
        value_if_seen: bool,
    ) -> syn::Result<()> {
        let attrs = zz::one_or_list::<Ident>(input)?;

        for attr in attrs {
            let name = attr.to_string();
            let span = attr.span();

            if self
                .seen_flags
                .get(&name)
                .is_some_and(|value| *value != value_if_seen)
            {
                return span
                    .fail("flag specified in both `with` and `without`");
            }
            if self.seen_flags.contains_key(&name) {
                return span.fail("duplicated flags");
            }

            if name.starts_with("konst_") {
                let Some(flag) = Konsts::from_str(&name).ok()
                else {
                    return span.fail("unknown flag");
                };
                self.konst_flags.insert(flag, value_if_seen);
            }
            else if self.bit.set_bool(&name, value_if_seen).is_err() {
                return span.fail("unknown flag");
            }

            self.seen_flags.insert(name, value_if_seen);
        }

        return Ok(());
    }

    fn konst_for(
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

    fn konst_for_nightly(
        &self,
        flags: &[Konsts],
    ) -> bool {
        for flag in flags {
            if let Some(value) = self.konst_flags.get(flag) {
                return *value;
            }
        }

        return matches!(self.konst, Konst::Nightly);
    }

    fn apply_konst(&mut self) -> syn::Result<()> {
        for &flag in Konsts::INTEGRAL {
            let required = self.konst_for(&[flag]);

            if let Some((value, span)) = self.int.explicit_konst_flag(flag)
                && value != required
            {
                return Err(syn::Error::new(
                    span,
                    "this value is not compatible with own konst config of bitflag",
                ));
            }

            self.int.set_konst_flag(flag, required);
        }

        self.int.konst = self.konst;
        return Ok(());
    }

    fn parse_attr(
        &mut self,
        attr: &str,
        rest: ParseStream,
    ) -> syn::Result<bool> {
        match attr {
            "konst" => {
                self.konst = rest.parse()?;
            }
            "repr" => {
                let repr: Ident = rest.parse()?;
                self.repr = N::of(repr.to_string());
                if self.repr.is_none() {
                    return repr.fail("repr must be a Rust numeric type");
                }
            }
            "with" => self.parse_flags(rest, true)?,
            "without" => self.parse_flags(rest, false)?,
            "friend" => {
                let friends = zz::one_or_list(rest)?.set();
                self.friends = friends;
            }
            "tree" => {
                self.tree = Some(marker_tree::Cfg::parse_attribute(rest)?);
            }
            "suffix" => {
                let as_str: syn::LitStr = rest.parse()?;
                if as_str.value().is_empty() {
                    self.bit.suffix = "".to_string();
                }
                else {
                    let as_idn: Ident = syn::parse_str(&as_str.value())?;
                    self.bit.suffix = as_idn.to_string();
                }
            }
            "integral" => {
                let content;
                let _ = bracketed!(content in rest);

                let mut int =
                    integral::Cfg::parse_with_konst_unresolved(&content)?;
                if int.facets_present() {
                    return content.span().fail(
                        "bitflag integral configuration does not support facets",
                    );
                }
                if !int.ty_out.is_self() {
                    return int.ty_out.conversion.fail(
                        "bitflag integral configuration does not support produce",
                    );
                }
                if !int.profile.is_default() {
                    return content.span().fail(
                        "bitflag integral configuration only supports the default profile",
                    );
                }
                int.resolve()?;
                if int.non_zero == integral::NonZeroPolicy::Require {
                    return content.span().fail(
                        "bitflag integral configuration does not support `non_zero`",
                    );
                }
                if int.validation.domain_excludes_zero() {
                    return content
                        .span()
                        .fail("bitflag integral domain must include zero");
                }
                if int.requires_non_zero_storage() {
                    return content.span().fail(
                        "bitflag integral configuration cannot use nonzero storage",
                    );
                }
                *self.int = int;

                return Ok(true);
            }
            "value_name" => {
                let as_str: syn::LitStr = rest.parse()?;
                let as_idn: Ident = syn::parse_str(&as_str.value())?;
                self.bit.value_name = as_idn.to_string();
            }
            _ => return Ok(false),
        };

        return Ok(true);
    }
}

// =============================================================================

mk_flags! {
    #[flag_default(bool=true, str="")]
    #[derive(Debug, Clone)]
    pub(crate) struct BitFlags {
        pub clone: bool,
        pub copy: bool,
        pub hash: bool,
        pub debug: bool,
        pub make_value: bool,
        pub impl_value: bool,
        pub display: bool = false,
        pub repr: bool,
        pub serde: bool = false,
        pub serde_with_auto: bool = false,
        pub serde_with_name: bool = false,
        pub serde_with_value: bool = false,

        pub value_name: String,
        pub suffix: String = "Value",
    }
}
