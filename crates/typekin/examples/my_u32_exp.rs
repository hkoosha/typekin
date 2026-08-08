// AUTO-GENERATED VIA typekin-unexpand, DO NOT MODIFY
#![allow(clippy::needless_return)]
#![allow(dead_code)]
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_destruct)]
#![feature(derive_const)]
extern crate std;
use typekin_testing::demo_u128;
mod subject {
    use std::fmt::Formatter;
    #[repr(transparent)]
    #[derive(:: core :: fmt :: Debug, :: core :: marker :: Copy)]
    #[derive_const(::core::clone::Clone)]
    pub struct MyU32(u32);
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    const _: () = {
        const impl ::core::convert::Into<usize> for MyU32 {
            #[inline(always)]
            fn into(self) -> usize {
                return MyU32::into_usize(self);
            }
        }
        const impl ::core::convert::Into<u32> for MyU32 {
            #[inline(always)]
            fn into(self) -> u32 {
                return MyU32::into_u32(self);
            }
        }
        const impl ::core::convert::Into<u64> for MyU32 {
            #[inline(always)]
            fn into(self) -> u64 {
                return MyU32::into_u64(self);
            }
        }
        const impl ::core::convert::Into<u128> for MyU32 {
            #[inline(always)]
            fn into(self) -> u128 {
                return MyU32::into_u128(self);
            }
        }
        const impl ::core::convert::Into<i64> for MyU32 {
            #[inline(always)]
            fn into(self) -> i64 {
                return MyU32::into_i64(self);
            }
        }
        const impl ::core::convert::Into<i128> for MyU32 {
            #[inline(always)]
            fn into(self) -> i128 {
                return MyU32::into_i128(self);
            }
        }
        const impl ::core::convert::TryInto<u8> for MyU32 {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u8, Self::Error> {
                return MyU32::try_into_u8(self);
            }
        }
        const impl ::core::convert::TryInto<u16> for MyU32 {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u16, Self::Error> {
                return MyU32::try_into_u16(self);
            }
        }
        const impl ::core::convert::TryInto<i8> for MyU32 {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i8, Self::Error> {
                return MyU32::try_into_i8(self);
            }
        }
        const impl ::core::convert::TryInto<i16> for MyU32 {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i16, Self::Error> {
                return MyU32::try_into_i16(self);
            }
        }
        const impl ::core::convert::TryInto<i32> for MyU32 {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i32, Self::Error> {
                return MyU32::try_into_i32(self);
            }
        }
        const impl ::core::ops::Shr<usize> for MyU32 {
            type Output = Self;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        const impl ::core::ops::Shl<usize> for MyU32 {
            type Output = Self;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        const impl ::core::ops::ShrAssign<usize> for MyU32 {
            #[inline(always)]
            fn shr_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shr(other);
            }
        }
        const impl ::core::ops::ShlAssign<usize> for MyU32 {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shl(other);
            }
        }
        const impl ::core::ops::BitAndAssign<MyU32> for MyU32 {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                other: MyU32,
            ) {
                *self = self._and(other.0);
            }
        }
        const impl ::core::ops::AddAssign<MyU32> for MyU32 {
            #[inline(always)]
            fn add_assign(
                &mut self,
                other: MyU32,
            ) {
                *self = self._add(other.0);
            }
        }
        const impl ::core::ops::SubAssign<MyU32> for MyU32 {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                other: MyU32,
            ) {
                *self = self._sub(other.0);
            }
        }
        const impl ::core::ops::MulAssign<MyU32> for MyU32 {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                other: MyU32,
            ) {
                *self = self._mul(other.0);
            }
        }
        const impl ::core::ops::DivAssign<MyU32> for MyU32 {
            #[inline(always)]
            fn div_assign(
                &mut self,
                other: MyU32,
            ) {
                *self = self._div(other.0);
            }
        }
        const impl ::core::ops::RemAssign<MyU32> for MyU32 {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                other: MyU32,
            ) {
                *self = self._rem(other.0);
            }
        }
        const impl ::core::ops::BitOrAssign<MyU32> for MyU32 {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                other: MyU32,
            ) {
                *self = self._or(other.0);
            }
        }
        const impl ::core::cmp::Eq for MyU32 {}
        const impl ::core::cmp::Ord for MyU32 {
            #[inline(always)]
            fn cmp(
                &self,
                other: &Self,
            ) -> ::core::cmp::Ordering {
                return self.partial_cmp(other).unwrap();
            }
        }
        if !(size_of::<MyU32>() == size_of::<u32>()) {
            {
                panic!("invalid memory layout: #ty(#el) != #el");
            };
        };
        const impl<T> ::core::cmp::PartialEq<T> for MyU32
        where
            T: [const] FriendMathRel + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let that = Seal::conv_my_u32(rhs);
                return Self::raw(*self) == that;
            }
        }
        const impl<T> ::core::cmp::PartialOrd<T> for MyU32
        where
            T: [const] PartialEq<MyU32>
                + [const] FriendMathRel
                + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let that = Seal::conv_my_u32(rhs);
                return Self::raw(*self).partial_cmp(&that);
            }
        }
        const impl<T> ::core::ops::Add<T> for MyU32
        where
            T: [const] FriendMathOps + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_u32(&rhs);
                return self._add(that);
            }
        }
        const impl<T> ::core::ops::Sub<T> for MyU32
        where
            T: [const] FriendMathOps + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_u32(&rhs);
                return self._sub(that);
            }
        }
        const impl<T> ::core::ops::Mul<T> for MyU32
        where
            T: [const] FriendMathOps + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_u32(&rhs);
                return self._mul(that);
            }
        }
        const impl<T> ::core::ops::Div<T> for MyU32
        where
            T: [const] FriendMathOps + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_u32(&rhs);
                return self._div(that);
            }
        }
        const impl<T> ::core::ops::Rem<T> for MyU32
        where
            T: [const] FriendMathOps + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_u32(&rhs);
                return self._rem(that);
            }
        }
        const impl<T> ::core::ops::BitAnd<T> for MyU32
        where
            T: [const] FriendMathBit + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_u32(&rhs);
                return self._and(that);
            }
        }
        const impl<T> ::core::ops::BitOr<T> for MyU32
        where
            T: [const] FriendMathBit + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_u32(&rhs);
                return self._or(that);
            }
        }
        const impl<T> ::core::ops::BitXor<T> for MyU32
        where
            T: [const] FriendMathBit + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_u32(&rhs);
                return self._xor(that);
            }
        }
        const impl ::core::ops::Not for MyU32 {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        const trait Seal {
            #[must_use]
            fn conv_my_u32(&self) -> u32;
        }
        const trait FriendMake: [const] Seal {}
        const trait FriendMathOps: [const] Seal {}
        const trait FriendMathBit: [const] Seal {}
        const trait FriendMathRel: [const] Seal {}
        const impl Seal for MyU32 {
            #[inline(always)]
            fn conv_my_u32(&self) -> u32 {
                return Self::raw(*self);
            }
        }
        const impl FriendMake for MyU32 {}
        const impl FriendMathOps for MyU32 {}
        const impl FriendMathBit for MyU32 {}
        const impl FriendMathRel for MyU32 {}
        const impl FriendMathOps for u32 {}
        const impl FriendMathBit for u32 {}
        const impl FriendMathRel for u32 {}
        const impl FriendMake for u32 {}
        const impl Seal for u32 {
            #[inline(always)]
            fn conv_my_u32(&self) -> u32 {
                let it: u32 = *self;
                return it;
            }
        }
        impl ::core::fmt::Binary for MyU32 {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl ::core::fmt::Octal for MyU32 {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl ::core::fmt::LowerHex for MyU32 {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl ::core::fmt::UpperHex for MyU32 {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        impl MyU32 {
            #[inline(always)]
            #[allow(private_bounds)]
            pub const fn of<T, B>(it: B) -> MyU32
            where
                T: [const] FriendMake,
                B: [const] ::core::borrow::Borrow<T>
                    + [const] ::core::marker::Destruct,
            {
                let this = Seal::conv_my_u32(it.borrow());
                return Self::_unchecked(this);
            }
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> u32 {
                return self.0;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_usize(self) -> usize {
                return Self::raw(self) as usize;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u32(self) -> u32 {
                return Self::raw(self) as u32;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u64(self) -> u64 {
                return Self::raw(self) as u64;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u128(self) -> u128 {
                return Self::raw(self) as u128;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_i64(self) -> i64 {
                return Self::raw(self) as i64;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_i128(self) -> i128 {
                return Self::raw(self) as i128;
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(self) -> Result<usize, ()> {
                return Ok(Self::raw(self) as usize);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u32(self) -> Result<u32, ()> {
                return Ok(Self::raw(self) as u32);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u64(self) -> Result<u64, ()> {
                return Ok(Self::raw(self) as u64);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u128(self) -> Result<u128, ()> {
                return Ok(Self::raw(self) as u128);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i64(self) -> Result<i64, ()> {
                return Ok(Self::raw(self) as i64);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i128(self) -> Result<i128, ()> {
                return Ok(Self::raw(self) as i128);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) -> Result<isize, ()> {
                let r = Self::raw(self);
                let t = r as isize;
                let s = t as u32;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u8(self) -> Result<u8, ()> {
                let r = Self::raw(self);
                let t = r as u8;
                let s = t as u32;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u16(self) -> Result<u16, ()> {
                let r = Self::raw(self);
                let t = r as u16;
                let s = t as u32;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i8(self) -> Result<i8, ()> {
                let r = Self::raw(self);
                let t = r as i8;
                let s = t as u32;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i16(self) -> Result<i16, ()> {
                let r = Self::raw(self);
                let t = r as i16;
                let s = t as u32;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i32(self) -> Result<i32, ()> {
                let r = Self::raw(self);
                let t = r as i32;
                let s = t as u32;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _add(
                self,
                it: u32,
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
                it: u32,
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
                it: u32,
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
                it: u32,
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
                it: u32,
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
                it: u32,
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
                it: u32,
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
                it: u32,
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
                it: u32,
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
                it: u32,
            ) -> ::core::cmp::Ordering {
                let this = Self::raw(self);
                let result = ::core::cmp::PartialOrd::partial_cmp(&this, &it);
                return result.unwrap();
            }
            #[inline(always)]
            pub const fn try_make(it: u32) -> Result<Self, u32> {
                return Ok(Self::_unchecked(it));
            }
            #[must_use]
            #[inline(always)]
            pub(self) const fn _unchecked(it: u32) -> Self {
                return Self(it);
            }
        }
    };
    impl std::fmt::Display for MyU32 {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            f.write_fmt(format_args!("MyU32({0})", self.raw()))
        }
    }
}
type Subject = subject::MyU32;
fn main() {
    let lhs = 0b1101u32;
    let rhs = 0b0110u32;
    let demo = demo_u128(Subject::of(lhs), rhs, |it| it.raw() as u128);
    {
        print!("{0}\n", demo.print());
    };
}
