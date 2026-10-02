#[cfg(test)]
mod tests {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[repr(u8)]
    #[typekin::bitflag(
        konst = false,
        with = [display],
        friends = _(u8) -> Bit,
        integral = [
            friends = _(u8) -> [Make, Math, Bit, Relation],
        ],
    )]
    pub enum Thingy {
        FirstThing = 0,
        ReadThing = 0b001,
        WritersBlock = 0b010,
        Execute = 0b100,
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
        assert_eq!(format!("{}", combined), "3");
        assert_eq!(format!("{:05}", combined), "00003");
        assert_eq!(format!("{:_<5}", combined), "3____");
        assert_eq!(format!("{}", Thingy::from_bits_retain(0b1000)), "8");
        assert_eq!(format!("{}", Thingy::from_bits_retain(0b1011)), "11");
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[repr(u8)]
    #[typekin::bitflag(
        suffix = "Holder",
        konst = false,
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
        assert!(
            std::panic::catch_unwind(|| Thingy1::from_bits_retain(0b1000))
                .is_err()
        );
    }

    // =============================================================================

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[repr(u8)]
    #[typekin::bitflag(value_name = "Thingies2", suffix = "", konst = false)]
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
}

#[cfg(test)]
mod trusted_friends {
    struct DefaultBits(Box<u8>);
    struct CheckedBits(Box<u8>);
    struct TrustBits(Box<u8>);
    struct BitOnlyBits(Box<u8>);
    struct MathOnlyBits(Box<u8>);

    fn default_bits(source: DefaultBits) -> u8 {
        *source.0
    }

    fn checked_bits(source: CheckedBits) -> u8 {
        *source.0
    }

    fn trusted_bits(source: TrustBits) -> u8 {
        *source.0
    }

    fn bit_only_bits(source: BitOnlyBits) -> u8 {
        *source.0
    }

    fn math_only_bits(source: MathOnlyBits) -> u8 {
        *source.0
    }

    const fn is_even(bits: u8) -> bool {
        bits % 2 == 0
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[repr(u8)]
    #[typekin::bitflag(
        konst = false,
        friends = [
            trusted_bits(TrustBits) -> [Make, Bit, Trust],
            bit_only_bits(BitOnlyBits) -> [Bit, Trust],
        ],
        integral = [
            friends = [
                default_bits(DefaultBits) -> Make,
                checked_bits(CheckedBits) -> [Make],
                trusted_bits(TrustBits) -> [Make, Bit, Trust],
                bit_only_bits(BitOnlyBits) -> [Bit, Trust],
                math_only_bits(MathOnlyBits) -> [Math, Trust],
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
    fn generated_value_untrusted_friends_validate_scalar_and_list_make() {
        for raw in [3, 8] {
            assert!(
                std::panic::catch_unwind(|| {
                    FlagsValue::of(DefaultBits(Box::new(raw)))
                })
                .is_err()
            );
            assert!(
                std::panic::catch_unwind(|| {
                    FlagsValue::of(CheckedBits(Box::new(raw)))
                })
                .is_err()
            );
        }
        assert_eq!(FlagsValue::of(DefaultBits(Box::new(6))).raw(), 6);
        assert_eq!(FlagsValue::of(CheckedBits(Box::new(6))).raw(), 6);
    }

    #[test]
    fn generated_value_trusted_owned_friend_bypasses_only_construction() {
        for raw in [3, 8] {
            let source = TrustBits(Box::new(raw));
            assert_eq!(FlagsValue::of(source).raw(), raw);
            assert_eq!(FlagsValue::try_make(raw), Err(raw));
            assert!(
                std::panic::catch_unwind(|| Flags::from_bits_retain(raw))
                    .is_err()
            );
        }
        for rhs in [1, 8] {
            assert!(
                std::panic::catch_unwind(|| {
                    let _ = FlagsValue::try_make(2).unwrap()
                        | TrustBits(Box::new(rhs));
                })
                .is_err()
            );
            assert!(
                std::panic::catch_unwind(|| {
                    let _ = Flags::Read | TrustBits(Box::new(rhs));
                })
                .is_err()
            );
        }
        assert!(
            std::panic::catch_unwind(|| {
                let _ = Flags::Read.into_value()
                    | FlagsValue::of(TrustBits(Box::new(8)));
            })
            .is_err()
        );
    }

    #[test]
    fn trusted_owned_assignment_results_validate_before_mutating() {
        for rhs in [1, 8] {
            let mut flags = FlagsValue::try_make(2).unwrap();
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    flags |= TrustBits(Box::new(rhs));
                }))
                .is_err()
            );
            assert_eq!(flags.raw(), 2);
        }
    }

    #[test]
    fn trusted_operation_only_friends_validate_enum_and_generated_value_results()
     {
        let root_source = BitOnlyBits(Box::new(4));
        assert_eq!((Flags::Read | root_source).raw(), 6);
        let value_source = BitOnlyBits(Box::new(4));
        assert_eq!((Flags::Read.into_value() | value_source).raw(), 6);
        let math_source = MathOnlyBits(Box::new(4));
        assert_eq!((Flags::Read.into_value() + math_source).raw(), 6);

        for rhs in [1, 8] {
            assert!(
                std::panic::catch_unwind(|| {
                    let _ = Flags::Read | BitOnlyBits(Box::new(rhs));
                })
                .is_err()
            );
            assert!(
                std::panic::catch_unwind(|| {
                    let _ =
                        Flags::Read.into_value() | BitOnlyBits(Box::new(rhs));
                })
                .is_err()
            );
            assert!(
                std::panic::catch_unwind(|| {
                    let _ =
                        Flags::Read.into_value() + MathOnlyBits(Box::new(rhs));
                })
                .is_err()
            );

            let mut flags = Flags::Read.into_value();
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    flags |= BitOnlyBits(Box::new(rhs));
                }))
                .is_err()
            );
            assert_eq!(flags.raw(), 2);
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    flags += MathOnlyBits(Box::new(rhs));
                }))
                .is_err()
            );
            assert_eq!(flags.raw(), 2);
        }
        assert_eq!(FlagsValue::try_make(3), Err(3));
        assert_eq!(FlagsValue::try_make(8), Err(8));
    }
}

