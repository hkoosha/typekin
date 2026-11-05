//! Literal macros: every validated wrapper gets a macro named after its
//! snake-cased type, which checks a constant at compile time.

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
    #[allow(clippy::manual_range_contains)]
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
                    Ok(Self(match core::num::NonZeroU16::new(value) {
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
            #[inline(always)]
            pub const fn checked_neg(self) -> Option<Self> {
                return match Self::raw(self).checked_neg() {
                    Some(value) => Self::_make(value),
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
            #[inline(always)]
            pub const fn isqrt(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).isqrt());
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
            #[inline(always)]
            pub const fn strict_neg(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).strict_neg());
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
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitor(rhs.0.get());
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
            #[inline(always)]
            fn bitor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitor(rhs.0.get());
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
                    Ok(Self(match core::num::NonZeroU16::new(value) {
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
                    return Ok(Self(match core::num::NonZeroU16::new(it) {
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
            #[must_use]
            #[inline(always)]
            pub(self) fn _or_panic(it: u16) -> Self {
                if it >= 1 && it <= 1023 || it >= 49_152 {
                    return Self(match core::num::NonZeroU16::new(it) {
                        Some(value) => value,
                        None => {
                            core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
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
                return Self(match core::num::NonZeroU16::new(it) {
                    Some(value) => value,
                    None => {
                        core::panic!(
                            "zero cannot be stored by this integral type"
                        )
                    }
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
                    return Self(match core::num::NonZeroU16::new(value) {
                        Some(value) => value,
                        None => {
                            core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    });
                }
                else {
                    core::panic!("operation produced an invalid value");
                }
            }
            #[inline(always)]
            pub(self) const fn _make(value: u16) -> Option<Self> {
                return if value >= 1 && value <= 1023 || value >= 49_152 {
                    Some(Self(match core::num::NonZeroU16::new(value) {
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

#[doc(hidden)]
mod __packet {
    #[repr(transparent)]
    pub struct Packet<D: Direction>(
        core::num::NonZeroU32,
        core::marker::PhantomData<D>,
    );

    #[allow(unused_imports)]
    use super::*;
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __typekin_packet_8b71a21cb91fb41a {
        (const [$($args: tt)*]$value: expr) => {
            const {
                match Packet:: < $($args)* > ::try_make($value){
                    ::core::result::Result::Ok(value) => {
                        value.__typekin_lit_8b71a21cb91fb41a();
                        value
                    }::core::result::Result::Err(_) =>  ::core::panic!("{}", ::core::concat!(::core::stringify!($value), " is outside the domain of Packet"),),
                }
            }
        };
        (const $value: expr) => {
            const {
                match Packet::try_make($value){
                    ::core::result::Result::Ok(value) => {
                        value.__typekin_lit_8b71a21cb91fb41a();
                        value
                    }::core::result::Result::Err(_) =>  ::core::panic!("{}", ::core::concat!(::core::stringify!($value), " is outside the domain of Packet"),),
                }
            }
        };
        ([$($args: tt)*]$value: expr) => {
            {
                const VALUE: Packet< $($args)* >  = match Packet:: < $($args)* > ::try_make($value){
                    ::core::result::Result::Ok(value) => {
                        value.__typekin_lit_8b71a21cb91fb41a();
                        value
                    }::core::result::Result::Err(_) =>  ::core::panic!("{}", ::core::concat!(::core::stringify!($value), " is outside the domain of Packet"),),
                };
                VALUE
            }
        };
        ($value: expr) => {
            ::core::compile_error!("`packet!` needs generic arguments: `packet!([Args] value)`, or `packet!(const value)` to infer them")
        };
    }
    #[doc = " Builds a [`Packet`] from a constant checked at compile time: an invalid value is a compile error."]
    #[doc = ""]
    #[doc = " `packet!([Args] value)` uses a `const` item, checked by `cargo check`; `packet!(const [Args] value)` uses an inline `const` block, and `packet!(const value)` infers the arguments."]
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use __typekin_packet_8b71a21cb91fb41a as packet;
    impl<D: Direction> Copy for Packet<D> {}
    #[allow(clippy::expl_impl_clone_on_copy)]
    impl<D: Direction> Clone for Packet<D> {
        #[inline(always)]
        fn clone(&self) -> Self {
            return {
                let () = <Packet<D>>::__TYPEKIN_LAYOUT_ASSERT;
                Self(self.0, core::marker::PhantomData)
            };
        }
    }
    impl<D: Direction> Packet<D> {
        #[doc(hidden)]
        #[inline(always)]
        pub const fn __typekin_lit_8b71a21cb91fb41a(&self) {}
    }
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::manual_range_contains)]
    const _: () = {
        trait Seal<D: Direction> {
            fn conv_packet(self) -> u32;
        }
        trait Make<D: Direction>: Seal<D> {
            fn make(self) -> Packet<D>;
        }
        trait Math<D: Direction>: Seal<D> {}
        trait Bit<D: Direction>: Seal<D> {}
        trait Rel<D: Direction>: Seal<D> {}
        impl<D: Direction> Seal<D> for Packet<D> {
            #[inline(always)]
            fn conv_packet(self) -> u32 {
                return Self::raw(self);
            }
        }
        impl<D: Direction> Bit<D> for Packet<D> {}
        impl<D: Direction> Make<D> for Packet<D> {
            #[inline(always)]
            fn make(self) -> Packet<D> {
                let raw = <Self as Seal<D>>::conv_packet(self);
                return <Packet<D>>::_or_panic(raw);
            }
        }
        impl<D: Direction> Math<D> for Packet<D> {}
        impl<D: Direction> Rel<D> for Packet<D> {}
        impl<D: Direction> Seal<D> for u32 {
            #[inline(always)]
            fn conv_packet(self) -> u32 {
                return self as u32;
            }
        }
        impl<D: Direction> Bit<D> for u32 {}
        impl<D: Direction> Math<D> for u32 {}
        impl<D: Direction> Rel<D> for u32 {}
        impl<D: Direction> core::str::FromStr for Packet<D> {
            type Err = ();
            #[inline(always)]
            fn from_str(source: &str) -> Result<Self, Self::Err> {
                let value = match <u32 as core::str::FromStr>::from_str(source)
                {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(());
                    }
                };
                return if value >= 1 {
                    Ok({
                        let () = <Packet<D>>::__TYPEKIN_LAYOUT_ASSERT;
                        Self(
                            match core::num::NonZeroU32::new(value) {
                                Some(value) => value,
                                None => {
                                    core::panic!(
                                        "zero cannot be stored by this integral type"
                                    )
                                }
                            },
                            core::marker::PhantomData,
                        )
                    })
                }
                else {
                    Err(())
                };
            }
        }
        impl<D: Direction> Packet<D> {
            #[doc = r" The number of bits in the wrapped primitive integer."]
            pub const BITS: u32 = u32::BITS;
        }
        impl<D: Direction> Packet<D> {
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
            pub const fn to_be_bytes(self) -> [u8; size_of::<u32>()] {
                return Self::raw(self).to_be_bytes();
            }
            #[inline(always)]
            pub const fn to_le_bytes(self) -> [u8; size_of::<u32>()] {
                return Self::raw(self).to_le_bytes();
            }
            #[inline(always)]
            pub const fn to_ne_bytes(self) -> [u8; size_of::<u32>()] {
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
                return Self::_result_or_panic(u32::from_be(Self::raw(value)));
            }
            #[inline(always)]
            pub const fn from_le(value: Self) -> Self {
                return Self::_result_or_panic(u32::from_le(Self::raw(value)));
            }
            #[inline(always)]
            pub const fn from_ne_bytes(bytes: [u8; size_of::<u32>()]) -> Self {
                return Self::_result_or_panic(u32::from_ne_bytes(bytes));
            }
            #[inline(always)]
            pub const fn from_be_bytes(bytes: [u8; size_of::<u32>()]) -> Self {
                return Self::_result_or_panic(u32::from_be_bytes(bytes));
            }
            #[inline(always)]
            pub const fn from_le_bytes(bytes: [u8; size_of::<u32>()]) -> Self {
                return Self::_result_or_panic(u32::from_le_bytes(bytes));
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
            #[inline(always)]
            pub const fn checked_neg(self) -> Option<Self> {
                return match Self::raw(self).checked_neg() {
                    Some(value) => Self::_make(value),
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
            #[inline(always)]
            pub const fn isqrt(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).isqrt());
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
            #[inline(always)]
            pub const fn strict_neg(self) -> Self {
                return Self::_result_or_panic(Self::raw(self).strict_neg());
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
            pub const fn cast_signed(self) -> i32 {
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
            ) -> u32 {
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
        #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
        impl<D: Direction> Into<usize> for Packet<D> {
            #[inline(always)]
            fn into(self) -> usize {
                return Self::into_usize(self);
            }
        }
        #[cfg(target_pointer_width = "64")]
        impl<D: Direction> Into<isize> for Packet<D> {
            #[inline(always)]
            fn into(self) -> isize {
                return Self::into_isize(self);
            }
        }
        impl<D: Direction> Into<u32> for Packet<D> {
            #[inline(always)]
            fn into(self) -> u32 {
                return Self::into_u32(self);
            }
        }
        impl<D: Direction> Into<u64> for Packet<D> {
            #[inline(always)]
            fn into(self) -> u64 {
                return Self::into_u64(self);
            }
        }
        impl<D: Direction> Into<u128> for Packet<D> {
            #[inline(always)]
            fn into(self) -> u128 {
                return Self::into_u128(self);
            }
        }
        impl<D: Direction> Into<i64> for Packet<D> {
            #[inline(always)]
            fn into(self) -> i64 {
                return Self::into_i64(self);
            }
        }
        impl<D: Direction> Into<i128> for Packet<D> {
            #[inline(always)]
            fn into(self) -> i128 {
                return Self::into_i128(self);
            }
        }
        impl<D: Direction> Into<f64> for Packet<D> {
            #[inline(always)]
            fn into(self) -> f64 {
                return Self::into_f64(self);
            }
        }
        #[cfg(target_pointer_width = "16")]
        impl<D: Direction> TryInto<usize> for Packet<D> {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<usize, Self::Error> {
                return Self::try_into_usize(self);
            }
        }
        #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
        impl<D: Direction> TryInto<isize> for Packet<D> {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<isize, Self::Error> {
                return Self::try_into_isize(self);
            }
        }
        impl<D: Direction> TryInto<u8> for Packet<D> {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u8, Self::Error> {
                return Self::try_into_u8(self);
            }
        }
        impl<D: Direction> TryInto<u16> for Packet<D> {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<u16, Self::Error> {
                return Self::try_into_u16(self);
            }
        }
        impl<D: Direction> TryInto<i8> for Packet<D> {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i8, Self::Error> {
                return Self::try_into_i8(self);
            }
        }
        impl<D: Direction> TryInto<i16> for Packet<D> {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i16, Self::Error> {
                return Self::try_into_i16(self);
            }
        }
        impl<D: Direction> TryInto<i32> for Packet<D> {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i32, Self::Error> {
                return Self::try_into_i32(self);
            }
        }
        impl<D: Direction> TryInto<f32> for Packet<D> {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<f32, Self::Error> {
                return Self::try_into_f32(self);
            }
        }
        impl<D: Direction> core::ops::Shr<usize> for Packet<D> {
            type Output = Self;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        impl<D: Direction> core::ops::Shl<usize> for Packet<D> {
            type Output = Self;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        impl<D: Direction> core::ops::ShrAssign<usize> for Packet<D> {
            #[inline(always)]
            fn shr_assign(
                &mut self,
                that: usize,
            ) {
                *self = self._shr(that);
            }
        }
        impl<D: Direction> core::ops::ShlAssign<usize> for Packet<D> {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                that: usize,
            ) {
                *self = self._shl(that);
            }
        }
        impl<D: Direction, T> core::ops::BitAndAssign<T> for Packet<D>
        where
            T: Bit<D> + Seal<D>,
        {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: T,
            ) {
                let it = <T as Seal<D>>::conv_packet(rhs);
                *self = self._bitand(it);
            }
        }
        impl<D: Direction, T> core::ops::AddAssign<T> for Packet<D>
        where
            T: Math<D> + Seal<D>,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = <T as Seal<D>>::conv_packet(rhs);
                *self = self._add(it);
            }
        }
        impl<D: Direction, T> core::ops::SubAssign<T> for Packet<D>
        where
            T: Math<D> + Seal<D>,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = <T as Seal<D>>::conv_packet(rhs);
                *self = self._sub(it);
            }
        }
        impl<D: Direction, T> core::ops::MulAssign<T> for Packet<D>
        where
            T: Math<D> + Seal<D>,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = <T as Seal<D>>::conv_packet(rhs);
                *self = self._mul(it);
            }
        }
        impl<D: Direction, T> core::ops::DivAssign<T> for Packet<D>
        where
            T: Math<D> + Seal<D>,
        {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = <T as Seal<D>>::conv_packet(rhs);
                *self = self._div(it);
            }
        }
        impl<D: Direction, T> core::ops::RemAssign<T> for Packet<D>
        where
            T: Math<D> + Seal<D>,
        {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = <T as Seal<D>>::conv_packet(rhs);
                *self = self._rem(it);
            }
        }
        impl<D: Direction, T> core::ops::BitOrAssign<T> for Packet<D>
        where
            T: Bit<D> + Seal<D>,
        {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = <T as Seal<D>>::conv_packet(rhs);
                *self = self._bitor(it);
            }
        }
        impl<D: Direction> core::fmt::Debug for Packet<D> {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f.debug_tuple("Packet").field(&self.0.get()).finish();
            }
        }
        impl<D: Direction> core::hash::Hash for Packet<D> {
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
        impl<D: Direction> Eq for Packet<D> {}
        impl<D: Direction> Ord for Packet<D> {
            #[inline(always)]
            fn cmp(
                &self,
                that: &Self,
            ) -> core::cmp::Ordering {
                return self.partial_cmp(that).unwrap();
            }
        }
        impl<D: Direction> Packet<D> {
            const __TYPEKIN_LAYOUT_ASSERT: () = {
                if !(size_of::<Packet<D>>() == size_of::<u32>()) {
                    core::panic!("invalid integral storage size");
                };
                if !(align_of::<Packet<D>>() == align_of::<u32>()) {
                    core::panic!("invalid integral storage alignment");
                };
            };
        }
        impl<D: Direction, T> core::ops::Add<T> for Packet<D>
        where
            T: Math<D> + Seal<D>,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = <T as Seal<D>>::conv_packet(rhs);
                return self._add(it);
            }
        }
        impl<D: Direction, T> core::ops::Sub<T> for Packet<D>
        where
            T: Math<D> + Seal<D>,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = <T as Seal<D>>::conv_packet(rhs);
                return self._sub(it);
            }
        }
        impl<D: Direction, T> core::ops::Mul<T> for Packet<D>
        where
            T: Math<D> + Seal<D>,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = <T as Seal<D>>::conv_packet(rhs);
                return self._mul(it);
            }
        }
        impl<D: Direction, T> core::ops::Div<T> for Packet<D>
        where
            T: Math<D> + Seal<D>,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = <T as Seal<D>>::conv_packet(rhs);
                return self._div(it);
            }
        }
        impl<D: Direction, T> core::ops::Rem<T> for Packet<D>
        where
            T: Math<D> + Seal<D>,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = <T as Seal<D>>::conv_packet(rhs);
                return self._rem(it);
            }
        }
        impl<D: Direction, T> core::ops::BitAnd<T> for Packet<D>
        where
            T: Bit<D> + Seal<D>,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = <T as Seal<D>>::conv_packet(rhs);
                return self._bitand(it);
            }
        }
        impl<D: Direction, T> core::ops::BitOr<T> for Packet<D>
        where
            T: Bit<D> + Seal<D>,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = <T as Seal<D>>::conv_packet(rhs);
                return self._bitor(it);
            }
        }
        impl<D: Direction, T> core::ops::BitXor<T> for Packet<D>
        where
            T: Bit<D> + Seal<D>,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = <T as Seal<D>>::conv_packet(rhs);
                return self._bitxor(it);
            }
        }
        impl<D: Direction, T> core::ops::BitXorAssign<T> for Packet<D>
        where
            T: Bit<D> + Seal<D>,
        {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = <T as Seal<D>>::conv_packet(rhs);
                *self = self._bitxor(it);
            }
        }
        impl<D: Direction> core::ops::Not for Packet<D> {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        impl<D: Direction> core::fmt::Binary for Packet<D> {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl<D: Direction> core::fmt::Octal for Packet<D> {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl<D: Direction> core::fmt::LowerHex for Packet<D> {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl<D: Direction> core::fmt::UpperHex for Packet<D> {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        impl<D: Direction, T> PartialEq<T> for Packet<D>
        where
            T: Rel<D> + Seal<D> + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = <T as Seal<D>>::conv_packet(*rhs);
                return lhs == rhs;
            }
        }
        impl<D: Direction, T> PartialOrd<T> for Packet<D>
        where
            T: Rel<D> + Seal<D> + Copy,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> Option<core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = <T as Seal<D>>::conv_packet(*rhs);
                return PartialOrd::partial_cmp(&lhs, &rhs);
            }
            #[inline(always)]
            fn lt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) < <T as Seal<D>>::conv_packet(*rhs);
            }
            #[inline(always)]
            fn le(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) <= <T as Seal<D>>::conv_packet(*rhs);
            }
            #[inline(always)]
            fn gt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) > <T as Seal<D>>::conv_packet(*rhs);
            }
            #[inline(always)]
            fn ge(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) >= <T as Seal<D>>::conv_packet(*rhs);
            }
        }
        impl<D: Direction> Packet<D> {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> Packet<D>
            where
                T: Make<D>,
            {
                return <T as Make<D>>::make(it);
            }
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> u32 {
                return self.0.get();
            }
            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64"
            ))]
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_usize(self) -> usize {
                let it = Self::raw(self);
                return it as usize;
            }
            #[cfg(target_pointer_width = "64")]
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
            ) -> Result<Self, u32>
            where
                F: FnOnce(u32) -> u32,
            {
                let value = function(Self::raw(self));
                return if value >= 1 {
                    Ok({
                        let () = <Packet<D>>::__TYPEKIN_LAYOUT_ASSERT;
                        Self(
                            match core::num::NonZeroU32::new(value) {
                                Some(value) => value,
                                None => {
                                    core::panic!(
                                        "zero cannot be stored by this integral type"
                                    )
                                }
                            },
                            core::marker::PhantomData,
                        )
                    })
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
                F: FnOnce(u32) -> u32,
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
            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64"
            ))]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(self) -> Result<usize, ()> {
                return Ok(Self::raw(self) as usize);
            }
            #[cfg(target_pointer_width = "64")]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) -> Result<isize, ()> {
                return Ok(Self::raw(self) as isize);
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
            pub const fn try_into_f64(self) -> Result<f64, ()> {
                return Ok(Self::raw(self) as f64);
            }
            #[cfg(target_pointer_width = "16")]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(self) -> Result<usize, ()> {
                let r = Self::raw(self);
                let t = r as usize;
                let s = t as u32;
                return if s == r && true { Ok(t) } else { Err(()) };
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
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_f32(self) -> Result<f32, ()> {
                let r = Self::raw(self);
                let t = r as f32;
                return if t as u32 == r && t < 4294967296.0 {
                    Ok(t)
                }
                else {
                    Err(())
                };
            }
            #[inline(always)]
            pub(self) fn _add(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs + rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _sub(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs - rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _mul(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs * rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _div(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs / rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _rem(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs % rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _bitxor(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs ^ rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _bitand(
                &self,
                rhs: u32,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs & rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _bitor(
                &self,
                rhs: u32,
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
                return if it >= 1 {
                    return Ok({
                        let () = <Packet<D>>::__TYPEKIN_LAYOUT_ASSERT;
                        Self(
                            match core::num::NonZeroU32::new(it) {
                                Some(value) => value,
                                None => {
                                    core::panic!(
                                        "zero cannot be stored by this integral type"
                                    )
                                }
                            },
                            core::marker::PhantomData,
                        )
                    });
                }
                else {
                    Err(it)
                };
            }
            #[must_use]
            #[inline(always)]
            pub(self) fn _or_panic(it: u32) -> Self {
                if it >= 1 {
                    return {
                        let () = <Packet<D>>::__TYPEKIN_LAYOUT_ASSERT;
                        Self(
                            match core::num::NonZeroU32::new(it) {
                                Some(value) => value,
                                None => {
                                    core::panic!(
                                        "zero cannot be stored by this integral type"
                                    )
                                }
                            },
                            core::marker::PhantomData,
                        )
                    };
                }
                else {
                    core::panic!("invalid value");
                };
            }
            #[doc = r" Builds a result that an expansion-time closure proof"]
            #[doc = r" shows valid; debug builds still check it."]
            #[must_use]
            #[inline(always)]
            pub(self) const fn _proven(it: u32) -> Self {
                debug_assert!((it >= (1)), "typekin closure proof violated",);
                return {
                    let () = <Packet<D>>::__TYPEKIN_LAYOUT_ASSERT;
                    Self(
                        match core::num::NonZeroU32::new(it) {
                            Some(value) => value,
                            None => {
                                core::panic!(
                                    "zero cannot be stored by this integral type"
                                )
                            }
                        },
                        core::marker::PhantomData,
                    )
                };
            }
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_RAW: (bool, u32) = (false, u32::BITS);
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_FORM: Option<(
                u128,
                &'static [(u128, bool, u128, bool, u128)],
            )> = Some((1u128, &[(0u128, false, 1u128, false, 4294967295u128)]));
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_WHY: &'static str = "";
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_TABLE: Option<[bool; 256]> = None;
            #[doc(hidden)]
            #[inline(always)]
            pub const fn __typekin_within_v1_raw(self) -> u32 {
                return self.0.get();
            }
            #[inline(always)]
            #[track_caller]
            pub(self) const fn _result_or_panic(value: u32) -> Self {
                if value >= 1 {
                    return {
                        let () = <Packet<D>>::__TYPEKIN_LAYOUT_ASSERT;
                        Self(
                            match core::num::NonZeroU32::new(value) {
                                Some(value) => value,
                                None => {
                                    core::panic!(
                                        "zero cannot be stored by this integral type"
                                    )
                                }
                            },
                            core::marker::PhantomData,
                        )
                    };
                }
                else {
                    core::panic!("operation produced an invalid value");
                }
            }
            #[inline(always)]
            pub(self) const fn _make(value: u32) -> Option<Self> {
                return if value >= 1 {
                    Some({
                        let () = <Packet<D>>::__TYPEKIN_LAYOUT_ASSERT;
                        Self(
                            match core::num::NonZeroU32::new(value) {
                                Some(value) => value,
                                None => {
                                    core::panic!(
                                        "zero cannot be stored by this integral type"
                                    )
                                }
                            },
                            core::marker::PhantomData,
                        )
                    })
                }
                else {
                    None
                };
            }
            #[inline(always)]
            pub(self) const fn _make_proven(value: u32) -> Option<Self> {
                return Some(Self::_proven(value));
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl<D: Direction> Packet<D> {
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
                return (Self::raw(self) >> 8 * 0usize & 0xFF as u32) as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte1(self) -> u8 {
                return (Self::raw(self) >> 8 * 1usize & 0xFF as u32) as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte2(self) -> u8 {
                return (Self::raw(self) >> 8 * 2usize & 0xFF as u32) as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn byte3(self) -> u8 {
                return (Self::raw(self) >> 8 * 3usize & 0xFF as u32) as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word0(self) -> u16 {
                return (Self::raw(self) >> 16 * 0usize & 0xFFFF as u32) as u16;
            }
            #[must_use]
            #[inline(always)]
            pub const fn word1(self) -> u16 {
                return (Self::raw(self) >> 16 * 1usize & 0xFFFF as u32) as u16;
            }
            #[must_use]
            #[inline(always)]
            pub const fn dword0(self) -> u32 {
                return (Self::raw(self) >> 32 * 0usize & 0xFFFFFFFF as u32)
                    as u32;
            }
        }
    };
}
#[allow(unused_imports)]
pub use __packet::Packet;
#[allow(unused_imports)]
pub use __packet::packet;
#[doc(hidden)]
#[allow(dead_code)]
#[allow(non_snake_case)]
mod __typekin_marker_tree_for_Packet {
    trait MarkerTreeRoot {}
    #[allow(private_bounds)]
    pub trait Direction: MarkerTreeRoot {}
    pub struct In;

    impl MarkerTreeRoot for In {}
    impl Direction for In {}
    pub struct Out;

    impl MarkerTreeRoot for Out {}
    impl Direction for Out {}
}
#[allow(unused_imports)]
pub use __typekin_marker_tree_for_Packet::Direction;
#[allow(unused_imports)]
pub use __typekin_marker_tree_for_Packet::In;
#[allow(unused_imports)]
pub use __typekin_marker_tree_for_Packet::Out;

// `port!(0)` would not compile:
// error[E0080]: evaluation panicked: 0 is outside the domain of Port
const HTTPS: Port = port!(443);

fn first<D: Direction>() -> Packet<D> {
    // Inside generic code, the inline form infers the arguments.
    return packet!(const 1);
}

fn main() {
    let ssh = port!(22);
    let inbound: Packet<In> = first();
    let outbound = packet!([Out] 7);

    println!("{} {}", HTTPS.raw(), ssh.raw());
    println!("{} {}", inbound.raw(), outbound.raw());
}
