// AUTO-GENERATED VIA typekin-unexpand, ANY MANUAL MODIFICATIONS WILL BE LOST IF THE CODE IS RE-GENERATED
#![allow(clippy::needless_return)]

#[repr(transparent)]
#[derive(
    :: core :: clone :: Clone,
    :: core :: cmp :: Eq,
    :: core :: cmp :: Ord,
    :: core :: fmt :: Debug,
    :: core :: marker :: Copy,
)]
struct PageHeader(u32);
#[allow(dead_code)]
#[allow(unused_qualifications)]
const _: () = {
    impl Into<usize> for PageHeader {
        #[inline(always)]
        fn into(self) -> usize {
            return PageHeader::into_usize(self);
        }
    }
    impl Into<u32> for PageHeader {
        #[inline(always)]
        fn into(self) -> u32 {
            return PageHeader::into_u32(self);
        }
    }
    impl Into<u64> for PageHeader {
        #[inline(always)]
        fn into(self) -> u64 {
            return PageHeader::into_u64(self);
        }
    }
    impl Into<u128> for PageHeader {
        #[inline(always)]
        fn into(self) -> u128 {
            return PageHeader::into_u128(self);
        }
    }
    impl Into<i64> for PageHeader {
        #[inline(always)]
        fn into(self) -> i64 {
            return PageHeader::into_i64(self);
        }
    }
    impl Into<i128> for PageHeader {
        #[inline(always)]
        fn into(self) -> i128 {
            return PageHeader::into_i128(self);
        }
    }
    impl TryInto<isize> for PageHeader {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<isize, Self::Error> {
            return PageHeader::try_into_isize(self);
        }
    }
    impl TryInto<u8> for PageHeader {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<u8, Self::Error> {
            return PageHeader::try_into_u8(self);
        }
    }
    impl TryInto<u16> for PageHeader {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<u16, Self::Error> {
            return PageHeader::try_into_u16(self);
        }
    }
    impl TryInto<i8> for PageHeader {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<i8, Self::Error> {
            return PageHeader::try_into_i8(self);
        }
    }
    impl TryInto<i16> for PageHeader {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<i16, Self::Error> {
            return PageHeader::try_into_i16(self);
        }
    }
    impl TryInto<i32> for PageHeader {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<i32, Self::Error> {
            return PageHeader::try_into_i32(self);
        }
    }
    impl ::core::ops::Shr<usize> for PageHeader {
        type Output = Self;
        #[inline(always)]
        fn shr(
            self,
            rhs: usize,
        ) -> Self::Output {
            return self._shr(rhs);
        }
    }
    impl ::core::ops::Shl<usize> for PageHeader {
        type Output = Self;
        #[inline(always)]
        fn shl(
            self,
            rhs: usize,
        ) -> Self::Output {
            return self._shl(rhs);
        }
    }
    impl ::core::ops::ShrAssign<usize> for PageHeader {
        #[inline(always)]
        fn shr_assign(
            &mut self,
            other: usize,
        ) {
            *self = self._shr(other);
        }
    }
    impl ::core::ops::ShlAssign<usize> for PageHeader {
        #[inline(always)]
        fn shl_assign(
            &mut self,
            other: usize,
        ) {
            *self = self._shl(other);
        }
    }
    impl<T> ::core::ops::BitAndAssign<T> for PageHeader
    where
        T: FriendBit,
    {
        #[inline(always)]
        fn bitand_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_page_header(&rhs);
            *self = self._bitand(it);
        }
    }
    impl<T> ::core::ops::AddAssign<T> for PageHeader
    where
        T: FriendMath,
    {
        #[inline(always)]
        fn add_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_page_header(&rhs);
            *self = self._add(it);
        }
    }
    impl<T> ::core::ops::SubAssign<T> for PageHeader
    where
        T: FriendMath,
    {
        #[inline(always)]
        fn sub_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_page_header(&rhs);
            *self = self._sub(it);
        }
    }
    impl<T> ::core::ops::MulAssign<T> for PageHeader
    where
        T: FriendMath,
    {
        #[inline(always)]
        fn mul_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_page_header(&rhs);
            *self = self._mul(it);
        }
    }
    impl<T> ::core::ops::DivAssign<T> for PageHeader
    where
        T: FriendMath,
    {
        #[inline(always)]
        fn div_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_page_header(&rhs);
            *self = self._div(it);
        }
    }
    impl<T> ::core::ops::RemAssign<T> for PageHeader
    where
        T: FriendMath,
    {
        #[inline(always)]
        fn rem_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_page_header(&rhs);
            *self = self._rem(it);
        }
    }
    impl<T> ::core::ops::BitOrAssign<T> for PageHeader
    where
        T: FriendBit,
    {
        #[inline(always)]
        fn bitor_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_page_header(&rhs);
            *self = self._bitor(it);
        }
    }
    if !(::core::mem::size_of::<PageHeader>() == ::core::mem::size_of::<u32>())
    {
        {
            panic!("invalid memory layout: #ty(#el) != #el");
        };
    };
    impl<T> ::core::ops::Add<T> for PageHeader
    where
        T: FriendMath,
    {
        type Output = Self;
        #[inline(always)]
        fn add(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_page_header(&rhs);
            return self._add(it);
        }
    }
    impl<T> ::core::ops::Sub<T> for PageHeader
    where
        T: FriendMath,
    {
        type Output = Self;
        #[inline(always)]
        fn sub(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_page_header(&rhs);
            return self._sub(it);
        }
    }
    impl<T> ::core::ops::Mul<T> for PageHeader
    where
        T: FriendMath,
    {
        type Output = Self;
        #[inline(always)]
        fn mul(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_page_header(&rhs);
            return self._mul(it);
        }
    }
    impl<T> ::core::ops::Div<T> for PageHeader
    where
        T: FriendMath,
    {
        type Output = Self;
        #[inline(always)]
        fn div(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_page_header(&rhs);
            return self._div(it);
        }
    }
    impl<T> ::core::ops::Rem<T> for PageHeader
    where
        T: FriendMath,
    {
        type Output = Self;
        #[inline(always)]
        fn rem(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_page_header(&rhs);
            return self._rem(it);
        }
    }
    impl<T> ::core::ops::BitAnd<T> for PageHeader
    where
        T: FriendBit,
    {
        type Output = Self;
        #[inline(always)]
        fn bitand(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_page_header(&rhs);
            return self._bitand(it);
        }
    }
    impl<T> ::core::ops::BitOr<T> for PageHeader
    where
        T: FriendBit,
    {
        type Output = Self;
        #[inline(always)]
        fn bitor(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_page_header(&rhs);
            return self._bitor(it);
        }
    }
    impl<T> ::core::ops::BitXor<T> for PageHeader
    where
        T: FriendBit,
    {
        type Output = Self;
        #[inline(always)]
        fn bitxor(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_page_header(&rhs);
            return self._bitxor(it);
        }
    }
    impl<T> ::core::ops::BitXorAssign<T> for PageHeader
    where
        T: FriendBit,
    {
        #[inline(always)]
        fn bitxor_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_page_header(&rhs);
            *self = self._bitxor(it);
        }
    }
    impl ::core::ops::Not for PageHeader {
        type Output = Self;
        #[inline(always)]
        fn not(self) -> Self::Output {
            return self._not();
        }
    }
    impl Seal for PageHeader {
        #[inline(always)]
        fn conv_page_header(&self) -> u32 {
            return Self::raw(*self);
        }
    }
    impl ::core::fmt::Binary for PageHeader {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::Binary::fmt(&raw, f);
        }
    }
    impl ::core::fmt::Octal for PageHeader {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::Octal::fmt(&raw, f);
        }
    }
    impl ::core::fmt::LowerHex for PageHeader {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::LowerHex::fmt(&raw, f);
        }
    }
    impl ::core::fmt::UpperHex for PageHeader {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::UpperHex::fmt(&raw, f);
        }
    }
    impl Seal for PageData {
        #[inline(always)]
        fn conv_page_header(&self) -> u32 {
            return PageData::to_header(*self);
        }
    }
    impl Seal for PageId {
        #[inline(always)]
        fn conv_page_header(&self) -> u32 {
            return id_to_header(*self);
        }
    }
    impl Seal for PageState {
        #[inline(always)]
        fn conv_page_header(&self) -> u32 {
            return state_to_header(*self);
        }
    }
    impl Seal for u32 {
        #[inline(always)]
        fn conv_page_header(&self) -> u32 {
            let it: u32 = *self;
            return it;
        }
    }
    impl FriendMake for PageData {}
    impl FriendRel for PageData {}
    impl FriendBit for PageData {}
    impl FriendMath for PageData {}
    impl FriendMake for PageId {}
    impl FriendRel for PageId {}
    impl FriendBit for PageId {}
    impl FriendMath for PageId {}
    impl FriendMake for PageState {}
    impl FriendRel for PageState {}
    impl FriendBit for PageState {}
    impl FriendMath for PageState {}
    impl FriendMake for PageHeader {}
    impl FriendRel for PageHeader {}
    impl FriendBit for PageHeader {}
    impl FriendMath for PageHeader {}
    impl FriendMake for u32 {}
    impl FriendRel for u32 {}
    impl FriendBit for u32 {}
    impl FriendMath for u32 {}
    impl<T> ::core::cmp::PartialEq<T> for PageHeader
    where
        T: FriendRel,
    {
        #[inline(always)]
        fn eq(
            &self,
            rhs: &T,
        ) -> bool {
            let lhs = Self::raw(*self);
            let rhs = Seal::conv_page_header(rhs);
            return lhs == rhs;
        }
    }
    impl<T> ::core::cmp::PartialOrd<T> for PageHeader
    where
        T: FriendRel,
    {
        #[inline(always)]
        fn partial_cmp(
            &self,
            rhs: &T,
        ) -> Option<::core::cmp::Ordering> {
            let lhs = Self::raw(*self);
            let rhs = Seal::conv_page_header(rhs);
            return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
        }
    }
    trait Seal {
        fn conv_page_header(&self) -> u32;
    }
    impl<T> Seal for &T
    where
        T: Seal,
    {
        #[inline(always)]
        fn conv_page_header(&self) -> u32 {
            return Seal::conv_page_header(&**self);
        }
    }
    impl<T> Seal for &mut T
    where
        T: Seal,
    {
        #[inline(always)]
        fn conv_page_header(&self) -> u32 {
            return Seal::conv_page_header(&**self);
        }
    }
    trait FriendMake: Seal {}
    impl<T> FriendMake for &T where T: FriendMake {}
    impl<T> FriendMake for &mut T where T: FriendMake {}
    trait FriendMath: Seal {}
    impl<T> FriendMath for &T where T: FriendMath {}
    impl<T> FriendMath for &mut T where T: FriendMath {}
    trait FriendBit: Seal {}
    impl<T> FriendBit for &T where T: FriendBit {}
    impl<T> FriendBit for &mut T where T: FriendBit {}
    trait FriendRel: Seal {}
    impl<T> FriendRel for &T where T: FriendRel {}
    impl<T> FriendRel for &mut T where T: FriendRel {}
    impl PageHeader {
        #[inline(always)]
        #[allow(private_bounds)]
        pub fn of<T>(it: T) -> PageHeader
        where
            T: FriendMake,
        {
            let this = Seal::conv_page_header(&it);
            return Self::_unchecked(this);
        }
        #[inline(always)]
        pub fn make(it: u32) -> PageHeader {
            return PageHeader::of(it);
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
    impl PageHeader {
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
struct PageId(u8);
#[repr(transparent)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
struct PageState(u8);
#[repr(transparent)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
struct PageData(u8);
fn id_to_header(it: PageId) -> u32 {
    let bits = it.0 as u32;
    return bits << 24;
}
fn state_to_header(it: PageState) -> u32 {
    let bits = it.0 as u32;
    return bits << 8;
}
impl PageData {
    fn to_header(self) -> u32 {
        return self.0 as u32;
    }
}
fn main() {
    let id = PageId(0b0000_0101);
    let state = PageState(0b1010_1010);
    let data = PageData(0b1111_1111);
    let mut header = PageHeader::of(0);
    header |= id;
    header |= state;
    header |= data;

    let expected = 0b00000101_10101010_00000000_11111111;
    //               ^..ID..^ ^......STATE....^ ^.DATA.^
    
    assert_eq!(header.raw(), expected);
}
