#![allow(dead_code)]

#[cfg(test)]
mod test {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[repr(u8)]
    #[typekin::bitflag(
        friends = [u8(conv = self, level = [Bit])],
        integral = [
            friends = [u8(conv = self, level = [Full])],
            konst = false,
        ],
    )]
    pub enum Thingy {
        FirstThing = 0,
        ReadThing = 0b001,
        WritersBlock = 0b010,
        Execute = 0b100,
    }

    #[test]
    fn value_makes_from_borrowed_integral_friend() {
        let raw = 0b011u8;
        let it = ThingyValue::of(&raw);
        assert_eq!(it.raw(), raw);
    }

    #[test]
    fn value_makes_from_mutably_borrowed_integral_friend() {
        let mut raw = 0b011u8;
        let it = ThingyValue::of(&mut raw);
        assert_eq!(it.raw(), raw);
        assert_eq!(raw, 0b011u8);
    }

    #[test]
    fn enum_left_bitwise_accepts_borrowed_generated_value_friend() {
        let rhs = Thingy::WritersBlock.into_value();
        let it = Thingy::ReadThing | &rhs;
        assert_eq!(it.raw(), 0b011);
    }

    #[test]
    fn enum_left_bitwise_accepts_mutably_borrowed_generated_value_friend() {
        let mut rhs = Thingy::WritersBlock.into_value();
        let it = Thingy::ReadThing | &mut rhs;
        assert_eq!(it.raw(), 0b011);
        assert_eq!(rhs.raw(), 0b010);
    }

    #[test]
    fn enum_left_bitwise_accepts_borrowed_configured_friend() {
        let rhs = 0b010u8;
        let it = Thingy::ReadThing | &rhs;
        assert_eq!(it.raw(), 0b011);
    }

    #[test]
    fn value_left_bitwise_accepts_borrowed_generated_enum_friend() {
        let value = Thingy::ReadThing.into_value();
        let it = value | &Thingy::WritersBlock;
        assert_eq!(it.raw(), 0b011);
    }
}

#[cfg(test)]
mod cfg_test_value_type_name_suffix {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[repr(u8)]
    #[typekin::bitflag(
        value_name_suffix = "Holder",
        integral = [
            konst = false,
        ],
    )]
    pub enum Thingy {
        FirstThing = 0,
        ReadThing = 0b001,
        WritersBlock = 0b010,
        Execute = 0b100,
    }

    #[test]
    fn validate_flag_value_name_suffix() {
        let _ = ThingyHolder::all();
    }
}

#[cfg(test)]
mod cfg_test_value_type_name {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[repr(u8)]
    #[typekin::bitflag(
        value_name = "Thingies",
        value_name_suffix = "",
        integral = [
            konst = false,
        ],
    )]
    pub enum Thingy {
        FirstThing = 0,
        ReadThing = 0b001,
        WritersBlock = 0b010,
        Execute = 0b100,
    }

    #[test]
    fn validate_flag_value_name() {
        let _ = Thingies::all();
    }
}
