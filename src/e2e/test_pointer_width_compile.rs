//! 32-bit `usize`/`isize` behavior, checked on `wasm32-unknown-unknown`

use crate::fx_pointer_width_compile::*;
use crate::*;

#[cfg(test)]
mod accepted {
    use super::*;

    #[test]
    fn wasm32_u64_to_usize_checked_conversion_rejects_truncation() {
        assert_compiles(&ACCEPTED_CHECKED, &U64_TO_USIZE_CHECKED);
    }

    #[test]
    fn wasm32_u32_to_isize_rejects_values_above_the_signed_maximum() {
        assert_compiles(&ACCEPTED_CHECKED, &U32_TO_ISIZE_CHECKED);
    }

    #[test]
    fn wasm32_u32_to_usize_remains_infallible() {
        assert_compiles(&ACCEPTED_CHECKED, &U32_TO_USIZE_INFALLIBLE);
    }

    #[test]
    fn wasm32_usize_has_no_fifth_byte_accessor() {
        assert_compiles(&ACCEPTED_CHECKED, &USIZE_NO_FIFTH_BYTE);
    }

    #[test]
    fn wasm32_usize_keeps_the_fourth_byte_accessor() {
        assert_compiles(&ACCEPTED_CHECKED, &USIZE_FOURTH_BYTE);
    }

    #[test]
    fn wasm32_u64_has_no_infallible_usize_method() {
        assert_compiles(&ACCEPTED_CHECKED, &U64_NO_USIZE_METHOD);
    }

    #[test]
    fn wasm32_u64_has_no_infallible_usize_trait() {
        assert_compiles(&ACCEPTED_CHECKED, &U64_NO_USIZE_TRAIT);
    }

    #[test]
    fn wasm32_explicit_u64_friend_is_retained() {
        assert_compiles(&ACCEPTED_CHECKED, &EXPLICIT_U64_FRIEND);
    }

    #[test]
    fn wasm32_bitflag_nested_usize_has_no_fifth_byte_accessor() {
        assert_compiles(&ACCEPTED_CHECKED, &BITFLAG_NO_FIFTH_BYTE);
    }

    #[test]
    fn wasm32_bitflag_nested_usize_has_infallible_u32_conversion() {
        assert_compiles(&ACCEPTED_CHECKED, &BITFLAG_U32_CONVERSION);
    }

    #[test]
    fn wasm32_usize_source_is_within_u32() {
        assert_compiles(&ACCEPTED_CHECKED, &WITHIN_USIZE_INTO_U32);
    }
}

#[cfg(test)]
mod rejected {
    use super::*;

    #[test]
    fn wasm32_u64_is_not_an_automatic_usize_make_friend() {
        assert_rejected(&REJECTED_CHECKED, &U64_NOT_AUTOMATIC_FRIEND);
    }

    #[test]
    fn wasm32_bitflag_nested_usize_has_no_automatic_u64_friend() {
        assert_rejected(&REJECTED_CHECKED, &BITFLAG_U64_NOT_AUTOMATIC_FRIEND);
    }
}
