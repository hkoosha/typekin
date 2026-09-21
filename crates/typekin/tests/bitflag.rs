#[cfg(test)]
mod tests {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[repr(u8)]
    #[typekin::bitflag(
        friends = [u8(conv = self, cap = [Bit])],
        integral = [
            friends = [u8(conv = self, cap = [Make, Math, Bit, Relation])],
            konst = false,
        ],
    )]
    pub enum Thingy {
        FirstThing = 0,
        ReadThing = 0b001,
        WritersBlock = 0b010,
        Execute = 0b100,
    }

    // =============================================================================

    #[test]
    fn value_empty() {
        let it = Thingy::empty();
        assert_eq!(it.raw(), 0b0);
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
    #[typekin::bitflag(suffix = "Holder", konst = false)]
    pub enum Thingy1 {
        FirstThing = 0,
        ReadThing = 0b001,
        WritersBlock = 0b010,
        Execute = 0b100,
    }

    #[test]
    fn suffix() {
        let _ = Thingy1Holder::all();
    }

    // =============================================================================

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[repr(u8)]
    #[typekin::bitflag(
        value_name = "Thingies2",
        suffix = "",
        integral = [
            konst = false,
        ],
    )]
    pub enum Thingy2 {
        FirstThing = 0,
        ReadThing = 0b001,
        WritersBlock = 0b010,
        Execute = 0b100,
    }

    #[test]
    fn value_name() {
        let _ = Thingies2::all();
    }
}
