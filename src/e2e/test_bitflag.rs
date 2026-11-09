use crate::fx_bitflag::*;
use crate::*;

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use serde_test::{
        Compact,
        Configure,
        Readable,
        Token,
        assert_de_tokens_error,
        assert_tokens,
    };

    use super::*;
    use crate::{
        accepts_copy_clone,
        accepts_debug,
        accepts_hash,
        assert_no_associated_fn,
        assert_not_impl,
        assert_panics,
        unsafe_assert_panics,
    };

    #[test]
    fn raw_identifier_enum_builds_its_value_type() {
        let one = r#Type::One;
        let two = r#Type::Two;
        let both: TypeValue = one.inserted(two);

        assert_eq!(both.raw(), 0b11);
    }

    #[test]
    fn value_empty() {
        let it = Thingy::empty();
        assert_eq!(it.raw(), 0b0);
    }

    #[test]
    fn nested_without_fn_removes_a_value_method() {
        let value = NoCountOnesFlag::all();

        crate::assert_no_method!(NoCountOnesFlagValue, count_ones);
        assert_eq!(value.count_zeros(), 6, "{value:?}");
    }

    #[test]
    fn value_named_constants_match_enum_variants() {
        assert_eq!(THINGY_READ_VALUE.raw(), Thingy::ReadThing.raw());
        assert_eq!(ThingyValue::FirstThing.raw(), Thingy::FirstThing.raw());
        assert_eq!(ThingyValue::WritersBlock.raw(), Thingy::WritersBlock.raw());
        assert_eq!(ThingyValue::Execute.raw(), Thingy::Execute.raw());
    }

    #[test]
    fn full_bitflag_generates_common_enum_traits() {
        fn accepts_traits<
            T: Copy + Clone + core::fmt::Debug + Eq + Ord + core::hash::Hash,
        >() {
        }

        accepts_traits::<FullyGeneratedFlag>();
        assert_eq!(FULLY_GENERATED_FLAG_CLONE.raw(), 0b0001);
    }

    #[test]
    fn formats_debug_for_flags_and_values() {
        assert_eq!(format!("{:?}", FullyGeneratedFlag::Read), "Read");
        assert_eq!(
            format!("{:?}", FullyGeneratedFlag::Read.into_value()),
            "FullyGeneratedFlagValue(1)",
        );
    }

    #[test]
    fn bitflag_copy_and_clone_can_be_supplied_manually() {
        fn accepts_traits<T: Copy + Clone>() {}

        accepts_traits::<ManuallyDerivedFlag>();
        assert_eq!(ManuallyDerivedFlag::Read.raw(), 0b0001);
    }

    #[test]
    fn generated_values_preserve_their_u8_layout() {
        fn assert_layout<T>() {
            assert_eq!(size_of::<T>(), size_of::<u8>());
            assert_eq!(align_of::<T>(), align_of::<u8>());
        }

        assert_layout::<ThingyValue>();
        assert_layout::<Thingy1Holder>();
        assert_layout::<Thingies2>();
    }

    #[test]
    fn from_bits_accepts_only_known_bits() {
        assert_eq!(
            ThingyValue::from_bits(0b011).map(ThingyValue::raw),
            Some(0b011)
        );
        assert_eq!(ThingyValue::from_bits(0b1011), None);
    }

    #[test]
    fn from_bits_retain_preserves_unknown_bits() {
        assert_eq!(ThingyValue::from_bits_retain(0b1011).raw(), 0b1011);
    }

    #[test]
    fn from_bits_truncate_discards_unknown_bits() {
        assert_eq!(ThingyValue::from_bits_truncate(0b1011).raw(), 0b011);
    }

    #[test]
    fn value_reports_known_and_unknown_bits() {
        let mixed = ThingyValue::from_bits_retain(0b1011);
        assert_eq!(mixed.known_bits(), 0b011);
        assert_eq!(mixed.unknown_bits(), 0b1000);
        assert!(mixed.contains_unknown_bits());
    }

    #[test]
    fn value_extracts_known_and_unknown_bits() {
        let mixed = ThingyValue::from_bits_retain(0b1011);
        assert_eq!(mixed.into_known_bits().raw(), 0b011);
        assert_eq!(mixed.into_unknown_bits().raw(), 0b1000);
    }

    #[test]
    fn known_bit_check_rejects_unknown_bits() {
        let mixed = ThingyValue::from_bits_retain(0b1011);
        assert_eq!(mixed.try_as_known_bits_only(), Err(mixed));
        assert_eq!(
            ThingyValue::from_bits_retain(0b011)
                .try_as_known_bits_only()
                .map(ThingyValue::raw),
            Ok(0b011)
        );
    }

    #[test]
    fn all_unknown_contains_only_unknown_bits() {
        let value = ThingyValue::all_unknown();
        assert_eq!(value.known_bits(), 0);
        assert!(value.contains_unknown_bits());
    }

    #[test]
    fn value_converts_to_a_flag_only_for_exact_named_bits() {
        assert_eq!(ThingyValue::ReadThing.into_flag(), Ok(Thingy::ReadThing));
        assert_eq!(
            ThingyValue::from_bits_retain(0b011).into_flag(),
            Err(ThingyValue::from_bits_retain(0b011))
        );
        assert_eq!(
            ThingyValue::from_bits_retain(0b1000).into_flag(),
            Err(ThingyValue::from_bits_retain(0b1000))
        );
    }

    #[test]
    fn all_flags_iterates_declared_flags() {
        assert_eq!(
            Thingy::iter().collect::<Vec<_>>(),
            [
                Thingy::FirstThing,
                Thingy::ReadThing,
                Thingy::WritersBlock,
                Thingy::Execute,
            ]
        );
    }

    #[test]
    fn all_values_iterates_declared_values() {
        assert_eq!(
            Thingy::iter_values()
                .map(ThingyValue::raw)
                .collect::<Vec<_>>(),
            [
                0,
                1,
                2,
                4
            ]
        );
    }

    #[test]
    fn value_iter_known_flags_includes_zero_and_present_flags() {
        let mixed = ThingyValue::from_bits_retain(0b1011);
        assert_eq!(
            mixed.iter_known_flags().collect::<Vec<_>>(),
            [
                Thingy::FirstThing,
                Thingy::ReadThing,
                Thingy::WritersBlock,
            ]
        );
    }

    #[test]
    fn value_iter_includes_unknown_bits() {
        let mixed = ThingyValue::from_bits_retain(0b1011);
        assert_eq!(
            mixed.iter().map(ThingyValue::raw).collect::<Vec<_>>(),
            [
                0,
                1,
                2,
                8
            ]
        );
    }

    #[test]
    fn value_iter_names_includes_present_known_flags() {
        let mixed = ThingyValue::from_bits_retain(0b1011);
        assert_eq!(
            mixed
                .iter_names()
                .map(|(name, value)| (name, value.raw()))
                .collect::<Vec<_>>(),
            [
                ("FirstThing", 0),
                ("ReadThing", 1),
                ("WritersBlock", 2)
            ]
        );
    }

    #[test]
    fn defined_names_iterates_all_declared_flags() {
        assert_eq!(
            ThingyValue::iter_defined_names()
                .map(|(name, value)| (name, value.raw()))
                .collect::<Vec<_>>(),
            [
                ("FirstThing", 0),
                ("ReadThing", 1),
                ("WritersBlock", 2),
                ("Execute", 4),
            ]
        );
    }

    #[test]
    fn equal_names_iterates_matching_declarations() {
        assert_eq!(
            ThingyValue::FirstThing
                .iter_equal_names()
                .collect::<Vec<_>>(),
            ["FirstThing"]
        );
    }

    #[test]
    fn value_predicates_report_flag_membership() {
        let read = ThingyValue::ReadThing;
        let write = ThingyValue::WritersBlock;

        assert!(read.contains(Thingy::ReadThing));
        assert!(read.contains_all(read));
        assert!(read.contains_any(read));
        assert!(!read.intersects(write));
    }

    #[test]
    fn value_non_mutating_set_operations_return_expected_bits() {
        let read = ThingyValue::ReadThing;
        let write = ThingyValue::WritersBlock;

        assert_eq!(read.inserted(write).raw(), 0b011);
        assert_eq!(read.with(write).raw(), 0b011);
        assert_eq!(read.removed(read).raw(), 0);
        assert_eq!(read.without(read).raw(), 0);
        assert_eq!(read.toggled(write).raw(), 0b011);
        assert_eq!(read.intersection(write).raw(), 0);
        assert_eq!(read.union(write).raw(), 0b011);
        assert_eq!(read.difference(write).raw(), 1);
        assert_eq!(read.symmetric_difference(write).raw(), 0b011);
        assert_eq!(read.complemented().raw(), 0b110);
        assert_eq!((!read).raw(), 0b110);
    }

    #[test]
    fn all_bits_predicates_distinguish_unknown_bits() {
        let unknown = ThingyValue::from_bits_retain(0b1000);
        assert!(ThingyValue::all().is_all());
        assert!(ThingyValue::all().is_exactly_all_known_bits());
        assert!(ThingyValue::all().inserted(unknown).is_all());
        assert!(
            !ThingyValue::all()
                .inserted(unknown)
                .is_exactly_all_known_bits()
        );
    }

    #[test]
    fn value_mutating_set_operations_update_bits() {
        let read = ThingyValue::ReadThing;
        let write = ThingyValue::WritersBlock;
        let execute = ThingyValue::Execute;
        let mut value = read;
        value.insert(write);
        assert_eq!(value.raw(), 0b011);
        value.remove(read);
        assert_eq!(value.raw(), 0b010);
        value.toggle(execute);
        assert_eq!(value.raw(), 0b110);
        value.set(write, false);
        assert_eq!(value.raw(), 0b100);
        value.set(read, true);
        assert_eq!(value.raw(), 0b101);
        value.unset(read);
        assert_eq!(value.raw(), 0b100);
        value.complement();
        assert_eq!(value.raw(), 0b011);
        value.clear();
        assert!(value.is_empty());
    }

    #[test]
    fn value_truncation_removes_unknown_bits() {
        let read = ThingyValue::ReadThing;
        let unknown = ThingyValue::from_bits_retain(0b1000);
        let mut mixed = read.inserted(unknown);
        mixed.truncate();
        assert_eq!(mixed.raw(), 1);
        let mut mixed = read.inserted(unknown);
        mixed.truncate_into_known_bits();
        assert_eq!(mixed.raw(), 1);
        assert_eq!(read.inserted(unknown).truncated_into_known_bits().raw(), 1);
    }

    #[test]
    fn display_formats_enum_variant_names() {
        assert_eq!(format!("{}", Thingy::FirstThing), "FirstThing");
        assert_eq!(format!("{}", Thingy::Execute), "Execute");
        assert_eq!(format!("{:.4}", Thingy::Execute), "Exec");
        assert_eq!(format!("{:>10.4}", Thingy::Execute), "      Exec");
        assert_eq!(format!("{:*^11}", Thingy::Execute), "**Execute**");
        assert_eq!(format!("{:+05}", Thingy::Execute), "Execute");
    }

    #[test]
    fn display_formats_combined_and_unknown_value_bits() {
        let combined = Thingy::ReadThing | Thingy::WritersBlock;
        assert_eq!(format!("{}", combined), "ReadThing | WritersBlock");
        assert_eq!(format!("{:05}", combined), "ReadThing | WritersBlock");
        assert_eq!(format!("{}", Thingy::from_bits_retain(0b1000)), "0x8");
        assert_eq!(
            format!("{}", Thingy::from_bits_retain(0b1011)),
            "ReadThing | WritersBlock | 0x8"
        );
    }

    #[test]
    fn value_text_parses_empty_known_composite_and_unknown_bits() {
        assert_eq!(ThingyValue::empty().to_string(), "");
        assert_eq!(ThingyValue::from_str(""), Ok(ThingyValue::empty()));

        assert_eq!(ThingyValue::ReadThing.to_string(), "ReadThing");
        assert_eq!(
            ThingyValue::from_str("ReadThing"),
            Ok(ThingyValue::ReadThing),
        );
        assert_eq!(Thingy::from_str("ReadThing"), Ok(Thingy::ReadThing));

        let combined = ThingyValue::ReadThing | ThingyValue::WritersBlock;
        assert_eq!(combined.to_string(), "ReadThing | WritersBlock");
        assert_eq!(
            ThingyValue::from_str(" ReadThing | WritersBlock "),
            Ok(combined),
        );

        let unknown = ThingyValue::from_bits_retain(0b1000);
        assert_eq!(unknown.to_string(), "0x8");
        assert_eq!(ThingyValue::from_str("0x8"), Ok(unknown));
        assert_eq!(
            ThingyValue::from_str("ReadThing | 0x8"),
            Ok(ThingyValue::ReadThing | unknown),
        );

        let composite = CompositeFlagValue::from_str("ReadWrite").unwrap();
        assert_eq!(composite.raw(), 0b0011);
        assert_eq!(composite.to_string(), "Read | Write");
    }

    #[test]
    fn value_text_rejects_malformed_unknown_overflow_and_invalid_values() {
        assert_eq!(ThingyValue::from_str("ReadThing || WritersBlock"), Err(()));
        assert_eq!(ThingyValue::from_str("Unknown"), Err(()));
        assert_eq!(ThingyValue::from_str("0x"), Err(()));
        assert_eq!(ThingyValue::from_str("0x100"), Err(()));
        assert_eq!(Thingy1Holder::from_str("0x8"), Err(()));
    }

    #[test]
    fn value_text_round_trips_known_unknown_and_all_bits() {
        fn assert_round_trip(bits: u8) {
            let value = ThingyValue::from_bits_retain(bits);
            assert_eq!(
                ThingyValue::from_str(&value.to_string()),
                Ok(value),
                "{bits:#x}",
            );
        }

        assert_round_trip(0);
        assert_round_trip(0b0001);
        assert_round_trip(0b0010);
        assert_round_trip(0b0100);
        assert_round_trip(0b1000);
        assert_round_trip(0b1_0000);
        assert_round_trip(0b10_0000);
        assert_round_trip(0b100_0000);
        assert_round_trip(u8::MAX);
    }

    #[test]
    fn bitflag_serde_auto_uses_text_for_readable_and_bits_for_compact_formats()
    {
        let value = AutoSerdeFlagValue::from_bits_retain(0b1011);
        assert_tokens(&value.readable(), &[Token::Str("Read | Write | 0x8")]);
        assert_tokens(&value.compact(), &[Token::U8(0b1011)]);
        assert_tokens(&AutoSerdeFlag::Read.readable(), &[Token::Str("Read")]);
        assert_tokens(&AutoSerdeFlag::Read.compact(), &[Token::U8(1)]);
        assert_de_tokens_error::<Compact<AutoSerdeFlag>>(
            &[Token::U8(3)],
            "raw value is not a declared flag",
        );
    }

    #[test]
    fn bitflag_serde_name_and_value_modes_are_forced() {
        let named = NameSerdeFlag::Read | NameSerdeFlag::Write;
        assert_tokens(&named.readable(), &[Token::Str("Read | Write")]);
        assert_tokens(&named.compact(), &[Token::Str("Read | Write")]);

        let valued = ValueSerdeFlag::Read | ValueSerdeFlag::Write;
        assert_tokens(&valued.readable(), &[Token::U8(3)]);
        assert_tokens(&valued.compact(), &[Token::U8(3)]);
    }

    #[test]
    fn bitflag_serde_retains_unknown_bits_and_revalidates_values() {
        let unknown = AutoSerdeFlagValue::from_bits_retain(0b1000);
        assert_tokens(&unknown.compact(), &[Token::U8(0b1000)]);
        assert_de_tokens_error::<Compact<ValidatedSerdeFlagValue>>(
            &[Token::U8(0b1000)],
            "invalid value",
        );
        assert_de_tokens_error::<Readable<ValidatedSerdeFlagValue>>(
            &[Token::Str("0x8")],
            "invalid value",
        );
    }

    #[test]
    fn value_from_owned_integral_friend() {
        let raw = 0b011u8;
        let it = ThingyValue::of(raw);
        assert_eq!(it.raw(), raw);
    }

    #[test]
    fn enum_left_bitwise_owned_generated_value_friend() {
        let rhs = Thingy::WritersBlock.into_value();
        let it = Thingy::ReadThing | rhs;
        assert_eq!(it.raw(), 0b011);
    }

    #[test]
    fn enum_left_bitwise_owned_configured_friend() {
        let rhs = 0b010u8;
        let it = Thingy::ReadThing | rhs;
        assert_eq!(it.raw(), 0b011);
    }

    #[test]
    fn value_left_bitwise_owned_generated_enum_friend() {
        let value = Thingy::ReadThing.into_value();
        let it = value | Thingy::WritersBlock;
        assert_eq!(it.raw(), 0b011);
    }

    #[test]
    fn suffix() {
        let all = Thingy1Holder::all();
        assert_eq!(all.raw(), 0b111);
        assert_eq!(Thingy1Holder::Execute.raw(), Thingy1::Execute.raw());
    }

    #[test]
    fn value_bitxor_assignment_is_generated_by_default() {
        let rhs = ThingyValue::WritersBlock;
        let mut value = ThingyValue::from_bits_retain(0b011);
        value ^= rhs;

        assert_eq!(value.raw(), 0b001);
    }

    #[test]
    fn nested_without_removes_value_bitxor_assignment_only() {
        let read = XorUnassignedFlagValue::Read;
        let write = XorUnassignedFlagValue::Write;
        let combined = read ^ write;

        assert_eq!(combined.raw(), 0b11);
        assert_not_impl!(
            XorUnassignedFlagValue: core::ops::BitXorAssign<XorUnassignedFlagValue>
        );
    }

    #[test]
    fn nested_integral_ranges_reject_invalid_bits() {
        assert_panics(|| Thingy1::from_bits_retain(0b1000));
    }

    #[test]
    fn value_name() {
        let _ = Thingies2::all();
        assert_eq!(Thingies2::ReadThing.raw(), Thingy2::ReadThing.raw());
    }

    #[test]
    fn bitflag_value_apis_are_const_usable() {
        assert_eq!(RETAINED.raw(), 0b1011);
        assert_eq!(RETAINED.known_bits(), 0b011);
        assert_eq!(RETAINED.unknown_bits(), 0b1000);
        assert_eq!(TRUNCATED.raw(), 0b011);
        assert_eq!(SET_ALGEBRA.raw(), 0b111);
        assert_eq!(FROM_ENUM.raw(), 0b001);
        assert_eq!(ENUM_NEGATED.raw(), 0b110);
        assert_eq!(VALUE_NEGATED.raw(), 0b110);
    }

    #[test]
    fn generated_value_preserves_the_u8_layout_const() {
        assert_eq!(size_of::<ConstFlagValue>(), size_of::<u8>());
        assert_eq!(align_of::<ConstFlagValue>(), align_of::<u8>());
    }

    #[test]
    fn root_display_overrides_nested_disable_in_either_order() {
        assert_eq!(format!("{}", RootBeforeNested::Read), "Read");
        assert_eq!(
            format!("{}", RootBeforeNested::Read | RootBeforeNested::Write),
            "Read | Write"
        );
        assert_eq!(format!("{}", NestedBeforeRoot::Write), "Write");
        assert_eq!(
            format!("{}", NestedBeforeRoot::Read | NestedBeforeRoot::Write),
            "Read | Write"
        );
    }

    #[test]
    fn nested_display_formats_values_without_root_opt_in() {
        assert_eq!(
            format!("{}", NestedOnly::Read | NestedOnly::Write),
            "Read | Write"
        );
        assert_eq!(
            format!("{:+05}", NestedOnly::from_bits_retain(0b1000)),
            "0x8"
        );
    }

    #[test]
    fn disabling_root_preserves_nested_value_opt_in_in_either_order() {
        assert_eq!(
            format!(
                "{}",
                DisabledRootBeforeNested::Read
                    | DisabledRootBeforeNested::Write
            ),
            "Read | Write"
        );
        assert_eq!(
            format!(
                "{}",
                NestedBeforeDisabledRoot::Read
                    | NestedBeforeDisabledRoot::Write
            ),
            "Read | Write"
        );
        assert_eq!(
            format!(
                "{:04}",
                DisabledRootBeforeNested::from_bits_retain(0b1000)
            ),
            "0x8"
        );
        assert_eq!(
            format!(
                "{:04}",
                NestedBeforeDisabledRoot::from_bits_retain(0b1000)
            ),
            "0x8"
        );
    }

    #[test]
    fn generated_values_preserve_their_u8_layout_on_disabled_ordered() {
        fn assert_layout<T>() {
            assert_eq!(size_of::<T>(), size_of::<u8>());
            assert_eq!(align_of::<T>(), align_of::<u8>());
        }

        assert_layout::<RootBeforeNestedValue>();
        assert_layout::<NestedBeforeRootValue>();
        assert_layout::<NestedOnlyValue>();
        assert_layout::<DisabledRootBeforeNestedValue>();
        assert_layout::<NestedBeforeDisabledRootValue>();
    }

    #[test]
    fn unsigned_display_uses_retain_text() {
        assert_eq!(DisplayWide::from_bits_retain(0).to_string(), "");
        assert_eq!(
            DisplayWide::from_bits_retain(0x8000_0001).to_string(),
            "Low | High",
        );
        assert_eq!(
            DisplayWide::from_bits_retain(u32::MAX).to_string(),
            "Low | High | 0x7ffffffe",
        );
    }

    #[test]
    fn display_ignores_numeric_formatter_options() {
        let value = DisplayWide::from_bits_retain(u32::MAX);
        assert_eq!(format!("{value:+014}"), "Low | High | 0x7ffffffe");
        assert_eq!(format!("{value:*^15.2}"), "Low | High | 0x7ffffffe");
    }

    #[test]
    fn signed_display_uses_unsigned_hex_for_remaining_bits() {
        assert_eq!(DisplaySigned::from_bits_retain(0).to_string(), "");
        assert_eq!(
            DisplaySigned::from_bits_retain(i16::MIN).to_string(),
            "0x8000",
        );
        assert_eq!(
            DisplaySigned::from_bits_retain(-123).to_string(),
            "Positive | 0xff84",
        );
        assert_eq!(
            DisplaySigned::from_bits_retain(i16::MAX).to_string(),
            "Positive | 0x7ffe",
        );
        assert_eq!(
            DisplaySigned::from_bits_retain(-1).to_string(),
            "Positive | 0xfffe",
        );
    }

    #[test]
    fn signed_text_round_trips() {
        for raw in [
            0,
            i16::MIN,
            -123,
            i16::MAX,
            -1,
        ] {
            let value = DisplaySigned::from_bits_retain(raw);
            assert_eq!(
                DisplaySignedValue::from_str(&value.to_string())
                    .map(DisplaySignedValue::bits),
                Ok(raw),
            );
        }
    }

    #[test]
    fn generated_values_preserve_their_declared_integer_layout() {
        assert_eq!(size_of::<DisplayWideValue>(), size_of::<u32>());
        assert_eq!(align_of::<DisplayWideValue>(), align_of::<u32>());
        assert_eq!(size_of::<DisplaySignedValue>(), size_of::<i16>());
        assert_eq!(align_of::<DisplaySignedValue>(), align_of::<i16>());
    }

    #[test]
    fn untrusted_generated_value_friend_rejects_invalid_value() {
        let raw = 3;
        assert_panics(|| FlagsValue::of(DefaultBits(raw)));
        assert_panics(|| FlagsValue::of(CheckedBits(raw)));

        assert_eq!(FlagsValue::of(DefaultBits(6)).raw(), 6);
        assert_eq!(FlagsValue::of(CheckedBits(6)).raw(), 6);
    }

    #[test]
    fn untrusted_generated_value_friend_rejects_unknown_bits() {
        let raw = 8;
        assert_panics(|| FlagsValue::of(DefaultBits(raw)));
        assert_panics(|| FlagsValue::of(CheckedBits(raw)));

        assert_eq!(FlagsValue::of(DefaultBits(6)).raw(), 6);
        assert_eq!(FlagsValue::of(CheckedBits(6)).raw(), 6);
    }

    #[test]
    fn blind_trust_value_friend_bypasses_invalid_value_construction() {
        let raw = 3;
        let source = BlindTrustBits(raw);
        assert_eq!(FlagsValue::of(source).raw(), raw);
        assert_eq!(FlagsValue::try_make(raw), Err(raw));
        assert_panics(|| Flags::from_bits_retain(raw));

        let rhs = 1;
        assert_panics(|| {
            let _ = FlagsValue::try_make(2).unwrap() | BlindTrustBits(rhs);
        });
        assert_panics(|| {
            let _ = Flags::Read | BlindTrustBits(rhs);
        });

        assert_panics(|| {
            let _ =
                Flags::Read.into_value() | FlagsValue::of(BlindTrustBits(8));
        });
    }

    #[test]
    fn blind_trust_value_friend_bypasses_unknown_bits_construction() {
        let raw = 8;
        let source = BlindTrustBits(raw);
        assert_eq!(FlagsValue::of(source).raw(), raw);
        assert_eq!(FlagsValue::try_make(raw), Err(raw));
        assert_panics(|| Flags::from_bits_retain(raw));

        let rhs = 1;
        assert_panics(|| {
            let _ = FlagsValue::try_make(2).unwrap() | BlindTrustBits(rhs);
        });
        assert_panics(|| {
            let _ = Flags::Read | BlindTrustBits(rhs);
        });

        assert_panics(|| {
            let _ =
                Flags::Read.into_value() | FlagsValue::of(BlindTrustBits(8));
        });
    }

    #[test]
    fn blind_trust_assignment_rejects_invalid_rhs_without_mutating() {
        let rhs = 1;
        let mut flags = FlagsValue::try_make(2).unwrap();
        unsafe_assert_panics(|| {
            flags |= BlindTrustBits(rhs);
        });
        assert_eq!(flags.raw(), 2);
    }

    #[test]
    fn blind_trust_assignment_rejects_unknown_rhs_without_mutating() {
        let rhs = 8;
        let mut flags = FlagsValue::try_make(2).unwrap();
        unsafe_assert_panics(|| {
            flags |= BlindTrustBits(rhs);
        });
        assert_eq!(flags.raw(), 2);
    }

    #[test]
    fn blind_trust_bit_and_math_friends_still_validate_results() {
        let root_source = BitOnlyBits(4);
        assert_eq!((Flags::Read | root_source).raw(), 6);
        let value_source = BitOnlyBits(4);
        assert_eq!((Flags::Read.into_value() | value_source).raw(), 6);
        let math_source = MathOnlyBits(4);
        assert_eq!((Flags::Read.into_value() + math_source).raw(), 6);

        for rhs in [
            1,
            8,
        ] {
            assert_panics(|| Flags::Read | BitOnlyBits(rhs));
            assert_panics(|| Flags::Read.into_value() | BitOnlyBits(rhs));
            assert_panics(|| Flags::Read.into_value() + MathOnlyBits(rhs));

            let mut flags = Flags::Read.into_value();
            unsafe_assert_panics(|| flags |= BitOnlyBits(rhs));
            assert_eq!(flags.raw(), 2);
            unsafe_assert_panics(|| flags += MathOnlyBits(rhs));
            assert_eq!(flags.raw(), 2);
        }

        assert_eq!(FlagsValue::try_make(3), Err(3));
        assert_eq!(FlagsValue::try_make(8), Err(8));
    }

    #[test]
    fn generated_value_preserves_the_u8_layout() {
        assert_eq!(size_of::<FlagsValue>(), size_of::<u8>());
        assert_eq!(align_of::<FlagsValue>(), align_of::<u8>());
    }

    #[test]
    fn traits_and_layout_can_be_supplied_by_the_caller() {
        accepts_copy_clone::<ManuallyDerivedFlag>();

        assert_eq!(ManuallyDerivedFlag::Read.raw(), 1);
    }

    #[test]
    fn stable_konst_makes_value_api_const() {
        assert_eq!(STABLE_RETAINED.raw(), 0b111);
        assert_eq!(STABLE_UNION.raw(), 0b11);
    }

    #[test]
    fn default_konst_makes_value_api_const() {
        assert_eq!(DEFAULT_RETAINED.raw(), 0b1011);
        assert_eq!(DEFAULT_UNION.raw(), 0b101);
    }

    #[test]
    fn nested_konst_value_makes_value_api_const() {
        assert_eq!(FORWARDED_RETAINED.raw(), 0b111);
        assert_eq!(FORWARDED_UNION.raw(), 0b11);
    }

    #[test]
    fn konst_value_overrides_konst_none() {
        assert_eq!(OVERRIDDEN_RETAINED.raw(), 0b111);
        assert_eq!(OVERRIDDEN_UNION.raw(), 0b11);
    }

    #[test]
    fn explicit_derive_flags_generate_enum_traits() {
        accepts_copy_clone::<DerivedFlag>();
        accepts_hash::<DerivedFlag>();
        accepts_debug::<DerivedFlag>();
    }

    #[test]
    fn disabled_derive_flags_omit_hash_and_debug() {
        accepts_copy_clone::<UnderivedFlag>();
        assert_not_impl!(UnderivedFlag: core::hash::Hash);
        assert_not_impl!(UnderivedFlag: core::fmt::Debug);
    }

    #[test]
    fn without_display_omits_enum_and_value_display() {
        assert_not_impl!(UndisplayedFlag: core::fmt::Display);
        assert_not_impl!(UndisplayedFlagValue: core::fmt::Display);
    }

    #[test]
    fn nested_without_display_omits_value_display() {
        assert_not_impl!(UndisplayedValueFlag: core::fmt::Display);
        assert_not_impl!(UndisplayedValueFlagValue: core::fmt::Display);
    }

    #[test]
    fn nested_unit_error_keeps_constructor_errors() {
        let raw = 0b1000;
        let text = "0x8";
        let made = UnitErrorFlagValue::try_make(raw);
        let parsed = UnitErrorFlagValue::from_str(text);

        assert_eq!(made, Err(raw), "{raw:#x}");
        assert_eq!(parsed, Err(()), "{text}");
    }

    #[test]
    fn signed_value_negates_without_nested_integral() {
        let value = SignedWithoutNestedValue::Read;
        let negated = -value;

        assert_eq!(negated.bits(), -1, "{value:?}");
    }

    #[test]
    fn signed_value_negates_with_empty_nested_integral() {
        let value = SignedEmptyNestedValue::Read;
        let negated = -value;

        assert_eq!(negated.bits(), -1, "{value:?}");
    }

    #[test]
    fn unsigned_value_lacks_neg_without_nested_integral() {
        assert_not_impl!(UnsignedWithoutNestedValue: core::ops::Neg);
    }

    #[test]
    fn unsigned_value_lacks_neg_with_empty_nested_integral() {
        assert_not_impl!(UnsignedEmptyNestedValue: core::ops::Neg);
    }

    #[test]
    fn value_unsafe_friend_constructs_without_validation() {
        let raw = 0b1000;
        let source = ValueUnsafeBits(raw);
        let value = ValueUnsafeFlagValue::of(source);
        let made = ValueUnsafeFlagValue::try_make(raw);

        assert_eq!(value.raw(), raw, "{raw:#x}");
        assert_eq!(made, Err(raw), "{raw:#x}");
    }

    #[test]
    fn value_unsafe_friend_grants_no_operations() {
        assert_not_impl!(
            ValueUnsafeFlagValue: core::ops::BitOr<ValueUnsafeBits>
        );
        assert_not_impl!(ValueUnsafeFlagValue: core::ops::Add<ValueUnsafeBits>);
    }

    #[test]
    fn empty_friend_lists_keep_only_the_automatic_value_friend() {
        let rhs = 0b10u8;
        let combined = FriendlessFlagValue::Read | rhs;

        assert_eq!(combined.raw(), 0b11, "{rhs:#b}");
        assert_not_impl!(FriendlessFlag: core::ops::BitOr<u8>);
    }

    #[test]
    fn nested_math_and_bit_friend_operates_on_values() {
        let read = MathBitFlagValue::Read;
        let rhs = 4;
        let made = MathBitFlagValue::of(MathBitBits(rhs));

        assert_eq!((read | MathBitBits(rhs)).raw(), 6, "{rhs}");
        assert_eq!((read + MathBitBits(rhs)).raw(), 6, "{rhs}");
        assert_eq!(made.raw(), rhs, "{rhs}");
    }

    #[test]
    fn nested_math_and_bit_friend_rejects_invalid_results() {
        let read = MathBitFlagValue::Read;
        let odd = 1;

        assert_panics(|| read | MathBitBits(odd));
        assert_panics(|| read + MathBitBits(odd));
        assert_panics(|| MathBitFlagValue::of(MathBitBits(odd)));
    }

    #[test]
    fn unhidden_nested_blind_trust_stays_in_the_defining_module() {
        let raw = 0b100;
        let value = unhidden_blind_trust(raw);
        let made = UnhiddenBlindTrustFlagValue::try_make(raw);

        assert_eq!(value.raw(), raw, "{raw:#b}");
        assert_eq!(made, Err(raw), "{raw:#b}");
        assert_no_associated_fn!(UnhiddenBlindTrustFlagValue, _blind_trust);
    }
}

