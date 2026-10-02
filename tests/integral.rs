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
            in = 2..=4 + 8..10,
            valid = accepts_ranged,
        )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct Ranged(u8);

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
        assert!(
            std::panic::catch_unwind(|| {
                let _ = TrustEven::try_make(4).unwrap() + 1u8;
            })
            .is_err()
        );
        assert!(
            std::panic::catch_unwind(|| {
                let _ = TrustEven::try_make(4).unwrap() | 1u8;
            })
            .is_err()
        );
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
    fn range_validation_is_const() {
        assert_eq!(RANGED_NUMBER.raw(), 3);
    }

    #[test]
    fn range_validation_checks_friended_construction_and_math() {
        assert!(std::panic::catch_unwind(|| Ranged::of(5u8)).is_err());

        let value = Ranged::try_make(4).unwrap();
        assert!(std::panic::catch_unwind(|| value + 1u8).is_err());
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
}

#[cfg(test)]
mod non_const_trust {
    struct DefaultSource(Box<u8>);
    struct CheckedSource(Box<u8>);
    struct TrustSource(Box<u8>);
    struct MathOnlySource(Box<u8>);
    struct BitOnlySource(Box<u8>);

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
        for raw in [3, 12] {
            let untrusted_target = Number::of(TrustSource(Box::new(raw)));
            assert!(
                std::panic::catch_unwind(|| Number::of(untrusted_target))
                    .is_err()
            );

            let trusted_target = TrustSelfNumber::of(raw);
            assert_eq!(TrustSelfNumber::of(trusted_target).raw(), raw);
            assert_eq!(TrustSelfNumber::try_make(raw), Err(raw));
        }
    }

    #[test]
    fn untrusted_owned_friends_validate_scalar_and_list_make() {
        for raw in [3, 12] {
            assert!(
                std::panic::catch_unwind(|| {
                    Number::of(DefaultSource(Box::new(raw)))
                })
                .is_err()
            );
            assert!(
                std::panic::catch_unwind(|| {
                    Number::of(CheckedSource(Box::new(raw)))
                })
                .is_err()
            );
        }
        assert_eq!(Number::of(DefaultSource(Box::new(4))).raw(), 4);
        assert_eq!(Number::of(CheckedSource(Box::new(4))).raw(), 4);
    }

    #[test]
    fn trusted_owned_friend_bypasses_only_construction_validation() {
        for raw in [3, 12] {
            let source = TrustSource(Box::new(raw));
            assert_eq!(Number::of(source).raw(), raw);
            assert_eq!(Number::try_make(raw), Err(raw));
        }
        assert!(
            std::panic::catch_unwind(|| {
                let _ = Number::try_make(4).unwrap() + TrustSource(Box::new(1));
            })
            .is_err()
        );
        assert!(
            std::panic::catch_unwind(|| {
                let _ = Number::try_make(4).unwrap() | TrustSource(Box::new(1));
            })
            .is_err()
        );
        assert!(
            std::panic::catch_unwind(|| {
                let _ =
                    Number::try_make(6).unwrap() + Number::try_make(6).unwrap();
            })
            .is_err()
        );
    }

    #[test]
    fn trusted_owned_assignment_results_validate_before_mutating() {
        for rhs in [1, 8] {
            let mut number = Number::try_make(4).unwrap();
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    number += TrustSource(Box::new(rhs));
                }))
                .is_err()
            );
            assert_eq!(number.raw(), 4);

            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    number |= TrustSource(Box::new(rhs));
                }))
                .is_err()
            );
            assert_eq!(number.raw(), 4);
        }
    }

    #[test]
    fn trusted_operation_only_friends_consume_sources_and_validate_results() {
        let math_source = MathOnlySource(Box::new(2));
        assert_eq!((Number::try_make(4).unwrap() + math_source).raw(), 6);
        let bit_source = BitOnlySource(Box::new(2));
        assert_eq!((Number::try_make(4).unwrap() | bit_source).raw(), 6);

        for rhs in [1, 8] {
            assert!(
                std::panic::catch_unwind(|| {
                    let _ = Number::try_make(4).unwrap()
                        + MathOnlySource(Box::new(rhs));
                })
                .is_err()
            );
            assert!(
                std::panic::catch_unwind(|| {
                    let _ = Number::try_make(4).unwrap()
                        | BitOnlySource(Box::new(rhs));
                })
                .is_err()
            );

            let mut number = Number::try_make(4).unwrap();
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    number += MathOnlySource(Box::new(rhs));
                }))
                .is_err()
            );
            assert_eq!(number.raw(), 4);
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    number |= BitOnlySource(Box::new(rhs));
                }))
                .is_err()
            );
            assert_eq!(number.raw(), 4);
        }
        assert_eq!(Number::try_make(3), Err(3));
        assert_eq!(Number::try_make(12), Err(12));
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
