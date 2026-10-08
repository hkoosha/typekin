#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct MyNonConstU32(u32);

#[allow(dead_code)]
#[allow(unused_qualifications)]
#[allow(clippy::unnecessary_cast)]
const _: () = {
    trait Seal {
        fn conv_my_non_const_u32(self) -> u32;
    }
    trait Make: Seal {
        fn make(self) -> MyNonConstU32;
    }
    trait Math: Seal {}
    trait Bit: Seal {}
    trait Rel: Seal {}
    trait Trust: Seal {}
    impl Seal for MyNonConstU32 {
        #[inline(always)]
        fn conv_my_non_const_u32(self) -> u32 {
            return Self::raw(self);
        }
    }
    impl Bit for MyNonConstU32 {}
    impl Make for MyNonConstU32 {
        #[inline(always)]
        fn make(self) -> MyNonConstU32 {
            let raw = <Self as Seal>::conv_my_non_const_u32(self);
            return MyNonConstU32::_unchecked(raw);
        }
    }
    impl Math for MyNonConstU32 {}
    impl Rel for MyNonConstU32 {}
    impl Seal for i16 {
        #[inline(always)]
        fn conv_my_non_const_u32(self) -> u32 {
            return self as u32;
        }
    }
    impl Make for i16 {
        #[inline(always)]
        fn make(self) -> MyNonConstU32 {
            let raw = <Self as Seal>::conv_my_non_const_u32(self);
            return MyNonConstU32::_unchecked(raw);
        }
    }
    impl Seal for u16 {
        #[inline(always)]
        fn conv_my_non_const_u32(self) -> u32 {
            return self as u32;
        }
    }
    impl Make for u16 {
        #[inline(always)]
        fn make(self) -> MyNonConstU32 {
            let raw = <Self as Seal>::conv_my_non_const_u32(self);
            return MyNonConstU32::_unchecked(raw);
        }
    }
    impl Seal for u32 {
        #[inline(always)]
        fn conv_my_non_const_u32(self) -> u32 {
            return self as u32;
        }
    }
    impl Bit for u32 {}
    impl Make for u32 {
        #[inline(always)]
        fn make(self) -> MyNonConstU32 {
            let raw = <Self as Seal>::conv_my_non_const_u32(self);
            return MyNonConstU32::_unchecked(raw);
        }
    }
    impl Math for u32 {}
    impl Rel for u32 {}
    impl Seal for u8 {
        #[inline(always)]
        fn conv_my_non_const_u32(self) -> u32 {
            return self as u32;
        }
    }
    impl Make for u8 {
        #[inline(always)]
        fn make(self) -> MyNonConstU32 {
            let raw = <Self as Seal>::conv_my_non_const_u32(self);
            return MyNonConstU32::_unchecked(raw);
        }
    }
    impl ::core::convert::AsRef<u32> for MyNonConstU32 {
        #[inline(always)]
        fn as_ref(&self) -> &u32 {
            return &self.0;
        }
    }
    impl ::core::str::FromStr for MyNonConstU32 {
        type Err = ();
        #[inline(always)]
        fn from_str(source: &str) -> ::core::result::Result<Self, Self::Err> {
            let value = match <u32 as ::core::str::FromStr>::from_str(source) {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(_) => {
                    return ::core::result::Result::Err(());
                }
            };
            return if true {
                ::core::result::Result::Ok(Self(value))
            }
            else {
                ::core::result::Result::Err(())
            };
        }
    }
    impl MyNonConstU32 {
        #[doc = r" The number of bits in the wrapped primitive integer."]
        pub const BITS: u32 = u32::BITS;
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
        pub const fn highest_one(self) -> ::core::option::Option<u32> {
            return Self::raw(self).highest_one();
        }
        #[inline(always)]
        pub const fn lowest_one(self) -> ::core::option::Option<u32> {
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
        ) -> ::core::option::Option<u32> {
            return Self::raw(self).checked_ilog(Self::raw(base));
        }
        #[inline(always)]
        pub const fn checked_ilog2(self) -> ::core::option::Option<u32> {
            return Self::raw(self).checked_ilog2();
        }
        #[inline(always)]
        pub const fn checked_ilog10(self) -> ::core::option::Option<u32> {
            return Self::raw(self).checked_ilog10();
        }
        #[inline(always)]
        pub const fn to_be_bytes(self) -> [u8; ::core::mem::size_of::<u32>()] {
            return Self::raw(self).to_be_bytes();
        }
        #[inline(always)]
        pub const fn to_le_bytes(self) -> [u8; ::core::mem::size_of::<u32>()] {
            return Self::raw(self).to_le_bytes();
        }
        #[inline(always)]
        pub const fn to_ne_bytes(self) -> [u8; ::core::mem::size_of::<u32>()] {
            return Self::raw(self).to_ne_bytes();
        }
    }
    impl MyNonConstU32 {
        #[inline(always)]
        pub fn rotate_left(
            self,
            n: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).rotate_left(n));
        }
        #[inline(always)]
        pub fn rotate_right(
            self,
            n: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).rotate_right(n));
        }
        #[inline(always)]
        pub fn swap_bytes(self) -> Self {
            return Self::_core_int(Self::raw(self).swap_bytes());
        }
        #[inline(always)]
        pub fn reverse_bits(self) -> Self {
            return Self::_core_int(Self::raw(self).reverse_bits());
        }
        #[inline(always)]
        pub fn isolate_highest_one(self) -> Self {
            return Self::_core_int(Self::raw(self).isolate_highest_one());
        }
        #[inline(always)]
        pub fn isolate_lowest_one(self) -> Self {
            return Self::_core_int(Self::raw(self).isolate_lowest_one());
        }
        #[inline(always)]
        pub fn to_be(self) -> Self {
            return Self::_core_int(Self::raw(self).to_be());
        }
        #[inline(always)]
        pub fn to_le(self) -> Self {
            return Self::_core_int(Self::raw(self).to_le());
        }
        #[inline(always)]
        pub fn from_be(value: Self) -> Self {
            return Self::_core_int(u32::from_be(Self::raw(value)));
        }
        #[inline(always)]
        pub fn from_le(value: Self) -> Self {
            return Self::_core_int(u32::from_le(Self::raw(value)));
        }
        #[inline(always)]
        pub fn from_ne_bytes(
            bytes: [u8; ::core::mem::size_of::<u32>()]
        ) -> Self {
            return Self::_core_int(u32::from_ne_bytes(bytes));
        }
        #[inline(always)]
        pub fn from_be_bytes(
            bytes: [u8; ::core::mem::size_of::<u32>()]
        ) -> Self {
            return Self::_core_int(u32::from_be_bytes(bytes));
        }
        #[inline(always)]
        pub fn from_le_bytes(
            bytes: [u8; ::core::mem::size_of::<u32>()]
        ) -> Self {
            return Self::_core_int(u32::from_le_bytes(bytes));
        }
        #[inline(always)]
        pub fn checked_add(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_add(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_sub(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_sub(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_neg(self) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_neg() {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_mul(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_mul(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_div(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_div(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_rem(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_rem(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_div_euclid(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_div_euclid(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_rem_euclid(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_rem_euclid(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_shl(
            self,
            rhs: u32,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_shl(rhs) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_shr(
            self,
            rhs: u32,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_shr(rhs) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_pow(
            self,
            exp: u32,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_pow(exp) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
    }
    impl MyNonConstU32 {
        #[inline(always)]
        pub fn midpoint(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).midpoint(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn unbounded_shl(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).unbounded_shl(rhs));
        }
        #[inline(always)]
        pub fn unbounded_shr(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).unbounded_shr(rhs));
        }
        #[inline(always)]
        pub fn saturating_add(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_add(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_sub(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_sub(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_mul(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_mul(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_div(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_div(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).saturating_pow(exp));
        }
        #[inline(always)]
        pub fn wrapping_add(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_add(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_sub(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_sub(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_neg(self) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_neg());
        }
        #[inline(always)]
        pub fn wrapping_mul(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_mul(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_div(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_div(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_rem(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_rem(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_div_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_div_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_rem_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_rem_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_shl(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_shl(rhs));
        }
        #[inline(always)]
        pub fn wrapping_shr(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_shr(rhs));
        }
        #[inline(always)]
        pub fn wrapping_pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_pow(exp));
        }
        #[inline(always)]
        pub fn pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).pow(exp));
        }
        #[inline(always)]
        pub fn isqrt(self) -> Self {
            return Self::_core_int(Self::raw(self).isqrt());
        }
        #[inline(always)]
        pub fn div_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).div_euclid(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn rem_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).rem_euclid(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_add(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_add(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_sub(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_sub(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_neg(self) -> Self {
            return Self::_core_int(Self::raw(self).strict_neg());
        }
        #[inline(always)]
        pub fn strict_mul(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_mul(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_div(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_div(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_rem(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_rem(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_div_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).strict_div_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn strict_rem_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).strict_rem_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn strict_shl(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_shl(rhs));
        }
        #[inline(always)]
        pub fn strict_shr(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_shr(rhs));
        }
        #[inline(always)]
        pub fn strict_pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_pow(exp));
        }
    }
    impl MyNonConstU32 {
        #[inline(always)]
        pub fn overflowing_add(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_add(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_sub(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_sub(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_neg(self) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_neg();
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_mul(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_mul(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_div(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_div(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_rem(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_rem(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_div_euclid(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_div_euclid(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_rem_euclid(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_rem_euclid(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_shl(
            self,
            rhs: u32,
        ) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_shl(rhs);
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_shr(
            self,
            rhs: u32,
        ) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_shr(rhs);
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_pow(
            self,
            exp: u32,
        ) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_pow(exp);
            return (Self::_core_int(value), overflowed);
        }
    }
    impl MyNonConstU32 {
        #[inline(always)]
        pub fn cast_signed(self) -> i32 {
            return Self::raw(self).cast_signed();
        }
        #[inline(always)]
        pub fn bit_width(self) -> u32 {
            return Self::raw(self).bit_width();
        }
        #[inline(always)]
        pub fn funnel_shl(
            self,
            right: Self,
            n: u32,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).funnel_shl(Self::raw(right), n),
            );
        }
        #[inline(always)]
        pub fn funnel_shr(
            self,
            right: Self,
            n: u32,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).funnel_shr(Self::raw(right), n),
            );
        }
        #[inline(always)]
        pub fn abs_diff(
            self,
            rhs: Self,
        ) -> u32 {
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
        pub fn next_power_of_two(self) -> Self {
            return Self::_core_int(Self::raw(self).next_power_of_two());
        }
        #[inline(always)]
        pub fn checked_next_power_of_two(self) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_next_power_of_two() {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn div_ceil(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).div_ceil(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn next_multiple_of(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).next_multiple_of(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn checked_next_multiple_of(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self)
                .checked_next_multiple_of(Self::raw(rhs))
            {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
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
            return (Self::_core_int(value), carry);
        }
        #[inline(always)]
        pub fn borrowing_sub(
            self,
            rhs: Self,
            borrow: bool,
        ) -> (Self, bool) {
            let (value, borrow) =
                Self::raw(self).borrowing_sub(Self::raw(rhs), borrow);
            return (Self::_core_int(value), borrow);
        }
        #[inline(always)]
        pub fn carrying_mul(
            self,
            rhs: Self,
            carry: Self,
        ) -> (Self, Self) {
            let (low, high) =
                Self::raw(self).carrying_mul(Self::raw(rhs), Self::raw(carry));
            return (Self::_core_int(low), Self::_core_int(high));
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
            return (Self::_core_int(low), Self::_core_int(high));
        }
    }
    impl ::core::convert::Into<usize> for MyNonConstU32 {
        #[inline(always)]
        fn into(self) -> usize {
            return MyNonConstU32::into_usize(self);
        }
    }
    impl ::core::convert::Into<u32> for MyNonConstU32 {
        #[inline(always)]
        fn into(self) -> u32 {
            return MyNonConstU32::into_u32(self);
        }
    }
    impl ::core::convert::Into<u64> for MyNonConstU32 {
        #[inline(always)]
        fn into(self) -> u64 {
            return MyNonConstU32::into_u64(self);
        }
    }
    impl ::core::convert::Into<u128> for MyNonConstU32 {
        #[inline(always)]
        fn into(self) -> u128 {
            return MyNonConstU32::into_u128(self);
        }
    }
    impl ::core::convert::Into<i64> for MyNonConstU32 {
        #[inline(always)]
        fn into(self) -> i64 {
            return MyNonConstU32::into_i64(self);
        }
    }
    impl ::core::convert::Into<i128> for MyNonConstU32 {
        #[inline(always)]
        fn into(self) -> i128 {
            return MyNonConstU32::into_i128(self);
        }
    }
    impl ::core::convert::TryInto<isize> for MyNonConstU32 {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<isize, Self::Error> {
            return MyNonConstU32::try_into_isize(self);
        }
    }
    impl ::core::convert::TryInto<u8> for MyNonConstU32 {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u8, Self::Error> {
            return MyNonConstU32::try_into_u8(self);
        }
    }
    impl ::core::convert::TryInto<u16> for MyNonConstU32 {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<u16, Self::Error> {
            return MyNonConstU32::try_into_u16(self);
        }
    }
    impl ::core::convert::TryInto<i8> for MyNonConstU32 {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i8, Self::Error> {
            return MyNonConstU32::try_into_i8(self);
        }
    }
    impl ::core::convert::TryInto<i16> for MyNonConstU32 {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i16, Self::Error> {
            return MyNonConstU32::try_into_i16(self);
        }
    }
    impl ::core::convert::TryInto<i32> for MyNonConstU32 {
        type Error = ();
        #[inline(always)]
        fn try_into(self) -> ::core::result::Result<i32, Self::Error> {
            return MyNonConstU32::try_into_i32(self);
        }
    }
    impl ::core::ops::Shr<usize> for MyNonConstU32 {
        type Output = Self;
        #[inline(always)]
        fn shr(
            self,
            rhs: usize,
        ) -> Self::Output {
            return self._shr(rhs);
        }
    }
    impl ::core::ops::Shl<usize> for MyNonConstU32 {
        type Output = Self;
        #[inline(always)]
        fn shl(
            self,
            rhs: usize,
        ) -> Self::Output {
            return self._shl(rhs);
        }
    }
    impl ::core::ops::ShrAssign<usize> for MyNonConstU32 {
        #[inline(always)]
        fn shr_assign(
            &mut self,
            other: usize,
        ) {
            *self = self._shr(other);
        }
    }
    impl ::core::ops::ShlAssign<usize> for MyNonConstU32 {
        #[inline(always)]
        fn shl_assign(
            &mut self,
            other: usize,
        ) {
            *self = self._shl(other);
        }
    }
    impl<T> ::core::ops::BitAndAssign<T> for MyNonConstU32
    where
        T: Bit + Seal,
    {
        #[inline(always)]
        fn bitand_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_my_non_const_u32(rhs);
            *self = self._bitand(it);
        }
    }
    impl<T> ::core::ops::AddAssign<T> for MyNonConstU32
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn add_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_my_non_const_u32(rhs);
            *self = self._add(it);
        }
    }
    impl<T> ::core::ops::SubAssign<T> for MyNonConstU32
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn sub_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_my_non_const_u32(rhs);
            *self = self._sub(it);
        }
    }
    impl<T> ::core::ops::MulAssign<T> for MyNonConstU32
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn mul_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_my_non_const_u32(rhs);
            *self = self._mul(it);
        }
    }
    impl<T> ::core::ops::DivAssign<T> for MyNonConstU32
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn div_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_my_non_const_u32(rhs);
            *self = self._div(it);
        }
    }
    impl<T> ::core::ops::RemAssign<T> for MyNonConstU32
    where
        T: Math + Seal,
    {
        #[inline(always)]
        fn rem_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_my_non_const_u32(rhs);
            *self = self._rem(it);
        }
    }
    impl<T> ::core::ops::BitOrAssign<T> for MyNonConstU32
    where
        T: Bit + Seal,
    {
        #[inline(always)]
        fn bitor_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_my_non_const_u32(rhs);
            *self = self._bitor(it);
        }
    }
    impl ::core::fmt::Debug for MyNonConstU32 {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            ::core::write!(f, "MyNonConstU32({})", self.0)
        }
    }
    impl ::core::cmp::Eq for MyNonConstU32 {}
    impl ::core::cmp::Ord for MyNonConstU32 {
        #[inline(always)]
        fn cmp(
            &self,
            other: &Self,
        ) -> ::core::cmp::Ordering {
            return self.partial_cmp(other).unwrap();
        }
    }
    if !(::core::mem::size_of::<MyNonConstU32>()
        == ::core::mem::size_of::<u32>())
    {
        panic!("invalid memory layout, mismatching sizes: #ty(#el) != #el");
    };
    if !(::core::mem::align_of::<MyNonConstU32>()
        == ::core::mem::align_of::<u32>())
    {
        panic!("invalid memory layout, mismatching alignment: #ty(#el) != #el");
    };
    impl<T> ::core::ops::Add<T> for MyNonConstU32
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn add(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_my_non_const_u32(rhs);
            return self._add(it);
        }
    }
    impl<T> ::core::ops::Sub<T> for MyNonConstU32
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn sub(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_my_non_const_u32(rhs);
            return self._sub(it);
        }
    }
    impl<T> ::core::ops::Mul<T> for MyNonConstU32
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn mul(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_my_non_const_u32(rhs);
            return self._mul(it);
        }
    }
    impl<T> ::core::ops::Div<T> for MyNonConstU32
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn div(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_my_non_const_u32(rhs);
            return self._div(it);
        }
    }
    impl<T> ::core::ops::Rem<T> for MyNonConstU32
    where
        T: Math + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn rem(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_my_non_const_u32(rhs);
            return self._rem(it);
        }
    }
    impl<T> ::core::ops::BitAnd<T> for MyNonConstU32
    where
        T: Bit + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn bitand(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_my_non_const_u32(rhs);
            return self._bitand(it);
        }
    }
    impl<T> ::core::ops::BitOr<T> for MyNonConstU32
    where
        T: Bit + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn bitor(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_my_non_const_u32(rhs);
            return self._bitor(it);
        }
    }
    impl<T> ::core::ops::BitXor<T> for MyNonConstU32
    where
        T: Bit + Seal,
    {
        type Output = Self;
        #[inline(always)]
        fn bitxor(
            self,
            rhs: T,
        ) -> Self::Output {
            let it = Seal::conv_my_non_const_u32(rhs);
            return self._bitxor(it);
        }
    }
    impl<T> ::core::ops::BitXorAssign<T> for MyNonConstU32
    where
        T: Bit + Seal,
    {
        #[inline(always)]
        fn bitxor_assign(
            &mut self,
            rhs: T,
        ) {
            let it = Seal::conv_my_non_const_u32(rhs);
            *self = self._bitxor(it);
        }
    }
    impl ::core::ops::Not for MyNonConstU32 {
        type Output = Self;
        #[inline(always)]
        fn not(self) -> Self::Output {
            return self._not();
        }
    }
    impl ::core::fmt::Binary for MyNonConstU32 {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::Binary::fmt(&raw, f);
        }
    }
    impl ::core::fmt::Octal for MyNonConstU32 {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::Octal::fmt(&raw, f);
        }
    }
    impl ::core::fmt::LowerHex for MyNonConstU32 {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::LowerHex::fmt(&raw, f);
        }
    }
    impl ::core::fmt::UpperHex for MyNonConstU32 {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            let raw = Self::raw(*self);
            return ::core::fmt::UpperHex::fmt(&raw, f);
        }
    }
    impl<T> ::core::cmp::PartialEq<T> for MyNonConstU32
    where
        T: Rel + Seal + ::core::marker::Copy,
    {
        #[inline(always)]
        fn eq(
            &self,
            rhs: &T,
        ) -> bool {
            let lhs = Self::raw(*self);
            let rhs = Seal::conv_my_non_const_u32(*rhs);
            return lhs == rhs;
        }
    }
    impl<T> ::core::cmp::PartialOrd<T> for MyNonConstU32
    where
        T: Rel + Seal + ::core::marker::Copy,
    {
        #[inline(always)]
        fn partial_cmp(
            &self,
            rhs: &T,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let lhs = Self::raw(*self);
            let rhs = Seal::conv_my_non_const_u32(*rhs);
            return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
        }
    }
    impl MyNonConstU32 {
        #[inline(always)]
        #[allow(private_bounds)]
        pub fn of<T>(it: T) -> MyNonConstU32
        where
            T: Make,
        {
            return <T as Make>::make(it);
        }
        #[inline(always)]
        pub fn make(it: u32) -> MyNonConstU32 {
            return MyNonConstU32::of(it);
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
        pub const fn try_into_usize(self) -> ::core::result::Result<usize, ()> {
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
        pub const fn try_into_u128(self) -> ::core::result::Result<u128, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as u128);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i64(self) -> ::core::result::Result<i64, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as i64);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i128(self) -> ::core::result::Result<i128, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as i128);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_isize(self) -> ::core::result::Result<isize, ()> {
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
            return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs).unwrap();
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
        #[inline(always)]
        #[track_caller]
        pub(self) fn _core_int(value: u32) -> Self {
            if true {
                return Self(value);
            }
            else {
                ::core::panic!("integral operation produced an invalid value");
            }
        }
        #[inline(always)]
        pub(self) fn _core_int_checked(
            value: u32
        ) -> ::core::option::Option<Self> {
            return if true {
                ::core::option::Option::Some(Self(value))
            }
            else {
                ::core::option::Option::None
            };
        }
    }
    #[allow(clippy::unnecessary_cast)]
    impl MyNonConstU32 {
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
            return ((Self::raw(self) >> (8 * 0usize)) & (0xFF as u32)) as u8;
        }
        #[must_use]
        #[inline(always)]
        pub const fn byte1(self) -> u8 {
            return ((Self::raw(self) >> (8 * 1usize)) & (0xFF as u32)) as u8;
        }
        #[must_use]
        #[inline(always)]
        pub const fn byte2(self) -> u8 {
            return ((Self::raw(self) >> (8 * 2usize)) & (0xFF as u32)) as u8;
        }
        #[must_use]
        #[inline(always)]
        pub const fn byte3(self) -> u8 {
            return ((Self::raw(self) >> (8 * 3usize)) & (0xFF as u32)) as u8;
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
            return ((Self::raw(self) >> (32 * 0usize)) & (0xFFFFFFFF as u32))
                as u32;
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
    trait Rel: Seal {}
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
    impl Rel for Limited {}
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
    impl Rel for u32 {}
    impl ::core::convert::AsRef<u32> for Limited {
        #[inline(always)]
        fn as_ref(&self) -> &u32 {
            return &self.0;
        }
    }
    impl ::core::str::FromStr for Limited {
        type Err = ();
        #[inline(always)]
        fn from_str(source: &str) -> ::core::result::Result<Self, Self::Err> {
            let value = match <u32 as ::core::str::FromStr>::from_str(source) {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(_) => {
                    return ::core::result::Result::Err(());
                }
            };
            return if (value >= (1)) && (value <= (100)) {
                ::core::result::Result::Ok(Self(value))
            }
            else {
                ::core::result::Result::Err(())
            };
        }
    }
    impl Limited {
        #[doc = r" The number of bits in the wrapped primitive integer."]
        pub const BITS: u32 = u32::BITS;
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
        pub const fn highest_one(self) -> ::core::option::Option<u32> {
            return Self::raw(self).highest_one();
        }
        #[inline(always)]
        pub const fn lowest_one(self) -> ::core::option::Option<u32> {
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
        ) -> ::core::option::Option<u32> {
            return Self::raw(self).checked_ilog(Self::raw(base));
        }
        #[inline(always)]
        pub const fn checked_ilog2(self) -> ::core::option::Option<u32> {
            return Self::raw(self).checked_ilog2();
        }
        #[inline(always)]
        pub const fn checked_ilog10(self) -> ::core::option::Option<u32> {
            return Self::raw(self).checked_ilog10();
        }
        #[inline(always)]
        pub const fn to_be_bytes(self) -> [u8; ::core::mem::size_of::<u32>()] {
            return Self::raw(self).to_be_bytes();
        }
        #[inline(always)]
        pub const fn to_le_bytes(self) -> [u8; ::core::mem::size_of::<u32>()] {
            return Self::raw(self).to_le_bytes();
        }
        #[inline(always)]
        pub const fn to_ne_bytes(self) -> [u8; ::core::mem::size_of::<u32>()] {
            return Self::raw(self).to_ne_bytes();
        }
    }
    impl Limited {
        #[inline(always)]
        pub fn rotate_left(
            self,
            n: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).rotate_left(n));
        }
        #[inline(always)]
        pub fn rotate_right(
            self,
            n: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).rotate_right(n));
        }
        #[inline(always)]
        pub fn swap_bytes(self) -> Self {
            return Self::_core_int(Self::raw(self).swap_bytes());
        }
        #[inline(always)]
        pub fn reverse_bits(self) -> Self {
            return Self::_core_int(Self::raw(self).reverse_bits());
        }
        #[inline(always)]
        pub fn isolate_highest_one(self) -> Self {
            return Self::_core_int(Self::raw(self).isolate_highest_one());
        }
        #[inline(always)]
        pub fn isolate_lowest_one(self) -> Self {
            return Self::_core_int(Self::raw(self).isolate_lowest_one());
        }
        #[inline(always)]
        pub fn to_be(self) -> Self {
            return Self::_core_int(Self::raw(self).to_be());
        }
        #[inline(always)]
        pub fn to_le(self) -> Self {
            return Self::_core_int(Self::raw(self).to_le());
        }
        #[inline(always)]
        pub fn from_be(value: Self) -> Self {
            return Self::_core_int(u32::from_be(Self::raw(value)));
        }
        #[inline(always)]
        pub fn from_le(value: Self) -> Self {
            return Self::_core_int(u32::from_le(Self::raw(value)));
        }
        #[inline(always)]
        pub fn from_ne_bytes(
            bytes: [u8; ::core::mem::size_of::<u32>()]
        ) -> Self {
            return Self::_core_int(u32::from_ne_bytes(bytes));
        }
        #[inline(always)]
        pub fn from_be_bytes(
            bytes: [u8; ::core::mem::size_of::<u32>()]
        ) -> Self {
            return Self::_core_int(u32::from_be_bytes(bytes));
        }
        #[inline(always)]
        pub fn from_le_bytes(
            bytes: [u8; ::core::mem::size_of::<u32>()]
        ) -> Self {
            return Self::_core_int(u32::from_le_bytes(bytes));
        }
        #[inline(always)]
        pub fn checked_add(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_add(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_sub(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_sub(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_neg(self) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_neg() {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_mul(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_mul(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_div(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_div(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_rem(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_rem(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_div_euclid(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_div_euclid(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_rem_euclid(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_rem_euclid(Self::raw(rhs)) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_shl(
            self,
            rhs: u32,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_shl(rhs) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_shr(
            self,
            rhs: u32,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_shr(rhs) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn checked_pow(
            self,
            exp: u32,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_pow(exp) {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
    }
    impl Limited {
        #[inline(always)]
        pub fn midpoint(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).midpoint(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn unbounded_shl(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).unbounded_shl(rhs));
        }
        #[inline(always)]
        pub fn unbounded_shr(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).unbounded_shr(rhs));
        }
        #[inline(always)]
        pub fn saturating_add(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_add(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_sub(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_sub(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_mul(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_mul(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_div(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).saturating_div(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn saturating_pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).saturating_pow(exp));
        }
        #[inline(always)]
        pub fn wrapping_add(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_add(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_sub(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_sub(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_neg(self) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_neg());
        }
        #[inline(always)]
        pub fn wrapping_mul(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_mul(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_div(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_div(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_rem(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_rem(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_div_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_div_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_rem_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).wrapping_rem_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn wrapping_shl(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_shl(rhs));
        }
        #[inline(always)]
        pub fn wrapping_shr(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_shr(rhs));
        }
        #[inline(always)]
        pub fn wrapping_pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).wrapping_pow(exp));
        }
        #[inline(always)]
        pub fn pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).pow(exp));
        }
        #[inline(always)]
        pub fn isqrt(self) -> Self {
            return Self::_core_int(Self::raw(self).isqrt());
        }
        #[inline(always)]
        pub fn div_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).div_euclid(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn rem_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).rem_euclid(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_add(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_add(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_sub(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_sub(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_neg(self) -> Self {
            return Self::_core_int(Self::raw(self).strict_neg());
        }
        #[inline(always)]
        pub fn strict_mul(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_mul(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_div(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_div(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_rem(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_rem(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn strict_div_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).strict_div_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn strict_rem_euclid(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).strict_rem_euclid(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn strict_shl(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_shl(rhs));
        }
        #[inline(always)]
        pub fn strict_shr(
            self,
            rhs: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_shr(rhs));
        }
        #[inline(always)]
        pub fn strict_pow(
            self,
            exp: u32,
        ) -> Self {
            return Self::_core_int(Self::raw(self).strict_pow(exp));
        }
    }
    impl Limited {
        #[inline(always)]
        pub fn overflowing_add(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_add(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_sub(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_sub(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_neg(self) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_neg();
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_mul(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_mul(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_div(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_div(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_rem(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_rem(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_div_euclid(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_div_euclid(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_rem_euclid(
            self,
            rhs: Self,
        ) -> (Self, bool) {
            let (value, overflowed) =
                Self::raw(self).overflowing_rem_euclid(Self::raw(rhs));
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_shl(
            self,
            rhs: u32,
        ) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_shl(rhs);
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_shr(
            self,
            rhs: u32,
        ) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_shr(rhs);
            return (Self::_core_int(value), overflowed);
        }
        #[inline(always)]
        pub fn overflowing_pow(
            self,
            exp: u32,
        ) -> (Self, bool) {
            let (value, overflowed) = Self::raw(self).overflowing_pow(exp);
            return (Self::_core_int(value), overflowed);
        }
    }
    impl Limited {
        #[inline(always)]
        pub fn cast_signed(self) -> i32 {
            return Self::raw(self).cast_signed();
        }
        #[inline(always)]
        pub fn bit_width(self) -> u32 {
            return Self::raw(self).bit_width();
        }
        #[inline(always)]
        pub fn funnel_shl(
            self,
            right: Self,
            n: u32,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).funnel_shl(Self::raw(right), n),
            );
        }
        #[inline(always)]
        pub fn funnel_shr(
            self,
            right: Self,
            n: u32,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).funnel_shr(Self::raw(right), n),
            );
        }
        #[inline(always)]
        pub fn abs_diff(
            self,
            rhs: Self,
        ) -> u32 {
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
        pub fn next_power_of_two(self) -> Self {
            return Self::_core_int(Self::raw(self).next_power_of_two());
        }
        #[inline(always)]
        pub fn checked_next_power_of_two(self) -> ::core::option::Option<Self> {
            return match Self::raw(self).checked_next_power_of_two() {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
            };
        }
        #[inline(always)]
        pub fn div_ceil(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(Self::raw(self).div_ceil(Self::raw(rhs)));
        }
        #[inline(always)]
        pub fn next_multiple_of(
            self,
            rhs: Self,
        ) -> Self {
            return Self::_core_int(
                Self::raw(self).next_multiple_of(Self::raw(rhs)),
            );
        }
        #[inline(always)]
        pub fn checked_next_multiple_of(
            self,
            rhs: Self,
        ) -> ::core::option::Option<Self> {
            return match Self::raw(self)
                .checked_next_multiple_of(Self::raw(rhs))
            {
                ::core::option::Option::Some(value) => {
                    Self::_core_int_checked(value)
                }
                ::core::option::Option::None => ::core::option::Option::None,
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
            return (Self::_core_int(value), carry);
        }
        #[inline(always)]
        pub fn borrowing_sub(
            self,
            rhs: Self,
            borrow: bool,
        ) -> (Self, bool) {
            let (value, borrow) =
                Self::raw(self).borrowing_sub(Self::raw(rhs), borrow);
            return (Self::_core_int(value), borrow);
        }
        #[inline(always)]
        pub fn carrying_mul(
            self,
            rhs: Self,
            carry: Self,
        ) -> (Self, Self) {
            let (low, high) =
                Self::raw(self).carrying_mul(Self::raw(rhs), Self::raw(carry));
            return (Self::_core_int(low), Self::_core_int(high));
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
            return (Self::_core_int(low), Self::_core_int(high));
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
    if !(::core::mem::size_of::<Limited>() == ::core::mem::size_of::<u32>()) {
        panic!("invalid memory layout, mismatching sizes: #ty(#el) != #el");
    };
    if !(::core::mem::align_of::<Limited>() == ::core::mem::align_of::<u32>()) {
        panic!("invalid memory layout, mismatching alignment: #ty(#el) != #el");
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
    impl<T> ::core::cmp::PartialEq<T> for Limited
    where
        T: Rel + Seal + ::core::marker::Copy,
    {
        #[inline(always)]
        fn eq(
            &self,
            rhs: &T,
        ) -> bool {
            let lhs = Self::raw(*self);
            let rhs = Seal::conv_limited(*rhs);
            return lhs == rhs;
        }
    }
    impl<T> ::core::cmp::PartialOrd<T> for Limited
    where
        T: Rel + Seal + ::core::marker::Copy,
    {
        #[inline(always)]
        fn partial_cmp(
            &self,
            rhs: &T,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let lhs = Self::raw(*self);
            let rhs = Seal::conv_limited(*rhs);
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
        pub const fn try_into_usize(self) -> ::core::result::Result<usize, ()> {
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
        pub const fn try_into_u128(self) -> ::core::result::Result<u128, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as u128);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i64(self) -> ::core::result::Result<i64, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as i64);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_i128(self) -> ::core::result::Result<i128, ()> {
            return ::core::result::Result::Ok(Self::raw(self) as i128);
        }
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn try_into_isize(self) -> ::core::result::Result<isize, ()> {
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
            return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs).unwrap();
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
        #[inline(always)]
        #[track_caller]
        pub(self) fn _core_int(value: u32) -> Self {
            if (value >= (1)) && (value <= (100)) {
                return Self(value);
            }
            else {
                ::core::panic!("integral operation produced an invalid value");
            }
        }
        #[inline(always)]
        pub(self) fn _core_int_checked(
            value: u32
        ) -> ::core::option::Option<Self> {
            return if (value >= (1)) && (value <= (100)) {
                ::core::option::Option::Some(Self(value))
            }
            else {
                ::core::option::Option::None
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
            return ((Self::raw(self) >> (8 * 0usize)) & (0xFF as u32)) as u8;
        }
        #[must_use]
        #[inline(always)]
        pub const fn byte1(self) -> u8 {
            return ((Self::raw(self) >> (8 * 1usize)) & (0xFF as u32)) as u8;
        }
        #[must_use]
        #[inline(always)]
        pub const fn byte2(self) -> u8 {
            return ((Self::raw(self) >> (8 * 2usize)) & (0xFF as u32)) as u8;
        }
        #[must_use]
        #[inline(always)]
        pub const fn byte3(self) -> u8 {
            return ((Self::raw(self) >> (8 * 3usize)) & (0xFF as u32)) as u8;
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
            return ((Self::raw(self) >> (32 * 0usize)) & (0xFFFFFFFF as u32))
                as u32;
        }
    }
};

fn main() {
    let lhs = 0b1101u32;
    let rhs = 0b0110u32;

    let lhs = MyNonConstU32::of(lhs);
    let rhs = MyNonConstU32::of(rhs);
    println!("{:?}", lhs + rhs);

    let verified = Verified::try_make(42).unwrap();
    // The non-Copy friend moves its already-validated value into Limited.
    let limited = Limited::of(verified);
    assert_eq!(limited.raw(), 42);
    assert_eq!(Limited::try_make(0), Err(0));
}
