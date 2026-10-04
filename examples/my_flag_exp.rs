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
            fn conv_my_flags(self) -> u128;
        }
        const trait Make: [const] Seal {
            fn make(self) -> MyFlags;
        }
        const trait Math: [const] Seal {}
        const trait Bit: [const] Seal {}
        const trait Relation: [const] Seal {}
        const trait Trust: [const] Seal {}
        const impl Seal for MyFlags {
            #[inline(always)]
            fn conv_my_flags(self) -> u128 {
                return Self::raw(self);
            }
        }
        const impl Seal for MyFlag {
            #[inline(always)]
            fn conv_my_flags(self) -> u128 {
                return MyFlag::raw(self);
            }
        }
        const impl Bit for MyFlag {}
        const impl Make for MyFlag {
            #[inline(always)]
            fn make(self) -> MyFlags {
                let raw = <Self as Seal>::conv_my_flags(self);
                return MyFlags::_unchecked(raw);
            }
        }
        const impl Bit for MyFlags {}
        const impl Make for MyFlags {
            #[inline(always)]
            fn make(self) -> MyFlags {
                let raw = <Self as Seal>::conv_my_flags(self);
                return MyFlags::_unchecked(raw);
            }
        }
        const impl Math for MyFlags {}
        const impl Relation for MyFlags {}
        const impl Seal for i16 {
            #[inline(always)]
            fn conv_my_flags(self) -> u128 {
                return self as u128;
            }
        }
        const impl Make for i16 {
            #[inline(always)]
            fn make(self) -> MyFlags {
                let raw = <Self as Seal>::conv_my_flags(self);
                return MyFlags::_unchecked(raw);
            }
        }
        const impl Seal for u128 {
            #[inline(always)]
            fn conv_my_flags(self) -> u128 {
                return self as u128;
            }
        }
        const impl Bit for u128 {}
        const impl Make for u128 {
            #[inline(always)]
            fn make(self) -> MyFlags {
                let raw = <Self as Seal>::conv_my_flags(self);
                return MyFlags::_unchecked(raw);
            }
        }
        const impl Math for u128 {}
        const impl Relation for u128 {}
        const impl Seal for u16 {
            #[inline(always)]
            fn conv_my_flags(self) -> u128 {
                return self as u128;
            }
        }
        const impl Make for u16 {
            #[inline(always)]
            fn make(self) -> MyFlags {
                let raw = <Self as Seal>::conv_my_flags(self);
                return MyFlags::_unchecked(raw);
            }
        }
        const impl Seal for u32 {
            #[inline(always)]
            fn conv_my_flags(self) -> u128 {
                return self as u128;
            }
        }
        const impl Make for u32 {
            #[inline(always)]
            fn make(self) -> MyFlags {
                let raw = <Self as Seal>::conv_my_flags(self);
                return MyFlags::_unchecked(raw);
            }
        }
        const impl Seal for u64 {
            #[inline(always)]
            fn conv_my_flags(self) -> u128 {
                return self as u128;
            }
        }
        const impl Make for u64 {
            #[inline(always)]
            fn make(self) -> MyFlags {
                let raw = <Self as Seal>::conv_my_flags(self);
                return MyFlags::_unchecked(raw);
            }
        }
        const impl Seal for u8 {
            #[inline(always)]
            fn conv_my_flags(self) -> u128 {
                return self as u128;
            }
        }
        const impl Make for u8 {
            #[inline(always)]
            fn make(self) -> MyFlags {
                let raw = <Self as Seal>::conv_my_flags(self);
                return MyFlags::_unchecked(raw);
            }
        }
        const impl Seal for usize {
            #[inline(always)]
            fn conv_my_flags(self) -> u128 {
                return self as u128;
            }
        }
        const impl Make for usize {
            #[inline(always)]
            fn make(self) -> MyFlags {
                let raw = <Self as Seal>::conv_my_flags(self);
                return MyFlags::_unchecked(raw);
            }
        }
        impl ::core::convert::AsRef<u128> for MyFlags {
            #[inline(always)]
            fn as_ref(&self) -> &u128 {
                return &self.0;
            }
        }
        impl ::core::str::FromStr for MyFlags {
            type Err = ();
            #[inline(always)]
            fn from_str(
                source: &str
            ) -> ::core::result::Result<Self, Self::Err> {
                let value =
                    match <u128 as ::core::str::FromStr>::from_str(source) {
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
        impl MyFlags {
            #[doc = r" The number of bits in the wrapped primitive integer."]
            pub const BITS: u32 = u128::BITS;
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
            pub const fn to_be_bytes(
                self
            ) -> [u8; ::core::mem::size_of::<u128>()] {
                return Self::raw(self).to_be_bytes();
            }
            #[inline(always)]
            pub const fn to_le_bytes(
                self
            ) -> [u8; ::core::mem::size_of::<u128>()] {
                return Self::raw(self).to_le_bytes();
            }
            #[inline(always)]
            pub const fn to_ne_bytes(
                self
            ) -> [u8; ::core::mem::size_of::<u128>()] {
                return Self::raw(self).to_ne_bytes();
            }
        }
        impl MyFlags {
            #[inline(always)]
            pub const fn rotate_left(
                self,
                n: u32,
            ) -> Self {
                return Self::_core_int(Self::raw(self).rotate_left(n));
            }
            #[inline(always)]
            pub const fn rotate_right(
                self,
                n: u32,
            ) -> Self {
                return Self::_core_int(Self::raw(self).rotate_right(n));
            }
            #[inline(always)]
            pub const fn swap_bytes(self) -> Self {
                return Self::_core_int(Self::raw(self).swap_bytes());
            }
            #[inline(always)]
            pub const fn reverse_bits(self) -> Self {
                return Self::_core_int(Self::raw(self).reverse_bits());
            }
            #[inline(always)]
            pub const fn isolate_highest_one(self) -> Self {
                return Self::_core_int(Self::raw(self).isolate_highest_one());
            }
            #[inline(always)]
            pub const fn isolate_lowest_one(self) -> Self {
                return Self::_core_int(Self::raw(self).isolate_lowest_one());
            }
            #[inline(always)]
            pub const fn to_be(self) -> Self {
                return Self::_core_int(Self::raw(self).to_be());
            }
            #[inline(always)]
            pub const fn to_le(self) -> Self {
                return Self::_core_int(Self::raw(self).to_le());
            }
            #[inline(always)]
            pub const fn from_be(value: Self) -> Self {
                return Self::_core_int(u128::from_be(Self::raw(value)));
            }
            #[inline(always)]
            pub const fn from_le(value: Self) -> Self {
                return Self::_core_int(u128::from_le(Self::raw(value)));
            }
            #[inline(always)]
            pub const fn from_ne_bytes(
                bytes: [u8; ::core::mem::size_of::<u128>()]
            ) -> Self {
                return Self::_core_int(u128::from_ne_bytes(bytes));
            }
            #[inline(always)]
            pub const fn from_be_bytes(
                bytes: [u8; ::core::mem::size_of::<u128>()]
            ) -> Self {
                return Self::_core_int(u128::from_be_bytes(bytes));
            }
            #[inline(always)]
            pub const fn from_le_bytes(
                bytes: [u8; ::core::mem::size_of::<u128>()]
            ) -> Self {
                return Self::_core_int(u128::from_le_bytes(bytes));
            }
            #[inline(always)]
            pub const fn checked_add(
                self,
                rhs: Self,
            ) -> ::core::option::Option<Self> {
                return match Self::raw(self).checked_add(Self::raw(rhs)) {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }
            #[inline(always)]
            pub const fn checked_sub(
                self,
                rhs: Self,
            ) -> ::core::option::Option<Self> {
                return match Self::raw(self).checked_sub(Self::raw(rhs)) {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }
            #[inline(always)]
            pub const fn checked_neg(self) -> ::core::option::Option<Self> {
                return match Self::raw(self).checked_neg() {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }
            #[inline(always)]
            pub const fn checked_mul(
                self,
                rhs: Self,
            ) -> ::core::option::Option<Self> {
                return match Self::raw(self).checked_mul(Self::raw(rhs)) {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }
            #[inline(always)]
            pub const fn checked_div(
                self,
                rhs: Self,
            ) -> ::core::option::Option<Self> {
                return match Self::raw(self).checked_div(Self::raw(rhs)) {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }
            #[inline(always)]
            pub const fn checked_rem(
                self,
                rhs: Self,
            ) -> ::core::option::Option<Self> {
                return match Self::raw(self).checked_rem(Self::raw(rhs)) {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }
            #[inline(always)]
            pub const fn checked_div_euclid(
                self,
                rhs: Self,
            ) -> ::core::option::Option<Self> {
                return match Self::raw(self).checked_div_euclid(Self::raw(rhs))
                {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }
            #[inline(always)]
            pub const fn checked_rem_euclid(
                self,
                rhs: Self,
            ) -> ::core::option::Option<Self> {
                return match Self::raw(self).checked_rem_euclid(Self::raw(rhs))
                {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }
            #[inline(always)]
            pub const fn checked_shl(
                self,
                rhs: u32,
            ) -> ::core::option::Option<Self> {
                return match Self::raw(self).checked_shl(rhs) {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }
            #[inline(always)]
            pub const fn checked_shr(
                self,
                rhs: u32,
            ) -> ::core::option::Option<Self> {
                return match Self::raw(self).checked_shr(rhs) {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }
            #[inline(always)]
            pub const fn checked_pow(
                self,
                exp: u32,
            ) -> ::core::option::Option<Self> {
                return match Self::raw(self).checked_pow(exp) {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }
        }
        impl MyFlags {
            #[inline(always)]
            pub const fn midpoint(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).midpoint(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn unbounded_shl(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_core_int(Self::raw(self).unbounded_shl(rhs));
            }
            #[inline(always)]
            pub const fn unbounded_shr(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_core_int(Self::raw(self).unbounded_shr(rhs));
            }
            #[inline(always)]
            pub const fn saturating_add(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).saturating_add(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn saturating_sub(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).saturating_sub(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn saturating_mul(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).saturating_mul(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn saturating_div(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).saturating_div(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn saturating_pow(
                self,
                exp: u32,
            ) -> Self {
                return Self::_core_int(Self::raw(self).saturating_pow(exp));
            }
            #[inline(always)]
            pub const fn wrapping_add(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).wrapping_add(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_sub(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).wrapping_sub(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_neg(self) -> Self {
                return Self::_core_int(Self::raw(self).wrapping_neg());
            }
            #[inline(always)]
            pub const fn wrapping_mul(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).wrapping_mul(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_div(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).wrapping_div(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_rem(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).wrapping_rem(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).wrapping_div_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_rem_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).wrapping_rem_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn wrapping_shl(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_core_int(Self::raw(self).wrapping_shl(rhs));
            }
            #[inline(always)]
            pub const fn wrapping_shr(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_core_int(Self::raw(self).wrapping_shr(rhs));
            }
            #[inline(always)]
            pub const fn wrapping_pow(
                self,
                exp: u32,
            ) -> Self {
                return Self::_core_int(Self::raw(self).wrapping_pow(exp));
            }
            #[inline(always)]
            pub const fn pow(
                self,
                exp: u32,
            ) -> Self {
                return Self::_core_int(Self::raw(self).pow(exp));
            }
            #[inline(always)]
            pub const fn isqrt(self) -> Self {
                return Self::_core_int(Self::raw(self).isqrt());
            }
            #[inline(always)]
            pub const fn div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).div_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn rem_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).rem_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_add(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).strict_add(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_sub(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).strict_sub(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_neg(self) -> Self {
                return Self::_core_int(Self::raw(self).strict_neg());
            }
            #[inline(always)]
            pub const fn strict_mul(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).strict_mul(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_div(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).strict_div(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_rem(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).strict_rem(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).strict_div_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_rem_euclid(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).strict_rem_euclid(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn strict_shl(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_core_int(Self::raw(self).strict_shl(rhs));
            }
            #[inline(always)]
            pub const fn strict_shr(
                self,
                rhs: u32,
            ) -> Self {
                return Self::_core_int(Self::raw(self).strict_shr(rhs));
            }
            #[inline(always)]
            pub const fn strict_pow(
                self,
                exp: u32,
            ) -> Self {
                return Self::_core_int(Self::raw(self).strict_pow(exp));
            }
        }
        impl MyFlags {
            #[inline(always)]
            pub const fn overflowing_add(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_add(Self::raw(rhs));
                return (Self::_core_int(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_sub(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_sub(Self::raw(rhs));
                return (Self::_core_int(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_neg(self) -> (Self, bool) {
                let (value, overflowed) = Self::raw(self).overflowing_neg();
                return (Self::_core_int(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_mul(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_mul(Self::raw(rhs));
                return (Self::_core_int(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_div(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_div(Self::raw(rhs));
                return (Self::_core_int(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_rem(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_rem(Self::raw(rhs));
                return (Self::_core_int(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_div_euclid(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_div_euclid(Self::raw(rhs));
                return (Self::_core_int(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_rem_euclid(
                self,
                rhs: Self,
            ) -> (Self, bool) {
                let (value, overflowed) =
                    Self::raw(self).overflowing_rem_euclid(Self::raw(rhs));
                return (Self::_core_int(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_shl(
                self,
                rhs: u32,
            ) -> (Self, bool) {
                let (value, overflowed) = Self::raw(self).overflowing_shl(rhs);
                return (Self::_core_int(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_shr(
                self,
                rhs: u32,
            ) -> (Self, bool) {
                let (value, overflowed) = Self::raw(self).overflowing_shr(rhs);
                return (Self::_core_int(value), overflowed);
            }
            #[inline(always)]
            pub const fn overflowing_pow(
                self,
                exp: u32,
            ) -> (Self, bool) {
                let (value, overflowed) = Self::raw(self).overflowing_pow(exp);
                return (Self::_core_int(value), overflowed);
            }
        }
        impl MyFlags {
            #[inline(always)]
            pub const fn cast_signed(self) -> i128 {
                return Self::raw(self).cast_signed();
            }
            #[inline(always)]
            pub const fn bit_width(self) -> u32 {
                return Self::raw(self).bit_width();
            }
            #[inline(always)]
            pub const fn funnel_shl(
                self,
                right: Self,
                n: u32,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).funnel_shl(Self::raw(right), n),
                );
            }
            #[inline(always)]
            pub const fn funnel_shr(
                self,
                right: Self,
                n: u32,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).funnel_shr(Self::raw(right), n),
                );
            }
            #[inline(always)]
            pub const fn abs_diff(
                self,
                rhs: Self,
            ) -> u128 {
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
                return Self::_core_int(Self::raw(self).next_power_of_two());
            }
            #[inline(always)]
            pub const fn checked_next_power_of_two(
                self
            ) -> ::core::option::Option<Self> {
                return match Self::raw(self).checked_next_power_of_two() {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
                };
            }
            #[inline(always)]
            pub const fn div_ceil(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).div_ceil(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn next_multiple_of(
                self,
                rhs: Self,
            ) -> Self {
                return Self::_core_int(
                    Self::raw(self).next_multiple_of(Self::raw(rhs)),
                );
            }
            #[inline(always)]
            pub const fn checked_next_multiple_of(
                self,
                rhs: Self,
            ) -> ::core::option::Option<Self> {
                return match Self::raw(self)
                    .checked_next_multiple_of(Self::raw(rhs))
                {
                    ::core::option::Option::Some(value) => {
                        Self::_core_int_checked(value)
                    }
                    ::core::option::Option::None => {
                        ::core::option::Option::None
                    }
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
                let (low, high) = Self::raw(self)
                    .carrying_mul(Self::raw(rhs), Self::raw(carry));
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
            panic!("invalid memory layout, mismatching sizes: #ty(#el) != #el");
        };
        if !(::core::mem::align_of::<MyFlags>()
            == ::core::mem::align_of::<u128>())
        {
            panic!(
                "invalid memory layout, mismatching alignment: #ty(#el) != #el"
            );
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
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
                let it = Seal::conv_my_flags(rhs);
                *self = self._bitxor(it);
            }
        }
        impl ::core::fmt::Display for MyFlags {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                let raw = Self::raw(*self);
                return ::core::fmt::Display::fmt(&raw, f);
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
        const impl ::core::cmp::PartialEq for MyFlags {
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
        const impl ::core::cmp::PartialOrd for MyFlags {
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
        impl MyFlags {
            #[inline(always)]
            #[allow(private_bounds)]
            pub const fn of<T>(it: T) -> MyFlags
            where
                T: [const] Make + [const] ::core::marker::Destruct,
            {
                return <T as Make>::make(it);
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
            #[inline(always)]
            #[track_caller]
            pub(self) const fn _core_int(value: u128) -> Self {
                if true {
                    return Self(value);
                }
                else {
                    ::core::panic!(
                        "integral operation produced an invalid value"
                    );
                }
            }
            #[inline(always)]
            pub(self) const fn _core_int_checked(
                value: u128
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
            #[allow(non_upper_case_globals)]
            pub const Z: Self = Self::_unchecked(MyFlag::Z as u128);
            #[allow(non_upper_case_globals)]
            pub const A: Self = Self::_unchecked(MyFlag::A as u128);
            #[allow(non_upper_case_globals)]
            pub const B: Self = Self::_unchecked(MyFlag::B as u128);
            #[allow(non_upper_case_globals)]
            pub const C: Self = Self::_unchecked(MyFlag::C as u128);
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
                return self.unknown_bits() != Self::empty().bits();
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
                const ITEMS: &'static [MyFlag] = &[
                    MyFlag::Z,
                    MyFlag::A,
                    MyFlag::B,
                    MyFlag::C,
                ];
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
        impl ::core::fmt::Display for MyFlag {
            #[inline(always)]
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                return ::core::fmt::Display::fmt(self.name(), f);
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
                let rhs = FlagSeal::conv_my_flag(rhs);
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
                let rhs = FlagSeal::conv_my_flag(rhs);
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
                let rhs = FlagSeal::conv_my_flag(rhs);
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
            fn conv_my_flag(self) -> u128;
        }
        const trait FlagMake: [const] FlagSeal {}
        const trait FlagMath: [const] FlagSeal {}
        const trait FlagBit: [const] FlagSeal {}
        const trait FlagCmp: [const] FlagSeal {}
        const trait FlagTrust: [const] FlagSeal {}
        const impl FlagSeal for MyFlag {
            #[inline(always)]
            fn conv_my_flag(self) -> u128 {
                return MyFlag::raw(self);
            }
        }
        const impl FlagBit for MyFlag {}
        const impl FlagCmp for MyFlag {}
        const impl FlagMake for MyFlag {}
        const impl FlagSeal for MyFlags {
            #[inline(always)]
            fn conv_my_flag(self) -> u128 {
                return MyFlags::raw(self);
            }
        }
        const impl FlagBit for MyFlags {}
        const impl FlagMake for MyFlags {}
    };
}

type Subject = subject::MyFlag;
type Value = subject::MyFlags;

fn main() {
    let lhs = Subject::A;
    let rhs = Subject::B.into_value();

    assert_eq!(lhs.to_string(), "A");
    assert_eq!(lhs.into_value().to_string(), "10");
    println!("{:?}", (lhs & rhs) == (lhs & rhs));
    println!("{}", rhs & lhs);
    println!("{}", lhs & rhs);
    println!("{}", lhs);

    for x in Subject::items() {
        println!("{}", x.name());
    }
}
