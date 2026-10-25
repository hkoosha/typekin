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
    #[doc = "# Safety"]
    #[doc = ""]
    #[doc = "Implementing this trait asserts that values \
                        produced by the sealed conversion satisfy every \
                        invariant required by the target type."]
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
        ::core::panic!("invalid integral storage size");
    };
    if !(::core::mem::align_of::<SeqPoint>() == ::core::mem::align_of::<u32>())
    {
        ::core::panic!("invalid integral storage alignment");
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
                    ::core::panic!(
                        "mapping operation produced an invalid value"
                    )
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
            rhs: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_shr(rhs as u32);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _shl(
            &self,
            rhs: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_shl(rhs as u32);
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
    #[doc = "# Safety"]
    #[doc = ""]
    #[doc = "Implementing this trait asserts that values \
                        produced by the sealed conversion satisfy every \
                        invariant required by the target type."]
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
        ::core::panic!("invalid integral storage size");
    };
    if !(::core::mem::align_of::<SeqOffset>() == ::core::mem::align_of::<i32>())
    {
        ::core::panic!("invalid integral storage alignment");
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
                    ::core::panic!(
                        "mapping operation produced an invalid value"
                    )
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
            rhs: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_shr(rhs as u32);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _shl(
            &self,
            rhs: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_shl(rhs as u32);
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

#[repr(transparent)]
#[derive(Copy, Clone)]
struct WrappingCounter(::core::num::Wrapping<u32>);

#[allow(dead_code)]
#[allow(unused_qualifications)]
#[allow(clippy::unnecessary_cast)]
const _: () = {
    trait Seal {
        fn conv_wrapping_counter(self) -> u32;
    }
    trait Make: Seal {
        fn make(self) -> WrappingCounter;
    }
    trait Math: Seal {}
    trait Bit: Seal {}
    trait Rel: Seal {}
    #[doc = "# Safety"]
    #[doc = ""]
    #[doc = "Implementing this trait asserts that values \
                        produced by the sealed conversion satisfy every \
                        invariant required by the target type."]
    unsafe trait UnsafeTrust: Seal {}
    impl Seal for WrappingCounter {
        #[inline(always)]
        fn conv_wrapping_counter(self) -> u32 {
            return Self::raw(self);
        }
    }
    impl Make for WrappingCounter {
        #[inline(always)]
        fn make(self) -> WrappingCounter {
            let raw = <Self as Seal>::conv_wrapping_counter(self);
            return WrappingCounter::_unchecked(raw);
        }
    }
    impl Rel for WrappingCounter {}
    impl Seal for i16 {
        #[inline(always)]
        fn conv_wrapping_counter(self) -> u32 {
            return self as u32;
        }
    }
    impl Make for i16 {
        #[inline(always)]
        fn make(self) -> WrappingCounter {
            let raw = <Self as Seal>::conv_wrapping_counter(self);
            return WrappingCounter::_unchecked(raw);
        }
    }
    impl Seal for u16 {
        #[inline(always)]
        fn conv_wrapping_counter(self) -> u32 {
            return self as u32;
        }
    }
    impl Make for u16 {
        #[inline(always)]
        fn make(self) -> WrappingCounter {
            let raw = <Self as Seal>::conv_wrapping_counter(self);
            return WrappingCounter::_unchecked(raw);
        }
    }
    impl Seal for u32 {
        #[inline(always)]
        fn conv_wrapping_counter(self) -> u32 {
            return self as u32;
        }
    }
    impl Bit for u32 {}
    impl Make for u32 {
        #[inline(always)]
        fn make(self) -> WrappingCounter {
            let raw = <Self as Seal>::conv_wrapping_counter(self);
            return WrappingCounter::_unchecked(raw);
        }
    }
    impl Math for u32 {}
    impl Rel for u32 {}
    impl Seal for u8 {
        #[inline(always)]
        fn conv_wrapping_counter(self) -> u32 {
            return self as u32;
        }
    }
    impl Make for u8 {
        #[inline(always)]
        fn make(self) -> WrappingCounter {
            let raw = <Self as Seal>::conv_wrapping_counter(self);
            return WrappingCounter::_unchecked(raw);
        }
    }
    impl ::core::convert::AsRef<u32> for WrappingCounter {
        #[inline(always)]
        fn as_ref(&self) -> &u32 {
            return unsafe { &*(::core::ptr::addr_of!(self.0).cast::<u32>()) };
        }
    }
    impl ::core::str::FromStr for WrappingCounter {
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
                ::core::result::Result::Ok(Self(::core::num::Wrapping(value)))
            }
            else {
                ::core::result::Result::Err(())
            };
        }
    }
    impl WrappingCounter {
        #[doc = r" The number of bits in the wrapped primitive integer."]
        pub const BITS: u32 = u32::BITS;
        #[inline(always)]
        pub const fn count_ones(self) -> u32 {
            return Self::raw(self).count_ones();
        }
        #[inline(always)]
        pub const fn count_zeros(self) -> u32 {
            return Self::raw(self).count_zeros();
        }
        #[inline(always)]
        pub const fn leading_zeros(self) -> u32 {
            return Self::raw(self).leading_zeros();
        }
        #[inline(always)]
        pub const fn trailing_zeros(self) -> u32 {
            return Self::raw(self).trailing_zeros();
        }
        #[inline(always)]
        pub const fn leading_ones(self) -> u32 {
            return Self::raw(self).leading_ones();
        }
        #[inline(always)]
        pub const fn trailing_ones(self) -> u32 {
            return Self::raw(self).trailing_ones();
        }
        #[inline(always)]
        pub const fn highest_one(self) -> ::core::option::Option<u32> {
            return Self::raw(self).highest_one();
        }
        #[inline(always)]
        pub const fn lowest_one(self) -> ::core::option::Option<u32> {
            return Self::raw(self).lowest_one();
        }
        #[inline(always)]
        pub const fn ilog(
            self,
            base: Self,
        ) -> u32 {
            return Self::raw(self).ilog(Self::raw(base));
        }
        #[inline(always)]
        pub const fn ilog2(self) -> u32 {
            return Self::raw(self).ilog2();
        }
        #[inline(always)]
        pub const fn ilog10(self) -> u32 {
            return Self::raw(self).ilog10();
        }
        #[inline(always)]
        pub const fn checked_ilog(
            self,
            base: Self,
        ) -> ::core::option::Option<u32> {
            return Self::raw(self).checked_ilog(Self::raw(base));
        }
        #[inline(always)]
        pub const fn checked_ilog2(self) -> ::core::option::Option<u32> {
            return Self::raw(self).checked_ilog2();
        }
        #[inline(always)]
        pub const fn checked_ilog10(self) -> ::core::option::Option<u32> {
            return Self::raw(self).checked_ilog10();
        }
        #[inline(always)]
        pub const fn to_be_bytes(self) -> [u8; ::core::mem::size_of::<u32>()] {
            return Self::raw(self).to_be_bytes();
        }
        #[inline(always)]
        pub const fn to_le_bytes(self) -> [u8; ::core::mem::size_of::<u32>()] {
            return Self::raw(self).to_le_bytes();
        }
        #[inline(always)]
        pub const fn to_ne_bytes(self) -> [u8; ::core::mem::size_of::<u32>()] {
            return Self::raw(self).to_ne_bytes();
        }
    }
    impl WrappingCounter {
        #[inline(always)]
        pub fn rotate_left(
            self,
            n: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).rotate_left(n));
        }
        #[inline(always)]
        pub fn rotate_right(
            self,
            n: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).rotate_right(n));
        }
        #[inline(always)]
        pub fn swap_bytes(self) -> Self {
            return Self::_core_int(Self::raw(self).swap_bytes());
        }
        #[inline(always)]
        pub fn reverse_bits(self) -> Self {
            return Self::_core_int(Self::raw(self).reverse_bits());
        }
        #[inline(always)]
        pub fn isolate_highest_one(self) -> Self {
            return Self::_core_int(Self::raw(self).isolate_highest_one());
        }
        #[inline(always)]
        pub fn isolate_lowest_one(self) -> Self {
            return Self::_core_int(Self::raw(self).isolate_lowest_one());
        }
        #[inline(always)]
        pub fn to_be(self) -> Self {
            return Self::_core_int(Self::raw(self).to_be());
        }
        #[inline(always)]
        pub fn to_le(self) -> Self {
            return Self::_core_int(Self::raw(self).to_le());
        }
        #[inline(always)]
        pub fn from_be(value: Self) -> Self {
            return Self::_core_int(u32::from_be(Self::raw(value)));
        }
        #[inline(always)]
        pub fn from_le(value: Self) -> Self {
            return Self::_core_int(u32::from_le(Self::raw(value)));
        }
        #[inline(always)]
        pub fn from_ne_bytes(
            bytes: [u8; ::core::mem::size_of::<u32>()]
        ) -> Self {
            return Self::_core_int(u32::from_ne_bytes(bytes));
        }
        #[inline(always)]
        pub fn from_be_bytes(
            bytes: [u8; ::core::mem::size_of::<u32>()]
        ) -> Self {
            return Self::_core_int(u32::from_be_bytes(bytes));
        }
        #[inline(always)]
        pub fn from_le_bytes(
            bytes: [u8; ::core::mem::size_of::<u32>()]
        ) -> Self {
            return Self::_core_int(u32::from_le_bytes(bytes));
        }
        #[inline(always)]
        pub fn checked_add(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_add(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_sub(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_sub(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_neg(self) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_neg() {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_mul(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_mul(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_div(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_div(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_rem(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_rem(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_div_euclid(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_div_euclid(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_rem_euclid(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_rem_euclid(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_shl(
            self,
            rhs: u32,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_shl(rhs) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_shr(
            self,
            rhs: u32,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_shr(rhs) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_pow(
            self,
            exp: u32,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_pow(exp) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
    }
    impl WrappingCounter {
        #[inline(always)]
        pub fn midpoint(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).midpoint(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn unbounded_shl(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).unbounded_shl(rhs));
        }
        #[inline(always)]
        pub fn unbounded_shr(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).unbounded_shr(rhs));
        }
        #[inline(always)]
        pub fn saturating_add(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_add(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_sub(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_sub(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_mul(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_mul(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_div(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_div(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).saturating_pow(exp));
        }
        #[inline(always)]
        pub fn wrapping_add(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_add(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_sub(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_sub(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_neg(self) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_neg());
        }
        #[inline(always)]
        pub fn wrapping_mul(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_mul(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_div(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_div(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_rem(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_rem(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_div_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_div_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_rem_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_rem_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_shl(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_shl(rhs));
        }
        #[inline(always)]
        pub fn wrapping_shr(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_shr(rhs));
        }
        #[inline(always)]
        pub fn wrapping_pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_pow(exp));
        }
        #[inline(always)]
        pub fn pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).pow(exp));
        }
        #[inline(always)]
        pub fn isqrt(self) -> Self {
            return Self::_core_int(Self::raw(self).isqrt());
        }
        #[inline(always)]
        pub fn div_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).div_euclid(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn rem_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).rem_euclid(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_add(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_add(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_sub(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_sub(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_neg(self) -> Self {
            return Self::_core_int(Self::raw(self).strict_neg());
        }
        #[inline(always)]
        pub fn strict_mul(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_mul(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_div(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_div(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_rem(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_rem(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_div_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).strict_div_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn strict_rem_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).strict_rem_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn strict_shl(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_shl(rhs));
        }
        #[inline(always)]
        pub fn strict_shr(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_shr(rhs));
        }
        #[inline(always)]
        pub fn strict_pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_pow(exp));
        }
    }
    impl WrappingCounter {
        #[inline(always)]
        pub fn overflowing_add(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_add(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_sub(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_sub(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_neg(self) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_neg();
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_mul(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_mul(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_div(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_div(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_rem(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_rem(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_div_euclid(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_div_euclid(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_rem_euclid(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_rem_euclid(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_shl(
            self,
            rhs: u32,
        ) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_shl(rhs);
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_shr(
            self,
            rhs: u32,
        ) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_shr(rhs);
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_pow(
            self,
            exp: u32,
        ) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_pow(exp);
            return (Self::_core_int(value), overflowed);
        }
    }
    impl WrappingCounter {
        #[inline(always)]
        pub fn cast_signed(self) -> i32 {
            return Self::raw(self).cast_signed();
        }
        #[inline(always)]
        pub fn bit_width(self) -> u32 {
            return Self::raw(self).bit_width();
        }
        #[inline(always)]
        pub fn abs_diff(
            self,
            rhs: Self,
        ) -> u32 {
            return Self::raw(self).abs_diff(Self::raw(rhs));
        }
        #[inline(always)]
        pub fn is_multiple_of(
            self,
            rhs: Self,
        ) -> bool {
            return Self::raw(self).is_multiple_of(Self::raw(rhs));
        }
        #[inline(always)]
        pub fn is_power_of_two(self) -> bool {
            return Self::raw(self).is_power_of_two();
        }
        #[inline(always)]
        pub fn next_power_of_two(self) -> Self {
            return Self::_core_int(Self::raw(self).next_power_of_two());
        }
        #[inline(always)]
        pub fn checked_next_power_of_two(self) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_next_power_of_two() {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn div_ceil(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).div_ceil(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn next_multiple_of(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).next_multiple_of(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn checked_next_multiple_of(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self)
                .checked_next_multiple_of(Self::raw(rhs))
            {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn carrying_add(
            self,
            rhs: Self,
            carry: bool,
        ) -> (Self, bool) {
            let (value, carry) =
                Self::raw(self).carrying_add(Self::raw(rhs), carry);
            return (Self::_core_int(value), carry);
        }
        #[inline(always)]
        pub fn borrowing_sub(
            self,
            rhs: Self,
            borrow: bool,
        ) -> (Self, bool) {
            let (value, borrow) =
                Self::raw(self).borrowing_sub(Self::raw(rhs), borrow);
            return (Self::_core_int(value), borrow);
        }
        #[inline(always)]
        pub fn carrying_mul(
            self,
            rhs: Self,
            carry: Self,
        ) -> (Self, Self) {
            let (low, high) =
                Self::raw(self).carrying_mul(Self::raw(rhs), Self::raw(carry));
            return (Self::_core_int(low), Self::_core_int(high));
        }
        #[inline(always)]
        pub fn carrying_mul_add(
            self,
            rhs: Self,
            carry: Self,
            add: Self,
        ) -> (Self, Self) {
            let (low, high) = Self::raw(self).carrying_mul_add(
                Self::raw(rhs),
                Self::raw(carry),
                Self::raw(add),
            );
            return (Self::_core_int(low), Self::_core_int(high));
        }
    }
    impl ::core::convert::Into<usize> for WrappingCounter {
        #[inline(always)]
        fn into(self) -> usize {
            return WrappingCounter::into_usize(self);
        }
    }
    impl ::core::convert::Into<u32> for WrappingCounter {
        #[inline(always)]
        fn into(self) -> u32 {
            return WrappingCounter::into_u32(self);
        }
    }
    impl ::core::convert::Into<u64> for WrappingCounter {
        #[inline(always)]
        fn into(self) -> u64 {
            return WrappingCounter::into_u64(self);
        }
    }
    impl ::core::convert::Into<u128> for WrappingCounter {
        #[inline(always)]
        fn into(self) -> u128 {
            return WrappingCounter::into_u128(self);
        }
    }
    impl ::core::convert::Into<i64> for WrappingCounter {
        #[inline(always)]
        fn into(self) -> i64 {
            return WrappingCounter::into_i64(self);
        }
    }
    impl ::core::convert::Into<i128> for WrappingCounter {
        #[inline(always)]
        fn into(self) -> i128 {
            return WrappingCounter::into_i128(self);
        }
    }
    impl ::core::convert::TryInto<isize> for WrappingCounter {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<isize, Self::Error> {
            return WrappingCounter::try_into_isize(self);
        }
    }
    impl ::core::convert::TryInto<u8> for WrappingCounter {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u8, Self::Error> {
            return WrappingCounter::try_into_u8(self);
        }
    }
    impl ::core::convert::TryInto<u16> for WrappingCounter {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u16, Self::Error> {
            return WrappingCounter::try_into_u16(self);
        }
    }
    impl ::core::convert::TryInto<i8> for WrappingCounter {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i8, Self::Error> {
            return WrappingCounter::try_into_i8(self);
        }
    }
    impl ::core::convert::TryInto<i16> for WrappingCounter {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i16, Self::Error> {
            return WrappingCounter::try_into_i16(self);
        }
    }
    impl ::core::convert::TryInto<i32> for WrappingCounter {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i32, Self::Error> {
            return WrappingCounter::try_into_i32(self);
        }
    }
    impl ::core::ops::Shr<usize> for WrappingCounter {
        type Output = Self;
        #[inline(always)]
        fn shr(
            self,
            rhs: usize,
        ) -> Self::Output {
            return self._shr_wrapping(rhs);
        }
    }
    impl ::core::ops::Shl<usize> for WrappingCounter {
        type Output = Self;
        #[inline(always)]
        fn shl(
            self,
            rhs: usize,
        ) -> Self::Output {
            return self._shl_wrapping(rhs);
        }
    }
    impl ::core::ops::ShrAssign<usize> for WrappingCounter {
        #[inline(always)]
        fn shr_assign(
            &mut self,
            other: usize,
        ) {
            *self = self._shr_wrapping(other);
        }
    }
    impl ::core::ops::ShlAssign<usize> for WrappingCounter {
        #[inline(always)]
        fn shl_assign(
            &mut self,
            other: usize,
        ) {
            *self = self._shl_wrapping(other);
        }
    }
    impl ::core::ops::BitAndAssign for WrappingCounter {
        #[inline(always)]
        fn bitand_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._bitand((rhs.0).0);
        }
    }
    impl<T> ::core::ops::BitAndAssign<T> for WrappingCounter
    where
        T: Bit + Seal,
    {
        #[inline(always)]
        fn bitand_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_wrapping_counter(rhs);
            *self = self._bitand(it);
        }
    }
    impl ::core::ops::AddAssign for WrappingCounter {
        #[inline(always)]
        fn add_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._add_wrapping((rhs.0).0);
        }
    }
    impl<T> ::core::ops::AddAssign<T> for WrappingCounter
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn add_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_wrapping_counter(rhs);
            *self = self._add_wrapping(it);
        }
    }
    impl ::core::ops::SubAssign for WrappingCounter {
        #[inline(always)]
        fn sub_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._sub_wrapping((rhs.0).0);
        }
    }
    impl<T> ::core::ops::SubAssign<T> for WrappingCounter
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn sub_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_wrapping_counter(rhs);
            *self = self._sub_wrapping(it);
        }
    }
    impl ::core::ops::MulAssign for WrappingCounter {
        #[inline(always)]
        fn mul_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._mul_wrapping((rhs.0).0);
        }
    }
    impl<T> ::core::ops::MulAssign<T> for WrappingCounter
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn mul_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_wrapping_counter(rhs);
            *self = self._mul_wrapping(it);
        }
    }
    impl ::core::ops::DivAssign for WrappingCounter {
        #[inline(always)]
        fn div_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._div_wrapping((rhs.0).0);
        }
    }
    impl<T> ::core::ops::DivAssign<T> for WrappingCounter
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn div_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_wrapping_counter(rhs);
            *self = self._div_wrapping(it);
        }
    }
    impl ::core::ops::RemAssign for WrappingCounter {
        #[inline(always)]
        fn rem_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._rem_wrapping((rhs.0).0);
        }
    }
    impl<T> ::core::ops::RemAssign<T> for WrappingCounter
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn rem_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_wrapping_counter(rhs);
            *self = self._rem_wrapping(it);
        }
    }
    impl ::core::ops::BitOrAssign for WrappingCounter {
        #[inline(always)]
        fn bitor_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._bitor((rhs.0).0);
        }
    }
    impl<T> ::core::ops::BitOrAssign<T> for WrappingCounter
    where
        T: Bit + Seal,
    {
        #[inline(always)]
        fn bitor_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_wrapping_counter(rhs);
            *self = self._bitor(it);
        }
    }
    impl ::core::fmt::Debug for WrappingCounter {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            return f
                .debug_tuple("WrappingCounter")
                .field(&(self.0).0)
                .finish();
        }
    }
    impl ::core::hash::Hash for WrappingCounter {
        #[inline(always)]
        fn hash<H>(
            &self,
            state: &mut H,
        ) where
            H: ::core::hash::Hasher,
        {
            return ::core::hash::Hash::hash(&(self.0).0, state);
        }
    }
    impl ::core::cmp::Eq for WrappingCounter {}
    impl ::core::cmp::Ord for WrappingCounter {
        #[inline(always)]
        fn cmp(
            &self,
            other: &Self,
        ) -> ::core::cmp::Ordering {
            return self.partial_cmp(other).unwrap();
        }
    }
    if !(::core::mem::size_of::<WrappingCounter>()
        == ::core::mem::size_of::<u32>())
    {
        ::core::panic!("invalid integral storage size");
    };
    if !(::core::mem::align_of::<WrappingCounter>()
        == ::core::mem::align_of::<u32>())
    {
        ::core::panic!("invalid integral storage alignment");
    };
    impl ::core::ops::Add for WrappingCounter {
        type Output = Self;
        #[inline(always)]
        fn add(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._add_wrapping((rhs.0).0);
        }
    }
    impl<T> ::core::ops::Add<T> for WrappingCounter
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn add(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_wrapping_counter(rhs);
            return self._add_wrapping(it);
        }
    }
    impl ::core::ops::Sub for WrappingCounter {
        type Output = Self;
        #[inline(always)]
        fn sub(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._sub_wrapping((rhs.0).0);
        }
    }
    impl<T> ::core::ops::Sub<T> for WrappingCounter
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn sub(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_wrapping_counter(rhs);
            return self._sub_wrapping(it);
        }
    }
    impl ::core::ops::Mul for WrappingCounter {
        type Output = Self;
        #[inline(always)]
        fn mul(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._mul_wrapping((rhs.0).0);
        }
    }
    impl<T> ::core::ops::Mul<T> for WrappingCounter
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn mul(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_wrapping_counter(rhs);
            return self._mul_wrapping(it);
        }
    }
    impl ::core::ops::Div for WrappingCounter {
        type Output = Self;
        #[inline(always)]
        fn div(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._div_wrapping((rhs.0).0);
        }
    }
    impl<T> ::core::ops::Div<T> for WrappingCounter
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn div(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_wrapping_counter(rhs);
            return self._div_wrapping(it);
        }
    }
    impl ::core::ops::Rem for WrappingCounter {
        type Output = Self;
        #[inline(always)]
        fn rem(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._rem_wrapping((rhs.0).0);
        }
    }
    impl<T> ::core::ops::Rem<T> for WrappingCounter
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn rem(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_wrapping_counter(rhs);
            return self._rem_wrapping(it);
        }
    }
    impl ::core::ops::BitAnd for WrappingCounter {
        type Output = Self;
        #[inline(always)]
        fn bitand(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._bitand((rhs.0).0);
        }
    }
    impl<T> ::core::ops::BitAnd<T> for WrappingCounter
    where
        T: Bit + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn bitand(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_wrapping_counter(rhs);
            return self._bitand(it);
        }
    }
    impl ::core::ops::BitOr for WrappingCounter {
        type Output = Self;
        #[inline(always)]
        fn bitor(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._bitor((rhs.0).0);
        }
    }
    impl<T> ::core::ops::BitOr<T> for WrappingCounter
    where
        T: Bit + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn bitor(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_wrapping_counter(rhs);
            return self._bitor(it);
        }
    }
    impl ::core::ops::BitXor for WrappingCounter {
        type Output = Self;
        #[inline(always)]
        fn bitxor(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._bitxor((rhs.0).0);
        }
    }
    impl ::core::ops::BitXorAssign for WrappingCounter {
        #[inline(always)]
        fn bitxor_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._bitxor((rhs.0).0);
        }
    }
    impl<T> ::core::ops::BitXor<T> for WrappingCounter
    where
        T: Bit + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn bitxor(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_wrapping_counter(rhs);
            return self._bitxor(it);
        }
    }
    impl<T> ::core::ops::BitXorAssign<T> for WrappingCounter
    where
        T: Bit + Seal,
    {
        #[inline(always)]
        fn bitxor_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_wrapping_counter(rhs);
            *self = self._bitxor(it);
        }
    }
    impl ::core::ops::Not for WrappingCounter {
        type Output = Self;
        #[inline(always)]
        fn not(self) -> Self::Output {
            return self._not();
        }
    }
    impl ::core::fmt::Binary for WrappingCounter {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::Binary::fmt(&raw, f);
        }
    }
    impl ::core::fmt::Octal for WrappingCounter {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::Octal::fmt(&raw, f);
        }
    }
    impl ::core::fmt::LowerHex for WrappingCounter {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::LowerHex::fmt(&raw, f);
        }
    }
    impl ::core::fmt::UpperHex for WrappingCounter {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::UpperHex::fmt(&raw, f);
        }
    }
    impl<T> ::core::cmp::PartialEq<T> for WrappingCounter
    where
        T: Rel + Seal + ::core::marker::Copy,
    {
        #[inline(always)]
        fn eq(
            &self,
            rhs: &T,
        ) -> bool {
            let lhs = Self::raw(*self);
            let rhs = Seal::conv_wrapping_counter(*rhs);
            return lhs == rhs;
        }
    }
    impl<T> ::core::cmp::PartialOrd<T> for WrappingCounter
    where
        T: Rel + Seal + ::core::marker::Copy,
    {
        #[inline(always)]
        fn partial_cmp(
            &self,
            rhs: &T,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let lhs = Self::raw(*self);
            let rhs = Seal::conv_wrapping_counter(*rhs);
            return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
        }
    }
    impl WrappingCounter {
        #[inline(always)]
        #[allow(private_bounds)]
        pub fn of<T>(it: T) -> WrappingCounter
        where
            T: Make,
        {
            return <T as Make>::make(it);
        }
        #[inline(always)]
        pub fn make(it: u32) -> WrappingCounter {
            return WrappingCounter::of(it);
        }
        #[must_use]
        #[inline(always)]
        pub const fn raw(self) -> u32 {
            return (self.0).0;
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
                ::core::result::Result::Ok(Self(::core::num::Wrapping(value)))
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
                    ::core::panic!(
                        "mapping operation produced an invalid value"
                    )
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
        pub(self) fn _add_wrapping(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it =
                (::core::num::Wrapping(lhs) + ::core::num::Wrapping(rhs)).0;
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
        pub(self) fn _sub_wrapping(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it =
                (::core::num::Wrapping(lhs) - ::core::num::Wrapping(rhs)).0;
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
        pub(self) fn _mul_wrapping(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it =
                (::core::num::Wrapping(lhs) * ::core::num::Wrapping(rhs)).0;
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
        pub(self) fn _div_wrapping(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it =
                (::core::num::Wrapping(lhs) / ::core::num::Wrapping(rhs)).0;
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
        pub(self) fn _rem_wrapping(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it =
                (::core::num::Wrapping(lhs) % ::core::num::Wrapping(rhs)).0;
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
            rhs: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_shr(rhs as u32);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) fn _shr_wrapping(
            &self,
            rhs: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = (::core::num::Wrapping(lhs) >> rhs).0;
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _shl(
            &self,
            rhs: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_shl(rhs as u32);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) fn _shl_wrapping(
            &self,
            rhs: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = (::core::num::Wrapping(lhs) << rhs).0;
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
            return Self(::core::num::Wrapping(it));
        }
        #[inline(always)]
        #[track_caller]
        pub(self) fn _core_int(value: u32) -> Self {
            return Self(::core::num::Wrapping(value));
        }
        #[inline(always)]
        pub(self) fn _core_int_checked(
            value: u32
        ) -> ::core::option::Option<Self> {
            return if true {
                ::core::option::Option::Some(Self(::core::num::Wrapping(value)))
            }
            else {
                ::core::option::Option::None
            };
        }
    }
    #[allow(clippy::unnecessary_cast)]
    impl WrappingCounter {
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
            return ((Self::raw(self) >> (8 * 0usize)) & (0xFF as u32)) as u8;
        }
        #[must_use]
        #[inline(always)]
        pub const fn byte1(self) -> u8 {
            return ((Self::raw(self) >> (8 * 1usize)) & (0xFF as u32)) as u8;
        }
        #[must_use]
        #[inline(always)]
        pub const fn byte2(self) -> u8 {
            return ((Self::raw(self) >> (8 * 2usize)) & (0xFF as u32)) as u8;
        }
        #[must_use]
        #[inline(always)]
        pub const fn byte3(self) -> u8 {
            return ((Self::raw(self) >> (8 * 3usize)) & (0xFF as u32)) as u8;
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
            return ((Self::raw(self) >> (32 * 0usize)) & (0xFFFFFFFF as u32))
                as u32;
        }
    }
};

#[repr(transparent)]
#[derive(Copy, Clone)]
struct NonZeroSerial(::core::num::Wrapping<::core::num::NonZeroU32>);

#[allow(dead_code)]
#[allow(unused_qualifications)]
#[allow(clippy::unnecessary_cast)]
const _: () = {
    trait Seal {
        fn conv_non_zero_serial(self) -> u32;
    }
    trait Make: Seal {
        fn make(self) -> NonZeroSerial;
    }
    trait Math: Seal {}
    trait Bit: Seal {}
    trait Rel: Seal {}
    #[doc = "# Safety"]
    #[doc = ""]
    #[doc = "Implementing this trait asserts that values \
                        produced by the sealed conversion satisfy every \
                        invariant required by the target type."]
    unsafe trait UnsafeTrust: Seal {}
    impl Seal for NonZeroSerial {
        #[inline(always)]
        fn conv_non_zero_serial(self) -> u32 {
            return Self::raw(self);
        }
    }
    impl Make for NonZeroSerial {
        #[inline(always)]
        fn make(self) -> NonZeroSerial {
            let raw = <Self as Seal>::conv_non_zero_serial(self);
            return NonZeroSerial::_unchecked(raw);
        }
    }
    impl Rel for NonZeroSerial {}
    impl Seal for u32 {
        #[inline(always)]
        fn conv_non_zero_serial(self) -> u32 {
            return self as u32;
        }
    }
    impl Bit for u32 {}
    impl Math for u32 {}
    impl Rel for u32 {}
    impl ::core::convert::AsRef<u32> for NonZeroSerial {
        #[inline(always)]
        fn as_ref(&self) -> &u32 {
            return unsafe { &*(::core::ptr::addr_of!(self.0).cast::<u32>()) };
        }
    }
    impl ::core::str::FromStr for NonZeroSerial {
        type Err = ();
        #[inline(always)]
        fn from_str(source: &str) -> ::core::result::Result<Self, Self::Err> {
            let value = match <u32 as ::core::str::FromStr>::from_str(source) {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(_) => {
                    return ::core::result::Result::Err(());
                }
            };
            return if value >= (1) {
                ::core::result::Result::Ok(Self(::core::num::Wrapping(
                    match ::core::num::NonZeroU32::new(value) {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => {
                            ::core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    },
                )))
            }
            else {
                ::core::result::Result::Err(())
            };
        }
    }
    impl NonZeroSerial {
        #[doc = r" The number of bits in the wrapped primitive integer."]
        pub const BITS: u32 = u32::BITS;
        #[inline(always)]
        pub const fn count_ones(self) -> u32 {
            return Self::raw(self).count_ones();
        }
        #[inline(always)]
        pub const fn count_zeros(self) -> u32 {
            return Self::raw(self).count_zeros();
        }
        #[inline(always)]
        pub const fn leading_zeros(self) -> u32 {
            return Self::raw(self).leading_zeros();
        }
        #[inline(always)]
        pub const fn trailing_zeros(self) -> u32 {
            return Self::raw(self).trailing_zeros();
        }
        #[inline(always)]
        pub const fn leading_ones(self) -> u32 {
            return Self::raw(self).leading_ones();
        }
        #[inline(always)]
        pub const fn trailing_ones(self) -> u32 {
            return Self::raw(self).trailing_ones();
        }
        #[inline(always)]
        pub const fn highest_one(self) -> ::core::option::Option<u32> {
            return Self::raw(self).highest_one();
        }
        #[inline(always)]
        pub const fn lowest_one(self) -> ::core::option::Option<u32> {
            return Self::raw(self).lowest_one();
        }
        #[inline(always)]
        pub const fn ilog(
            self,
            base: Self,
        ) -> u32 {
            return Self::raw(self).ilog(Self::raw(base));
        }
        #[inline(always)]
        pub const fn ilog2(self) -> u32 {
            return Self::raw(self).ilog2();
        }
        #[inline(always)]
        pub const fn ilog10(self) -> u32 {
            return Self::raw(self).ilog10();
        }
        #[inline(always)]
        pub const fn checked_ilog(
            self,
            base: Self,
        ) -> ::core::option::Option<u32> {
            return Self::raw(self).checked_ilog(Self::raw(base));
        }
        #[inline(always)]
        pub const fn checked_ilog2(self) -> ::core::option::Option<u32> {
            return Self::raw(self).checked_ilog2();
        }
        #[inline(always)]
        pub const fn checked_ilog10(self) -> ::core::option::Option<u32> {
            return Self::raw(self).checked_ilog10();
        }
        #[inline(always)]
        pub const fn to_be_bytes(self) -> [u8; ::core::mem::size_of::<u32>()] {
            return Self::raw(self).to_be_bytes();
        }
        #[inline(always)]
        pub const fn to_le_bytes(self) -> [u8; ::core::mem::size_of::<u32>()] {
            return Self::raw(self).to_le_bytes();
        }
        #[inline(always)]
        pub const fn to_ne_bytes(self) -> [u8; ::core::mem::size_of::<u32>()] {
            return Self::raw(self).to_ne_bytes();
        }
    }
    impl NonZeroSerial {
        #[inline(always)]
        pub fn rotate_left(
            self,
            n: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).rotate_left(n));
        }
        #[inline(always)]
        pub fn rotate_right(
            self,
            n: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).rotate_right(n));
        }
        #[inline(always)]
        pub fn swap_bytes(self) -> Self {
            return Self::_core_int(Self::raw(self).swap_bytes());
        }
        #[inline(always)]
        pub fn reverse_bits(self) -> Self {
            return Self::_core_int(Self::raw(self).reverse_bits());
        }
        #[inline(always)]
        pub fn isolate_highest_one(self) -> Self {
            return Self::_core_int(Self::raw(self).isolate_highest_one());
        }
        #[inline(always)]
        pub fn isolate_lowest_one(self) -> Self {
            return Self::_core_int(Self::raw(self).isolate_lowest_one());
        }
        #[inline(always)]
        pub fn to_be(self) -> Self {
            return Self::_core_int(Self::raw(self).to_be());
        }
        #[inline(always)]
        pub fn to_le(self) -> Self {
            return Self::_core_int(Self::raw(self).to_le());
        }
        #[inline(always)]
        pub fn from_be(value: Self) -> Self {
            return Self::_core_int(u32::from_be(Self::raw(value)));
        }
        #[inline(always)]
        pub fn from_le(value: Self) -> Self {
            return Self::_core_int(u32::from_le(Self::raw(value)));
        }
        #[inline(always)]
        pub fn from_ne_bytes(
            bytes: [u8; ::core::mem::size_of::<u32>()]
        ) -> Self {
            return Self::_core_int(u32::from_ne_bytes(bytes));
        }
        #[inline(always)]
        pub fn from_be_bytes(
            bytes: [u8; ::core::mem::size_of::<u32>()]
        ) -> Self {
            return Self::_core_int(u32::from_be_bytes(bytes));
        }
        #[inline(always)]
        pub fn from_le_bytes(
            bytes: [u8; ::core::mem::size_of::<u32>()]
        ) -> Self {
            return Self::_core_int(u32::from_le_bytes(bytes));
        }
        #[inline(always)]
        pub fn checked_add(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_add(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_sub(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_sub(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_neg(self) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_neg() {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_mul(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_mul(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_div(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_div(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_rem(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_rem(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_div_euclid(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_div_euclid(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_rem_euclid(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_rem_euclid(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_shl(
            self,
            rhs: u32,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_shl(rhs) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_shr(
            self,
            rhs: u32,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_shr(rhs) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_pow(
            self,
            exp: u32,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_pow(exp) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
    }
    impl NonZeroSerial {
        #[inline(always)]
        pub fn midpoint(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).midpoint(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn unbounded_shl(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).unbounded_shl(rhs));
        }
        #[inline(always)]
        pub fn unbounded_shr(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).unbounded_shr(rhs));
        }
        #[inline(always)]
        pub fn saturating_add(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_add(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_sub(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_sub(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_mul(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_mul(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_div(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_div(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).saturating_pow(exp));
        }
        #[inline(always)]
        pub fn wrapping_add(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_add(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_sub(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_sub(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_neg(self) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_neg());
        }
        #[inline(always)]
        pub fn wrapping_mul(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_mul(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_div(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_div(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_rem(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_rem(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_div_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_div_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_rem_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_rem_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_shl(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_shl(rhs));
        }
        #[inline(always)]
        pub fn wrapping_shr(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_shr(rhs));
        }
        #[inline(always)]
        pub fn wrapping_pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_pow(exp));
        }
        #[inline(always)]
        pub fn pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).pow(exp));
        }
        #[inline(always)]
        pub fn isqrt(self) -> Self {
            return Self::_core_int(Self::raw(self).isqrt());
        }
        #[inline(always)]
        pub fn div_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).div_euclid(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn rem_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).rem_euclid(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_add(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_add(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_sub(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_sub(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_neg(self) -> Self {
            return Self::_core_int(Self::raw(self).strict_neg());
        }
        #[inline(always)]
        pub fn strict_mul(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_mul(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_div(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_div(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_rem(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_rem(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_div_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).strict_div_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn strict_rem_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).strict_rem_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn strict_shl(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_shl(rhs));
        }
        #[inline(always)]
        pub fn strict_shr(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_shr(rhs));
        }
        #[inline(always)]
        pub fn strict_pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_pow(exp));
        }
    }
    impl NonZeroSerial {
        #[inline(always)]
        pub fn overflowing_add(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_add(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_sub(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_sub(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_neg(self) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_neg();
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_mul(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_mul(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_div(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_div(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_rem(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_rem(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_div_euclid(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_div_euclid(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_rem_euclid(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_rem_euclid(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_shl(
            self,
            rhs: u32,
        ) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_shl(rhs);
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_shr(
            self,
            rhs: u32,
        ) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_shr(rhs);
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_pow(
            self,
            exp: u32,
        ) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_pow(exp);
            return (Self::_core_int(value), overflowed);
        }
    }
    impl NonZeroSerial {
        #[inline(always)]
        pub fn cast_signed(self) -> i32 {
            return Self::raw(self).cast_signed();
        }
        #[inline(always)]
        pub fn bit_width(self) -> u32 {
            return Self::raw(self).bit_width();
        }
        #[inline(always)]
        pub fn abs_diff(
            self,
            rhs: Self,
        ) -> u32 {
            return Self::raw(self).abs_diff(Self::raw(rhs));
        }
        #[inline(always)]
        pub fn is_multiple_of(
            self,
            rhs: Self,
        ) -> bool {
            return Self::raw(self).is_multiple_of(Self::raw(rhs));
        }
        #[inline(always)]
        pub fn is_power_of_two(self) -> bool {
            return Self::raw(self).is_power_of_two();
        }
        #[inline(always)]
        pub fn next_power_of_two(self) -> Self {
            return Self::_core_int(Self::raw(self).next_power_of_two());
        }
        #[inline(always)]
        pub fn checked_next_power_of_two(self) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_next_power_of_two() {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn div_ceil(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).div_ceil(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn next_multiple_of(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).next_multiple_of(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn checked_next_multiple_of(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self)
                .checked_next_multiple_of(Self::raw(rhs))
            {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn carrying_add(
            self,
            rhs: Self,
            carry: bool,
        ) -> (Self, bool) {
            let (value, carry) =
                Self::raw(self).carrying_add(Self::raw(rhs), carry);
            return (Self::_core_int(value), carry);
        }
        #[inline(always)]
        pub fn borrowing_sub(
            self,
            rhs: Self,
            borrow: bool,
        ) -> (Self, bool) {
            let (value, borrow) =
                Self::raw(self).borrowing_sub(Self::raw(rhs), borrow);
            return (Self::_core_int(value), borrow);
        }
        #[inline(always)]
        pub fn carrying_mul(
            self,
            rhs: Self,
            carry: Self,
        ) -> (Self, Self) {
            let (low, high) =
                Self::raw(self).carrying_mul(Self::raw(rhs), Self::raw(carry));
            return (Self::_core_int(low), Self::_core_int(high));
        }
        #[inline(always)]
        pub fn carrying_mul_add(
            self,
            rhs: Self,
            carry: Self,
            add: Self,
        ) -> (Self, Self) {
            let (low, high) = Self::raw(self).carrying_mul_add(
                Self::raw(rhs),
                Self::raw(carry),
                Self::raw(add),
            );
            return (Self::_core_int(low), Self::_core_int(high));
        }
    }
    impl ::core::convert::Into<usize> for NonZeroSerial {
        #[inline(always)]
        fn into(self) -> usize {
            return NonZeroSerial::into_usize(self);
        }
    }
    impl ::core::convert::Into<u32> for NonZeroSerial {
        #[inline(always)]
        fn into(self) -> u32 {
            return NonZeroSerial::into_u32(self);
        }
    }
    impl ::core::convert::Into<u64> for NonZeroSerial {
        #[inline(always)]
        fn into(self) -> u64 {
            return NonZeroSerial::into_u64(self);
        }
    }
    impl ::core::convert::Into<u128> for NonZeroSerial {
        #[inline(always)]
        fn into(self) -> u128 {
            return NonZeroSerial::into_u128(self);
        }
    }
    impl ::core::convert::Into<i64> for NonZeroSerial {
        #[inline(always)]
        fn into(self) -> i64 {
            return NonZeroSerial::into_i64(self);
        }
    }
    impl ::core::convert::Into<i128> for NonZeroSerial {
        #[inline(always)]
        fn into(self) -> i128 {
            return NonZeroSerial::into_i128(self);
        }
    }
    impl ::core::convert::TryInto<isize> for NonZeroSerial {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<isize, Self::Error> {
            return NonZeroSerial::try_into_isize(self);
        }
    }
    impl ::core::convert::TryInto<u8> for NonZeroSerial {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u8, Self::Error> {
            return NonZeroSerial::try_into_u8(self);
        }
    }
    impl ::core::convert::TryInto<u16> for NonZeroSerial {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u16, Self::Error> {
            return NonZeroSerial::try_into_u16(self);
        }
    }
    impl ::core::convert::TryInto<i8> for NonZeroSerial {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i8, Self::Error> {
            return NonZeroSerial::try_into_i8(self);
        }
    }
    impl ::core::convert::TryInto<i16> for NonZeroSerial {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i16, Self::Error> {
            return NonZeroSerial::try_into_i16(self);
        }
    }
    impl ::core::convert::TryInto<i32> for NonZeroSerial {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i32, Self::Error> {
            return NonZeroSerial::try_into_i32(self);
        }
    }
    impl ::core::ops::Shr<usize> for NonZeroSerial {
        type Output = Self;
        #[inline(always)]
        fn shr(
            self,
            rhs: usize,
        ) -> Self::Output {
            return self._shr_wrapping(rhs);
        }
    }
    impl ::core::ops::Shl<usize> for NonZeroSerial {
        type Output = Self;
        #[inline(always)]
        fn shl(
            self,
            rhs: usize,
        ) -> Self::Output {
            return self._shl_wrapping(rhs);
        }
    }
    impl ::core::ops::ShrAssign<usize> for NonZeroSerial {
        #[inline(always)]
        fn shr_assign(
            &mut self,
            other: usize,
        ) {
            *self = self._shr_wrapping(other);
        }
    }
    impl ::core::ops::ShlAssign<usize> for NonZeroSerial {
        #[inline(always)]
        fn shl_assign(
            &mut self,
            other: usize,
        ) {
            *self = self._shl_wrapping(other);
        }
    }
    impl ::core::ops::BitAndAssign for NonZeroSerial {
        #[inline(always)]
        fn bitand_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._bitand(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::BitAndAssign<T> for NonZeroSerial
    where
        T: Bit + Seal,
    {
        #[inline(always)]
        fn bitand_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_non_zero_serial(rhs);
            *self = self._bitand(it);
        }
    }
    impl ::core::ops::AddAssign for NonZeroSerial {
        #[inline(always)]
        fn add_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._add_wrapping(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::AddAssign<T> for NonZeroSerial
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn add_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_non_zero_serial(rhs);
            *self = self._add_wrapping(it);
        }
    }
    impl ::core::ops::SubAssign for NonZeroSerial {
        #[inline(always)]
        fn sub_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._sub_wrapping(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::SubAssign<T> for NonZeroSerial
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn sub_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_non_zero_serial(rhs);
            *self = self._sub_wrapping(it);
        }
    }
    impl ::core::ops::MulAssign for NonZeroSerial {
        #[inline(always)]
        fn mul_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._mul_wrapping(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::MulAssign<T> for NonZeroSerial
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn mul_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_non_zero_serial(rhs);
            *self = self._mul_wrapping(it);
        }
    }
    impl ::core::ops::DivAssign for NonZeroSerial {
        #[inline(always)]
        fn div_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._div_wrapping(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::DivAssign<T> for NonZeroSerial
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn div_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_non_zero_serial(rhs);
            *self = self._div_wrapping(it);
        }
    }
    impl ::core::ops::RemAssign for NonZeroSerial {
        #[inline(always)]
        fn rem_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._rem_wrapping(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::RemAssign<T> for NonZeroSerial
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn rem_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_non_zero_serial(rhs);
            *self = self._rem_wrapping(it);
        }
    }
    impl ::core::ops::BitOrAssign for NonZeroSerial {
        #[inline(always)]
        fn bitor_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._bitor(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::BitOrAssign<T> for NonZeroSerial
    where
        T: Bit + Seal,
    {
        #[inline(always)]
        fn bitor_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_non_zero_serial(rhs);
            *self = self._bitor(it);
        }
    }
    impl ::core::fmt::Debug for NonZeroSerial {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            return f
                .debug_tuple("NonZeroSerial")
                .field(&((self.0).0).get())
                .finish();
        }
    }
    impl ::core::hash::Hash for NonZeroSerial {
        #[inline(always)]
        fn hash<H>(
            &self,
            state: &mut H,
        ) where
            H: ::core::hash::Hasher,
        {
            return ::core::hash::Hash::hash(&((self.0).0).get(), state);
        }
    }
    impl ::core::cmp::Eq for NonZeroSerial {}
    impl ::core::cmp::Ord for NonZeroSerial {
        #[inline(always)]
        fn cmp(
            &self,
            other: &Self,
        ) -> ::core::cmp::Ordering {
            return self.partial_cmp(other).unwrap();
        }
    }
    if !(::core::mem::size_of::<NonZeroSerial>()
        == ::core::mem::size_of::<u32>())
    {
        ::core::panic!("invalid integral storage size");
    };
    if !(::core::mem::align_of::<NonZeroSerial>()
        == ::core::mem::align_of::<u32>())
    {
        ::core::panic!("invalid integral storage alignment");
    };
    impl ::core::ops::Add for NonZeroSerial {
        type Output = Self;
        #[inline(always)]
        fn add(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._add_wrapping(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::Add<T> for NonZeroSerial
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn add(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_non_zero_serial(rhs);
            return self._add_wrapping(it);
        }
    }
    impl ::core::ops::Sub for NonZeroSerial {
        type Output = Self;
        #[inline(always)]
        fn sub(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._sub_wrapping(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::Sub<T> for NonZeroSerial
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn sub(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_non_zero_serial(rhs);
            return self._sub_wrapping(it);
        }
    }
    impl ::core::ops::Mul for NonZeroSerial {
        type Output = Self;
        #[inline(always)]
        fn mul(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._mul_wrapping(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::Mul<T> for NonZeroSerial
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn mul(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_non_zero_serial(rhs);
            return self._mul_wrapping(it);
        }
    }
    impl ::core::ops::Div for NonZeroSerial {
        type Output = Self;
        #[inline(always)]
        fn div(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._div_wrapping(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::Div<T> for NonZeroSerial
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn div(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_non_zero_serial(rhs);
            return self._div_wrapping(it);
        }
    }
    impl ::core::ops::Rem for NonZeroSerial {
        type Output = Self;
        #[inline(always)]
        fn rem(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._rem_wrapping(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::Rem<T> for NonZeroSerial
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn rem(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_non_zero_serial(rhs);
            return self._rem_wrapping(it);
        }
    }
    impl ::core::ops::BitAnd for NonZeroSerial {
        type Output = Self;
        #[inline(always)]
        fn bitand(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._bitand(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::BitAnd<T> for NonZeroSerial
    where
        T: Bit + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn bitand(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_non_zero_serial(rhs);
            return self._bitand(it);
        }
    }
    impl ::core::ops::BitOr for NonZeroSerial {
        type Output = Self;
        #[inline(always)]
        fn bitor(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._bitor(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::BitOr<T> for NonZeroSerial
    where
        T: Bit + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn bitor(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_non_zero_serial(rhs);
            return self._bitor(it);
        }
    }
    impl ::core::ops::BitXor for NonZeroSerial {
        type Output = Self;
        #[inline(always)]
        fn bitxor(
            self,
            rhs: Self,
        ) -> Self::Output {
            return self._bitxor(((rhs.0).0).get());
        }
    }
    impl ::core::ops::BitXorAssign for NonZeroSerial {
        #[inline(always)]
        fn bitxor_assign(
            &mut self,
            rhs: Self,
        ) {
            *self = self._bitxor(((rhs.0).0).get());
        }
    }
    impl<T> ::core::ops::BitXor<T> for NonZeroSerial
    where
        T: Bit + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn bitxor(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_non_zero_serial(rhs);
            return self._bitxor(it);
        }
    }
    impl<T> ::core::ops::BitXorAssign<T> for NonZeroSerial
    where
        T: Bit + Seal,
    {
        #[inline(always)]
        fn bitxor_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_non_zero_serial(rhs);
            *self = self._bitxor(it);
        }
    }
    impl ::core::ops::Not for NonZeroSerial {
        type Output = Self;
        #[inline(always)]
        fn not(self) -> Self::Output {
            return self._not();
        }
    }
    impl ::core::fmt::Binary for NonZeroSerial {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::Binary::fmt(&raw, f);
        }
    }
    impl ::core::fmt::Octal for NonZeroSerial {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::Octal::fmt(&raw, f);
        }
    }
    impl ::core::fmt::LowerHex for NonZeroSerial {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::LowerHex::fmt(&raw, f);
        }
    }
    impl ::core::fmt::UpperHex for NonZeroSerial {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::UpperHex::fmt(&raw, f);
        }
    }
    impl<T> ::core::cmp::PartialEq<T> for NonZeroSerial
    where
        T: Rel + Seal + ::core::marker::Copy,
    {
        #[inline(always)]
        fn eq(
            &self,
            rhs: &T,
        ) -> bool {
            let lhs = Self::raw(*self);
            let rhs = Seal::conv_non_zero_serial(*rhs);
            return lhs == rhs;
        }
    }
    impl<T> ::core::cmp::PartialOrd<T> for NonZeroSerial
    where
        T: Rel + Seal + ::core::marker::Copy,
    {
        #[inline(always)]
        fn partial_cmp(
            &self,
            rhs: &T,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let lhs = Self::raw(*self);
            let rhs = Seal::conv_non_zero_serial(*rhs);
            return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
        }
    }
    impl NonZeroSerial {
        #[inline(always)]
        #[allow(private_bounds)]
        pub fn of<T>(it: T) -> NonZeroSerial
        where
            T: Make,
        {
            return <T as Make>::make(it);
        }
        #[must_use]
        #[inline(always)]
        pub const fn raw(self) -> u32 {
            return ((self.0).0).get();
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
            return if value >= (1) {
                ::core::result::Result::Ok(Self(::core::num::Wrapping(
                    match ::core::num::NonZeroU32::new(value) {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => {
                            ::core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    },
                )))
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
                    ::core::panic!(
                        "mapping operation produced an invalid value"
                    )
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
        pub(self) fn _add_wrapping(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it =
                (::core::num::Wrapping(lhs) + ::core::num::Wrapping(rhs)).0;
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
        pub(self) fn _sub_wrapping(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it =
                (::core::num::Wrapping(lhs) - ::core::num::Wrapping(rhs)).0;
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
        pub(self) fn _mul_wrapping(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it =
                (::core::num::Wrapping(lhs) * ::core::num::Wrapping(rhs)).0;
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
        pub(self) fn _div_wrapping(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it =
                (::core::num::Wrapping(lhs) / ::core::num::Wrapping(rhs)).0;
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
        pub(self) fn _rem_wrapping(
            &self,
            rhs: u32,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it =
                (::core::num::Wrapping(lhs) % ::core::num::Wrapping(rhs)).0;
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
            rhs: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_shr(rhs as u32);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) fn _shr_wrapping(
            &self,
            rhs: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = (::core::num::Wrapping(lhs) >> rhs).0;
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) const fn _shl(
            &self,
            rhs: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = lhs.wrapping_shl(rhs as u32);
            return Self::_unchecked(it);
        }
        #[inline(always)]
        pub(self) fn _shl_wrapping(
            &self,
            rhs: usize,
        ) -> Self {
            let lhs = Self::raw(*self);
            let it = (::core::num::Wrapping(lhs) << rhs).0;
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
        pub fn try_make(it: u32) -> Result<Self, u32> {
            return if it >= (1) {
                return ::core::result::Result::Ok(Self(
                    ::core::num::Wrapping(
                        match ::core::num::NonZeroU32::new(it) {
                            ::core::option::Option::Some(value) => value,
                            ::core::option::Option::None => {
                                ::core::panic!(
                                    "zero cannot be stored by this integral type"
                                )
                            }
                        },
                    ),
                ));
            }
            else {
                ::core::result::Result::Err(it)
            };
        }
        #[must_use]
        #[inline(always)]
        pub(self) const fn _unchecked(it: u32) -> Self {
            if it >= (1) {
                return Self(::core::num::Wrapping(
                    match ::core::num::NonZeroU32::new(it) {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => {
                            ::core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    },
                ));
            }
            else {
                ::core::panic!("invalid value");
            };
        }
        #[inline(always)]
        #[track_caller]
        pub(self) fn _core_int(value: u32) -> Self {
            if value >= (1) {
                return Self(::core::num::Wrapping(
                    match ::core::num::NonZeroU32::new(value) {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => {
                            ::core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    },
                ));
            }
            else {
                ::core::panic!("operation produced an invalid value");
            }
        }
        #[inline(always)]
        pub(self) fn _core_int_checked(
            value: u32
        ) -> ::core::option::Option<Self> {
            return if value >= (1) {
                ::core::option::Option::Some(Self(::core::num::Wrapping(
                    match ::core::num::NonZeroU32::new(value) {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => {
                            ::core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    },
                )))
            }
            else {
                ::core::option::Option::None
            };
        }
    }
    #[allow(clippy::unnecessary_cast)]
    impl NonZeroSerial {
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
            return ((Self::raw(self) >> (8 * 0usize)) & (0xFF as u32)) as u8;
        }
        #[must_use]
        #[inline(always)]
        pub const fn byte1(self) -> u8 {
            return ((Self::raw(self) >> (8 * 1usize)) & (0xFF as u32)) as u8;
        }
        #[must_use]
        #[inline(always)]
        pub const fn byte2(self) -> u8 {
            return ((Self::raw(self) >> (8 * 2usize)) & (0xFF as u32)) as u8;
        }
        #[must_use]
        #[inline(always)]
        pub const fn byte3(self) -> u8 {
            return ((Self::raw(self) >> (8 * 3usize)) & (0xFF as u32)) as u8;
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
            return ((Self::raw(self) >> (32 * 0usize)) & (0xFFFFFFFF as u32))
                as u32;
        }
    }
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

    let maximum = WrappingCounter::make(u32::MAX);
    let wrapped = maximum + WrappingCounter::make(1);
    assert_eq!(wrapped.raw(), 0);
    assert!(WrappingCounter::make(1) < WrappingCounter::make(2));

    let serial = NonZeroSerial::try_make(7).unwrap();
    assert_eq!(serial.raw(), 7);
    assert_eq!(NonZeroSerial::try_make(0), Err(0));
}
