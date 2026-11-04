//! Closure proofs: operations that cannot leave the domain skip their check.
//! They are on by default for `u8`/`i8`; wider integers opt in with
//! `with = [proven]`, and `prove` turns a proof into a compile-time contract.

#[doc(hidden)]
mod __percent {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct Percent(u8);

    #[allow(unused_imports)]
    use super::*;
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __typekin_percent_6fb2e49ce6302edf {
        (const $value: expr) => {
            const {
                match Percent::try_make($value) {
                    ::core::result::Result::Ok(value) => {
                        value.__typekin_lit_6fb2e49ce6302edf();
                        value
                    }
                    ::core::result::Result::Err(_) => ::core::panic!(
                        "{}",
                        ::core::concat!(
                            ::core::stringify!($value),
                            " is outside the domain of Percent"
                        ),
                    ),
                }
            }
        };
        ($value: expr) => {{
            const VALUE: Percent = match Percent::try_make($value) {
                ::core::result::Result::Ok(value) => {
                    value.__typekin_lit_6fb2e49ce6302edf();
                    value
                }
                ::core::result::Result::Err(_) => ::core::panic!(
                    "{}",
                    ::core::concat!(
                        ::core::stringify!($value),
                        " is outside the domain of Percent"
                    ),
                ),
            };
            VALUE
        }};
    }
    #[doc = " Builds a [`Percent`] from a constant checked at compile time: an invalid value is a compile error."]
    #[doc = ""]
    #[doc = " `percent!(value)` uses a `const` item, checked by `cargo check`; `percent!(const value)` uses an inline `const` block."]
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use __typekin_percent_6fb2e49ce6302edf as percent;
    impl Percent {
        #[doc(hidden)]
        #[inline(always)]
        pub const fn __typekin_lit_6fb2e49ce6302edf(&self) {}
    }
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    const _: () = {
        trait Seal {
            fn conv_percent(self) -> u8;
        }
        trait Make: Seal {
            fn make(self) -> Percent;
        }
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Rel: Seal {}
        #[doc = "# Safety"]
        #[doc = ""]
        #[doc = "Implementing this trait asserts that values \
                        produced by the sealed conversion satisfy every \
                        invariant required by the target type."]
        unsafe trait Unsafe: Seal {}
        impl Seal for Percent {
            #[inline(always)]
            fn conv_percent(self) -> u8 {
                return Self::raw(self);
            }
        }
        impl Make for Percent {
            #[inline(always)]
            fn make(self) -> Percent {
                let raw = <Self as Seal>::conv_percent(self);
                return Percent::_or_panic(raw);
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
        impl AsRef<u8> for Percent {
            #[inline(always)]
            fn as_ref(&self) -> &u8 {
                return &self.0;
            }
        }
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
                return if value <= 100 {
                    Ok(Self(value))
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
            pub const fn rotate_left(
                self,
                n: u32,
            ) -> Self {
                return Self::_result_or_panic(Self::raw(self).rotate_left(n));
            }
            #[inline(always)]
            pub const fn rotate_right(
                self,
                n: u32,
            ) -> Self {
                return Self::_result_or_panic(Self::raw(self).rotate_right(n));
            }
            #[inline(always)]
            pub const fn swap_bytes(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).swap_bytes());
            }
            #[inline(always)]
            pub const fn reverse_bits(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).reverse_bits());
            }
            #[inline(always)]
            pub const fn isolate_highest_one(self) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).isolate_highest_one(),
                );
            }
            #[inline(always)]
            pub const fn isolate_lowest_one(self) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).isolate_lowest_one(),
                );
            }
            #[inline(always)]
            pub const fn to_be(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).to_be());
            }
            #[inline(always)]
            pub const fn to_le(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).to_le());
            }
            #[inline(always)]
            pub const fn from_be(value: Self) -> Self {
                return Self::_result_or_panic(u8::from_be(Self::raw(value)));
            }
            #[inline(always)]
            pub const fn from_le(value: Self) -> Self {
                return Self::_result_or_panic(u8::from_le(Self::raw(value)));
            }
            #[inline(always)]
            pub const fn from_ne_bytes(bytes: [u8; size_of::<u8>()]) -> Self {
                return Self::_result_or_panic(u8::from_ne_bytes(bytes));
            }
            #[inline(always)]
            pub const fn from_be_bytes(bytes: [u8; size_of::<u8>()]) -> Self {
                return Self::_result_or_panic(u8::from_be_bytes(bytes));
            }
            #[inline(always)]
            pub const fn from_le_bytes(bytes: [u8; size_of::<u8>()]) -> Self {
                return Self::_result_or_panic(u8::from_le_bytes(bytes));
            }
            #[inline(always)]
            pub const fn checked_add(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_add(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn checked_sub(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_sub(Self::raw(rhs)) {
                    Some(value) => Self::_make_proven(value),
                    None => None,
                };
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn checked_neg(self) -> Option<Self> {
                return match Self::raw(self).checked_neg() {
                    Some(value) => Self::_make_proven(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn checked_mul(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_mul(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn checked_div(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_div(Self::raw(rhs)) {
                    Some(value) => Self::_make_proven(value),
                    None => None,
                };
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn checked_rem(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_rem(Self::raw(rhs)) {
                    Some(value) => Self::_make_proven(value),
                    None => None,
                };
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn checked_div_euclid(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_div_euclid(Self::raw(rhs))
                {
                    Some(value) => Self::_make_proven(value),
                    None => None,
                };
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn checked_rem_euclid(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_rem_euclid(Self::raw(rhs))
                {
                    Some(value) => Self::_make_proven(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn checked_shl(
                self,
                rhs: u32,
            ) -> Option<Self> {
                return match Self::raw(self).checked_shl(rhs) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn checked_shr(
                self,
                rhs: u32,
            ) -> Option<Self> {
                return match Self::raw(self).checked_shr(rhs) {
                    Some(value) => Self::_make_proven(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn checked_pow(
                self,
                exp: u32,
            ) -> Option<Self> {
                return match Self::raw(self).checked_pow(exp) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn midpoint(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(Self::raw(self).midpoint(Self::raw(rhs)));
            }
            #[inline(always)]
            pub const fn unbounded_shl(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).unbounded_shl(rhs),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn unbounded_shr(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_proven(Self::raw(self).unbounded_shr(rhs));
            }
            #[inline(always)]
            pub const fn saturating_add(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).saturating_add(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn saturating_sub(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).saturating_sub(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn saturating_mul(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).saturating_mul(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn saturating_div(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).saturating_div(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn saturating_pow(
                self,
                exp: u32,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).saturating_pow(exp),
                );
            }
            #[inline(always)]
            pub const fn wrapping_add(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_add(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_sub(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_sub(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_neg(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).wrapping_neg());
            }
            #[inline(always)]
            pub const fn wrapping_mul(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_mul(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn wrapping_div(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).wrapping_div(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn wrapping_rem(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).wrapping_rem(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn wrapping_div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).wrapping_div_euclid(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn wrapping_rem_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).wrapping_rem_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_shl(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_shl(rhs),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn wrapping_shr(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_proven(Self::raw(self).wrapping_shr(rhs));
            }
            #[inline(always)]
            pub const fn wrapping_pow(
                self,
                exp: u32,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_pow(exp),
                );
            }
            #[inline(always)]
            pub const fn pow(
                self,
                exp: u32,
            ) -> Self {
                return Self::_result_or_panic(Self::raw(self).pow(exp));
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn isqrt(self) -> Self {
                return Self::_proven(Self::raw(self).isqrt());
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).div_euclid(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn rem_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).rem_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_add(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).strict_add(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_sub(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).strict_sub(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_neg(self) -> Self {
                return Self::_proven(Self::raw(self).strict_neg());
            }
            #[inline(always)]
            pub const fn strict_mul(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).strict_mul(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_div(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).strict_div(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_rem(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).strict_rem(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).strict_div_euclid(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_rem_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).strict_rem_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_shl(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_result_or_panic(Self::raw(self).strict_shl(rhs));
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_shr(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_proven(Self::raw(self).strict_shr(rhs));
            }
            #[inline(always)]
            pub const fn strict_pow(
                self,
                exp: u32,
            ) -> Self {
                return Self::_result_or_panic(Self::raw(self).strict_pow(exp));
            }
            #[inline(always)]
            pub const fn overflowing_add(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_add(Self::raw(rhs));
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_sub(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_sub(Self::raw(rhs));
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_neg(self) -> (Self, bool) {
                let (value, overflowed) = Self::raw(self).overflowing_neg();
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_mul(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_mul(Self::raw(rhs));
                return (Self::_result_or_panic(value), overflowed);
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn overflowing_div(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_div(Self::raw(rhs));
                return (Self::_proven(value), overflowed);
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn overflowing_rem(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_rem(Self::raw(rhs));
                return (Self::_proven(value), overflowed);
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn overflowing_div_euclid(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_div_euclid(Self::raw(rhs));
                return (Self::_proven(value), overflowed);
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn overflowing_rem_euclid(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_rem_euclid(Self::raw(rhs));
                return (Self::_proven(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_shl(
                self,
                rhs: u32,
            ) -> (Self, bool) {
                let (value, overflowed) = Self::raw(self).overflowing_shl(rhs);
                return (Self::_result_or_panic(value), overflowed);
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn overflowing_shr(
                self,
                rhs: u32,
            ) -> (Self, bool) {
                let (value, overflowed) = Self::raw(self).overflowing_shr(rhs);
                return (Self::_proven(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_pow(
                self,
                exp: u32,
            ) -> (Self, bool) {
                let (value, overflowed) = Self::raw(self).overflowing_pow(exp);
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn cast_signed(self) -> i8 {
                return Self::raw(self).cast_signed();
            }
            #[inline(always)]
            pub const fn bit_width(self) -> u32 {
                return Self::raw(self).bit_width();
            }
            #[inline(always)]
            pub const fn abs_diff(
                self,
                rhs: Self,
            ) -> u8 {
                return Self::raw(self).abs_diff(Self::raw(rhs));
            }
            #[inline(always)]
            pub const fn is_multiple_of(
                self,
                rhs: Self,
            ) -> bool {
                return Self::raw(self).is_multiple_of(Self::raw(rhs));
            }
            #[inline(always)]
            pub const fn is_power_of_two(self) -> bool {
                return Self::raw(self).is_power_of_two();
            }
            #[inline(always)]
            pub const fn next_power_of_two(self) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).next_power_of_two(),
                );
            }
            #[inline(always)]
            pub const fn checked_next_power_of_two(self) -> Option<Self> {
                return match Self::raw(self).checked_next_power_of_two() {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn div_ceil(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).div_ceil(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn next_multiple_of(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).next_multiple_of(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn checked_next_multiple_of(
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
            #[inline(always)]
            pub fn carrying_add(
                self,
                rhs: Self,
                carry: bool,
            ) -> (Self, bool) {
                let (value, carry) =
                    Self::raw(self).carrying_add(Self::raw(rhs), carry);
                return (Self::_result_or_panic(value), carry);
            }
            #[inline(always)]
            pub fn borrowing_sub(
                self,
                rhs: Self,
                borrow: bool,
            ) -> (Self, bool) {
                let (value, borrow) =
                    Self::raw(self).borrowing_sub(Self::raw(rhs), borrow);
                return (Self::_result_or_panic(value), borrow);
            }
            #[inline(always)]
            pub fn carrying_mul(
                self,
                rhs: Self,
                carry: Self,
            ) -> (Self, Self) {
                let (low, high) = Self::raw(self)
                    .carrying_mul(Self::raw(rhs), Self::raw(carry));
                return (
                    Self::_result_or_panic(low),
                    Self::_result_or_panic(high),
                );
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
                return (
                    Self::_result_or_panic(low),
                    Self::_result_or_panic(high),
                );
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
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        impl core::ops::Shl<usize> for Percent {
            type Output = Self;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        impl core::ops::ShrAssign<usize> for Percent {
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn shr_assign(
                &mut self,
                that: usize,
            ) {
                *self = self._shr(that);
            }
        }
        impl core::ops::ShlAssign<usize> for Percent {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                that: usize,
            ) {
                *self = self._shl(that);
            }
        }
        impl core::ops::BitAndAssign for Percent {
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitand(rhs.0);
            }
        }
        impl<T> core::ops::BitAndAssign<T> for Percent
        where
            T: Bit + Seal,
        {
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_percent(rhs);
                *self = self._bitand(it);
            }
        }
        impl core::ops::AddAssign for Percent {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._add(rhs.0);
            }
        }
        impl<T> core::ops::AddAssign<T> for Percent
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_percent(rhs);
                *self = self._add(it);
            }
        }
        impl core::ops::SubAssign for Percent {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._sub(rhs.0);
            }
        }
        impl<T> core::ops::SubAssign<T> for Percent
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_percent(rhs);
                *self = self._sub(it);
            }
        }
        impl core::ops::MulAssign for Percent {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._mul(rhs.0);
            }
        }
        impl<T> core::ops::MulAssign<T> for Percent
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_percent(rhs);
                *self = self._mul(it);
            }
        }
        impl core::ops::DivAssign for Percent {
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._div(rhs.0);
            }
        }
        impl<T> core::ops::DivAssign<T> for Percent
        where
            T: Math + Seal,
        {
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_percent(rhs);
                *self = self._div(it);
            }
        }
        impl core::ops::RemAssign for Percent {
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._rem(rhs.0);
            }
        }
        impl<T> core::ops::RemAssign<T> for Percent
        where
            T: Math + Seal,
        {
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_percent(rhs);
                *self = self._rem(it);
            }
        }
        impl core::ops::BitOrAssign for Percent {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitor(rhs.0);
            }
        }
        impl<T> core::ops::BitOrAssign<T> for Percent
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_percent(rhs);
                *self = self._bitor(it);
            }
        }
        impl core::fmt::Debug for Percent {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f.debug_tuple("Percent").field(&self.0).finish();
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
                return core::hash::Hash::hash(&self.0, state);
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
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._add(rhs.0);
            }
        }
        impl<T> core::ops::Add<T> for Percent
        where
            T: Math + Seal,
        {
            type Output = Self;
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
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._sub(rhs.0);
            }
        }
        impl<T> core::ops::Sub<T> for Percent
        where
            T: Math + Seal,
        {
            type Output = Self;
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
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._mul(rhs.0);
            }
        }
        impl<T> core::ops::Mul<T> for Percent
        where
            T: Math + Seal,
        {
            type Output = Self;
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
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn div(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._div(rhs.0);
            }
        }
        impl<T> core::ops::Div<T> for Percent
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
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
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn rem(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._rem(rhs.0);
            }
        }
        impl<T> core::ops::Rem<T> for Percent
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
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
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitand(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitand(rhs.0);
            }
        }
        impl<T> core::ops::BitAnd<T> for Percent
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..=100` at every pointer width (proven by typekin at expansion)."]
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
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitor(rhs.0);
            }
        }
        impl<T> core::ops::BitOr<T> for Percent
        where
            T: Bit + Seal,
        {
            type Output = Self;
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
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitxor(rhs.0);
            }
        }
        impl core::ops::BitXorAssign for Percent {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitxor(rhs.0);
            }
        }
        impl<T> core::ops::BitXor<T> for Percent
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_percent(rhs);
                return self._bitxor(it);
            }
        }
        impl<T> core::ops::BitXorAssign<T> for Percent
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_percent(rhs);
                *self = self._bitxor(it);
            }
        }
        impl core::ops::Not for Percent {
            type Output = Self;
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
            pub fn of<T>(it: T) -> Percent
            where
                T: Make,
            {
                return <T as Make>::make(it);
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
                return if value <= 100 {
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
                F: FnOnce(u8) -> u8,
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
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs + rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _sub(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs - rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _mul(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs * rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _div(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs / rhs;
                return Self::_proven(it);
            }
            #[inline(always)]
            pub(self) fn _rem(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs % rhs;
                return Self::_proven(it);
            }
            #[inline(always)]
            pub(self) fn _bitxor(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs ^ rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _bitand(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs & rhs;
                return Self::_proven(it);
            }
            #[inline(always)]
            pub(self) fn _bitor(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs | rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _shr(
                &self,
                rhs: usize,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs >> rhs;
                return Self::_proven(it);
            }
            #[inline(always)]
            pub(self) fn _shl(
                &self,
                rhs: usize,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs << rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _not(&self) -> Self {
                let lhs = Self::raw(*self);
                let it = !lhs;
                return Self::_or_panic(it);
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
                return if it <= 100 {
                    return Ok(Self(it));
                }
                else {
                    Err(it)
                };
            }
            #[must_use]
            #[inline(always)]
            pub(self) fn _or_panic(it: u8) -> Self {
                if it <= 100 {
                    return Self(it);
                }
                else {
                    core::panic!("invalid value");
                };
            }
            #[doc = r" Builds a result that an expansion-time closure proof"]
            #[doc = r" shows valid; debug builds still check it."]
            #[must_use]
            #[inline(always)]
            pub(self) const fn _proven(it: u8) -> Self {
                debug_assert!((it <= (100)), "typekin closure proof violated",);
                return Self(it);
            }
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_RAW: (bool, u32) = (false, u8::BITS);
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_FORM: Option<(
                u128,
                &'static [(u128, bool, u128, bool, u128)],
            )> = Some((1u128, &[(0u128, false, 0u128, false, 100u128)]));
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_WHY: &'static str = "";
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_TABLE: Option<[bool; 256]> = {
                let mut table = [false; 256];
                let mut index = 0;
                while index < 256 {
                    let value: u8 = index as u8;
                    table[index] = value <= 100;
                    index += 1;
                }
                Some(table)
            };
            #[doc(hidden)]
            #[inline(always)]
            pub const fn __typekin_within_v1_raw(self) -> u8 {
                return self.0;
            }
            #[inline(always)]
            #[track_caller]
            pub(self) const fn _result_or_panic(value: u8) -> Self {
                if value <= 100 {
                    return Self(value);
                }
                else {
                    core::panic!("operation produced an invalid value");
                }
            }
            #[inline(always)]
            pub(self) const fn _make(value: u8) -> Option<Self> {
                return if value <= 100 {
                    Some(Self(value))
                }
                else {
                    None
                };
            }
            #[inline(always)]
            pub(self) const fn _make_proven(value: u8) -> Option<Self> {
                return Some(Self::_proven(value));
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
pub use __percent::Percent;
#[allow(unused_imports)]
pub use __percent::percent;

#[doc(hidden)]
mod __port {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct Port(core::num::NonZeroU16);

    #[allow(unused_imports)]
    use super::*;
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __typekin_port_3b9cf883dc2108c3 {
        (const $value: expr) => {
            const {
                match Port::try_make($value) {
                    ::core::result::Result::Ok(value) => {
                        value.__typekin_lit_3b9cf883dc2108c3();
                        value
                    }
                    ::core::result::Result::Err(_) => ::core::panic!(
                        "{}",
                        ::core::concat!(
                            ::core::stringify!($value),
                            " is outside the domain of Port"
                        ),
                    ),
                }
            }
        };
        ($value: expr) => {{
            const VALUE: Port = match Port::try_make($value) {
                ::core::result::Result::Ok(value) => {
                    value.__typekin_lit_3b9cf883dc2108c3();
                    value
                }
                ::core::result::Result::Err(_) => ::core::panic!(
                    "{}",
                    ::core::concat!(
                        ::core::stringify!($value),
                        " is outside the domain of Port"
                    ),
                ),
            };
            VALUE
        }};
    }
    #[doc = " Builds a [`Port`] from a constant checked at compile time: an invalid value is a compile error."]
    #[doc = ""]
    #[doc = " `port!(value)` uses a `const` item, checked by `cargo check`; `port!(const value)` uses an inline `const` block."]
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use __typekin_port_3b9cf883dc2108c3 as port;
    impl Port {
        #[doc(hidden)]
        #[inline(always)]
        pub const fn __typekin_lit_3b9cf883dc2108c3(&self) {}
    }
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    const _: () = {
        trait Seal {
            fn conv_port(self) -> u16;
        }
        trait Make: Seal {
            fn make(self) -> Port;
        }
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Rel: Seal {}
        #[doc = "# Safety"]
        #[doc = ""]
        #[doc = "Implementing this trait asserts that values \
                        produced by the sealed conversion satisfy every \
                        invariant required by the target type."]
        unsafe trait Unsafe: Seal {}
        impl Seal for Port {
            #[inline(always)]
            fn conv_port(self) -> u16 {
                return Self::raw(self);
            }
        }
        impl Make for Port {
            #[inline(always)]
            fn make(self) -> Port {
                let raw = <Self as Seal>::conv_port(self);
                return Port::_or_panic(raw);
            }
        }
        impl Rel for Port {}
        impl Seal for u16 {
            #[inline(always)]
            fn conv_port(self) -> u16 {
                return self as u16;
            }
        }
        impl Bit for u16 {}
        impl Math for u16 {}
        impl Rel for u16 {}
        impl AsRef<u16> for Port {
            #[inline(always)]
            fn as_ref(&self) -> &u16 {
                return unsafe { &*core::ptr::addr_of!(self.0).cast::<u16>() };
            }
        }
        impl core::str::FromStr for Port {
            type Err = ();
            #[inline(always)]
            fn from_str(source: &str) -> Result<Self, Self::Err> {
                let value = match <u16 as core::str::FromStr>::from_str(source)
                {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(());
                    }
                };
                return if value >= 1 && value <= 1023 || value >= 49_152 {
                    Ok(Self(unsafe {
                        core::num::NonZeroU16::new_unchecked(value)
                    }))
                }
                else {
                    Err(())
                };
            }
        }
        impl Port {
            #[doc = r" The number of bits in the wrapped primitive integer."]
            pub const BITS: u32 = u16::BITS;
        }
        impl Port {
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
            pub const fn to_be_bytes(self) -> [u8; size_of::<u16>()] {
                return Self::raw(self).to_be_bytes();
            }
            #[inline(always)]
            pub const fn to_le_bytes(self) -> [u8; size_of::<u16>()] {
                return Self::raw(self).to_le_bytes();
            }
            #[inline(always)]
            pub const fn to_ne_bytes(self) -> [u8; size_of::<u16>()] {
                return Self::raw(self).to_ne_bytes();
            }
            #[inline(always)]
            pub const fn rotate_left(
                self,
                n: u32,
            ) -> Self {
                return Self::_result_or_panic(Self::raw(self).rotate_left(n));
            }
            #[inline(always)]
            pub const fn rotate_right(
                self,
                n: u32,
            ) -> Self {
                return Self::_result_or_panic(Self::raw(self).rotate_right(n));
            }
            #[inline(always)]
            pub const fn swap_bytes(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).swap_bytes());
            }
            #[inline(always)]
            pub const fn reverse_bits(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).reverse_bits());
            }
            #[inline(always)]
            pub const fn isolate_highest_one(self) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).isolate_highest_one(),
                );
            }
            #[inline(always)]
            pub const fn isolate_lowest_one(self) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).isolate_lowest_one(),
                );
            }
            #[inline(always)]
            pub const fn to_be(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).to_be());
            }
            #[inline(always)]
            pub const fn to_le(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).to_le());
            }
            #[inline(always)]
            pub const fn from_be(value: Self) -> Self {
                return Self::_result_or_panic(u16::from_be(Self::raw(value)));
            }
            #[inline(always)]
            pub const fn from_le(value: Self) -> Self {
                return Self::_result_or_panic(u16::from_le(Self::raw(value)));
            }
            #[inline(always)]
            pub const fn from_ne_bytes(bytes: [u8; size_of::<u16>()]) -> Self {
                return Self::_result_or_panic(u16::from_ne_bytes(bytes));
            }
            #[inline(always)]
            pub const fn from_be_bytes(bytes: [u8; size_of::<u16>()]) -> Self {
                return Self::_result_or_panic(u16::from_be_bytes(bytes));
            }
            #[inline(always)]
            pub const fn from_le_bytes(bytes: [u8; size_of::<u16>()]) -> Self {
                return Self::_result_or_panic(u16::from_le_bytes(bytes));
            }
            #[inline(always)]
            pub const fn checked_add(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_add(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn checked_sub(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_sub(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[doc = " Cannot fail validation: every result lies in the domain `1..=1023 | 49_152..=65_535` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn checked_neg(self) -> Option<Self> {
                return match Self::raw(self).checked_neg() {
                    Some(value) => Self::_make_proven(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn checked_mul(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_mul(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn checked_div(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_div(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn checked_rem(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_rem(Self::raw(rhs)) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn checked_div_euclid(
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
            pub const fn checked_rem_euclid(
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
            pub const fn checked_shl(
                self,
                rhs: u32,
            ) -> Option<Self> {
                return match Self::raw(self).checked_shl(rhs) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn checked_shr(
                self,
                rhs: u32,
            ) -> Option<Self> {
                return match Self::raw(self).checked_shr(rhs) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn checked_pow(
                self,
                exp: u32,
            ) -> Option<Self> {
                return match Self::raw(self).checked_pow(exp) {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn midpoint(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).midpoint(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn unbounded_shl(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).unbounded_shl(rhs),
                );
            }
            #[inline(always)]
            pub const fn unbounded_shr(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).unbounded_shr(rhs),
                );
            }
            #[inline(always)]
            pub const fn saturating_add(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).saturating_add(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn saturating_sub(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).saturating_sub(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn saturating_mul(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).saturating_mul(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn saturating_div(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).saturating_div(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn saturating_pow(
                self,
                exp: u32,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).saturating_pow(exp),
                );
            }
            #[inline(always)]
            pub const fn wrapping_add(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_add(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_sub(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_sub(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_neg(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).wrapping_neg());
            }
            #[inline(always)]
            pub const fn wrapping_mul(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_mul(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_div(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_div(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_rem(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_rem(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_div_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_rem_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_rem_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_shl(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_shl(rhs),
                );
            }
            #[inline(always)]
            pub const fn wrapping_shr(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_shr(rhs),
                );
            }
            #[inline(always)]
            pub const fn wrapping_pow(
                self,
                exp: u32,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).wrapping_pow(exp),
                );
            }
            #[inline(always)]
            pub const fn pow(
                self,
                exp: u32,
            ) -> Self {
                return Self::_result_or_panic(Self::raw(self).pow(exp));
            }
            #[doc = " Cannot fail validation: every result lies in the domain `1..=1023 | 49_152..=65_535` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn isqrt(self) -> Self {
                return Self::_proven(Self::raw(self).isqrt());
            }
            #[inline(always)]
            pub const fn div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).div_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn rem_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).rem_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_add(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).strict_add(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_sub(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).strict_sub(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `1..=1023 | 49_152..=65_535` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_neg(self) -> Self {
                return Self::_proven(Self::raw(self).strict_neg());
            }
            #[inline(always)]
            pub const fn strict_mul(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).strict_mul(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_div(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).strict_div(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_rem(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).strict_rem(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).strict_div_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_rem_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).strict_rem_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_shl(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_result_or_panic(Self::raw(self).strict_shl(rhs));
            }
            #[inline(always)]
            pub const fn strict_shr(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_result_or_panic(Self::raw(self).strict_shr(rhs));
            }
            #[inline(always)]
            pub const fn strict_pow(
                self,
                exp: u32,
            ) -> Self {
                return Self::_result_or_panic(Self::raw(self).strict_pow(exp));
            }
            #[inline(always)]
            pub const fn overflowing_add(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_add(Self::raw(rhs));
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_sub(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_sub(Self::raw(rhs));
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_neg(self) -> (Self, bool) {
                let (value, overflowed) = Self::raw(self).overflowing_neg();
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_mul(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_mul(Self::raw(rhs));
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_div(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_div(Self::raw(rhs));
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_rem(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_rem(Self::raw(rhs));
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_div_euclid(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_div_euclid(Self::raw(rhs));
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_rem_euclid(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_rem_euclid(Self::raw(rhs));
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_shl(
                self,
                rhs: u32,
            ) -> (Self, bool) {
                let (value, overflowed) = Self::raw(self).overflowing_shl(rhs);
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_shr(
                self,
                rhs: u32,
            ) -> (Self, bool) {
                let (value, overflowed) = Self::raw(self).overflowing_shr(rhs);
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_pow(
                self,
                exp: u32,
            ) -> (Self, bool) {
                let (value, overflowed) = Self::raw(self).overflowing_pow(exp);
                return (Self::_result_or_panic(value), overflowed);
            }
            #[inline(always)]
            pub const fn cast_signed(self) -> i16 {
                return Self::raw(self).cast_signed();
            }
            #[inline(always)]
            pub const fn bit_width(self) -> u32 {
                return Self::raw(self).bit_width();
            }
            #[inline(always)]
            pub const fn abs_diff(
                self,
                rhs: Self,
            ) -> u16 {
                return Self::raw(self).abs_diff(Self::raw(rhs));
            }
            #[inline(always)]
            pub const fn is_multiple_of(
                self,
                rhs: Self,
            ) -> bool {
                return Self::raw(self).is_multiple_of(Self::raw(rhs));
            }
            #[inline(always)]
            pub const fn is_power_of_two(self) -> bool {
                return Self::raw(self).is_power_of_two();
            }
            #[inline(always)]
            pub const fn next_power_of_two(self) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).next_power_of_two(),
                );
            }
            #[inline(always)]
            pub const fn checked_next_power_of_two(self) -> Option<Self> {
                return match Self::raw(self).checked_next_power_of_two() {
                    Some(value) => Self::_make(value),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn div_ceil(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).div_ceil(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn next_multiple_of(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).next_multiple_of(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn checked_next_multiple_of(
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
            #[inline(always)]
            pub fn carrying_add(
                self,
                rhs: Self,
                carry: bool,
            ) -> (Self, bool) {
                let (value, carry) =
                    Self::raw(self).carrying_add(Self::raw(rhs), carry);
                return (Self::_result_or_panic(value), carry);
            }
            #[inline(always)]
            pub fn borrowing_sub(
                self,
                rhs: Self,
                borrow: bool,
            ) -> (Self, bool) {
                let (value, borrow) =
                    Self::raw(self).borrowing_sub(Self::raw(rhs), borrow);
                return (Self::_result_or_panic(value), borrow);
            }
            #[inline(always)]
            pub fn carrying_mul(
                self,
                rhs: Self,
                carry: Self,
            ) -> (Self, Self) {
                let (low, high) = Self::raw(self)
                    .carrying_mul(Self::raw(rhs), Self::raw(carry));
                return (
                    Self::_result_or_panic(low),
                    Self::_result_or_panic(high),
                );
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
                return (
                    Self::_result_or_panic(low),
                    Self::_result_or_panic(high),
                );
            }
        }
        impl Into<usize> for Port {
            #[inline(always)]
            fn into(self) -> usize {
                return Self::into_usize(self);
            }
        }
        #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
        impl Into<isize> for Port {
            #[inline(always)]
            fn into(self) -> isize {
                return Self::into_isize(self);
            }
        }
        impl Into<u16> for Port {
            #[inline(always)]
            fn into(self) -> u16 {
                return Self::into_u16(self);
            }
        }
        impl Into<u32> for Port {
            #[inline(always)]
            fn into(self) -> u32 {
                return Self::into_u32(self);
            }
        }
        impl Into<u64> for Port {
            #[inline(always)]
            fn into(self) -> u64 {
                return Self::into_u64(self);
            }
        }
        impl Into<u128> for Port {
            #[inline(always)]
            fn into(self) -> u128 {
                return Self::into_u128(self);
            }
        }
        impl Into<i32> for Port {
            #[inline(always)]
            fn into(self) -> i32 {
                return Self::into_i32(self);
            }
        }
        impl Into<i64> for Port {
            #[inline(always)]
            fn into(self) -> i64 {
                return Self::into_i64(self);
            }
        }
        impl Into<i128> for Port {
            #[inline(always)]
            fn into(self) -> i128 {
                return Self::into_i128(self);
            }
        }
        impl Into<f32> for Port {
            #[inline(always)]
            fn into(self) -> f32 {
                return Self::into_f32(self);
            }
        }
        impl Into<f64> for Port {
            #[inline(always)]
            fn into(self) -> f64 {
                return Self::into_f64(self);
            }
        }
        #[cfg(target_pointer_width = "16")]
        impl TryInto<isize> for Port {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<isize, Self::Error> {
                return Self::try_into_isize(self);
            }
        }
        impl TryInto<u8> for Port {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u8, Self::Error> {
                return Self::try_into_u8(self);
            }
        }
        impl TryInto<i8> for Port {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i8, Self::Error> {
                return Self::try_into_i8(self);
            }
        }
        impl TryInto<i16> for Port {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i16, Self::Error> {
                return Self::try_into_i16(self);
            }
        }
        impl core::ops::Shr<usize> for Port {
            type Output = Self;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        impl core::ops::Shl<usize> for Port {
            type Output = Self;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        impl core::ops::ShrAssign<usize> for Port {
            #[inline(always)]
            fn shr_assign(
                &mut self,
                that: usize,
            ) {
                *self = self._shr(that);
            }
        }
        impl core::ops::ShlAssign<usize> for Port {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                that: usize,
            ) {
                *self = self._shl(that);
            }
        }
        impl core::ops::BitAndAssign for Port {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitand(rhs.0.get());
            }
        }
        impl<T> core::ops::BitAndAssign<T> for Port
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_port(rhs);
                *self = self._bitand(it);
            }
        }
        impl core::ops::AddAssign for Port {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._add(rhs.0.get());
            }
        }
        impl<T> core::ops::AddAssign<T> for Port
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_port(rhs);
                *self = self._add(it);
            }
        }
        impl core::ops::SubAssign for Port {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._sub(rhs.0.get());
            }
        }
        impl<T> core::ops::SubAssign<T> for Port
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_port(rhs);
                *self = self._sub(it);
            }
        }
        impl core::ops::MulAssign for Port {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._mul(rhs.0.get());
            }
        }
        impl<T> core::ops::MulAssign<T> for Port
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_port(rhs);
                *self = self._mul(it);
            }
        }
        impl core::ops::DivAssign for Port {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._div(rhs.0.get());
            }
        }
        impl<T> core::ops::DivAssign<T> for Port
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_port(rhs);
                *self = self._div(it);
            }
        }
        impl core::ops::RemAssign for Port {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._rem(rhs.0.get());
            }
        }
        impl<T> core::ops::RemAssign<T> for Port
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_port(rhs);
                *self = self._rem(it);
            }
        }
        impl core::ops::BitOrAssign for Port {
            #[doc = " Cannot fail validation: every result lies in the domain `1..=1023 | 49_152..=65_535` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitor_proven(rhs.0.get());
            }
        }
        impl<T> core::ops::BitOrAssign<T> for Port
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_port(rhs);
                *self = self._bitor(it);
            }
        }
        impl core::fmt::Debug for Port {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f.debug_tuple("Port").field(&self.0.get()).finish();
            }
        }
        impl core::hash::Hash for Port {
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
        impl Eq for Port {}
        impl Ord for Port {
            #[inline(always)]
            fn cmp(
                &self,
                that: &Self,
            ) -> core::cmp::Ordering {
                return self.partial_cmp(that).unwrap();
            }
        }
        if !(size_of::<Port>() == size_of::<u16>()) {
            core::panic!("invalid integral storage size");
        };
        if !(align_of::<Port>() == align_of::<u16>()) {
            core::panic!("invalid integral storage alignment");
        };
        impl core::ops::Add for Port {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._add(rhs.0.get());
            }
        }
        impl<T> core::ops::Add<T> for Port
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_port(rhs);
                return self._add(it);
            }
        }
        impl core::ops::Sub for Port {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._sub(rhs.0.get());
            }
        }
        impl<T> core::ops::Sub<T> for Port
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_port(rhs);
                return self._sub(it);
            }
        }
        impl core::ops::Mul for Port {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._mul(rhs.0.get());
            }
        }
        impl<T> core::ops::Mul<T> for Port
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_port(rhs);
                return self._mul(it);
            }
        }
        impl core::ops::Div for Port {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._div(rhs.0.get());
            }
        }
        impl<T> core::ops::Div<T> for Port
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_port(rhs);
                return self._div(it);
            }
        }
        impl core::ops::Rem for Port {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._rem(rhs.0.get());
            }
        }
        impl<T> core::ops::Rem<T> for Port
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_port(rhs);
                return self._rem(it);
            }
        }
        impl core::ops::BitAnd for Port {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitand(rhs.0.get());
            }
        }
        impl<T> core::ops::BitAnd<T> for Port
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_port(rhs);
                return self._bitand(it);
            }
        }
        impl core::ops::BitOr for Port {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `1..=1023 | 49_152..=65_535` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitor_proven(rhs.0.get());
            }
        }
        impl<T> core::ops::BitOr<T> for Port
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_port(rhs);
                return self._bitor(it);
            }
        }
        impl core::ops::BitXor for Port {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitxor(rhs.0.get());
            }
        }
        impl core::ops::BitXorAssign for Port {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitxor(rhs.0.get());
            }
        }
        impl<T> core::ops::BitXor<T> for Port
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_port(rhs);
                return self._bitxor(it);
            }
        }
        impl<T> core::ops::BitXorAssign<T> for Port
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_port(rhs);
                *self = self._bitxor(it);
            }
        }
        impl core::ops::Not for Port {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        impl core::fmt::Binary for Port {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl core::fmt::Octal for Port {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl core::fmt::LowerHex for Port {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl core::fmt::UpperHex for Port {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        impl<T> PartialEq<T> for Port
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_port(*rhs);
                return lhs == rhs;
            }
        }
        impl<T> PartialOrd<T> for Port
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> Option<core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_port(*rhs);
                return PartialOrd::partial_cmp(&lhs, &rhs);
            }
            #[inline(always)]
            fn lt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) < Seal::conv_port(*rhs);
            }
            #[inline(always)]
            fn le(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) <= Seal::conv_port(*rhs);
            }
            #[inline(always)]
            fn gt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) > Seal::conv_port(*rhs);
            }
            #[inline(always)]
            fn ge(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) >= Seal::conv_port(*rhs);
            }
        }
        impl Port {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> Port
            where
                T: Make,
            {
                return <T as Make>::make(it);
            }
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> u16 {
                return self.0.get();
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_usize(self) -> usize {
                let it = Self::raw(self);
                return it as usize;
            }
            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64"
            ))]
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
            ) -> Result<Self, u16>
            where
                F: FnOnce(u16) -> u16,
            {
                let value = function(Self::raw(self));
                return if value >= 1 && value <= 1023 || value >= 49_152 {
                    Ok(Self(unsafe {
                        core::num::NonZeroU16::new_unchecked(value)
                    }))
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
                F: FnOnce(u16) -> u16,
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
            pub const fn try_into_usize(self) -> Result<usize, ()> {
                return Ok(Self::raw(self) as usize);
            }
            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64"
            ))]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) -> Result<isize, ()> {
                return Ok(Self::raw(self) as isize);
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
            #[cfg(target_pointer_width = "16")]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) -> Result<isize, ()> {
                let r = Self::raw(self);
                let t = r as isize;
                let s = t as u16;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u8(self) -> Result<u8, ()> {
                let r = Self::raw(self);
                let t = r as u8;
                let s = t as u16;
                return if s == r && true { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i8(self) -> Result<i8, ()> {
                let r = Self::raw(self);
                let t = r as i8;
                let s = t as u16;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i16(self) -> Result<i16, ()> {
                let r = Self::raw(self);
                let t = r as i16;
                let s = t as u16;
                return if s == r && t >= 0 { Ok(t) } else { Err(()) };
            }
            #[inline(always)]
            pub(self) fn _add(
                &self,
                rhs: u16,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs + rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _sub(
                &self,
                rhs: u16,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs - rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _mul(
                &self,
                rhs: u16,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs * rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _div(
                &self,
                rhs: u16,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs / rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _rem(
                &self,
                rhs: u16,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs % rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _bitxor(
                &self,
                rhs: u16,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs ^ rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _bitand(
                &self,
                rhs: u16,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs & rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _bitor(
                &self,
                rhs: u16,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs | rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _bitor_proven(
                &self,
                rhs: u16,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs | rhs;
                return Self::_proven(it);
            }
            #[inline(always)]
            pub(self) fn _shr(
                &self,
                rhs: usize,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs >> rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _shl(
                &self,
                rhs: usize,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs << rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _not(&self) -> Self {
                let lhs = Self::raw(*self);
                let it = !lhs;
                return Self::_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _eq(
                self,
                rhs: u16,
            ) -> bool {
                let lhs = Self::raw(self);
                return lhs == rhs;
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) fn _cmp(
                self,
                rhs: u16,
            ) -> core::cmp::Ordering {
                let lhs = Self::raw(self);
                return PartialOrd::partial_cmp(&lhs, &rhs).unwrap();
            }
            #[inline(always)]
            pub const fn try_make(it: u16) -> Result<Self, u16> {
                return if it >= 1 && it <= 1023 || it >= 49_152 {
                    return Ok(Self(unsafe {
                        core::num::NonZeroU16::new_unchecked(it)
                    }));
                }
                else {
                    Err(it)
                };
            }
            #[must_use]
            #[inline(always)]
            pub(self) fn _or_panic(it: u16) -> Self {
                if it >= 1 && it <= 1023 || it >= 49_152 {
                    return Self(unsafe {
                        core::num::NonZeroU16::new_unchecked(it)
                    });
                }
                else {
                    core::panic!("invalid value");
                };
            }
            #[doc = r" Builds a result that an expansion-time closure proof"]
            #[doc = r" shows valid; debug builds still check it."]
            #[must_use]
            #[inline(always)]
            pub(self) const fn _proven(it: u16) -> Self {
                debug_assert!(
                    ((it >= (1)) && (it <= (1023))) || (it >= (49_152)),
                    "typekin closure proof violated",
                );
                return Self(unsafe {
                    core::num::NonZeroU16::new_unchecked(it)
                });
            }
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_RAW: (bool, u32) = (false, u16::BITS);
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_FORM: Option<(
                u128,
                &'static [(u128, bool, u128, bool, u128)],
            )> = Some((
                1u128,
                &[
                    (0u128, false, 1u128, false, 1023u128),
                    (0u128, false, 49152u128, false, 65535u128),
                ],
            ));
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_WHY: &'static str = "";
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_TABLE: Option<[bool; 256]> = None;
            #[doc(hidden)]
            #[inline(always)]
            pub const fn __typekin_within_v1_raw(self) -> u16 {
                return self.0.get();
            }
            #[inline(always)]
            #[track_caller]
            pub(self) const fn _result_or_panic(value: u16) -> Self {
                if value >= 1 && value <= 1023 || value >= 49_152 {
                    return Self(unsafe {
                        core::num::NonZeroU16::new_unchecked(value)
                    });
                }
                else {
                    core::panic!("operation produced an invalid value");
                }
            }
            #[inline(always)]
            pub(self) const fn _make(value: u16) -> Option<Self> {
                return if value >= 1 && value <= 1023 || value >= 49_152 {
                    Some(Self(unsafe {
                        core::num::NonZeroU16::new_unchecked(value)
                    }))
                }
                else {
                    None
                };
            }
            #[inline(always)]
            pub(self) const fn _make_proven(value: u16) -> Option<Self> {
                return Some(Self::_proven(value));
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl Port {
            #[must_use]
            #[inline(always)]
            pub const fn lo8(self) -> u8 {
                return self.byte0();
            }
            #[must_use]
            #[inline(always)]
            pub const fn hi8(self) -> u8 {
                return self.byte1();
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte0(self) -> u8 {
                return (Self::raw(self) >> 8 * 0usize & 0xFF as u16) as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte1(self) -> u8 {
                return (Self::raw(self) >> 8 * 1usize & 0xFF as u16) as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word0(self) -> u16 {
                return (Self::raw(self) >> 16 * 0usize & 0xFFFF as u16) as u16;
            }
        }
    };
}
#[allow(unused_imports)]
pub use __port::Port;
#[allow(unused_imports)]
pub use __port::port;

fn main() {
    let half = Percent::try_make(50).unwrap();
    let third = Percent::try_make(33).unwrap();

    // Proven: these compile to the bare `u8` operations.
    let ratio = half / third;
    let middle = half.midpoint(third);

    let ssh = Port::try_make(22).unwrap();
    let dynamic = Port::try_make(50_000).unwrap();

    // `Port | Port` is proven; `Port | u16` still validates.
    let combined = ssh | dynamic;

    println!("{} {}", ratio.raw(), middle.raw());
    println!("{}", combined.raw());
}
