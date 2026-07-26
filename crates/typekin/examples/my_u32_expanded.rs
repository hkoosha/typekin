// AUTO-GENERATED VIA typekin-unexpand, DO NOT MODIFY
#![allow(clippy::needless_return)]
#![allow(dead_code)]
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_destruct)]
extern crate std;
use typekin_testing::demo_u128;
mod subject {
    use std::fmt::Formatter;
    pub struct MyExample(u32);
    const _: () = {
        const impl Into<usize> for MyExample {
            #[inline(always)]
            fn into(self) -> usize {
                return MyExample::into_usize(self);
            }
        }
        const impl Into<u32> for MyExample {
            #[inline(always)]
            fn into(self) -> u32 {
                return MyExample::into_u32(self);
            }
        }
        const impl Into<u64> for MyExample {
            #[inline(always)]
            fn into(self) -> u64 {
                return MyExample::into_u64(self);
            }
        }
        const impl Into<u128> for MyExample {
            #[inline(always)]
            fn into(self) -> u128 {
                return MyExample::into_u128(self);
            }
        }
        const impl Into<i64> for MyExample {
            #[inline(always)]
            fn into(self) -> i64 {
                return MyExample::into_i64(self);
            }
        }
        const impl Into<i128> for MyExample {
            #[inline(always)]
            fn into(self) -> i128 {
                return MyExample::into_i128(self);
            }
        }
        const impl TryInto<u8> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u8, Self::Error> {
                return MyExample::try_into_u8(self);
            }
        }
        const impl TryInto<u16> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u16, Self::Error> {
                return MyExample::try_into_u16(self);
            }
        }
        const impl TryInto<i8> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i8, Self::Error> {
                return MyExample::try_into_i8(self);
            }
        }
        const impl TryInto<i16> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i16, Self::Error> {
                return MyExample::try_into_i16(self);
            }
        }
        const impl TryInto<i32> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i32, Self::Error> {
                return MyExample::try_into_i32(self);
            }
        }
        const impl core::ops::Shr<usize> for MyExample {
            type Output = MyExample;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        const impl core::ops::Shl<usize> for MyExample {
            type Output = MyExample;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        const impl core::ops::ShrAssign<usize> for MyExample {
            #[inline(always)]
            fn shr_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shr(other);
            }
        }
        const impl core::ops::ShlAssign<usize> for MyExample {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shl(other);
            }
        }
        const impl core::ops::BitAndAssign<MyExample> for MyExample {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                other: MyExample,
            ) {
                *self = self._and(other.0);
            }
        }
        const impl core::ops::AddAssign<MyExample> for MyExample {
            #[inline(always)]
            fn add_assign(
                &mut self,
                other: MyExample,
            ) {
                *self = self._add(other.0);
            }
        }
        const impl core::ops::SubAssign<MyExample> for MyExample {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                other: MyExample,
            ) {
                *self = self._sub(other.0);
            }
        }
        const impl core::ops::MulAssign<MyExample> for MyExample {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                other: MyExample,
            ) {
                *self = self._mul(other.0);
            }
        }
        const impl core::ops::DivAssign<MyExample> for MyExample {
            #[inline(always)]
            fn div_assign(
                &mut self,
                other: MyExample,
            ) {
                *self = self._div(other.0);
            }
        }
        const impl core::ops::RemAssign<MyExample> for MyExample {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                other: MyExample,
            ) {
                *self = self._rem(other.0);
            }
        }
        const impl core::ops::BitOrAssign<MyExample> for MyExample {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                other: MyExample,
            ) {
                *self = self._or(other.0);
            }
        }
        impl core::fmt::Debug for MyExample {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                f.write_fmt(format_args!("MyExample({0})", self.0))
            }
        }
        impl core::marker::Copy for MyExample {}
        const impl Clone for MyExample {
            #[inline(always)]
            fn clone(&self) -> Self {
                Self(Clone::clone(&self.0))
            }
        }
        const impl core::cmp::Eq for MyExample {}
        const impl core::cmp::Ord for MyExample {
            #[inline(always)]
            fn cmp(
                &self,
                other: &Self,
            ) -> core::cmp::Ordering {
                return self.partial_cmp(other).unwrap();
            }
        }
        if !(size_of::<MyExample>() == size_of::<u32>()) {
            panic!("invalid memory layout: #ty(#el) != #el");
        };
        const impl<T> core::cmp::PartialEq<T> for MyExample
        where
            T: [const] FriendMathRel + [const] core::marker::Destruct,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let that = Seal::conv_my_example(rhs);
                return self.raw() == that;
            }
        }
        const impl<T> core::cmp::PartialOrd<T> for MyExample
        where
            T: [const] PartialEq<MyExample>
                + [const] FriendMathRel
                + [const] core::marker::Destruct,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> Option<core::cmp::Ordering> {
                let that = Seal::conv_my_example(rhs);
                return self.raw().partial_cmp(&that);
            }
        }
        const impl<T> core::ops::Add<T> for MyExample
        where
            T: [const] FriendMathOps + [const] core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_example(&rhs);
                return self._add(that);
            }
        }
        const impl<T> core::ops::Sub<T> for MyExample
        where
            T: [const] FriendMathOps + [const] core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_example(&rhs);
                return self._sub(that);
            }
        }
        const impl<T> core::ops::Mul<T> for MyExample
        where
            T: [const] FriendMathOps + [const] core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_example(&rhs);
                return self._mul(that);
            }
        }
        const impl<T> core::ops::Div<T> for MyExample
        where
            T: [const] FriendMathOps + [const] core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_example(&rhs);
                return self._div(that);
            }
        }
        const impl<T> core::ops::Rem<T> for MyExample
        where
            T: [const] FriendMathOps + [const] core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_example(&rhs);
                return self._rem(that);
            }
        }
        const impl<T> core::ops::BitAnd<T> for MyExample
        where
            T: [const] FriendMathBit + [const] core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_example(&rhs);
                return self._and(that);
            }
        }
        const impl<T> core::ops::BitOr<T> for MyExample
        where
            T: [const] FriendMathBit + [const] core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_example(&rhs);
                return self._or(that);
            }
        }
        const impl<T> core::ops::BitXor<T> for MyExample
        where
            T: [const] FriendMathBit + [const] core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let that = Seal::conv_my_example(&rhs);
                return self._xor(that);
            }
        }
        const impl core::ops::Not for MyExample {
            type Output = MyExample;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        const trait Seal {
            #[must_use]
            fn conv_my_example(&self) -> u32;
        }
        const trait FriendMake: [const] Seal {}
        const trait FriendMathOps: [const] Seal {}
        const trait FriendMathBit: [const] Seal {}
        const trait FriendMathRel: [const] Seal {}
        const impl Seal for MyExample {
            #[inline(always)]
            fn conv_my_example(&self) -> u32 {
                return Self::raw(*self);
            }
        }
        const impl FriendMake for MyExample {}
        const impl FriendMathOps for MyExample {}
        const impl FriendMathBit for MyExample {}
        const impl FriendMathRel for MyExample {}
        const impl FriendMathOps for u32 {}
        const impl FriendMathBit for u32 {}
        const impl FriendMathRel for u32 {}
        const impl FriendMake for u32 {}
        const impl Seal for u32 {
            #[inline(always)]
            fn conv_my_example(&self) -> u32 {
                let it: u32 = *self;
                return it;
            }
        }
        impl MyExample {
            #[inline(always)]
            #[allow(private_bounds)]
            pub const fn of<T, B>(it: B) -> MyExample
            where
                T: [const] FriendMake,
                B: [const] core::borrow::Borrow<T>
                    + [const] core::marker::Destruct,
            {
                let this = Seal::conv_my_example(it.borrow());
                return Self::_make(this);
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
                return self.raw() as usize;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u32(self) -> u32 {
                return self.raw() as u32;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u64(self) -> u64 {
                return self.raw() as u64;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u128(self) -> u128 {
                return self.raw() as u128;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_i64(self) -> i64 {
                return self.raw() as i64;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_i128(self) -> i128 {
                return self.raw() as i128;
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(self) -> Result<usize, ()> {
                return Ok(self.raw() as usize);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u32(self) -> Result<u32, ()> {
                return Ok(self.raw() as u32);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u64(self) -> Result<u64, ()> {
                return Ok(self.raw() as u64);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u128(self) -> Result<u128, ()> {
                return Ok(self.raw() as u128);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i64(self) -> Result<i64, ()> {
                return Ok(self.raw() as i64);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i128(self) -> Result<i128, ()> {
                return Ok(self.raw() as i128);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) -> Result<isize, ()> {
                let r = self.raw();
                let t = r as isize;
                let s = t as u32;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u8(self) -> Result<u8, ()> {
                let r = self.raw();
                let t = r as u8;
                let s = t as u32;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u16(self) -> Result<u16, ()> {
                let r = self.raw();
                let t = r as u16;
                let s = t as u32;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i8(self) -> Result<i8, ()> {
                let r = self.raw();
                let t = r as i8;
                let s = t as u32;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i16(self) -> Result<i16, ()> {
                let r = self.raw();
                let t = r as i16;
                let s = t as u32;
                return if s == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i32(self) -> Result<i32, ()> {
                let r = self.raw();
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
                return Self::_make(result);
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
                return Self::_make(result);
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
                return Self::_make(result);
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
                return Self::_make(result);
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
                return Self::_make(result);
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
                return Self::_make(result);
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
                return Self::_make(result);
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
                return Self::_make(result);
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
                return Self::_make(result);
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
                return Self::_make(result);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _not(self) -> Self {
                let this = Self::raw(self);
                let result = !this;
                return Self::_make(result);
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
            ) -> core::cmp::Ordering {
                let this = Self::raw(self);
                let result = core::cmp::PartialOrd::partial_cmp(&this, &it);
                return result.unwrap();
            }
            #[inline(always)]
            pub const fn try_make(it: u32) -> Result<Self, u32> {
                return Ok(Self::_make(it));
            }
            #[must_use]
            #[inline(always)]
            pub(self) const fn _make(it: u32) -> Self {
                return Self(it);
            }
        }
    };
    impl std::fmt::Display for MyExample {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            f.write_fmt(format_args!("MyExample({0})", self.raw()))
        }
    }
}
type Subject = subject::MyExample;
fn main() {
    let lhs0 = Subject::of(321);
    let demo = demo_u128(lhs0, 123, |it| it.raw() as u128);
    {
        print!("{0}\n", demo.print());
    };
}
