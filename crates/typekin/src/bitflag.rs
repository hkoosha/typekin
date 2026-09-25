use crate::{
    friendship::{
        Protocol,
        ProtocolFriend,
        cfg::Friend,
    },
    integral::{
        self,
        IntegralCfg,
    },
    runner::{
        self,
        MkErr,
        mk_flags,
    },
    value_type::N,
};
use std::collections::BTreeSet;

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::{
    Fields,
    bracketed,
    parse::{
        Parse,
        ParseStream,
    },
    parse_quote,
    spanned::Spanned,
};

pub(crate) fn bitflag(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let cfg = Box::new(syn::parse_macro_input!(attr as BitflagCfg));
    let item = syn::parse_macro_input!(item as syn::ItemEnum);

    return runner::catching(move || {
        let repr = runner::find_repr_n(&item.attrs, item.span())?;
        let ty = item.ident.clone();
        let items = item.variants.iter().map(|it| it.ident.clone()).collect();

        if let Some(bad) = item
            .variants
            .iter()
            .find(|it| !matches!(it.fields, Fields::Unit))
        {
            return bad.fail("only unit variants are supported");
        }

        let stream = Maker::new(ty, repr, cfg, items).ekran()?;
        return Ok(quote! { #item #stream });
    });
}

mk_flags! {
    #[flag_default(bool=true, str="")]
    #[derive(Debug, Clone)]
    pub(crate) struct BitFlags {
        pub make_value: bool,
        pub impl_value: bool,

        pub value_name: String,
        pub suffix: String = "Value",
    }
}

#[derive(Default)]
pub(crate) struct BitflagCfg {
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) bit: Box<BitFlags>,
    pub(crate) int: Box<IntegralCfg>,
}

impl Parse for BitflagCfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();
        let mut is_konst = false;
        let mut has_konst = false;

        runner::parse_inner_attributes_with_extra(
            input,
            |attr, rest| {
                match attr {
                    "konst" => {
                        has_konst = true;
                        is_konst = rest.parse::<syn::LitBool>()?.value
                    }
                    "with" => this.bit.parse_from(rest, true)?,
                    "without" => this.bit.parse_from(rest, false)?,
                    "friends" => this.friends = runner::list(rest)?.collect(),
                    "suffix" => {
                        let as_str: syn::LitStr = input.parse()?;
                        if as_str.value().is_empty() {
                            this.bit.suffix = "".to_string();
                        }
                        else {
                            let as_idn: Ident =
                                syn::parse_str(&as_str.value())?;
                            this.bit.suffix = as_idn.to_string();
                        }
                    }
                    "value_name" => {
                        let as_str: syn::LitStr = input.parse()?;
                        let as_idn: Ident = syn::parse_str(&as_str.value())?;
                        this.bit.value_name = as_idn.to_string();
                    }
                    _ => {}
                };

                return Ok(true);
            },
            // Turns out, this wasn't even need.
            // TODO cleanup the mess.
            Some("integral"),
            |_, rest| {
                let content;
                let _ = bracketed!(content in rest);
                let cfg = IntegralCfg::parse_with_konst(&content, false)?;
                this.int = Box::new(cfg);
                return Ok(true);
            },
        )?;

        if !has_konst {
            return Err(syn::Error::new(
                input.span(),
                "missing required `konst` argument",
            ));
        }
        this.int.konst = is_konst;

        return Ok(this);
    }
}

pub(crate) struct Maker {
    el: Ident,
    ty: Ident,
    vl: Ident,
    cfg: Box<BitflagCfg>,
    items: Vec<Ident>,
}

impl Maker {
    pub(crate) fn new(
        ty: Ident,
        repr: N,
        cfg: Box<BitflagCfg>,
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
            ty: ty.clone(),
            cfg,
            items,
        };