#[cfg(test)]
mod display_configuration {
    #[derive(Copy, Clone)]
    #[repr(u8)]
    #[typekin::bitflag(
        konst = false,
        with = [display],
        integral = [without = [display]],
    )]
    enum RootBeforeNested {
        Read = 1,
        Write = 2,
    }

    #[derive(Copy, Clone)]
    #[repr(u8)]
    #[typekin::bitflag(
        konst = false,
        integral = [without = [display]],
        with = [display],
    )]
    enum NestedBeforeRoot {
        Read = 1,
        Write = 2,
    }

    #[derive(Copy, Clone)]
    #[repr(u8)]
    #[typekin::bitflag(konst = false, integral = [with = [display]])]
    enum NestedOnly {
        Read = 1,
        Write = 2,
    }

    #[derive(Copy, Clone)]
    #[repr(u8)]
    #[typekin::bitflag(
        konst = false,
        without = [display],
        integral = [with = [display]],
    )]
    enum DisabledRootBeforeNested {
        Read = 1,
        Write = 2,
    }

    #[derive(Copy, Clone)]
    #[repr(u8)]
    #[typekin::bitflag(
        konst = false,
        integral = [with = [display]],
        without = [display],
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
            "3"
        );
        assert_eq!(format!("{}", NestedBeforeRoot::Write), "Write");
        assert_eq!(
            format!("{}", NestedBeforeRoot::Read | NestedBeforeRoot::Write),
            "3"
        );
    }

    #[test]
    fn nested_display_formats_values_without_root_opt_in() {
        assert_eq!(format!("{}", NestedOnly::Read | NestedOnly::Write), "3");
        assert_eq!(
            format!("{:+05}", NestedOnly::from_bits_retain(0b1000)),
            "+0008"
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
            "3"
        );
        assert_eq!(
            format!(
                "{}",
                NestedBeforeDisabledRoot::Read
                    | NestedBeforeDisabledRoot::Write
            ),
            "3"
        );
        assert_eq!(
            format!(
                "{:04}",
                DisabledRootBeforeNested::from_bits_retain(0b1000)
            ),
            "0008"
        );
        assert_eq!(
            format!(
                "{:04}",
                NestedBeforeDisabledRoot::from_bits_retain(0b1000)
            ),
            "0008"
        );
    }
}

#[cfg(test)]
mod display_native_values {
    #[typekin::bitflag(konst = false, with = [display])]
    #[repr(u32)]
    #[derive(Copy, Clone)]
    enum Wide {
        Low = 1,
        High = 0x8000_0000,
    }

    #[typekin::bitflag(konst = false, with = [display])]
    #[repr(i16)]
    #[derive(Copy, Clone)]
    enum Signed {
        Positive = 1,
        Negative = -1,
    }

    #[test]
    fn value_display_preserves_integer_width_and_sign() {
        assert_eq!(Wide::High.to_string(), "High");
        assert_eq!(Signed::Negative.to_string(), "Negative");

        for raw in [0u32, 0x8000_0001, u32::MAX] {
            let value = Wide::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+014}"), format!("{raw:+014}"));
            assert_eq!(format!("{value:*^15.2}"), format!("{raw:*^15.2}"));
        }
        for raw in [i16::MIN, -123, 0, i16::MAX] {
            let value = Signed::from_bits_retain(raw);
            assert_eq!(format!("{value}"), format!("{raw}"));
            assert_eq!(format!("{value:+09}"), format!("{raw:+09}"));
            assert_eq!(format!("{value:_<10.2}"), format!("{raw:_<10.2}"));
        }
    }
}
