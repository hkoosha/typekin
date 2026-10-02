mod subject {
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct MyExample(u32);

    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    const _: () = {
        trait Seal {
            fn conv_my_example(self) -> u32;
        }
        trait Make: Seal {
            fn make(self) -> MyExample;
        }
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Relation: Seal {}
        trait Trust: Seal {}
        impl Seal for MyExample {
            #[inline(always)]
            fn conv_my_example(self) -> u32 {
                return Self::raw(self);
            }
        }
        impl Bit for MyExample {}
        impl Make for MyExample {
            #[inline(always)]
            fn make(self) -> MyExample {
                let raw = <Self as Seal>::conv_my_example(self);
                return MyExample::_unchecked(raw);
            }
        }
        impl Math for MyExample {}
        impl Relation for MyExample {}
        impl Seal for i16 {
            #[inline(always)]
            fn conv_my_example(self) -> u32 {
                return self as u32;
            }
        }
        impl Make for i16 {
            #[inline(always)]
            fn make(self) -> MyExample {
                let raw = <Self as Seal>::conv_my_example(self);
                return MyExample::_unchecked(raw);
            }
        }
        impl Seal for u16 {
            #[inline(always)]
            fn conv_my_example(self) -> u32 {
                return self as u32;
            }
        }
        impl Make for u16 {
            #[inline(always)]
            fn make(self) -> MyExample {
                let raw = <Self as Seal>::conv_my_example(self);
                return MyExample::_unchecked(raw);
            }
        }
        impl Seal for u32 {
            #[inline(always)]
            fn conv_my_example(self) -> u32 {
                return self as u32;
            }
        }
        impl Bit for u32 {}
        impl Make for u32 {
            #[inline(always)]
            fn make(self) -> MyExample {
                let raw = <Self as Seal>::conv_my_example(self);
                return MyExample::_unchecked(raw);
            }
        }
        impl Math for u32 {}
        impl Relation for u32 {}
        impl Seal for u8 {
            #[inline(always)]
            fn conv_my_example(self) -> u32 {
                return self as u32;
            }
        }
        impl Make for u8 {
            #[inline(always)]
            fn make(self) -> MyExample {
                let raw = <Self as Seal>::conv_my_example(self);
                return MyExample::_unchecked(raw);
            }
        }
        impl ::core::convert::AsRef<u32> for MyExample {
            #[inline(always)]
            fn as_ref(&self) -> &u32 {
                return &self.0;
            }
        }
        impl ::core::convert::Into<usize> for MyExample {
            #[inline(always)]
            fn into(self) -> usize {
                return MyExample::into_usize(self);
            }
        }
        impl ::core::convert::Into<u32> for MyExample {
            #[inline(always)]
            fn into(self) -> u32 {
                return MyExample::into_u32(self);
            }
        }
        impl ::core::convert::Into<u64> for MyExample {
            #[inline(always)]
            fn into(self) -> u64 {
                return MyExample::into_u64(self);
            }
        }
        impl ::core::convert::Into<u128> for MyExample {
            #[inline(always)]
            fn into(self) -> u128 {
                return MyExample::into_u128(self);
            }
        }
        impl ::core::convert::Into<i64> for MyExample {
            #[inline(always)]
            fn into(self) -> i64 {
                return MyExample::into_i64(self);
            }
        }
        impl ::core::convert::Into<i128> for MyExample {
            #[inline(always)]
            fn into(self) -> i128 {
                return MyExample::into_i128(self);
            }
        }
        impl ::core::convert::TryInto<isize> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<isize, Self::Error> {
                return MyExample::try_into_isize(self);
            }
        }
        impl ::core::convert::TryInto<u8> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u8, Self::Error> {
                return MyExample::try_into_u8(self);
            }
        }
        impl ::core::convert::TryInto<u16> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u16, Self::Error> {
                return MyExample::try_into_u16(self);
            }
        }
        impl ::core::convert::TryInto<i8> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i8, Self::Error> {
                return MyExample::try_into_i8(self);
            }
        }
        impl ::core::convert::TryInto<i16> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i16, Self::Error> {
                return MyExample::try_into_i16(self);
            }
        }
        impl ::core::convert::TryInto<i32> for MyExample {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i32, Self::Error> {
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
                *self = self._bitor(it);
            }
        }
        impl ::core::fmt::Debug for MyExample {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                ::core::write!(f, "MyExample({})", self.0)
            }
        }
        impl ::core::cmp::Eq for MyExample {}
        impl ::core::cmp::Ord for MyExample {
            #[inline(always)]
            fn cmp(
                &self,
                other: &Self,
            ) -> ::core::cmp::Ordering {
                return self.partial_cmp(other).unwrap();
            }
        }
        if !(::core::mem::size_of::<MyExample>()
            == ::core::mem::size_of::<u32>())
        {
            panic!("invalid memory layout: #ty(#el) != #el");
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
                let it = Seal::conv_my_example(rhs);
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
        impl ::core::fmt::Display for MyExample {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Display::fmt(&raw, f);
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
        impl ::core::cmp::PartialEq for MyExample {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &Self,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Self::raw(*rhs);
                return lhs == rhs;
            }
        }
        impl ::core::cmp::PartialOrd for MyExample {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &Self,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Self::raw(*rhs);
                return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
            }
        }
        impl MyExample {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> MyExample
            where
                T: Make,
            {
                return <T as Make>::make(it);
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
            pub const fn try_into_usize(
                self
            ) -> ::core::result::Result<usize, ()> {
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
            pub const fn try_into_u128(
                self
            ) -> ::core::result::Result<u128, ()> {
                return ::core::result::Result::Ok(Self::raw(self) as u128);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i64(self) -> ::core::result::Result<i64, ()> {
                return ::core::result::Result::Ok(Self::raw(self) as i64);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i128(
                self
            ) -> ::core::result::Result<i128, ()> {
                return ::core::result::Result::Ok(Self::raw(self) as i128);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(
                self
            ) -> ::core::result::Result<isize, ()> {
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

    pub struct Verified(u32);

    impl Verified {
        pub fn try_make(raw: u32) -> Option<Self> {
            return (1..=100).contains(&raw).then_some(Self(raw));
        }

        fn into_raw(self) -> u32 {
            return self.0;
        }
    }

    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct Limited(u32);

    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    const _: () = {
        trait Seal {
            fn conv_limited(self) -> u32;
        }
        trait Make: Seal {
            fn make(self) -> Limited;
        }
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Relation: Seal {}
        trait Trust: Seal {}
        impl Seal for Limited {
            #[inline(always)]
            fn conv_limited(self) -> u32 {
                return Self::raw(self);
            }
        }
        impl Bit for Limited {}
        impl Make for Limited {
            #[inline(always)]
            fn make(self) -> Limited {
                let raw = <Self as Seal>::conv_limited(self);
                return Limited::_unchecked(raw);
            }
        }
        impl Math for Limited {}
        impl Relation for Limited {}
        impl Seal for Verified {
            #[inline(always)]
            fn conv_limited(self) -> u32 {
                return Verified::into_raw(self);
            }
        }
        impl Make for Verified {
            #[inline(always)]
            fn make(self) -> Limited {
                let raw = <Self as Seal>::conv_limited(self);
                return Limited(raw);
            }
        }
        impl Trust for Verified {}
        impl Seal for u32 {
            #[inline(always)]
            fn conv_limited(self) -> u32 {
                return self as u32;
            }
        }
        impl Bit for u32 {}
        impl Math for u32 {}
        impl Relation for u32 {}
        impl ::core::convert::AsRef<u32> for Limited {
            #[inline(always)]
            fn as_ref(&self) -> &u32 {
                return &self.0;
            }
        }
        impl ::core::convert::Into<usize> for Limited {
            #[inline(always)]
            fn into(self) -> usize {
                return Limited::into_usize(self);
            }
        }
        impl ::core::convert::Into<u32> for Limited {
            #[inline(always)]
            fn into(self) -> u32 {
                return Limited::into_u32(self);
            }
        }
        impl ::core::convert::Into<u64> for Limited {
            #[inline(always)]
            fn into(self) -> u64 {
                return Limited::into_u64(self);
            }
        }
        impl ::core::convert::Into<u128> for Limited {
            #[inline(always)]
            fn into(self) -> u128 {
                return Limited::into_u128(self);
            }
        }
        impl ::core::convert::Into<i64> for Limited {
            #[inline(always)]
            fn into(self) -> i64 {
                return Limited::into_i64(self);
            }
        }
        impl ::core::convert::Into<i128> for Limited {
            #[inline(always)]
            fn into(self) -> i128 {
                return Limited::into_i128(self);
            }
        }
        impl ::core::convert::TryInto<isize> for Limited {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<isize, Self::Error> {
                return Limited::try_into_isize(self);
            }
        }
        impl ::core::convert::TryInto<u8> for Limited {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u8, Self::Error> {
                return Limited::try_into_u8(self);
            }
        }
        impl ::core::convert::TryInto<u16> for Limited {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<u16, Self::Error> {
                return Limited::try_into_u16(self);
            }
        }
        impl ::core::convert::TryInto<i8> for Limited {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i8, Self::Error> {
                return Limited::try_into_i8(self);
            }
        }
        impl ::core::convert::TryInto<i16> for Limited {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i16, Self::Error> {
                return Limited::try_into_i16(self);
            }
        }
        impl ::core::convert::TryInto<i32> for Limited {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> ::core::result::Result<i32, Self::Error> {
                return Limited::try_into_i32(self);
            }
        }
        impl ::core::ops::Shr<usize> for Limited {
            type Output = Self;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        impl ::core::ops::Shl<usize> for Limited {
            type Output = Self;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        impl ::core::ops::ShrAssign<usize> for Limited {
            #[inline(always)]
            fn shr_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shr(other);
            }
        }
        impl ::core::ops::ShlAssign<usize> for Limited {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                other: usize,
            ) {
                *self = self._shl(other);
            }
        }
        impl<T> ::core::ops::BitAndAssign<T> for Limited
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_limited(rhs);
                *self = self._bitand(it);
            }
        }
        impl<T> ::core::ops::AddAssign<T> for Limited
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_limited(rhs);
                *self = self._add(it);
            }
        }
        impl<T> ::core::ops::SubAssign<T> for Limited
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_limited(rhs);
                *self = self._sub(it);
            }
        }
        impl<T> ::core::ops::MulAssign<T> for Limited
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_limited(rhs);
                *self = self._mul(it);
            }
        }
        impl<T> ::core::ops::DivAssign<T> for Limited
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_limited(rhs);
                *self = self._div(it);
            }
        }
        impl<T> ::core::ops::RemAssign<T> for Limited
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_limited(rhs);
                *self = self._rem(it);
            }
        }
        impl<T> ::core::ops::BitOrAssign<T> for Limited
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_limited(rhs);
                *self = self._bitor(it);
            }
        }
        impl ::core::fmt::Debug for Limited {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                ::core::write!(f, "Limited({})", self.0)
            }
        }
        impl ::core::cmp::Eq for Limited {}
        impl ::core::cmp::Ord for Limited {
            #[inline(always)]
            fn cmp(
                &self,
                other: &Self,
            ) -> ::core::cmp::Ordering {
                return self.partial_cmp(other).unwrap();
            }
        }
        if !(::core::mem::size_of::<Limited>() == ::core::mem::size_of::<u32>())
        {
            panic!("invalid memory layout: #ty(#el) != #el");
        };
        impl<T> ::core::ops::Add<T> for Limited
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_limited(rhs);
                return self._add(it);
            }
        }
        impl<T> ::core::ops::Sub<T> for Limited
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_limited(rhs);
                return self._sub(it);
            }
        }
        impl<T> ::core::ops::Mul<T> for Limited
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_limited(rhs);
                return self._mul(it);
            }
        }
        impl<T> ::core::ops::Div<T> for Limited
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_limited(rhs);
                return self._div(it);
            }
        }
        impl<T> ::core::ops::Rem<T> for Limited
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_limited(rhs);
                return self._rem(it);
            }
        }
        impl<T> ::core::ops::BitAnd<T> for Limited
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_limited(rhs);
                return self._bitand(it);
            }
        }
        impl<T> ::core::ops::BitOr<T> for Limited
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_limited(rhs);
                return self._bitor(it);
            }
        }
        impl<T> ::core::ops::BitXor<T> for Limited
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_limited(rhs);
                return self._bitxor(it);
            }
        }
        impl<T> ::core::ops::BitXorAssign<T> for Limited
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_limited(rhs);
                *self = self._bitxor(it);
            }
        }
        impl ::core::ops::Not for Limited {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        impl ::core::fmt::Binary for Limited {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl ::core::fmt::Octal for Limited {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl ::core::fmt::LowerHex for Limited {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl ::core::fmt::UpperHex for Limited {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        impl ::core::cmp::PartialEq for Limited {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &Self,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Self::raw(*rhs);
                return lhs == rhs;
            }
        }
        impl ::core::cmp::PartialOrd for Limited {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &Self,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Self::raw(*rhs);
                return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
            }
        }
        impl Limited {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> Limited
            where
                T: Make,
            {
                return <T as Make>::make(it);
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
            pub const fn try_into_usize(
                self
            ) -> ::core::result::Result<usize, ()> {
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
            pub const fn try_into_u128(
                self
            ) -> ::core::result::Result<u128, ()> {
                return ::core::result::Result::Ok(Self::raw(self) as u128);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i64(self) -> ::core::result::Result<i64, ()> {
                return ::core::result::Result::Ok(Self::raw(self) as i64);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i128(
                self
            ) -> ::core::result::Result<i128, ()> {
                return ::core::result::Result::Ok(Self::raw(self) as i128);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(
                self
            ) -> ::core::result::Result<isize, ()> {
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
            pub fn try_make(it: u32) -> Result<Self, u32> {
                return if (it >= (1)) && (it <= (100)) {
                    return ::core::result::Result::Ok(Self(it));
                }
                else {
                    ::core::result::Result::Err(it)
                };
            }
            #[must_use]
            #[inline(always)]
            pub(self) const fn _unchecked(it: u32) -> Self {
                if (it >= (1)) && (it <= (100)) {
                    return Self(it);
                }
                else {
                    ::core::panic!("invalid value");
                };
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl Limited {
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
    println!("{}", lhs + rhs);

    let verified = subject::Verified::try_make(42).unwrap();
    // The non-Copy friend moves its already-validated value into Limited.
    let limited = subject::Limited::of(verified);
    assert_eq!(limited.raw(), 42);
    assert_eq!(subject::Limited::try_make(0), Err(0));
}