        this.cfg.int.flags.impl_math_not = false;
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
            [format_ident!("Bit"), format_ident!("Make")],
            parse_quote! { #ty::raw },
        );
    }

    pub(crate) fn ekran(self) -> syn::Result<TokenStream> {
        let vl = self.ekran_vl();
        let vl_impl = self.ekran_vl_impl();
        let vl_impls = self.ekran_vl_impls();
        let ty_delegate = self.ekran_ty_delegate();
        let ty_impl = self.ekran_ty_impl();
        let ty_impls = self.ekran_ty_impls();
        let iter_impl = self.ekran_iter();
        let friends = self.ekran_friendship()?;

        let impl_int = match self.cfg.bit.impl_value {
            false => TokenStream::new(),
            true => {
                integral::Maker::new(self.vl, self.el, self.cfg.int)?.ekran()?
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
            };
        };

        return Ok(it);
    }

    // ---------------------------------

    fn ekran_vl(&self) -> TokenStream {
        let ty_value = &self.vl;
        let el = &self.el;
        let derive_clone = if self.cfg.int.konst {
            quote! { #[derive_const(Clone)] }
        }
        else {
            quote! { #[derive(Clone)] }
        };

        return match self.cfg.bit.make_value {
            true => quote! {
                #[derive(Copy)]
                #derive_clone
                #[repr(transparent)]
                pub struct #ty_value(#el);
            },
            false => TokenStream::new(),
        };
    }

    fn ekran_vl_impl(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let el = &self.el;
        let konst = runner::konst(self.cfg.int.konst);

        let raw = self
            .cfg
            .int
            .get_raw
            .as_ref()
            .map(|it| quote! { #it })
            .unwrap_or_else(|| quote! { #vl::raw });

        return quote! {
            impl #vl {
                #[must_use]
                #[inline(always)]
                pub #konst fn bits(self) -> #el {
                    return #raw(self);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn from_bits_retain(bits: #el) -> Self {
                    return Self::of(bits);
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
                    return self.unknown_bits() != Self::empty().into();
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
                pub #konst fn from_name(name: &str) -> Option<Self> {
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
                        if flag.into_value() == self {
                            return Ok(flag);
                        }
                        i += 1;
                    }

                    return Err(self);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn iter_known_flags(self) -> impl Iterator<Item = #ty> {
                    return IterFlags {
                        value: self,
                        index: 0,
                    };
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn iter(self) -> impl Iterator<Item = Self> {
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
                    return self == Self::all();
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

    fn ekran_vl_impls(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let konst = runner::konst(self.cfg.int.konst);

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
        let konst = runner::konst(self.cfg.int.konst);

        return quote! {
            impl #ty {
                #[must_use]
                #[inline(always)]
                pub #konst fn from_bits_retain(bits: #el) -> #vl {
                    return #vl::of(bits);
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
        let konst = runner::konst(self.cfg.int.konst);

        let name_arms = self
            .items
            .iter()
            .map(|it| {
                let name = it.to_string();
                return quote! { #ty::#it => #name, };
            })
            .collect::<Vec<_>>();

        let from_name_arms = self
            .items
            .iter()
            .map(|it| {
                let name = it.to_string();
                return quote! { #name => Some(#ty::#it), };
            })
            .collect::<Vec<_>>();

        let item_items = self
            .items
            .iter()
            .map(|it| quote! { #ty::#it })
            .collect::<Vec<_>>();

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
                pub #konst fn from_name(name: &str) -> Option<Self> {
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
                pub #konst fn iter() -> impl Iterator<Item = Self> {
                    return IterItems { index: 0 };
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn iter_values() -> impl Iterator<Item = #vl> {
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
            runner::snake_case_of(&self.ty.to_string())
        );
        let destruct = runner::destruct(self.cfg.int.konst);
        let konst = runner::konst(self.cfg.int.konst);

        let cond_seal = {
            let bonst = runner::konst(self.cfg.int.konst);
            quote! { #bonst #seal }
        };

        let mut stream = TokenStream::new();

        if self.cfg.int.flags.impl_math_shr {
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

        if self.cfg.int.flags.impl_math_shl {
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

        if self.cfg.int.flags.impl_math_and {
            stream.extend(quote! {
                #konst impl<T> ::core::ops::BitAnd<T> for #ty
                where
                    T: #bit_capability + #cond_seal #destruct,
                {
                    type Output = #value;

                    #[inline(always)]
                    fn bitand(self, rhs: T) -> Self::Output {
                        let rhs = #seal::#fn_conv(&rhs);
                        return self.into_value().intersection(
                            #value::from_bits_retain(rhs)
                        );
                    }
                }
            });
        }

        if self.cfg.int.flags.impl_math_or {
            stream.extend(quote! {
                #konst impl<T> ::core::ops::BitOr<T> for #ty
                where
                    T: #bit_capability + #cond_seal #destruct,
                {
                    type Output = #value;

                    #[inline(always)]
                    fn bitor(self, rhs: T) -> Self::Output {
                        let rhs = #seal::#fn_conv(&rhs);
                        return self.into_value().union(
                            #value::from_bits_retain(rhs)
                        );
                    }
                }
            });
        }

        if self.cfg.int.flags.impl_math_xor {
            stream.extend(quote! {
                #konst impl<T> ::core::ops::BitXor<T> for #ty
                where
                    T: #bit_capability + #cond_seal #destruct,
                {
                    type Output = #value;

                    #[inline(always)]
                    fn bitxor(self, rhs: T) -> Self::Output {
                        let rhs = #seal::#fn_conv(&rhs);
                        return self.into_value().symmetric_difference(
                            #value::from_bits_retain(rhs)
                        );
                    }
                }
            });
        }

        if self.cfg.int.flags.impl_partial_eq {
            stream.extend(quote! {
                #konst impl ::core::cmp::PartialEq<#value> for #ty {
                    #[inline(always)]
                    fn eq(&self, rhs: &#value) -> bool {
                        return self.raw() == rhs.raw();
                    }
                }
            });
        }

        if self.cfg.int.flags.impl_partial_ord {
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
        let konst = runner::konst(self.cfg.int.konst);
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
                let Some(ty) = friend.ty.clone()
                else {
                    return Ok(None);
                };
                let capabilities = friend
                    .capabilities
                    .iter()
                    .map(|it| format_ident!("Flag{}", it))
                    .collect::<BTreeSet<_>>();
                let relation = &self.el;
                let conversion = match &friend.conv {
                    Some(conv) if conv.is_ident("self") => quote! {
                        let relation: #relation = *self;
                        return relation;
                    },
                    Some(conv) => quote! {
                        return #conv(*self);
                    },
                    None => quote! {
                        let relation: #relation = (*self).into();
                        return relation;
                    },
                };

                return Ok(Some(ProtocolFriend {
                    ty,
                    capabilities,
                    conversion: Some(conversion),
                }));
            })
            .collect::<syn::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();

        return Ok(crate::friendship::emit_protocol(Protocol {
            target: self.ty.clone(),
            relation,
            seal: format_ident!("FlagSeal"),
            conversion: format_ident!(
                "conv_{}",
                runner::snake_case_of(&self.ty.to_string())
            ),
            capabilities: vec![
                format_ident!("FlagMake"),
                format_ident!("FlagMath"),
                format_ident!("FlagBit"),
                format_ident!("FlagCmp"),
            ],
            friends,
            emit_seal: true,
            target_conversion: None,
            konst: self.cfg.int.konst,
        }));
    }
}
