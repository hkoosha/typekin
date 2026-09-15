use crate::flag::cfg::BitflagCfg;
use crate::type_friendship::FriendReq;
use crate::type_friendship::FriendshipLevel;
use crate::value_type::N;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Path;
use syn::parse_quote;

pub(super) struct Maker {
    el: Ident,
    ty: Ident,

    trait_seal: Ident,
    trait_friend_make: Ident,
    trait_friend_math: Ident,
    trait_friend_bit: Ident,
    trait_friend_rel: Ident,

    cfg: Box<BitflagCfg>,
    items: Vec<Ident>,
    ty_value: Ident,
}

impl Maker {
    pub(super) fn new(
        ty: Ident,
        repr: N,
        cfg: Box<BitflagCfg>,
        items: Vec<Ident>,
    ) -> Self {
        let it = Self {
            ty_value: match (
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
            ty,
            cfg,
            items,
        };

        return it;
    }

    fn add_default_flag_friendships(&mut self) {
        let ty = self.ty.clone();
        self.add_flag_friend(&ty, parse_quote! { #ty::raw });

        let value = self.ty_value.clone();
        self.add_flag_friend(&value, parse_quote! { #value::raw });
    }

    fn add_flag_friend(
        &mut self,
        ty: &Ident,
        conv: Path,
    ) {
        if let Some(friendship) =
            self.cfg.friends.iter_mut().find(|it| it.ty.is_ident(ty))
        {
            friendship.level.insert(FriendshipLevel::Rel);
            friendship.level.insert(FriendshipLevel::Bit);
            friendship.conv = Some(conv);
        }
        else {
            self.cfg.friends.push(FriendReq {
                ty: Path::from(ty.clone()),
                level: [FriendshipLevel::Rel, FriendshipLevel::Bit]
                    .into_iter()
                    .collect(),
                conv: Some(conv),
            });
        }
    }

    pub(super) fn ekran(mut self) -> syn::Result<TokenStream> {
        self.add_default_flag_friendships();

        let value = self.ekran_value_t();
        let generic = self.ekran_items();
        let flag_friendships = self.ekran_flag_friendships();
        let flag = self.ekran_impls();
        let enum_value_impls = self.ekran_enum_value_impls();

        let mut impl_int = None;
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
                    self.cfg.int.friends.push(FriendReq {
                        ty: Path::from(self.el.clone()),
                        level: FriendshipLevel::Make.normalize(),
                        conv: None,
                    })
                }
            }
            let ty = &self.ty;
            let friendship = self
                .cfg
                .int
                .friends
                .iter_mut()
                .find(|it| it.ty.is_ident(ty));

            if let Some(friendship) = friendship {
                friendship.level.insert(FriendshipLevel::Rel);
                friendship.level.insert(FriendshipLevel::Bit);
                friendship.conv = Some(parse_quote! { #ty::raw });
            }
            else {
                self.cfg.int.friends.push(FriendReq {
                    ty: Path::from(ty.clone()),
                    level: [FriendshipLevel::Rel, FriendshipLevel::Bit]
                        .into_iter()
                        .collect(),
                    conv: Some(parse_quote! { #ty::raw }),
                });
            }

            self.cfg.int.flags.impl_math_not = false;

            let it = crate::integral::maker::Maker::new(
                self.ty_value,
                self.el,
                self.cfg.int,
            )?
            .ekran()?;

            impl_int = Some(quote::quote! {
                #[allow(dead_code)]
                #[allow(unused_qualifications)]
                const _: () = {
                    #it
                };
            });
        };

        let it = quote::quote! {
            #value

            #impl_int

            #[allow(dead_code)]
            #[allow(unused_qualifications)]
            const _: () = {
                #generic

                #enum_value_impls
                #flag_friendships
                #flag
            };
        };

        return Ok(it);
    }

    fn ekran_items(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.ty_value;
        let el = &self.el;

        let all_bits = self.items.iter();
        let konst = crate::runner::konst(self.cfg.int.konst);
        return quote! {
            type Flag = #ty;
            type Value = #vl;

            struct IterItems {
                index: usize,
            }

            #konst impl core::iter::Iterator for IterItems {
                type Item = Flag;

                fn next(&mut self) -> Option<Self::Item> {
                    let items = Flag::items();
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
                    let max = Flag::items().len();

                    return (max, Some(max));
                }
            }

            struct IterFlags {
                value: Value,
                index: usize,
            }

            #konst impl core::iter::Iterator for IterFlags {
                type Item = Flag;

                fn next(&mut self) -> Option<Self::Item> {
                    let items = Flag::items();
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
                value: Value,
                index: usize,
            }

            #konst impl Iterator for IterValues {
                type Item = Value;

                fn next(&mut self) -> Option<Self::Item> {
                    let items = Flag::items();
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

            impl ::core::convert::From<Flag> for Value {
                #[inline(always)]
                fn from(flag: Flag) -> Self {
                    return flag.into_value();
                }
            }

            #konst impl ::core::ops::Not for Value {
                type Output = Self;

                #[inline(always)]
                fn not(self) -> Self::Output {
                    return self.complemented();
                }
            }

            impl Flag {
                #[inline(always)]
                #[must_use]
                pub #konst fn raw(self) -> #el {
                    return self as #el;
                }

                #[inline(always)]
                #[must_use]
                pub #konst fn into_value(self) -> Value {
                    return Value::from_bits_retain(self.raw());
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all() -> Value {
                    #[allow(clippy::unnecessary_cast)]
                    return Value::from_bits_retain(0 as #el #(| (#ty::#all_bits as #el))*);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn iter() -> impl Iterator<Item = Self> {
                    return IterItems { index: 0 };
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn iter_values() -> impl Iterator<Item = Value> {
                    return Value::all().iter();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn inserted(self, other: Self) -> Value {
                    return self.into_value().inserted(other.into_value());
                }
            }

            impl Value {
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
                    return Flag::all();
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
                    return self.unknown_bits() != 0;
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
                    return match Flag::from_name(name) {
                        Some(flag) => Some(flag.into_value()),
                        None => None,
                    };
                }

                #[inline(always)]
                pub #konst fn into_flag(self) -> Result<Flag, Self> {
                    let items = Flag::items();
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
                pub #konst fn iter_known_flags(self) -> impl Iterator<Item = Flag> {
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
                    return Flag::iter().map(|flag| (flag.name(), flag.into_value()));
                }

                #[must_use]
                #[inline(always)]
                pub fn iter_equal_names(self) -> impl Iterator<Item = &'static str> {
                    return Flag::iter()
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

    fn ekran_flag_friendships(&self) -> TokenStream {
        let el = &self.el;
        let trait_seal = &self.trait_seal;
        let trait_friend_make = &self.trait_friend_make;
        let trait_friend_math = &self.trait_friend_math;
        let trait_friend_bit = &self.trait_friend_bit;
        let trait_friend_rel = &self.trait_friend_rel;
        let fn_conv = format_ident!(
            "conv_{}",
            crate::runner::snake_case_of(&self.ty.to_string())
        );
        let (konst, bonst, _) =
            crate::runner::konst_bonst_and_destruct(self.cfg.int.konst);

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

    fn ekran_enum_value_impls(&self) -> TokenStream {
        let ty = &self.ty;
        let value = &self.ty_value;
        let konst = crate::runner::konst(self.cfg.int.konst);
        let trait_seal = &self.trait_seal;
        let trait_friend_bit = &self.trait_friend_bit;
        let fn_conv = format_ident!(
            "conv_{}",
            crate::runner::snake_case_of(&self.ty.to_string())
        );
        let (_, bonst, destruct) =
            crate::runner::konst_bonst_and_destruct(self.cfg.int.konst);
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

    fn ekran_impls(&self) -> TokenStream {
        let ty = &self.ty;
        let items: &[Ident] = &self.items;
        let konst = crate::runner::konst(self.cfg.int.konst);

        let name_arms = items
            .into_iter()
            .map(|it| {
                let name = it.to_string();
                return quote! { #ty::#it => #name, };
            })
            .collect::<Vec<_>>();

        let from_name_arms = items
            .into_iter()
            .map(|it| {
                let name = it.to_string();
                return quote! { #name => Some(#ty::#it), };
            })
            .collect::<Vec<_>>();

        let item_items = items
            .into_iter()
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
            }
        };
    }

    fn ekran_value_t(&self) -> TokenStream {
        let ty_value = &self.ty_value;
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
}
