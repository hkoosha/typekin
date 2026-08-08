#![allow(dead_code)]

use crate::integral::driver::IntegralCfg;
use crate::runner::{Handy, mk_flags};
use crate::type_friendship::{FriendReq, FriendshipLevel};
use crate::value_type::N;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Path;
use syn::Visibility;

mk_flags! {
    #[derive(Debug, Clone)]
    pub(crate) struct BitFlags {
        pub make_value: bool = true,
        pub impl_value: bool = true,
        pub value_name: String = "",
        pub value_name_suffix: String = "Value",
    }
}

pub(super) struct Generator {
    ty: Ident,
    vl: Ident,
    vis: Visibility,
    el: Ident,
    items: Vec<Ident>,
    repr: N,
    int_cfg: IntegralCfg,
    bit: Box<BitFlags>,
    handy: Handy,
}

impl Generator {
    pub(super) fn new(
        ty: Ident,
        repr: N,
        int_cfg: IntegralCfg,
        bit: Box<BitFlags>,
        vis: Visibility,
        items: Vec<Ident>,
    ) -> Self {
        let vl = match (bit.value_name.as_str(), bit.value_name_suffix.as_str())
        {
            ("", "") => format_ident!("{}Value", ty),
            ("", suffix) => format_ident!("{}{}", ty, suffix),
            (prefix, "") => format_ident!("{}", prefix),
            (prefix, suffix) => format_ident!("{}{}", prefix, suffix),
        };

        let el = format_ident!("{}", repr.rust_name());

        let this = Self {
            handy: Handy::of(int_cfg.int.konst),
            vis,
            vl,
            ty,
            el,
            repr,
            int_cfg,
            bit,
            items,
        };

        return this;
    }

    pub(super) fn ekran(mut self) -> syn::Result<TokenStream> {
        let ty = &self.ty.clone();
        let impl_items = self.ekran_impl_items();
        let impls = self.ekran_impls();

        let mk_int = match self.bit.make_value {
            true => Some(self.make_int()),
            false => None,
        };

        let generic = self.gen_flag_generic();
        let flag = self.gen_flag_impl();

        let mut impl_int = None;
        if self.bit.impl_value {
            if !self.int_cfg.friends.iter().any(|it| {
                it.level.contains(&FriendshipLevel::Make)
                    && it.ty.is_ident(&self.el)
            }) {
                if let Some(friendship) = self
                    .int_cfg
                    .friends
                    .iter_mut()
                    .find(|it| it.ty.is_ident(&self.el))
                {
                    friendship.level.insert(FriendshipLevel::Make);
                }
                else {
                    self.int_cfg.friends.push(FriendReq {
                        ty: Path::from(self.el.clone()),
                        level: FriendshipLevel::Make.to_set(),
                        conv: None,
                    })
                }
            }

            let im =
                crate::integral::driver::ekran(self.vl, self.el, self.int_cfg)?;
            impl_int = Some(im);
        };

        let it = quote::quote! {
            #mk_int

            #impl_int

            #[allow(dead_code)]
            #[allow(unused_qualifications)]
            const _: () = {
                #impls

                impl #ty {
                    #impl_items
                }

                #generic

                #flag
            };
        };

