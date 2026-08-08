// AUTO-GENERATED VIA typekin-unexpand, DO NOT MODIFY
#![allow(clippy::needless_return)]
#![allow(dead_code)]
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_iter)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]
extern crate std;
use typekin_testing::demo_u128;
mod subject {
    use std::fmt::{Display, Formatter};
    #[repr(u128)]
    #[derive(
        :: core :: hash :: Hash,
        :: core :: fmt :: Debug,
        :: core :: marker :: Copy,
    )]
    #[derive_const(::core::clone::Clone)]
    #[derive_const(::core::cmp::Ord)]
    #[derive_const(::core::cmp::PartialEq)]
    #[derive_const(::core::cmp::Eq)]
    #[derive_const(::core::cmp::PartialOrd)]
    pub enum MyFlag {
        Z = 0,
        A = 10,
        B,
        C = 40,
    }
    #[repr(transparent)]
    #[derive(:: core :: fmt :: Debug, :: core :: marker :: Copy)]
    #[derive_const(::core::clone::Clone)]
    pub struct MyFlagValue(u128);
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    const _: () = {
        const impl ::core::convert::Into<u128> for MyFlagValue {
            #[inline(always)]
            fn into(self) -> u128 {
                return MyFlagValue::into_u128(self);
            }
        }
        const impl ::core::convert::TryInto<usize> for MyFlagValue {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<usize, Self::Error> {
                return MyFlagValue::try_into_usize(self);
            }
        }
        const impl ::core::convert::TryInto<isize> for MyFlagValue {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<isize, Self::Error> {
                return MyFlagValue::try_into_isize(self);
            }
        }
        const impl ::core::convert::TryInto<u8> for MyFlagValue {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u8, Self::Error> {
                return MyFlagValue::try_into_u8(self);
            }
        }
        const impl ::core::convert::TryInto<u16> for MyFlagValue {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u16, Self::Error> {
                return MyFlagValue::try_into_u16(self);
            }
        }
        const impl ::core::convert::TryInto<u32> for MyFlagValue {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u32, Self::Error> {
                return MyFlagValue::try_into_u32(self);
            }
        }
        const impl ::core::convert::TryInto<u64> for MyFlagValue {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u64, Self::Error> {
                return MyFlagValue::try_into_u64(self);
            }
        }
        const impl ::core::convert::TryInto<i8> for MyFlagValue {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i8, Self::Error> {
                return MyFlagValue::try_into_i8(self);
            }
        }
        const impl ::core::convert::TryInto<i16> for MyFlagValue {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i16, Self::Error> {
                return MyFlagValue::try_into_i16(self);
            }
        }
        const impl ::core::convert::TryInto<i32> for MyFlagValue {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i32, Self::Error> {
                return MyFlagValue::try_into_i32(self);
            }
        }
        const impl ::core::convert::TryInto<i64> for MyFlagValue {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i64, Self::Error> {
                return MyFlagValue::try_into_i64(self);
            }
        }
        const impl ::core::convert::TryInto<i128> for MyFlagValue {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i128, Self::Error> {
                return MyFlagValue::try_into_i128(self);
            }
        }
        const impl ::core::ops::Shr<usize> for MyFlagValue {
            type Output = Self;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        const impl ::core::ops::Shl<usize> for MyFlagValue {
            type Output = Self;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        const impl ::core::ops::ShrAssign<usize> for MyFlagValue {
            #[inline(always)]
            fn shr_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shr(other);
            }
        }
        const impl ::core::ops::ShlAssign<usize> for MyFlagValue {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shl(other);
            }
        }
        const impl ::core::ops::BitAndAssign<MyFlagValue> for MyFlagValue {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                other: MyFlagValue,
            ) {
                *self = self._and(other.0);
            }
        }
        const impl ::core::ops::AddAssign<MyFlagValue> for MyFlagValue {
            #[inline(always)]
            fn add_assign(
                &mut self,
                other: MyFlagValue,
            ) {
                *self = self._add(other.0);
            }
        }
        const impl ::core::ops::SubAssign<MyFlagValue> for MyFlagValue {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                other: MyFlagValue,
            ) {
                *self = self._sub(other.0);
            }
        }
        const impl ::core::ops::MulAssign<MyFlagValue> for MyFlagValue {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                other: MyFlagValue,
            ) {
                *self = self._mul(other.0);
            }
        }
        const impl ::core::ops::DivAssign<MyFlagValue> for MyFlagValue {
            #[inline(always)]
            fn div_assign(
                &mut self,
                other: MyFlagValue,
            ) {
                *self = self._div(other.0);
            }
        }
        const impl ::core::ops::RemAssign<MyFlagValue> for MyFlagValue {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                other: MyFlagValue,
            ) {
                *self = self._rem(other.0);
            }
        }
        const impl ::core::ops::BitOrAssign<MyFlagValue> for MyFlagValue {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                other: MyFlagValue,
            ) {
                *self = self._or(other.0);
            }
        }
        const impl ::core::cmp::Eq for MyFlagValue {}
        const impl ::core::cmp::Ord for MyFlagValue {
            #[inline(always)]
            fn cmp(
                &self,
                other: &Self,
            ) -> ::core::cmp::Ordering {
                return self.partial_cmp(other).unwrap();
            }
        }
        if !(size_of::<MyFlagValue>() == size_of::<u128>()) {
            {
                panic!("invalid memory layout: #ty(#el) != #el");
            };
        };
        const impl<T> ::core::cmp::PartialEq<T> for MyFlagValue
        where
            T: [const] FriendMathRel + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let that = Seal::conv_my_flag_value(rhs);
                return Self::raw(*self) == that;
            }
        }
        const impl<T> ::core::cmp::PartialOrd<T> for MyFlagValue
        where
            T: [const] PartialEq<MyFlagValue>
                + [const] FriendMathRel
                + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let that = Seal::conv_my_flag_value(rhs);
                return Self::raw(*self).partial_cmp(&that);
            }
        }
        const impl<T> ::core::ops::Add<T> for MyFlagValue
        where
            T: [const] FriendMathOps + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_flag_value(&rhs);
                return self._add(that);
            }
        }
        const impl<T> ::core::ops::Sub<T> for MyFlagValue
        where
            T: [const] FriendMathOps + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_flag_value(&rhs);
                return self._sub(that);
            }
        }
        const impl<T> ::core::ops::Mul<T> for MyFlagValue
        where
            T: [const] FriendMathOps + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_flag_value(&rhs);
                return self._mul(that);
            }
        }
        const impl<T> ::core::ops::Div<T> for MyFlagValue
        where
            T: [const] FriendMathOps + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_flag_value(&rhs);
                return self._div(that);
            }
        }
        const impl<T> ::core::ops::Rem<T> for MyFlagValue
        where
            T: [const] FriendMathOps + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_flag_value(&rhs);
                return self._rem(that);
            }
        }
        const impl<T> ::core::ops::BitAnd<T> for MyFlagValue
        where
            T: [const] FriendMathBit + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_flag_value(&rhs);
                return self._and(that);
            }
        }
        const impl<T> ::core::ops::BitOr<T> for MyFlagValue
        where
            T: [const] FriendMathBit + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_flag_value(&rhs);
                return self._or(that);
            }
        }
        const impl<T> ::core::ops::BitXor<T> for MyFlagValue
        where
            T: [const] FriendMathBit + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_flag_value(&rhs);
                return self._xor(that);
            }
        }
        const impl ::core::ops::Not for MyFlagValue {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        const trait Seal {
            #[must_use]
            fn conv_my_flag_value(&self) -> u128;
        }
        const trait FriendMake: [const] Seal {}
        const trait FriendMathOps: [const] Seal {}
        const trait FriendMathBit: [const] Seal {}
        const trait FriendMathRel: [const] Seal {}
        const impl Seal for MyFlagValue {
            #[inline(always)]
            fn conv_my_flag_value(&self) -> u128 {
                return Self::raw(*self);
            }
        }
        const impl FriendMake for MyFlagValue {}
        const impl FriendMathOps for MyFlagValue {}
        const impl FriendMathBit for MyFlagValue {}
        const impl FriendMathRel for MyFlagValue {}
        const impl FriendMathOps for u128 {}
        const impl FriendMathBit for u128 {}
        const impl FriendMathRel for u128 {}
        const impl FriendMake for u128 {}
        const impl Seal for u128 {
            #[inline(always)]
            fn conv_my_flag_value(&self) -> u128 {
                let it: u128 = *self;
                return it;
            }
        }
        impl ::core::fmt::Binary for MyFlagValue {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl ::core::fmt::Octal for MyFlagValue {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl ::core::fmt::LowerHex for MyFlagValue {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl ::core::fmt::UpperHex for MyFlagValue {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        impl MyFlagValue {
            #[inline(always)]
            #[allow(private_bounds)]
            pub const fn of<T, B>(it: B) -> MyFlagValue
            where
                T: [const] FriendMake,
                B: [const] ::core::borrow::Borrow<T>
                    + [const] ::core::marker::Destruct,
            {
                let this = Seal::conv_my_flag_value(it.borrow());
                return Self::_unchecked(this);
            }
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> u128 {
                return self.0;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u128(self) -> u128 {
                return Self::raw(self) as u128;
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u128(self) -> Result<u128, ()> {
                return Ok(Self::raw(self) as u128);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(self) -> Result<usize, ()> {
                let r = Self::raw(self);
                let t = r as usize;
                let s = t as u128;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) -> Result<isize, ()> {
                let r = Self::raw(self);
                let t = r as isize;
                let s = t as u128;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u8(self) -> Result<u8, ()> {
                let r = Self::raw(self);
                let t = r as u8;
                let s = t as u128;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u16(self) -> Result<u16, ()> {
                let r = Self::raw(self);
                let t = r as u16;
                let s = t as u128;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u32(self) -> Result<u32, ()> {
                let r = Self::raw(self);
                let t = r as u32;
                let s = t as u128;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u64(self) -> Result<u64, ()> {
                let r = Self::raw(self);
                let t = r as u64;
                let s = t as u128;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i8(self) -> Result<i8, ()> {
                let r = Self::raw(self);
                let t = r as i8;
                let s = t as u128;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i16(self) -> Result<i16, ()> {
                let r = Self::raw(self);
                let t = r as i16;
                let s = t as u128;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i32(self) -> Result<i32, ()> {
                let r = Self::raw(self);
                let t = r as i32;
                let s = t as u128;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i64(self) -> Result<i64, ()> {
                let r = Self::raw(self);
                let t = r as i64;
                let s = t as u128;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i128(self) -> Result<i128, ()> {
                let r = Self::raw(self);
                let t = r as i128;
                let s = t as u128;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _add(
                self,
                it: u128,
            ) -> Self {
                let this = Self::raw(self);
                let result = this + it;
                return Self::_unchecked(result);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _sub(
                self,
                it: u128,
            ) -> Self {
                let this = Self::raw(self);
                let result = this - it;
                return Self::_unchecked(result);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _mul(
                self,
                it: u128,
            ) -> Self {
                let this = Self::raw(self);
                let result = this * it;
                return Self::_unchecked(result);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _div(
                self,
                it: u128,
            ) -> Self {
                let this = Self::raw(self);
                let result = this / it;
                return Self::_unchecked(result);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _rem(
                self,
                it: u128,
            ) -> Self {
                let this = Self::raw(self);
                let result = this % it;
                return Self::_unchecked(result);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _xor(
                self,
                it: u128,
            ) -> Self {
                let this = Self::raw(self);
                let result = this ^ it;
                return Self::_unchecked(result);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _and(
                self,
                it: u128,
            ) -> Self {
                let this = Self::raw(self);
                let result = this & it;
                return Self::_unchecked(result);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _or(
                self,
                it: u128,
            ) -> Self {
                let this = Self::raw(self);
                let result = this | it;
                return Self::_unchecked(result);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _shr(
                self,
                it: usize,
            ) -> Self {
                let this = Self::raw(self);
                let result = this >> it;
                return Self::_unchecked(result);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _shl(
                self,
                it: usize,
            ) -> Self {
                let this = Self::raw(self);
                let result = this << it;
                return Self::_unchecked(result);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _not(self) -> Self {
                let this = Self::raw(self);
                let result = !this;
                return Self::_unchecked(result);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _eq(
                self,
                it: u128,
            ) -> bool {
                let this = Self::raw(self);
                let result = this == it;
                return result;
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _cmp(
                self,
                it: u128,
            ) -> ::core::cmp::Ordering {
                let this = Self::raw(self);
                let result = ::core::cmp::PartialOrd::partial_cmp(&this, &it);
                return result.unwrap();
            }
            #[inline(always)]
            pub const fn try_make(it: u128) -> Result<Self, u128> {
                return Ok(Self::_unchecked(it));
            }
            #[must_use]
            #[inline(always)]
            pub(self) const fn _unchecked(it: u128) -> Self {
                return Self(it);
            }
        }
    };
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    const _: () = {
        impl MyFlag {}
        type Flag = MyFlag;
        type Value = MyFlagValue;
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
            pub const fn raw(self) -> u128 {
                return self as u128;
            }
            #[inline(always)]
            #[must_use]
            pub const fn into_value(self) -> Value {
                return Value::of(self.raw());
            }
            #[must_use]
            #[inline(always)]
            pub const fn all() -> Value {
                return Value::all_unknown();
            }
            #[doc = r" Yield a set of flags values."]
            #[doc = r""]
            #[doc = r" Each yielded flags value will correspond to a defined named flag."]
            #[must_use]
            #[inline(always)]
            pub const fn iter() -> impl Iterator<Item = Self> {
                return IterItems { index: 0 };
            }
            #[doc = r" Yield a set of flags values."]
            #[doc = r""]
            #[doc = r" Each yielded flags value will correspond to a defined named flag."]
            #[must_use]
            #[inline(always)]
            pub const fn iter_values() -> impl Iterator<Item = Value> {
                return Value::all_known().iter();
            }
            #[must_use]
            #[inline(always)]
            pub const fn is_empty(self) -> bool {
                return self == Self::empty();
            }
            #[doc = r" Whether any set bits in `other` are also set in `self`."]
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
            #[doc = r" The bitwise or (`|`) of the bits in `self` and `other`."]
            #[must_use]
            #[inline(always)]
            pub const fn inserted(
                self,
                other: Self,
            ) -> Value {
                return self.into_value().inserted(other.into_value());
            }
            #[doc = r" The intersection of `self` with the complement of `other` (`&!`)."]
            #[doc = r""]
            #[doc = r" This method is not equivalent to `self & !other` when `other` has unknown bits set."]
            #[doc = r" `remove` won't truncate `other`, but the `!` operator will."]
            #[must_use]
            #[inline(always)]
            pub const fn removed(
                self,
                other: Self,
            ) -> Value {
                return self.into_value().removed(other.into_value());
            }
            #[doc = r" The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
            #[must_use]
            #[inline(always)]
            pub const fn toggled(
                self,
                other: Self,
            ) -> Value {
                return self.into_value().toggled(other.into_value());
            }
            #[doc = r" Call [`Self::insert`] when `value` is `true` or [`Self::remove`] when `value` is `false`."]
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
            #[doc = r" The bitwise or (`|`) of the bits in `self` and `other`."]
            #[must_use]
            #[inline(always)]
            pub const fn union_with(
                self,
                other: Self,
            ) -> Value {
                return self.into_value().union_with(other.into_value());
            }
            #[doc = r" The intersection of `self` with the complement of `other` (`&!`)."]
            #[doc = r""]
            #[doc = r" This method is not equivalent to `self & !other` when `other` has unknown bits set."]
            #[doc = r" `difference` won't truncate `other`, but the `!` operator will."]
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
            #[doc = r" The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
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
            #[doc = r" The bitwise negation (`!`) of the bits in `self`, truncating the result."]
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
            #[doc = r" Get a value with all bits unset."]
            #[must_use]
            #[inline(always)]
            pub const fn empty() -> Self {
                return Flag::empty().into_value();
            }
            #[doc = r" Get a value with all known bits set."]
            #[must_use]
            #[inline(always)]
            pub const fn all_known() -> Self {
                return Flag::all();
            }
            #[doc = r" Get a value with all unknown bits set."]
            #[must_use]
            #[inline(always)]
            pub const fn all_unknown() -> Self {
                return Self::all_known()._not();
            }
            #[doc = r" Yield a set of contained flags values."]
            #[doc = r""]
            #[doc = r" Each yielded flags value will correspond to a defined named flag."]
            #[must_use]
            #[inline(always)]
            pub const fn iter_known_flags(self) -> impl Iterator<Item = Flag> {
                return IterFlags {
                    value: self,
                    index: 0,
                };
            }
            #[doc = r" Yield a set of contained flags values."]
            #[doc = r""]
            #[doc = r" Each yielded flags value will correspond to a defined named flag. Any unknown bits"]
            #[doc = r" will be yielded together as a final flags value."]
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
            #[must_use]
            #[inline(always)]
            pub const fn into_known_bits(self) -> Self {
                return self & Self::all_known();
            }
            #[doc = r" Get the unknown bits from a value."]
            #[must_use]
            #[inline(always)]
            pub const fn into_unknown_bits(self) -> Self {
                return self & Self::all_unknown();
            }
            #[doc = r" This method will return `true` if any unknown bits are set."]
            #[must_use]
            #[inline(always)]
            pub const fn contains_unknown_bits(self) -> bool {
                return self != self.into_known_bits();
            }
            #[doc = r" Convert from a bits value."]
            #[doc = r""]
            #[doc = r" This method will return `None` if any unknown bits are set."]
            #[inline(always)]
            pub const fn try_as_known_bits_only(self) -> Result<Self, Self> {
                return if self.into_known_bits() == self {
                    Ok(self)
                }
                else {
                    Err(self)
                };
            }
            #[doc = r" Convert from a bits value, unsetting any unknown bits."]
            #[must_use]
            #[inline(always)]
            pub const fn truncated_into_known_bits(self) -> Self {
                return self & Self::all_known();
            }
            #[doc = r" Convert from a bits value, unsetting any unknown bits."]
            #[inline(always)]
            pub const fn truncate_into_known_bits(&mut self) {
                *self = self.truncated_into_known_bits();
            }
            #[doc = r" Whether all bits in this flags value are unset."]
            #[must_use]
            #[inline(always)]
            pub const fn is_empty(self) -> bool {
                return self == Self::empty();
            }
            #[doc = r" Whether all known bits in this flags value are set."]
            #[must_use]
            #[inline(always)]
            pub const fn is_exactly_all_known_bits(self) -> bool {
                return self == Self::all_known();
            }
            #[doc = r" Whether any set bits in `other` are also set in `self`."]
            #[must_use]
            #[inline(always)]
            pub const fn intersects(
                self,
                other: Self,
            ) -> bool {
                return (self & other) != Self::empty();
            }
            #[doc = r" Whether all set bits in `other` are also set in `self`."]
            #[must_use]
            #[inline(always)]
            pub const fn contains(
                self,
                other: Flag,
            ) -> bool {
                return self.contains_all(other.into_value());
            }
            #[doc = r" Whether all set bits in `other` are also set in `self`."]
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
            #[doc = r" Remove any unknown bits from the flags."]
            #[must_use]
            #[inline(always)]
            pub const fn truncated(self) -> Self {
                return self & Self::all_known();
            }
            #[doc = r" Remove any unknown bits from the flags."]
            #[inline(always)]
            pub const fn truncate(&mut self) {
                *self = self.truncated();
            }
            #[doc = r" The bitwise or (`|`) of the bits in `self` and `other`."]
            #[must_use]
            #[inline(always)]
            pub const fn inserted(
                self,
                other: Self,
            ) -> Self {
                return self | other;
            }
            #[doc = r" The bitwise or (`|`) of the bits in `self` and `other`."]
            #[inline(always)]
            pub const fn insert(
                &mut self,
                other: Self,
            ) {
                *self = self.inserted(other);
            }
            #[doc = r" The intersection of `self` with the complement of `other` (`&!`)."]
            #[doc = r""]
            #[doc = r" This method is not equivalent to `self & !other` when `other` has unknown bits set."]
            #[doc = r" `remove` won't truncate `other`, but the `!` operator will."]
            #[must_use]
            #[inline(always)]
            pub const fn removed(
                self,
                other: Self,
            ) -> Self {
                return self & !other;
            }
            #[doc = r" The intersection of `self` with the complement of `other` (`&!`)."]
            #[doc = r""]
            #[doc = r" This method is not equivalent to `self & !other` when `other` has unknown bits set."]
            #[doc = r" `remove` won't truncate `other`, but the `!` operator will."]
            #[inline(always)]
            pub const fn remove(
                &mut self,
                other: Self,
            ) {
                *self = self.removed(other);
            }
            #[doc = r" The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
            #[must_use]
            #[inline(always)]
            pub const fn toggled(
                self,
                other: Self,
            ) -> Self {
                return self ^ other;
            }
            #[doc = r" The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
            #[inline(always)]
            pub const fn toggle(
                &mut self,
                other: Self,
            ) {
                *self = self.toggled(other);
            }
            #[doc = r" Call [`Self::insert`] when `value` is `true` or [`Self::remove`] when `value` is `false`."]
            #[must_use]
            #[inline(always)]
            pub const fn with(
                self,
                other: Self,
            ) -> Self {
                return self | other;
            }
            #[doc = r" Call [`Self::insert`] when `value` is `true` or [`Self::remove`] when `value` is `false`."]
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
            #[doc = r" The bitwise and (`&`) of the bits in `self` and `other`."]
            #[must_use]
            #[inline(always)]
            pub const fn intersection_with(
                self,
                other: Self,
            ) -> Self {
                return self & other;
            }
            #[doc = r" The bitwise and (`&`) of the bits in `self` and `other`."]
            #[inline(always)]
            pub const fn intersection(
                &mut self,
                other: Self,
            ) {
                *self = self.intersection_with(other);
            }
            #[doc = r" The bitwise or (`|`) of the bits in `self` and `other`."]
            #[must_use]
            #[inline(always)]
            pub const fn union_with(
                self,
                other: Self,
            ) -> Self {
                return self | other;
            }
            #[doc = r" The bitwise or (`|`) of the bits in `self` and `other`."]
            #[inline(always)]
            pub const fn union(
                &mut self,
                other: Self,
            ) {
                *self = self.union_with(other);
            }
            #[doc = r" The intersection of `self` with the complement of `other` (`&!`)."]
            #[doc = r""]
            #[doc = r" This method is not equivalent to `self & !other` when `other` has unknown bits set."]
            #[doc = r" `difference` won't truncate `other`, but the `!` operator will."]
            #[must_use]
            #[inline(always)]
            pub const fn difference_with(
                self,
                other: Self,
            ) -> Self {
                return self & (!other);
            }
            #[doc = r" The intersection of `self` with the complement of `other` (`&!`)."]
            #[doc = r""]
            #[doc = r" This method is not equivalent to `self & !other` when `other` has unknown bits set."]
            #[doc = r" `difference` won't truncate `other`, but the `!` operator will."]
            #[inline(always)]
            pub const fn difference(
                &mut self,
                other: Self,
            ) {
                *self = self.difference_with(other);
            }
            #[doc = r" The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
            #[must_use]
            #[inline(always)]
            pub const fn symmetric_difference_with(
                self,
                other: Self,
            ) -> Self {
                return self ^ other;
            }
            #[doc = r" The bitwise exclusive-or (`^`) of the bits in `self` and `other`."]
            #[inline(always)]
            pub const fn symmetric_difference(
                &mut self,
                other: Self,
            ) {
                *self = self.symmetric_difference_with(other);
            }
            #[doc = r" The bitwise negation (`!`) of the bits in `self`, truncating the result."]
            #[must_use]
            #[inline(always)]
            pub const fn complemented(self) -> Self {
                return !self;
            }
            #[doc = r" The bitwise negation (`!`) of the bits in `self`, truncating the result."]
            #[inline(always)]
            pub const fn complement(&mut self) {
                *self = self.complemented();
            }
        }
        impl MyFlag {
            #[inline(always)]
            #[must_use]
            pub const fn name(self) -> &'static str {
                return match self {
                    MyFlag::Z => "Z",
                    MyFlag::A => "A",
                    MyFlag::B => "B",
                    MyFlag::C => "C",
                };
            }
            #[inline(always)]
            #[must_use]
            pub const fn items() -> &'static [Self] {
                const ITEMS: &'static [MyFlag] =
                    &[MyFlag::Z, MyFlag::A, MyFlag::B, MyFlag::C];
                return ITEMS;
            }
            #[inline]
            #[must_use]
            pub const fn from_name(name: &str) -> Option<Self> {
                return match name {
                    "Z" => Some(MyFlag::Z),
                    "A" => Some(MyFlag::A),
                    "B" => Some(MyFlag::B),
                    "C" => Some(MyFlag::C),
                    _ => None,
                };
            }
        }
    };
    impl MyFlag {
        #[inline(always)]
        #[must_use]
        pub const fn empty() -> Self {
            return Self::Z;
        }
    }
    impl Display for MyFlagValue {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            f.write_fmt(format_args!("MyFlagValue({0})", self.raw()))
        }
    }
}
type Subject = subject::MyFlag;
type Value = subject::MyFlagValue;
fn main() {
    let lhs = 0b11000;
    let rhs = 0b10100;
    let demo = demo_u128(Value::of(lhs), rhs, |it| it.raw());
    {
        print!("{0}\n", demo.print());
    };
    for x in Subject::items() {
        {
            print!("{0}\n", x.name());
        };
    }
}
