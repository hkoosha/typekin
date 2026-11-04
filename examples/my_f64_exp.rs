#[doc(hidden)]
mod __meters {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct Meters(f64);

    #[allow(unused_imports)]
    use super::*;
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    const _: () = {
        trait Seal {
            fn conv_meters(self) -> f64;
        }
        trait Make: Seal {
            fn make(self) -> Meters;
        }
        trait Math: Seal {}
        trait Rel: Seal {}
        #[doc = "# Safety"]
        #[doc = ""]
        #[doc = "Implementing this trait asserts that values \
                        produced by the sealed conversion satisfy every \
                        invariant required by the target type."]
        unsafe trait Unsafe: Seal {}
        impl Seal for Meters {
            #[inline(always)]
            fn conv_meters(self) -> f64 {
                return Self::raw(self);
            }
        }
        impl Make for Meters {
            #[inline(always)]
            fn make(self) -> Meters {
                let raw = <Self as Seal>::conv_meters(self);
                return Meters::_or_panic(raw);
            }
        }
        impl Rel for Meters {}
        impl Seal for f32 {
            #[inline(always)]
            fn conv_meters(self) -> f64 {
                return self as f64;
            }
        }
        impl Make for f32 {
            #[inline(always)]
            fn make(self) -> Meters {
                let raw = <Self as Seal>::conv_meters(self);
                return Meters::_or_panic(raw);
            }
        }
        impl Seal for f64 {
            #[inline(always)]
            fn conv_meters(self) -> f64 {
                return self as f64;
            }
        }
        impl Make for f64 {
            #[inline(always)]
            fn make(self) -> Meters {
                let raw = <Self as Seal>::conv_meters(self);
                return Meters::_or_panic(raw);
            }
        }
        impl Math for f64 {}
        impl Rel for f64 {}
        impl Seal for i16 {
            #[inline(always)]
            fn conv_meters(self) -> f64 {
                return self as f64;
            }
        }
        impl Make for i16 {
            #[inline(always)]
            fn make(self) -> Meters {
                let raw = <Self as Seal>::conv_meters(self);
                return Meters::_or_panic(raw);
            }
        }
        impl Seal for i32 {
            #[inline(always)]
            fn conv_meters(self) -> f64 {
                return self as f64;
            }
        }
        impl Make for i32 {
            #[inline(always)]
            fn make(self) -> Meters {
                let raw = <Self as Seal>::conv_meters(self);
                return Meters::_or_panic(raw);
            }
        }
        impl Seal for i8 {
            #[inline(always)]
            fn conv_meters(self) -> f64 {
                return self as f64;
            }
        }
        impl Make for i8 {
            #[inline(always)]
            fn make(self) -> Meters {
                let raw = <Self as Seal>::conv_meters(self);
                return Meters::_or_panic(raw);
            }
        }
        #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
        impl Seal for isize {
            #[inline(always)]
            fn conv_meters(self) -> f64 {
                return self as f64;
            }
        }
        #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
        impl Make for isize {
            #[inline(always)]
            fn make(self) -> Meters {
                let raw = <Self as Seal>::conv_meters(self);
                return Meters::_or_panic(raw);
            }
        }
        impl Seal for u16 {
            #[inline(always)]
            fn conv_meters(self) -> f64 {
                return self as f64;
            }
        }
        impl Make for u16 {
            #[inline(always)]
            fn make(self) -> Meters {
                let raw = <Self as Seal>::conv_meters(self);
                return Meters::_or_panic(raw);
            }
        }
        impl Seal for u32 {
            #[inline(always)]
            fn conv_meters(self) -> f64 {
                return self as f64;
            }
        }
        impl Make for u32 {
            #[inline(always)]
            fn make(self) -> Meters {
                let raw = <Self as Seal>::conv_meters(self);
                return Meters::_or_panic(raw);
            }
        }
        impl Seal for u8 {
            #[inline(always)]
            fn conv_meters(self) -> f64 {
                return self as f64;
            }
        }
        impl Make for u8 {
            #[inline(always)]
            fn make(self) -> Meters {
                let raw = <Self as Seal>::conv_meters(self);
                return Meters::_or_panic(raw);
            }
        }
        #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
        impl Seal for usize {
            #[inline(always)]
            fn conv_meters(self) -> f64 {
                return self as f64;
            }
        }
        #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
        impl Make for usize {
            #[inline(always)]
            fn make(self) -> Meters {
                let raw = <Self as Seal>::conv_meters(self);
                return Meters::_or_panic(raw);
            }
        }
        impl AsRef<f64> for Meters {
            #[inline(always)]
            fn as_ref(&self) -> &f64 {
                return &self.0;
            }
        }
        impl core::str::FromStr for Meters {
            type Err = ();
            #[inline(always)]
            fn from_str(source: &str) -> Result<Self, Self::Err> {
                let value = match <f64 as core::str::FromStr>::from_str(source)
                {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(());
                    }
                };
                return Ok(Self(value));
            }
        }
        impl Meters {
            pub const RADIX: u32 = f64::RADIX;
            pub const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;
            pub const DIGITS: u32 = f64::DIGITS;
            pub const MIN_EXP: i32 = f64::MIN_EXP;
            pub const MAX_EXP: i32 = f64::MAX_EXP;
            pub const MIN_10_EXP: i32 = f64::MIN_10_EXP;
            pub const MAX_10_EXP: i32 = f64::MAX_10_EXP;
        }
        impl Meters {
            #[must_use]
            #[inline(always)]
            pub const fn is_nan(self) -> bool {
                return Self::raw(self).is_nan();
            }
            #[must_use]
            #[inline(always)]
            pub const fn is_infinite(self) -> bool {
                return Self::raw(self).is_infinite();
            }
            #[must_use]
            #[inline(always)]
            pub const fn is_finite(self) -> bool {
                return Self::raw(self).is_finite();
            }
            #[must_use]
            #[inline(always)]
            pub const fn is_subnormal(self) -> bool {
                return Self::raw(self).is_subnormal();
            }
            #[must_use]
            #[inline(always)]
            pub const fn is_normal(self) -> bool {
                return Self::raw(self).is_normal();
            }
            #[must_use]
            #[inline(always)]
            pub const fn is_sign_positive(self) -> bool {
                return Self::raw(self).is_sign_positive();
            }
            #[must_use]
            #[inline(always)]
            pub const fn is_sign_negative(self) -> bool {
                return Self::raw(self).is_sign_negative();
            }
            #[must_use]
            #[inline(always)]
            pub const fn classify(self) -> core::num::FpCategory {
                return Self::raw(self).classify();
            }
            #[must_use]
            #[inline(always)]
            pub const fn to_bits(self) -> u64 {
                return Self::raw(self).to_bits();
            }
            #[inline(always)]
            #[track_caller]
            pub fn from_bits(bits: u64) -> Self {
                return Self::_result_or_panic(f64::from_bits(bits));
            }
            #[must_use]
            #[inline(always)]
            pub const fn to_ne_bytes(self) -> [u8; size_of::<f64>()] {
                return Self::raw(self).to_ne_bytes();
            }
            #[inline(always)]
            #[track_caller]
            pub fn from_ne_bytes(bytes: [u8; size_of::<f64>()]) -> Self {
                return Self::_result_or_panic(f64::from_ne_bytes(bytes));
            }
            #[must_use]
            #[inline(always)]
            pub const fn to_be_bytes(self) -> [u8; size_of::<f64>()] {
                return Self::raw(self).to_be_bytes();
            }
            #[inline(always)]
            #[track_caller]
            pub fn from_be_bytes(bytes: [u8; size_of::<f64>()]) -> Self {
                return Self::_result_or_panic(f64::from_be_bytes(bytes));
            }
            #[must_use]
            #[inline(always)]
            pub const fn to_le_bytes(self) -> [u8; size_of::<f64>()] {
                return Self::raw(self).to_le_bytes();
            }
            #[inline(always)]
            #[track_caller]
            pub fn from_le_bytes(bytes: [u8; size_of::<f64>()]) -> Self {
                return Self::_result_or_panic(f64::from_le_bytes(bytes));
            }
            #[must_use]
            #[inline(always)]
            pub fn total_cmp(
                &self,
                that: &Self,
            ) -> core::cmp::Ordering {
                return Self::raw(*self).total_cmp(&Self::raw(*that));
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub fn abs(self) -> Self {
                let it = Self::raw(self).abs();
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub fn copysign(
                self,
                sign: Self,
            ) -> Self {
                let it = Self::raw(self).copysign(Self::raw(sign));
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub fn recip(self) -> Self {
                let it = Self::raw(self).recip();
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub fn to_degrees(self) -> Self {
                let it = Self::raw(self).to_degrees();
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub fn to_radians(self) -> Self {
                let it = Self::raw(self).to_radians();
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub fn next_up(self) -> Self {
                let it = Self::raw(self).next_up();
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub fn next_down(self) -> Self {
                let it = Self::raw(self).next_down();
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub fn midpoint(
                self,
                that: Self,
            ) -> Self {
                let it = Self::raw(self).midpoint(Self::raw(that));
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub fn min(
                self,
                that: Self,
            ) -> Self {
                let it = Self::raw(self).min(Self::raw(that));
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub fn max(
                self,
                that: Self,
            ) -> Self {
                let it = Self::raw(self).max(Self::raw(that));
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub fn clamp(
                self,
                min: Self,
                max: Self,
            ) -> Self {
                let it = Self::raw(self).clamp(Self::raw(min), Self::raw(max));
                return Self::_result_or_panic(it);
            }
        }
        impl Into<f64> for Meters {
            #[inline(always)]
            fn into(self) -> f64 {
                return Self::into_f64(self);
            }
        }
        impl TryInto<usize> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<usize, Self::Error> {
                return Self::try_into_usize(self);
            }
        }
        impl TryInto<isize> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<isize, Self::Error> {
                return Self::try_into_isize(self);
            }
        }
        impl TryInto<u8> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u8, Self::Error> {
                return Self::try_into_u8(self);
            }
        }
        impl TryInto<u16> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u16, Self::Error> {
                return Self::try_into_u16(self);
            }
        }
        impl TryInto<u32> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u32, Self::Error> {
                return Self::try_into_u32(self);
            }
        }
        impl TryInto<u64> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u64, Self::Error> {
                return Self::try_into_u64(self);
            }
        }
        impl TryInto<u128> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u128, Self::Error> {
                return Self::try_into_u128(self);
            }
        }
        impl TryInto<i8> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i8, Self::Error> {
                return Self::try_into_i8(self);
            }
        }
        impl TryInto<i16> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i16, Self::Error> {
                return Self::try_into_i16(self);
            }
        }
        impl TryInto<i32> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i32, Self::Error> {
                return Self::try_into_i32(self);
            }
        }
        impl TryInto<i64> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i64, Self::Error> {
                return Self::try_into_i64(self);
            }
        }
        impl TryInto<i128> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i128, Self::Error> {
                return Self::try_into_i128(self);
            }
        }
        impl TryInto<f32> for Meters {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<f32, Self::Error> {
                return Self::try_into_f32(self);
            }
        }
        impl core::ops::AddAssign for Meters {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._add(rhs.0);
            }
        }
        impl<T> core::ops::AddAssign<T> for Meters
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_meters(rhs);
                *self = self._add(it);
            }
        }
        impl core::ops::SubAssign for Meters {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._sub(rhs.0);
            }
        }
        impl<T> core::ops::SubAssign<T> for Meters
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_meters(rhs);
                *self = self._sub(it);
            }
        }
        impl core::ops::MulAssign for Meters {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._mul(rhs.0);
            }
        }
        impl<T> core::ops::MulAssign<T> for Meters
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_meters(rhs);
                *self = self._mul(it);
            }
        }
        impl core::ops::DivAssign for Meters {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._div(rhs.0);
            }
        }
        impl<T> core::ops::DivAssign<T> for Meters
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_meters(rhs);
                *self = self._div(it);
            }
        }
        impl core::ops::RemAssign for Meters {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._rem(rhs.0);
            }
        }
        impl<T> core::ops::RemAssign<T> for Meters
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_meters(rhs);
                *self = self._rem(it);
            }
        }
        impl core::fmt::Debug for Meters {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f.debug_tuple("Meters").field(&self.0).finish();
            }
        }
        if !(size_of::<Meters>() == size_of::<f64>()) {
            core::panic!("invalid integral storage size");
        };
        if !(align_of::<Meters>() == align_of::<f64>()) {
            core::panic!("invalid integral storage alignment");
        };
        impl core::ops::Add for Meters {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._add(rhs.0);
            }
        }
        impl<T> core::ops::Add<T> for Meters
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_meters(rhs);
                return self._add(it);
            }
        }
        impl core::ops::Sub for Meters {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._sub(rhs.0);
            }
        }
        impl<T> core::ops::Sub<T> for Meters
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_meters(rhs);
                return self._sub(it);
            }
        }
        impl core::ops::Mul for Meters {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._mul(rhs.0);
            }
        }
        impl<T> core::ops::Mul<T> for Meters
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_meters(rhs);
                return self._mul(it);
            }
        }
        impl core::ops::Div for Meters {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._div(rhs.0);
            }
        }
        impl<T> core::ops::Div<T> for Meters
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_meters(rhs);
                return self._div(it);
            }
        }
        impl core::ops::Rem for Meters {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._rem(rhs.0);
            }
        }
        impl<T> core::ops::Rem<T> for Meters
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_meters(rhs);
                return self._rem(it);
            }
        }
        impl core::ops::Neg for Meters {
            type Output = Self;
            #[inline(always)]
            fn neg(self) -> Self::Output {
                return self._neg();
            }
        }
        impl<T> PartialEq<T> for Meters
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_meters(*rhs);
                return lhs == rhs;
            }
        }
        impl<T> PartialOrd<T> for Meters
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> Option<core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_meters(*rhs);
                return PartialOrd::partial_cmp(&lhs, &rhs);
            }
            #[inline(always)]
            fn lt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) < Seal::conv_meters(*rhs);
            }
            #[inline(always)]
            fn le(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) <= Seal::conv_meters(*rhs);
            }
            #[inline(always)]
            fn gt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) > Seal::conv_meters(*rhs);
            }
            #[inline(always)]
            fn ge(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) >= Seal::conv_meters(*rhs);
            }
        }
        impl core::fmt::LowerExp for Meters {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::LowerExp::fmt(&raw, f);
            }
        }
        impl core::fmt::UpperExp for Meters {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::UpperExp::fmt(&raw, f);
            }
        }
        impl core::iter::Sum for Meters {
            #[inline(always)]
            #[track_caller]
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                let raw = iter.map(|it| Self::raw(it)).sum::<f64>();
                return Self::_result_or_panic(raw);
            }
        }
        impl<'__typekin_item> core::iter::Sum<&'__typekin_item Meters> for Meters {
            #[inline(always)]
            #[track_caller]
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'__typekin_item Self>,
            {
                let raw = iter.map(|it| Self::raw(*it)).sum::<f64>();
                return Self::_result_or_panic(raw);
            }
        }
        impl core::iter::Product for Meters {
            #[inline(always)]
            #[track_caller]
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                let raw = iter.map(|it| Self::raw(it)).product::<f64>();
                return Self::_result_or_panic(raw);
            }
        }
        impl<'__typekin_item> core::iter::Product<&'__typekin_item Meters> for Meters {
            #[inline(always)]
            #[track_caller]
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'__typekin_item Self>,
            {
                let raw = iter.map(|it| Self::raw(*it)).product::<f64>();
                return Self::_result_or_panic(raw);
            }
        }
        impl Meters {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> Meters
            where
                T: Make,
            {
                return <T as Make>::make(it);
            }
            #[inline(always)]
            pub fn make(it: f64) -> Meters {
                return Meters::of(it);
            }
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> f64 {
                return self.0;
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
            ) -> Result<Self, f64>
            where
                F: FnOnce(f64) -> f64,
            {
                let value = function(Self::raw(self));
                return Ok(Self(value));
            }
            #[inline(always)]
            #[track_caller]
            pub fn map<F>(
                self,
                function: F,
            ) -> Self
            where
                F: FnOnce(f64) -> f64,
            {
                return match self.try_map(function) {
                    Ok(value) => value,
                    Err(_) => {
                        core::panic!(
                            "mapping operation produced an invalid value"
                        )
                    }
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_f64(self) -> Result<f64, ()> {
                return Ok(Self::raw(self) as f64);
            }
            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32"
            ))]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(self) -> Result<usize, ()> {
                let r = Self::raw(self);
                let t = r as usize;
                return if t as f64 == r { Ok(t) } else { Err(()) };
            }
            #[cfg(target_pointer_width = "64")]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(self) -> Result<usize, ()> {
                let r = Self::raw(self);
                let t = r as usize;
                return if t as f64 == r && r < 18446744073709551616.0 {
                    Ok(t)
                }
                else {
                    Err(())
                };
            }
            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32"
            ))]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) -> Result<isize, ()> {
                let r = Self::raw(self);
                let t = r as isize;
                return if t as f64 == r { Ok(t) } else { Err(()) };
            }
            #[cfg(target_pointer_width = "64")]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) -> Result<isize, ()> {
                let r = Self::raw(self);
                let t = r as isize;
                return if t as f64 == r && r < 9223372036854775808.0 {
                    Ok(t)
                }
                else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u8(self) -> Result<u8, ()> {
                let r = Self::raw(self);
                let t = r as u8;
                return if t as f64 == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u16(self) -> Result<u16, ()> {
                let r = Self::raw(self);
                let t = r as u16;
                return if t as f64 == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u32(self) -> Result<u32, ()> {
                let r = Self::raw(self);
                let t = r as u32;
                return if t as f64 == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u64(self) -> Result<u64, ()> {
                let r = Self::raw(self);
                let t = r as u64;
                return if t as f64 == r && r < 18446744073709551616.0 {
                    Ok(t)
                }
                else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u128(self) -> Result<u128, ()> {
                let r = Self::raw(self);
                let t = r as u128;
                return if t as f64 == r
                    && r < 340282366920938463463374607431768211456.0
                {
                    Ok(t)
                }
                else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i8(self) -> Result<i8, ()> {
                let r = Self::raw(self);
                let t = r as i8;
                return if t as f64 == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i16(self) -> Result<i16, ()> {
                let r = Self::raw(self);
                let t = r as i16;
                return if t as f64 == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i32(self) -> Result<i32, ()> {
                let r = Self::raw(self);
                let t = r as i32;
                return if t as f64 == r { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i64(self) -> Result<i64, ()> {
                let r = Self::raw(self);
                let t = r as i64;
                return if t as f64 == r && r < 9223372036854775808.0 {
                    Ok(t)
                }
                else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i128(self) -> Result<i128, ()> {
                let r = Self::raw(self);
                let t = r as i128;
                return if t as f64 == r
                    && r < 170141183460469231731687303715884105728.0
                {
                    Ok(t)
                }
                else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_f32(self) -> Result<f32, ()> {
                let r = Self::raw(self);
                let t = r as f32;
                return if t as f64 == r || r.is_nan() {
                    Ok(t)
                }
                else {
                    Err(())
                };
            }
            #[inline(always)]
            pub(self) const fn _add(
                &self,
                rhs: f64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs + rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _sub(
                &self,
                rhs: f64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs - rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _mul(
                &self,
                rhs: f64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs * rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _div(
                &self,
                rhs: f64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs / rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _rem(
                &self,
                rhs: f64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs % rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _neg(&self) -> Self {
                let lhs = Self::raw(*self);
                let it = -lhs;
                return Self::_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _eq(
                self,
                rhs: f64,
            ) -> bool {
                let lhs = Self::raw(self);
                return lhs == rhs;
            }
            #[inline(always)]
            pub const fn try_make(it: f64) -> Result<Self, f64> {
                return Ok(Self::_or_panic(it));
            }
            #[must_use]
            #[inline(always)]
            pub(self) const fn _or_panic(it: f64) -> Self {
                return Self(it);
            }
            #[inline(always)]
            #[track_caller]
            pub(self) fn _result_or_panic(value: f64) -> Self {
                {
                    return Self(value);
                }
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl Meters {}
    };
}
#[allow(unused_imports)]
pub use __meters::Meters;

impl Meters {
    /// `0.0` for either zero, unlike `f64::signum`.
    pub fn signum(self) -> Self {
        let raw = self.raw();
        return match raw == 0.0 {
            true => Self::make(0.0),
            false => Self::make(raw.signum()),
        };
    }
}

fn main() {
    let width = Meters::make(2.5);
    let height = Meters::make(4.0);

    let area = width * height;
    assert_eq!(area.raw(), 10.0);
    assert_eq!((-width).raw(), -2.5);

    // NaN-able: no `Eq`, `Ord`, or `Hash`.
    let nan = Meters::make(f64::NAN);
    assert_eq!(nan.partial_cmp(&width), None);

    // Checked conversions reject rounding, NaN, and out-of-range values.
    assert_eq!(height.try_into_u8(), Ok(4));
    assert_eq!(width.try_into_u8(), Err(()));
    assert_eq!(nan.try_into_i64(), Err(()));

    assert_eq!(Meters::make(0.0).signum().raw(), 0.0);
    assert_eq!(width.signum().raw(), 1.0);

    let total: Meters = [
        width,
        height,
    ]
    .iter()
    .sum();
    println!("{total:?} {area:e}");
}
