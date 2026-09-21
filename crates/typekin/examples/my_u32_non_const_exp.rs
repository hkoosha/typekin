// AUTO-GENERATED VIA typekin-unexpand, ANY MANUAL MODIFICATIONS WILL BE LOST IF THE CODE IS RE-GENERATED
#![allow(clippy::needless_return)]

mod subject {
    #[repr(transparent)]
    #[derive(
        :: core :: clone :: Clone,
        :: core :: cmp :: Eq,
        :: core :: cmp :: Ord,
        :: core :: fmt :: Debug,
        :: core :: marker :: Copy,
    )]
    pub struct MyExample(u32);
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    const _: () = {
        trait Seal {
            fn conv_my_example(&self) -> u32;
        }
        trait Make: Seal {}
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Relation: Seal {}
        impl Seal for MyExample {
            #[inline(always)]
            fn conv_my_example(&self) -> u32 {
                return Self::raw(*self);
            }
        }
        impl Bit for MyExample {}
        impl Make for MyExample {}
        impl Math for MyExample {}
        impl Relation for MyExample {}
        impl Seal for i16 {
            #[inline(always)]
            fn conv_my_example(&self) -> u32 {
                return *self as u32;
            }
        }
        impl Make for i16 {}
        impl Seal for u16 {
            #[inline(always)]
            fn conv_my_example(&self) -> u32 {
                return *self as u32;
            }
        }
        impl Make for u16 {}
        impl Seal for u32 {
            #[inline(always)]
            fn conv_my_example(&self) -> u32 {
                return *self as u32;
            }
        }
        impl Bit for u32 {}
        impl Make for u32 {}
        impl Math for u32 {}
        impl Relation for u32 {}
        impl Seal for u8 {
            #[inline(always)]
            fn conv_my_example(&self) -> u32 {
                return *self as u32;
            }
        }
        impl Make for u8 {}
        impl<T> Seal for &T
        where
            T: Seal,
        {
            #[inline(always)]
            fn conv_my_example(&self) -> u32 {
                return Seal::conv_my_example(&**self);
            }
        }
        impl<T> Seal for &mut T
        where
            T: Seal,
        {
            #[inline(always)]
            fn conv_my_example(&self) -> u32 {
                return Seal::conv_my_example(&**self);
            }
        }
        impl<T> Make for &T where T: Make {}
        impl<T> Make for &mut T where T: Make {}
        impl<T> Math for &T where T: Math {}
        impl<T> Math for &mut T where T: Math {}
        impl<T> Bit for &T where T: Bit {}
        impl<T> Bit for &mut T where T: Bit {}
        impl<T> Relation for &T where T: Relation {}
        impl<T> Relation for &mut T where T: Relation {}
        impl Into<usize> for MyExample {
            #[inline(always)]
            fn into(self) -> usize {
                return MyExample::into_usize(self);
            }
        }
        impl Into<u32> for MyExample {
            #[inline(always)]
            fn into(self) -> u32 {
                return MyExample::into_u32(self);
            }
        }
        impl Into<u64> for MyExample {
            #[inline(always)]
            fn into(self) -> u64 {
                return MyExample::into_u64(self);
            }
        }
        impl Into<u128> for MyExample {
            #[inline(always)]
            fn into(self) -> u128 {
                return MyExample::into_u128(self);
            }
        }
        impl Into<i64> for MyExample {
            #[inline(always)]
            fn into(self) -> i64 {
                return MyExample::into_i64(self);
            }
        }
        impl Into<i128> for MyExample {
            #[inline(always)]
            fn into(self) -> i128 {
                return MyExample::into_i128(self);
            }
        }
        impl TryInto<isize> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<isize, Self::Error> {
                return MyExample::try_into_isize(self);
            }
        }
        impl TryInto<u8> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u8, Self::Error> {
                return MyExample::try_into_u8(self);
            }
        }
        impl TryInto<u16> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u16, Self::Error> {
                return MyExample::try_into_u16(self);
            }
        }
        impl TryInto<i8> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i8, Self::Error> {
                return MyExample::try_into_i8(self);
            }
        }
        impl TryInto<i16> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i16, Self::Error> {
                return MyExample::try_into_i16(self);
            }
        }
        impl TryInto<i32> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i32, Self::Error> {
                return MyExample::try_into_i32(self);
            }
        }
        impl ::core::ops::Shr<usize> for MyExample {
            type Output = Self;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        impl ::core::ops::Shl<usize> for MyExample {
            type Output = Self;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        impl ::core::ops::ShrAssign<usize> for MyExample {
            #[inline(always)]
            fn shr_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shr(other);
            }
        }
        impl ::core::ops::ShlAssign<usize> for MyExample {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shl(other);
            }
        }
        impl<T> ::core::ops::BitAndAssign<T> for MyExample
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_example(&rhs);
                *self = self._bitand(it);
            }
        }
        impl<T> ::core::ops::AddAssign<T> for MyExample
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_example(&rhs);
                *self = self._add(it);
            }
        }
        impl<T> ::core::ops::SubAssign<T> for MyExample
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_example(&rhs);
                *self = self._sub(it);
            }
        }
        impl<T> ::core::ops::MulAssign<T> for MyExample
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_example(&rhs);
                *self = self._mul(it);
            }
        }
        impl<T> ::core::ops::DivAssign<T> for MyExample
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_example(&rhs);
                *self = self._div(it);
            }
        }
        impl<T> ::core::ops::RemAssign<T> for MyExample
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_example(&rhs);
                *self = self._rem(it);
            }
        }
        impl<T> ::core::ops::BitOrAssign<T> for MyExample
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_example(&rhs);
                *self = self._bitor(it);
            }
        }
        if !(::core::mem::size_of::<MyExample>()
            == ::core::mem::size_of::<u32>())
        {
            {
                panic!("invalid memory layout: #ty(#el) != #el");
            };
        };
        impl<T> ::core::ops::Add<T> for MyExample
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_example(&rhs);
                return self._add(it);
            }
        }
        impl<T> ::core::ops::Sub<T> for MyExample
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_example(&rhs);
                return self._sub(it);
            }
        }
        impl<T> ::core::ops::Mul<T> for MyExample
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_example(&rhs);
                return self._mul(it);
            }
        }
        impl<T> ::core::ops::Div<T> for MyExample
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_example(&rhs);
                return self._div(it);
            }
        }
        impl<T> ::core::ops::Rem<T> for MyExample
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_example(&rhs);
                return self._rem(it);
            }
        }
        impl<T> ::core::ops::BitAnd<T> for MyExample
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_example(&rhs);
                return self._bitand(it);
            }
        }
        impl<T> ::core::ops::BitOr<T> for MyExample
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_example(&rhs);
                return self._bitor(it);
            }
        }
        impl<T> ::core::ops::BitXor<T> for MyExample
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_example(&rhs);
                return self._bitxor(it);
            }
        }
        impl<T> ::core::ops::BitXorAssign<T> for MyExample
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_example(&rhs);
                *self = self._bitxor(it);
            }
        }
        impl ::core::ops::Not for MyExample {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        impl ::core::fmt::Binary for MyExample {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl ::core::fmt::Octal for MyExample {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl ::core::fmt::LowerHex for MyExample {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl ::core::fmt::UpperHex for MyExample {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        impl<T> ::core::cmp::PartialEq<T> for MyExample
        where
            T: Relation + Seal,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_my_example(rhs);
                return lhs == rhs;
            }
        }
        impl<T> ::core::cmp::PartialOrd<T> for MyExample
        where
            T: Relation + Seal,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> Option<::core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_my_example(rhs);
                return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
            }
        }
        impl MyExample {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> MyExample
            where
                T: Make + Seal,
            {
                let this = Seal::conv_my_example(&it);
                return Self::_unchecked(this);
            }
            #[inline(always)]
            pub fn make(it: u32) -> MyExample {
                return MyExample::of(it);
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
        impl MyExample {
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
}
type Subject = subject::MyExample;
fn main() {
    let lhs = 0b1101u32;
    let rhs = 0b0110u32;
    let lhs = Subject::of(lhs);
    let rhs = Subject::of(rhs);
    {
        print!("{0:?}\n", lhs + rhs);
    };
}
