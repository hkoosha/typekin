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

    #[typekin::integral(konst = true, with = [display])]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct MyI32(i32);

    #[typekin::integral(
        konst = true,
        with = [display],
        without = [fn_conv_raw],
        get_raw = Self::decoded,
    )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct EncodedDisplay(i32);

    impl EncodedDisplay {
        const fn decoded(self) -> i32 {
            self.0 - 100
        }
    }

    const CONST_NUMBER: My32 = match My32::try_make(23) {
        Ok(number) => number,
        Err(_) => panic!("valid number rejected"),
    };

    const fn is_even(value: u8) -> bool {
        return value % 2 == 0;
    }

    const fn is_not_fifty(value: u8) -> bool {
        return value != 50;
    }

    const fn accepts_ranged(_: u8) -> bool {
        return true;
    }

    #[typekin::integral(
        konst = true,
        friends = _(u8) -> [Make, Math, Bit, Relation],
        valid = [is_even, is_not_fifty],
        in = 2..=100,
    )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct Even(u8);

    #[typekin::integral(
        konst = true,
        friends = [_(u8) -> [Make, Math, Bit, Relation]],
        in = [2..=4, 8..10],
        valid = accepts_ranged,
    )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct Ranged(u8);

    #[typekin::integral(konst = true, in = ..)]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct AnyU8(u8);

    #[typekin::integral(konst = true, in = ..5)]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct BelowFive(u8);

    #[typekin::integral(konst = true, in = ..=5)]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct AtMostFive(u8);

    #[typekin::integral(konst = true, in = -2..)]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct FromNegativeTwo(i8);

    const fn is_even_i8(value: i8) -> bool {
        return value % 2 == 0;
    }

    #[typekin::integral(
        konst = true,
        valid = is_even_i8,
        in = [..=-6, 4..],
    )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct SplitEven(i8);

    #[typekin::integral(
        konst = true,
        valid = is_even,
        without = [fn_make_unchecked_try],
    )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct ParsedEven(u8);

    #[typekin::integral(
        konst = true,
        without = [impl_core_int],
    )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct CoreDisabled(u8);

    const RANGED_NUMBER: Ranged = match Ranged::try_make(3) {
        Ok(number) => number,
        Err(_) => panic!("valid ranged number rejected"),
    };

    #[typekin::integral(
        konst = true,
        friends = [_(u16) -> [Make, Math, Bit, Relation]],
        get_raw = Self::value
    )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct CustomRaw(u16);

    #[typekin::integral(konst = true)]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct Wide(u128);

    #[typekin::integral(
        konst = true,
        friends = _(u8) -> [Make, Math, Bit, Trust],
        valid = is_even,
        in = 2..=10,
    )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct TrustEven(u8);

    const TRUSTED_ODD_NUMBER: TrustEven = TrustEven::of(3u8);

    #[test]
    fn trusted_const_construction_bypasses_callback_and_range_validation() {
        assert_eq!(TRUSTED_ODD_NUMBER.raw(), 3);
        assert_eq!(TrustEven::of(12u8).raw(), 12);
        assert_eq!(TrustEven::try_make(3), Err(3));
        assert_eq!(TrustEven::try_make(12), Err(12));
    }

    #[test]
    fn trusted_const_friend_operations_still_validate() {
        assert_panics(|| {
            let _ = TrustEven::try_make(4).unwrap() + 1u8;
        });
        assert_panics(|| {
            let _ = TrustEven::try_make(4).unwrap() | 1u8;
        });
    }

    #[test]
    fn bit_accessors_split_u16() {
        let it = CustomRaw::make(0xa1b2);
        assert_eq!(it.lo8(), 0xb2);
        assert_eq!(it.hi8(), 0xa1);
    }

    #[test]
    fn bit_accessors_split_u32() {
        let it = My32::make(0xa1_b2_c3_d4);
        assert_eq!(it.lo16(), 0xc3d4);
        assert_eq!(it.hi16(), 0xa1b2);
        assert_eq!(it.byte0(), 0xd4);
        assert_eq!(it.byte1(), 0xc3);
        assert_eq!(it.byte2(), 0xb2);
        assert_eq!(it.byte3(), 0xa1);
    }

    #[test]
    fn bit_accessors_split_u64() {
        let it = MyAuto64::make(0x01_02_03_04_05_06_07_08);
        assert_eq!(it.lo32(), 0x05_06_07_08);
        assert_eq!(it.hi32(), 0x01_02_03_04);
        assert_eq!(it.word0(), 0x0708);
        assert_eq!(it.word3(), 0x0102);
        assert_eq!(it.byte0(), 0x08);
        assert_eq!(it.byte7(), 0x01);
    }

    #[test]
    fn bit_accessors_split_u128() {
        let it = Wide::make(0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f_10);
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
    fn makes_from_raw_value() {
        let it = My32::make(23);
        assert_eq!(it.raw(), 23);
    }

    #[test]
    fn makes_from_automatic_raw_friend() {
        let it = MyAuto64::make(23);
        assert_eq!(it.raw(), 23);
    }

    #[test]
    fn makes_from_safely_widening_raw_friends() {
        assert_eq!(My32::of(23u8).raw(), 23);
        assert_eq!(My32::of(23u16).raw(), 23);
    }

    #[test]
    fn makes_from_owned_friend() {
        let value = 23u32;
        let it = My32::of(value);

        assert_eq!(it.raw(), value);
    }

    #[test]
    fn exposes_raw_value() {
        let it = My32::make(23);
        assert_eq!(it.raw(), 23);
    }

    #[test]
    fn exposes_underlying_reference_with_as_ref() {
        let it = My32::make(23);
        let raw: &u32 = it.as_ref();

        assert_eq!(*raw, 23);
    }

    #[test]
    fn exposes_configured_raw_value() {
        assert_eq!(CustomRaw::make(23).value(), 23);
    }

    #[test]
    fn try_make_accepts_unvalidated_value() {
        let it = My32::make(23);
        assert_eq!(My32::try_make(23), Ok(it));
    }

    #[test]
    fn parses_integrals_from_plain_numbers() {
        assert_eq!("23".parse::<My32>().map(My32::raw), Ok(23));
        assert_eq!("-23".parse::<MyI32>().map(MyI32::raw), Ok(-23));
        assert_eq!("24".parse::<Even>().map(Even::raw), Ok(24));
        assert_eq!("-6".parse::<SplitEven>().map(SplitEven::raw), Ok(-6));
    }

    #[test]
    fn parsing_returns_unit_error_for_invalid_syntax_and_values() {
        for source in [
            "",
            "-1",
            "4_294_967_296",
            "not a number",
        ] {
            assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");
        }

        for source in [
            "23", "50", "102",
        ] {
            assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");
        }
        for source in [
            "-5", "0",
        ] {
            assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
        }
    }

    #[test]
    fn parsing_remains_checked_when_try_make_is_disabled() {
        assert_eq!("24".parse::<ParsedEven>().map(ParsedEven::raw), Ok(24));
        assert_eq!("23".parse::<ParsedEven>(), Err(()));
    }

    #[test]
    fn forwards_common_stable_integral_methods() {
        let number = My32::make(0x12_34_56_78);
        assert_eq!(My32::BITS, u32::BITS);
        assert_eq!(number.count_ones(), 13);
        assert_eq!(number.count_zeros(), 19);
        assert_eq!(number.leading_zeros(), 3);
        assert_eq!(number.trailing_zeros(), 3);
        assert_eq!(number.leading_ones(), 0);
        assert_eq!(number.trailing_ones(), 0);
        assert_eq!(number.highest_one(), Some(28));
        assert_eq!(number.lowest_one(), Some(3));
        assert_eq!(number.ilog2(), 28);
        assert_eq!(number.ilog10(), 8);
        assert_eq!(number.checked_ilog2(), Some(28));
        assert_eq!(number.to_be_bytes(), 0x12_34_56_78u32.to_be_bytes());
        assert_eq!(number.rotate_left(4).raw(), 0x23_45_67_81);
        assert_eq!(number.rotate_right(4).raw(), 0x81_23_45_67);
        assert_eq!(number.swap_bytes().raw(), 0x78_56_34_12);
        assert_eq!(number.reverse_bits().raw(), 0x1e_6a_2c_48);
        assert_eq!(My32::from_be_bytes(number.to_be_bytes()), number);
        assert_eq!(
            number.checked_add(My32::make(1)).map(My32::raw),
            Some(0x12_34_56_79)
        );
        assert_eq!(number.checked_shl(32), None);
        assert_eq!(number.unbounded_shl(32).raw(), 0);
        assert_eq!(number.wrapping_shr(32), number);
        assert_eq!(number.midpoint(My32::make(0)).raw(), 0x09_1a_2b_3c);
        assert_eq!(My32::make(12).checked_pow(3).map(My32::raw), Some(1728));
        assert_eq!(My32::make(12).saturating_mul(My32::make(3)).raw(), 36);
        assert_eq!(My32::make(12).overflowing_mul(My32::make(3)).0.raw(), 36);
        assert_eq!(MyI32::make(-7).div_euclid(MyI32::make(3)).raw(), -3);
        assert_eq!(MyI32::make(-7).rem_euclid(MyI32::make(3)).raw(), 2);
        assert_eq!(My32::make(144).isqrt().raw(), 12);
    }

    #[test]
    fn checked_core_operations_reject_invalid_results() {
        let upper_bound = Even::try_make(100).unwrap();
        let two = Even::try_make(2).unwrap();
        assert_eq!(upper_bound.checked_add(two), None);
        assert_panics(|| upper_bound.saturating_add(two));
    }

    #[test]
    fn core_operations_validate_every_wrapped_result() {
        let two = Even::try_make(2).unwrap();
        let hundred = Even::try_make(100).unwrap();

        assert_panics(|| Even::from_ne_bytes([1]));
        assert_panics(|| two.midpoint(hundred));
        assert_panics(|| hundred.overflowing_add(two));
        assert_panics(|| hundred.carrying_add(two, false));

        let negative = SplitEven::try_make(-6).unwrap();
        assert_panics(|| negative.signum());
    }

    #[test]
    fn forwards_signed_stable_integral_methods() {
        let negative = MyI32::make(-12);
        assert_eq!(negative.abs().raw(), 12);
        assert_eq!(negative.checked_abs().map(MyI32::raw), Some(12));
        assert_eq!(negative.strict_abs().raw(), 12);
        assert_eq!(negative.saturating_neg().raw(), 12);
        assert_eq!(negative.saturating_abs().raw(), 12);
        assert_eq!(negative.wrapping_abs().raw(), 12);
        assert_eq!(negative.overflowing_abs().0.raw(), 12);
        assert_eq!(negative.unsigned_abs(), 12u32);
        assert_eq!(negative.abs_diff(MyI32::make(3)), 15u32);
        assert_eq!(negative.cast_unsigned(), (-12i32).cast_unsigned());
        assert_eq!(negative.signum().raw(), -1);
        assert!(negative.is_negative());
        assert!(!negative.is_positive());
        assert_eq!(MyI32::make(144).checked_isqrt().map(MyI32::raw), Some(12));
        assert_eq!(negative.checked_isqrt(), None);
    }

    #[test]
    fn forwards_unsigned_stable_integral_methods() {
        assert_eq!(My32::make(15).bit_width(), 4);
        assert_eq!(My32::make(12).abs_diff(My32::make(3)), 9u32);
        assert_eq!(My32::make(u32::MAX).cast_signed(), -1i32);
        assert_eq!(
            My32::make(0x12_34_56_78)
                .funnel_shl(My32::make(0x9a_bc_de_f0), 4)
                .raw(),
            0x23_45_67_89
        );
        assert_eq!(
            My32::make(0x12_34_56_78)
                .funnel_shr(My32::make(0x9a_bc_de_f0), 4)
                .raw(),
            0x89_ab_cd_ef
        );
        assert!(My32::make(12).is_multiple_of(My32::make(3)));
        assert!(!My32::make(12).is_multiple_of(My32::make(5)));
        assert!(My32::make(16).is_power_of_two());
        assert_eq!(My32::make(15).next_power_of_two().raw(), 16);
        assert_eq!(
            My32::make(15).checked_next_power_of_two().map(My32::raw),
            Some(16)
        );
        assert_eq!(My32::make(10).div_ceil(My32::make(3)).raw(), 4);
        assert_eq!(My32::make(10).next_multiple_of(My32::make(6)).raw(), 12);
        assert_eq!(
            My32::make(10)
                .checked_next_multiple_of(My32::make(6))
                .map(My32::raw),
            Some(12)
        );
        assert_eq!(My32::make(1).carrying_add(My32::make(2), false).0.raw(), 3);
        assert_eq!(
            My32::make(5).borrowing_sub(My32::make(2), false).0.raw(),
            3
        );
        assert_eq!(
            My32::make(3)
                .carrying_mul(My32::make(4), My32::make(5))
                .0
                .raw(),
            17
        );
        assert_eq!(
            My32::make(3)
                .carrying_mul_add(My32::make(4), My32::make(5), My32::make(7))
                .0
                .raw(),
            24
        );
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
    fn try_make_rejects_second_callback() {
        assert_eq!(Even::try_make(50), Err(50));
    }

    #[test]
    fn try_make_rejects_even_value_outside_configured_ranges() {
        assert_eq!(Even::try_make(102), Err(102));
    }

    #[test]
    fn range_validation_accepts_each_configured_interval() {
        assert_eq!(Ranged::try_make(2).map(Ranged::raw), Ok(2));
        assert_eq!(Ranged::try_make(4).map(Ranged::raw), Ok(4));
        assert_eq!(Ranged::try_make(8).map(Ranged::raw), Ok(8));
        assert_eq!(Ranged::try_make(9).map(Ranged::raw), Ok(9));
        assert_eq!(Ranged::try_make(1), Err(1));
        assert_eq!(Ranged::try_make(5), Err(5));
        assert_eq!(Ranged::try_make(10), Err(10));
    }

    #[test]
    fn range_validation_lowers_open_and_closed_endpoints() {
        assert_eq!(AnyU8::try_make(0).map(AnyU8::raw), Ok(0));
        assert_eq!(AnyU8::try_make(u8::MAX).map(AnyU8::raw), Ok(u8::MAX));

        assert_eq!(BelowFive::try_make(0).map(BelowFive::raw), Ok(0));
        assert_eq!(BelowFive::try_make(4).map(BelowFive::raw), Ok(4));
        assert_eq!(BelowFive::try_make(5), Err(5));

        assert_eq!(AtMostFive::try_make(0).map(AtMostFive::raw), Ok(0));
        assert_eq!(AtMostFive::try_make(5).map(AtMostFive::raw), Ok(5));
        assert_eq!(AtMostFive::try_make(6), Err(6));

        assert_eq!(
            FromNegativeTwo::try_make(-2).map(FromNegativeTwo::raw),
            Ok(-2)
        );
        assert_eq!(
            FromNegativeTwo::try_make(i8::MAX).map(FromNegativeTwo::raw),
            Ok(i8::MAX)
        );
        assert_eq!(FromNegativeTwo::try_make(-3), Err(-3));
    }

    #[test]
    fn open_range_union_ors_ranges_and_ands_callbacks() {
        for value in [
            -8, -6, 4, 6,
        ] {
            assert_eq!(
                SplitEven::try_make(value).map(SplitEven::raw),
                Ok(value)
            );
        }
        for value in [
            -7, -5, 0, 5,
        ] {
            assert_eq!(SplitEven::try_make(value), Err(value));
        }
    }

    #[test]
    fn range_validation_is_const() {
        assert_eq!(RANGED_NUMBER.raw(), 3);
    }

    #[test]
    fn range_validation_checks_friended_construction_and_math() {
        assert_panics(|| Ranged::of(5u8));

        let value = Ranged::try_make(4).unwrap();
        assert_panics(|| value + 1u8);
    }

    #[test]
    fn converts_into_usize() {
        let it = My32::make(23);
        let value: usize = it.into_usize();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_u32() {
        let it = My32::make(23);
        let value: u32 = it.into_u32();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_u64() {
        let it = My32::make(23);
        let value: u64 = it.into_u64();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_u128() {
        let it = My32::make(23);
        let value: u128 = it.into_u128();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_i64() {
        let it = My32::make(23);
        let value: i64 = it.into_i64();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_i128() {
        let it = My32::make(23);
        let value: i128 = it.into_i128();
        assert_eq!(value, 23);
    }

    #[test]
    fn safely_tries_into_usize() {
        let it = My32::make(23);
        assert_eq!(it.try_into_usize(), Ok(23));
    }

    #[test]
    fn safely_tries_into_u32() {
        let it = My32::make(23);
        assert_eq!(it.try_into_u32(), Ok(23));
    }

    #[test]
    fn safely_tries_into_u64() {
        let it = My32::make(23);
        assert_eq!(it.try_into_u64(), Ok(23));
    }

    #[test]
    fn safely_tries_into_u128() {
        let it = My32::make(23);
        assert_eq!(it.try_into_u128(), Ok(23));
    }

    #[test]
    fn safely_tries_into_i64() {
        let it = My32::make(23);
        assert_eq!(it.try_into_i64(), Ok(23));
    }

    #[test]
    fn safely_tries_into_i128() {
        let it = My32::make(23);
        assert_eq!(it.try_into_i128(), Ok(23));
    }

    #[test]
    fn checked_conversion_to_isize_accepts_representable_value() {
        let it = My32::make(23);
        assert_eq!(it.try_into_isize(), Ok(23));
    }

    #[test]
    fn checked_conversion_to_u8_rejects_overflow() {
        let it = My32::make(256);
        assert_eq!(it.try_into_u8(), Err(()));
    }

    #[test]
    fn checked_conversion_to_u16_rejects_overflow() {
        let it = My32::make(65_536);
        assert_eq!(it.try_into_u16(), Err(()));
    }

    #[test]
    fn checked_conversion_to_i8_rejects_overflow() {
        let it = My32::make(128);
        assert_eq!(it.try_into_i8(), Err(()));
    }

    #[test]
    fn checked_conversion_to_i16_rejects_overflow() {
        let it = My32::make(32_768);
        assert_eq!(it.try_into_i16(), Err(()));
    }

    #[test]
    fn checked_conversion_to_i32_rejects_overflow() {
        let it = My32::make(2_147_483_648);
        assert_eq!(it.try_into_i32(), Err(()));
    }

    #[test]
    fn checked_conversion_from_signed_to_unsigned_rejects_negative() {
        assert_eq!(MyI32::of(-1).try_into_u32(), Err(()));
    }

    #[test]
    fn implements_into_usize() {
        let it = My32::make(23);
        let value: usize = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_u32() {
        let it = My32::make(23);
        let value: u32 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_u64() {
        let it = My32::make(23);
        let value: u64 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_u128() {
        let it = My32::make(23);
        let value: u128 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_i64() {
        let it = My32::make(23);
        let value: i64 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_i128() {
        let it = My32::make(23);
        let value: i128 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_try_into_isize() {
        let it = My32::make(23);
        let value: Result<isize, ()> = it.try_into();
        assert_eq!(value, Ok(23));
    }

    #[test]
    fn implements_try_into_u8() {
        let it = My32::make(256);
        let value: Result<u8, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn implements_try_into_u16() {
        let it = My32::make(65_536);
        let value: Result<u16, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn implements_try_into_i8() {
        let it = My32::make(128);
        let value: Result<i8, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn implements_try_into_i16() {
        let it = My32::make(32_768);
        let value: Result<i16, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn implements_try_into_i32() {
        let it = My32::make(2_147_483_648);
        let value: Result<i32, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    fn bitwise_ands_friend_value() {
        let it = My32::make(0b10111);
        assert_eq!((it & 0b10111).raw(), 0b10111);
    }

    #[test]
    fn bitwise_ors_friend_value() {
        let it = My32::make(0b10001);
        assert_eq!((it | 0b00110).raw(), 0b10111);
    }

    #[test]
    fn bitwise_xors_friend_value() {
        let it = My32::make(0b10111);
        assert_eq!((it ^ 0b01000).raw(), 0b11111);
    }

    #[test]
    fn shifts_left() {
        let it = My32::make(23);
        assert_eq!((it << 1).raw(), 46);
    }

    #[test]
    fn shifts_right() {
        let it = My32::make(46);
        assert_eq!((it >> 1).raw(), 23);
    }

    #[test]
    fn bitwise_nots() {
        let it = My32::make(0);
        assert_eq!((!it).raw(), u32::MAX);
    }

    #[test]
    fn adds_assign() {
        let it = My32::make(20);
        let mut value = it;
        let it = My32::make(3);
        value += it;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn subtracts_assign() {
        let it = My32::make(26);
        let mut value = it;
        let it = My32::make(3);
        value -= it;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn multiplies_assign() {
        let it = My32::make(23);
        let mut value = it;
        let it = My32::make(3);
        value *= it;
        assert_eq!(value.raw(), 69);
    }

    #[test]
    fn divides_assign() {
        let it = My32::make(69);
        let mut value = it;
        let it = My32::make(3);
        value /= it;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn takes_remainder_assign() {
        let it = My32::make(70);
        let mut value = it;
        let it = My32::make(47);
        value %= it;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn bitwise_ands_assign() {
        let it = My32::make(0b10111);
        let mut value = it;
        let it = My32::make(0b10111);
        value &= it;
        assert_eq!(value.raw(), 0b10111);
    }

    #[test]
    fn bitwise_ors_assign() {
        let it = My32::make(0b10001);
        let mut value = it;
        let it = My32::make(0b00110);
        value |= it;
        assert_eq!(value.raw(), 0b10111);
    }

    #[test]
    fn shifts_left_assign() {
        let it = My32::make(23);
        let mut value = it;
        value <<= 1;
        assert_eq!(value.raw(), 46);
    }

    #[test]
    fn shifts_right_assign() {
        let it = My32::make(46);
        let mut value = it;
        value >>= 1;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn implements_eq() {
        fn requires_eq<T: Eq>() {}
        requires_eq::<My32>();
    }

    #[test]
    fn implements_ord() {
        let it = My32::make(24);
        let other = it;
        let it = My32::make(23);
        assert_eq!(it.cmp(&other), Ordering::Less);
    }

    #[test]
    fn const_mode_display_preserves_signed_decimal_formatting() {
        let negative = MyI32::make(-23);
        assert_eq!(format!("{}", negative), "-23");
        assert_eq!(format!("{:+06}", negative), "-00023");
        assert_eq!(format!("{:+06}", MyI32::make(23)), "+00023");
        assert_eq!(format!("{:_<7}", negative), "-23____");
        assert_eq!(format!("{:*^7}", negative), "**-23**");
        assert_eq!(format!("{:>6}", negative), "   -23");
    }

    #[test]
    fn const_mode_display_uses_configured_raw_accessor() {
        let value = EncodedDisplay::make(123);
        assert_eq!(value.0, 123);
        assert_eq!(format!("{}", value), "23");
        assert_eq!(format!("{:+06}", value), "+00023");
    }

    #[test]
    fn formats_debug() {
        let it = My32::make(23);
        assert_eq!(format!("{:?}", it), "My32(23)");
    }

    #[test]
    fn formats_binary() {
        let it = My32::make(23);
        assert_eq!(format!("{:b}", it), "10111");
    }

    #[test]
    fn formats_octal() {
        let it = My32::make(23);
        assert_eq!(format!("{:o}", it), "27");
    }

    #[test]
    fn formats_lower_hex() {
        let it = My32::make(23);
        assert_eq!(format!("{:x}", it), "17");
    }

    #[test]
    fn formats_upper_hex() {
        let it = My32::make(23);
        assert_eq!(format!("{:X}", it), "17");
    }

    #[test]
    fn preserves_transparent_layout() {
        assert_eq!(size_of::<My32>(), size_of::<u32>());
    }

    #[cfg(test)]
    mod non_const_trust {
        use super::assert_panics;

        struct DefaultSource(Box<u8>);
        struct CheckedSource(Box<u8>);
        struct TrustSource(Box<u8>);
        struct MathOnlySource(Box<u8>);
        struct BitOnlySource(Box<u8>);
        struct NumericSource(Box<u8>);

        fn default_raw(source: DefaultSource) -> u8 {
            *source.0
        }

        fn checked_raw(source: CheckedSource) -> u8 {
            *source.0
        }

        fn trusted_raw(source: TrustSource) -> u8 {
            *source.0
        }

        fn math_only_raw(source: MathOnlySource) -> u8 {
            *source.0
        }

        fn bit_only_raw(source: BitOnlySource) -> u8 {
            *source.0
        }

        fn numeric_raw(source: NumericSource) -> u8 {
            *source.0
        }

        const fn is_even(value: u8) -> bool {
            value % 2 == 0
        }

        #[typekin::integral(
            konst = false,
            friends = [
                default_raw(DefaultSource) -> Make,
                checked_raw(CheckedSource) -> [Make],
                trusted_raw(TrustSource) -> [Make, Math, Bit, Trust],
                math_only_raw(MathOnlySource) -> [Math, Trust],
                bit_only_raw(BitOnlySource) -> [Bit, Trust],
                numeric_raw(NumericSource) -> [Numeric, Trust],
            ],
            valid = is_even,
            in = 2..=10,
        )]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        struct Number(u8);

        #[typekin::integral(
            konst = false,
            friends = [
                _(TrustSelfNumber) -> [Make, Trust],
                _(u8) -> [Make, Trust],
            ],
            valid = is_even,
            in = 2..=10,
        )]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        struct TrustSelfNumber(u8);

        #[test]
        fn same_type_construction_requires_explicit_target_trust_to_bypass_validation()
         {
            for raw in [
                3, 12,
            ] {
                let untrusted_target = Number::of(TrustSource(Box::new(raw)));
                assert_panics(|| Number::of(untrusted_target));

                let trusted_target = TrustSelfNumber::of(raw);
                assert_eq!(TrustSelfNumber::of(trusted_target).raw(), raw);
                assert_eq!(TrustSelfNumber::try_make(raw), Err(raw));
            }
        }

        #[test]
        fn untrusted_owned_friends_validate_scalar_and_list_make() {
            for raw in [
                3, 12,
            ] {
                assert_panics(|| Number::of(DefaultSource(Box::new(raw))));
                assert_panics(|| Number::of(CheckedSource(Box::new(raw))));
            }
            assert_eq!(Number::of(DefaultSource(Box::new(4))).raw(), 4);
            assert_eq!(Number::of(CheckedSource(Box::new(4))).raw(), 4);
        }

        #[test]
        fn trusted_owned_friend_bypasses_only_construction_validation() {
            for raw in [
                3, 12,
            ] {
                let source = TrustSource(Box::new(raw));
                assert_eq!(Number::of(source).raw(), raw);
                assert_eq!(Number::try_make(raw), Err(raw));
            }
            assert_panics(|| {
                let _ = Number::try_make(4).unwrap() + TrustSource(Box::new(1));
            });
            assert_panics(|| {
                let _ = Number::try_make(4).unwrap() | TrustSource(Box::new(1));
            });
            assert_panics(|| {
                let _ =
                    Number::try_make(6).unwrap() + Number::try_make(6).unwrap();
            });
        }

        #[test]
        fn trusted_owned_assignment_results_validate_before_mutating() {
            for rhs in [
                1, 8,
            ] {
                let mut number = Number::try_make(4).unwrap();
                assert_panics(move || number += TrustSource(Box::new(rhs)));
                assert_eq!(number.raw(), 4);

                assert_panics(move || number |= TrustSource(Box::new(rhs)));
                assert_eq!(number.raw(), 4);
            }
        }

        #[test]
        fn trusted_operation_only_friends_consume_sources_and_validate_results()
        {
            let math_source = MathOnlySource(Box::new(2));
            assert_eq!((Number::try_make(4).unwrap() + math_source).raw(), 6);
            let bit_source = BitOnlySource(Box::new(2));
            assert_eq!((Number::try_make(4).unwrap() | bit_source).raw(), 6);

            for rhs in [
                1, 8,
            ] {
                assert_panics(|| {
                    let _ = Number::try_make(4).unwrap()
                        + MathOnlySource(Box::new(rhs));
                });
                assert_panics(|| {
                    let _ = Number::try_make(4).unwrap()
                        | BitOnlySource(Box::new(rhs));
                });

                let mut number = Number::try_make(4).unwrap();
                assert_panics(move || number += MathOnlySource(Box::new(rhs)));
                assert_eq!(number.raw(), 4);
                assert_panics(move || number |= BitOnlySource(Box::new(rhs)));
                assert_eq!(number.raw(), 4);
            }
            assert_eq!(Number::try_make(3), Err(3));
            assert_eq!(Number::try_make(12), Err(12));
        }

        #[test]
        fn numeric_friend_enables_construction_math_and_bit_operations() {
            let construction_source = NumericSource(Box::new(2));
            assert_eq!(Number::of(construction_source).raw(), 2);

            let math_source = NumericSource(Box::new(2));
            assert_eq!((Number::try_make(4).unwrap() + math_source).raw(), 6);

            let bit_source = NumericSource(Box::new(2));
            assert_eq!((Number::try_make(4).unwrap() | bit_source).raw(), 6);
        }
    }

    #[cfg(test)]
    mod non_const_display {
        #[typekin::integral(konst = false, with = [display])]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        struct Number(i32);

        #[typekin::integral(
            konst = false,
            with = [display],
            without = [fn_conv_raw],
            get_raw = Self::decoded,
        )]
        #[repr(transparent)]
        #[derive(Copy, Clone)]
        struct EncodedDisplay(i32);

        impl EncodedDisplay {
            const fn decoded(self) -> i32 {
                self.0 - 100
            }
        }

        #[test]
        fn display_is_signed_decimal_and_preserves_formatter_flags() {
            let negative = Number::make(-23);
            assert_eq!(format!("{}", negative), "-23");
            assert_eq!(format!("{}", Number::make(i32::MIN)), "-2147483648");
            assert_eq!(format!("{:+}", Number::make(23)), "+23");
            assert_eq!(format!("{:+06}", Number::make(23)), "+00023");
            assert_eq!(format!("{:06}", negative), "-00023");
            assert_eq!(format!("{:_<7}", negative), "-23____");
            assert_eq!(format!("{:*^7}", negative), "**-23**");
            assert_eq!(format!("{:>6}", negative), "   -23");
        }

        #[test]
        fn display_uses_configured_raw_accessor() {
            let value = EncodedDisplay::make(123);
            assert_eq!(value.0, 123);
            assert_eq!(format!("{}", value), "23");
            assert_eq!(format!("{:+06}", value), "+00023");
        }
    }

    pub fn assert_panics<T>(f: impl FnOnce() -> T + std::panic::UnwindSafe) {
        std::panic::set_hook(Box::new(move |_| {}));
        let result = std::panic::catch_unwind(f);
        let _ = std::panic::take_hook();
        assert!(result.is_err());
    }
}
