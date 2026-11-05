use std::collections::BTreeSet;

#[doc(hidden)]
mod __unit {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    #[doc = " Every range rejects NaN, so `Unit` is totally ordered: `Eq`, `Ord`, and"]
    #[doc = " `Hash` are generated. Queries the domain decides, such as `is_nan` and"]
    #[doc = " `is_finite`, return constants."]
    pub struct Unit(f64);

    #[allow(unused_imports)]
    use super::*;
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __typekin_unit_ac7349ebbed4d348 {
        (const $value: expr) => {
            const {
                match Unit::try_make($value) {
                    ::core::result::Result::Ok(value) => {
                        value.__typekin_lit_ac7349ebbed4d348();
                        value
                    }
                    ::core::result::Result::Err(_) => ::core::panic!(
                        "{}",
                        ::core::concat!(
                            ::core::stringify!($value),
                            " is outside the domain of Unit"
                        ),
                    ),
                }
            }
        };
        ($value: expr) => {{
            const VALUE: Unit = match Unit::try_make($value) {
                ::core::result::Result::Ok(value) => {
                    value.__typekin_lit_ac7349ebbed4d348();
                    value
                }
                ::core::result::Result::Err(_) => ::core::panic!(
                    "{}",
                    ::core::concat!(
                        ::core::stringify!($value),
                        " is outside the domain of Unit"
                    ),
                ),
            };
            VALUE
        }};
    }
    #[doc = " Builds a [`Unit`] from a constant checked at compile time: an invalid value is a compile error."]
    #[doc = ""]
    #[doc = " `unit!(value)` uses a `const` item, checked by `cargo check`; `unit!(const value)` uses an inline `const` block."]
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use __typekin_unit_ac7349ebbed4d348 as unit;
    impl Unit {
        #[doc(hidden)]
        #[inline(always)]
        pub const fn __typekin_lit_ac7349ebbed4d348(&self) {}
    }
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::manual_range_contains)]
    const _: () = {
        trait Seal {
            fn conv_unit(self) -> f64;
        }
        trait Make: Seal {
            fn make(self) -> Unit;
        }
        trait Math: Seal {}
        trait Rel: Seal {}
        impl Seal for Unit {
            #[inline(always)]
            fn conv_unit(self) -> f64 {
                return Self::raw(self);
            }
        }
        impl Make for Unit {
            #[inline(always)]
            fn make(self) -> Unit {
                let raw = <Self as Seal>::conv_unit(self);
                return Unit::_or_panic(raw);
            }
        }
        impl Rel for Unit {}
        impl Seal for f64 {
            #[inline(always)]
            fn conv_unit(self) -> f64 {
                return self as f64;
            }
        }
        impl Math for f64 {}
        impl Rel for f64 {}
        impl AsRef<f64> for Unit {
            #[inline(always)]
            fn as_ref(&self) -> &f64 {
                return &self.0;
            }
        }
        impl core::str::FromStr for Unit {
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
                return if value >= 0.0 && value <= 1.0 {
                    Ok(Self(value))
                }
                else {
                    Err(())
                };
            }
        }
        impl Unit {
            pub const RADIX: u32 = f64::RADIX;
            pub const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;
            pub const DIGITS: u32 = f64::DIGITS;
            pub const MIN_EXP: i32 = f64::MIN_EXP;
            pub const MAX_EXP: i32 = f64::MAX_EXP;
            pub const MIN_10_EXP: i32 = f64::MIN_10_EXP;
            pub const MAX_10_EXP: i32 = f64::MAX_10_EXP;
        }
        impl Unit {
            #[doc = " Always `false`: every value in the domain `0.0..=1.0` answers alike (decided by typekin at expansion)."]
            #[must_use]
            #[inline(always)]
            pub const fn is_nan(self) -> bool {
                return false;
            }
            #[doc = " Always `false`: every value in the domain `0.0..=1.0` answers alike (decided by typekin at expansion)."]
            #[must_use]
            #[inline(always)]
            pub const fn is_infinite(self) -> bool {
                return false;
            }
            #[doc = " Always `true`: every value in the domain `0.0..=1.0` answers alike (decided by typekin at expansion)."]
            #[must_use]
            #[inline(always)]
            pub const fn is_finite(self) -> bool {
                return true;
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
            pub const fn from_bits(bits: u64) -> Self {
                return Self::_result_or_panic(f64::from_bits(bits));
            }
            #[must_use]
            #[inline(always)]
            pub const fn to_ne_bytes(self) -> [u8; size_of::<f64>()] {
                return Self::raw(self).to_ne_bytes();
            }
            #[inline(always)]
            #[track_caller]
            pub const fn from_ne_bytes(bytes: [u8; size_of::<f64>()]) -> Self {
                return Self::_result_or_panic(f64::from_ne_bytes(bytes));
            }
            #[must_use]
            #[inline(always)]
            pub const fn to_be_bytes(self) -> [u8; size_of::<f64>()] {
                return Self::raw(self).to_be_bytes();
            }
            #[inline(always)]
            #[track_caller]
            pub const fn from_be_bytes(bytes: [u8; size_of::<f64>()]) -> Self {
                return Self::_result_or_panic(f64::from_be_bytes(bytes));
            }
            #[must_use]
            #[inline(always)]
            pub const fn to_le_bytes(self) -> [u8; size_of::<f64>()] {
                return Self::raw(self).to_le_bytes();
            }
            #[inline(always)]
            #[track_caller]
            pub const fn from_le_bytes(bytes: [u8; size_of::<f64>()]) -> Self {
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
            pub const fn abs(self) -> Self {
                let it = Self::raw(self).abs();
                return Self(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub const fn signum(self) -> Self {
                let it = Self::raw(self).signum();
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub const fn copysign(
                self,
                sign: Self,
            ) -> Self {
                let it = Self::raw(self).copysign(Self::raw(sign));
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub const fn recip(self) -> Self {
                let it = Self::raw(self).recip();
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub const fn to_degrees(self) -> Self {
                let it = Self::raw(self).to_degrees();
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub const fn to_radians(self) -> Self {
                let it = Self::raw(self).to_radians();
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub const fn next_up(self) -> Self {
                let it = Self::raw(self).next_up();
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub const fn next_down(self) -> Self {
                let it = Self::raw(self).next_down();
                return Self::_result_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[track_caller]
            pub const fn midpoint(
                self,
                that: Self,
            ) -> Self {
                let it = Self::raw(self).midpoint(Self::raw(that));
                return Self::_result_or_panic(it);
            }
        }
        impl Into<f64> for Unit {
            #[inline(always)]
            fn into(self) -> f64 {
                return Self::into_f64(self);
            }
        }
        impl TryInto<usize> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<usize, Self::Error> {
                return Self::try_into_usize(self);
            }
        }
        impl TryInto<isize> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<isize, Self::Error> {
                return Self::try_into_isize(self);
            }
        }
        impl TryInto<u8> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u8, Self::Error> {
                return Self::try_into_u8(self);
            }
        }
        impl TryInto<u16> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u16, Self::Error> {
                return Self::try_into_u16(self);
            }
        }
        impl TryInto<u32> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u32, Self::Error> {
                return Self::try_into_u32(self);
            }
        }
        impl TryInto<u64> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u64, Self::Error> {
                return Self::try_into_u64(self);
            }
        }
        impl TryInto<u128> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u128, Self::Error> {
                return Self::try_into_u128(self);
            }
        }
        impl TryInto<i8> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i8, Self::Error> {
                return Self::try_into_i8(self);
            }
        }
        impl TryInto<i16> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i16, Self::Error> {
                return Self::try_into_i16(self);
            }
        }
        impl TryInto<i32> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i32, Self::Error> {
                return Self::try_into_i32(self);
            }
        }
        impl TryInto<i64> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i64, Self::Error> {
                return Self::try_into_i64(self);
            }
        }
        impl TryInto<i128> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i128, Self::Error> {
                return Self::try_into_i128(self);
            }
        }
        impl TryInto<f32> for Unit {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<f32, Self::Error> {
                return Self::try_into_f32(self);
            }
        }
        impl core::ops::AddAssign for Unit {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._add(rhs.0);
            }
        }
        impl<T> core::ops::AddAssign<T> for Unit
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_unit(rhs);
                *self = self._add(it);
            }
        }
        impl core::ops::SubAssign for Unit {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._sub(rhs.0);
            }
        }
        impl<T> core::ops::SubAssign<T> for Unit
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_unit(rhs);
                *self = self._sub(it);
            }
        }
        impl core::ops::MulAssign for Unit {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._mul(rhs.0);
            }
        }
        impl<T> core::ops::MulAssign<T> for Unit
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_unit(rhs);
                *self = self._mul(it);
            }
        }
        impl core::ops::DivAssign for Unit {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._div(rhs.0);
            }
        }
        impl<T> core::ops::DivAssign<T> for Unit
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_unit(rhs);
                *self = self._div(it);
            }
        }
        impl core::ops::RemAssign for Unit {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._rem(rhs.0);
            }
        }
        impl<T> core::ops::RemAssign<T> for Unit
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_unit(rhs);
                *self = self._rem(it);
            }
        }
        impl core::fmt::Debug for Unit {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f.debug_tuple("Unit").field(&self.0).finish();
            }
        }
        impl core::hash::Hash for Unit {
            #[inline(always)]
            fn hash<H>(
                &self,
                state: &mut H,
            ) where
                H: core::hash::Hasher,
            {
                return core::hash::Hash::hash(
                    &(self.0 + 0.0).to_bits(),
                    state,
                );
            }
        }
        impl Eq for Unit {}
        impl Ord for Unit {
            #[inline(always)]
            fn cmp(
                &self,
                that: &Self,
            ) -> core::cmp::Ordering {
                let lhs = Self::raw(*self);
                let rhs = Self::raw(*that);
                return if lhs < rhs {
                    core::cmp::Ordering::Less
                }
                else if lhs > rhs {
                    core::cmp::Ordering::Greater
                }
                else {
                    core::cmp::Ordering::Equal
                };
            }
        }
        if !(size_of::<Unit>() == size_of::<f64>()) {
            core::panic!("invalid integral storage size");
        };
        if !(align_of::<Unit>() == align_of::<f64>()) {
            core::panic!("invalid integral storage alignment");
        };
        impl core::ops::Add for Unit {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._add(rhs.0);
            }
        }
        impl<T> core::ops::Add<T> for Unit
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_unit(rhs);
                return self._add(it);
            }
        }
        impl core::ops::Sub for Unit {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._sub(rhs.0);
            }
        }
        impl<T> core::ops::Sub<T> for Unit
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_unit(rhs);
                return self._sub(it);
            }
        }
        impl core::ops::Mul for Unit {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._mul(rhs.0);
            }
        }
        impl<T> core::ops::Mul<T> for Unit
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_unit(rhs);
                return self._mul(it);
            }
        }
        impl core::ops::Div for Unit {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._div(rhs.0);
            }
        }
        impl<T> core::ops::Div<T> for Unit
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_unit(rhs);
                return self._div(it);
            }
        }
        impl core::ops::Rem for Unit {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._rem(rhs.0);
            }
        }
        impl<T> core::ops::Rem<T> for Unit
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_unit(rhs);
                return self._rem(it);
            }
        }
        impl core::ops::Neg for Unit {
            type Output = Self;
            #[inline(always)]
            fn neg(self) -> Self::Output {
                return self._neg();
            }
        }
        impl core::fmt::Display for Unit {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Display::fmt(&raw, f);
            }
        }
        impl<T> PartialEq<T> for Unit
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_unit(*rhs);
                return lhs == rhs;
            }
        }
        impl<T> PartialOrd<T> for Unit
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> Option<core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_unit(*rhs);
                return PartialOrd::partial_cmp(&lhs, &rhs);
            }
            #[inline(always)]
            fn lt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) < Seal::conv_unit(*rhs);
            }
            #[inline(always)]
            fn le(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) <= Seal::conv_unit(*rhs);
            }
            #[inline(always)]
            fn gt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) > Seal::conv_unit(*rhs);
            }
            #[inline(always)]
            fn ge(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) >= Seal::conv_unit(*rhs);
            }
        }
        impl core::fmt::LowerExp for Unit {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::LowerExp::fmt(&raw, f);
            }
        }
        impl core::fmt::UpperExp for Unit {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::UpperExp::fmt(&raw, f);
            }
        }
        impl core::iter::Sum for Unit {
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
        impl<'__typekin_item> core::iter::Sum<&'__typekin_item Unit> for Unit {
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
        impl core::iter::Product for Unit {
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
        impl<'__typekin_item> core::iter::Product<&'__typekin_item Unit> for Unit {
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
        impl Unit {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> Unit
            where
                T: Make,
            {
                return <T as Make>::make(it);
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
                return if value >= 0.0 && value <= 1.0 {
                    Ok(Self(value))
                }
                else {
                    Err(value)
                };
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
            pub(self) fn _add(
                &self,
                rhs: f64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs + rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _sub(
                &self,
                rhs: f64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs - rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _mul(
                &self,
                rhs: f64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs * rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _div(
                &self,
                rhs: f64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs / rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _rem(
                &self,
                rhs: f64,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs % rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _neg(&self) -> Self {
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
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _cmp(
                self,
                rhs: f64,
            ) -> core::cmp::Ordering {
                let lhs = Self::raw(self);
                return if lhs < rhs {
                    core::cmp::Ordering::Less
                }
                else if lhs > rhs {
                    core::cmp::Ordering::Greater
                }
                else {
                    core::cmp::Ordering::Equal
                };
            }
            #[inline(always)]
            pub const fn try_make(it: f64) -> Result<Self, f64> {
                return if it >= 0.0 && it <= 1.0 {
                    return Ok(Self(it));
                }
                else {
                    Err(it)
                };
            }
            #[must_use]
            #[inline(always)]
            pub(self) fn _or_panic(it: f64) -> Self {
                if it >= 0.0 && it <= 1.0 {
                    return Self(it);
                }
                else {
                    core::panic!("invalid value");
                };
            }
            #[inline(always)]
            #[track_caller]
            pub(self) const fn _result_or_panic(value: f64) -> Self {
                if value >= 0.0 && value <= 1.0 {
                    return Self(value);
                }
                else {
                    core::panic!("operation produced an invalid value");
                }
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl Unit {}
    };
}
#[allow(unused_imports)]
pub use __unit::Unit;
#[allow(unused_imports)]
pub use __unit::unit;

fn main() {
    assert!(Unit::try_make(f64::NAN).is_err());
    assert!(Unit::try_make(1.5).is_err());

    let half = Unit::try_make(0.5).unwrap();
    let quarter = Unit::try_make(0.25).unwrap();
    assert_eq!(half.max(quarter), half);
    assert!(half.is_finite());

    let set = [
        half,
        quarter,
        half,
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(set.len(), 2);

    // `-0.0 == 0.0`, and both hash alike.
    assert_eq!(Unit::try_make(-0.0).unwrap(), Unit::try_make(0.0).unwrap());

    println!("{half:.2} {quarter}");
}
