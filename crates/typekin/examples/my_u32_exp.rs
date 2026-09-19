// AUTO-GENERATED VIA typekin-unexpand, ANY MANUAL MODIFICATIONS WILL BE LOST IF THE CODE IS RE-GENERATED
#![allow(clippy::needless_return)]
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_destruct)]
#![feature(derive_const)]
mod subject {
    #[repr(transparent)]
    #[derive(::core::fmt::Debug, ::core::marker::Copy)]
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
        const impl ::core::convert::TryInto<isize> for MyU32 {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<isize, Self::Error> {
                return MyU32::try_into_isize(self);
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
        const impl<T> ::core::ops::BitAndAssign<T> for MyU32
        where
            T: [const] FriendBit + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_u32(&rhs);
                *self = self._bitand(it);
            }
        }
        const impl<T> ::core::ops::AddAssign<T> for MyU32
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_u32(&rhs);
                *self = self._add(it);
            }
        }
        const impl<T> ::core::ops::SubAssign<T> for MyU32
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_u32(&rhs);
                *self = self._sub(it);
            }
        }
        const impl<T> ::core::ops::MulAssign<T> for MyU32
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_u32(&rhs);
                *self = self._mul(it);
            }
        }
        const impl<T> ::core::ops::DivAssign<T> for MyU32
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_u32(&rhs);
                *self = self._div(it);
            }
        }
        const impl<T> ::core::ops::RemAssign<T> for MyU32
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_u32(&rhs);
                *self = self._rem(it);
            }
        }
        const impl<T> ::core::ops::BitOrAssign<T> for MyU32
        where
            T: [const] FriendBit + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_u32(&rhs);
                *self = self._bitor(it);
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
        if !(::core::mem::size_of::<MyU32>() == ::core::mem::size_of::<u32>()) {
            {
                panic!("invalid memory layout: #ty(#el) != #el");
            };
        };
        const impl<T> ::core::ops::Add<T> for MyU32
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_u32(&rhs);
                return self._add(it);
            }
        }
        const impl<T> ::core::ops::Sub<T> for MyU32
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_u32(&rhs);
                return self._sub(it);
            }
        }
        const impl<T> ::core::ops::Mul<T> for MyU32
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_u32(&rhs);
                return self._mul(it);
            }
        }
        const impl<T> ::core::ops::Div<T> for MyU32
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_u32(&rhs);
                return self._div(it);
            }
        }
        const impl<T> ::core::ops::Rem<T> for MyU32
        where
            T: [const] FriendMath + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_u32(&rhs);
                return self._rem(it);
            }
        }
        const impl<T> ::core::ops::BitAnd<T> for MyU32
        where
            T: [const] FriendBit + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_u32(&rhs);
                return self._bitand(it);
            }
        }
        const impl<T> ::core::ops::BitOr<T> for MyU32
        where
            T: [const] FriendBit + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_u32(&rhs);
                return self._bitor(it);
            }
        }
        const impl<T> ::core::ops::BitXor<T> for MyU32
        where
            T: [const] FriendBit + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_u32(&rhs);
                return self._bitxor(it);
            }
        }
        const impl<T> ::core::ops::BitXorAssign<T> for MyU32
        where
            T: [const] FriendBit + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_u32(&rhs);
                *self = self._bitxor(it);
            }
        }
        const impl ::core::ops::Not for MyU32 {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        const impl Seal for MyU32 {
            #[inline(always)]
            fn conv_my_u32(&self) -> u32 {
                return Self::raw(*self);
            }
        }
        impl ::core::fmt::Binary for MyU32 {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl ::core::fmt::Octal for MyU32 {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl ::core::fmt::LowerHex for MyU32 {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl ::core::fmt::UpperHex for MyU32 {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        const impl Seal for MyFriend {
            #[inline(always)]
            fn conv_my_u32(&self) -> u32 {
                return MyFriend::my_conv(*self);
            }
        }
        const impl Seal for u32 {
            #[inline(always)]
            fn conv_my_u32(&self) -> u32 {
                let it: u32 = *self;
                return it;
            }
        }
        const impl FriendMake for MyFriend {}
        const impl FriendRel for MyFriend {}
        const impl FriendBit for MyFriend {}
        const impl FriendMath for MyFriend {}
        const impl FriendMake for MyU32 {}
        const impl FriendRel for MyU32 {}
        const impl FriendBit for MyU32 {}
        const impl FriendMath for MyU32 {}
        const impl FriendMake for u32 {}
        const impl FriendRel for u32 {}
        const impl FriendBit for u32 {}
        const impl FriendMath for u32 {}
        const impl<T> ::core::cmp::PartialEq<T> for MyU32
        where
            T: [const] FriendRel + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_my_u32(rhs);
                return lhs == rhs;
            }
        }
        const impl<T> ::core::cmp::PartialOrd<T> for MyU32
        where
            T: [const] FriendRel + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_my_u32(rhs);
                return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
            }
        }
        const trait Seal {
            fn conv_my_u32(&self) -> u32;
        }
        const impl<T> Seal for &T
        where
            T: [const] Seal,
        {
            #[inline(always)]
            fn conv_my_u32(&self) -> u32 {
                return Seal::conv_my_u32(&**self);
            }
        }
        const impl<T> Seal for &mut T
        where
            T: [const] Seal,
        {
            #[inline(always)]
            fn conv_my_u32(&self) -> u32 {
                return Seal::conv_my_u32(&**self);
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
        impl MyU32 {
            #[inline(always)]
            #[allow(private_bounds)]
            pub const fn of<T>(it: T) -> MyU32
            where
                T: [const] FriendMake + [const] ::core::marker::Destruct,
            {
                let this = Seal::conv_my_u32(&it);
                return Self::_unchecked(this);
            }
            #[inline(always)]
            pub const fn make(it: u32) -> MyU32 {
                return MyU32::of(it);
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
                let it = Self::raw(self);
                return it as usize;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u32(self) -> u32 {
                let it = Self::raw(self);
                return it as u32;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u64(self) -> u64 {
                let it = Self::raw(self);
                return it as u64;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u128(self) -> u128 {
                let it = Self::raw(self);
                return it as u128;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_i64(self) -> i64 {
                let it = Self::raw(self);
                return it as i64;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_i128(self) -> i128 {
                let it = Self::raw(self);
                return it as i128;
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
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u8(self) -> Result<u8, ()> {
                let r = Self::raw(self);
                let t = r as u8;
                let s = t as u32;
                return if s == r && true { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u16(self) -> Result<u16, ()> {
                let r = Self::raw(self);
                let t = r as u16;
                let s = t as u32;
                return if s == r && true { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i8(self) -> Result<i8, ()> {
                let r = Self::raw(self);
                let t = r as i8;
                let s = t as u32;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i16(self) -> Result<i16, ()> {
                let r = Self::raw(self);
                let t = r as i16;
                let s = t as u32;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i32(self) -> Result<i32, ()> {
                let r = Self::raw(self);
                let t = r as i32;
                let s = t as u32;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            pub(self) const fn _add(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs + rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _sub(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs - rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _mul(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs * rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _div(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs / rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _rem(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs % rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _bitxor(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs ^ rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _bitand(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs & rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _bitor(
                &self,
                rhs: u32,
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
                rhs: u32,
            ) -> bool {
                let lhs = Self::raw(self);
                return lhs == rhs;
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) fn _cmp(
                self,
                rhs: u32,
            ) -> ::core::cmp::Ordering {
                let lhs = Self::raw(self);
                return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs)
                    .unwrap();
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
        #[allow(clippy::unnecessary_cast)]
        impl MyU32 {
            #[must_use]
            #[inline(always)]
            pub const fn lo16(self) -> u16 {
                return self.word0();
            }
            #[must_use]
            #[inline(always)]
            pub const fn hi16(self) -> u16 {
                return self.word1();
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte0(self) -> u8 {
                return ((Self::raw(self) >> (8 * 0usize)) & (0xFF as u32))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte1(self) -> u8 {
                return ((Self::raw(self) >> (8 * 1usize)) & (0xFF as u32))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte2(self) -> u8 {
                return ((Self::raw(self) >> (8 * 2usize)) & (0xFF as u32))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte3(self) -> u8 {
                return ((Self::raw(self) >> (8 * 3usize)) & (0xFF as u32))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word0(self) -> u16 {
                return ((Self::raw(self) >> (16 * 0usize)) & (0xFFFF as u32))
                    as u16;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word1(self) -> u16 {
                return ((Self::raw(self) >> (16 * 1usize)) & (0xFFFF as u32))
                    as u16;
            }
            #[must_use]
            #[inline(always)]
            pub const fn dword0(self) -> u32 {
                return ((Self::raw(self) >> (32 * 0usize))
                    & (0xFFFFFFFF as u32)) as u32;
            }
        }
    };
    #[repr(transparent)]
    #[derive(::core::marker::Copy)]
    #[derive_const(::core::clone::Clone)]
    pub struct MyFriend(u32);
    impl MyFriend {
        const fn my_conv(self) -> u32 {
            return self.0 * 2;
        }
    }
    impl std::fmt::Display for MyU32 {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            f.write_fmt(format_args!("MyU16({0})", self.raw()))
        }
    }
}
type Subject = subject::MyU32;
fn main() {
    let lhs = 0b1101u32;
    let rhs = 0b0110u32;
    let lhs = Subject::of(lhs);
    let rhs = Subject::of(rhs);
    {
        print!("{0:?}\n", lhs + rhs);
    };
}
