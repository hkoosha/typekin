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
    pub enum Permission {
        Empty = 0,
        Read = 0b001,
        Write = 0b010,
        Execute = 0b100,
    }

    #[test]
    fn value_makes_from_borrowed_integral_friend() {
        let raw = 0b011u8;
        let it = PermissionValue::of(&raw);
        assert_eq!(it.raw(), raw);
    }

    #[test]
    fn value_makes_from_mutably_borrowed_integral_friend() {
        let mut raw = 0b011u8;
        let it = PermissionValue::of(&mut raw);
        assert_eq!(it.raw(), raw);
        assert_eq!(raw, 0b011u8);
    }

    #[test]
    fn enum_left_bitwise_accepts_borrowed_generated_value_friend() {
        let rhs = Permission::Write.into_value();
        let it = Permission::Read | &rhs;
        assert_eq!(it.raw(), 0b011);
    }

    #[test]
    fn enum_left_bitwise_accepts_mutably_borrowed_generated_value_friend() {
        let mut rhs = Permission::Write.into_value();
        let it = Permission::Read | &mut rhs;
        assert_eq!(it.raw(), 0b011);
        assert_eq!(rhs.raw(), 0b010);
    }

    #[test]
    fn enum_left_bitwise_accepts_borrowed_configured_friend() {
        let rhs = 0b010u8;
        let it = Permission::Read | &rhs;
        assert_eq!(it.raw(), 0b011);
    }

    #[test]
    fn value_left_bitwise_accepts_borrowed_generated_enum_friend() {
        let value = Permission::Read.into_value();
        let it = value | &Permission::Write;
        assert_eq!(it.raw(), 0b011);
    }
}
