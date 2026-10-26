mod sealed {
    pub trait Direction {}
}

trait Direction: sealed::Direction {}

struct In;
struct Out;

impl sealed::Direction for In {}
impl sealed::Direction for Out {}
impl Direction for In {}
impl Direction for Out {}

#[repr(transparent)]
struct SeqPoint<D: Direction>(u32, core::marker::PhantomData<D>);

impl<D: Direction> Copy for SeqPoint<D> {}
impl<D: Direction> Clone for SeqPoint<D> {
    #[inline(always)]
    fn clone(&self) -> Self {
        return {
            let () = <SeqPoint<D>>::__TYPEKIN_LAYOUT_ASSERT;
            Self(self.0, core::marker::PhantomData)
        };
    }
}
#[allow(dead_code)]
#[allow(unused_qualifications)]
#[allow(clippy::unnecessary_cast)]
const _: () = {
    trait Seal<D: Direction> {
        fn conv_seq_point(self) -> u32;
    }
    trait Make<D: Direction>: Seal<D> {
        fn make(self) -> SeqPoint<D>;
    }
    trait Rel<D: Direction>: Seal<D> {}
    #[doc = "# Safety"]
    #[doc = ""]
    #[doc = "Implementing this trait asserts that values \
                        produced by the sealed conversion satisfy every \
                        invariant required by the target type."]
    unsafe trait UnsafeTrust<D: Direction>: Seal<D> {}
    impl<D: Direction> Seal<D> for SeqPoint<D> {
        #[inline(always)]
        fn conv_seq_point(self) -> u32 {
            return Self::raw(self);
        }
    }
    impl<D: Direction> Make<D> for SeqPoint<D> {
        #[inline(always)]
        fn make(self) -> SeqPoint<D> {
            let raw = <Self as Seal<D>>::conv_seq_point(self);
            return <SeqPoint<D>>::_unchecked(raw);
        }
    }
    impl<D: Direction> Rel<D> for SeqPoint<D> {}
    impl<D: Direction> Seal<D> for i16 {
        #[inline(always)]
        fn conv_seq_point(self) -> u32 {
            return self as u32;
        }
    }
    impl<D: Direction> Make<D> for i16 {
        #[inline(always)]
        fn make(self) -> SeqPoint<D> {
            let raw = <Self as Seal<D>>::conv_seq_point(self);
            return <SeqPoint<D>>::_unchecked(raw);
        }
    }
    impl<D: Direction> Seal<D> for u16 {
        #[inline(always)]
        fn conv_seq_point(self) -> u32 {
            return self as u32;
        }
    }
    impl<D: Direction> Make<D> for u16 {
        #[inline(always)]
        fn make(self) -> SeqPoint<D> {
            let raw = <Self as Seal<D>>::conv_seq_point(self);
            return <SeqPoint<D>>::_unchecked(raw);
        }
    }
    impl<D: Direction> Seal<D> for u32 {
        #[inline(always)]
        fn conv_seq_point(self) -> u32 {
            return self as u32;
        }
    }
    impl<D: Direction> Make<D> for u32 {
        #[inline(always)]
        fn make(self) -> SeqPoint<D> {
            let raw = <Self as Seal<D>>::conv_seq_point(self);
            return <SeqPoint<D>>::_unchecked(raw);
        }
    }
    impl<D: Direction> Seal<D> for u8 {
        #[inline(always)]
        fn conv_seq_point(self) -> u32 {
            return self as u32;
        }
    }
    impl<D: Direction> Make<D> for u8 {
        #[inline(always)]
        fn make(self) -> SeqPoint<D> {
            let raw = <Self as Seal<D>>::conv_seq_point(self);
            return <SeqPoint<D>>::_unchecked(raw);
        }
    }
    #[doc(hidden)]
    #[allow(private_interfaces)]
    pub trait AddRhs<D: Direction> {
        type Output;
        fn add(
            lhs: SeqPoint<D>,
            rhs: Self,
        ) -> Self::Output;
    }
    #[doc(hidden)]
    #[allow(private_interfaces)]
    pub trait SubRhs<D: Direction> {
        type Output;
        fn sub(
            lhs: SeqPoint<D>,
            rhs: Self,
        ) -> Self::Output;
    }
    impl<D: Direction> AddRhs<D> for SeqOffset<D> {
        type Output = SeqPoint<D>;
        #[inline(always)]
        fn add(
            lhs: SeqPoint<D>,
            rhs: SeqOffset<D>,
        ) -> Self::Output {
            let lhs = <SeqPoint<D>>::raw(lhs);
            let rhs = <SeqPoint<D>>::offset_raw(rhs);
            let value = lhs.wrapping_add(rhs);
            let value = <SeqPoint<D>>::_unchecked(value);
            return value;
        }
    }
    impl<D: Direction> SubRhs<D> for SeqPoint<D> {
        type Output = SeqOffset<D>;
        #[inline(always)]
        fn sub(
            lhs: SeqPoint<D>,
            rhs: SeqPoint<D>,
        ) -> Self::Output {
            let lhs = <SeqPoint<D>>::raw(lhs);
            let rhs = <SeqPoint<D>>::raw(rhs);
            let value = lhs.wrapping_sub(rhs);
            let value = <SeqPoint<D>>::_unchecked(value);
            return <SeqPoint<D>>::into_offset(value);
        }
    }
    impl<D: Direction> SubRhs<D> for SeqOffset<D> {
        type Output = SeqPoint<D>;
        #[inline(always)]
        fn sub(
            lhs: SeqPoint<D>,
            rhs: SeqOffset<D>,
        ) -> Self::Output {
            let lhs = <SeqPoint<D>>::raw(lhs);
            let rhs = <SeqPoint<D>>::offset_raw(rhs);
            let value = lhs.wrapping_sub(rhs);
            let value = <SeqPoint<D>>::_unchecked(value);
            return value;
        }
    }
    #[allow(private_bounds)]
    impl<D: Direction, T> core::ops::Add<T> for SeqPoint<D>
    where
        T: AddRhs<D>,
    {
        type Output = <T as AddRhs<D>>::Output;
        #[inline(always)]
        fn add(
            self,
            rhs: T,
        ) -> Self::Output {
            return <T as AddRhs<D>>::add(self, rhs);
        }
    }
    #[allow(private_bounds)]
    impl<D: Direction, T> core::ops::Sub<T> for SeqPoint<D>
    where
        T: SubRhs<D>,
    {
        type Output = <T as SubRhs<D>>::Output;
        #[inline(always)]
        fn sub(
            self,
            rhs: T,
        ) -> Self::Output {
            return <T as SubRhs<D>>::sub(self, rhs);
        }
    }
    impl<D: Direction> AsRef<u32> for SeqPoint<D> {
        #[inline(always)]
        fn as_ref(&self) -> &u32 {
            return &self.0;
        }
    }
    impl<D: Direction> core::str::FromStr for SeqPoint<D> {
        type Err = ();
        #[inline(always)]
        fn from_str(source: &str) -> Result<Self, Self::Err> {
            let value = match <u32 as core::str::FromStr>::from_str(source) {
                Ok(value) => value,
                Err(_) => {
                    return Err(());
                }
            };
            return Ok({
                let () = <SeqPoint<D>>::__TYPEKIN_LAYOUT_ASSERT;
                Self(value, core::marker::PhantomData)
            });
        }
    }
    impl<D: Direction> Into<usize> for SeqPoint<D> {
        #[inline(always)]
        fn into(self) -> usize {
            return Self::into_usize(self);
        }
    }
    impl<D: Direction> Into<u32> for SeqPoint<D> {
        #[inline(always)]
        fn into(self) -> u32 {
            return Self::into_u32(self);
        }
    }
    impl<D: Direction> Into<u64> for SeqPoint<D> {
        #[inline(always)]
        fn into(self) -> u64 {
            return Self::into_u64(self);
        }
    }
    impl<D: Direction> Into<u128> for SeqPoint<D> {
        #[inline(always)]
        fn into(self) -> u128 {
            return Self::into_u128(self);
        }
    }
    impl<D: Direction> Into<i64> for SeqPoint<D> {
        #[inline(always)]
        fn into(self) -> i64 {
            return Self::into_i64(self);
        }
    }
    impl<D: Direction> Into<i128> for SeqPoint<D> {
        #[inline(always)]
        fn into(self) -> i128 {
            return Self::into_i128(self);
        }
    }
    impl<D: Direction> TryInto<isize> for SeqPoint<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<isize, Self::Error> {
            return Self::try_into_isize(self);
        }
    }
    impl<D: Direction> TryInto<u8> for SeqPoint<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<u8, Self::Error> {
            return Self::try_into_u8(self);
        }
    }
    impl<D: Direction> TryInto<u16> for SeqPoint<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<u16, Self::Error> {
            return Self::try_into_u16(self);
        }
    }
    impl<D: Direction> TryInto<i8> for SeqPoint<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<i8, Self::Error> {
            return Self::try_into_i8(self);
        }
    }
    impl<D: Direction> TryInto<i16> for SeqPoint<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<i16, Self::Error> {
            return Self::try_into_i16(self);
        }
    }
    impl<D: Direction> TryInto<i32> for SeqPoint<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<i32, Self::Error> {
            return Self::try_into_i32(self);
        }
    }
    impl<D: Direction> core::fmt::Debug for SeqPoint<D> {
        fn fmt(
            &self,
            f: &mut core::fmt::Formatter<'_>,
        ) -> core::fmt::Result {
            return f.debug_tuple("SeqPoint").field(&self.0).finish();
        }
    }
    impl<D: Direction> core::hash::Hash for SeqPoint<D> {
        #[inline(always)]
        fn hash<H>(
            &self,
            state: &mut H,
        ) where
            H: core::hash::Hasher,
        {
            return core::hash::Hash::hash(&self.0, state);
        }
    }
    impl<D: Direction> Eq for SeqPoint<D> {}
    impl<D: Direction> SeqPoint<D> {
        const __TYPEKIN_LAYOUT_ASSERT: () = {
            if !(size_of::<SeqPoint<D>>() == size_of::<u32>()) {
                core::panic!("invalid integral storage size");
            };
            if !(align_of::<SeqPoint<D>>() == align_of::<u32>()) {
                core::panic!("invalid integral storage alignment");
            };
        };
    }
    impl<D: Direction> core::fmt::Binary for SeqPoint<D> {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut core::fmt::Formatter<'_>,
        ) -> core::fmt::Result {
            let raw = Self::raw(*self);
            return core::fmt::Binary::fmt(&raw, f);
        }
    }
    impl<D: Direction> core::fmt::Octal for SeqPoint<D> {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut core::fmt::Formatter<'_>,
        ) -> core::fmt::Result {
            let raw = Self::raw(*self);
            return core::fmt::Octal::fmt(&raw, f);
        }
    }
    impl<D: Direction> core::fmt::LowerHex for SeqPoint<D> {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut core::fmt::Formatter<'_>,
        ) -> core::fmt::Result {
            let raw = Self::raw(*self);
            return core::fmt::LowerHex::fmt(&raw, f);
        }
    }
    impl<D: Direction> core::fmt::UpperHex for SeqPoint<D> {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut core::fmt::Formatter<'_>,
        ) -> core::fmt::Result {
            let raw = Self::raw(*self);
            return core::fmt::UpperHex::fmt(&raw, f);
        }
    }
    impl<D: Direction, T> PartialEq<T> for SeqPoint<D>
    where
        T: Rel<D> + Seal<D> + Copy,
    {
        #[inline(always)]
        fn eq(
            &self,
            rhs: &T,
        ) -> bool {
            let lhs = Self::raw(*self);
            let rhs = <T as Seal<D>>::conv_seq_point(*rhs);
            return lhs == rhs;
        }
    }
    impl<D: Direction> SeqPoint<D> {
        #[inline(always)]
        #[allow(private_bounds)]
        pub fn of<T>(it: T) -> SeqPoint<D>
        where
            T: Make<D>,
        {
            return <T as Make<D>>::make(it);
        }
        #[inline(always)]
        pub fn make(it: u32) -> SeqPoint<D> {
            return <SeqPoint<D>>::of(it);
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
        ) -> Result<Self, u32>
        where
            F: FnOnce(u32) -> u32,
        {
            let value = function(Self::raw(self));
            return Ok({
                let () = <SeqPoint<D>>::__TYPEKIN_LAYOUT_ASSERT;
                Self(value, core::marker::PhantomData)
            });
        }
        #[inline(always)]
        #[track_caller]
        pub fn map<F>(
            self,
            function: F,
        ) -> Self
        where
            F: FnOnce(u32) -> u32,
        {
            return match self.try_map(function) {
                Ok(value) => value,
                Err(_) => {
                    core::panic!("mapping operation produced an invalid value")
                }
            };
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
        pub const fn precedes(
            self,
            other: Self,
        ) -> bool {
            let lhs = Self::raw(self) as u32;
            let rhs = Self::raw(other) as u32;
            let difference = rhs.wrapping_sub(lhs);
            let half = (1 as u32) << u32::BITS - 1;
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
        ) -> core::cmp::Ordering {
            let lhs = Self::raw(self);
            return PartialOrd::partial_cmp(&lhs, &rhs).unwrap();
        }
        #[inline(always)]
        pub const fn try_make(it: u32) -> Result<Self, u32> {
            return Ok(Self::_unchecked(it));
        }
        #[must_use]
        #[inline(always)]
        pub(self) const fn _unchecked(it: u32) -> Self {
            return {
                let () = <SeqPoint<D>>::__TYPEKIN_LAYOUT_ASSERT;
                Self(it, core::marker::PhantomData)
            };
        }
    }
    #[allow(clippy::unnecessary_cast)]
    impl<D: Direction> SeqPoint<D> {}
};

