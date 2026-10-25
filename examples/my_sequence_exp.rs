use core::ops::{
    Add,
    Sub,
};

#[repr(transparent)]
#[derive(Copy, Clone)]
struct SeqPoint(u32);

#[allow(dead_code)]
#[allow(unused_qualifications)]
#[allow(clippy::unnecessary_cast)]
const _: () = {
    trait Seal {
        fn conv_seq_point(self) -> u32;
    }
    trait Make: Seal {
        fn make(self) -> SeqPoint;
    }
    trait Math: Seal {}
    trait Bit: Seal {}
    trait Rel: Seal {}
    unsafe trait UnsafeTrust: Seal {}
    impl Seal for SeqPoint {
        #[inline(always)]
        fn conv_seq_point(self) -> u32 {
            return Self::raw(self);
        }
    }
    impl Make for SeqPoint {
        #[inline(always)]
        fn make(self) -> SeqPoint {
            let raw = <Self as Seal>::conv_seq_point(self);
            return SeqPoint::_unchecked(raw);
        }
    }
    impl Rel for SeqPoint {}
    impl Seal for i16 {
        #[inline(always)]
        fn conv_seq_point(self) -> u32 {
            return self as u32;
        }
    }
    impl Make for i16 {
        #[inline(always)]
        fn make(self) -> SeqPoint {
            let raw = <Self as Seal>::conv_seq_point(self);
            return SeqPoint::_unchecked(raw);
        }
    }
    impl Seal for u16 {
        #[inline(always)]
        fn conv_seq_point(self) -> u32 {
            return self as u32;
        }
    }
    impl Make for u16 {
        #[inline(always)]
        fn make(self) -> SeqPoint {
            let raw = <Self as Seal>::conv_seq_point(self);
            return SeqPoint::_unchecked(raw);
        }
    }
    impl Seal for u32 {
        #[inline(always)]
        fn conv_seq_point(self) -> u32 {
            return self as u32;
        }
    }
    impl Make for u32 {
        #[inline(always)]
        fn make(self) -> SeqPoint {
            let raw = <Self as Seal>::conv_seq_point(self);
            return SeqPoint::_unchecked(raw);
        }
    }
    impl Seal for u8 {
        #[inline(always)]
        fn conv_seq_point(self) -> u32 {
            return self as u32;
        }
    }
    impl Make for u8 {
        #[inline(always)]
        fn make(self) -> SeqPoint {
            let raw = <Self as Seal>::conv_seq_point(self);
            return SeqPoint::_unchecked(raw);
        }
    }
    impl ::core::convert::AsRef<u32> for SeqPoint {
        #[inline(always)]
        fn as_ref(&self) -> &u32 {
            return &self.0;
        }
    }
    impl ::core::str::FromStr for SeqPoint {
        type Err = ();
        #[inline(always)]
        fn from_str(source: &str) -> ::core::result::Result<Self, Self::Err> {
            let value = match <u32 as ::core::str::FromStr>::from_str(source) {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(_) => {
                    return ::core::result::Result::Err(());
                }
            };
            return if true {
                ::core::result::Result::Ok(Self(value))
            }
            else {
                ::core::result::Result::Err(())
            };
        }
    }
    impl ::core::convert::Into<usize> for SeqPoint {
        #[inline(always)]
        fn into(self) -> usize {
            return SeqPoint::into_usize(self);
        }
    }
    impl ::core::convert::Into<u32> for SeqPoint {
        #[inline(always)]
        fn into(self) -> u32 {
            return SeqPoint::into_u32(self);
        }
    }
    impl ::core::convert::Into<u64> for SeqPoint {
        #[inline(always)]
        fn into(self) -> u64 {
            return SeqPoint::into_u64(self);
        }
    }
    impl ::core::convert::Into<u128> for SeqPoint {
        #[inline(always)]
        fn into(self) -> u128 {
            return SeqPoint::into_u128(self);
        }
    }
    impl ::core::convert::Into<i64> for SeqPoint {
        #[inline(always)]
        fn into(self) -> i64 {
            return SeqPoint::into_i64(self);
        }
    }
    impl ::core::convert::Into<i128> for SeqPoint {
        #[inline(always)]
        fn into(self) -> i128 {
            return SeqPoint::into_i128(self);
        }
    }
    impl ::core::convert::TryInto<isize> for SeqPoint {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<isize, Self::Error> {
            return SeqPoint::try_into_isize(self);
        }
    }
    impl ::core::convert::TryInto<u8> for SeqPoint {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u8, Self::Error> {
            return SeqPoint::try_into_u8(self);
        }
    }
    impl ::core::convert::TryInto<u16> for SeqPoint {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u16, Self::Error> {
            return SeqPoint::try_into_u16(self);
        }
    }
    impl ::core::convert::TryInto<i8> for SeqPoint {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i8, Self::Error> {
            return SeqPoint::try_into_i8(self);
        }
    }
    impl ::core::convert::TryInto<i16> for SeqPoint {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i16, Self::Error> {
            return SeqPoint::try_into_i16(self);
        }
    }
    impl ::core::convert::TryInto<i32> for SeqPoint {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i32, Self::Error> {
            return SeqPoint::try_into_i32(self);
        }
    }
    impl ::core::fmt::Debug for SeqPoint {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            return f.debug_tuple("SeqPoint").field(&self.0).finish();
        }
    }
    impl ::core::hash::Hash for SeqPoint {
        #[inline(always)]
        fn hash<H>(
            &self,
            state: &mut H,
        ) where
            H: ::core::hash::Hasher,
        {
            return ::core::hash::Hash::hash(&self.0, state);
        }
    }
    impl ::core::cmp::Eq for SeqPoint {}
    if !(::core::mem::size_of::<SeqPoint>() == ::core::mem::size_of::<u32>()) {
        panic!("invalid memory layout, mismatching sizes: #ty(#el) != #el");
    };
    if !(::core::mem::align_of::<SeqPoint>() == ::core::mem::align_of::<u32>())
    {
        panic!("invalid memory layout, mismatching alignment: #ty(#el) != #el");
    };
    impl ::core::ops::Sub for SeqPoint {
        type Output = SeqOffset;
        #[inline(always)]
        fn sub(
            self,
            rhs: Self,
        ) -> Self::Output {
            return SeqPoint::into_offset(self._sub(rhs.0));
        }
    }
    impl<T> ::core::ops::Sub<T> for SeqPoint
    where
        T: Math + Seal,
    {
        type Output = SeqOffset;
        #[inline(always)]
        fn sub(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_seq_point(rhs);
            return SeqPoint::into_offset(self._sub(it));
        }
    }
    impl ::core::fmt::Binary for SeqPoint {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::Binary::fmt(&raw, f);
        }
    }
    impl ::core::fmt::Octal for SeqPoint {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::Octal::fmt(&raw, f);
        }
    }
    impl ::core::fmt::LowerHex for SeqPoint {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::LowerHex::fmt(&raw, f);
        }
    }
    impl ::core::fmt::UpperHex for SeqPoint {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::UpperHex::fmt(&raw, f);
        }
    }
    impl<T> ::core::cmp::PartialEq<T> for SeqPoint
    where
        T: Rel + Seal + ::core::marker::Copy,
    {
        #[inline(always)]
        fn eq(
            &self,
            rhs: &T,
        ) -> bool {
            let lhs = Self::raw(*self);
            let rhs = Seal::conv_seq_point(*rhs);
            return lhs == rhs;
        }
    }
    impl SeqPoint {
        #[inline(always)]
        #[allow(private_bounds)]
        pub fn of<T>(it: T) -> SeqPoint
        where
            T: Make,
        {
            return <T as Make>::make(it);
        }
        #[inline(always)]
        pub fn make(it: u32) -> SeqPoint {
            return SeqPoint::of(it);
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
        pub fn try_map<F>(
            self,
            function: F,
        ) -> ::core::result::Result<Self, u32>
        where
            F: ::core::ops::FnOnce(u32) -> u32,
        {
            let value = function(Self::raw(self));
            return if true {
                ::core::result::Result::Ok(Self(value))
            }
            else {
                ::core::result::Result::Err(value)
            };
        }
        #[inline(always)]
        #[track_caller]
        pub fn map<F>(
            self,
            function: F,
        ) -> Self
        where
            F: ::core::ops::FnOnce(u32) -> u32,
        {
            return match self.try_map(function) {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(_) => {
                    ::core::panic!("integral mapping produced an invalid value")
                }
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_usize(self) -> ::core::result::Result<usize, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as usize);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u32(self) -> ::core::result::Result<u32, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as u32);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u64(self) -> ::core::result::Result<u64, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as u64);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u128(self) -> ::core::result::Result<u128, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as u128);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i64(self) -> ::core::result::Result<i64, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as i64);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i128(self) -> ::core::result::Result<i128, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as i128);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_isize(self) -> ::core::result::Result<isize, ()> {
            let r = Self::raw(self);
            let t = r as isize;
            let s = t as u32;
            return if s == r && t >= 0 {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u8(self) -> ::core::result::Result<u8, ()> {
            let r = Self::raw(self);
            let t = r as u8;
            let s = t as u32;
            return if s == r && true {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u16(self) -> ::core::result::Result<u16, ()> {
            let r = Self::raw(self);
            let t = r as u16;
            let s = t as u32;
            return if s == r && true {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i8(self) -> ::core::result::Result<i8, ()> {
            let r = Self::raw(self);
            let t = r as i8;
            let s = t as u32;
            return if s == r && t >= 0 {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i16(self) -> ::core::result::Result<i16, ()> {
            let r = Self::raw(self);
            let t = r as i16;
            let s = t as u32;
            return if s == r && t >= 0 {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i32(self) -> ::core::result::Result<i32, ()> {
            let r = Self::raw(self);
            let t = r as i32;
            let s = t as u32;
            return if s == r && t >= 0 {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        pub(self) const fn _add(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_add(rhs);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _sub(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_sub(rhs);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _mul(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_mul(rhs);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _div(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_div(rhs);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _rem(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_rem(rhs);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _bitxor(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs ^ rhs;
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _bitand(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs & rhs;
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _bitor(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs | rhs;
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _shr(
            &self,
            count: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_shr(count as u32);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _shl(
            &self,
            count: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_shl(count as u32);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _not(&self) -> Self {
            let lhs = Self::raw(*self);
            let it = !lhs;
            return Self::_unchecked(it);
        }
        #[must_use]
        #[inline(always)]
        pub fn precedes(
            self,
            other: Self,
        ) -> bool {
            let lhs = Self::raw(self) as u32;
            let rhs = Self::raw(other) as u32;
            let difference = rhs.wrapping_sub(lhs);
            let half = (1 as u32) << (u32::BITS - 1);
            return difference != 0 && difference < half;
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
            return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs).unwrap();
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
    impl SeqPoint {}
};

#[repr(transparent)]
#[derive(Copy, Clone)]
struct SeqOffset(i32);

#[allow(dead_code)]
#[allow(unused_qualifications)]
#[allow(clippy::unnecessary_cast)]
const _: () = {
    trait Seal {
        fn conv_seq_offset(self) -> i32;
    }
    trait Make: Seal {
        fn make(self) -> SeqOffset;
    }
    trait Math: Seal {}
    trait Bit: Seal {}
    trait Rel: Seal {}
    unsafe trait UnsafeTrust: Seal {}
    impl Seal for SeqOffset {
        #[inline(always)]
        fn conv_seq_offset(self) -> i32 {
            return Self::raw(self);
        }
    }
    impl Make for SeqOffset {
        #[inline(always)]
        fn make(self) -> SeqOffset {
            let raw = <Self as Seal>::conv_seq_offset(self);
            return SeqOffset::_unchecked(raw);
        }
    }
    impl Rel for SeqOffset {}
    impl Seal for i32 {
        #[inline(always)]
        fn conv_seq_offset(self) -> i32 {
            return self as i32;
        }
    }
    impl Make for i32 {
        #[inline(always)]
        fn make(self) -> SeqOffset {
            let raw = <Self as Seal>::conv_seq_offset(self);
            return SeqOffset::_unchecked(raw);
        }
    }
    impl Seal for i8 {
        #[inline(always)]
        fn conv_seq_offset(self) -> i32 {
            return self as i32;
        }
    }
    impl Make for i8 {
        #[inline(always)]
        fn make(self) -> SeqOffset {
            let raw = <Self as Seal>::conv_seq_offset(self);
            return SeqOffset::_unchecked(raw);
        }
    }
    impl Seal for u16 {
        #[inline(always)]
        fn conv_seq_offset(self) -> i32 {
            return self as i32;
        }
    }
    impl Make for u16 {
        #[inline(always)]
        fn make(self) -> SeqOffset {
            let raw = <Self as Seal>::conv_seq_offset(self);
            return SeqOffset::_unchecked(raw);
        }
    }
    impl Seal for u8 {
        #[inline(always)]
        fn conv_seq_offset(self) -> i32 {
            return self as i32;
        }
    }
    impl Make for u8 {
        #[inline(always)]
        fn make(self) -> SeqOffset {
            let raw = <Self as Seal>::conv_seq_offset(self);
            return SeqOffset::_unchecked(raw);
        }
    }
    impl ::core::convert::AsRef<i32> for SeqOffset {
        #[inline(always)]
        fn as_ref(&self) -> &i32 {
            return &self.0;
        }
    }
    impl ::core::str::FromStr for SeqOffset {
        type Err = ();
        #[inline(always)]
        fn from_str(source: &str) -> ::core::result::Result<Self, Self::Err> {
            let value = match <i32 as ::core::str::FromStr>::from_str(source) {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(_) => {
                    return ::core::result::Result::Err(());
                }
            };
            return if true {
                ::core::result::Result::Ok(Self(value))
            }
            else {
                ::core::result::Result::Err(())
            };
        }
    }
    impl ::core::convert::Into<isize> for SeqOffset {
        #[inline(always)]
        fn into(self) -> isize {
            return SeqOffset::into_isize(self);
        }
    }
    impl ::core::convert::Into<i32> for SeqOffset {
        #[inline(always)]
        fn into(self) -> i32 {
            return SeqOffset::into_i32(self);
        }
    }
    impl ::core::convert::Into<i64> for SeqOffset {
        #[inline(always)]
        fn into(self) -> i64 {
            return SeqOffset::into_i64(self);
        }
    }
    impl ::core::convert::Into<i128> for SeqOffset {
        #[inline(always)]
        fn into(self) -> i128 {
            return SeqOffset::into_i128(self);
        }
    }
    impl ::core::convert::TryInto<usize> for SeqOffset {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<usize, Self::Error> {
            return SeqOffset::try_into_usize(self);
        }
    }
    impl ::core::convert::TryInto<u8> for SeqOffset {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u8, Self::Error> {
            return SeqOffset::try_into_u8(self);
        }
    }
    impl ::core::convert::TryInto<u16> for SeqOffset {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u16, Self::Error> {
            return SeqOffset::try_into_u16(self);
        }
    }
    impl ::core::convert::TryInto<u32> for SeqOffset {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u32, Self::Error> {
            return SeqOffset::try_into_u32(self);
        }
    }
    impl ::core::convert::TryInto<u64> for SeqOffset {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u64, Self::Error> {
            return SeqOffset::try_into_u64(self);
        }
    }
    impl ::core::convert::TryInto<u128> for SeqOffset {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u128, Self::Error> {
            return SeqOffset::try_into_u128(self);
        }
    }
    impl ::core::convert::TryInto<i8> for SeqOffset {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i8, Self::Error> {
            return SeqOffset::try_into_i8(self);
        }
    }
    impl ::core::convert::TryInto<i16> for SeqOffset {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i16, Self::Error> {
            return SeqOffset::try_into_i16(self);
        }
    }
    impl ::core::ops::AddAssign for SeqOffset {
        #[inline(always)]
        fn add_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._add(rhs.0);
        }
    }
    impl<T> ::core::ops::AddAssign<T> for SeqOffset
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn add_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_seq_offset(rhs);
            *self = self._add(it);
        }
    }
    impl ::core::ops::SubAssign for SeqOffset {
        #[inline(always)]
        fn sub_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._sub(rhs.0);
        }
    }
    impl<T> ::core::ops::SubAssign<T> for SeqOffset
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn sub_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_seq_offset(rhs);
            *self = self._sub(it);
        }
    }
    impl ::core::fmt::Debug for SeqOffset {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            return f.debug_tuple("SeqOffset").field(&self.0).finish();
        }
    }
    impl ::core::hash::Hash for SeqOffset {
        #[inline(always)]
        fn hash<H>(
            &self,
            state: &mut H,
        ) where
            H: ::core::hash::Hasher,
        {
            return ::core::hash::Hash::hash(&self.0, state);
        }
    }
    impl ::core::cmp::Eq for SeqOffset {}
    if !(::core::mem::size_of::<SeqOffset>() == ::core::mem::size_of::<i32>()) {
        panic!("invalid memory layout, mismatching sizes: #ty(#el) != #el");
    };
    if !(::core::mem::align_of::<SeqOffset>() == ::core::mem::align_of::<i32>())
    {
        panic!("invalid memory layout, mismatching alignment: #ty(#el) != #el");
    };
    impl ::core::ops::Add for SeqOffset {
        type Output = Self;
        #[inline(always)]
        fn add(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._add(rhs.0);
        }
    }
    impl<T> ::core::ops::Add<T> for SeqOffset
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn add(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_seq_offset(rhs);
            return self._add(it);
        }
    }
    impl ::core::ops::Sub for SeqOffset {
        type Output = Self;
        #[inline(always)]
        fn sub(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._sub(rhs.0);
        }
    }
    impl<T> ::core::ops::Sub<T> for SeqOffset
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn sub(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_seq_offset(rhs);
            return self._sub(it);
        }
    }
    impl ::core::fmt::Binary for SeqOffset {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::Binary::fmt(&raw, f);
        }
    }
    impl ::core::fmt::Octal for SeqOffset {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::Octal::fmt(&raw, f);
        }
    }
    impl ::core::fmt::LowerHex for SeqOffset {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::LowerHex::fmt(&raw, f);
        }
    }
    impl ::core::fmt::UpperHex for SeqOffset {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::UpperHex::fmt(&raw, f);
        }
    }
    impl<T> ::core::cmp::PartialEq<T> for SeqOffset
    where
        T: Rel + Seal + ::core::marker::Copy,
    {
        #[inline(always)]
        fn eq(
            &self,
            rhs: &T,
        ) -> bool {
            let lhs = Self::raw(*self);
            let rhs = Seal::conv_seq_offset(*rhs);
            return lhs == rhs;
        }
    }
    impl SeqOffset {
        #[inline(always)]
        #[allow(private_bounds)]
        pub fn of<T>(it: T) -> SeqOffset
        where
            T: Make,
        {
            return <T as Make>::make(it);
        }
        #[inline(always)]
        pub fn make(it: i32) -> SeqOffset {
            return SeqOffset::of(it);
        }
        #[must_use]
        #[inline(always)]
        pub const fn raw(self) -> i32 {
            return self.0;
        }
        #[must_use]
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn into_isize(self) -> isize {
            let it = Self::raw(self);
            return it as isize;
        }
        #[must_use]
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn into_i32(self) -> i32 {
            let it = Self::raw(self);
            return it as i32;
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
        pub fn try_map<F>(
            self,
            function: F,
        ) -> ::core::result::Result<Self, i32>
        where
            F: ::core::ops::FnOnce(i32) -> i32,
        {
            let value = function(Self::raw(self));
            return if true {
                ::core::result::Result::Ok(Self(value))
            }
            else {
                ::core::result::Result::Err(value)
            };
        }
        #[inline(always)]
        #[track_caller]
        pub fn map<F>(
            self,
            function: F,
        ) -> Self
        where
            F: ::core::ops::FnOnce(i32) -> i32,
        {
            return match self.try_map(function) {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(_) => {
                    ::core::panic!("integral mapping produced an invalid value")
                }
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_isize(self) -> ::core::result::Result<isize, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as isize);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i32(self) -> ::core::result::Result<i32, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as i32);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i64(self) -> ::core::result::Result<i64, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as i64);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i128(self) -> ::core::result::Result<i128, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as i128);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_usize(self) -> ::core::result::Result<usize, ()> {
            let r = Self::raw(self);
            let t = r as usize;
            let s = t as i32;
            return if s == r && r >= 0 {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u8(self) -> ::core::result::Result<u8, ()> {
            let r = Self::raw(self);
            let t = r as u8;
            let s = t as i32;
            return if s == r && r >= 0 {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u16(self) -> ::core::result::Result<u16, ()> {
            let r = Self::raw(self);
            let t = r as u16;
            let s = t as i32;
            return if s == r && r >= 0 {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u32(self) -> ::core::result::Result<u32, ()> {
            let r = Self::raw(self);
            let t = r as u32;
            let s = t as i32;
            return if s == r && r >= 0 {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u64(self) -> ::core::result::Result<u64, ()> {
            let r = Self::raw(self);
            let t = r as u64;
            let s = t as i32;
            return if s == r && r >= 0 {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u128(self) -> ::core::result::Result<u128, ()> {
            let r = Self::raw(self);
            let t = r as u128;
            let s = t as i32;
            return if s == r && r >= 0 {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i8(self) -> ::core::result::Result<i8, ()> {
            let r = Self::raw(self);
            let t = r as i8;
            let s = t as i32;
            return if s == r && true {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i16(self) -> ::core::result::Result<i16, ()> {
            let r = Self::raw(self);
            let t = r as i16;
            let s = t as i32;
            return if s == r && true {
                ::core::result::Result::Ok(t)
            }
            else {
                ::core::result::Result::Err(())
            };
        }
        #[inline(always)]
        pub(self) const fn _add(
            &self,
            rhs: i32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_add(rhs);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _sub(
            &self,
            rhs: i32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_sub(rhs);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _mul(
            &self,
            rhs: i32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_mul(rhs);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _div(
            &self,
            rhs: i32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_div(rhs);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _rem(
            &self,
            rhs: i32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_rem(rhs);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _bitxor(
            &self,
            rhs: i32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs ^ rhs;
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _bitand(
            &self,
            rhs: i32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs & rhs;
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _bitor(
            &self,
            rhs: i32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs | rhs;
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _shr(
            &self,
            count: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_shr(count as u32);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _shl(
            &self,
            count: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_shl(count as u32);
            return Self::_unchecked(it);
        }
        #[inline(always)]
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
            rhs: i32,
        ) -> bool {
            let lhs = Self::raw(self);
            return lhs == rhs;
        }
        #[must_use]
        #[inline(always)]
        #[doc(hidden)]
        pub(self) fn _cmp(
            self,
            rhs: i32,
        ) -> ::core::cmp::Ordering {
            let lhs = Self::raw(self);
            return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs).unwrap();
        }
        #[inline(always)]
        pub const fn try_make(it: i32) -> Result<Self, i32> {
            return Ok(Self::_unchecked(it));
        }
        #[must_use]
        #[inline(always)]
        pub(self) const fn _unchecked(it: i32) -> Self {
            return Self(it);
        }
    }
    #[allow(clippy::unnecessary_cast)]
    impl SeqOffset {}
};

impl SeqPoint {
    fn into_offset(self) -> SeqOffset {
        return SeqOffset::make(self.raw() as i32);
    }
}

impl Add<SeqOffset> for SeqPoint {
    type Output = Self;

    #[inline(always)]
    fn add(
        self,
        rhs: SeqOffset,
    ) -> Self::Output {
        return Self::make(self.raw().wrapping_add(rhs.raw() as u32));
    }
}

impl Sub<SeqOffset> for SeqPoint {
    type Output = Self;

    #[inline(always)]
    fn sub(
        self,
        rhs: SeqOffset,
    ) -> Self::Output {
        return Self::make(self.raw().wrapping_sub(rhs.raw() as u32));
    }
}

fn main() {
    let maximum = SeqPoint::make(u32::MAX);
    let zero = SeqPoint::make(0);
    let offset = zero - maximum;

    assert_eq!(offset.raw(), 1);
    assert!(maximum.precedes(zero));
    assert!(!zero.precedes(SeqPoint::make(1 << 31)));

    let wrapped = maximum + SeqOffset::make(1);
    assert_eq!(wrapped.raw(), 0);

    let offset = SeqOffset::make(i32::MAX) + SeqOffset::make(1);
    assert_eq!(offset.raw(), i32::MIN);
}
