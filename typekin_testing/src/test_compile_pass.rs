//! Consumers that only a separate crate can express: another crate using
//! exported literal macros or `Within` sources, `cargo check` alone, and
//! crate-level incomplete features.

use crate::fx_compile_pass::*;
use crate::*;

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod tests {
    use super::*;

    #[test]
    fn public_literal_is_usable_from_another_crate() {
        assert_compiles(&LITERAL, &PUBLIC_LITERAL);
    }

    #[test]
    fn const_parameter_types_keep_their_generic_scope() {
        assert_compiles(&DEPENDENT, &CONST_PARAMETER_TYPES);
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod within {
    use super::*;

    #[test]
    fn within_accepts_a_source_from_another_crate() {
        assert_compiles(&LITERAL, &WITHIN_OTHER_CRATE_SOURCE);
    }

    #[test]
    fn within_generic_target_proves_only_where_instantiated() {
        assert_compiles(&LITERAL, &WITHIN_GENERIC_UNINSTANTIATED);
    }
}
