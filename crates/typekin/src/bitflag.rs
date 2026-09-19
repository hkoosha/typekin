use crate::{
    integral::{
        self,
        IntegralCfg,
    },
    runner::{
        self,
        MkErr,
        mk_flags,
    },
    type_friendship::{
        FriendReq,
        FriendshipLevel,
    },
    value_type::N,
};

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
    Path,
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

    return runner::ekran_catching(move || {
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
        pub value_name_suffix: String = "Value",

        pub trait_seal: String = "BitSeal",
        pub trait_friend_make: String = "BitFriendMake",
        pub trait_friend_math: String = "BitFriendMath",
        pub trait_friend_bit: String = "BitFriendBit",
        pub trait_friend_rel: String = "BitFriendRel",
    }
}

#[derive(Default)]
pub(crate) struct BitflagCfg {
    pub(crate) friends: Vec<FriendReq>,
    pub(crate) bit: Box<BitFlags>,
    pub(crate) int: Box<IntegralCfg>,
}

impl Parse for BitflagCfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();
        let mut has_integral = false;

        runner::parse_inner_attributes(input, |attr, rest| {
            match attr {
                "with" => this.bit.parse_from(rest, true)?,
                "without" => this.bit.parse_from(rest, false)?,
                "friends" => this.friends = runner::list(rest)?.collect(),
                "value_name_suffix" => {
                    let as_str: syn::LitStr = input.parse()?;
                    if as_str.value().is_empty() {
                        this.bit.value_name_suffix = "".to_string();
                    }
                    else {
                        let as_idn: Ident = syn::parse_str(&as_str.value())?;
                        this.bit.value_name_suffix = as_idn.to_string();
                    }
                }
                "value_name" => {
                    let as_str: syn::LitStr = input.parse()?;
                    let as_idn: Ident = syn::parse_str(&as_str.value())?;
                    this.bit.value_name = as_idn.to_string();
                }
                "integral" => {
                    let content;
                    let _ = bracketed!(content in input);
                    let cfg = IntegralCfg::parse(&content)?;
                    this.int = Box::new(cfg);
                    has_integral = true;
                }
                _ => {}
            };

            return Ok(true);
        })?;

        if !has_integral {
            return Err(syn::Error::new(
                input.span(),
                "missing required `konst` argument in `integral = [...]`",
            ));
        }

        return Ok(this);
    }
}

pub(crate) struct Maker {
    el: Ident,
    ty: Ident,
    vl: Ident,

    trait_seal: Ident,
    trait_friend_make: Ident,
    trait_friend_math: Ident,
    trait_friend_bit: Ident,
    trait_friend_rel: Ident,

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
            vl: match (
                cfg.bit.value_name.as_str(),
                cfg.bit.value_name_suffix.as_str(),
            ) {
                ("", "") => format_ident!("{}Value", ty),
                ("", suffix) => format_ident!("{}{}", ty, suffix),
                (prefix, "") => format_ident!("{}", prefix),
                (prefix, suffix) => format_ident!("{}{}", prefix, suffix),
            },

            trait_seal: format_ident!("{}", cfg.bit.trait_seal),
            trait_friend_make: format_ident!("{}", cfg.bit.trait_friend_make),
            trait_friend_math: format_ident!("{}", cfg.bit.trait_friend_math),
            trait_friend_bit: format_ident!("{}", cfg.bit.trait_friend_bit),
            trait_friend_rel: format_ident!("{}", cfg.bit.trait_friend_rel),

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

        self.cfg.friends.sort();

