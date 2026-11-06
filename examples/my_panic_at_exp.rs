#[doc(hidden)]
mod __percent {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub(super) struct Percent(core::num::NonZeroU8);

    #[allow(unused_imports)]
    use super::*;
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::manual_range_contains)]
    const _: () = {
        trait Seal {
            fn conv_percent(self) -> u8;
        }
        trait Make: Seal {
            fn try_make(self) -> Result<Percent, u8>;
        }
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Rel: Seal {}
        impl Seal for Percent {
            #[inline(always)]
            fn conv_percent(self) -> u8 {
                return Self::raw(self);
            }
        }
        impl Make for Percent {
            #[inline(always)]
            fn try_make(self) -> Result<Percent, u8> {
                let raw = <Self as Seal>::conv_percent(self);
                return match Percent::_or_err(Some(raw)) {
                    Ok(it) => Ok(it),
                    Err(_) => Err(raw),
                };
            }
        }
        impl Rel for Percent {}
        impl Seal for u8 {
            #[inline(always)]
            fn conv_percent(self) -> u8 {
                return self as u8;
            }
        }
        impl Bit for u8 {}
        impl Math for u8 {}
        impl Rel for u8 {}
        impl core::str::FromStr for Percent {
            type Err = ();
            #[inline(always)]
            fn from_str(source: &str) -> Result<Self, Self::Err> {
                let value = match <u8 as core::str::FromStr>::from_str(source) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(());
                    }
                };
                return if value >= 1 && value <= 100 {
                    Ok(Self(match core::num::NonZeroU8::new(value) {
                        Some(value) => value,
                        None => {
                            core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    }))
                }
                else {
                    Err(())
                };
            }
        }
        impl Percent {
            #[doc = r" The number of bits in the wrapped primitive integer."]
            pub const BITS: u32 = u8::BITS;
        }
        impl Percent {
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
            pub const fn highest_one(self) -> Option<u32> {
                return Self::raw(self).highest_one();
            }
            #[inline(always)]
            pub const fn lowest_one(self) -> Option<u32> {
                return Self::raw(self).lowest_one();
            }
            #[inline(always)]
            pub const fn checked_ilog(
                self,
                base: Self,
            ) -> Option<u32> {
                return Self::raw(self).checked_ilog(Self::raw(base));
            }
            #[inline(always)]
            pub const fn checked_ilog2(self) -> Option<u32> {
                return Self::raw(self).checked_ilog2();
            }
            #[inline(always)]
            pub const fn checked_ilog10(self) -> Option<u32> {
                return Self::raw(self).checked_ilog10();
            }
            #[inline(always)]
            pub const fn to_be_bytes(self) -> [u8; size_of::<u8>()] {
                return Self::raw(self).to_be_bytes();
            }
            #[inline(always)]
            pub const fn to_le_bytes(self) -> [u8; size_of::<u8>()] {
                return Self::raw(self).to_le_bytes();
            }
            #[inline(always)]
            pub const fn to_ne_bytes(self) -> [u8; size_of::<u8>()] {
                return Self::raw(self).to_ne_bytes();
            }
            #[inline(always)]
            pub fn checked_add(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_add(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_sub(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_sub(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_neg(self) -> Option<Self> {
                return match Self::raw(self).checked_neg() {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_mul(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_mul(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_div(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_div(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_rem(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_rem(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_div_euclid(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_div_euclid(Self::raw(rhs))
                {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_rem_euclid(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_rem_euclid(Self::raw(rhs))
                {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_shl(
                self,
                rhs: u32,
            ) -> Option<Self> {
                return match Self::raw(self).checked_shl(rhs) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_shr(
                self,
                rhs: u32,
            ) -> Option<Self> {
                return match Self::raw(self).checked_shr(rhs) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_pow(
                self,
                exp: u32,
            ) -> Option<Self> {
                return match Self::raw(self).checked_pow(exp) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn cast_signed(self) -> i8 {
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
            ) -> u8 {
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
            pub fn checked_next_power_of_two(self) -> Option<Self> {
                return match Self::raw(self).checked_next_power_of_two() {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_next_multiple_of(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self)
                    .checked_next_multiple_of(Self::raw(rhs))
                {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
        }
        impl Into<usize> for Percent {
            #[inline(always)]
            fn into(self) -> usize {
                return Self::into_usize(self);
            }
        }
        impl Into<isize> for Percent {
            #[inline(always)]
            fn into(self) -> isize {
                return Self::into_isize(self);
            }
        }
        impl Into<u8> for Percent {
            #[inline(always)]
            fn into(self) -> u8 {
                return Self::into_u8(self);
            }
        }
        impl Into<u16> for Percent {
            #[inline(always)]
            fn into(self) -> u16 {
                return Self::into_u16(self);
            }
        }
        impl Into<u32> for Percent {
            #[inline(always)]
            fn into(self) -> u32 {
                return Self::into_u32(self);
            }
        }
        impl Into<u64> for Percent {
            #[inline(always)]
            fn into(self) -> u64 {
                return Self::into_u64(self);
            }
        }
        impl Into<u128> for Percent {
            #[inline(always)]
            fn into(self) -> u128 {
                return Self::into_u128(self);
            }
        }
        impl Into<i16> for Percent {
            #[inline(always)]
            fn into(self) -> i16 {
                return Self::into_i16(self);
            }
        }
        impl Into<i32> for Percent {
            #[inline(always)]
            fn into(self) -> i32 {
                return Self::into_i32(self);
            }
        }
        impl Into<i64> for Percent {
            #[inline(always)]
            fn into(self) -> i64 {
                return Self::into_i64(self);
            }
        }
        impl Into<i128> for Percent {
            #[inline(always)]
            fn into(self) -> i128 {
                return Self::into_i128(self);
            }
        }
        impl Into<f32> for Percent {
            #[inline(always)]
            fn into(self) -> f32 {
                return Self::into_f32(self);
            }
        }
        impl Into<f64> for Percent {
            #[inline(always)]
            fn into(self) -> f64 {
                return Self::into_f64(self);
            }
        }
        impl TryInto<i8> for Percent {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i8, Self::Error> {
                return Self::try_into_i8(self);
            }
        }
        impl core::ops::Shr<usize> for Percent {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        impl core::ops::Shl<usize> for Percent {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        impl core::fmt::Debug for Percent {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f.debug_tuple("Percent").field(&self.0.get()).finish();
            }
        }
        impl core::hash::Hash for Percent {
            #[inline(always)]
            fn hash<H>(
                &self,
                state: &mut H,
            ) where
                H: core::hash::Hasher,
            {
                return core::hash::Hash::hash(&self.0.get(), state);
            }
        }
        impl Eq for Percent {}
        impl Ord for Percent {
            #[inline(always)]
            fn cmp(
                &self,
                that: &Self,
            ) -> core::cmp::Ordering {
                return self.partial_cmp(that).unwrap();
            }
        }
        if !(size_of::<Percent>() == size_of::<u8>()) {
            core::panic!("invalid integral storage size");
        };
        if !(align_of::<Percent>() == align_of::<u8>()) {
            core::panic!("invalid integral storage alignment");
        };
        impl core::ops::Add for Percent {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn add(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._add(rhs.0.get());
            }
        }
        impl<T> core::ops::Add<T> for Percent
        where
            T: Math + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_percent(rhs);
                return self._add(it);
            }
        }
        impl core::ops::Sub for Percent {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn sub(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._sub(rhs.0.get());
            }
        }
        impl<T> core::ops::Sub<T> for Percent
        where
            T: Math + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_percent(rhs);
                return self._sub(it);
            }
        }
        impl core::ops::Mul for Percent {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn mul(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._mul(rhs.0.get());
            }
        }
        impl<T> core::ops::Mul<T> for Percent
        where
            T: Math + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_percent(rhs);
                return self._mul(it);
            }
        }
        impl core::ops::Div for Percent {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn div(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._div(rhs.0.get());
            }
        }
        impl<T> core::ops::Div<T> for Percent
        where
            T: Math + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_percent(rhs);
                return self._div(it);
            }
        }
        impl core::ops::Rem for Percent {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn rem(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._rem(rhs.0.get());
            }
        }
        impl<T> core::ops::Rem<T> for Percent
        where
            T: Math + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_percent(rhs);
                return self._rem(it);
            }
        }
        impl core::ops::BitAnd for Percent {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn bitand(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitand(rhs.0.get());
            }
        }
        impl<T> core::ops::BitAnd<T> for Percent
        where
            T: Bit + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_percent(rhs);
                return self._bitand(it);
            }
        }
        impl core::ops::BitOr for Percent {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn bitor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitor(rhs.0.get());
            }
        }
        impl<T> core::ops::BitOr<T> for Percent
        where
            T: Bit + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_percent(rhs);
                return self._bitor(it);
            }
        }
        impl core::ops::BitXor for Percent {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitxor(rhs.0.get());
            }
        }
        impl<T> core::ops::BitXor<T> for Percent
        where
            T: Bit + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_percent(rhs);
                return self._bitxor(it);
            }
        }
        impl core::ops::Not for Percent {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        impl core::fmt::Binary for Percent {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl core::fmt::Octal for Percent {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl core::fmt::LowerHex for Percent {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl core::fmt::UpperHex for Percent {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        impl<T> PartialEq<T> for Percent
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_percent(*rhs);
                return lhs == rhs;
            }
        }
        impl<T> PartialOrd<T> for Percent
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> Option<core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_percent(*rhs);
                return PartialOrd::partial_cmp(&lhs, &rhs);
            }
            #[inline(always)]
            fn lt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) < Seal::conv_percent(*rhs);
            }
            #[inline(always)]
            fn le(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) <= Seal::conv_percent(*rhs);
            }
            #[inline(always)]
            fn gt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) > Seal::conv_percent(*rhs);
            }
            #[inline(always)]
            fn ge(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) >= Seal::conv_percent(*rhs);
            }
        }
        impl Percent {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn try_of<T>(it: T) -> Result<Percent, u8>
            where
                T: Make,
            {
                return <T as Make>::try_make(it);
            }
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> u8 {
                return self.0.get();
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
            pub const fn into_isize(self) -> isize {
                let it = Self::raw(self);
                return it as isize;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u8(self) -> u8 {
                let it = Self::raw(self);
                return it as u8;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u16(self) -> u16 {
                let it = Self::raw(self);
                return it as u16;
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
            pub const fn into_i16(self) -> i16 {
                let it = Self::raw(self);
                return it as i16;
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
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_f32(self) -> f32 {
                let it = Self::raw(self);
                return it as f32;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_f64(self) -> f64 {
                let it = Self::raw(self);
                return it as f64;
            }
            #[inline(always)]
            pub fn try_map<F>(
                self,
                function: F,
            ) -> Result<Self, u8>
            where
                F: FnOnce(u8) -> u8,
            {
                let value = function(Self::raw(self));
                return if value >= 1 && value <= 100 {
                    Ok(Self(match core::num::NonZeroU8::new(value) {
                        Some(value) => value,
                        None => {
                            core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    }))
                }
                else {
                    Err(value)
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(self) -> Result<usize, ()> {
                return Ok(Self::raw(self) as usize);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) -> Result<isize, ()> {
                return Ok(Self::raw(self) as isize);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u8(self) -> Result<u8, ()> {
                return Ok(Self::raw(self) as u8);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u16(self) -> Result<u16, ()> {
                return Ok(Self::raw(self) as u16);
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
            pub const fn try_into_i16(self) -> Result<i16, ()> {
                return Ok(Self::raw(self) as i16);
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
            pub const fn try_into_f32(self) -> Result<f32, ()> {
                return Ok(Self::raw(self) as f32);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_f64(self) -> Result<f64, ()> {
                return Ok(Self::raw(self) as f64);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i8(self) -> Result<i8, ()> {
                let r = Self::raw(self);
                let t = r as i8;
                let s = t as u8;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            pub(self) fn _add(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = lhs.checked_add(rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) fn _sub(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = lhs.checked_sub(rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) fn _mul(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = lhs.checked_mul(rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) fn _div(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = lhs.checked_div(rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) fn _rem(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = lhs.checked_rem(rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) fn _bitxor(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = Some(lhs ^ rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) fn _bitand(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = Some(lhs & rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) fn _bitor(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = Some(lhs | rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) fn _shr(
                &self,
                rhs: usize,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = if rhs > u32::MAX as usize {
                    None
                }
                else {
                    lhs.checked_shr(rhs as u32)
                };
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) fn _shl(
                &self,
                rhs: usize,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = if rhs > u32::MAX as usize {
                    None
                }
                else {
                    lhs.checked_shl(rhs as u32)
                };
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) fn _not(&self) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = Some(!lhs);
                return Self::_or_err(it);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _eq(
                self,
                rhs: u8,
            ) -> bool {
                let lhs = Self::raw(self);
                return lhs == rhs;
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) fn _cmp(
                self,
                rhs: u8,
            ) -> core::cmp::Ordering {
                let lhs = Self::raw(self);
                return PartialOrd::partial_cmp(&lhs, &rhs).unwrap();
            }
            #[inline(always)]
            pub fn try_make(it: u8) -> Result<Self, u8> {
                return if it >= 1 && it <= 100 {
                    return Ok(Self(match core::num::NonZeroU8::new(it) {
                        Some(value) => value,
                        None => {
                            core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    }));
                }
                else {
                    Err(it)
                };
            }
            #[inline(always)]
            pub(self) fn _or_err(it: Option<u8>) -> Result<Self, Option<u8>> {
                return match it {
                    Some(it) if it >= 1 && it <= 100 => {
                        Ok(Self(match core::num::NonZeroU8::new(it) {
                            Some(value) => value,
                            None => {
                                core::panic!(
                                    "zero cannot be stored by this integral type"
                                )
                            }
                        }))
                    }
                    it => Err(it),
                };
            }
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_RAW: (bool, u32) = (false, u8::BITS);
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_FORM: Option<(
                u128,
                &'static [(u128, bool, u128, bool, u128)],
            )> = Some((1u128, &[(0u128, false, 1u128, false, 100u128)]));
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_WHY: &'static str = "";
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_TABLE: Option<[bool; 256]> = {
                let mut table = [false; 256];
                let mut index = 0;
                while index < 256 {
                    let value: u8 = index as u8;
                    table[index] = value >= 1 && value <= 100;
                    index += 1;
                }
                Some(table)
            };
            #[doc(hidden)]
            #[inline(always)]
            pub const fn __typekin_within_v1_raw(self) -> u8 {
                return self.0.get();
            }
            #[inline(always)]
            pub(self) fn _make(value: u8) -> Option<Self> {
                return if value >= 1 && value <= 100 {
                    Some(Self(match core::num::NonZeroU8::new(value) {
                        Some(value) => value,
                        None => {
                            core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    }))
                }
                else {
                    None
                };
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl Percent {
            #[must_use]
            #[inline(always)]
            pub const fn byte0(self) -> u8 {
                return (Self::raw(self) >> 8 * 0usize & 0xFF as u8) as u8;
            }
        }
    };
}
#[allow(unused_imports)]
use __percent::Percent;

#[doc(hidden)]
mod __count {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub(super) struct Count(u8);

    #[allow(unused_imports)]
    use super::*;
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::manual_range_contains)]
    const _: () = {
        trait Seal {
            fn conv_count(self) -> u8;
        }
        trait Make: Seal {
            fn try_make(self) -> Result<Count, u8>;
        }
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Rel: Seal {}
        impl Seal for Count {
            #[inline(always)]
            fn conv_count(self) -> u8 {
                return Self::raw(self);
            }
        }
        impl Make for Count {
            #[inline(always)]
            fn try_make(self) -> Result<Count, u8> {
                let raw = <Self as Seal>::conv_count(self);
                return match Count::_or_err(Some(raw)) {
                    Ok(it) => Ok(it),
                    Err(_) => Err(raw),
                };
            }
        }
        impl Rel for Count {}
        impl Seal for u8 {
            #[inline(always)]
            fn conv_count(self) -> u8 {
                return self as u8;
            }
        }
        impl Bit for u8 {}
        impl Make for u8 {
            #[inline(always)]
            fn try_make(self) -> Result<Count, u8> {
                let raw = <Self as Seal>::conv_count(self);
                return match Count::_or_err(Some(raw)) {
                    Ok(it) => Ok(it),
                    Err(_) => Err(raw),
                };
            }
        }
        impl Math for u8 {}
        impl Rel for u8 {}
        impl AsRef<u8> for Count {
            #[inline(always)]
            fn as_ref(&self) -> &u8 {
                return &self.0;
            }
        }
        impl core::str::FromStr for Count {
            type Err = ();
            #[inline(always)]
            fn from_str(source: &str) -> Result<Self, Self::Err> {
                let value = match <u8 as core::str::FromStr>::from_str(source) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(());
                    }
                };
                return Ok(Self(value));
            }
        }
        impl Count {
            #[doc = r" The number of bits in the wrapped primitive integer."]
            pub const BITS: u32 = u8::BITS;
        }
        impl Count {
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
            pub const fn highest_one(self) -> Option<u32> {
                return Self::raw(self).highest_one();
            }
            #[inline(always)]
            pub const fn lowest_one(self) -> Option<u32> {
                return Self::raw(self).lowest_one();
            }
            #[inline(always)]
            pub const fn checked_ilog(
                self,
                base: Self,
            ) -> Option<u32> {
                return Self::raw(self).checked_ilog(Self::raw(base));
            }
            #[inline(always)]
            pub const fn checked_ilog2(self) -> Option<u32> {
                return Self::raw(self).checked_ilog2();
            }
            #[inline(always)]
            pub const fn checked_ilog10(self) -> Option<u32> {
                return Self::raw(self).checked_ilog10();
            }
            #[inline(always)]
            pub const fn to_be_bytes(self) -> [u8; size_of::<u8>()] {
                return Self::raw(self).to_be_bytes();
            }
            #[inline(always)]
            pub const fn to_le_bytes(self) -> [u8; size_of::<u8>()] {
                return Self::raw(self).to_le_bytes();
            }
            #[inline(always)]
            pub const fn to_ne_bytes(self) -> [u8; size_of::<u8>()] {
                return Self::raw(self).to_ne_bytes();
            }
            #[inline(always)]
            pub fn checked_add(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_add(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_sub(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_sub(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_neg(self) -> Option<Self> {
                return match Self::raw(self).checked_neg() {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_mul(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_mul(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_div(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_div(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_rem(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_rem(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_div_euclid(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_div_euclid(Self::raw(rhs))
                {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_rem_euclid(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_rem_euclid(Self::raw(rhs))
                {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_shl(
                self,
                rhs: u32,
            ) -> Option<Self> {
                return match Self::raw(self).checked_shl(rhs) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_shr(
                self,
                rhs: u32,
            ) -> Option<Self> {
                return match Self::raw(self).checked_shr(rhs) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_pow(
                self,
                exp: u32,
            ) -> Option<Self> {
                return match Self::raw(self).checked_pow(exp) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn cast_signed(self) -> i8 {
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
            ) -> u8 {
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
            pub fn checked_next_power_of_two(self) -> Option<Self> {
                return match Self::raw(self).checked_next_power_of_two() {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub fn checked_next_multiple_of(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self)
                    .checked_next_multiple_of(Self::raw(rhs))
                {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
        }
        impl Into<usize> for Count {
            #[inline(always)]
            fn into(self) -> usize {
                return Self::into_usize(self);
            }
        }
        impl Into<isize> for Count {
            #[inline(always)]
            fn into(self) -> isize {
                return Self::into_isize(self);
            }
        }
        impl Into<u8> for Count {
            #[inline(always)]
            fn into(self) -> u8 {
                return Self::into_u8(self);
            }
        }
        impl Into<u16> for Count {
            #[inline(always)]
            fn into(self) -> u16 {
                return Self::into_u16(self);
            }
        }
        impl Into<u32> for Count {
            #[inline(always)]
            fn into(self) -> u32 {
                return Self::into_u32(self);
            }
        }
        impl Into<u64> for Count {
            #[inline(always)]
            fn into(self) -> u64 {
                return Self::into_u64(self);
            }
        }
        impl Into<u128> for Count {
            #[inline(always)]
            fn into(self) -> u128 {
                return Self::into_u128(self);
            }
        }
        impl Into<i16> for Count {
            #[inline(always)]
            fn into(self) -> i16 {
                return Self::into_i16(self);
            }
        }
        impl Into<i32> for Count {
            #[inline(always)]
            fn into(self) -> i32 {
                return Self::into_i32(self);
            }
        }
        impl Into<i64> for Count {
            #[inline(always)]
            fn into(self) -> i64 {
                return Self::into_i64(self);
            }
        }
        impl Into<i128> for Count {
            #[inline(always)]
            fn into(self) -> i128 {
                return Self::into_i128(self);
            }
        }
        impl Into<f32> for Count {
            #[inline(always)]
            fn into(self) -> f32 {
                return Self::into_f32(self);
            }
        }
        impl Into<f64> for Count {
            #[inline(always)]
            fn into(self) -> f64 {
                return Self::into_f64(self);
            }
        }
        impl TryInto<i8> for Count {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i8, Self::Error> {
                return Self::try_into_i8(self);
            }
        }
        impl core::ops::Shr<usize> for Count {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        impl core::ops::Shl<usize> for Count {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        impl core::fmt::Debug for Count {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f.debug_tuple("Count").field(&self.0).finish();
            }
        }
        impl core::hash::Hash for Count {
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
        impl Eq for Count {}
        impl Ord for Count {
            #[inline(always)]
            fn cmp(
                &self,
                that: &Self,
            ) -> core::cmp::Ordering {
                return self.partial_cmp(that).unwrap();
            }
        }
        if !(size_of::<Count>() == size_of::<u8>()) {
            core::panic!("invalid integral storage size");
        };
        if !(align_of::<Count>() == align_of::<u8>()) {
            core::panic!("invalid integral storage alignment");
        };
        impl core::ops::Add for Count {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn add(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._add(rhs.0);
            }
        }
        impl<T> core::ops::Add<T> for Count
        where
            T: Math + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_count(rhs);
                return self._add(it);
            }
        }
        impl core::ops::Sub for Count {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn sub(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._sub(rhs.0);
            }
        }
        impl<T> core::ops::Sub<T> for Count
        where
            T: Math + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_count(rhs);
                return self._sub(it);
            }
        }
        impl core::ops::Mul for Count {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn mul(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._mul(rhs.0);
            }
        }
        impl<T> core::ops::Mul<T> for Count
        where
            T: Math + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_count(rhs);
                return self._mul(it);
            }
        }
        impl core::ops::Div for Count {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn div(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._div(rhs.0);
            }
        }
        impl<T> core::ops::Div<T> for Count
        where
            T: Math + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_count(rhs);
                return self._div(it);
            }
        }
        impl core::ops::Rem for Count {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn rem(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._rem(rhs.0);
            }
        }
        impl<T> core::ops::Rem<T> for Count
        where
            T: Math + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_count(rhs);
                return self._rem(it);
            }
        }
        impl core::ops::BitAnd for Count {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn bitand(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitand(rhs.0);
            }
        }
        impl<T> core::ops::BitAnd<T> for Count
        where
            T: Bit + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_count(rhs);
                return self._bitand(it);
            }
        }
        impl core::ops::BitOr for Count {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn bitor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitor(rhs.0);
            }
        }
        impl<T> core::ops::BitOr<T> for Count
        where
            T: Bit + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_count(rhs);
                return self._bitor(it);
            }
        }
        impl core::ops::BitXor for Count {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitxor(rhs.0);
            }
        }
        impl<T> core::ops::BitXor<T> for Count
        where
            T: Bit + Seal,
        {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_count(rhs);
                return self._bitxor(it);
            }
        }
        impl core::ops::Not for Count {
            type Output = Result<Self, Option<u8>>;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        impl core::fmt::Binary for Count {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl core::fmt::Octal for Count {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl core::fmt::LowerHex for Count {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl core::fmt::UpperHex for Count {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        impl<T> PartialEq<T> for Count
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_count(*rhs);
                return lhs == rhs;
            }
        }
        impl<T> PartialOrd<T> for Count
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> Option<core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_count(*rhs);
                return PartialOrd::partial_cmp(&lhs, &rhs);
            }
            #[inline(always)]
            fn lt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) < Seal::conv_count(*rhs);
            }
            #[inline(always)]
            fn le(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) <= Seal::conv_count(*rhs);
            }
            #[inline(always)]
            fn gt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) > Seal::conv_count(*rhs);
            }
            #[inline(always)]
            fn ge(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) >= Seal::conv_count(*rhs);
            }
        }
        impl Count {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn try_of<T>(it: T) -> Result<Count, u8>
            where
                T: Make,
            {
                return <T as Make>::try_make(it);
            }
            #[inline(always)]
            pub const fn make(it: u8) -> Count {
                return Self::_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> u8 {
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
            pub const fn into_isize(self) -> isize {
                let it = Self::raw(self);
                return it as isize;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u8(self) -> u8 {
                let it = Self::raw(self);
                return it as u8;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_u16(self) -> u16 {
                let it = Self::raw(self);
                return it as u16;
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
            pub const fn into_i16(self) -> i16 {
                let it = Self::raw(self);
                return it as i16;
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
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_f32(self) -> f32 {
                let it = Self::raw(self);
                return it as f32;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_f64(self) -> f64 {
                let it = Self::raw(self);
                return it as f64;
            }
            #[inline(always)]
            pub fn try_map<F>(
                self,
                function: F,
            ) -> Result<Self, u8>
            where
                F: FnOnce(u8) -> u8,
            {
                let value = function(Self::raw(self));
                return Ok(Self(value));
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(self) -> Result<usize, ()> {
                return Ok(Self::raw(self) as usize);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) -> Result<isize, ()> {
                return Ok(Self::raw(self) as isize);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u8(self) -> Result<u8, ()> {
                return Ok(Self::raw(self) as u8);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u16(self) -> Result<u16, ()> {
                return Ok(Self::raw(self) as u16);
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
            pub const fn try_into_i16(self) -> Result<i16, ()> {
                return Ok(Self::raw(self) as i16);
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
            pub const fn try_into_f32(self) -> Result<f32, ()> {
                return Ok(Self::raw(self) as f32);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_f64(self) -> Result<f64, ()> {
                return Ok(Self::raw(self) as f64);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i8(self) -> Result<i8, ()> {
                let r = Self::raw(self);
                let t = r as i8;
                let s = t as u8;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            pub(self) const fn _add(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = lhs.checked_add(rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) const fn _sub(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = lhs.checked_sub(rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) const fn _mul(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = lhs.checked_mul(rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) const fn _div(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = lhs.checked_div(rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) const fn _rem(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = lhs.checked_rem(rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) const fn _bitxor(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = Some(lhs ^ rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) const fn _bitand(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = Some(lhs & rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) const fn _bitor(
                &self,
                rhs: u8,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = Some(lhs | rhs);
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) const fn _shr(
                &self,
                rhs: usize,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = if rhs > u32::MAX as usize {
                    None
                }
                else {
                    lhs.checked_shr(rhs as u32)
                };
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) const fn _shl(
                &self,
                rhs: usize,
            ) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = if rhs > u32::MAX as usize {
                    None
                }
                else {
                    lhs.checked_shl(rhs as u32)
                };
                return Self::_or_err(it);
            }
            #[inline(always)]
            pub(self) const fn _not(&self) -> Result<Self, Option<u8>> {
                let lhs = Self::raw(*self);
                let it = Some(!lhs);
                return Self::_or_err(it);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _eq(
                self,
                rhs: u8,
            ) -> bool {
                let lhs = Self::raw(self);
                return lhs == rhs;
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) fn _cmp(
                self,
                rhs: u8,
            ) -> core::cmp::Ordering {
                let lhs = Self::raw(self);
                return PartialOrd::partial_cmp(&lhs, &rhs).unwrap();
            }
            #[inline(always)]
            pub const fn try_make(it: u8) -> Result<Self, u8> {
                return Ok(Self::_or_panic(it));
            }
            #[must_use]
            #[inline(always)]
            pub(self) const fn _or_panic(it: u8) -> Self {
                return Self(it);
            }
            #[inline(always)]
            pub(self) const fn _or_err(
                it: Option<u8>
            ) -> Result<Self, Option<u8>> {
                return match it {
                    Some(it) => Ok(Self(it)),
                    it => Err(it),
                };
            }
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_RAW: (bool, u32) = (false, u8::BITS);
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_FORM: Option<(
                u128,
                &'static [(u128, bool, u128, bool, u128)],
            )> = Some((1u128, &[(0u128, false, 0u128, false, 255u128)]));
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_WHY: &'static str = "";
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_TABLE: Option<[bool; 256]> =
                Some([true; 256]);
            #[doc(hidden)]
            #[inline(always)]
            pub const fn __typekin_within_v1_raw(self) -> u8 {
                return self.0;
            }
            #[inline(always)]
            pub(self) fn _make(value: u8) -> Option<Self> {
                return Some(Self(value));
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl Count {
            #[must_use]
            #[inline(always)]
            pub const fn byte0(self) -> u8 {
                return (Self::raw(self) >> 8 * 0usize & 0xFF as u8) as u8;
            }
        }
    };
}
#[allow(unused_imports)]
use __count::Count;

fn main() {
    let ten = Percent::try_make(10).unwrap();
    let ninety = Percent::try_make(90).unwrap();
    assert_eq!((ten + ten).map(Percent::raw), Ok(20));
    // An invalid result reports its raw value.
    assert_eq!((ninety + ninety).map(Percent::raw), Err(Some(180)));

    // A result that does not exist reports none.
    let quotient = Count::make(1) / Count::make(0);
    assert_eq!(quotient.map(Count::raw), Err(None));
    let sum = Count::make(u8::MAX) + Count::make(1);
    assert_eq!(sum.map(Count::raw), Err(None));
}