#[repr(transparent)]
struct SeqOffset<D: Direction>(i32, core::marker::PhantomData<D>);

impl<D: Direction> Copy for SeqOffset<D> {}
impl<D: Direction> Clone for SeqOffset<D> {
    #[inline(always)]
    fn clone(&self) -> Self {
        return {
            let () = <SeqOffset<D>>::__TYPEKIN_LAYOUT_ASSERT;
            Self(self.0, core::marker::PhantomData)
        };
    }
}
#[allow(dead_code)]
#[allow(unused_qualifications)]
#[allow(clippy::unnecessary_cast)]
const _: () = {
    trait Seal<D: Direction> {
        fn conv_seq_offset(self) -> i32;
    }
    trait Make<D: Direction>: Seal<D> {
        fn make(self) -> SeqOffset<D>;
    }
    trait Math<D: Direction>: Seal<D> {}
    trait Bit<D: Direction>: Seal<D> {}
    trait Rel<D: Direction>: Seal<D> {}
    #[doc = "# Safety"]
    #[doc = ""]
    #[doc = "Implementing this trait asserts that values \
                        produced by the sealed conversion satisfy every \
                        invariant required by the target type."]
    unsafe trait UnsafeTrust<D: Direction>: Seal<D> {}
    impl<D: Direction> Seal<D> for SeqOffset<D> {
        #[inline(always)]
        fn conv_seq_offset(self) -> i32 {
            return Self::raw(self);
        }
    }
    impl<D: Direction> Make<D> for SeqOffset<D> {
        #[inline(always)]
        fn make(self) -> SeqOffset<D> {
            let raw = <Self as Seal<D>>::conv_seq_offset(self);
            return <SeqOffset<D>>::_unchecked(raw);
        }
    }
    impl<D: Direction> Math<D> for SeqOffset<D> {}
    impl<D: Direction> Rel<D> for SeqOffset<D> {}
    impl<D: Direction> Seal<D> for i32 {
        #[inline(always)]
        fn conv_seq_offset(self) -> i32 {
            return self as i32;
        }
    }
    impl<D: Direction> Make<D> for i32 {
        #[inline(always)]
        fn make(self) -> SeqOffset<D> {
            let raw = <Self as Seal<D>>::conv_seq_offset(self);
            return <SeqOffset<D>>::_unchecked(raw);
        }
    }
    impl<D: Direction> Seal<D> for i8 {
        #[inline(always)]
        fn conv_seq_offset(self) -> i32 {
            return self as i32;
        }
    }
    impl<D: Direction> Make<D> for i8 {
        #[inline(always)]
        fn make(self) -> SeqOffset<D> {
            let raw = <Self as Seal<D>>::conv_seq_offset(self);
            return <SeqOffset<D>>::_unchecked(raw);
        }
    }
    impl<D: Direction> Seal<D> for u16 {
        #[inline(always)]
        fn conv_seq_offset(self) -> i32 {
            return self as i32;
        }
    }
    impl<D: Direction> Make<D> for u16 {
        #[inline(always)]
        fn make(self) -> SeqOffset<D> {
            let raw = <Self as Seal<D>>::conv_seq_offset(self);
            return <SeqOffset<D>>::_unchecked(raw);
        }
    }
    impl<D: Direction> Seal<D> for u8 {
        #[inline(always)]
        fn conv_seq_offset(self) -> i32 {
            return self as i32;
        }
    }
    impl<D: Direction> Make<D> for u8 {
        #[inline(always)]
        fn make(self) -> SeqOffset<D> {
            let raw = <Self as Seal<D>>::conv_seq_offset(self);
            return <SeqOffset<D>>::_unchecked(raw);
        }
    }
    impl<D: Direction> AsRef<i32> for SeqOffset<D> {
        #[inline(always)]
        fn as_ref(&self) -> &i32 {
            return &self.0;
        }
    }
    impl<D: Direction> core::str::FromStr for SeqOffset<D> {
        type Err = ();
        #[inline(always)]
        fn from_str(source: &str) -> Result<Self, Self::Err> {
            let value = match <i32 as core::str::FromStr>::from_str(source) {
                Ok(value) => value,
                Err(_) => {
                    return Err(());
                }
            };
            return Ok({
                let () = <SeqOffset<D>>::__TYPEKIN_LAYOUT_ASSERT;
                Self(value, core::marker::PhantomData)
            });
        }
    }
    impl<D: Direction> Into<isize> for SeqOffset<D> {
        #[inline(always)]
        fn into(self) -> isize {
            return Self::into_isize(self);
        }
    }
    impl<D: Direction> Into<i32> for SeqOffset<D> {
        #[inline(always)]
        fn into(self) -> i32 {
            return Self::into_i32(self);
        }
    }
    impl<D: Direction> Into<i64> for SeqOffset<D> {
        #[inline(always)]
        fn into(self) -> i64 {
            return Self::into_i64(self);
        }
    }
    impl<D: Direction> Into<i128> for SeqOffset<D> {
        #[inline(always)]
        fn into(self) -> i128 {
            return Self::into_i128(self);
        }
    }
    impl<D: Direction> TryInto<usize> for SeqOffset<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<usize, Self::Error> {
            return Self::try_into_usize(self);
        }
    }
    impl<D: Direction> TryInto<u8> for SeqOffset<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<u8, Self::Error> {
            return Self::try_into_u8(self);
        }
    }
    impl<D: Direction> TryInto<u16> for SeqOffset<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<u16, Self::Error> {
            return Self::try_into_u16(self);
        }
    }
    impl<D: Direction> TryInto<u32> for SeqOffset<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<u32, Self::Error> {
            return Self::try_into_u32(self);
        }
    }
    impl<D: Direction> TryInto<u64> for SeqOffset<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<u64, Self::Error> {
            return Self::try_into_u64(self);
        }
    }
    impl<D: Direction> TryInto<u128> for SeqOffset<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<u128, Self::Error> {
            return Self::try_into_u128(self);
        }
    }
    impl<D: Direction> TryInto<i8> for SeqOffset<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<i8, Self::Error> {
            return Self::try_into_i8(self);
        }
    }
    impl<D: Direction> TryInto<i16> for SeqOffset<D> {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> Result<i16, Self::Error> {
            return Self::try_into_i16(self);
        }
    }
    impl<D: Direction, T> core::ops::AddAssign<T> for SeqOffset<D>
    where
        T: Math<D> + Seal<D>,
    {
        #[inline(always)]
        fn add_assign(
            &mut self,
            rhs: T,
        ) {
            let it = <T as Seal<D>>::conv_seq_offset(rhs);
            *self = self._add(it);
        }
    }
    impl<D: Direction, T> core::ops::SubAssign<T> for SeqOffset<D>
    where
        T: Math<D> + Seal<D>,
    {
        #[inline(always)]
        fn sub_assign(
            &mut self,
            rhs: T,
        ) {
            let it = <T as Seal<D>>::conv_seq_offset(rhs);
            *self = self._sub(it);
        }
    }
    impl<D: Direction> core::fmt::Debug for SeqOffset<D> {
        fn fmt(
            &self,
            f: &mut core::fmt::Formatter<'_>,
        ) -> core::fmt::Result {
            return f.debug_tuple("SeqOffset").field(&self.0).finish();
        }
    }
    impl<D: Direction> core::hash::Hash for SeqOffset<D> {
        #[inline(always)]
        fn hash<H>(
            &self,
            state: &mut H,
        ) where
            H: core::hash::Hasher,
        {
            return core::hash::Hash::hash(&self.0, state);
        }
    }
    impl<D: Direction> Eq for SeqOffset<D> {}
    impl<D: Direction> SeqOffset<D> {
        const __TYPEKIN_LAYOUT_ASSERT: () = {
            if !(size_of::<SeqOffset<D>>() == size_of::<i32>()) {
                core::panic!("invalid integral storage size");
            };
            if !(align_of::<SeqOffset<D>>() == align_of::<i32>()) {
                core::panic!("invalid integral storage alignment");
            };
        };
    }
    impl<D: Direction, T> core::ops::Add<T> for SeqOffset<D>
    where
        T: Math<D> + Seal<D>,
    {
        type Output = Self;
        #[inline(always)]
        fn add(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = <T as Seal<D>>::conv_seq_offset(rhs);
            return self._add(it);
        }
    }
    impl<D: Direction, T> core::ops::Sub<T> for SeqOffset<D>
    where
        T: Math<D> + Seal<D>,
    {
        type Output = Self;
        #[inline(always)]
        fn sub(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = <T as Seal<D>>::conv_seq_offset(rhs);
            return self._sub(it);
        }
    }
    impl<D: Direction> core::fmt::Binary for SeqOffset<D> {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut core::fmt::Formatter<'_>,
        ) -> core::fmt::Result {
            let raw = Self::raw(*self);
            return core::fmt::Binary::fmt(&raw, f);
        }
    }
    impl<D: Direction> core::fmt::Octal for SeqOffset<D> {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut core::fmt::Formatter<'_>,
        ) -> core::fmt::Result {
            let raw = Self::raw(*self);
            return core::fmt::Octal::fmt(&raw, f);
        }
    }
    impl<D: Direction> core::fmt::LowerHex for SeqOffset<D> {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut core::fmt::Formatter<'_>,
        ) -> core::fmt::Result {
            let raw = Self::raw(*self);
            return core::fmt::LowerHex::fmt(&raw, f);
        }
    }
    impl<D: Direction> core::fmt::UpperHex for SeqOffset<D> {
        #[inline(always)]
        fn fmt(
            &self,
            f: &mut core::fmt::Formatter<'_>,
        ) -> core::fmt::Result {
            let raw = Self::raw(*self);
            return core::fmt::UpperHex::fmt(&raw, f);
        }
    }
    impl<D: Direction, T> PartialEq<T> for SeqOffset<D>
    where
        T: Rel<D> + Seal<D> + Copy,
    {
        #[inline(always)]
        fn eq(
            &self,
            rhs: &T,
        ) -> bool {
            let lhs = Self::raw(*self);
            let rhs = <T as Seal<D>>::conv_seq_offset(*rhs);
            return lhs == rhs;
        }
    }
    impl<D: Direction> SeqOffset<D> {
        #[inline(always)]
        #[allow(private_bounds)]
        pub fn of<T>(it: T) -> SeqOffset<D>
        where
            T: Make<D>,
        {
            return <T as Make<D>>::make(it);
        }
        #[inline(always)]
        pub fn make(it: i32) -> SeqOffset<D> {
            return <SeqOffset<D>>::of(it);
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
        ) -> Result<Self, i32>
        where
            F: FnOnce(i32) -> i32,
        {
            let value = function(Self::raw(self));
            return Ok({
                let () = <SeqOffset<D>>::__TYPEKIN_LAYOUT_ASSERT;
                Self(value, core::marker::PhantomData)
            });
        }
        #[inline(always)]
        #[track_caller]
        pub fn map<F>(
            self,
            function: F,
        ) -> Self
        where
            F: FnOnce(i32) -> i32,
        {
            return match self.try_map(function) {
                Ok(value) => value,
                Err(_) => {
                    core::panic!("mapping operation produced an invalid value")
                }
            };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_isize(self) -> Result<isize, ()> {
            return Ok(Self::raw(self) as isize);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i32(self) -> Result<i32, ()> {
            return Ok(Self::raw(self) as i32);
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
            let s = t as i32;
            return if s == r && r >= 0 { Ok(t) } else { Err(()) };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u8(self) -> Result<u8, ()> {
            let r = Self::raw(self);
            let t = r as u8;
            let s = t as i32;
            return if s == r && r >= 0 { Ok(t) } else { Err(()) };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u16(self) -> Result<u16, ()> {
            let r = Self::raw(self);
            let t = r as u16;
            let s = t as i32;
            return if s == r && r >= 0 { Ok(t) } else { Err(()) };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u32(self) -> Result<u32, ()> {
            let r = Self::raw(self);
            let t = r as u32;
            let s = t as i32;
            return if s == r && r >= 0 { Ok(t) } else { Err(()) };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u64(self) -> Result<u64, ()> {
            let r = Self::raw(self);
            let t = r as u64;
            let s = t as i32;
            return if s == r && r >= 0 { Ok(t) } else { Err(()) };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_u128(self) -> Result<u128, ()> {
            let r = Self::raw(self);
            let t = r as u128;
            let s = t as i32;
            return if s == r && r >= 0 { Ok(t) } else { Err(()) };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i8(self) -> Result<i8, ()> {
            let r = Self::raw(self);
            let t = r as i8;
            let s = t as i32;
            return if s == r && true { Ok(t) } else { Err(()) };
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i16(self) -> Result<i16, ()> {
            let r = Self::raw(self);
            let t = r as i16;
            let s = t as i32;
            return if s == r && true { Ok(t) } else { Err(()) };
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
        ) -> core::cmp::Ordering {
            let lhs = Self::raw(self);
            return PartialOrd::partial_cmp(&lhs, &rhs).unwrap();
        }
        #[inline(always)]
        pub const fn try_make(it: i32) -> Result<Self, i32> {
            return Ok(Self::_unchecked(it));
        }
        #[must_use]
        #[inline(always)]
        pub(self) const fn _unchecked(it: i32) -> Self {
            return {
                let () = <SeqOffset<D>>::__TYPEKIN_LAYOUT_ASSERT;
                Self(it, core::marker::PhantomData)
            };
        }
    }
    #[allow(clippy::unnecessary_cast)]
    impl<D: Direction> SeqOffset<D> {}
};

impl<D: Direction> SeqPoint<D> {
    #[inline(always)]
    const fn into_offset(self) -> SeqOffset<D> {
        return match SeqOffset::try_make(self.raw() as i32) {
            Ok(value) => value,
            Err(_) => panic!("unvalidated sequence offset rejected"),
        };
    }
    #[inline(always)]
    const fn offset_raw(offset: SeqOffset<D>) -> u32 {
        return offset.raw() as u32;
    }
}

struct ConnState {
    snd_nxt: SeqPoint<Out>,
    rcv_nxt: SeqPoint<In>,
    snd_una: SeqPoint<Out>,
}

impl ConnState {
    fn sent_but_not_acked(&self) -> SeqOffset<Out> {
        return self.snd_nxt - self.snd_una;
    }
}

fn sequence_arithmetic<D: Direction>() {
    let min = SeqPoint::<D>::make(u32::MIN);
    let max = SeqPoint::<D>::make(u32::MAX);
    let one = SeqOffset::<D>::make(1);
    let two = SeqOffset::<D>::make(2);
    let three = SeqOffset::<D>::make(3);

    assert!(max.precedes(min));
    assert!(!min.precedes(max));
    assert!(!min.precedes(min));
    // Half a sequence space has no defined serial ordering.
    let antipode = SeqPoint::<D>::make(1 << 31);
    assert!(!min.precedes(antipode));
    assert!(!antipode.precedes(min));

    assert_eq!(max + one, min);
    assert_eq!(one + two, three);
    assert_eq!(min - one, max);
    assert_eq!(two - three, SeqOffset::<D>::make(-1));
    assert_eq!(max - min, SeqOffset::<D>::make(-1));

    assert_eq!(size_of::<SeqPoint<D>>(), 4);
    assert_eq!(align_of::<SeqPoint<D>>(), align_of::<u32>());
    assert_eq!(size_of::<SeqOffset<D>>(), 4);
}

fn main() {
    sequence_arithmetic::<In>();
    sequence_arithmetic::<Out>();

    let mut conn = ConnState {
        snd_nxt: SeqPoint::make(7),
        rcv_nxt: SeqPoint::make(3),
        snd_una: SeqPoint::make(1),
    };
    let sent_but_not_acked: SeqOffset<Out> = conn.sent_but_not_acked();
    assert_eq!(sent_but_not_acked.raw(), 6);

    // Incoming progress cannot participate in the outgoing subtraction above.
    let received = SeqOffset::<In>::make(4);
    conn.rcv_nxt = conn.rcv_nxt + received;
    assert_eq!(conn.rcv_nxt.raw(), 7);
    assert_eq!(conn.sent_but_not_acked(), sent_but_not_acked);

    let acknowledged = SeqOffset::<Out>::make(2);
    conn.snd_una = conn.snd_una + acknowledged;
    assert_eq!(conn.sent_but_not_acked().raw(), 4);

    // Rejected by the types: point + point, offset + point, offset - point,
    // point<Out> + offset<In>, and conn.rcv_nxt - conn.snd_una.
    println!("TCP sequence spaces: 6 bytes in flight, 4 after acknowledgement");
}