        for (f_ty, f_conv) in [
            (ty, parse_quote! { #ty::raw }),
            (vl, parse_quote! { #vl::raw }),
        ] {
            if let Some(friendship) =
                self.cfg.friends.iter_mut().find(|it| it.ty.is_ident(f_ty))
            {
                friendship.level.insert(FriendshipLevel::Rel);
                friendship.level.insert(FriendshipLevel::Bit);
                friendship.conv = Some(f_conv);
            }
            else {
                self.cfg.friends.push(FriendReq::new(
                    Path::from(f_ty.clone()),
                    [FriendshipLevel::Rel, FriendshipLevel::Bit]
                        .into_iter()
                        .collect(),
                    Some(f_conv),
                ));
            }
        }

        if self.cfg.bit.impl_value {
            if !self.cfg.int.friends.iter().any(|it| {
                it.level.contains(&FriendshipLevel::Make)
                    && it.ty.is_ident(&self.el)
            }) {
                if let Some(friendship) = self
                    .cfg
                    .int
                    .friends
                    .iter_mut()
                    .find(|it| it.ty.is_ident(&self.el))
                {
                    friendship.level.insert(FriendshipLevel::Make);
                }
                else {
                    self.cfg.int.friends.push(FriendReq::new(
                        Path::from(self.el.clone()),
                        FriendshipLevel::Make.normalize(),
                        None,
                    ))
                }
            }

            if let Some(friendship) = self
                .cfg
                .int
                .friends
                .iter_mut()
                .find(|it| it.ty.is_ident(ty))
            {
                friendship.level.insert(FriendshipLevel::Rel);
                friendship.level.insert(FriendshipLevel::Bit);
                friendship.conv = Some(parse_quote! { #ty::raw });
            }
            else {
                self.cfg.int.friends.push(FriendReq::new(
                    Path::from(ty.clone()),
                    [FriendshipLevel::Rel, FriendshipLevel::Bit]
                        .into_iter()
                        .collect(),
                    Some(parse_quote! { #ty::raw }),
                ));
            }
        }
    }

    pub(crate) fn ekran(self) -> syn::Result<TokenStream> {
        let vl = self.ekran_vl();
        let vl_impl = self.ekran_vl_impl();
        let vl_impls = self.ekran_vl_impls();
        let ty_delegate = self.ekran_ty_delegate();
        let ty_impl = self.ekran_ty_impl();
        let ty_impls = self.ekran_ty_impls();
        let iter_impl = self.ekran_iter();
        let friends = self.ekran_friendship();

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
        let (konst, _, _) =
            runner::konst_bonst_and_destruct(self.cfg.int.konst);

        return quote! {
            impl #vl {
                #[must_use]
                #[inline(always)]
                pub #konst fn bits(self) -> #el {
                    return self.raw();
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
        let (konst, _, _) =
            runner::konst_bonst_and_destruct(self.cfg.int.konst);

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
        let (konst, _, _) =
            runner::konst_bonst_and_destruct(self.cfg.int.konst);

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
        let (konst, _, _) =
            runner::konst_bonst_and_destruct(self.cfg.int.konst);

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
        let trait_seal = &self.trait_seal;
        let trait_friend_bit = &self.trait_friend_bit;
        let fn_conv = format_ident!(
            "conv_{}",
            runner::snake_case_of(&self.ty.to_string())
        );
        let (konst, bonst, destruct) =
            runner::konst_bonst_and_destruct(self.cfg.int.konst);

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
                    T: #bonst #trait_friend_bit #destruct,
                {
                    type Output = #value;

                    #[inline(always)]
                    fn bitand(self, rhs: T) -> Self::Output {
                        let rhs = #trait_seal::#fn_conv(&rhs);
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
                    T: #bonst #trait_friend_bit #destruct,
                {
                    type Output = #value;

                    #[inline(always)]
                    fn bitor(self, rhs: T) -> Self::Output {
                        let rhs = #trait_seal::#fn_conv(&rhs);
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
                    T: #bonst #trait_friend_bit #destruct,
                {
                    type Output = #value;

                    #[inline(always)]
                    fn bitxor(self, rhs: T) -> Self::Output {
                        let rhs = #trait_seal::#fn_conv(&rhs);
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
        let (konst, _, _) =
            runner::konst_bonst_and_destruct(self.cfg.int.konst);

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
                    let bound = self.value.raw().count_ones() as usize;
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
                    let bound = (self.value.raw().count_ones() + 1) as usize;
                    return (bound, Some(bound));
                }
            }
        };
    }

    fn ekran_friendship(&self) -> TokenStream {
        let el = &self.el;
        let trait_seal = &self.trait_seal;
        let trait_friend_make = &self.trait_friend_make;
        let trait_friend_math = &self.trait_friend_math;
        let trait_friend_bit = &self.trait_friend_bit;
        let trait_friend_rel = &self.trait_friend_rel;
        let fn_conv = format_ident!(
            "conv_{}",
            runner::snake_case_of(&self.ty.to_string())
        );
        let (konst, bonst, _) =
            runner::konst_bonst_and_destruct(self.cfg.int.konst);

        let seal_impls = self
            .cfg
            .friends
            .iter()
            .map(|friendship| {
                let ty = &friendship.ty;
                let body = match &friendship.conv {
                    Some(conv) if conv.is_ident("self") => quote! {
                        let value: #el = *self;
                        return value;
                    },
                    Some(conv) => quote! {
                        return #conv(*self);
                    },
                    None => quote! {
                        let value: #el = (*self).into();
                        return value;
                    },
                };

                return quote! {
                    #konst impl #trait_seal for #ty {
                        #[inline(always)]
                        fn #fn_conv(&self) -> #el {
                            #body
                        }
                    }
                };
            })
            .collect::<Vec<_>>();

        let mut marker_impls = Vec::<TokenStream>::new();
        for friendship in &self.cfg.friends {
            for level in &friendship.level {
                for level in level.normalize() {
                    let friend = match level {
                        FriendshipLevel::Make => trait_friend_make,
                        FriendshipLevel::Math => trait_friend_math,
                        FriendshipLevel::Bit => trait_friend_bit,
                        FriendshipLevel::Rel => trait_friend_rel,
                        FriendshipLevel::Full | FriendshipLevel::None => {
                            unreachable!()
                        }
                        FriendshipLevel::XCustom(custom) => {
                            unimplemented!(
                                "custom flag friendship level not implemented: {}",
                                custom,
                            )
                        }
                    };
                    let ty = &friendship.ty;

                    marker_impls.push(quote! {
                        #konst impl #friend for #ty {}
                    });
                }
            }
        }

        return quote! {
            #(#seal_impls)*
            #(#marker_impls)*

            #konst trait #trait_seal {
                fn #fn_conv(&self) -> #el;
            }

            #konst trait #trait_friend_make: #bonst #trait_seal {}
            #konst trait #trait_friend_math: #bonst #trait_seal {}
            #konst trait #trait_friend_bit: #bonst #trait_seal {}
            #konst trait #trait_friend_rel: #bonst #trait_seal {}

            #konst impl<T> #trait_seal for &T
            where
                T: #bonst #trait_seal,
            {
                #[inline(always)]
                fn #fn_conv(&self) -> #el {
                    return #trait_seal::#fn_conv(&**self);
                }
            }

            #konst impl<T> #trait_seal for &mut T
            where
                T: #bonst #trait_seal,
            {
                #[inline(always)]
                fn #fn_conv(&self) -> #el {
                    return #trait_seal::#fn_conv(&**self);
                }
            }

            #konst impl<T> #trait_friend_make for &T
            where
                T: #bonst #trait_friend_make,
            {}

            #konst impl<T> #trait_friend_make for &mut T
            where
                T: #bonst #trait_friend_make,
            {}

            #konst impl<T> #trait_friend_math for &T
            where
                T: #bonst #trait_friend_math,
            {}

            #konst impl<T> #trait_friend_math for &mut T
            where
                T: #bonst #trait_friend_math,
            {}

            #konst impl<T> #trait_friend_bit for &T
            where
                T: #bonst #trait_friend_bit,
            {}

            #konst impl<T> #trait_friend_bit for &mut T
            where
                T: #bonst #trait_friend_bit,
            {}

            #konst impl<T> #trait_friend_rel for &T
            where
                T: #bonst #trait_friend_rel,
            {}

            #konst impl<T> #trait_friend_rel for &mut T
            where
                T: #bonst #trait_friend_rel,
            {}

        };
    }
}
