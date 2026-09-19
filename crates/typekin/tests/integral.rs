#![allow(dead_code)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_ops)]
#![feature(const_trait_impl)]

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;

    #[typekin::integral(konst = true)]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct My32(u32);

    #[typekin::integral(konst = true)]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct MyAuto64(u64);

    #[typekin::integral(konst = true)]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct MyI32(i32);

    const CONST_NUMBER: My32 = match My32::try_make(23) {
        Ok(number) => number,
        Err(_) => panic!("valid number rejected"),
    };

    const fn is_even(value: u8) -> bool {
        return value % 2 == 0;
    }

    #[typekin::integral(
            konst = true,
            friends = [u8(conv = self, level = [Full])],
            fn_validator = is_even
        )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct Even(u8);

    #[typekin::integral(
            konst = true,
            friends = [u16(conv = self, level = [Full])],
            fn_get_raw = Self::value
    )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct CustomRaw(u16);

    #[typekin::integral(konst = true)]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct Wide(u128);

    #[test]
    fn bit_accessors_split_u16() {
        let it = CustomRaw::of(0xa1b2u16);
        assert_eq!(it.lo8(), 0xb2);
        assert_eq!(it.hi8(), 0xa1);
    }

    #[test]
    fn bit_accessors_split_u32() {
        let it = My32::of(0xa1_b2_c3_d4u32);
        assert_eq!(it.lo16(), 0xc3d4);
        assert_eq!(it.hi16(), 0xa1b2);
        assert_eq!(it.byte0(), 0xd4);
        assert_eq!(it.byte1(), 0xc3);
        assert_eq!(it.byte2(), 0xb2);
        assert_eq!(it.byte3(), 0xa1);
    }

    #[test]
    fn bit_accessors_split_u64() {
        let it = MyAuto64::make(0x01_02_03_04_05_06_07_08u64);
        assert_eq!(it.lo32(), 0x05_06_07_08);
        assert_eq!(it.hi32(), 0x01_02_03_04);
        assert_eq!(it.word0(), 0x0708);
        assert_eq!(it.word3(), 0x0102);
        assert_eq!(it.byte0(), 0x08);
        assert_eq!(it.byte7(), 0x01);
    }

    #[test]
    fn bit_accessors_split_u128() {
        let it =
            Wide::make(0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f_10u128);
        assert_eq!(it.lo64(), 0x09_0a_0b_0c_0d_0e_0f_10);
        assert_eq!(it.hi64(), 0x01_02_03_04_05_06_07_08);
        assert_eq!(it.dword0(), 0x0d_0e_0f_10);
        assert_eq!(it.dword3(), 0x01_02_03_04);
        assert_eq!(it.word0(), 0x0f10);
        assert_eq!(it.word7(), 0x0102);
        assert_eq!(it.byte0(), 0x10);
        assert_eq!(it.byte15(), 0x01);
    }

    #[test]
    fn makes_from_owned_friend() {
        let it = My32::of(23u32);
        assert_eq!(it.raw(), 23);
    }

    #[test]
    fn makes_from_automatic_raw_friend() {
        let it = MyAuto64::make(23u64);
        assert_eq!(it.raw(), 23);
    }

    #[test]
    fn makes_from_borrowed_friend() {
        let value = 23u32;
        let it = My32::of(&value);
        assert_eq!(it.raw(), value);
    }

    #[test]
    fn makes_from_mutably_borrowed_friend() {
        let mut value = 23u32;
        let it = My32::of(&mut value);
        assert_eq!(it.raw(), value);
        assert_eq!(value, 23u32);
    }

    #[test]
    fn exposes_raw_value() {
        let it = My32::of(23u32);
        assert_eq!(it.raw(), 23);
    }

    #[test]
    fn exposes_configured_raw_value() {
        assert_eq!(CustomRaw::of(23u16).value(), 23);
    }

    #[test]
    fn try_make_accepts_unvalidated_value() {
        let it = My32::of(23);
        assert_eq!(My32::try_make(23), Ok(it));
    }

    #[test]
    fn try_make_is_const() {
        assert_eq!(CONST_NUMBER.raw(), 23);
    }

    #[test]
    fn try_make_accepts_validated_value() {
        assert_eq!(Even::try_make(24).map(Even::raw), Ok(24));
    }

    #[test]
    fn try_make_rejects_invalidated_value() {
        assert_eq!(Even::try_make(23), Err(23));
    }

    #[test]
    fn converts_into_usize() {
        let it = My32::of(23);
        let value: usize = it.into_usize();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_u32() {
        let it = My32::of(23);
        let value: u32 = it.into_u32();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_u64() {
        let it = My32::of(23);
        let value: u64 = it.into_u64();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_u128() {
        let it = My32::of(23);
        let value: u128 = it.into_u128();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_i64() {
        let it = My32::of(23);
        let value: i64 = it.into_i64();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_i128() {
        let it = My32::of(23);
        let value: i128 = it.into_i128();
        assert_eq!(value, 23);
    }

    #[test]
    fn safely_tries_into_usize() {
        let it = My32::of(23);
        assert_eq!(it.try_into_usize(), Ok(23));
    }

    #[test]
    fn safely_tries_into_u32() {
        let it = My32::of(23);
        assert_eq!(it.try_into_u32(), Ok(23));
    }

    #[test]
    fn safely_tries_into_u64() {
        let it = My32::of(23);
        assert_eq!(it.try_into_u64(), Ok(23));
    }

    #[test]
    fn safely_tries_into_u128() {
        let it = My32::of(23);
        assert_eq!(it.try_into_u128(), Ok(23));
    }

    #[test]
    fn safely_tries_into_i64() {
        let it = My32::of(23);
        assert_eq!(it.try_into_i64(), Ok(23));
    }

    #[test]
    fn safely_tries_into_i128() {
        let it = My32::of(23);
        assert_eq!(it.try_into_i128(), Ok(23));
    }

    #[test]
    fn checked_conversion_to_isize_accepts_representable_value() {
        let it = My32::of(23);
        assert_eq!(it.try_into_isize(), Ok(23));
    }

    #[test]
    fn checked_conversion_to_u8_rejects_overflow() {
        let it = My32::of(256);
        assert_eq!(it.try_into_u8(), Err(()));
    }

    #[test]
    fn checked_conversion_to_u16_rejects_overflow() {
        let it = My32::of(65_536);
        assert_eq!(it.try_into_u16(), Err(()));
    }

    #[test]
    fn checked_conversion_to_i8_rejects_overflow() {
        let it = My32::of(128);
        assert_eq!(it.try_into_i8(), Err(()));
    }

    #[test]
    fn checked_conversion_to_i16_rejects_overflow() {
        let it = My32::of(32_768);
        assert_eq!(it.try_into_i16(), Err(()));
    }

    #[test]
    fn checked_conversion_to_i32_rejects_overflow() {
        let it = My32::of(2_147_483_648);
        assert_eq!(it.try_into_i32(), Err(()));
    }

    #[test]
    fn checked_conversion_from_signed_to_unsigned_rejects_negative() {
        assert_eq!(MyI32::of(-1).try_into_u32(), Err(()));
    }

    #[test]
    fn implements_into_usize() {
        let it = My32::of(23);
        let value: usize = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_u32() {
        let it = My32::of(23);
        let value: u32 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_u64() {
        let it = My32::of(23);
        let value: u64 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_u128() {
        let it = My32::of(23);
        let value: u128 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_i64() {
        let it = My32::of(23);
        let value: i64 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_i128() {
        let it = My32::of(23);
        let value: i128 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_try_into_isize() {
        let it = My32::of(23);
        let value: Result<isize, ()> = it.try_into();
        assert_eq!(value, Ok(23));
    }

    #[test]
    fn implements_try_into_u8() {
        let it = My32::of(256);
        let value: Result<u8, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn implements_try_into_u16() {
        let it = My32::of(65_536);
        let value: Result<u16, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn implements_try_into_i8() {
        let it = My32::of(128);
        let value: Result<i8, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn implements_try_into_i16() {
        let it = My32::of(32_768);
        let value: Result<i16, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn implements_try_into_i32() {
        let it = My32::of(2_147_483_648);
        let value: Result<i32, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    fn bitwise_ands_friend_value() {
        let it = My32::of(0b10111);
        assert_eq!((it & 0b10111u32).raw(), 0b10111);
    }

    #[test]
    fn bitwise_ors_friend_value() {
        let it = My32::of(0b10001);
        assert_eq!((it | 0b00110u32).raw(), 0b10111);
    }

    #[test]
    fn bitwise_xors_friend_value() {
        let it = My32::of(0b10111);
        assert_eq!((it ^ 0b01000u32).raw(), 0b11111);
    }

    #[test]
    fn shifts_left() {
        let it = My32::of(23);
        assert_eq!((it << 1).raw(), 46);
    }

    #[test]
    fn shifts_right() {
        let it = My32::of(46);
        assert_eq!((it >> 1).raw(), 23);
    }

    #[test]
    fn bitwise_nots() {
        let it = My32::of(0);
        assert_eq!((!it).raw(), u32::MAX);
    }

    #[test]
    fn adds_assign() {
        let it = My32::of(20);
        let mut value = it;
        let it = My32::of(3);
        value += it;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn subtracts_assign() {
        let it = My32::of(26);
        let mut value = it;
        let it = My32::of(3);
        value -= it;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn multiplies_assign() {
        let it = My32::of(23);
        let mut value = it;
        let it = My32::of(3);
        value *= it;
        assert_eq!(value.raw(), 69);
    }

    #[test]
    fn divides_assign() {
        let it = My32::of(69);
        let mut value = it;
        let it = My32::of(3);
        value /= it;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn takes_remainder_assign() {
        let it = My32::of(70);
        let mut value = it;
        let it = My32::of(47);
        value %= it;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn bitwise_ands_assign() {
        let it = My32::of(0b10111);
        let mut value = it;
        let it = My32::of(0b10111);
        value &= it;
        assert_eq!(value.raw(), 0b10111);
    }

    #[test]
    fn bitwise_ors_assign() {
        let it = My32::of(0b10001);
        let mut value = it;
        let it = My32::of(0b00110);
        value |= it;
        assert_eq!(value.raw(), 0b10111);
    }

    #[test]
    fn shifts_left_assign() {
        let it = My32::of(23);
        let mut value = it;
        value <<= 1;
        assert_eq!(value.raw(), 46);
    }

    #[test]
    fn shifts_right_assign() {
        let it = My32::of(46);
        let mut value = it;
        value >>= 1;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn compares_for_equality_with_friend() {
        let it = My32::of(23);
        assert_eq!(it, 23u32);
    }

    #[test]
    fn compares_for_equality_with_borrowed_friend() {
        let value = 23u32;
        let it = My32::of(23);
        assert_eq!(it, &value);
    }

    #[test]
    fn compares_for_partial_order_with_friend() {
        let it = My32::of(23);
        assert!(it < 24u32);
    }

    #[test]
    fn implements_eq() {
        fn requires_eq<T: Eq>() {}
        requires_eq::<My32>();
    }

    #[test]
    fn implements_ord() {
        let it = My32::of(24);
        let other = it;
        let it = My32::of(23);
        assert_eq!(it.cmp(&other), Ordering::Less);
    }

    #[test]
    fn formats_debug() {
        let it = My32::of(23);
        assert_eq!(format!("{:?}", it), "My32(23)");
    }

    #[test]
    fn formats_binary() {
        let it = My32::of(23);
        assert_eq!(format!("{:b}", it), "10111");
    }

    #[test]
    fn formats_octal() {
        let it = My32::of(23);
        assert_eq!(format!("{:o}", it), "27");
    }

    #[test]
    fn formats_lower_hex() {
        let it = My32::of(23);
        assert_eq!(format!("{:x}", it), "17");
    }

    #[test]
    fn formats_upper_hex() {
        let it = My32::of(23);
        assert_eq!(format!("{:X}", it), "17");
    }

    #[test]
    fn preserves_transparent_layout() {
        assert_eq!(size_of::<My32>(), size_of::<u32>());
    }
}