        return Ok(it);
    }

    fn make_int(&self) -> TokenStream {
        let el = &self.el;
        let vl = &self.vl;

        return quote! {
            #[derive(Copy)]
            #[derive_const(Clone)]
            #[repr(transparent)]
            pub struct #vl(#el);
        };
    }

    fn ekran_impl_items(&self) -> TokenStream {
        let stream = TokenStream::new();

        if self.int_cfg.int.impl_fmt_hex_upper {
            // foo
        }

        return stream;
    }

    fn ekran_impls(&self) -> TokenStream {
        let stream = TokenStream::new();

        if self.int_cfg.int.impl_fmt_hex_upper {
            // foo
        }

        return stream;
    }

    fn gen_flag_generic(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let el = &self.el;

        return quote! {
            type Flag = #ty;
            type Value = #vl;

            struct IterItems {
                index: usize,
            }

            const impl core::iter::Iterator for IterItems {
                type Item = Flag;

                fn next(&mut self) -> Option<Self::Item> {
                    const ITER_ITEMS: &'static [Flag] = Flag::items();
                    const MAX: usize = ITER_ITEMS.len();

                    while self.index < MAX {
                        let next = ITER_ITEMS[self.index];
                        self.index += 1;
                        return Some(next);
                    }

                    return None;
                }

                #[inline(always)]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    const MAX: usize = Flag::items().len();

                    return (MAX, Some(MAX));
                }
            }

            struct IterFlags {
                value: Value,
                index: usize,
            }

            const impl core::iter::Iterator for IterFlags {
                type Item = Flag;

                fn next(&mut self) -> Option<Self::Item> {
                    const ITER_ITEMS: &'static [Flag] = Flag::items();
                    const MAX: usize = ITER_ITEMS.len();

                    while self.index < MAX {
                        let next = ITER_ITEMS[self.index];
                        self.index += 1;
                        if self.value.contains(next) {
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

            const impl Iterator for IterValues {
                type Item = Value;

                fn next(&mut self) -> Option<Self::Item> {
                    const ITER_ITEMS: &'static [Flag] = Flag::items();
                    const MAX: usize = ITER_ITEMS.len();

                    while self.index < MAX {
                        let next = ITER_ITEMS[self.index].into_value();
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

            impl Flag {
                #[inline(always)]
                #[must_use]
                pub const fn raw(self) -> #el {
                    return self as #el;
                }

                #[inline(always)]
                #[must_use]
                pub const fn into_value(self) -> Value {
                    return Value::of(self.raw());
                }

                //noinspection RsUnresolvedPath
                #[must_use]
                #[inline(always)]
                pub const fn all() -> Value {
                    return Value::all_unknown();
                }

                /// Yield a set of flags values.
                ///
                /// Each yielded flags value will correspond to a defined named flag.
                #[must_use]
                #[inline(always)]
                pub const fn iter() -> impl Iterator<Item = Self> {
                    return IterItems { index: 0 };
                }

                //noinspection RsUnresolvedPath
                /// Yield a set of flags values.
                ///
                /// Each yielded flags value will correspond to a defined named flag.
                #[must_use]
                #[inline(always)]
                pub const fn iter_values() -> impl Iterator<Item = Value> {
                    return Value::all_known().iter();
                }

                // =========================================================================

                #[must_use]
                #[inline(always)]
                pub const fn is_empty(self) -> bool {
                    return self == Self::empty();
                }

                /// Whether any set bits in `other` are also set in `self`.
                #[must_use]
                #[inline(always)]
                pub const fn intersects(
                    self,
                    other: Value,
                ) -> bool {
                    return self.into_value().intersects(other);
                }

                #[must_use]
                #[inline(always)]
                pub const fn contained_in(
                    self,
                    other: Self,
                ) -> bool {
                    return other.into_value().contains_all(self.into_value());
                }

                /// The bitwise or (`|`) of the bits in `self` and `other`.
                #[must_use]
                #[inline(always)]
                pub const fn inserted(
                    self,
                    other: Self,
                ) -> Value {
                    return self.into_value().inserted(other.into_value());
                }

                /// The intersection of `self` with the complement of `other` (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `remove` won't truncate `other`, but the `!` operator will.
                #[must_use]
                #[inline(always)]
                pub const fn removed(
                    self,
                    other: Self,
                ) -> Value {
                    return self.into_value().removed(other.into_value());
                }

                /// The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
                #[must_use]
                #[inline(always)]
                pub const fn toggled(
                    self,
                    other: Self,
                ) -> Value {
                    return self.into_value().toggled(other.into_value());
                }

                /// Call [`Self::insert`] when `value` is `true` or [`Self::remove`] when `value` is `false`.
                #[must_use]
                #[inline(always)]
                pub const fn with(
                    self,
                    other: Self,
                ) -> Value {
                    return self.into_value().with(other.into_value());
                }

                #[must_use]
                #[inline(always)]
                pub const fn without(
                    self,
                    other: Self,
                ) -> Value {
                    return self.into_value().without(other.into_value());
                }

                #[inline(always)]
                pub const fn unset(
                    &mut self,
                    other: Self,
                ) {
                    if *self == other {
                        *self = Self::empty();
                    }
                }

                #[must_use]
                #[inline(always)]
                pub const fn intersection_with(
                    self,
                    other: Self,
                ) -> Self {
                    return if self == other { self } else { Self::empty() };
                }

                /// The bitwise or (`|`) of the bits in `self` and `other`.
                #[must_use]
                #[inline(always)]
                pub const fn union_with(
                    self,
                    other: Self,
                ) -> Value {
                    return self.into_value().union_with(other.into_value());
                }

                /// The intersection of `self` with the complement of `other` (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[must_use]
                #[inline(always)]
                pub const fn difference_with(
                    self,
                    other: Self,
                ) -> Self {
                    return if other.contained_in(self) {
                        self
                    }
                    else {
                        Self::empty()
                    };
                }

                /// The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
                #[must_use]
                #[inline(always)]
                pub const fn symmetric_difference_with(
                    self,
                    other: Self,
                ) -> Value {
                    return self
                        .into_value()
                        .symmetric_difference_with(other.into_value());
                }

                /// The bitwise negation (`!`) of the bits in `self`, truncating the result.
                #[must_use]
                #[inline(always)]
                pub const fn complemented(self) -> Value {
                    return self.into_value().complemented();
                }
            }

            impl Value {
                pub const fn from_name(name: &str) -> Option<Self> {
                    return match Flag::from_name(name) {
                        None => None,
                        Some(it) => Some(it.into_value()),
                    };
                }

                pub const fn into_flag(self) -> Result<Flag, Self> {
                    const ITEMS: &'static [Flag] = Flag::items();
                    const MAX: usize = ITEMS.len();

                    let mut i = 0;
                    while i < MAX {
                        let it = &ITEMS[i];
                        if it.into_value() == self {
                            return Ok(*it);
                        }
                        i += 1;
                    }

                    return Err(self);
                }

                // =========================================================================

                /// Get a value with all bits unset.
                #[must_use]
                #[inline(always)]
                pub const fn empty() -> Self {
                    return Flag::empty().into_value();
                }

                /// Get a value with all known bits set.
                #[must_use]
                #[inline(always)]
                pub const fn all_known() -> Self {
                    return Flag::all();
                }

                /// Get a value with all unknown bits set.
                #[must_use]
                #[inline(always)]
                pub const fn all_unknown() -> Self {
                    return Self::all_known()._not();
                }

                // =========================================================================

                /// Yield a set of contained flags values.
                ///
                /// Each yielded flags value will correspond to a defined named flag.
                #[must_use]
                #[inline(always)]
                pub const fn iter_known_flags(self) -> impl Iterator<Item = Flag> {
                    return IterFlags {
                        value: self,
                        index: 0,
                    };
                }

                /// Yield a set of contained flags values.
                ///
                /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
                /// will be yielded together as a final flags value.
                #[must_use]
                pub const fn iter(self) -> impl Iterator<Item = Self> {
                    return IterValues {
                        value: self,
                        index: 0,
                    };
                }

                #[inline(always)]
                #[must_use]
                pub const fn into_iter(self) -> impl Iterator<Item = Self> {
                    return self.iter();
                }

                #[inline(always)]
                #[must_use]
                pub const fn into_iter_known_flags(
                    self
                ) -> impl Iterator<Item = Flag> {
                    return self.iter_known_flags();
                }

                // =========================================================================

                #[must_use]
                #[inline(always)]
                pub const fn into_known_bits(self) -> Self {
                    return self & Self::all_known();
                }

                /// Get the unknown bits from a value.
                #[must_use]
                #[inline(always)]
                pub const fn into_unknown_bits(self) -> Self {
                    return self & Self::all_unknown();
                }

                /// This method will return `true` if any unknown bits are set.
                #[must_use]
                #[inline(always)]
                pub const fn contains_unknown_bits(self) -> bool {
                    return self != self.into_known_bits();
                }

                /// Convert from a bits value.
                ///
                /// This method will return `None` if any unknown bits are set.
                #[inline(always)]
                pub const fn try_as_known_bits_only(self) -> Result<Self, Self> {
                    return if self.into_known_bits() == self {
                        Ok(self)
                    }
                    else {
                        Err(self)
                    };
                }

                /// Convert from a bits value, unsetting any unknown bits.
                #[must_use]
                #[inline(always)]
                pub const fn truncated_into_known_bits(self) -> Self {
                    return self & Self::all_known();
                }

                /// Convert from a bits value, unsetting any unknown bits.
                #[inline(always)]
                pub const fn truncate_into_known_bits(&mut self) {
                    *self = self.truncated_into_known_bits();
                }

                /// Whether all bits in this flags value are unset.
                #[must_use]
                #[inline(always)]
                pub const fn is_empty(self) -> bool {
                    return self == Self::empty();
                }

                /// Whether all known bits in this flags value are set.
                #[must_use]
                #[inline(always)]
                pub const fn is_exactly_all_known_bits(self) -> bool {
                    return self == Self::all_known();
                }

                /// Whether any set bits in `other` are also set in `self`.
                #[must_use]
                #[inline(always)]
                pub const fn intersects(
                    self,
                    other: Self,
                ) -> bool {
                    return (self & other) != Self::empty();
                }

                /// Whether all set bits in `other` are also set in `self`.
                #[must_use]
                #[inline(always)]
                pub const fn contains(
                    self,
                    other: Flag,
                ) -> bool {
                    return self.contains_all(other.into_value());
                }

                /// Whether all set bits in `other` are also set in `self`.
                #[must_use]
                #[inline(always)]
                pub const fn contains_all(
                    self,
                    other: Self,
                ) -> bool {
                    return self & other == other;
                }

                #[must_use]
                #[inline(always)]
                pub const fn contains_any(
                    self,
                    other: Self,
                ) -> bool {
                    return self & other != Self::empty();
                }

                /// Remove any unknown bits from the flags.
                #[must_use]
                #[inline(always)]
                pub const fn truncated(self) -> Self {
                    return self & Self::all_known();
                }

                /// Remove any unknown bits from the flags.
                #[inline(always)]
                pub const fn truncate(&mut self) {
                    *self = self.truncated();
                }

                /// The bitwise or (`|`) of the bits in `self` and `other`.
                #[must_use]
                #[inline(always)]
                pub const fn inserted(
                    self,
                    other: Self,
                ) -> Self {
                    return self | other;
                }

                /// The bitwise or (`|`) of the bits in `self` and `other`.
                #[inline(always)]
                pub const fn insert(
                    &mut self,
                    other: Self,
                ) {
                    *self = self.inserted(other);
                }

                /// The intersection of `self` with the complement of `other` (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `remove` won't truncate `other`, but the `!` operator will.
                #[must_use]
                #[inline(always)]
                pub const fn removed(
                    self,
                    other: Self,
                ) -> Self {
                    return self & !other;
                }

                /// The intersection of `self` with the complement of `other` (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `remove` won't truncate `other`, but the `!` operator will.
                #[inline(always)]
                pub const fn remove(
                    &mut self,
                    other: Self,
                ) {
                    *self = self.removed(other);
                }

                /// The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
                #[must_use]
                #[inline(always)]
                pub const fn toggled(
                    self,
                    other: Self,
                ) -> Self {
                    return self ^ other;
                }

                /// The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
                #[inline(always)]
                pub const fn toggle(
                    &mut self,
                    other: Self,
                ) {
                    *self = self.toggled(other);
                }

                /// Call [`Self::insert`] when `value` is `true` or [`Self::remove`] when `value` is `false`.
                #[must_use]
                #[inline(always)]
                pub const fn with(
                    self,
                    other: Self,
                ) -> Self {
                    return self | other;
                }

                /// Call [`Self::insert`] when `value` is `true` or [`Self::remove`] when `value` is `false`.
                #[inline(always)]
                pub const fn set(
                    &mut self,
                    other: Self,
                ) {
                    *self = self.with(other);
                }

                #[must_use]
                #[inline(always)]
                pub const fn without(
                    self,
                    other: Self,
                ) -> Self {
                    return self & (!other);
                }

                #[inline(always)]
                pub const fn unset(
                    &mut self,
                    other: Self,
                ) {
                    *self = self.without(other);
                }

                /// The bitwise and (`&`) of the bits in `self` and `other`.
                #[must_use]
                #[inline(always)]
                pub const fn intersection_with(
                    self,
                    other: Self,
                ) -> Self {
                    return self & other;
                }

                /// The bitwise and (`&`) of the bits in `self` and `other`.
                #[inline(always)]
                pub const fn intersection(
                    &mut self,
                    other: Self,
                ) {
                    *self = self.intersection_with(other);
                }

                /// The bitwise or (`|`) of the bits in `self` and `other`.
                #[must_use]
                #[inline(always)]
                pub const fn union_with(
                    self,
                    other: Self,
                ) -> Self {
                    return self | other;
                }

                /// The bitwise or (`|`) of the bits in `self` and `other`.
                #[inline(always)]
                pub const fn union(
                    &mut self,
                    other: Self,
                ) {
                    *self = self.union_with(other);
                }

                /// The intersection of `self` with the complement of `other` (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[must_use]
                #[inline(always)]
                pub const fn difference_with(
                    self,
                    other: Self,
                ) -> Self {
                    return self & (!other);
                }

                /// The intersection of `self` with the complement of `other` (`&!`).
                ///
                /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
                /// `difference` won't truncate `other`, but the `!` operator will.
                #[inline(always)]
                pub const fn difference(
                    &mut self,
                    other: Self,
                ) {
                    *self = self.difference_with(other);
                }

                /// The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
                #[must_use]
                #[inline(always)]
                pub const fn symmetric_difference_with(
                    self,
                    other: Self,
                ) -> Self {
                    return self ^ other;
                }

                /// The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
                #[inline(always)]
                pub const fn symmetric_difference(
                    &mut self,
                    other: Self,
                ) {
                    *self = self.symmetric_difference_with(other);
                }

                /// The bitwise negation (`!`) of the bits in `self`, truncating the result.
                #[must_use]
                #[inline(always)]
                pub const fn complemented(self) -> Self {
                    return !self;
                }

                /// The bitwise negation (`!`) of the bits in `self`, truncating the result.
                #[inline(always)]
                pub const fn complement(&mut self) {
                    *self = self.complemented();
                }
            }
        };
    }

    fn gen_flag_impl(&self) -> TokenStream {
        let ty = &self.ty;
        let items: &[Ident] = &self.items;

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
                pub const fn name(self) -> &'static str {
                    return match self {
                        #(#name_arms)*
                    };
                }

                #[inline(always)]
                #[must_use]
                pub const fn items() -> &'static [Self] {
                    const ITEMS: &'static [#ty] = &[#(#item_items),*];

                    return ITEMS;
                }

                #[inline]
                #[must_use]
                pub const fn from_name(name: &str) -> Option<Self> {
                    return match name {
                        #(#from_name_arms)*

                        _ => ::core::option::Option::None,
                    };
                }
            }
        };
    }
}
