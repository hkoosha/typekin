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
mod subject {
    #[repr(u128)]
    #[derive(
        :: core :: hash :: Hash,
        :: core :: marker :: Copy,
        :: core :: fmt :: Debug,
    )]
    #[derive_const(::core::cmp::Eq)]
    #[derive_const(::core::cmp::Ord)]
    #[derive_const(::core::cmp::PartialOrd)]
    #[derive_const(::core::clone::Clone)]
    #[derive_const(::core::cmp::PartialEq)]
    pub enum MyFlag {
        Z = 0,
        A = 10,
        B,
        C = 40,
    }
    #[repr(transparent)]
    #[derive(:: core :: marker :: Copy, :: core :: fmt :: Debug)]
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
                *self = self._bitand(other.0);
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
                *self = self._bitor(other.0);
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
        if !(::core::mem::size_of::<MyFlagValue>()
            == ::core::mem::size_of::<u128>())
        {
            {
                panic!("invalid memory layout: #ty(#el) != #el");
            };
        };
        const impl<T> ::core::ops::Add<T> for MyFlagValue
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flag_value(&rhs);
                return self._add(it);
            }
        }
        const impl<T> ::core::ops::Sub<T> for MyFlagValue
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flag_value(&rhs);
                return self._sub(it);
            }
        }
        const impl<T> ::core::ops::Mul<T> for MyFlagValue
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flag_value(&rhs);
                return self._mul(it);
            }
        }
        const impl<T> ::core::ops::Div<T> for MyFlagValue
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flag_value(&rhs);
                return self._div(it);
            }
        }
        const impl<T> ::core::ops::Rem<T> for MyFlagValue
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flag_value(&rhs);
                return self._rem(it);
            }
        }
        const impl<T> ::core::ops::BitAnd<T> for MyFlagValue
        where
            T: [const] FriendBit + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flag_value(&rhs);
                return self._bitand(it);
            }
        }
        const impl<T> ::core::ops::BitOr<T> for MyFlagValue
        where
            T: [const] FriendBit + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flag_value(&rhs);
                return self._bitor(it);
            }
        }
        const impl<T> ::core::ops::BitXor<T> for MyFlagValue
        where
            T: [const] FriendBit + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flag_value(&rhs);
                return self._bitxor(it);
            }
        }
        const impl Seal for MyFlagValue {
            #[inline(always)]
            fn conv_my_flag_value(&self) -> u128 {
                return Self::raw(*self);
            }
        }
        const impl FriendMake for MyFlagValue {}
        const impl FriendMath for MyFlagValue {}
        const impl FriendBit for MyFlagValue {}
        const impl FriendRel for MyFlagValue {}
        impl ::core::fmt::Binary for MyFlagValue {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl ::core::fmt::Octal for MyFlagValue {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl ::core::fmt::LowerHex for MyFlagValue {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl ::core::fmt::UpperHex for MyFlagValue {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        const impl Seal for u128 {
            #[inline(always)]
            fn conv_my_flag_value(&self) -> u128 {
                let it: u128 = (*self).into();
                return it;
            }
        }
        const impl Seal for MyFlag {
            #[inline(always)]
            fn conv_my_flag_value(&self) -> u128 {
                return MyFlag::raw(*self);
            }
        }
        const impl FriendMath for u128 {}
        const impl FriendBit for u128 {}
        const impl FriendRel for u128 {}
        const impl FriendMake for u128 {}
        const impl FriendRel for MyFlag {}
        const impl FriendBit for MyFlag {}
        const impl<T> ::core::cmp::PartialEq<T> for MyFlagValue
        where
            T: [const] FriendRel + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_my_flag_value(rhs);
                return lhs == rhs;
            }
        }
        const impl<T> ::core::cmp::PartialOrd<T> for MyFlagValue
        where
            T: [const] FriendRel + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_my_flag_value(rhs);
                return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
            }
        }
        const trait Seal {
            fn conv_my_flag_value(&self) -> u128;
        }
        const impl<T> Seal for &T
        where
            T: [const] Seal,
        {
            #[inline(always)]
            fn conv_my_flag_value(&self) -> u128 {
                return Seal::conv_my_flag_value(&**self);
            }
        }
        const impl<T> Seal for &mut T
        where
            T: [const] Seal,
        {
            #[inline(always)]
            fn conv_my_flag_value(&self) -> u128 {
                return Seal::conv_my_flag_value(&**self);
            }
        }
        const trait FriendMake: [const] Seal {}
        const impl<T> FriendMake for &T where T: [const] FriendMake {}
        const impl<T> FriendMake for &mut T where T: [const] FriendMake {}
        const trait FriendMath: [const] Seal {}
        const impl<T> FriendMath for &T where T: [const] FriendMath {}
        const impl<T> FriendMath for &mut T where T: [const] FriendMath {}
        const trait FriendBit: [const] Seal {}
        const impl<T> FriendBit for &T where T: [const] FriendBit {}
        const impl<T> FriendBit for &mut T where T: [const] FriendBit {}
        const trait FriendRel: [const] Seal {}
        const impl<T> FriendRel for &T where T: [const] FriendRel {}
        const impl<T> FriendRel for &mut T where T: [const] FriendRel {}
        impl MyFlagValue {
            #[inline(always)]
            #[allow(private_bounds)]
            pub const fn of<T>(it: T) -> MyFlagValue
            where
                T: [const] FriendMake + [const] ::core::marker::Destruct,
            {
                let this = Seal::conv_my_flag_value(&it);
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
                let it = Self::raw(self);
                return it as u128;
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
                return if s == r && true { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) -> Result<isize, ()> {
                let r = Self::raw(self);
                let t = r as isize;
                let s = t as u128;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u8(self) -> Result<u8, ()> {
                let r = Self::raw(self);
                let t = r as u8;
                let s = t as u128;
                return if s == r && true { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u16(self) -> Result<u16, ()> {
                let r = Self::raw(self);
                let t = r as u16;
                let s = t as u128;
                return if s == r && true { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u32(self) -> Result<u32, ()> {
                let r = Self::raw(self);
                let t = r as u32;
                let s = t as u128;
                return if s == r && true { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u64(self) -> Result<u64, ()> {
                let r = Self::raw(self);
                let t = r as u64;
                let s = t as u128;
                return if s == r && true { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i8(self) -> Result<i8, ()> {
                let r = Self::raw(self);
                let t = r as i8;
                let s = t as u128;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i16(self) -> Result<i16, ()> {
                let r = Self::raw(self);
                let t = r as i16;
                let s = t as u128;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i32(self) -> Result<i32, ()> {
                let r = Self::raw(self);
                let t = r as i32;
                let s = t as u128;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i64(self) -> Result<i64, ()> {
                let r = Self::raw(self);
                let t = r as i64;
                let s = t as u128;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i128(self) -> Result<i128, ()> {
                let r = Self::raw(self);
                let t = r as i128;
                let s = t as u128;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            pub(self) const fn _add(
                &self,
                rhs: u128,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs + rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _sub(
                &self,
                rhs: u128,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs - rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _mul(
                &self,
                rhs: u128,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs * rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _div(
                &self,
                rhs: u128,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs / rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _rem(
                &self,
                rhs: u128,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs % rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _bitxor(
                &self,
                rhs: u128,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs ^ rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _bitand(
                &self,
                rhs: u128,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs & rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _bitor(
                &self,
                rhs: u128,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs | rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _shr(
                &self,
                count: usize,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs >> count;
                return Self::_unchecked(it);
            }
            pub(self) const fn _shl(
                &self,
                count: usize,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs << count;
                return Self::_unchecked(it);
            }
            pub(self) const fn _not(&self) -> Self {
                let lhs = Self::raw(*self);
                let it = !lhs;
                return Self::_unchecked(it);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _eq(
                self,
                rhs: u128,
            ) -> bool {
                let lhs = Self::raw(self);
                return lhs == rhs;
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) fn _cmp(
                self,
                rhs: u128,
            ) -> ::core::cmp::Ordering {
                let lhs = Self::raw(self);
                return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs)
                    .unwrap();
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
        type Flag = MyFlag;
        type Value = MyFlagValue;
        struct IterItems {
            index: usize,
        }
        const impl core::iter::Iterator for IterItems {
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
        const impl core::iter::Iterator for IterFlags {
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
        const impl Iterator for IterValues {
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
        const impl ::core::ops::Not for Value {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self.complemented();
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
                return Value::from_bits_retain(self.raw());
            }
            #[must_use]
            #[inline(always)]
            pub const fn all() -> Value {
                #[allow(clippy::unnecessary_cast)]
                return Value::from_bits_retain(
                    0 as u128
                        | (MyFlag::Z as u128)
                        | (MyFlag::A as u128)
                        | (MyFlag::B as u128)
                        | (MyFlag::C as u128),
                );
            }
            #[must_use]
            #[inline(always)]
            pub const fn iter() -> impl Iterator<Item = Self> {
                return IterItems { index: 0 };
            }
            #[must_use]
            #[inline(always)]
            pub const fn iter_values() -> impl Iterator<Item = Value> {
                return Value::all().iter();
            }
            #[must_use]
            #[inline(always)]
            pub const fn inserted(
                self,
                other: Self,
            ) -> Value {
                return self.into_value().inserted(other.into_value());
            }
        }
        impl Value {
            #[must_use]
            #[inline(always)]
            pub const fn bits(self) -> u128 {
                return self.raw();
            }
            #[must_use]
            #[inline(always)]
            pub const fn from_bits_retain(bits: u128) -> Self {
                return Self::of(bits);
            }
            #[must_use]
            #[inline(always)]
            pub const fn empty() -> Self {
                return Self::from_bits_retain(0);
            }
            #[must_use]
            #[inline(always)]
            pub const fn all() -> Self {
                return Flag::all();
            }
            #[must_use]
            #[inline(always)]
            pub const fn all_named() -> Self {
                return Self::all();
            }
            #[must_use]
            #[inline(always)]
            pub const fn all_known() -> Self {
                return Self::all();
            }
            #[must_use]
            #[inline(always)]
            pub const fn all_unknown() -> Self {
                return Self::from_bits_retain(!Self::all().bits());
            }
            #[must_use]
            #[inline(always)]
            pub const fn known_bits(self) -> u128 {
                return self.bits() & Self::all().bits();
            }
            #[must_use]
            #[inline(always)]
            pub const fn unknown_bits(self) -> u128 {
                return self.bits() & !Self::all().bits();
            }
            #[must_use]
            #[inline(always)]
            pub const fn into_known_bits(self) -> Self {
                return Self::from_bits_retain(self.known_bits());
            }
            #[must_use]
            #[inline(always)]
            pub const fn into_unknown_bits(self) -> Self {
                return Self::from_bits_retain(self.unknown_bits());
            }
            #[must_use]
            #[inline(always)]
            pub const fn contains_unknown_bits(self) -> bool {
                return self.unknown_bits() != 0;
            }
            #[must_use]
            #[inline(always)]
            pub const fn from_bits(bits: u128) -> Option<Self> {
                let value = Self::from_bits_retain(bits);
                return if value.contains_unknown_bits() {
                    None
                }
                else {
                    Some(value)
                };
            }
            #[inline(always)]
            pub const fn try_as_known_bits_only(self) -> Result<Self, Self> {
                return if self.contains_unknown_bits() {
                    Err(self)
                }
                else {
                    Ok(self)
                };
            }
            #[must_use]
            #[inline(always)]
            pub const fn from_bits_truncate(bits: u128) -> Self {
                return Self::from_bits_retain(bits).truncated();
            }
            #[must_use]
            #[inline(always)]
            pub const fn truncated_into_known_bits(self) -> Self {
                return self.truncated();
            }
            #[inline(always)]
            pub const fn truncate_into_known_bits(&mut self) {
                self.truncate();
            }
            #[must_use]
            #[inline(always)]
            pub const fn from_name(name: &str) -> Option<Self> {
                return match Flag::from_name(name) {
                    Some(flag) => Some(flag.into_value()),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn into_flag(self) -> Result<Flag, Self> {
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
            pub const fn iter_known_flags(self) -> impl Iterator<Item = Flag> {
                return IterFlags {
                    value: self,
                    index: 0,
                };
            }
            #[must_use]
            #[inline(always)]
            pub const fn iter(self) -> impl Iterator<Item = Self> {
                return IterValues {
                    value: self,
                    index: 0,
                };
            }
            #[must_use]
            #[inline(always)]
            pub fn iter_names(
                self
            ) -> impl Iterator<Item = (&'static str, Self)> {
                return self
                    .iter_known_flags()
                    .map(|flag| (flag.name(), flag.into_value()));
            }
            #[must_use]
            #[inline(always)]
            pub fn iter_defined_names()
            -> impl Iterator<Item = (&'static str, Self)> {
                return Flag::iter()
                    .map(|flag| (flag.name(), flag.into_value()));
            }
            #[must_use]
            #[inline(always)]
            pub fn iter_equal_names(
                self
            ) -> impl Iterator<Item = &'static str> {
                return Flag::iter()
                    .filter(move |flag| flag.into_value() == self)
                    .map(|flag| flag.name());
            }
            #[must_use]
            #[inline(always)]
            pub const fn is_empty(self) -> bool {
                return self.bits() == 0;
            }
            #[must_use]
            #[inline(always)]
            pub const fn is_all(self) -> bool {
                return self.contains_all(Self::all());
            }
            #[must_use]
            #[inline(always)]
            pub const fn is_exactly_all_known_bits(self) -> bool {
                return self == Self::all();
            }
            #[must_use]
            #[inline(always)]
            pub const fn intersects(
                self,
                other: Self,
            ) -> bool {
                return self.bits() & other.bits() != 0;
            }
            #[must_use]
            #[inline(always)]
            pub fn contains<T>(
                self,
                other: T,
            ) -> bool
            where
                T: Into<Self>,
            {
                return self.contains_all(other.into());
            }
            #[must_use]
            #[inline(always)]
            pub const fn contains_all(
                self,
                other: Self,
            ) -> bool {
                return self.bits() & other.bits() == other.bits();
            }
            #[must_use]
            #[inline(always)]
            pub const fn contains_any(
                self,
                other: Self,
            ) -> bool {
                return self.intersects(other);
            }
            #[must_use]
            #[inline(always)]
            pub const fn truncated(self) -> Self {
                return Self::from_bits_retain(self.known_bits());
            }
            #[inline(always)]
            pub const fn truncate(&mut self) {
                *self = self.truncated();
            }
            #[must_use]
            #[inline(always)]
            pub const fn inserted(
                self,
                other: Self,
            ) -> Self {
                return Self::from_bits_retain(self.bits() | other.bits());
            }
            #[inline(always)]
            pub const fn insert(
                &mut self,
                other: Self,
            ) {
                *self = self.inserted(other);
            }
            #[must_use]
            #[inline(always)]
            pub const fn removed(
                self,
                other: Self,
            ) -> Self {
                return Self::from_bits_retain(self.bits() & !other.bits());
            }
            #[inline(always)]
            pub const fn remove(
                &mut self,
                other: Self,
            ) {
                *self = self.removed(other);
            }
            #[must_use]
            #[inline(always)]
            pub const fn toggled(
                self,
                other: Self,
            ) -> Self {
                return Self::from_bits_retain(self.bits() ^ other.bits());
            }
            #[inline(always)]
            pub const fn toggle(
                &mut self,
                other: Self,
            ) {
                *self = self.toggled(other);
            }
            #[inline(always)]
            pub const fn set(
                &mut self,
                other: Self,
                value: bool,
            ) {
                if value {
                    self.insert(other);
                }
                else {
                    self.remove(other);
                }
            }
            #[inline(always)]
            pub const fn clear(&mut self) {
                *self = Self::empty();
            }
            #[must_use]
            #[inline(always)]
            pub const fn with(
                self,
                other: Self,
            ) -> Self {
                return self.inserted(other);
            }
            #[must_use]
            #[inline(always)]
            pub const fn without(
                self,
                other: Self,
            ) -> Self {
                return self.removed(other);
            }
            #[inline(always)]
            pub const fn unset(
                &mut self,
                other: Self,
            ) {
                self.remove(other);
            }
            #[must_use]
            #[inline(always)]
            pub const fn intersection(
                self,
                other: Self,
            ) -> Self {
                return Self::from_bits_retain(self.bits() & other.bits());
            }
            #[must_use]
            #[inline(always)]
            pub const fn union(
                self,
                other: Self,
            ) -> Self {
                return self.inserted(other);
            }
            #[must_use]
            #[inline(always)]
            pub const fn difference(
                self,
                other: Self,
            ) -> Self {
                return self.removed(other);
            }
            #[must_use]
            #[inline(always)]
            pub const fn symmetric_difference(
                self,
                other: Self,
            ) -> Self {
                return self.toggled(other);
            }
            #[must_use]
            #[inline(always)]
            pub const fn complemented(self) -> Self {
                return Self::from_bits_retain(
                    !self.bits() & Self::all().bits(),
                );
            }
            #[inline(always)]
            pub const fn complement(&mut self) {
                *self = self.complemented();
            }
        }
        const impl ::core::ops::Shr<usize> for MyFlag {
            type Output = MyFlagValue;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self.into_value() >> rhs;
            }
        }
        const impl ::core::ops::Shl<usize> for MyFlag {
            type Output = MyFlagValue;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self.into_value() << rhs;
            }
        }
        const impl<T> ::core::ops::BitAnd<T> for MyFlag
        where
            T: [const] BitFriendBit + [const] ::core::marker::Destruct,
        {
            type Output = MyFlagValue;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let rhs = BitSeal::conv_my_flag(&rhs);
                return self
                    .into_value()
                    .intersection(MyFlagValue::from_bits_retain(rhs));
            }
        }
        const impl<T> ::core::ops::BitOr<T> for MyFlag
        where
            T: [const] BitFriendBit + [const] ::core::marker::Destruct,
        {
            type Output = MyFlagValue;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let rhs = BitSeal::conv_my_flag(&rhs);
                return self
                    .into_value()
                    .union(MyFlagValue::from_bits_retain(rhs));
            }
        }
        const impl<T> ::core::ops::BitXor<T> for MyFlag
        where
            T: [const] BitFriendBit + [const] ::core::marker::Destruct,
        {
            type Output = MyFlagValue;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let rhs = BitSeal::conv_my_flag(&rhs);
                return self
                    .into_value()
                    .symmetric_difference(MyFlagValue::from_bits_retain(rhs));
            }
        }
        const impl ::core::cmp::PartialEq<MyFlagValue> for MyFlag {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &MyFlagValue,
            ) -> bool {
                return self.raw() == rhs.raw();
            }
        }
        const impl ::core::cmp::PartialOrd<MyFlagValue> for MyFlag {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &MyFlagValue,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                return self.raw().partial_cmp(&rhs.raw());
            }
        }
        const impl ::core::ops::Not for MyFlag {
            type Output = MyFlagValue;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self.into_value().complemented();
            }
        }
        const impl BitSeal for MyFlag {
            #[inline(always)]
            fn conv_my_flag(&self) -> u128 {
                return MyFlag::raw(*self);
            }
        }
        const impl BitSeal for MyFlagValue {
            #[inline(always)]
            fn conv_my_flag(&self) -> u128 {
                return MyFlagValue::raw(*self);
            }
        }
        const impl BitFriendRel for MyFlag {}
        const impl BitFriendBit for MyFlag {}
        const impl BitFriendRel for MyFlagValue {}
        const impl BitFriendBit for MyFlagValue {}
        const trait BitSeal {
            fn conv_my_flag(&self) -> u128;
        }
        const trait BitFriendMake: [const] BitSeal {}
        const trait BitFriendMath: [const] BitSeal {}
        const trait BitFriendBit: [const] BitSeal {}
        const trait BitFriendRel: [const] BitSeal {}
        const impl<T> BitSeal for &T
        where
            T: [const] BitSeal,
        {
            #[inline(always)]
            fn conv_my_flag(&self) -> u128 {
                return BitSeal::conv_my_flag(&**self);
            }
        }
        const impl<T> BitSeal for &mut T
        where
            T: [const] BitSeal,
        {
            #[inline(always)]
            fn conv_my_flag(&self) -> u128 {
                return BitSeal::conv_my_flag(&**self);
            }
        }
        const impl<T> BitFriendMake for &T where T: [const] BitFriendMake {}
        const impl<T> BitFriendMake for &mut T where T: [const] BitFriendMake {}
        const impl<T> BitFriendMath for &T where T: [const] BitFriendMath {}
        const impl<T> BitFriendMath for &mut T where T: [const] BitFriendMath {}
        const impl<T> BitFriendBit for &T where T: [const] BitFriendBit {}
        const impl<T> BitFriendBit for &mut T where T: [const] BitFriendBit {}
        const impl<T> BitFriendRel for &T where T: [const] BitFriendRel {}
        const impl<T> BitFriendRel for &mut T where T: [const] BitFriendRel {}
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
}
type Subject = subject::MyFlag;
type Value = subject::MyFlagValue;
fn main() {
    let lhs = Subject::A;
    let rhs = Subject::B.into_value();
    {
        print!("{0:?}\n", (lhs & rhs) == (lhs & rhs));
    };
    {
        print!("{0:?}\n", rhs & lhs);
    };
    {
        print!("{0:?}\n", lhs & rhs);
    };
    for x in Subject::items() {
        {
            print!("{0}\n", x.name());
        };
    }
}
