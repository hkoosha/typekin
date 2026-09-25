#![allow(dead_code)]
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_iter)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]

mod subject {
    #[derive_const(Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[derive(Copy, Debug, Hash)]
    #[repr(u128)]
    pub enum MyFlag {
        Z = 0,
        A = 10,
        B,
        C = 40,
    }
    #[derive(Copy)]
    #[derive_const(Clone)]
    #[repr(transparent)]
    pub struct MyFlags(u128);

    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    const _: () = {
        const trait Seal {
            fn conv_my_flags(&self) -> u128;
        }
        const trait Make: [const] Seal {}
        const trait Math: [const] Seal {}
        const trait Bit: [const] Seal {}
        const trait Relation: [const] Seal {}
        const impl Seal for MyFlags {
            #[inline(always)]
            fn conv_my_flags(&self) -> u128 {
                return Self::raw(*self);
            }
        }
        const impl Seal for MyFlag {
            #[inline(always)]
            fn conv_my_flags(&self) -> u128 {
                return MyFlag::raw(*self);
            }
        }
        const impl Bit for MyFlag {}
        const impl Make for MyFlag {}
        const impl Bit for MyFlags {}
        const impl Make for MyFlags {}
        const impl Math for MyFlags {}
        const impl Relation for MyFlags {}
        const impl Seal for i16 {
            #[inline(always)]
            fn conv_my_flags(&self) -> u128 {
                return *self as u128;
            }
        }
        const impl Make for i16 {}
        const impl Seal for u128 {
            #[inline(always)]
            fn conv_my_flags(&self) -> u128 {
                return *self as u128;
            }
        }
        const impl Bit for u128 {}
        const impl Make for u128 {}
        const impl Math for u128 {}
        const impl Relation for u128 {}
        const impl Seal for u16 {
            #[inline(always)]
            fn conv_my_flags(&self) -> u128 {
                return *self as u128;
            }
        }
        const impl Make for u16 {}
        const impl Seal for u32 {
            #[inline(always)]
            fn conv_my_flags(&self) -> u128 {
                return *self as u128;
            }
        }
        const impl Make for u32 {}
        const impl Seal for u64 {
            #[inline(always)]
            fn conv_my_flags(&self) -> u128 {
                return *self as u128;
            }
        }
        const impl Make for u64 {}
        const impl Seal for u8 {
            #[inline(always)]
            fn conv_my_flags(&self) -> u128 {
                return *self as u128;
            }
        }
        const impl Make for u8 {}
        const impl Seal for usize {
            #[inline(always)]
            fn conv_my_flags(&self) -> u128 {
                return *self as u128;
            }
        }
        const impl Make for usize {}
        const impl<T> Seal for &T
        where
            T: [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn conv_my_flags(&self) -> u128 {
                return Seal::conv_my_flags(&**self);
            }
        }
        const impl<T> Seal for &mut T
        where
            T: [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn conv_my_flags(&self) -> u128 {
                return Seal::conv_my_flags(&**self);
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
        const impl ::core::convert::Into<u128> for MyFlags {
            #[inline(always)]
            fn into(self) -> u128 {
                return MyFlags::into_u128(self);
            }
        }
        const impl ::core::convert::TryInto<usize> for MyFlags {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<usize, Self::Error> {
                return MyFlags::try_into_usize(self);
            }
        }
        const impl ::core::convert::TryInto<isize> for MyFlags {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<isize, Self::Error> {
                return MyFlags::try_into_isize(self);
            }
        }
        const impl ::core::convert::TryInto<u8> for MyFlags {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u8, Self::Error> {
                return MyFlags::try_into_u8(self);
            }
        }
        const impl ::core::convert::TryInto<u16> for MyFlags {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u16, Self::Error> {
                return MyFlags::try_into_u16(self);
            }
        }
        const impl ::core::convert::TryInto<u32> for MyFlags {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u32, Self::Error> {
                return MyFlags::try_into_u32(self);
            }
        }
        const impl ::core::convert::TryInto<u64> for MyFlags {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u64, Self::Error> {
                return MyFlags::try_into_u64(self);
            }
        }
        const impl ::core::convert::TryInto<i8> for MyFlags {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i8, Self::Error> {
                return MyFlags::try_into_i8(self);
            }
        }
        const impl ::core::convert::TryInto<i16> for MyFlags {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i16, Self::Error> {
                return MyFlags::try_into_i16(self);
            }
        }
        const impl ::core::convert::TryInto<i32> for MyFlags {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i32, Self::Error> {
                return MyFlags::try_into_i32(self);
            }
        }
        const impl ::core::convert::TryInto<i64> for MyFlags {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i64, Self::Error> {
                return MyFlags::try_into_i64(self);
            }
        }
        const impl ::core::convert::TryInto<i128> for MyFlags {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i128, Self::Error> {
                return MyFlags::try_into_i128(self);
            }
        }
        const impl ::core::ops::Shr<usize> for MyFlags {
            type Output = Self;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        const impl ::core::ops::Shl<usize> for MyFlags {
            type Output = Self;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        const impl ::core::ops::ShrAssign<usize> for MyFlags {
            #[inline(always)]
            fn shr_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shr(other);
            }
        }
        const impl ::core::ops::ShlAssign<usize> for MyFlags {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shl(other);
            }
        }
        const impl<T> ::core::ops::BitAndAssign<T> for MyFlags
        where
            T: Bit + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_flags(&rhs);
                *self = self._bitand(it);
            }
        }
        const impl<T> ::core::ops::AddAssign<T> for MyFlags
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_flags(&rhs);
                *self = self._add(it);
            }
        }
        const impl<T> ::core::ops::SubAssign<T> for MyFlags
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_flags(&rhs);
                *self = self._sub(it);
            }
        }
        const impl<T> ::core::ops::MulAssign<T> for MyFlags
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_flags(&rhs);
                *self = self._mul(it);
            }
        }
        const impl<T> ::core::ops::DivAssign<T> for MyFlags
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_flags(&rhs);
                *self = self._div(it);
            }
        }
        const impl<T> ::core::ops::RemAssign<T> for MyFlags
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_flags(&rhs);
                *self = self._rem(it);
            }
        }
        const impl<T> ::core::ops::BitOrAssign<T> for MyFlags
        where
            T: Bit + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_flags(&rhs);
                *self = self._bitor(it);
            }
        }
        impl ::core::fmt::Debug for MyFlags {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                ::core::write!(f, "MyFlags({})", self.0)
            }
        }
        const impl ::core::cmp::Eq for MyFlags {}
        const impl ::core::cmp::Ord for MyFlags {
            #[inline(always)]
            fn cmp(
                &self,
                other: &Self,
            ) -> ::core::cmp::Ordering {
                return self.partial_cmp(other).unwrap();
            }
        }
        if !(::core::mem::size_of::<MyFlags>()
            == ::core::mem::size_of::<u128>())
        {
            panic!("invalid memory layout: #ty(#el) != #el");
        };
        const impl<T> ::core::ops::Add<T> for MyFlags
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flags(&rhs);
                return self._add(it);
            }
        }
        const impl<T> ::core::ops::Sub<T> for MyFlags
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flags(&rhs);
                return self._sub(it);
            }
        }
        const impl<T> ::core::ops::Mul<T> for MyFlags
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flags(&rhs);
                return self._mul(it);
            }
        }
        const impl<T> ::core::ops::Div<T> for MyFlags
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flags(&rhs);
                return self._div(it);
            }
        }
        const impl<T> ::core::ops::Rem<T> for MyFlags
        where
            T: Math + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flags(&rhs);
                return self._rem(it);
            }
        }
        const impl<T> ::core::ops::BitAnd<T> for MyFlags
        where
            T: Bit + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flags(&rhs);
                return self._bitand(it);
            }
        }
        const impl<T> ::core::ops::BitOr<T> for MyFlags
        where
            T: Bit + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flags(&rhs);
                return self._bitor(it);
            }
        }
        const impl<T> ::core::ops::BitXor<T> for MyFlags
        where
            T: Bit + [const] Seal + [const] ::core::marker::Destruct,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_my_flags(&rhs);
                return self._bitxor(it);
            }
        }
        const impl<T> ::core::ops::BitXorAssign<T> for MyFlags
        where
            T: Bit + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_my_flags(&rhs);
                *self = self._bitxor(it);
            }
        }
        impl ::core::fmt::Binary for MyFlags {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl ::core::fmt::Octal for MyFlags {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl ::core::fmt::LowerHex for MyFlags {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl ::core::fmt::UpperHex for MyFlags {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        const impl<T> ::core::cmp::PartialEq<T> for MyFlags
        where
            T: Relation + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_my_flags(rhs);
                return lhs == rhs;
            }
        }
        const impl<T> ::core::cmp::PartialOrd<T> for MyFlags
        where
            T: Relation + [const] Seal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_my_flags(rhs);
                return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
            }
        }
        impl MyFlags {
            #[inline(always)]
            #[allow(private_bounds)]
            pub const fn of<T>(it: T) -> MyFlags
            where
                T: Make + [const] Seal + [const] ::core::marker::Destruct,
            {
                let this = Seal::conv_my_flags(&it);
                return Self::_unchecked(this);
            }
            #[inline(always)]
            pub const fn make(it: u128) -> MyFlags {
                return MyFlags::of(it);
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
            pub const fn try_into_u128(
                self
            ) -> ::core::result::Result<u128, ()> {
                return ::core::result::Result::Ok(Self::raw(self) as u128);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(
                self
            ) -> ::core::result::Result<usize, ()> {
                let r = Self::raw(self);
                let t = r as usize;
                let s = t as u128;
                return if s == r && true {
                    ::core::result::Result::Ok(t)
                }
                else {
                    ::core::result::Result::Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(
                self
            ) -> ::core::result::Result<isize, ()> {
                let r = Self::raw(self);
                let t = r as isize;
                let s = t as u128;
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
                let s = t as u128;
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
                let s = t as u128;
                return if s == r && true {
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
                let s = t as u128;
                return if s == r && true {
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
                let s = t as u128;
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
                let s = t as u128;
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
                let s = t as u128;
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
                let s = t as u128;
                return if s == r && t >= 0 {
                    ::core::result::Result::Ok(t)
                }
                else {
                    ::core::result::Result::Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i64(self) -> ::core::result::Result<i64, ()> {
                let r = Self::raw(self);
                let t = r as i64;
                let s = t as u128;
                return if s == r && t >= 0 {
                    ::core::result::Result::Ok(t)
                }
                else {
                    ::core::result::Result::Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i128(
                self
            ) -> ::core::result::Result<i128, ()> {
                let r = Self::raw(self);
                let t = r as i128;
                let s = t as u128;
                return if s == r && t >= 0 {
                    ::core::result::Result::Ok(t)
                }
                else {
                    ::core::result::Result::Err(())
                };
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
        #[allow(clippy::unnecessary_cast)]
        impl MyFlags {
            #[must_use]
            #[inline(always)]
            pub const fn lo64(self) -> u64 {
                return self.qword0();
            }
            #[must_use]
            #[inline(always)]
            pub const fn hi64(self) -> u64 {
                return self.qword1();
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte0(self) -> u8 {
                return ((Self::raw(self) >> (8 * 0usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte1(self) -> u8 {
                return ((Self::raw(self) >> (8 * 1usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte2(self) -> u8 {
                return ((Self::raw(self) >> (8 * 2usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte3(self) -> u8 {
                return ((Self::raw(self) >> (8 * 3usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte4(self) -> u8 {
                return ((Self::raw(self) >> (8 * 4usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte5(self) -> u8 {
                return ((Self::raw(self) >> (8 * 5usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte6(self) -> u8 {
                return ((Self::raw(self) >> (8 * 6usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte7(self) -> u8 {
                return ((Self::raw(self) >> (8 * 7usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte8(self) -> u8 {
                return ((Self::raw(self) >> (8 * 8usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte9(self) -> u8 {
                return ((Self::raw(self) >> (8 * 9usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte10(self) -> u8 {
                return ((Self::raw(self) >> (8 * 10usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte11(self) -> u8 {
                return ((Self::raw(self) >> (8 * 11usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte12(self) -> u8 {
                return ((Self::raw(self) >> (8 * 12usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte13(self) -> u8 {
                return ((Self::raw(self) >> (8 * 13usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte14(self) -> u8 {
                return ((Self::raw(self) >> (8 * 14usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte15(self) -> u8 {
                return ((Self::raw(self) >> (8 * 15usize)) & (0xFF as u128))
                    as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word0(self) -> u16 {
                return ((Self::raw(self) >> (16 * 0usize)) & (0xFFFF as u128))
                    as u16;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word1(self) -> u16 {
                return ((Self::raw(self) >> (16 * 1usize)) & (0xFFFF as u128))
                    as u16;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word2(self) -> u16 {
                return ((Self::raw(self) >> (16 * 2usize)) & (0xFFFF as u128))
                    as u16;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word3(self) -> u16 {
                return ((Self::raw(self) >> (16 * 3usize)) & (0xFFFF as u128))
                    as u16;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word4(self) -> u16 {
                return ((Self::raw(self) >> (16 * 4usize)) & (0xFFFF as u128))
                    as u16;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word5(self) -> u16 {
                return ((Self::raw(self) >> (16 * 5usize)) & (0xFFFF as u128))
                    as u16;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word6(self) -> u16 {
                return ((Self::raw(self) >> (16 * 6usize)) & (0xFFFF as u128))
                    as u16;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word7(self) -> u16 {
                return ((Self::raw(self) >> (16 * 7usize)) & (0xFFFF as u128))
                    as u16;
            }
            #[must_use]
            #[inline(always)]
            pub const fn dword0(self) -> u32 {
                return ((Self::raw(self) >> (32 * 0usize))
                    & (0xFFFFFFFF as u128)) as u32;
            }
            #[must_use]
            #[inline(always)]
            pub const fn dword1(self) -> u32 {
                return ((Self::raw(self) >> (32 * 1usize))
                    & (0xFFFFFFFF as u128)) as u32;
            }
            #[must_use]
            #[inline(always)]
            pub const fn dword2(self) -> u32 {
                return ((Self::raw(self) >> (32 * 2usize))
                    & (0xFFFFFFFF as u128)) as u32;
            }
            #[must_use]
            #[inline(always)]
            pub const fn dword3(self) -> u32 {
                return ((Self::raw(self) >> (32 * 3usize))
                    & (0xFFFFFFFF as u128)) as u32;
            }
            #[must_use]
            #[inline(always)]
            pub const fn qword0(self) -> u64 {
                return ((Self::raw(self) >> (64 * 0usize))
                    & (0xFFFFFFFFFFFFFFFF as u128))
                    as u64;
            }
            #[must_use]
            #[inline(always)]
            pub const fn qword1(self) -> u64 {
                return ((Self::raw(self) >> (64 * 1usize))
                    & (0xFFFFFFFFFFFFFFFF as u128))
                    as u64;
            }
        }
        impl MyFlags {
            #[must_use]
            #[inline(always)]
            pub const fn bits(self) -> u128 {
                return MyFlags::raw(self);
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
                return MyFlag::all();
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
                return self.unknown_bits() != Self::empty().into();
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
                return match MyFlag::from_name(name) {
                    Some(flag) => Some(flag.into_value()),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn into_flag(self) -> Result<MyFlag, Self> {
                let items = MyFlag::items();
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
            pub const fn iter_known_flags(
                self
            ) -> impl Iterator<Item = MyFlag> {
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
                return MyFlag::iter()
                    .map(|flag| (flag.name(), flag.into_value()));
            }
            #[must_use]
            #[inline(always)]
            pub fn iter_equal_names(
                self
            ) -> impl Iterator<Item = &'static str> {
                return MyFlag::iter()
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
        impl ::core::convert::From<MyFlag> for MyFlags {
            #[inline(always)]
            fn from(flag: MyFlag) -> Self {
                return flag.into_value();
            }
        }
        const impl ::core::ops::Not for MyFlags {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self.complemented();
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
                    _ => ::core::option::Option::None,
                };
            }
            #[inline(always)]
            #[must_use]
            pub const fn raw(self) -> u128 {
                return self as u128;
            }
            #[inline(always)]
            #[must_use]
            pub const fn into_value(self) -> MyFlags {
                return MyFlags::from_bits_retain(self.raw());
            }
            #[must_use]
            #[inline(always)]
            pub const fn all() -> MyFlags {
                #[allow(clippy::unnecessary_cast)]
                return MyFlags::from_bits_retain(
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
            pub const fn iter_values() -> impl Iterator<Item = MyFlags> {
                return MyFlags::all().iter();
            }
            #[must_use]
            #[inline(always)]
            pub const fn inserted(
                self,
                other: Self,
            ) -> MyFlags {
                return self.into_value().inserted(other.into_value());
            }
        }
        impl MyFlag {
            #[must_use]
            #[inline(always)]
            pub const fn from_bits_retain(bits: u128) -> MyFlags {
                return MyFlags::of(bits);
            }
            #[must_use]
            #[inline(always)]
            pub const fn empty() -> MyFlags {
                return MyFlags::empty();
            }
            #[must_use]
            #[inline(always)]
            pub const fn all_named() -> MyFlags {
                return MyFlags::all();
            }
            #[must_use]
            #[inline(always)]
            pub const fn all_known() -> MyFlags {
                return MyFlags::all();
            }
            #[must_use]
            #[inline(always)]
            pub const fn all_unknown() -> MyFlags {
                return MyFlags::from_bits_retain(!MyFlags::all().bits());
            }
            #[must_use]
            #[inline(always)]
            pub const fn from_bits(bits: u128) -> Option<MyFlags> {
                return MyFlags::from_bits(bits);
            }
            #[must_use]
            #[inline(always)]
            pub const fn from_bits_truncate(bits: u128) -> MyFlags {
                return MyFlags::from_bits_truncate(bits);
            }
            #[must_use]
            #[inline(always)]
            pub fn iter_defined_names()
            -> impl Iterator<Item = (&'static str, MyFlags)> {
                return MyFlags::iter_defined_names();
            }
        }
        const impl ::core::ops::Shr<usize> for MyFlag {
            type Output = MyFlags;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self.into_value() >> rhs;
            }
        }
        const impl ::core::ops::Shl<usize> for MyFlag {
            type Output = MyFlags;
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
            T: FlagBit + const FlagSeal + [const] ::core::marker::Destruct,
        {
            type Output = MyFlags;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let rhs = FlagSeal::conv_my_flag(&rhs);
                return self
                    .into_value()
                    .intersection(MyFlags::from_bits_retain(rhs));
            }
        }
        const impl<T> ::core::ops::BitOr<T> for MyFlag
        where
            T: FlagBit + const FlagSeal + [const] ::core::marker::Destruct,
        {
            type Output = MyFlags;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let rhs = FlagSeal::conv_my_flag(&rhs);
                return self.into_value().union(MyFlags::from_bits_retain(rhs));
            }
        }
        const impl<T> ::core::ops::BitXor<T> for MyFlag
        where
            T: FlagBit + const FlagSeal + [const] ::core::marker::Destruct,
        {
            type Output = MyFlags;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let rhs = FlagSeal::conv_my_flag(&rhs);
                return self
                    .into_value()
                    .symmetric_difference(MyFlags::from_bits_retain(rhs));
            }
        }
        const impl ::core::cmp::PartialEq<MyFlags> for MyFlag {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &MyFlags,
            ) -> bool {
                return self.raw() == rhs.raw();
            }
        }
        const impl ::core::cmp::PartialOrd<MyFlags> for MyFlag {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &MyFlags,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                return self.raw().partial_cmp(&rhs.raw());
            }
        }
        const impl ::core::ops::Not for MyFlag {
            type Output = MyFlags;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self.into_value().complemented();
            }
        }
        struct IterItems {
            index: usize,
        }
        const impl core::iter::Iterator for IterItems {
            type Item = MyFlag;
            fn next(&mut self) -> Option<Self::Item> {
                let items = MyFlag::items();
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
                let max = MyFlag::items().len();
                return (max, Some(max));
            }
        }
        struct IterFlags {
            value: MyFlags,
            index: usize,
        }
        const impl core::iter::Iterator for IterFlags {
            type Item = MyFlag;
            fn next(&mut self) -> Option<Self::Item> {
                let items = MyFlag::items();
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
                let bound = MyFlags::raw(self.value).count_ones() as usize;
                return (bound, Some(bound));
            }
        }
        pub struct IterValues {
            value: MyFlags,
            index: usize,
        }
        const impl Iterator for IterValues {
            type Item = MyFlags;
            fn next(&mut self) -> Option<Self::Item> {
                let items = MyFlag::items();
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
                let bound =
                    (MyFlags::raw(self.value).count_ones() + 1) as usize;
                return (bound, Some(bound));
            }
        }
        const trait FlagSeal {
            fn conv_my_flag(&self) -> u128;
        }
        const trait FlagMake: [const] FlagSeal {}
        const trait FlagMath: [const] FlagSeal {}
        const trait FlagBit: [const] FlagSeal {}
        const trait FlagCmp: [const] FlagSeal {}
        const impl FlagSeal for MyFlag {
            #[inline(always)]
            fn conv_my_flag(&self) -> u128 {
                return MyFlag::raw(*self);
            }
        }
        const impl FlagBit for MyFlag {}
        const impl FlagCmp for MyFlag {}
        const impl FlagMake for MyFlag {}
        const impl FlagSeal for MyFlags {
            #[inline(always)]
            fn conv_my_flag(&self) -> u128 {
                return MyFlags::raw(*self);
            }
        }
        const impl FlagBit for MyFlags {}
        const impl FlagMake for MyFlags {}
        const impl<T> FlagSeal for &T
        where
            T: [const] FlagSeal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn conv_my_flag(&self) -> u128 {
                return FlagSeal::conv_my_flag(&**self);
            }
        }
        const impl<T> FlagSeal for &mut T
        where
            T: [const] FlagSeal + [const] ::core::marker::Destruct,
        {
            #[inline(always)]
            fn conv_my_flag(&self) -> u128 {
                return FlagSeal::conv_my_flag(&**self);
            }
        }
        const impl<T> FlagMake for &T where
            T: [const] FlagMake + [const] ::core::marker::Destruct
        {
        }
        const impl<T> FlagMake for &mut T where
            T: [const] FlagMake + [const] ::core::marker::Destruct
        {
        }
        const impl<T> FlagMath for &T where
            T: [const] FlagMath + [const] ::core::marker::Destruct
        {
        }
        const impl<T> FlagMath for &mut T where
            T: [const] FlagMath + [const] ::core::marker::Destruct
        {
        }
        const impl<T> FlagBit for &T where
            T: [const] FlagBit + [const] ::core::marker::Destruct
        {
        }
        const impl<T> FlagBit for &mut T where
            T: [const] FlagBit + [const] ::core::marker::Destruct
        {
        }
        const impl<T> FlagCmp for &T where
            T: [const] FlagCmp + [const] ::core::marker::Destruct
        {
        }
        const impl<T> FlagCmp for &mut T where
            T: [const] FlagCmp + [const] ::core::marker::Destruct
        {
        }
    };
}

type Subject = subject::MyFlag;
type Value = subject::MyFlags;

fn main() {
    let lhs = Subject::A;
    let rhs = Subject::B.into_value();

    println!("{:?}", (lhs & rhs) == (lhs & rhs));
    println!("{:?}", rhs & lhs);
    println!("{:?}", lhs & rhs);

    for x in Subject::items() {
        println!("{}", x.name());
    }
}
