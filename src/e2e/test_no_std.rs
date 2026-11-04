use crate::fx_no_std::*;
use crate::*;

#[cfg(test)]
mod core_only {
    use super::*;

    #[test]
    fn integral_compiles_for_a_no_std_consumer() {
        assert_compiles(&CORE_CHECKED, &INTEGRAL);
    }

    #[test]
    fn bitflag_compiles_for_a_no_std_consumer() {
        assert_compiles(&CORE_CHECKED, &BITFLAG);
    }

    #[test]
    fn float_compiles_for_a_no_std_consumer() {
        assert_compiles(&CORE_CHECKED, &FLOAT);
    }

    #[test]
    fn validated_float_compiles_for_a_no_std_consumer() {
        assert_compiles(&CORE_CHECKED, &VALIDATED_FLOAT);
    }

    #[test]
    fn float_point_compiles_for_a_no_std_consumer() {
        assert_compiles(&CORE_CHECKED, &FLOAT_POINT);
    }

    #[test]
    fn nightly_math_float_compiles_for_a_no_std_consumer() {
        assert_compiles(&CORE_CHECKED, &NIGHTLY_MATH_FLOAT);
    }
}

#[cfg(test)]
mod with_alloc {
    use super::*;

    #[test]
    fn text_compiles_for_a_no_std_alloc_consumer() {
        assert_compiles(&ALLOC_CHECKED, &TEXT);
    }
}

#[cfg(test)]
mod with_alloc_and_serde {
    use super::*;

    #[test]
    fn serde_compiles_for_a_no_std_alloc_consumer() {
        assert_compiles(&SERDE_CHECKED, &SERDE_TYPES);
    }
}

#[cfg(test)]
mod without_alloc_and_serde_and_stable {
    use super::*;

    #[test]
    fn generic_integrals_compile_in_no_std_on_stable() {
        assert_compiles(&STABLE_CHECKED, &GENERIC);
    }
}
