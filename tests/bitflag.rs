#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_iter)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]

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

    #[typekin::bitflag(
        with = [display],
        friends = self(u8) -> Bit,
        integral = [
            friends = self(u8) -> [Math, Rel],
        ],
    )]
    #[repr(u8)]
    pub enum Thingy {
        FirstThing = 0,
        ReadThing = 0b001,
        WritersBlock = 0b010,
        Execute = 0b100,
    }

    #[typekin::bitflag(with = [serde])]
    #[repr(u8)]
    enum AutoSerdeFlag {
        Read = 0b0001,
        Write = 0b0010,
    }

    #[typekin::bitflag(with = [serde_with_name])]
    #[repr(u8)]
    enum NameSerdeFlag {
        Read = 0b0001,
        Write = 0b0010,
    }

    #[typekin::bitflag(with = [serde_with_value])]
    #[repr(u8)]
    enum ValueSerdeFlag {
        Read = 0b0001,
        Write = 0b0010,
    }

    #[typekin::bitflag(
        with = [serde],
        integral = [in = 0..=7],
    )]
    #[repr(u8)]
    enum ValidatedSerdeFlag {
        Read = 0b0001,
        Write = 0b0010,
    }

    #[typekin::bitflag(with = [display])]
    #[repr(u8)]
    enum CompositeFlag {
        Read = 0b0001,
        Write = 0b0010,
        ReadWrite = 0b0011,
    }

    #[typekin::bitflag(repr = u8)]
    enum FullyGeneratedFlag {
        Read = 0b0001,
        Write = 0b0010,
    }

    #[repr(u8)]
    #[typekin::bitflag(konst = nightly)]
    enum FullyGeneratedConstFlag {
        Read = 0b0001,
    }

    const FULLY_GENERATED_FLAG_CLONE: FullyGeneratedConstFlag =
        FullyGeneratedConstFlag::Read.clone();

    #[derive(Copy, Clone)]
    #[repr(u8)]
    #[typekin::bitflag(
        konst = none,
        without = [clone, copy, repr],
    )]
    enum ManuallyDerivedFlag {
        Read = 0b0001,
    }

    const THINGY_READ_VALUE: ThingyValue = ThingyValue::ReadThing;

    // =============================================================================

    #[test]
    fn value_empty() {
        let it = Thingy::empty();
        assert_eq!(it.raw(), 0b0);
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
                0, 1, 2, 4
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
                0, 1, 2, 8
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

    // =============================================================================

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

    // =============================================================================
    #[repr(u8)]
    #[typekin::bitflag(
        suffix = "Holder",
        integral = [in = 0..=7],
    )]
    pub enum Thingy1 {
        FirstThing = 0,
        ReadThing = 0b001,
        WritersBlock = 0b010,
        Execute = 0b100,
    }

    #[test]
    fn suffix() {
        let all = Thingy1Holder::all();
        assert_eq!(all.raw(), 0b111);
        assert_eq!(Thingy1Holder::Execute.raw(), Thingy1::Execute.raw());
    }

    #[test]
    fn nested_integral_ranges_reject_invalid_bits() {
        assert_panics(|| Thingy1::from_bits_retain(0b1000));
    }

    // =============================================================================

    #[repr(u8)]
    #[typekin::bitflag(value_name = "Thingies2", suffix = "")]
    pub enum Thingy2 {
        FirstThing = 0,
        ReadThing = 0b001,
        WritersBlock = 0b010,
        Execute = 0b100,
    }

    #[test]
    fn value_name() {
        let _ = Thingies2::all();
        assert_eq!(Thingies2::ReadThing.raw(), Thingy2::ReadThing.raw());
    }

    #[cfg(test)]
    mod const_bitflag {
        #[repr(u8)]
        #[typekin::bitflag(konst = nightly)]
        enum ConstFlag {
            Empty = 0,
            Read = 0b001,
            Write = 0b010,
            Execute = 0b100,
        }

        const RETAINED: ConstFlagValue = ConstFlag::from_bits_retain(0b1011);
        const TRUNCATED: ConstFlagValue = ConstFlag::from_bits_truncate(0b1011);
        const SET_ALGEBRA: ConstFlagValue = ConstFlagValue::Read
            .union(ConstFlagValue::Write)
            .toggled(ConstFlagValue::Execute);
        const FROM_ENUM: ConstFlagValue = ConstFlag::Read.into_value();
        const ENUM_NEGATED: ConstFlagValue = !ConstFlag::Read;
        const VALUE_NEGATED: ConstFlagValue = !ConstFlagValue::Read;

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
        fn generated_value_preserves_the_u8_layout() {
            assert_eq!(size_of::<ConstFlagValue>(), size_of::<u8>());
            assert_eq!(align_of::<ConstFlagValue>(), align_of::<u8>());
        }
    }

    #[cfg(test)]
    mod trusted_friends {
        use crate::tests::{
            assert_panics,
            unsafe_assert_panics,
        };

        struct DefaultBits(u8);
        struct CheckedBits(u8);
        struct TrustBits(u8);
        struct BitOnlyBits(u8);
        struct MathOnlyBits(u8);

        const fn default_bits(source: DefaultBits) -> u8 {
            source.0
        }

        const fn checked_bits(source: CheckedBits) -> u8 {
            source.0
        }

        const fn trusted_bits(source: TrustBits) -> u8 {
            source.0
        }

        const fn bit_only_bits(source: BitOnlyBits) -> u8 {
            source.0
        }

        const fn math_only_bits(source: MathOnlyBits) -> u8 {
            source.0
        }

        const fn is_even(bits: u8) -> bool {
            bits % 2 == 0
        }

        #[repr(u8)]
        #[typekin::bitflag(
            friends = [
                trusted_bits(TrustBits) -> [Make, Bit, unsafe],
                bit_only_bits(BitOnlyBits) -> [Bit, unsafe],
            ],
            integral = [
                friends = [
                    default_bits(DefaultBits) -> Make,
                    checked_bits(CheckedBits) -> [Make],
                    trusted_bits(TrustBits) -> [Make, Bit, unsafe],
                    bit_only_bits(BitOnlyBits) -> [Bit, unsafe],
                    math_only_bits(MathOnlyBits) -> [Math, unsafe],
                ],
                valid = is_even,
                in = 0..=6,
            ],
        )]
        enum Flags {
            Empty = 0,
            Read = 2,
            Write = 4,
        }

        #[test]
        fn generated_value_untrusted_friends_validate_scalar_and_list_make_3_1()
        {
            let raw = 3;
            assert_panics(|| FlagsValue::of(DefaultBits(raw)));
            assert_panics(|| FlagsValue::of(CheckedBits(raw)));

            assert_eq!(FlagsValue::of(DefaultBits(6)).raw(), 6);
            assert_eq!(FlagsValue::of(CheckedBits(6)).raw(), 6);
        }

        #[test]
        fn generated_value_untrusted_friends_validate_scalar_and_list_make_8_2()
        {
            let raw = 8;
            assert_panics(|| FlagsValue::of(DefaultBits(raw)));
            assert_panics(|| FlagsValue::of(CheckedBits(raw)));

            assert_eq!(FlagsValue::of(DefaultBits(6)).raw(), 6);
            assert_eq!(FlagsValue::of(CheckedBits(6)).raw(), 6);
        }

        #[test]
        fn generated_value_trusted_owned_friend_bypasses_only_construction_3_1_1_1()
         {
            let raw = 3;
            let source = TrustBits(raw);
            assert_eq!(FlagsValue::of(source).raw(), raw);
            assert_eq!(FlagsValue::try_make(raw), Err(raw));
            assert_panics(|| Flags::from_bits_retain(raw));

            let rhs = 1;
            assert_panics(|| {
                let _ = FlagsValue::try_make(2).unwrap() | TrustBits(rhs);
            });
            assert_panics(|| {
                let _ = Flags::Read | TrustBits(rhs);
            });

            assert_panics(|| {
                let _ = Flags::Read.into_value() | FlagsValue::of(TrustBits(8));
            });
        }

        #[test]
        fn generated_value_trusted_owned_friend_bypasses_only_construction_3_1_8_2()
         {
            let raw = 3;
            let source = TrustBits(raw);
            assert_eq!(FlagsValue::of(source).raw(), raw);
            assert_eq!(FlagsValue::try_make(raw), Err(raw));
            assert_panics(|| Flags::from_bits_retain(raw));

            let rhs = 8;
            assert_panics(|| {
                let _ = FlagsValue::try_make(2).unwrap() | TrustBits(rhs);
            });
            assert_panics(|| {
                let _ = Flags::Read | TrustBits(rhs);
            });

            assert_panics(|| {
                let _ = Flags::Read.into_value() | FlagsValue::of(TrustBits(8));
            });
        }

        #[test]
        fn generated_value_trusted_owned_friend_bypasses_only_construction_8_2_1_1()
         {
            let raw = 8;
            let source = TrustBits(raw);
            assert_eq!(FlagsValue::of(source).raw(), raw);
            assert_eq!(FlagsValue::try_make(raw), Err(raw));
            assert_panics(|| Flags::from_bits_retain(raw));

            let rhs = 1;
            assert_panics(|| {
                let _ = FlagsValue::try_make(2).unwrap() | TrustBits(rhs);
            });
            assert_panics(|| {
                let _ = Flags::Read | TrustBits(rhs);
            });

            assert_panics(|| {
                let _ = Flags::Read.into_value() | FlagsValue::of(TrustBits(8));
            });
        }

        #[test]
        fn generated_value_trusted_owned_friend_bypasses_only_construction_8_2_8_2()
         {
            let raw = 8;
            let source = TrustBits(raw);
            assert_eq!(FlagsValue::of(source).raw(), raw);
            assert_eq!(FlagsValue::try_make(raw), Err(raw));
            assert_panics(|| Flags::from_bits_retain(raw));

            let rhs = 8;
            assert_panics(|| {
                let _ = FlagsValue::try_make(2).unwrap() | TrustBits(rhs);
            });
            assert_panics(|| {
                let _ = Flags::Read | TrustBits(rhs);
            });

            assert_panics(|| {
                let _ = Flags::Read.into_value() | FlagsValue::of(TrustBits(8));
            });
        }

        #[test]
        fn trusted_owned_assignment_results_validate_before_mutating_1_1() {
            let rhs = 1;
            let mut flags = FlagsValue::try_make(2).unwrap();
            unsafe_assert_panics(|| {
                flags |= TrustBits(rhs);
            });
            assert_eq!(flags.raw(), 2);
        }

        #[test]
        fn trusted_owned_assignment_results_validate_before_mutating_8_2() {
            let rhs = 8;
            let mut flags = FlagsValue::try_make(2).unwrap();
            unsafe_assert_panics(|| {
                flags |= TrustBits(rhs);
            });
            assert_eq!(flags.raw(), 2);
        }

        #[test]
        fn trusted_operation_only_friends_validate_enum_and_generated_value_results()
         {
            let root_source = BitOnlyBits(4);
            assert_eq!((Flags::Read | root_source).raw(), 6);
            let value_source = BitOnlyBits(4);
            assert_eq!((Flags::Read.into_value() | value_source).raw(), 6);
            let math_source = MathOnlyBits(4);
            assert_eq!((Flags::Read.into_value() + math_source).raw(), 6);

            for rhs in [
                1, 8,
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
    }

    #[cfg(test)]
    mod display_configuration {
        #[repr(u8)]
        #[typekin::bitflag(
            with = display,
            integral = [without = display],
        )]
        enum RootBeforeNested {
            Read = 1,
            Write = 2,
        }

        #[repr(u8)]
        #[typekin::bitflag(
            integral = [without = display],
            with = display,
        )]
        enum NestedBeforeRoot {
            Read = 1,
            Write = 2,
        }

        #[repr(u8)]
        #[typekin::bitflag(integral = [with = display])]
        enum NestedOnly {
            Read = 1,
            Write = 2,
        }

        #[repr(u8)]
        #[typekin::bitflag(
            without = display,
            integral = [with = display],
        )]
        enum DisabledRootBeforeNested {
            Read = 1,
            Write = 2,
        }

        #[repr(u8)]
        #[typekin::bitflag(
            integral = [with = display],
            without = display,
        )]
        enum NestedBeforeDisabledRoot {
            Read = 1,
            Write = 2,
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
        fn generated_values_preserve_their_u8_layout() {
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
    }

    #[cfg(any())]
    mod display_native_values {
        #[typekin::bitflag(konst = none, with = [display])]
        #[repr(u32)]
        #[derive(Copy, Clone)]
        enum Wide {
            Low = 1,
            High = 0x8000_0000,
        }

        #[typekin::bitflag(konst = none, with = [display])]
        #[repr(i16)]
        #[derive(Copy, Clone)]
        enum Signed {
            Positive = 1,
            Negative = -1,
        }

        #[test]
        fn value_display_preserves_integer_width_and_sign_0u32_1_i16_min_1() {
            assert_eq!(Wide::High.to_string(), "High");
            assert_eq!(Signed::Negative.to_string(), "Negative");

            let raw = 0u32;
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));

            let raw = i16::MIN;
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }

        #[test]
        fn value_display_preserves_integer_width_and_sign_0u32_1_123_2() {
            assert_eq!(Wide::High.to_string(), "High");
            assert_eq!(Signed::Negative.to_string(), "Negative");

            let raw = 0u32;
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));

            let raw = -123;
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }

        #[test]
        fn value_display_preserves_integer_width_and_sign_0u32_1_0_3() {
            assert_eq!(Wide::High.to_string(), "High");
            assert_eq!(Signed::Negative.to_string(), "Negative");

            let raw = 0u32;
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));

            let raw = 0;
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }

        #[test]
        fn value_display_preserves_integer_width_and_sign_0u32_1_i16_max_4() {
            assert_eq!(Wide::High.to_string(), "High");
            assert_eq!(Signed::Negative.to_string(), "Negative");

            let raw = 0u32;
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));

            let raw = i16::MAX;
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }

        #[test]
        fn value_display_preserves_integer_width_and_sign_0x8000_0001_2_i16_min_1()
         {
            assert_eq!(Wide::High.to_string(), "High");
            assert_eq!(Signed::Negative.to_string(), "Negative");

            let raw = 0x8000_0001;
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));

            let raw = i16::MIN;
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }

        #[test]
        fn value_display_preserves_integer_width_and_sign_0x8000_0001_2_123_2()
        {
            assert_eq!(Wide::High.to_string(), "High");
            assert_eq!(Signed::Negative.to_string(), "Negative");

            let raw = 0x8000_0001;
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));

            let raw = -123;
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }

        #[test]
        fn value_display_preserves_integer_width_and_sign_0x8000_0001_2_0_3() {
            assert_eq!(Wide::High.to_string(), "High");
            assert_eq!(Signed::Negative.to_string(), "Negative");

            let raw = 0x8000_0001;
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));

            let raw = 0;
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }

        #[test]
        fn value_display_preserves_integer_width_and_sign_0x8000_0001_2_i16_max_4()
         {
            assert_eq!(Wide::High.to_string(), "High");
            assert_eq!(Signed::Negative.to_string(), "Negative");

            let raw = 0x8000_0001;
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));

            let raw = i16::MAX;
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }

        #[test]
        fn value_display_preserves_integer_width_and_sign_u32_max_3_i16_min_1()
        {
            assert_eq!(Wide::High.to_string(), "High");
            assert_eq!(Signed::Negative.to_string(), "Negative");

            let raw = u32::MAX;
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));

            let raw = i16::MIN;
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }

        #[test]
        fn value_display_preserves_integer_width_and_sign_u32_max_3_123_2() {
            assert_eq!(Wide::High.to_string(), "High");
            assert_eq!(Signed::Negative.to_string(), "Negative");

            let raw = u32::MAX;
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));

            let raw = -123;
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }

        #[test]
        fn value_display_preserves_integer_width_and_sign_u32_max_3_0_3() {
            assert_eq!(Wide::High.to_string(), "High");
            assert_eq!(Signed::Negative.to_string(), "Negative");

            let raw = u32::MAX;
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));

            let raw = 0;
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }

        #[test]
        fn value_display_preserves_integer_width_and_sign_u32_max_3_i16_max_4()
        {
            assert_eq!(Wide::High.to_string(), "High");
            assert_eq!(Signed::Negative.to_string(), "Negative");

            let raw = u32::MAX;
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));

            let raw = i16::MAX;
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }

        #[test]
        fn generated_values_preserve_their_declared_integer_layout() {
            assert_eq!(size_of::<WideValue>(), size_of::<u32>());
            assert_eq!(align_of::<WideValue>(), align_of::<u32>());
            assert_eq!(size_of::<SignedValue>(), size_of::<i16>());
            assert_eq!(align_of::<SignedValue>(), align_of::<i16>());
        }
    }

    mod display_retain_values {
        use core::str::FromStr;

        #[typekin::bitflag(with = [display])]
        #[repr(u32)]
        enum Wide {
            Low = 1,
            High = 0x8000_0000,
        }

        #[typekin::bitflag(with = [display])]
        #[repr(i16)]
        enum Signed {
            Positive = 1,
            Negative = -1,
        }

        #[test]
        fn unsigned_display_uses_retain_text() {
            assert_eq!(Wide::from_bits_retain(0).to_string(), "");
            assert_eq!(
                Wide::from_bits_retain(0x8000_0001).to_string(),
                "Low | High",
            );
            assert_eq!(
                Wide::from_bits_retain(u32::MAX).to_string(),
                "Low | High | 0x7ffffffe",
            );
        }

        #[test]
        fn display_ignores_numeric_formatter_options() {
            let value = Wide::from_bits_retain(u32::MAX);
            assert_eq!(format!("{value:+014}"), "Low | High | 0x7ffffffe");
            assert_eq!(format!("{value:*^15.2}"), "Low | High | 0x7ffffffe");
        }

        #[test]
        fn signed_display_uses_unsigned_hex_for_remaining_bits() {
            assert_eq!(Signed::from_bits_retain(0).to_string(), "");
            assert_eq!(
                Signed::from_bits_retain(i16::MIN).to_string(),
                "0x8000",
            );
            assert_eq!(
                Signed::from_bits_retain(-123).to_string(),
                "Positive | 0xff84",
            );
            assert_eq!(
                Signed::from_bits_retain(i16::MAX).to_string(),
                "Positive | 0x7ffe",
            );
            assert_eq!(
                Signed::from_bits_retain(-1).to_string(),
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
                let value = Signed::from_bits_retain(raw);
                assert_eq!(
                    SignedValue::from_str(&value.to_string())
                        .map(SignedValue::bits),
                    Ok(raw),
                );
            }
        }

        #[test]
        fn generated_values_preserve_their_declared_integer_layout() {
            assert_eq!(size_of::<WideValue>(), size_of::<u32>());
            assert_eq!(align_of::<WideValue>(), align_of::<u32>());
            assert_eq!(size_of::<SignedValue>(), size_of::<i16>());
            assert_eq!(align_of::<SignedValue>(), align_of::<i16>());
        }
    }

    pub fn assert_panics<T>(f: impl FnOnce() -> T + std::panic::UnwindSafe) {
        std::panic::set_hook(Box::new(move |_| {}));
        let result = std::panic::catch_unwind(f);
        let _ = std::panic::take_hook();
        assert!(result.is_err());
    }

    pub fn unsafe_assert_panics<T>(f: impl FnOnce() -> T) {
        assert_panics(std::panic::AssertUnwindSafe(f))
    }
}
