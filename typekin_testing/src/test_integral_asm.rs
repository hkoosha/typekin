use crate::*;

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod tests {
    use super::*;

    #[test]
    fn unoptimized_integral_asm_level0() {
        assert_ok(&MY_U32, OptLevel::O0, Expect::DiffEach, U32_OPERATIONS);
    }

    #[test]
    fn optimized_integral_asm_level3() {
        assert_ok(&MY_U32, OptLevel::O3, Expect::Match, U32_OPERATIONS);
    }

    #[test]
    fn optimized_integral_const_arm_level3() {
        assert_ok(
            &MY_U64_CONST,
            OptLevel::O3,
            Expect::Match,
            U64_CONST_OPERATIONS,
        );
    }

    #[test]
    fn optimized_map_asm_level3() {
        assert_ok(&MY_U32, OptLevel::O3, Expect::Match, U32_MAP_OPERATIONS);
    }

    #[test]
    fn optimized_circular_output_asm_level3() {
        assert_ok(
            &MY_SEQUENCE,
            OptLevel::O3,
            Expect::Match,
            SEQUENCE_OPERATIONS,
        );
    }

    /// `generic circular precedes` is identical at level 0 once jump targets
    /// are compared position-independently (rustc inlines the one
    /// `#[inline(always)]` layer even unoptimized), so the negative control is
    /// per fixture.
    #[test]
    fn unoptimized_generic_sequence_asm_level0() {
        assert_ok(
            &MY_SEQUENCE,
            OptLevel::O0,
            Expect::DiffAny,
            GENERIC_SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_generic_sequence_asm_level3() {
        assert_ok(
            &MY_SEQUENCE,
            OptLevel::O3,
            Expect::Match,
            GENERIC_SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_integral_asm_level1() {
        assert_ok(&MY_U32, OptLevel::O1, Expect::Match, U32_OPERATIONS);
    }

    #[test]
    fn optimized_integral_asm_level2() {
        assert_ok(&MY_U32, OptLevel::O2, Expect::Match, U32_OPERATIONS);
    }

    #[test]
    fn optimized_integral_asm_level_s() {
        assert_ok(&MY_U32, OptLevel::Os, Expect::Match, U32_OPERATIONS);
    }

    #[test]
    fn optimized_integral_const_asm_level1() {
        assert_ok(
            &MY_U64_CONST,
            OptLevel::O1,
            Expect::Match,
            U64_CONST_OPERATIONS,
        );
    }

    #[test]
    fn optimized_integral_const_asm_level2() {
        assert_ok(
            &MY_U64_CONST,
            OptLevel::O2,
            Expect::Match,
            U64_CONST_OPERATIONS,
        );
    }

    #[test]
    fn optimized_integral_const_asm_level_s() {
        assert_ok(
            &MY_U64_CONST,
            OptLevel::Os,
            Expect::Match,
            U64_CONST_OPERATIONS,
        );
    }

    #[test]
    fn optimized_map_asm_level1() {
        assert_ok(&MY_U32, OptLevel::O1, Expect::Match, U32_MAP_OPERATIONS);
    }

    #[test]
    fn optimized_map_asm_level2() {
        assert_ok(&MY_U32, OptLevel::O2, Expect::Match, U32_MAP_OPERATIONS);
    }

    #[test]
    fn optimized_map_asm_level_s() {
        assert_ok(&MY_U32, OptLevel::Os, Expect::Match, U32_MAP_OPERATIONS);
    }

    #[test]
    fn optimized_circular_output_asm_level1() {
        assert_ok(
            &MY_SEQUENCE,
            OptLevel::O1,
            Expect::Match,
            SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_circular_output_asm_level2() {
        assert_ok(
            &MY_SEQUENCE,
            OptLevel::O2,
            Expect::Match,
            SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_circular_output_asm_level_s() {
        assert_ok(
            &MY_SEQUENCE,
            OptLevel::Os,
            Expect::Match,
            SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_generic_sequence_asm_level1() {
        assert_ok(
            &MY_SEQUENCE,
            OptLevel::O1,
            Expect::Match,
            GENERIC_SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_generic_sequence_asm_level2() {
        assert_ok(
            &MY_SEQUENCE,
            OptLevel::O2,
            Expect::Match,
            GENERIC_SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_generic_sequence_asm_level_s() {
        assert_ok(
            &MY_SEQUENCE,
            OptLevel::Os,
            Expect::Match,
            GENERIC_SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn unoptimized_i32_asm_level0() {
        assert_ok(&MY_I32, OptLevel::O0, Expect::DiffAny, I32_OPERATIONS);
    }

    #[test]
    fn optimized_i32_asm_level1() {
        assert_ok(&MY_I32, OptLevel::O1, Expect::Match, I32_SIZE_OPERATIONS);
    }

    #[test]
    fn optimized_i32_asm_level2() {
        assert_ok(&MY_I32, OptLevel::O2, Expect::Match, I32_OPERATIONS);
    }

    #[test]
    fn optimized_i32_asm_level3() {
        assert_ok(&MY_I32, OptLevel::O3, Expect::Match, I32_OPERATIONS);
    }

    #[test]
    fn optimized_i32_asm_level_s() {
        assert_ok(&MY_I32, OptLevel::Os, Expect::Match, I32_SIZE_OPERATIONS);
    }

    #[test]
    fn optimized_i32_conversions_not_longer_level2() {
        assert_not_longer(&MY_I32, OptLevel::O2, I32_CONVERSION_OPERATIONS);
    }

    #[test]
    fn optimized_i32_conversions_not_longer_level3() {
        assert_not_longer(&MY_I32, OptLevel::O3, I32_CONVERSION_OPERATIONS);
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
#[cfg(feature = "test_slow_z")]
mod opt_level_z {
    use super::*;

    #[test]
    fn optimized_integral_asm_level_z() {
        assert_ok(&MY_U32, OptLevel::Oz, Expect::Match, U32_OPERATIONS);
    }

    #[test]
    fn optimized_integral_const_asm_level_z() {
        assert_ok(
            &MY_U64_CONST,
            OptLevel::Oz,
            Expect::Match,
            U64_CONST_OPERATIONS,
        );
    }

    #[test]
    fn optimized_map_asm_level_z() {
        assert_ok(&MY_U32, OptLevel::Oz, Expect::Match, U32_MAP_OPERATIONS);
    }

    #[test]
    fn optimized_circular_output_asm_level_z() {
        assert_ok(
            &MY_SEQUENCE,
            OptLevel::Oz,
            Expect::Match,
            SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_generic_sequence_asm_level_z() {
        assert_ok(
            &MY_SEQUENCE,
            OptLevel::Oz,
            Expect::Match,
            GENERIC_SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_i32_asm_level_z() {
        assert_ok(&MY_I32, OptLevel::Oz, Expect::Match, I32_SIZE_OPERATIONS);
    }
}
