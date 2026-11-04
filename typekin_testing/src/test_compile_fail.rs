//! Rejections only rustc can report: privacy, name resolution, trait
//! bounds inside generated code, borrow checking, and constant evaluation.
//! Typekin's own diagnostics are unit-tested in `src/tests/`, and missing
//! traits or methods are asserted in the fast modules.

use crate::fx_compile_fail::*;
use crate::*;

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod hide_in {
    use super::*;

    #[test]
    fn hidden_type_rejects_tuple_construction_in_defining_module() {
        assert_rejected(&CHECKED, &HIDDEN_TUPLE_CONSTRUCTION);
    }

    #[test]
    fn hidden_type_rejects_field_access_in_defining_module() {
        assert_rejected(&CHECKED, &HIDDEN_FIELD_ACCESS);
    }

    #[test]
    fn hidden_type_cannot_see_fn_local_validator() {
        assert_rejected(&CHECKED, &HIDDEN_FN_LOCAL_VALIDATOR);
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod friends {
    use super::*;

    #[test]
    fn friend_construction_cannot_change_direction() {
        assert_rejected(&CHECKED, &FRIEND_DIRECTION);
    }

    #[test]
    fn float_rejects_a_lossy_integral_newtype_friend() {
        assert_rejected(&CHECKED, &LOSSY_INTEGRAL_FRIEND);
    }

    #[test]
    fn integral_rejects_a_float_newtype_friend() {
        assert_rejected(&CHECKED, &FLOAT_FRIEND_OF_INTEGRAL);
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod generic_markers {
    use super::*;

    #[test]
    fn lifetime_markers_cannot_escape_their_borrow() {
        assert_rejected(&CHECKED, &LIFETIME_ESCAPE);
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod literal_macros {
    use super::*;

    #[test]
    fn literal_rejects_a_same_named_type_at_the_call_site() {
        assert_rejected(&CHECKED, &LITERAL_SAME_NAMED_TYPE);
    }

    #[test]
    fn literal_item_form_rejects_outer_generic_parameters() {
        assert_rejected(&CHECKED, &LITERAL_OUTER_GENERICS);
    }

    #[test]
    fn literal_rejects_a_value_outside_the_domain() {
        assert_rejected(&CHECKED, &LITERAL_OUTSIDE_DOMAIN);
    }

    #[test]
    fn literal_rejects_a_float_outside_the_domain() {
        assert_rejected(&CHECKED, &LITERAL_FLOAT_OUTSIDE_DOMAIN);
    }

    #[test]
    fn literal_rejects_a_value_failing_a_valid_callback() {
        assert_rejected(&CHECKED, &LITERAL_FAILING_CALLBACK);
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod constness {
    use super::*;

    #[test]
    fn float_domain_sqrt_is_not_const() {
        assert_rejected(&CHECKED, &SQRT_DOMAIN_NOT_CONST);
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod within {
    use super::*;

    #[test]
    fn within_refutes_a_value_outside_the_domain() {
        assert_rejected(&CHECKED, &WITHIN_OUTSIDE_DOMAIN);
    }

    #[test]
    fn within_refutes_a_value_outside_the_raw_range() {
        assert_rejected(&CHECKED, &WITHIN_OUTSIDE_RAW_RANGE);
    }

    #[test]
    fn within_table_refutes_a_failing_callback() {
        assert_rejected(&CHECKED, &WITHIN_FAILING_CALLBACK);
    }

    #[test]
    fn within_reports_an_opaque_source() {
        assert_rejected(&CHECKED, &WITHIN_OPAQUE_SOURCE);
    }

    #[test]
    fn within_reports_a_check_over_budget() {
        assert_rejected(&CHECKED, &WITHIN_OVER_BUDGET);
    }

    #[test]
    fn within_needs_the_source_constants() {
        assert_rejected(&CHECKED, &WITHIN_UNPUBLISHED_SOURCE);
    }

    #[test]
    fn within_refutes_a_source_from_another_crate() {
        assert_rejected(&CHECKED, &WITHIN_OTHER_CRATE);
    }

    #[test]
    fn within_refutes_a_usize_source_on_64_bit_targets() {
        assert_rejected(&CHECKED, &WITHIN_POINTER_WIDTH);
    }

    #[test]
    fn within_refutes_a_generic_target_where_instantiated() {
        assert_rejected(&BUILT, &WITHIN_GENERIC_TARGET);
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod packed {
    use super::*;

    #[test]
    fn packed_refutes_a_field_too_wide_for_its_window() {
        assert_rejected(&CHECKED, &PACKED_FIELD_TOO_WIDE);
    }

    #[test]
    fn packed_refutes_a_signed_field_too_wide_for_its_window() {
        assert_rejected(&CHECKED, &PACKED_SIGNED_FIELD_TOO_WIDE);
    }

    #[test]
    fn packed_cannot_prove_non_zero_when_every_field_admits_zero() {
        assert_rejected(&CHECKED, &PACKED_NON_ZERO_UNPROVEN);
    }

    #[test]
    fn packed_rejects_a_non_const_field_under_stable_konst() {
        assert_rejected(&CHECKED, &PACKED_NON_CONST_FIELD);
    }
}
