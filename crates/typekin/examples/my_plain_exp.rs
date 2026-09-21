// AUTO-GENERATED VIA typekin-unexpand, ANY MANUAL MODIFICATIONS WILL BE LOST IF THE CODE IS RE-GENERATED
#![allow(clippy::needless_return)]
#![allow(dead_code)]
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]
mod subject {
    #[repr(transparent)]
    #[derive(:: core :: fmt :: Debug, :: core :: marker :: Copy)]
    #[derive_const(::core::clone::Clone)]
    pub struct MyPlain(i64);
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    const _: () = {
        const trait Seal {
            fn conv_my_plain(&self) -> i64;
        }
        const trait Make: [const] Seal {}
        const trait Math: [const] Seal {}
        const trait Bit: [const] Seal {}
        const trait Relation: [const] Seal {}
        const impl Seal for MyPlain {
            #[inline(always)]
            fn conv_my_plain(&self) -> i64 {
                return Self::raw(*self);
            }
        }
        const impl Bit for MyPlain {}
        const impl Make for MyPlain {}
        const impl Math for MyPlain {}
        const impl Relation for MyPlain {}
        const impl Seal for i32 {
            #[inline(always)]
            fn conv_my_plain(&self) -> i64 {
                return *self as i64;
            }
        }
        const impl Make for i32 {}
        const impl Seal for i64 {
            #[inline(always)]
            fn conv_my_plain(&self) -> i64 {
                return *self as i64;
            }
        }
        const impl Bit for i64 {}
        const impl Make for i64 {}
        const impl Math for i64 {}
        const impl Relation for i64 {}
        const impl Seal for i8 {
            #[inline(always)]
            fn conv_my_plain(&self) -> i64 {
                return *self as i64;
            }
        }
        const impl Make for i8 {}
        const impl Seal for isize {
            #[inline(always)]
            fn conv_my_plain(&self) -> i64 {
                return *self as i64;
            }
        }
        const impl Make for isize {}
        const impl Seal for u16 {
            #[inline(always)]
            fn conv_my_plain(&self) -> i64 {
                return *self as i64;
            }
        }
        const impl Make for u16 {}
        const impl Seal for u32 {
            #[inline(always)]
            fn conv_my_plain(&self) -> i64 {
                return *self as i64;
            }
        }
        const impl Make for u32 {}
        const impl Seal for u8 {
            #[inline(always)]
            fn conv_my_plain(&self) -> i64 {
                return *self as i64;
            }
        }
        const impl Make for u8 {}
        const impl<T> Seal for &T
        where
            T: [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn conv_my_plain(&self) -> i64 {
                return Seal::conv_my_plain(&**self);
            }
        }
        const impl<T> Seal for &mut T
        where
            T: [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn conv_my_plain(&self) -> i64 {
                return Seal::conv_my_plain(&**self);
            }
        }
        const impl<T> Make for &T where
            T: [const] Make + [const] ::core::marker::Destruct
        {
        }
        const impl<T> Make for &mut T where
            T: [const] Make + [const] ::core::marker::Destruct
        {
        }
        const impl<T> Math for &T where
            T: [const] Math + [const] ::core::marker::Destruct
        {
        }
        const impl<T> Math for &mut T where
            T: [const] Math + [const] ::core::marker::Destruct
        {
        }
        const impl<T> Bit for &T where T: [const] Bit + [const] ::core::marker::Destruct {}
        const impl<T> Bit for &mut T where
            T: [const] Bit + [const] ::core::marker::Destruct
        {
        }
        const impl<T> Relation for &T where
            T: [const] Relation + [const] ::core::marker::Destruct
        {
        }
        const impl<T> Relation for &mut T where
            T: [const] Relation + [const] ::core::marker::Destruct
        {
        }
        const impl ::core::convert::Into<isize> for MyPlain {
            #[inline(always)]
            fn into(self) -> isize {
                return MyPlain::into_isize(self);
            }
        }
        const impl ::core::convert::Into<i64> for MyPlain {
            #[inline(always)]
            fn into(self) -> i64 {
                return MyPlain::into_i64(self);
            }
        }
        const impl ::core::convert::Into<i128> for MyPlain {
            #[inline(always)]
            fn into(self) -> i128 {
                return MyPlain::into_i128(self);
            }
        }
        const impl ::core::convert::TryInto<usize> for MyPlain {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<usize, Self::Error> {
                return MyPlain::try_into_usize(self);
            }
        }
        const impl ::core::convert::TryInto<u8> for MyPlain {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u8, Self::Error> {
                return MyPlain::try_into_u8(self);
            }
        }
        const impl ::core::convert::TryInto<u16> for MyPlain {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u16, Self::Error> {
                return MyPlain::try_into_u16(self);
            }
        }
        const impl ::core::convert::TryInto<u32> for MyPlain {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u32, Self::Error> {
                return MyPlain::try_into_u32(self);
            }
        }
        const impl ::core::convert::TryInto<u64> for MyPlain {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u64, Self::Error> {
                return MyPlain::try_into_u64(self);
            }
        }
        const impl ::core::convert::TryInto<u128> for MyPlain {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u128, Self::Error> {
                return MyPlain::try_into_u128(self);
            }
        }
        const impl ::core::convert::TryInto<i8> for MyPlain {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i8, Self::Error> {
                return MyPlain::try_into_i8(self);
            }
        }
        const impl ::core::convert::TryInto<i16> for MyPlain {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i16, Self::Error> {
                return MyPlain::try_into_i16(self);
            }
        }
        const impl ::core::convert::TryInto<i32> for MyPlain {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i32, Self::Error> {
                return MyPlain::try_into_i32(self);
            }
        }
        const impl ::core::ops::Shr<usize> for MyPlain {
            type Output = Self;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        const impl ::core::ops::Shl<usize> for MyPlain {
            type Output = Self;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        const impl ::core::ops::ShrAssign<usize> for MyPlain {
            #[inline(always)]
            fn shr_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shr(other);
            }
        }
        const impl ::core::ops::ShlAssign<usize> for MyPlain {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shl(other);
            }
        }
        const impl<T> ::core::ops::BitAndAssign<T> for MyPlain
        where
            T: Bit + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_plain(&rhs);
                *self = self._bitand(it);
            }
        }
        const impl<T> ::core::ops::AddAssign<T> for MyPlain
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_plain(&rhs);
                *self = self._add(it);
            }
        }
        const impl<T> ::core::ops::SubAssign<T> for MyPlain
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_plain(&rhs);
                *self = self._sub(it);
            }
        }
        const impl<T> ::core::ops::MulAssign<T> for MyPlain
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_plain(&rhs);
                *self = self._mul(it);
            }
        }
        const impl<T> ::core::ops::DivAssign<T> for MyPlain
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_plain(&rhs);
                *self = self._div(it);
            }
        }
        const impl<T> ::core::ops::RemAssign<T> for MyPlain
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_plain(&rhs);
                *self = self._rem(it);
            }
        }
        const impl<T> ::core::ops::BitOrAssign<T> for MyPlain
        where
            T: Bit + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_plain(&rhs);
                *self = self._bitor(it);
            }
        }
        const impl ::core::cmp::Eq for MyPlain {}
        const impl ::core::cmp::Ord for MyPlain {
            #[inline(always)]
            fn cmp(
                &self,
                other: &Self,
            ) -> ::core::cmp::Ordering {
                return self.partial_cmp(other).unwrap();
            }
        }
        if !(::core::mem::size_of::<MyPlain>() == ::core::mem::size_of::<i64>())
        {
            {
                panic!("invalid memory layout: #ty(#el) != #el");
            };
        };
        const impl<T> ::core::ops::Add<T> for MyPlain
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_plain(&rhs);
                return self._add(it);
            }
        }
        const impl<T> ::core::ops::Sub<T> for MyPlain
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_plain(&rhs);
                return self._sub(it);
            }
        }
        const impl<T> ::core::ops::Mul<T> for MyPlain
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_plain(&rhs);
                return self._mul(it);
            }
        }
        const impl<T> ::core::ops::Div<T> for MyPlain
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_plain(&rhs);
                return self._div(it);
            }
        }
        const impl<T> ::core::ops::Rem<T> for MyPlain
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_plain(&rhs);
                return self._rem(it);
            }
        }
        const impl<T> ::core::ops::BitAnd<T> for MyPlain
        where
            T: Bit + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_plain(&rhs);
                return self._bitand(it);
            }
        }
        const impl<T> ::core::ops::BitOr<T> for MyPlain
        where
            T: Bit + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_plain(&rhs);
                return self._bitor(it);
            }
        }
        const impl<T> ::core::ops::BitXor<T> for MyPlain
        where
            T: Bit + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_plain(&rhs);
                return self._bitxor(it);
            }
        }
        const impl<T> ::core::ops::BitXorAssign<T> for MyPlain
        where
            T: Bit + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_plain(&rhs);
                *self = self._bitxor(it);
            }
        }
        const impl ::core::ops::Not for MyPlain {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        impl ::core::fmt::Binary for MyPlain {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl ::core::fmt::Octal for MyPlain {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl ::core::fmt::LowerHex for MyPlain {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl ::core::fmt::UpperHex for MyPlain {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        const impl<T> ::core::cmp::PartialEq<T> for MyPlain
        where
            T: Relation + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_my_plain(rhs);
                return lhs == rhs;
            }
        }
        const impl<T> ::core::cmp::PartialOrd<T> for MyPlain
        where
            T: Relation + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_my_plain(rhs);
                return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
            }
        }
        impl MyPlain {
            #[inline(always)]
            #[allow(private_bounds)]
            pub const fn of<T>(it: T) -> MyPlain
            where
                T: Make + [const] Seal + [const] ::core::marker::Destruct,
            {
                let this = Seal::conv_my_plain(&it);
                return Self::_unchecked(this);
            }
            #[inline(always)]
            pub const fn make(it: i64) -> MyPlain {
                return MyPlain::of(it);
            }
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> i64 {
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
            pub const fn try_into_isize(self) -> Result<isize, ()> {
                return Ok(Self::raw(self) as isize);
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
            pub const fn try_into_usize(self) -> Result<usize, ()> {
                let r = Self::raw(self);
                let t = r as usize;
                let s = t as i64;
                return if s == r && r >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u8(self) -> Result<u8, ()> {
                let r = Self::raw(self);
                let t = r as u8;
                let s = t as i64;
                return if s == r && r >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u16(self) -> Result<u16, ()> {
                let r = Self::raw(self);
                let t = r as u16;
                let s = t as i64;
                return if s == r && r >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u32(self) -> Result<u32, ()> {
                let r = Self::raw(self);
                let t = r as u32;
                let s = t as i64;
                return if s == r && r >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u64(self) -> Result<u64, ()> {
                let r = Self::raw(self);
                let t = r as u64;
                let s = t as i64;
                return if s == r && r >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u128(self) -> Result<u128, ()> {
                let r = Self::raw(self);
                let t = r as u128;
                let s = t as i64;
                return if s == r && r >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i8(self) -> Result<i8, ()> {
                let r = Self::raw(self);
                let t = r as i8;
                let s = t as i64;
                return if s == r && true { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i16(self) -> Result<i16, ()> {
                let r = Self::raw(self);
                let t = r as i16;
                let s = t as i64;
                return if s == r && true { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i32(self) -> Result<i32, ()> {
                let r = Self::raw(self);
                let t = r as i32;
                let s = t as i64;
                return if s == r && true { Ok(t) } else { Err(()) };
            }
            pub(self) const fn _add(
                &self,
                rhs: i64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs + rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _sub(
                &self,
                rhs: i64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs - rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _mul(
                &self,
                rhs: i64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs * rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _div(
                &self,
                rhs: i64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs / rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _rem(
                &self,
                rhs: i64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs % rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _bitxor(
                &self,
                rhs: i64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs ^ rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _bitand(
                &self,
                rhs: i64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs & rhs;
                return Self::_unchecked(it);
            }
            pub(self) const fn _bitor(
                &self,
                rhs: i64,
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
                rhs: i64,
            ) -> bool {
                let lhs = Self::raw(self);
                return lhs == rhs;
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) fn _cmp(
                self,
                rhs: i64,
            ) -> ::core::cmp::Ordering {
                let lhs = Self::raw(self);
                return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs)
                    .unwrap();
            }
            #[inline(always)]
            pub const fn try_make(it: i64) -> Result<Self, i64> {
                return Ok(Self::_unchecked(it));
            }
            #[must_use]
            #[inline(always)]
            pub(self) const fn _unchecked(it: i64) -> Self {
                return Self(it);
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl MyPlain {}
    };
}
type Subject = subject::MyPlain;
fn main() {
    {
        print!("{0:?}\n", Subject::make(123));
    };
}
