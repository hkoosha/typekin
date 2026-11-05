//! Packed records: a named-field struct becomes a transparent `u32` whose
//! fields are typed bit windows. `make` and the `with_*` setters never
//! validate: each field value is already valid, and rustc proves that every
//! value of each field type fits its window. Only raw ingress (`try_make`)
//! checks the fields and the unclaimed bits.

#[doc(hidden)]
mod __page_id {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct PageId(core::num::NonZeroU8);

    #[allow(unused_imports)]
    use super::*;
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __typekin_page_id_31064660153b11b4 {
        (const $value: expr) => {
            const {
                match PageId::try_make($value) {
                    ::core::result::Result::Ok(value) => {
                        value.__typekin_lit_31064660153b11b4();
                        value
                    }
                    ::core::result::Result::Err(_) => ::core::panic!(
                        "{}",
                        ::core::concat!(
                            ::core::stringify!($value),
                            " is outside the domain of PageId"
                        ),
                    ),
                }
            }
        };
        ($value: expr) => {{
            const VALUE: PageId = match PageId::try_make($value) {
                ::core::result::Result::Ok(value) => {
                    value.__typekin_lit_31064660153b11b4();
                    value
                }
                ::core::result::Result::Err(_) => ::core::panic!(
                    "{}",
                    ::core::concat!(
                        ::core::stringify!($value),
                        " is outside the domain of PageId"
                    ),
                ),
            };
            VALUE
        }};
    }
    #[doc = " Builds a [`PageId`] from a constant checked at compile time: an invalid value is a compile error."]
    #[doc = ""]
    #[doc = " `page_id!(value)` uses a `const` item, checked by `cargo check`; `page_id!(const value)` uses an inline `const` block."]
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use __typekin_page_id_31064660153b11b4 as page_id;
    impl PageId {
        #[doc(hidden)]
        #[inline(always)]
        pub const fn __typekin_lit_31064660153b11b4(&self) {}
    }
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::manual_range_contains)]
    const _: () = {
        trait Seal {
            fn conv_page_id(self) -> u8;
        }
        trait Make: Seal {
            fn make(self) -> PageId;
        }
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Rel: Seal {}
        impl Seal for PageId {
            #[inline(always)]
            fn conv_page_id(self) -> u8 {
                return Self::raw(self);
            }
        }
        impl Make for PageId {
            #[inline(always)]
            fn make(self) -> PageId {
                let raw = <Self as Seal>::conv_page_id(self);
                return PageId::_or_panic(raw);
            }
        }
        impl Rel for PageId {}
        impl Seal for u8 {
            #[inline(always)]
            fn conv_page_id(self) -> u8 {
                return self as u8;
            }
        }
        impl Bit for u8 {}
        impl Math for u8 {}
        impl Rel for u8 {}
        impl core::str::FromStr for PageId {
            type Err = ();
            #[inline(always)]
            fn from_str(source: &str) -> Result<Self, Self::Err> {
                let value = match <u8 as core::str::FromStr>::from_str(source) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(());
                    }
                };
                return if value >= 1 {
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
        impl PageId {
            #[doc = r" The number of bits in the wrapped primitive integer."]
            pub const BITS: u32 = u8::BITS;
        }
        impl PageId {
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
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn checked_add(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_add(Self::raw(rhs)) {
                    Some(value) => Self::_make_proven(value),
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
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn checked_neg(self) -> Option<Self> {
                return match Self::raw(self).checked_neg() {
                    Some(value) => Self::_make_proven(value),
                    None => None,
                };
            }
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn checked_mul(
                self,
                rhs: Self,
            ) -> Option<Self> {
                return match Self::raw(self).checked_mul(Self::raw(rhs)) {
                    Some(value) => Self::_make_proven(value),
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
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
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
            #[inline(always)]
            pub const fn unbounded_shr(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_result_or_panic(
                    Self::raw(self).unbounded_shr(rhs),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn saturating_add(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
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
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn saturating_mul(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
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
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_add(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
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
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_neg(self) -> Self {
                return Self::_proven(Self::raw(self).strict_neg());
            }
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_mul(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
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
        impl Into<usize> for PageId {
            #[inline(always)]
            fn into(self) -> usize {
                return Self::into_usize(self);
            }
        }
        impl Into<isize> for PageId {
            #[inline(always)]
            fn into(self) -> isize {
                return Self::into_isize(self);
            }
        }
        impl Into<u8> for PageId {
            #[inline(always)]
            fn into(self) -> u8 {
                return Self::into_u8(self);
            }
        }
        impl Into<u16> for PageId {
            #[inline(always)]
            fn into(self) -> u16 {
                return Self::into_u16(self);
            }
        }
        impl Into<u32> for PageId {
            #[inline(always)]
            fn into(self) -> u32 {
                return Self::into_u32(self);
            }
        }
        impl Into<u64> for PageId {
            #[inline(always)]
            fn into(self) -> u64 {
                return Self::into_u64(self);
            }
        }
        impl Into<u128> for PageId {
            #[inline(always)]
            fn into(self) -> u128 {
                return Self::into_u128(self);
            }
        }
        impl Into<i16> for PageId {
            #[inline(always)]
            fn into(self) -> i16 {
                return Self::into_i16(self);
            }
        }
        impl Into<i32> for PageId {
            #[inline(always)]
            fn into(self) -> i32 {
                return Self::into_i32(self);
            }
        }
        impl Into<i64> for PageId {
            #[inline(always)]
            fn into(self) -> i64 {
                return Self::into_i64(self);
            }
        }
        impl Into<i128> for PageId {
            #[inline(always)]
            fn into(self) -> i128 {
                return Self::into_i128(self);
            }
        }
        impl Into<f32> for PageId {
            #[inline(always)]
            fn into(self) -> f32 {
                return Self::into_f32(self);
            }
        }
        impl Into<f64> for PageId {
            #[inline(always)]
            fn into(self) -> f64 {
                return Self::into_f64(self);
            }
        }
        impl TryInto<i8> for PageId {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i8, Self::Error> {
                return Self::try_into_i8(self);
            }
        }
        impl core::ops::Shr<usize> for PageId {
            type Output = Self;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        impl core::ops::Shl<usize> for PageId {
            type Output = Self;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        impl core::ops::ShrAssign<usize> for PageId {
            #[inline(always)]
            fn shr_assign(
                &mut self,
                that: usize,
            ) {
                *self = self._shr(that);
            }
        }
        impl core::ops::ShlAssign<usize> for PageId {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                that: usize,
            ) {
                *self = self._shl(that);
            }
        }
        impl core::ops::BitAndAssign for PageId {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitand(rhs.0.get());
            }
        }
        impl<T> core::ops::BitAndAssign<T> for PageId
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_id(rhs);
                *self = self._bitand(it);
            }
        }
        impl core::ops::AddAssign for PageId {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._add(rhs.0.get());
            }
        }
        impl<T> core::ops::AddAssign<T> for PageId
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_id(rhs);
                *self = self._add(it);
            }
        }
        impl core::ops::SubAssign for PageId {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._sub(rhs.0.get());
            }
        }
        impl<T> core::ops::SubAssign<T> for PageId
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_id(rhs);
                *self = self._sub(it);
            }
        }
        impl core::ops::MulAssign for PageId {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._mul(rhs.0.get());
            }
        }
        impl<T> core::ops::MulAssign<T> for PageId
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_id(rhs);
                *self = self._mul(it);
            }
        }
        impl core::ops::DivAssign for PageId {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._div(rhs.0.get());
            }
        }
        impl<T> core::ops::DivAssign<T> for PageId
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_id(rhs);
                *self = self._div(it);
            }
        }
        impl core::ops::RemAssign for PageId {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._rem(rhs.0.get());
            }
        }
        impl<T> core::ops::RemAssign<T> for PageId
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_id(rhs);
                *self = self._rem(it);
            }
        }
        impl core::ops::BitOrAssign for PageId {
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitor(rhs.0.get());
            }
        }
        impl<T> core::ops::BitOrAssign<T> for PageId
        where
            T: Bit + Seal,
        {
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_id(rhs);
                *self = self._bitor(it);
            }
        }
        impl core::fmt::Debug for PageId {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f.debug_tuple("PageId").field(&self.0.get()).finish();
            }
        }
        impl core::hash::Hash for PageId {
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
        impl Eq for PageId {}
        impl Ord for PageId {
            #[inline(always)]
            fn cmp(
                &self,
                that: &Self,
            ) -> core::cmp::Ordering {
                return self.partial_cmp(that).unwrap();
            }
        }
        if !(size_of::<PageId>() == size_of::<u8>()) {
            core::panic!("invalid integral storage size");
        };
        if !(align_of::<PageId>() == align_of::<u8>()) {
            core::panic!("invalid integral storage alignment");
        };
        impl core::ops::Add for PageId {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._add(rhs.0.get());
            }
        }
        impl<T> core::ops::Add<T> for PageId
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_id(rhs);
                return self._add(it);
            }
        }
        impl core::ops::Sub for PageId {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._sub(rhs.0.get());
            }
        }
        impl<T> core::ops::Sub<T> for PageId
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_id(rhs);
                return self._sub(it);
            }
        }
        impl core::ops::Mul for PageId {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._mul(rhs.0.get());
            }
        }
        impl<T> core::ops::Mul<T> for PageId
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_id(rhs);
                return self._mul(it);
            }
        }
        impl core::ops::Div for PageId {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._div(rhs.0.get());
            }
        }
        impl<T> core::ops::Div<T> for PageId
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_id(rhs);
                return self._div(it);
            }
        }
        impl core::ops::Rem for PageId {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._rem(rhs.0.get());
            }
        }
        impl<T> core::ops::Rem<T> for PageId
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_id(rhs);
                return self._rem(it);
            }
        }
        impl core::ops::BitAnd for PageId {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitand(rhs.0.get());
            }
        }
        impl<T> core::ops::BitAnd<T> for PageId
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_id(rhs);
                return self._bitand(it);
            }
        }
        impl core::ops::BitOr for PageId {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitor(rhs.0.get());
            }
        }
        impl<T> core::ops::BitOr<T> for PageId
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `1..` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_id(rhs);
                return self._bitor(it);
            }
        }
        impl core::ops::BitXor for PageId {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitxor(rhs.0.get());
            }
        }
        impl core::ops::BitXorAssign for PageId {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitxor(rhs.0.get());
            }
        }
        impl<T> core::ops::BitXor<T> for PageId
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_id(rhs);
                return self._bitxor(it);
            }
        }
        impl<T> core::ops::BitXorAssign<T> for PageId
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_id(rhs);
                *self = self._bitxor(it);
            }
        }
        impl core::ops::Not for PageId {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        impl core::fmt::Binary for PageId {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl core::fmt::Octal for PageId {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl core::fmt::LowerHex for PageId {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl core::fmt::UpperHex for PageId {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        impl<T> PartialEq<T> for PageId
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_page_id(*rhs);
                return lhs == rhs;
            }
        }
        impl<T> PartialOrd<T> for PageId
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> Option<core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_page_id(*rhs);
                return PartialOrd::partial_cmp(&lhs, &rhs);
            }
            #[inline(always)]
            fn lt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) < Seal::conv_page_id(*rhs);
            }
            #[inline(always)]
            fn le(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) <= Seal::conv_page_id(*rhs);
            }
            #[inline(always)]
            fn gt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) > Seal::conv_page_id(*rhs);
            }
            #[inline(always)]
            fn ge(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) >= Seal::conv_page_id(*rhs);
            }
        }
        impl PageId {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> PageId
            where
                T: Make,
            {
                return <T as Make>::make(it);
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
                return if value >= 1 {
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
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _rem(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs % rhs;
                return Self::_or_panic(it);
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
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) fn _bitor(
                &self,
                rhs: u8,
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
                return if it >= 1 {
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
            #[must_use]
            #[inline(always)]
            pub(self) fn _or_panic(it: u8) -> Self {
                if it >= 1 {
                    return Self(match core::num::NonZeroU8::new(it) {
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
            pub(self) const fn _proven(it: u8) -> Self {
                debug_assert!((it >= (1)), "typekin closure proof violated",);
                return Self(match core::num::NonZeroU8::new(it) {
                    Some(value) => value,
                    None => {
                        core::panic!(
                            "zero cannot be stored by this integral type"
                        )
                    }
                });
            }
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_RAW: (bool, u32) = (false, u8::BITS);
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_FORM: Option<(
                u128,
                &'static [(u128, bool, u128, bool, u128)],
            )> = Some((1u128, &[(0u128, false, 1u128, false, 255u128)]));
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_WHY: &'static str = "";
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_TABLE: Option<[bool; 256]> = {
                let mut table = [false; 256];
                let mut index = 0;
                while index < 256 {
                    let value: u8 = index as u8;
                    table[index] = value >= 1;
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
            #[track_caller]
            pub(self) const fn _result_or_panic(value: u8) -> Self {
                if value >= 1 {
                    return Self(match core::num::NonZeroU8::new(value) {
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
            pub(self) const fn _make(value: u8) -> Option<Self> {
                return if value >= 1 {
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
            #[inline(always)]
            pub(self) const fn _make_proven(value: u8) -> Option<Self> {
                return Some(Self::_proven(value));
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl PageId {
            #[must_use]
            #[inline(always)]
            pub const fn byte0(self) -> u8 {
                return (Self::raw(self) >> 8 * 0usize & 0xFF as u8) as u8;
            }
        }
    };
}
#[allow(unused_imports)]
pub use __page_id::PageId;
#[allow(unused_imports)]
pub use __page_id::page_id;

#[doc(hidden)]
mod __version {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct Version(u8);

    #[allow(unused_imports)]
    use super::*;
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __typekin_version_aaf8c3070a588f18 {
        (const $value: expr) => {
            const {
                match Version::try_make($value) {
                    ::core::result::Result::Ok(value) => {
                        value.__typekin_lit_aaf8c3070a588f18();
                        value
                    }
                    ::core::result::Result::Err(_) => ::core::panic!(
                        "{}",
                        ::core::concat!(
                            ::core::stringify!($value),
                            " is outside the domain of Version"
                        ),
                    ),
                }
            }
        };
        ($value: expr) => {{
            const VALUE: Version = match Version::try_make($value) {
                ::core::result::Result::Ok(value) => {
                    value.__typekin_lit_aaf8c3070a588f18();
                    value
                }
                ::core::result::Result::Err(_) => ::core::panic!(
                    "{}",
                    ::core::concat!(
                        ::core::stringify!($value),
                        " is outside the domain of Version"
                    ),
                ),
            };
            VALUE
        }};
    }
    #[doc = " Builds a [`Version`] from a constant checked at compile time: an invalid value is a compile error."]
    #[doc = ""]
    #[doc = " `version!(value)` uses a `const` item, checked by `cargo check`; `version!(const value)` uses an inline `const` block."]
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use __typekin_version_aaf8c3070a588f18 as version;
    impl Version {
        #[doc(hidden)]
        #[inline(always)]
        pub const fn __typekin_lit_aaf8c3070a588f18(&self) {}
    }
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::manual_range_contains)]
    const _: () = {
        trait Seal {
            fn conv_version(self) -> u8;
        }
        trait Make: Seal {
            fn make(self) -> Version;
        }
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Rel: Seal {}
        impl Seal for Version {
            #[inline(always)]
            fn conv_version(self) -> u8 {
                return Self::raw(self);
            }
        }
        impl Make for Version {
            #[inline(always)]
            fn make(self) -> Version {
                let raw = <Self as Seal>::conv_version(self);
                return Version::_or_panic(raw);
            }
        }
        impl Rel for Version {}
        impl Seal for u8 {
            #[inline(always)]
            fn conv_version(self) -> u8 {
                return self as u8;
            }
        }
        impl Bit for u8 {}
        impl Math for u8 {}
        impl Rel for u8 {}
        impl AsRef<u8> for Version {
            #[inline(always)]
            fn as_ref(&self) -> &u8 {
                return &self.0;
            }
        }
        impl core::str::FromStr for Version {
            type Err = ();
            #[inline(always)]
            fn from_str(source: &str) -> Result<Self, Self::Err> {
                let value = match <u8 as core::str::FromStr>::from_str(source) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(());
                    }
                };
                return if value < 64 { Ok(Self(value)) } else { Err(()) };
            }
        }
        impl Version {
            #[doc = r" The number of bits in the wrapped primitive integer."]
            pub const BITS: u32 = u8::BITS;
        }
        impl Version {
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn wrapping_div(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).wrapping_div(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn wrapping_rem(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).wrapping_rem(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn wrapping_div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).wrapping_div_euclid(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn isqrt(self) -> Self {
                return Self::_proven(Self::raw(self).isqrt());
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).div_euclid(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_sub(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).strict_sub(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_div(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).strict_div(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_rem(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).strict_rem(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn strict_div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_proven(
                    Self::raw(self).strict_div_euclid(Self::raw(rhs)),
                );
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn overflowing_div(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_div(Self::raw(rhs));
                return (Self::_proven(value), overflowed);
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn overflowing_rem(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_rem(Self::raw(rhs));
                return (Self::_proven(value), overflowed);
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            pub const fn overflowing_div_euclid(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_div_euclid(Self::raw(rhs));
                return (Self::_proven(value), overflowed);
            }
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
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
        impl Into<usize> for Version {
            #[inline(always)]
            fn into(self) -> usize {
                return Self::into_usize(self);
            }
        }
        impl Into<isize> for Version {
            #[inline(always)]
            fn into(self) -> isize {
                return Self::into_isize(self);
            }
        }
        impl Into<u8> for Version {
            #[inline(always)]
            fn into(self) -> u8 {
                return Self::into_u8(self);
            }
        }
        impl Into<u16> for Version {
            #[inline(always)]
            fn into(self) -> u16 {
                return Self::into_u16(self);
            }
        }
        impl Into<u32> for Version {
            #[inline(always)]
            fn into(self) -> u32 {
                return Self::into_u32(self);
            }
        }
        impl Into<u64> for Version {
            #[inline(always)]
            fn into(self) -> u64 {
                return Self::into_u64(self);
            }
        }
        impl Into<u128> for Version {
            #[inline(always)]
            fn into(self) -> u128 {
                return Self::into_u128(self);
            }
        }
        impl Into<i16> for Version {
            #[inline(always)]
            fn into(self) -> i16 {
                return Self::into_i16(self);
            }
        }
        impl Into<i32> for Version {
            #[inline(always)]
            fn into(self) -> i32 {
                return Self::into_i32(self);
            }
        }
        impl Into<i64> for Version {
            #[inline(always)]
            fn into(self) -> i64 {
                return Self::into_i64(self);
            }
        }
        impl Into<i128> for Version {
            #[inline(always)]
            fn into(self) -> i128 {
                return Self::into_i128(self);
            }
        }
        impl Into<f32> for Version {
            #[inline(always)]
            fn into(self) -> f32 {
                return Self::into_f32(self);
            }
        }
        impl Into<f64> for Version {
            #[inline(always)]
            fn into(self) -> f64 {
                return Self::into_f64(self);
            }
        }
        impl TryInto<i8> for Version {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i8, Self::Error> {
                return Self::try_into_i8(self);
            }
        }
        impl core::ops::Shr<usize> for Version {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        impl core::ops::Shl<usize> for Version {
            type Output = Self;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        impl core::ops::ShrAssign<usize> for Version {
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn shr_assign(
                &mut self,
                that: usize,
            ) {
                *self = self._shr(that);
            }
        }
        impl core::ops::ShlAssign<usize> for Version {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                that: usize,
            ) {
                *self = self._shl(that);
            }
        }
        impl core::ops::BitAndAssign for Version {
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitand(rhs.0);
            }
        }
        impl<T> core::ops::BitAndAssign<T> for Version
        where
            T: Bit + Seal,
        {
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_version(rhs);
                *self = self._bitand(it);
            }
        }
        impl core::ops::AddAssign for Version {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._add(rhs.0);
            }
        }
        impl<T> core::ops::AddAssign<T> for Version
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_version(rhs);
                *self = self._add(it);
            }
        }
        impl core::ops::SubAssign for Version {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._sub(rhs.0);
            }
        }
        impl<T> core::ops::SubAssign<T> for Version
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_version(rhs);
                *self = self._sub(it);
            }
        }
        impl core::ops::MulAssign for Version {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._mul(rhs.0);
            }
        }
        impl<T> core::ops::MulAssign<T> for Version
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_version(rhs);
                *self = self._mul(it);
            }
        }
        impl core::ops::DivAssign for Version {
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._div(rhs.0);
            }
        }
        impl<T> core::ops::DivAssign<T> for Version
        where
            T: Math + Seal,
        {
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_version(rhs);
                *self = self._div(it);
            }
        }
        impl core::ops::RemAssign for Version {
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._rem(rhs.0);
            }
        }
        impl<T> core::ops::RemAssign<T> for Version
        where
            T: Math + Seal,
        {
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_version(rhs);
                *self = self._rem(it);
            }
        }
        impl core::ops::BitOrAssign for Version {
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitor_proven(rhs.0);
            }
        }
        impl<T> core::ops::BitOrAssign<T> for Version
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_version(rhs);
                *self = self._bitor(it);
            }
        }
        impl core::fmt::Debug for Version {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f.debug_tuple("Version").field(&self.0).finish();
            }
        }
        impl core::hash::Hash for Version {
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
        impl Eq for Version {}
        impl Ord for Version {
            #[inline(always)]
            fn cmp(
                &self,
                that: &Self,
            ) -> core::cmp::Ordering {
                return self.partial_cmp(that).unwrap();
            }
        }
        if !(size_of::<Version>() == size_of::<u8>()) {
            core::panic!("invalid integral storage size");
        };
        if !(align_of::<Version>() == align_of::<u8>()) {
            core::panic!("invalid integral storage alignment");
        };
        impl core::ops::Add for Version {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._add(rhs.0);
            }
        }
        impl<T> core::ops::Add<T> for Version
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_version(rhs);
                return self._add(it);
            }
        }
        impl core::ops::Sub for Version {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._sub(rhs.0);
            }
        }
        impl<T> core::ops::Sub<T> for Version
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_version(rhs);
                return self._sub(it);
            }
        }
        impl core::ops::Mul for Version {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._mul(rhs.0);
            }
        }
        impl<T> core::ops::Mul<T> for Version
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_version(rhs);
                return self._mul(it);
            }
        }
        impl core::ops::Div for Version {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn div(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._div(rhs.0);
            }
        }
        impl<T> core::ops::Div<T> for Version
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_version(rhs);
                return self._div(it);
            }
        }
        impl core::ops::Rem for Version {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn rem(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._rem(rhs.0);
            }
        }
        impl<T> core::ops::Rem<T> for Version
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_version(rhs);
                return self._rem(it);
            }
        }
        impl core::ops::BitAnd for Version {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitand(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitand(rhs.0);
            }
        }
        impl<T> core::ops::BitAnd<T> for Version
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_version(rhs);
                return self._bitand(it);
            }
        }
        impl core::ops::BitOr for Version {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitor_proven(rhs.0);
            }
        }
        impl<T> core::ops::BitOr<T> for Version
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_version(rhs);
                return self._bitor(it);
            }
        }
        impl core::ops::BitXor for Version {
            type Output = Self;
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitxor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitxor_proven(rhs.0);
            }
        }
        impl core::ops::BitXorAssign for Version {
            #[doc = " Cannot fail validation: every result lies in the domain `..64` at every pointer width (proven by typekin at expansion)."]
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitxor_proven(rhs.0);
            }
        }
        impl<T> core::ops::BitXor<T> for Version
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_version(rhs);
                return self._bitxor(it);
            }
        }
        impl<T> core::ops::BitXorAssign<T> for Version
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_version(rhs);
                *self = self._bitxor(it);
            }
        }
        impl core::ops::Not for Version {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self._not();
            }
        }
        impl core::fmt::Binary for Version {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl core::fmt::Octal for Version {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl core::fmt::LowerHex for Version {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl core::fmt::UpperHex for Version {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        impl<T> PartialEq<T> for Version
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_version(*rhs);
                return lhs == rhs;
            }
        }
        impl<T> PartialOrd<T> for Version
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> Option<core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_version(*rhs);
                return PartialOrd::partial_cmp(&lhs, &rhs);
            }
            #[inline(always)]
            fn lt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) < Seal::conv_version(*rhs);
            }
            #[inline(always)]
            fn le(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) <= Seal::conv_version(*rhs);
            }
            #[inline(always)]
            fn gt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) > Seal::conv_version(*rhs);
            }
            #[inline(always)]
            fn ge(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) >= Seal::conv_version(*rhs);
            }
        }
        impl Version {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> Version
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
                return if value < 64 {
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
            pub(self) fn _bitxor_proven(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs ^ rhs;
                return Self::_proven(it);
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
            pub(self) fn _bitor_proven(
                &self,
                rhs: u8,
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
                return if it < 64 {
                    return Ok(Self(it));
                }
                else {
                    Err(it)
                };
            }
            #[must_use]
            #[inline(always)]
            pub(self) fn _or_panic(it: u8) -> Self {
                if it < 64 {
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
                debug_assert!((it < (64)), "typekin closure proof violated",);
                return Self(it);
            }
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_RAW: (bool, u32) = (false, u8::BITS);
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_FORM: Option<(
                u128,
                &'static [(u128, bool, u128, bool, u128)],
            )> = Some((1u128, &[(0u128, false, 0u128, false, 63u128)]));
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_WHY: &'static str = "";
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_TABLE: Option<[bool; 256]> = {
                let mut table = [false; 256];
                let mut index = 0;
                while index < 256 {
                    let value: u8 = index as u8;
                    table[index] = value < 64;
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
                if value < 64 {
                    return Self(value);
                }
                else {
                    core::panic!("operation produced an invalid value");
                }
            }
            #[inline(always)]
            pub(self) const fn _make(value: u8) -> Option<Self> {
                return if value < 64 { Some(Self(value)) } else { None };
            }
            #[inline(always)]
            pub(self) const fn _make_proven(value: u8) -> Option<Self> {
                return Some(Self::_proven(value));
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl Version {
            #[must_use]
            #[inline(always)]
            pub const fn byte0(self) -> u8 {
                return (Self::raw(self) >> 8 * 0usize & 0xFF as u8) as u8;
            }
        }
    };
}
#[allow(unused_imports)]
pub use __version::Version;
#[allow(unused_imports)]
pub use __version::version;

#[doc(hidden)]
mod __page_state {
    #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Copy, Hash, Debug)]
    #[repr(u8)]
    pub enum PageState {
        Dirty = 0b01,
        Pinned = 0b10,
    }
    #[allow(unused_imports)]
    use super::*;
    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct PageStateValue(u8);

    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::manual_range_contains)]
    const _: () = {
        trait Seal {
            fn conv_page_state_value(self) -> u8;
        }
        trait Make: Seal {
            fn make(self) -> PageStateValue;
        }
        trait Math: Seal {}
        trait Bit: Seal {}
        trait Rel: Seal {}
        impl Seal for PageStateValue {
            #[inline(always)]
            fn conv_page_state_value(self) -> u8 {
                return Self::raw(self);
            }
        }
        impl Seal for PageState {
            #[inline(always)]
            fn conv_page_state_value(self) -> u8 {
                return PageState::raw(self);
            }
        }
        impl Bit for PageState {}
        impl Make for PageState {
            #[inline(always)]
            fn make(self) -> PageStateValue {
                let raw = <Self as Seal>::conv_page_state_value(self);
                return PageStateValue::_or_panic(raw);
            }
        }
        impl Make for PageStateValue {
            #[inline(always)]
            fn make(self) -> PageStateValue {
                let raw = <Self as Seal>::conv_page_state_value(self);
                return PageStateValue::_or_panic(raw);
            }
        }
        impl Rel for PageStateValue {}
        impl Seal for u8 {
            #[inline(always)]
            fn conv_page_state_value(self) -> u8 {
                return self as u8;
            }
        }
        impl Bit for u8 {}
        impl Make for u8 {
            #[inline(always)]
            fn make(self) -> PageStateValue {
                let raw = <Self as Seal>::conv_page_state_value(self);
                return PageStateValue::_or_panic(raw);
            }
        }
        impl Math for u8 {}
        impl Rel for u8 {}
        impl AsRef<u8> for PageStateValue {
            #[inline(always)]
            fn as_ref(&self) -> &u8 {
                return &self.0;
            }
        }
        impl PageStateValue {
            #[doc = r" The number of bits in the wrapped primitive integer."]
            pub const BITS: u32 = u8::BITS;
        }
        impl PageStateValue {
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
        impl Into<usize> for PageStateValue {
            #[inline(always)]
            fn into(self) -> usize {
                return Self::into_usize(self);
            }
        }
        impl Into<isize> for PageStateValue {
            #[inline(always)]
            fn into(self) -> isize {
                return Self::into_isize(self);
            }
        }
        impl Into<u8> for PageStateValue {
            #[inline(always)]
            fn into(self) -> u8 {
                return Self::into_u8(self);
            }
        }
        impl Into<u16> for PageStateValue {
            #[inline(always)]
            fn into(self) -> u16 {
                return Self::into_u16(self);
            }
        }
        impl Into<u32> for PageStateValue {
            #[inline(always)]
            fn into(self) -> u32 {
                return Self::into_u32(self);
            }
        }
        impl Into<u64> for PageStateValue {
            #[inline(always)]
            fn into(self) -> u64 {
                return Self::into_u64(self);
            }
        }
        impl Into<u128> for PageStateValue {
            #[inline(always)]
            fn into(self) -> u128 {
                return Self::into_u128(self);
            }
        }
        impl Into<i16> for PageStateValue {
            #[inline(always)]
            fn into(self) -> i16 {
                return Self::into_i16(self);
            }
        }
        impl Into<i32> for PageStateValue {
            #[inline(always)]
            fn into(self) -> i32 {
                return Self::into_i32(self);
            }
        }
        impl Into<i64> for PageStateValue {
            #[inline(always)]
            fn into(self) -> i64 {
                return Self::into_i64(self);
            }
        }
        impl Into<i128> for PageStateValue {
            #[inline(always)]
            fn into(self) -> i128 {
                return Self::into_i128(self);
            }
        }
        impl Into<f32> for PageStateValue {
            #[inline(always)]
            fn into(self) -> f32 {
                return Self::into_f32(self);
            }
        }
        impl Into<f64> for PageStateValue {
            #[inline(always)]
            fn into(self) -> f64 {
                return Self::into_f64(self);
            }
        }
        impl TryInto<i8> for PageStateValue {
            type Error = ();
            #[inline(always)]
            fn try_into(self) -> Result<i8, Self::Error> {
                return Self::try_into_i8(self);
            }
        }
        impl core::ops::Shr<usize> for PageStateValue {
            type Output = Self;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shr(rhs);
            }
        }
        impl core::ops::Shl<usize> for PageStateValue {
            type Output = Self;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self._shl(rhs);
            }
        }
        impl core::ops::ShrAssign<usize> for PageStateValue {
            #[inline(always)]
            fn shr_assign(
                &mut self,
                that: usize,
            ) {
                *self = self._shr(that);
            }
        }
        impl core::ops::ShlAssign<usize> for PageStateValue {
            #[inline(always)]
            fn shl_assign(
                &mut self,
                that: usize,
            ) {
                *self = self._shl(that);
            }
        }
        impl core::ops::BitAndAssign for PageStateValue {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitand(rhs.0);
            }
        }
        impl<T> core::ops::BitAndAssign<T> for PageStateValue
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitand_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_state_value(rhs);
                *self = self._bitand(it);
            }
        }
        impl core::ops::AddAssign for PageStateValue {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._add(rhs.0);
            }
        }
        impl<T> core::ops::AddAssign<T> for PageStateValue
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn add_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_state_value(rhs);
                *self = self._add(it);
            }
        }
        impl core::ops::SubAssign for PageStateValue {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._sub(rhs.0);
            }
        }
        impl<T> core::ops::SubAssign<T> for PageStateValue
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn sub_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_state_value(rhs);
                *self = self._sub(it);
            }
        }
        impl core::ops::MulAssign for PageStateValue {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._mul(rhs.0);
            }
        }
        impl<T> core::ops::MulAssign<T> for PageStateValue
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn mul_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_state_value(rhs);
                *self = self._mul(it);
            }
        }
        impl core::ops::DivAssign for PageStateValue {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._div(rhs.0);
            }
        }
        impl<T> core::ops::DivAssign<T> for PageStateValue
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn div_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_state_value(rhs);
                *self = self._div(it);
            }
        }
        impl core::ops::RemAssign for PageStateValue {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._rem(rhs.0);
            }
        }
        impl<T> core::ops::RemAssign<T> for PageStateValue
        where
            T: Math + Seal,
        {
            #[inline(always)]
            fn rem_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_state_value(rhs);
                *self = self._rem(it);
            }
        }
        impl core::ops::BitOrAssign for PageStateValue {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitor(rhs.0);
            }
        }
        impl<T> core::ops::BitOrAssign<T> for PageStateValue
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_state_value(rhs);
                *self = self._bitor(it);
            }
        }
        impl core::fmt::Debug for PageStateValue {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f.debug_tuple("PageStateValue").field(&self.0).finish();
            }
        }
        impl core::hash::Hash for PageStateValue {
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
        impl Eq for PageStateValue {}
        impl Ord for PageStateValue {
            #[inline(always)]
            fn cmp(
                &self,
                that: &Self,
            ) -> core::cmp::Ordering {
                return self.partial_cmp(that).unwrap();
            }
        }
        if !(size_of::<PageStateValue>() == size_of::<u8>()) {
            core::panic!("invalid integral storage size");
        };
        if !(align_of::<PageStateValue>() == align_of::<u8>()) {
            core::panic!("invalid integral storage alignment");
        };
        impl core::ops::Add for PageStateValue {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._add(rhs.0);
            }
        }
        impl<T> core::ops::Add<T> for PageStateValue
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn add(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_state_value(rhs);
                return self._add(it);
            }
        }
        impl core::ops::Sub for PageStateValue {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._sub(rhs.0);
            }
        }
        impl<T> core::ops::Sub<T> for PageStateValue
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn sub(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_state_value(rhs);
                return self._sub(it);
            }
        }
        impl core::ops::Mul for PageStateValue {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._mul(rhs.0);
            }
        }
        impl<T> core::ops::Mul<T> for PageStateValue
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn mul(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_state_value(rhs);
                return self._mul(it);
            }
        }
        impl core::ops::Div for PageStateValue {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._div(rhs.0);
            }
        }
        impl<T> core::ops::Div<T> for PageStateValue
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn div(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_state_value(rhs);
                return self._div(it);
            }
        }
        impl core::ops::Rem for PageStateValue {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._rem(rhs.0);
            }
        }
        impl<T> core::ops::Rem<T> for PageStateValue
        where
            T: Math + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn rem(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_state_value(rhs);
                return self._rem(it);
            }
        }
        impl core::ops::BitAnd for PageStateValue {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitand(rhs.0);
            }
        }
        impl<T> core::ops::BitAnd<T> for PageStateValue
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_state_value(rhs);
                return self._bitand(it);
            }
        }
        impl core::ops::BitOr for PageStateValue {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitor(rhs.0);
            }
        }
        impl<T> core::ops::BitOr<T> for PageStateValue
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_state_value(rhs);
                return self._bitor(it);
            }
        }
        impl core::ops::BitXor for PageStateValue {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: Self,
            ) -> Self::Output {
                return self._bitxor(rhs.0);
            }
        }
        impl core::ops::BitXorAssign for PageStateValue {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: Self,
            ) {
                *self = self._bitxor(rhs.0);
            }
        }
        impl<T> core::ops::BitXor<T> for PageStateValue
        where
            T: Bit + Seal,
        {
            type Output = Self;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let it = Seal::conv_page_state_value(rhs);
                return self._bitxor(it);
            }
        }
        impl<T> core::ops::BitXorAssign<T> for PageStateValue
        where
            T: Bit + Seal,
        {
            #[inline(always)]
            fn bitxor_assign(
                &mut self,
                rhs: T,
            ) {
                let it = Seal::conv_page_state_value(rhs);
                *self = self._bitxor(it);
            }
        }
        impl core::fmt::Binary for PageStateValue {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Binary::fmt(&raw, f);
            }
        }
        impl core::fmt::Octal for PageStateValue {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::Octal::fmt(&raw, f);
            }
        }
        impl core::fmt::LowerHex for PageStateValue {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::LowerHex::fmt(&raw, f);
            }
        }
        impl core::fmt::UpperHex for PageStateValue {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::UpperHex::fmt(&raw, f);
            }
        }
        impl<T> PartialEq<T> for PageStateValue
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_page_state_value(*rhs);
                return lhs == rhs;
            }
        }
        impl<T> PartialOrd<T> for PageStateValue
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &T,
            ) -> Option<core::cmp::Ordering> {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_page_state_value(*rhs);
                return PartialOrd::partial_cmp(&lhs, &rhs);
            }
            #[inline(always)]
            fn lt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) < Seal::conv_page_state_value(*rhs);
            }
            #[inline(always)]
            fn le(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) <= Seal::conv_page_state_value(*rhs);
            }
            #[inline(always)]
            fn gt(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) > Seal::conv_page_state_value(*rhs);
            }
            #[inline(always)]
            fn ge(
                &self,
                rhs: &T,
            ) -> bool {
                return Self::raw(*self) >= Seal::conv_page_state_value(*rhs);
            }
        }
        impl PageStateValue {
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T) -> PageStateValue
            where
                T: Make,
            {
                return <T as Make>::make(it);
            }
            #[inline(always)]
            pub const fn make(it: u8) -> PageStateValue {
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
            pub(self) const fn _add(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs + rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _sub(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs - rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _mul(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs * rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _div(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs / rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _rem(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs % rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _bitxor(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs ^ rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _bitand(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs & rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _bitor(
                &self,
                rhs: u8,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs | rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _shr(
                &self,
                rhs: usize,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs >> rhs;
                return Self::_or_panic(it);
            }
            #[inline(always)]
            pub(self) const fn _shl(
                &self,
                rhs: usize,
            ) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs << rhs;
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
                return Ok(Self::_or_panic(it));
            }
            #[must_use]
            #[inline(always)]
            pub(self) const fn _or_panic(it: u8) -> Self {
                return Self(it);
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
            #[track_caller]
            pub(self) const fn _result_or_panic(value: u8) -> Self {
                {
                    return Self(value);
                }
            }
            #[inline(always)]
            pub(self) const fn _make(value: u8) -> Option<Self> {
                return Some(Self(value));
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl PageStateValue {
            #[must_use]
            #[inline(always)]
            pub const fn byte0(self) -> u8 {
                return (Self::raw(self) >> 8 * 0usize & 0xFF as u8) as u8;
            }
        }
        impl PageStateValue {
            #[allow(non_upper_case_globals)]
            pub const Dirty: Self = Self::_or_panic(PageState::Dirty as u8);
            #[allow(non_upper_case_globals)]
            pub const Pinned: Self = Self::_or_panic(PageState::Pinned as u8);
            #[must_use]
            #[inline(always)]
            pub const fn bits(self) -> u8 {
                return PageStateValue::raw(self);
            }
            #[must_use]
            #[inline(always)]
            pub const fn from_bits_retain(bits: u8) -> Self {
                return Self::_or_panic(bits);
            }
            #[must_use]
            #[inline(always)]
            pub const fn empty() -> Self {
                return Self::from_bits_retain(0);
            }
            #[must_use]
            #[inline(always)]
            pub const fn all() -> Self {
                return PageState::all();
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
            pub const fn known_bits(self) -> u8 {
                return self.bits() & Self::all().bits();
            }
            #[must_use]
            #[inline(always)]
            pub const fn unknown_bits(self) -> u8 {
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
                return self.unknown_bits() != Self::empty().bits();
            }
            #[must_use]
            #[inline(always)]
            pub const fn from_bits(bits: u8) -> Option<Self> {
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
            pub const fn from_bits_truncate(bits: u8) -> Self {
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
            pub fn from_name(name: &str) -> Option<Self> {
                return match PageState::from_name(name) {
                    Some(flag) => Some(flag.into_value()),
                    None => None,
                };
            }
            #[inline(always)]
            pub const fn into_flag(self) -> Result<PageState, Self> {
                let items = PageState::items();
                let max = items.len();
                let mut i = 0;
                while i < max {
                    let flag = items[i];
                    if flag.into_value().bits() == self.bits() {
                        return Ok(flag);
                    }
                    i += 1;
                }
                return Err(self);
            }
            #[must_use]
            #[inline(always)]
            pub fn iter_known_flags(self) -> impl Iterator<Item = PageState> {
                return IterFlags {
                    value: self,
                    index: 0,
                };
            }
            #[must_use]
            #[inline(always)]
            pub fn iter(self) -> impl Iterator<Item = Self> {
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
                return PageState::iter()
                    .map(|flag| (flag.name(), flag.into_value()));
            }
            #[must_use]
            #[inline(always)]
            pub fn iter_equal_names(
                self
            ) -> impl Iterator<Item = &'static str> {
                return PageState::iter()
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
                return self.bits() == Self::all().bits();
            }
            #[must_use]
            #[inline(always)]
            pub const fn intersects(
                self,
                that: Self,
            ) -> bool {
                return self.bits() & that.bits() != 0;
            }
            #[must_use]
            #[inline(always)]
            pub fn contains<T>(
                self,
                that: T,
            ) -> bool
            where
                T: Into<Self>,
            {
                return self.contains_all(that.into());
            }
            #[must_use]
            #[inline(always)]
            pub const fn contains_all(
                self,
                that: Self,
            ) -> bool {
                return self.bits() & that.bits() == that.bits();
            }
            #[must_use]
            #[inline(always)]
            pub const fn contains_any(
                self,
                that: Self,
            ) -> bool {
                return self.intersects(that);
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
                that: Self,
            ) -> Self {
                return Self::from_bits_retain(self.bits() | that.bits());
            }
            #[inline(always)]
            pub const fn insert(
                &mut self,
                that: Self,
            ) {
                *self = self.inserted(that);
            }
            #[must_use]
            #[inline(always)]
            pub const fn removed(
                self,
                that: Self,
            ) -> Self {
                return Self::from_bits_retain(self.bits() & !that.bits());
            }
            #[inline(always)]
            pub const fn remove(
                &mut self,
                that: Self,
            ) {
                *self = self.removed(that);
            }
            #[must_use]
            #[inline(always)]
            pub const fn toggled(
                self,
                that: Self,
            ) -> Self {
                return Self::from_bits_retain(self.bits() ^ that.bits());
            }
            #[inline(always)]
            pub const fn toggle(
                &mut self,
                that: Self,
            ) {
                *self = self.toggled(that);
            }
            #[inline(always)]
            pub const fn set(
                &mut self,
                that: Self,
                value: bool,
            ) {
                if value {
                    self.insert(that);
                }
                else {
                    self.remove(that);
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
                that: Self,
            ) -> Self {
                return self.inserted(that);
            }
            #[must_use]
            #[inline(always)]
            pub const fn without(
                self,
                that: Self,
            ) -> Self {
                return self.removed(that);
            }
            #[inline(always)]
            pub const fn unset(
                &mut self,
                that: Self,
            ) {
                self.remove(that);
            }
            #[must_use]
            #[inline(always)]
            pub const fn intersection(
                self,
                that: Self,
            ) -> Self {
                return Self::from_bits_retain(self.bits() & that.bits());
            }
            #[must_use]
            #[inline(always)]
            pub const fn union(
                self,
                that: Self,
            ) -> Self {
                return self.inserted(that);
            }
            #[must_use]
            #[inline(always)]
            pub const fn difference(
                self,
                that: Self,
            ) -> Self {
                return self.removed(that);
            }
            #[must_use]
            #[inline(always)]
            pub const fn symmetric_difference(
                self,
                that: Self,
            ) -> Self {
                return self.toggled(that);
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
        impl From<PageState> for PageStateValue {
            #[inline(always)]
            fn from(flag: PageState) -> Self {
                return flag.into_value();
            }
        }
        impl core::ops::Not for PageStateValue {
            type Output = Self;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self.complemented();
            }
        }
        impl PageState {
            #[inline(always)]
            #[must_use]
            pub const fn name(self) -> &'static str {
                return match self {
                    PageState::Dirty => "Dirty",
                    PageState::Pinned => "Pinned",
                };
            }
            #[inline(always)]
            #[must_use]
            pub const fn items() -> &'static [Self] {
                const ITEMS: &'static [PageState] = &[
                    PageState::Dirty,
                    PageState::Pinned,
                ];
                return ITEMS;
            }
            #[inline]
            #[must_use]
            pub fn from_name(name: &str) -> Option<Self> {
                return match name {
                    "Dirty" => Some(PageState::Dirty),
                    "Pinned" => Some(PageState::Pinned),
                    _ => None,
                };
            }
            #[inline(always)]
            #[must_use]
            pub const fn raw(self) -> u8 {
                return self as u8;
            }
            #[inline(always)]
            #[must_use]
            pub const fn into_value(self) -> PageStateValue {
                return PageStateValue::from_bits_retain(self.raw());
            }
            #[must_use]
            #[inline(always)]
            pub const fn all() -> PageStateValue {
                #[allow(clippy::unnecessary_cast)]
                return PageStateValue::from_bits_retain(
                    0 as u8 | PageState::Dirty as u8 | PageState::Pinned as u8,
                );
            }
            #[must_use]
            #[inline(always)]
            pub fn iter() -> impl Iterator<Item = Self> {
                return IterItems { index: 0 };
            }
            #[must_use]
            #[inline(always)]
            pub fn iter_values() -> impl Iterator<Item = PageStateValue> {
                return PageStateValue::all().iter();
            }
            #[must_use]
            #[inline(always)]
            pub const fn inserted(
                self,
                that: Self,
            ) -> PageStateValue {
                return self.into_value().inserted(that.into_value());
            }
        }
        impl PageState {
            #[must_use]
            #[inline(always)]
            pub const fn from_bits_retain(bits: u8) -> PageStateValue {
                return PageStateValue::from_bits_retain(bits);
            }
            #[must_use]
            #[inline(always)]
            pub const fn empty() -> PageStateValue {
                return PageStateValue::empty();
            }
            #[must_use]
            #[inline(always)]
            pub const fn all_named() -> PageStateValue {
                return PageStateValue::all();
            }
            #[must_use]
            #[inline(always)]
            pub const fn all_known() -> PageStateValue {
                return PageStateValue::all();
            }
            #[must_use]
            #[inline(always)]
            pub const fn all_unknown() -> PageStateValue {
                return PageStateValue::from_bits_retain(
                    !PageStateValue::all().bits(),
                );
            }
            #[must_use]
            #[inline(always)]
            pub const fn from_bits(bits: u8) -> Option<PageStateValue> {
                return PageStateValue::from_bits(bits);
            }
            #[must_use]
            #[inline(always)]
            pub const fn from_bits_truncate(bits: u8) -> PageStateValue {
                return PageStateValue::from_bits_truncate(bits);
            }
            #[must_use]
            #[inline(always)]
            pub fn iter_defined_names()
            -> impl Iterator<Item = (&'static str, PageStateValue)>
            {
                return PageStateValue::iter_defined_names();
            }
        }
        impl core::ops::Shr<usize> for PageState {
            type Output = PageStateValue;
            #[inline(always)]
            fn shr(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self.into_value() >> rhs;
            }
        }
        impl core::ops::Shl<usize> for PageState {
            type Output = PageStateValue;
            #[inline(always)]
            fn shl(
                self,
                rhs: usize,
            ) -> Self::Output {
                return self.into_value() << rhs;
            }
        }
        impl<T> core::ops::BitAnd<T> for PageState
        where
            T: FlagBit + FlagSeal,
        {
            type Output = PageStateValue;
            #[inline(always)]
            fn bitand(
                self,
                rhs: T,
            ) -> Self::Output {
                let rhs = FlagSeal::conv_page_state(rhs);
                return self
                    .into_value()
                    .intersection(PageStateValue::from_bits_retain(rhs));
            }
        }
        impl<T> core::ops::BitOr<T> for PageState
        where
            T: FlagBit + FlagSeal,
        {
            type Output = PageStateValue;
            #[inline(always)]
            fn bitor(
                self,
                rhs: T,
            ) -> Self::Output {
                let rhs = FlagSeal::conv_page_state(rhs);
                return self
                    .into_value()
                    .union(PageStateValue::from_bits_retain(rhs));
            }
        }
        impl<T> core::ops::BitXor<T> for PageState
        where
            T: FlagBit + FlagSeal,
        {
            type Output = PageStateValue;
            #[inline(always)]
            fn bitxor(
                self,
                rhs: T,
            ) -> Self::Output {
                let rhs = FlagSeal::conv_page_state(rhs);
                return self.into_value().symmetric_difference(
                    PageStateValue::from_bits_retain(rhs),
                );
            }
        }
        impl PartialEq<PageStateValue> for PageState {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &PageStateValue,
            ) -> bool {
                return self.raw() == rhs.raw();
            }
        }
        impl PartialOrd<PageStateValue> for PageState {
            #[inline(always)]
            fn partial_cmp(
                &self,
                rhs: &PageStateValue,
            ) -> Option<core::cmp::Ordering> {
                return self.raw().partial_cmp(&rhs.raw());
            }
        }
        impl core::ops::Not for PageState {
            type Output = PageStateValue;
            #[inline(always)]
            fn not(self) -> Self::Output {
                return self.into_value().complemented();
            }
        }
        struct IterItems {
            index: usize,
        }
        impl Iterator for IterItems {
            type Item = PageState;
            fn next(&mut self) -> Option<Self::Item> {
                let items = PageState::items();
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
                let max = PageState::items().len();
                return (max, Some(max));
            }
        }
        struct IterFlags {
            value: PageStateValue,
            index: usize,
        }
        impl Iterator for IterFlags {
            type Item = PageState;
            fn next(&mut self) -> Option<Self::Item> {
                let items = PageState::items();
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
                let bound =
                    PageStateValue::raw(self.value).count_ones() as usize;
                return (bound, Some(bound));
            }
        }
        pub struct IterValues {
            value: PageStateValue,
            index: usize,
        }
        impl Iterator for IterValues {
            type Item = PageStateValue;
            fn next(&mut self) -> Option<Self::Item> {
                let items = PageState::items();
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
                    (PageStateValue::raw(self.value).count_ones() + 1) as usize;
                return (bound, Some(bound));
            }
        }
        trait FlagSeal {
            fn conv_page_state(self) -> u8;
        }
        trait FlagBit: FlagSeal {}
        impl FlagSeal for PageState {
            #[inline(always)]
            fn conv_page_state(self) -> u8 {
                return PageState::raw(self);
            }
        }
        impl FlagBit for PageState {}
        impl FlagSeal for PageStateValue {
            #[inline(always)]
            fn conv_page_state(self) -> u8 {
                return PageStateValue::raw(self);
            }
        }
        impl FlagBit for PageStateValue {}
        impl core::str::FromStr for PageState {
            type Err = ();
            #[inline]
            fn from_str(source: &str) -> Result<Self, Self::Err> {
                return Self::from_name(source.trim()).ok_or(());
            }
        }
        impl core::str::FromStr for PageStateValue {
            type Err = ();
            #[inline]
            fn from_str(source: &str) -> Result<Self, Self::Err> {
                let source = source.trim();
                let mut bits: u8 = 0;
                if !source.is_empty() {
                    for part in source.split('|') {
                        let part = part.trim();
                        if part.is_empty() {
                            return Err(());
                        }
                        let value = if let Some(hex) = part.strip_prefix("0x") {
                            if hex.is_empty() {
                                return Err(());
                            }
                            match <u8>::from_str_radix(hex, 16) {
                                Ok(value) => value as u8,
                                Err(_) => {
                                    return Err(());
                                }
                            }
                        }
                        else {
                            match PageState::from_name(part) {
                                Some(flag) => flag.raw(),
                                None => {
                                    return Err(());
                                }
                            }
                        };
                        bits |= value;
                    }
                }
                return Ok(Self(bits));
            }
        }
    };
}
#[allow(unused_imports)]
pub use __page_state::PageState;
#[allow(unused_imports)]
pub use __page_state::PageStateValue;

#[doc(hidden)]
mod __page_header {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct PageHeader(core::num::NonZeroU32);

    #[allow(unused_imports)]
    use super::*;
    #[doc = " The field values of [`PageHeader`], for `make` and `fields`."]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct PageHeaderValues {
        #[doc = " Page number; 0 is reserved."]
        pub id: PageId,
        pub state: PageStateValue,
        #[doc = " 6 bits: `..64` is proven to fit."]
        pub version: Version,
        pub sealed: bool,
        pub data: u8,
    }
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::manual_range_contains)]
    const _: () = {
        trait Seal {
            fn conv_page_header(self) -> u32;
        }
        trait Rel: Seal {}
        impl Seal for PageHeader {
            #[inline(always)]
            fn conv_page_header(self) -> u32 {
                return Self::raw(self);
            }
        }
        impl Rel for PageHeader {}
        impl core::hash::Hash for PageHeader {
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
        impl Eq for PageHeader {}
        if !(size_of::<PageHeader>() == size_of::<u32>()) {
            core::panic!("invalid integral storage size");
        };
        if !(align_of::<PageHeader>() == align_of::<u32>()) {
            core::panic!("invalid integral storage alignment");
        };
        impl<T> PartialEq<T> for PageHeader
        where
            T: Rel + Seal + Copy,
        {
            #[inline(always)]
            fn eq(
                &self,
                rhs: &T,
            ) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_page_header(*rhs);
                return lhs == rhs;
            }
        }
        impl PageHeader {
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> u32 {
                return self.0.get();
            }
            #[inline(always)]
            pub const fn try_make(it: u32) -> Result<Self, u32> {
                return if it != 0 && __typekin_packed_valid(it) {
                    return Ok(Self(match core::num::NonZeroU32::new(it) {
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
            pub(self) fn _or_panic(it: u32) -> Self {
                if it != 0 && __typekin_packed_valid(it) {
                    return Self(match core::num::NonZeroU32::new(it) {
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
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_RAW: (bool, u32) = (false, u32::BITS);
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_FORM: Option<(
                u128,
                &'static [(u128, bool, u128, bool, u128)],
            )> = None;
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_WHY: &'static str = "`PageHeader`'s `valid = __typekin_packed_valid` is opaque to subdomain proofs";
            #[doc(hidden)]
            pub const __TYPEKIN_WITHIN_V1_TABLE: Option<[bool; 256]> = None;
            #[doc(hidden)]
            #[inline(always)]
            pub const fn __typekin_within_v1_raw(self) -> u32 {
                return self.0.get();
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl PageHeader {}
        #[must_use]
        #[inline(always)]
        const fn __typekin_packed_valid(raw: u32) -> bool {
            if raw & 0x100 != 0 {
                return false;
            }
            if <PageId>::try_make(if <PageId>::__TYPEKIN_WITHIN_V1_RAW.0 {
                (((raw >> 24 & 0xff) << 24) as i32 >> 24) as _
            }
            else {
                (raw >> 24 & 0xff) as _
            })
            .is_err()
            {
                return false;
            }
            if <PageStateValue>::try_make(
                if <PageStateValue>::__TYPEKIN_WITHIN_V1_RAW.0 {
                    (((raw >> 16 & 0xff) << 24) as i32 >> 24) as _
                }
                else {
                    (raw >> 16 & 0xff) as _
                },
            )
            .is_err()
            {
                return false;
            }
            if <Version>::try_make(if <Version>::__TYPEKIN_WITHIN_V1_RAW.0 {
                (((raw >> 10 & 0x3f) << 26) as i32 >> 26) as _
            }
            else {
                (raw >> 10 & 0x3f) as _
            })
            .is_err()
            {
                return false;
            }
            return true;
        }
        impl PageHeader {
            #[doc = r" Builds the record from its field values. Cannot fail: each"]
            #[doc = r" value is valid for its type, and each type's values fit its"]
            #[doc = r" bits (proven at compile time)."]
            #[must_use]
            #[inline(always)]
            pub const fn make(values: PageHeaderValues) -> Self {
                let raw: u32 = {
                    let bits =
                        <PageId>::__typekin_within_v1_raw(values.id) as u32;
                    if <PageId>::__TYPEKIN_WITHIN_V1_RAW.0 {
                        bits & 0xff
                    }
                    else {
                        bits
                    }
                } << 24
                    | {
                        let bits = <PageStateValue>::__typekin_within_v1_raw(
                            values.state,
                        ) as u32;
                        if <PageStateValue>::__TYPEKIN_WITHIN_V1_RAW.0 {
                            bits & 0xff
                        }
                        else {
                            bits
                        }
                    } << 16
                    | {
                        let bits =
                            <Version>::__typekin_within_v1_raw(values.version)
                                as u32;
                        if <Version>::__TYPEKIN_WITHIN_V1_RAW.0 {
                            bits & 0x3f
                        }
                        else {
                            bits
                        }
                    } << 10
                    | (values.sealed as u32) << 9
                    | (values.data as u8 as u32) << 0;
                debug_assert!(
                    __typekin_packed_valid(raw),
                    "typekin packed record invariant violated",
                );
                return Self(match core::num::NonZeroU32::new(raw) {
                    Some(value) => value,
                    None => {
                        core::panic!(
                            "zero cannot be stored by this integral type"
                        )
                    }
                });
            }
            #[doc = r" The record's field values."]
            #[must_use]
            #[inline(always)]
            pub const fn fields(self) -> PageHeaderValues {
                return PageHeaderValues {
                    id: Self::id(self),
                    state: Self::state(self),
                    version: Self::version(self),
                    sealed: Self::sealed(self),
                    data: Self::data(self),
                };
            }
            #[doc = " Page number; 0 is reserved."]
            #[must_use]
            #[inline(always)]
            pub const fn id(self) -> PageId {
                let raw: u32 = self.0.get();
                return match <PageId>::try_make(
                    if <PageId>::__TYPEKIN_WITHIN_V1_RAW.0 {
                        (((raw >> 24 & 0xff) << 24) as i32 >> 24) as _
                    }
                    else {
                        (raw >> 24 & 0xff) as _
                    },
                ) {
                    Ok(value) => value,
                    Err(_) => {
                        core::panic!("typekin packed field invariant violated")
                    }
                };
            }
            #[doc = " Page number; 0 is reserved."]
            #[must_use]
            #[inline(always)]
            pub const fn with_id(
                self,
                id: PageId,
            ) -> Self {
                let raw: u32 = self.0.get() & 0xffffff | {
                    let bits = <PageId>::__typekin_within_v1_raw(id) as u32;
                    if <PageId>::__TYPEKIN_WITHIN_V1_RAW.0 {
                        bits & 0xff
                    }
                    else {
                        bits
                    }
                } << 24;
                debug_assert!(
                    __typekin_packed_valid(raw),
                    "typekin packed record invariant violated",
                );
                return Self(match core::num::NonZeroU32::new(raw) {
                    Some(value) => value,
                    None => {
                        core::panic!(
                            "zero cannot be stored by this integral type"
                        )
                    }
                });
            }
            #[must_use]
            #[inline(always)]
            pub const fn state(self) -> PageStateValue {
                let raw: u32 = self.0.get();
                return match <PageStateValue>::try_make(
                    if <PageStateValue>::__TYPEKIN_WITHIN_V1_RAW.0 {
                        (((raw >> 16 & 0xff) << 24) as i32 >> 24) as _
                    }
                    else {
                        (raw >> 16 & 0xff) as _
                    },
                ) {
                    Ok(value) => value,
                    Err(_) => {
                        core::panic!("typekin packed field invariant violated")
                    }
                };
            }
            #[must_use]
            #[inline(always)]
            pub const fn with_state(
                self,
                state: PageStateValue,
            ) -> Self {
                let raw: u32 = self.0.get() & 0xff00ffff | {
                    let bits =
                        <PageStateValue>::__typekin_within_v1_raw(state) as u32;
                    if <PageStateValue>::__TYPEKIN_WITHIN_V1_RAW.0 {
                        bits & 0xff
                    }
                    else {
                        bits
                    }
                } << 16;
                debug_assert!(
                    __typekin_packed_valid(raw),
                    "typekin packed record invariant violated",
                );
                return Self(match core::num::NonZeroU32::new(raw) {
                    Some(value) => value,
                    None => {
                        core::panic!(
                            "zero cannot be stored by this integral type"
                        )
                    }
                });
            }
            #[doc = " 6 bits: `..64` is proven to fit."]
            #[must_use]
            #[inline(always)]
            pub const fn version(self) -> Version {
                let raw: u32 = self.0.get();
                return match <Version>::try_make(
                    if <Version>::__TYPEKIN_WITHIN_V1_RAW.0 {
                        (((raw >> 10 & 0x3f) << 26) as i32 >> 26) as _
                    }
                    else {
                        (raw >> 10 & 0x3f) as _
                    },
                ) {
                    Ok(value) => value,
                    Err(_) => {
                        core::panic!("typekin packed field invariant violated")
                    }
                };
            }
            #[doc = " 6 bits: `..64` is proven to fit."]
            #[must_use]
            #[inline(always)]
            pub const fn with_version(
                self,
                version: Version,
            ) -> Self {
                let raw: u32 = self.0.get() & 0xffff03ff | {
                    let bits =
                        <Version>::__typekin_within_v1_raw(version) as u32;
                    if <Version>::__TYPEKIN_WITHIN_V1_RAW.0 {
                        bits & 0x3f
                    }
                    else {
                        bits
                    }
                } << 10;
                debug_assert!(
                    __typekin_packed_valid(raw),
                    "typekin packed record invariant violated",
                );
                return Self(match core::num::NonZeroU32::new(raw) {
                    Some(value) => value,
                    None => {
                        core::panic!(
                            "zero cannot be stored by this integral type"
                        )
                    }
                });
            }
            #[must_use]
            #[inline(always)]
            pub const fn sealed(self) -> bool {
                let raw: u32 = self.0.get();
                return raw >> 9 & 1 != 0;
            }
            #[must_use]
            #[inline(always)]
            pub const fn with_sealed(
                self,
                sealed: bool,
            ) -> Self {
                let raw: u32 = self.0.get() & 0xfffffdff | (sealed as u32) << 9;
                debug_assert!(
                    __typekin_packed_valid(raw),
                    "typekin packed record invariant violated",
                );
                return Self(match core::num::NonZeroU32::new(raw) {
                    Some(value) => value,
                    None => {
                        core::panic!(
                            "zero cannot be stored by this integral type"
                        )
                    }
                });
            }
            #[must_use]
            #[inline(always)]
            pub const fn data(self) -> u8 {
                let raw: u32 = self.0.get();
                return (raw >> 0) as u8;
            }
            #[must_use]
            #[inline(always)]
            pub const fn with_data(
                self,
                data: u8,
            ) -> Self {
                let raw: u32 =
                    self.0.get() & 0xffffff00 | (data as u8 as u32) << 0;
                debug_assert!(
                    __typekin_packed_valid(raw),
                    "typekin packed record invariant violated",
                );
                return Self(match core::num::NonZeroU32::new(raw) {
                    Some(value) => value,
                    None => {
                        core::panic!(
                            "zero cannot be stored by this integral type"
                        )
                    }
                });
            }
        }
        impl core::fmt::Debug for PageHeader {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                return f
                    .debug_struct("PageHeader")
                    .field("id", &Self::id(*self))
                    .field("state", &Self::state(*self))
                    .field("version", &Self::version(*self))
                    .field("sealed", &Self::sealed(*self))
                    .field("data", &Self::data(*self))
                    .finish();
            }
        }
        pub(crate) type __TypekinWithinValue = (bool, u128);
        pub(crate) type __TypekinWithinPiece = (u128, bool, u128, bool, u128);
        pub(crate) type __TypekinWithinForm<'a> =
            (u128, &'a [__TypekinWithinPiece]);
        pub(crate) enum __TypekinWithin {
            Proven,
            Range(bool, u128),
            Domain(bool, u128),
            Budget(u128),
        }
        pub(crate) const __TYPEKIN_WITHIN_BUDGET: u128 = 65_536;
        pub(crate) const fn __typekin_within_subset(
            source: __TypekinWithinForm,
            target: __TypekinWithinForm,
            min: __TypekinWithinValue,
            max: __TypekinWithinValue,
        ) -> __TypekinWithin {
            if let Some(it) = __typekin_within_outside(source, min, max) {
                return __TypekinWithin::Range(it.0, it.1);
            }
            let (source_modulus, source_pieces) = source;
            let (target_modulus, target_pieces) = target;
            let divisor = __typekin_within_gcd(source_modulus, target_modulus);
            let step =
                match (source_modulus / divisor).checked_mul(target_modulus) {
                    Some(it) => it,
                    None => {
                        return __TypekinWithin::Budget(u128::MAX);
                    }
                };
            let progressions = step / source_modulus;
            let walks =
                match progressions.checked_mul(source_pieces.len() as u128) {
                    Some(it) => it,
                    None => {
                        return __TypekinWithin::Budget(u128::MAX);
                    }
                };
            let steps = match walks.checked_mul(target_pieces.len() as u128 + 1)
            {
                Some(it) => it,
                None => {
                    return __TypekinWithin::Budget(u128::MAX);
                }
            };
            if steps > __TYPEKIN_WITHIN_BUDGET {
                return __TypekinWithin::Budget(steps);
            }
            let mut least = None;
            let mut index = 0;
            while index < source_pieces.len() {
                let (_, lo_negative, lo, hi_negative, hi) =
                    source_pieces[index];
                let mut offset = 0;
                while offset < progressions {
                    if let Some(point) = __typekin_within_walk(
                        (lo_negative, lo),
                        offset * source_modulus,
                        (hi_negative, hi),
                        step,
                        target,
                    ) {
                        least = __typekin_within_least(least, point);
                    }
                    offset += 1;
                }
                index += 1;
            }
            return match least {
                Some(it) => __TypekinWithin::Domain(it.0, it.1),
                None => __TypekinWithin::Proven,
            };
        }
        const fn __typekin_within_outside(
            form: __TypekinWithinForm,
            min: __TypekinWithinValue,
            max: __TypekinWithinValue,
        ) -> Option<__TypekinWithinValue> {
            let (modulus, pieces) = form;
            let mut least = None;
            let mut index = 0;
            while index < pieces.len() {
                let (_, lo_negative, lo, hi_negative, hi) = pieces[index];
                let (lo, hi) = ((lo_negative, lo), (hi_negative, hi));
                if __typekin_within_less(lo, min) {
                    least = __typekin_within_least(least, lo);
                }
                else if __typekin_within_less(max, hi) {
                    let past = match __typekin_within_after(lo, max, modulus) {
                        Some(it) => it,
                        None => hi,
                    };
                    least = __typekin_within_least(least, past);
                }
                index += 1;
            }
            return least;
        }
        const fn __typekin_within_after(
            start: __TypekinWithinValue,
            bound: __TypekinWithinValue,
            step: u128,
        ) -> Option<__TypekinWithinValue> {
            if __typekin_within_less(bound, start) {
                return Some(start);
            }
            return match __typekin_within_gap(start, bound) {
                Some(gap) => __typekin_within_jump(start, gap, step),
                None => None,
            };
        }
        const fn __typekin_within_jump(
            start: __TypekinWithinValue,
            gap: u128,
            step: u128,
        ) -> Option<__TypekinWithinValue> {
            let jumps = match (gap / step).checked_add(1) {
                Some(it) => it,
                None => return None,
            };
            return match jumps.checked_mul(step) {
                Some(jump) => __typekin_within_add(start, jump),
                None => None,
            };
        }
        const fn __typekin_within_least(
            least: Option<__TypekinWithinValue>,
            value: __TypekinWithinValue,
        ) -> Option<__TypekinWithinValue> {
            return match least {
                Some(it) if !__typekin_within_less(value, it) => Some(it),
                _ => Some(value),
            };
        }
        const fn __typekin_within_walk(
            lo: __TypekinWithinValue,
            offset: u128,
            end: __TypekinWithinValue,
            step: u128,
            target: __TypekinWithinForm,
        ) -> Option<__TypekinWithinValue> {
            let mut point = match __typekin_within_add(lo, offset) {
                Some(it) => it,
                None => return None,
            };
            if __typekin_within_less(end, point) {
                return None;
            }
            let (target_modulus, pieces) = target;
            let residue = __typekin_within_residue(point, target_modulus);
            let mut index = 0;
            while index < pieces.len() && pieces[index].0 < residue {
                index += 1;
            }
            loop {
                while index < pieces.len()
                    && pieces[index].0 == residue
                    && __typekin_within_less(
                        (pieces[index].3, pieces[index].4),
                        point,
                    )
                {
                    index += 1;
                }
                if index == pieces.len()
                    || pieces[index].0 != residue
                    || __typekin_within_less(
                        point,
                        (pieces[index].1, pieces[index].2),
                    )
                {
                    return Some(point);
                }
                let gap = match __typekin_within_gap(
                    point,
                    (pieces[index].3, pieces[index].4),
                ) {
                    Some(it) => it,
                    None => {
                        return Some(point);
                    }
                };
                point = match __typekin_within_jump(point, gap, step) {
                    Some(it) => it,
                    None => {
                        return None;
                    }
                };
                if __typekin_within_less(end, point) {
                    return None;
                }
            }
        }
        const fn __typekin_within_less(
            left: __TypekinWithinValue,
            right: __TypekinWithinValue,
        ) -> bool {
            return match (left.0, right.0) {
                (true, false) => true,
                (false, true) => false,
                (false, false) => left.1 < right.1,
                (true, true) => left.1 > right.1,
            };
        }
        const fn __typekin_within_add(
            value: __TypekinWithinValue,
            delta: u128,
        ) -> Option<__TypekinWithinValue> {
            if !value.0 {
                return match value.1.checked_add(delta) {
                    Some(it) => Some((false, it)),
                    None => None,
                };
            }
            if delta >= value.1 {
                return Some((false, delta - value.1));
            }
            return Some((true, value.1 - delta));
        }
        const fn __typekin_within_gap(
            lo: __TypekinWithinValue,
            hi: __TypekinWithinValue,
        ) -> Option<u128> {
            return match (lo.0, hi.0) {
                (false, false) => hi.1.checked_sub(lo.1),
                (true, true) => lo.1.checked_sub(hi.1),
                (true, false) => lo.1.checked_add(hi.1),
                (false, true) => None,
            };
        }
        const fn __typekin_within_residue(
            value: __TypekinWithinValue,
            modulus: u128,
        ) -> u128 {
            let rest = value.1 % modulus;
            if value.0 && rest != 0 {
                return modulus - rest;
            }
            return rest;
        }
        const fn __typekin_within_gcd(
            left: u128,
            right: u128,
        ) -> u128 {
            let (mut left, mut right) = (left, right);
            while right != 0 {
                let rest = left % right;
                left = right;
                right = rest;
            }
            return left;
        }
        pub(crate) struct __TypekinWithinText {
            bytes: [u8; 512],
            len: usize,
        }
        impl __TypekinWithinText {
            pub(crate) const fn new() -> Self {
                return Self {
                    bytes: [0; 512],
                    len: 0,
                };
            }
            pub(crate) const fn push(
                &mut self,
                text: &str,
            ) {
                let bytes = text.as_bytes();
                let mut index = 0;
                while index < bytes.len() {
                    let lead = bytes[index];
                    let width = if lead < 0x80 {
                        1
                    }
                    else if lead >= 0xf0 {
                        4
                    }
                    else if lead >= 0xe0 {
                        3
                    }
                    else {
                        2
                    };
                    if self.len + width > self.bytes.len() {
                        return;
                    }
                    let mut offset = 0;
                    while offset < width {
                        self.bytes[self.len] = bytes[index + offset];
                        self.len += 1;
                        offset += 1;
                    }
                    index += width;
                }
            }
            pub(crate) const fn push_value(
                &mut self,
                value: __TypekinWithinValue,
            ) {
                if value.0 {
                    self.push("-");
                }
                let mut digits = [0u8; 39];
                let mut count = 0;
                let mut rest = value.1;
                loop {
                    digits[count] = b'0' + (rest % 10) as u8;
                    count += 1;
                    rest /= 10;
                    if rest == 0 {
                        break;
                    }
                }
                while count > 0 && self.len < self.bytes.len() {
                    count -= 1;
                    self.bytes[self.len] = digits[count];
                    self.len += 1;
                }
            }
            pub(crate) const fn as_str(&self) -> &str {
                let (text, _) = self.bytes.split_at(self.len);
                return match core::str::from_utf8(text) {
                    Ok(it) => it,
                    Err(_) => "",
                };
            }
        }
        const fn __typekin_within_refute(
            head: &str,
            value: __TypekinWithinValue,
            tail: &str,
        ) -> ! {
            let mut text = __TypekinWithinText::new();
            text.push(head);
            text.push_value(value);
            text.push(tail);
            core::panic!("{}", text.as_str());
        }
        const fn __typekin_within_explain(
            head: &str,
            reason: &str,
        ) -> ! {
            let mut text = __TypekinWithinText::new();
            text.push(head);
            text.push(reason);
            core::panic!("{}", text.as_str());
        }
        const _: () = {
            let (signed, bits): (bool, u32) = <PageId>::__TYPEKIN_WITHIN_V1_RAW;
            if bits < 8 {
                __typekin_within_refute(
                    "`PageId` does not fit bits 24..32 of `PageHeader`: the window is wider than `PageId`'s ",
                    (false, bits as u128),
                    " bits",
                );
            }
            if bits > 8 {
                let (min, max): (__TypekinWithinValue, __TypekinWithinValue) =
                    if signed {
                        ((true, 128u128), (false, 127u128))
                    }
                    else {
                        ((false, 0u128), (false, 255u128))
                    };
                let window: &[__TypekinWithinPiece] =
                    &[(0, min.0, min.1, max.0, max.1)];
                match <PageId>::__TYPEKIN_WITHIN_V1_FORM {
                    Some(source) => {
                        match __typekin_within_subset(
                            source,
                            (1, window),
                            min,
                            max,
                        ) {
                            __TypekinWithin::Proven => {}
                            __TypekinWithin::Range(negative, magnitude)
                            | __TypekinWithin::Domain(negative, magnitude) => {
                                __typekin_within_refute(
                                    "`PageId` does not fit bits 24..32 of `PageHeader`: ",
                                    (negative, magnitude),
                                    " needs more than 8 bits",
                                )
                            }
                            __TypekinWithin::Budget(needed) => {
                                __typekin_within_refute(
                                    "cannot prove `PageId` fits bits 24..32 of `PageHeader`: the check needs ",
                                    (false, needed),
                                    " steps, above the subdomain proof budget of 65536",
                                )
                            }
                        }
                    }
                    None => __typekin_within_explain(
                        "cannot prove `PageId` fits bits 24..32 of `PageHeader`: ",
                        <PageId>::__TYPEKIN_WITHIN_V1_WHY,
                    ),
                }
            }
        };
        const _: () = {
            let (signed, bits): (bool, u32) =
                <PageStateValue>::__TYPEKIN_WITHIN_V1_RAW;
            if bits < 8 {
                __typekin_within_refute(
                    "`PageStateValue` does not fit bits 16..24 of `PageHeader`: the window is wider than `PageStateValue`'s ",
                    (false, bits as u128),
                    " bits",
                );
            }
            if bits > 8 {
                let (min, max): (__TypekinWithinValue, __TypekinWithinValue) =
                    if signed {
                        ((true, 128u128), (false, 127u128))
                    }
                    else {
                        ((false, 0u128), (false, 255u128))
                    };
                let window: &[__TypekinWithinPiece] =
                    &[(0, min.0, min.1, max.0, max.1)];
                match <PageStateValue>::__TYPEKIN_WITHIN_V1_FORM {
                    Some(source) => {
                        match __typekin_within_subset(
                            source,
                            (1, window),
                            min,
                            max,
                        ) {
                            __TypekinWithin::Proven => {}
                            __TypekinWithin::Range(negative, magnitude)
                            | __TypekinWithin::Domain(negative, magnitude) => {
                                __typekin_within_refute(
                                    "`PageStateValue` does not fit bits 16..24 of `PageHeader`: ",
                                    (negative, magnitude),
                                    " needs more than 8 bits",
                                )
                            }
                            __TypekinWithin::Budget(needed) => {
                                __typekin_within_refute(
                                    "cannot prove `PageStateValue` fits bits 16..24 of `PageHeader`: the check needs ",
                                    (false, needed),
                                    " steps, above the subdomain proof budget of 65536",
                                )
                            }
                        }
                    }
                    None => __typekin_within_explain(
                        "cannot prove `PageStateValue` fits bits 16..24 of `PageHeader`: ",
                        <PageStateValue>::__TYPEKIN_WITHIN_V1_WHY,
                    ),
                }
            }
        };
        const _: () = {
            let (signed, bits): (bool, u32) =
                <Version>::__TYPEKIN_WITHIN_V1_RAW;
            if bits < 6 {
                __typekin_within_refute(
                    "`Version` does not fit bits 10..16 of `PageHeader`: the window is wider than `Version`'s ",
                    (false, bits as u128),
                    " bits",
                );
            }
            if bits > 6 {
                let (min, max): (__TypekinWithinValue, __TypekinWithinValue) =
                    if signed {
                        ((true, 32u128), (false, 31u128))
                    }
                    else {
                        ((false, 0u128), (false, 63u128))
                    };
                let window: &[__TypekinWithinPiece] =
                    &[(0, min.0, min.1, max.0, max.1)];
                match <Version>::__TYPEKIN_WITHIN_V1_FORM {
                    Some(source) => {
                        match __typekin_within_subset(
                            source,
                            (1, window),
                            min,
                            max,
                        ) {
                            __TypekinWithin::Proven => {}
                            __TypekinWithin::Range(negative, magnitude)
                            | __TypekinWithin::Domain(negative, magnitude) => {
                                __typekin_within_refute(
                                    "`Version` does not fit bits 10..16 of `PageHeader`: ",
                                    (negative, magnitude),
                                    " needs more than 6 bits",
                                )
                            }
                            __TypekinWithin::Budget(needed) => {
                                __typekin_within_refute(
                                    "cannot prove `Version` fits bits 10..16 of `PageHeader`: the check needs ",
                                    (false, needed),
                                    " steps, above the subdomain proof budget of 65536",
                                )
                            }
                        }
                    }
                    None => match <Version>::__TYPEKIN_WITHIN_V1_TABLE {
                        Some(table) => {
                            let (lo, hi): (i16, i16) = if signed {
                                (-32i16, 31i16)
                            }
                            else {
                                (0, 63i16)
                            };
                            let mut index = 0usize;
                            while index < 256 {
                                let value: i16 = if signed {
                                    index as i16 - 128
                                }
                                else {
                                    index as i16
                                };
                                if table[index] && (value < lo || value > hi) {
                                    __typekin_within_refute(
                                        "`Version` does not fit bits 10..16 of `PageHeader`: ",
                                        (
                                            value < 0,
                                            value.unsigned_abs() as u128,
                                        ),
                                        " needs more than 6 bits",
                                    );
                                }
                                index += 1;
                            }
                        }
                        None => __typekin_within_explain(
                            "cannot prove `Version` fits bits 10..16 of `PageHeader`: ",
                            <Version>::__TYPEKIN_WITHIN_V1_WHY,
                        ),
                    },
                }
            }
        };
        const _: () = {
            if !(false
                || match <PageId>::__TYPEKIN_WITHIN_V1_FORM {
                    Some(form) => matches!(
                        __typekin_within_subset(
                            (1, &[(0, false, 0, false, 0)]),
                            form,
                            (false, 0),
                            (false, 0),
                        ),
                        __TypekinWithin::Domain(..)
                    ),
                    None => match <PageId>::__TYPEKIN_WITHIN_V1_TABLE {
                        Some(table) => {
                            !table[if <PageId>::__TYPEKIN_WITHIN_V1_RAW.0 {
                                128
                            }
                            else {
                                0
                            }]
                        }
                        None => false,
                    },
                }
                || match <PageStateValue>::__TYPEKIN_WITHIN_V1_FORM {
                    Some(form) => matches!(
                        __typekin_within_subset(
                            (1, &[(0, false, 0, false, 0)]),
                            form,
                            (false, 0),
                            (false, 0),
                        ),
                        __TypekinWithin::Domain(..)
                    ),
                    None => {
                        match <PageStateValue>::__TYPEKIN_WITHIN_V1_TABLE {
                            Some(table) => !table
                                [if <PageStateValue>::__TYPEKIN_WITHIN_V1_RAW.0
                                {
                                    128
                                }
                                else {
                                    0
                                }],
                            None => false,
                        }
                    }
                }
                || match <Version>::__TYPEKIN_WITHIN_V1_FORM {
                    Some(form) => matches!(
                        __typekin_within_subset(
                            (1, &[(0, false, 0, false, 0)]),
                            form,
                            (false, 0),
                            (false, 0),
                        ),
                        __TypekinWithin::Domain(..)
                    ),
                    None => match <Version>::__TYPEKIN_WITHIN_V1_TABLE {
                        Some(table) => {
                            !table[if <Version>::__TYPEKIN_WITHIN_V1_RAW.0 {
                                128
                            }
                            else {
                                0
                            }]
                        }
                        None => false,
                    },
                })
            {
                core::panic!(
                    "cannot prove `non_zero` for `PageHeader`: every field admits zero"
                );
            }
        };
    };
}
#[allow(unused_imports)]
pub use __page_header::PageHeader;
#[allow(unused_imports)]
pub use __page_header::PageHeaderValues;

fn main() {
    let id = PageId::try_make(5).unwrap();
    let version = Version::try_make(42).unwrap();

    // Infallible, no validation.
    let header = PageHeader::make(PageHeaderValues {
        id,
        state: PageState::Dirty.into_value(),
        version,
        sealed: false,
        data: 0xFF,
    });
    let header = header.with_state(PageState::Pinned.into_value());
    let PageHeaderValues { data, .. } = header.fields();

    println!("{header:?} = {:#010x}", header.raw());
    println!("id {} data {data:#x}", header.id().raw());

    // Raw ingress validates: `id` bits 0, then unclaimed bit 8 set.
    assert!(PageHeader::try_make(0x0000_00FF).is_err());
    assert!(PageHeader::try_make(0x0100_0100).is_err());

    // `NonZero` storage, proven through `id`.
    assert_eq!(size_of::<Option<PageHeader>>(), 4);
}