#[cfg(test)]
#[cfg(target_pointer_width = "64")]
mod pointer_width_64 {
    use super::*;

    #[test]
    fn pointer_bitflag_value_exposes_upper_bytes_on_64_bit_targets() {
        let value = PointerFlagValue::of(0x0123_4567_89ab_cdefu64);
        let upper_byte = value.byte4();
        let highest_byte = value.byte7();
        let raw: u64 = value.into();

        assert_eq!(upper_byte, 0x67);
        assert_eq!(highest_byte, 0x01);
        assert_eq!(raw, 0x0123_4567_89ab_cdef);
    }

    #[test]
    fn pointer_bitflag_value_checks_u32_narrowing_on_64_bit_targets() {
        let value = PointerFlagValue::of(1u64 << 32);
        let result = value.try_into_u32();
        let via_trait: Result<u32, ()> = value.try_into();

        assert_eq!(result, Err(()));
        assert_eq!(via_trait, Err(()));
    }
}

#[cfg(test)]
mod display_native_values {
    use super::*;

    #[test]
    fn displays_zero_wide_value_with_signed_minimum() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        let raw = 0u32;
        let value = Wide::from_bits_retain(raw);
        assert_display(value, "");

        let raw = i16::MIN;
        let value = Signed::from_bits_retain(raw);
        assert_display(value, "0x8000");
    }

    #[test]
    fn displays_zero_wide_value_with_positive_signed_value() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        let raw = 0u32;
        let value = Wide::from_bits_retain(raw);
        assert_display(value, "");

        let raw = -123;
        let value = Signed::from_bits_retain(raw);
        assert_display(value, "Positive | 0xff84");
    }

    #[test]
    fn displays_zero_wide_value_with_zero_signed_value() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        let raw = 0u32;
        let value = Wide::from_bits_retain(raw);
        assert_display(value, "");

        let raw = 0;
        let value = Signed::from_bits_retain(raw);
        assert_display(value, "");
    }

    #[test]
    fn displays_zero_wide_value_with_signed_maximum() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        let raw = 0u32;
        let value = Wide::from_bits_retain(raw);
        assert_display(value, "");

        let raw = i16::MAX;
        let value = Signed::from_bits_retain(raw);
        assert_display(value, "Positive | 0x7ffe");
    }

    #[test]
    fn displays_wide_high_bit_value_with_signed_minimum() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        let raw = 0x8000_0001;
        let value = Wide::from_bits_retain(raw);
        assert_display(value, "Low | High");

        let raw = i16::MIN;
        let value = Signed::from_bits_retain(raw);
        assert_display(value, "0x8000");
    }

    #[test]
    fn displays_wide_high_bit_value_with_positive_signed_value() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        let raw = 0x8000_0001;
        let value = Wide::from_bits_retain(raw);
        assert_display(value, "Low | High");

        let raw = -123;
        let value = Signed::from_bits_retain(raw);
        assert_display(value, "Positive | 0xff84");
    }

    #[test]
    fn displays_wide_high_bit_value_with_zero_signed_value() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        let raw = 0x8000_0001;
        let value = Wide::from_bits_retain(raw);
        assert_display(value, "Low | High");

        let raw = 0;
        let value = Signed::from_bits_retain(raw);
        assert_display(value, "");
    }

    #[test]
    fn displays_wide_high_bit_value_with_signed_maximum() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        let raw = 0x8000_0001;
        let value = Wide::from_bits_retain(raw);
        assert_display(value, "Low | High");

        let raw = i16::MAX;
        let value = Signed::from_bits_retain(raw);
        assert_display(value, "Positive | 0x7ffe");
    }

    #[test]
    fn displays_wide_maximum_value_with_signed_minimum() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        let raw = u32::MAX;
        let value = Wide::from_bits_retain(raw);
        assert_display(value, "Low | High | 0x7ffffffe");

        let raw = i16::MIN;
        let value = Signed::from_bits_retain(raw);
        assert_display(value, "0x8000");
    }

    #[test]
    fn displays_wide_maximum_value_with_positive_signed_value() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        let raw = u32::MAX;
        let value = Wide::from_bits_retain(raw);
        assert_display(value, "Low | High | 0x7ffffffe");

        let raw = -123;
        let value = Signed::from_bits_retain(raw);
        assert_display(value, "Positive | 0xff84");
    }

    #[test]
    fn displays_wide_maximum_value_with_zero_signed_value() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        let raw = u32::MAX;
        let value = Wide::from_bits_retain(raw);
        assert_display(value, "Low | High | 0x7ffffffe");

        let raw = 0;
        let value = Signed::from_bits_retain(raw);
        assert_display(value, "");
    }

    #[test]
    fn displays_wide_maximum_value_with_signed_maximum() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        let raw = u32::MAX;
        let value = Wide::from_bits_retain(raw);
        assert_display(value, "Low | High | 0x7ffffffe");

        let raw = i16::MAX;
        let value = Signed::from_bits_retain(raw);
        assert_display(value, "Positive | 0x7ffe");
    }

    #[test]
    fn generated_values_preserve_their_declared_integer_layout() {
        assert_eq!(size_of::<WideValue>(), size_of::<u32>());
        assert_eq!(align_of::<WideValue>(), align_of::<u32>());
        assert_eq!(size_of::<SignedValue>(), size_of::<i16>());
        assert_eq!(align_of::<SignedValue>(), align_of::<i16>());
    }
}
