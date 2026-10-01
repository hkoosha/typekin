#[cfg(test)]
mod tests {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[repr(u8)]
    #[typekin::bitflag(
        konst = false,
        friends = [u8(conv = self, cap = [Bit])],
        integral = [
            friends = [u8(conv = self, cap = [Make, Math, Bit, Relation])],
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

    // =============================================================================

    #[test]
    fn value_from_borrowed_integral_friend() {
        let raw = 0b011u8;
        let it = ThingyValue::of(&raw);
        assert_eq!(it.raw(), raw);
    }

    #[test]
    fn value_from_mutably_borrowed_integral_friend() {
        let mut raw = 0b011u8;
        let it = ThingyValue::of(&mut raw);
        assert_eq!(it.raw(), raw);
        assert_eq!(raw, 0b011u8);
    }

    #[test]
    fn enum_left_bitwise_borrowed_generated_value_friend() {
        let rhs = Thingy::WritersBlock.into_value();
        let it = Thingy::ReadThing | &rhs;
        assert_eq!(it.raw(), 0b011);
    }

    #[test]
    fn enum_left_bitwise_mutably_borrowed_generated_value_friend() {
        let mut rhs = Thingy::WritersBlock.into_value();
        let it = Thingy::ReadThing | &mut rhs;
        assert_eq!(it.raw(), 0b011);
        assert_eq!(rhs.raw(), 0b010);
    }

    #[test]
    fn enum_left_bitwise_borrowed_configured_friend() {
        let rhs = 0b010u8;
        let it = Thingy::ReadThing | &rhs;
        assert_eq!(it.raw(), 0b011);
    }

    #[test]
    fn value_left_bitwise_borrowed_generated_enum_friend() {
        let value = Thingy::ReadThing.into_value();
        let it = value | &Thingy::WritersBlock;
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
