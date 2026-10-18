#![feature(const_closures)]
#![feature(const_destruct)]
#![feature(const_trait_impl)]

extern crate alloc;

#[cfg(test)]
mod tests {
    use core::cell::Cell;
    use core::str::FromStr;

    use serde_test::{
        Configure,
        Token,
        assert_de_tokens_error,
        assert_tokens,
    };

    const fn final_state_is_valid(value: u8) -> bool {
        return value == 0 || value >= 3;
    }

    #[typekin::integral(
        konst = none,
        valid = final_state_is_valid,
    )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct Mapped(u8);

    #[typekin::integral(konst = none)]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct PlainMapped(u8);

    #[typekin::integral(
        konst = none,
        with = [with_const_map_fn],
    )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct ConstMapped(u8);

    const CONST_MAPPED: ConstMapped =
        ConstMapped(20).map_raw(const |value| value + 3);

    const fn is_even(value: u8) -> bool {
        return value % 2 == 0;
    }

    #[typekin::integral(
        konst = none,
        valid = is_even,
        with = [serde],
    )]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    struct SerdeIntegral(u8);

    fn is_text_valid(value: &str) -> bool {
        return value == "valid";
    }

    #[typekin::text(
        konst = none,
        std = false,
        valid = is_text_valid,
        with = [serde],
    )]
    #[repr(transparent)]
    struct SerdeText(String);

    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[typekin::bitflag(
        konst = none,
        with = [display, serde],
    )]
    enum AutoFlag {
        Empty = 0,
        Read = 0b0001,
        Write = 0b0010,
    }

    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[typekin::bitflag(
        konst = none,
        with = [serde, serde_with_name],
    )]
    enum NameFlag {
        Read = 0b0001,
        Write = 0b0010,
    }

    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[typekin::bitflag(
        konst = none,
        with = [serde, serde_with_value],
    )]
    enum ValueFlag {
        Read = 0b0001,
        Write = 0b0010,
    }

    #[test]
    fn integral_mapping_validates_only_the_final_value() {
        let calls = Cell::new(0);
        let mapped = Mapped::try_make(0)
            .unwrap()
            .try_map_raw(|value| {
                calls.set(calls.get() + 1);
                return value + 1 + 2;
            })
            .unwrap();

        assert_eq!(mapped.raw(), 3);
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn integral_mapping_returns_or_panics_on_invalid_final_value() {
        assert_eq!(Mapped::try_make(3).unwrap().try_map_raw(|_| 1), Err(1),);
        assert!(
            std::panic::catch_unwind(|| {
                let _ = Mapped::try_make(3).unwrap().map_raw(|_| 1);
            })
            .is_err()
        );
    }

    #[test]
    fn unvalidated_and_const_integral_mapping_work() {
        assert_eq!(PlainMapped::make(2).map_raw(|value| value + 4).raw(), 6);
        assert_eq!(CONST_MAPPED.raw(), 23);
    }

    #[test]
    fn integral_serde_uses_raw_values_and_revalidates() {
        assert_tokens(
            &SerdeIntegral::try_make(4).unwrap().readable(),
            &[Token::U8(4)],
        );
        assert_de_tokens_error::<SerdeIntegral>(
            &[Token::U8(3)],
            "invalid value",
        );
    }

    #[test]
    fn text_serde_uses_strings_and_revalidates() {
        assert_tokens(
            &SerdeText::try_from_str("valid").unwrap().readable(),
            &[Token::Str("valid")],
        );
        assert_de_tokens_error::<SerdeText>(
            &[Token::Str("invalid")],
            "invalid value",
        );
    }

    #[test]
    fn bitflag_text_round_trips_known_and_unknown_bits() {
        let value = AutoFlagValue::from_bits_retain(0b1011);
        assert_eq!(value.to_string(), "Read | Write | 0x8");
        assert_eq!(
            AutoFlagValue::from_str(" Read|Write | 0x8 ").unwrap(),
            value,
        );
        assert_eq!(AutoFlagValue::empty().to_string(), "");
        assert_eq!(AutoFlag::from_str("Read"), Ok(AutoFlag::Read));
        assert!(AutoFlagValue::from_str("Read || Write").is_err());
        assert!(AutoFlagValue::from_str("Unknown").is_err());
    }

    #[test]
    fn bitflag_auto_serde_uses_readable_names_and_compact_bits() {
        let value = AutoFlagValue::from_bits_retain(0b1011);
        assert_tokens(&value.readable(), &[Token::Str("Read | Write | 0x8")]);
        assert_tokens(&value.compact(), &[Token::U8(0b1011)]);
        assert_tokens(&AutoFlag::Read.readable(), &[Token::Str("Read")]);
        assert_tokens(&AutoFlag::Read.compact(), &[Token::U8(1)]);
    }

    #[test]
    fn bitflag_name_and_value_serde_modes_are_forced() {
        let named = NameFlag::Read | NameFlag::Write;
        assert_tokens(&named.compact(), &[Token::Str("Read | Write")]);

        let valued = ValueFlag::Read | ValueFlag::Write;
        assert_tokens(&valued.readable(), &[Token::U8(3)]);
    }
}
