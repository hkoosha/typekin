use crate::*;

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod tests {
    use super::*;

    #[test]
    fn unoptimized_f64_asm_level0() {
        assert_ok(&MY_F64, OptLevel::O0, Expect::DiffAny, F64_OPERATIONS);
    }

    #[test]
    fn optimized_f64_asm_level1() {
        assert_ok(&MY_F64, OptLevel::O1, Expect::Match, F64_SIZE_OPERATIONS);
    }

    #[test]
    fn optimized_f64_asm_level2() {
        assert_ok(&MY_F64, OptLevel::O2, Expect::Match, F64_OPERATIONS);
    }

    #[test]
    fn optimized_f64_asm_level3() {
        assert_ok(&MY_F64, OptLevel::O3, Expect::Match, F64_OPERATIONS);
    }

    #[test]
    fn optimized_f64_asm_level_s() {
        assert_ok(&MY_F64, OptLevel::Os, Expect::Match, F64_SIZE_OPERATIONS);
    }

    #[test]
    fn unoptimized_f32_asm_level0() {
        assert_ok(&MY_F32, OptLevel::O0, Expect::DiffAny, F32_OPERATIONS);
    }

    #[test]
    fn optimized_f32_asm_level1() {
        assert_ok(&MY_F32, OptLevel::O1, Expect::Match, F32_SIZE_OPERATIONS);
    }

    #[test]
    fn optimized_f32_asm_level2() {
        assert_ok(&MY_F32, OptLevel::O2, Expect::Match, F32_OPERATIONS);
    }

    #[test]
    fn optimized_f32_asm_level3() {
        assert_ok(&MY_F32, OptLevel::O3, Expect::Match, F32_OPERATIONS);
    }

    #[test]
    fn optimized_f32_asm_level_s() {
        assert_ok(&MY_F32, OptLevel::Os, Expect::Match, F32_SIZE_OPERATIONS);
    }

    #[test]
    fn unoptimized_unit_asm_level0() {
        assert_ok(&MY_UNIT, OptLevel::O0, Expect::DiffAny, UNIT_OPERATIONS);
    }

    #[test]
    fn optimized_unit_asm_level1() {
        assert_ok(&MY_UNIT, OptLevel::O1, Expect::Match, UNIT_OPERATIONS);
    }

    #[test]
    fn optimized_unit_asm_level2() {
        assert_ok(&MY_UNIT, OptLevel::O2, Expect::Match, UNIT_OPERATIONS);
    }

    #[test]
    fn optimized_unit_asm_level3() {
        assert_ok(&MY_UNIT, OptLevel::O3, Expect::Match, UNIT_OPERATIONS);
    }

    #[test]
    fn optimized_unit_asm_level_s() {
        assert_ok(&MY_UNIT, OptLevel::Os, Expect::Match, UNIT_OPERATIONS);
    }

    #[test]
    fn optimized_unit_clamp_not_longer_level1() {
        assert_not_longer(&MY_UNIT, OptLevel::O1, UNIT_ASSERTING_OPERATIONS);
    }

    #[test]
    fn optimized_unit_clamp_not_longer_level2() {
        assert_not_longer(&MY_UNIT, OptLevel::O2, UNIT_ASSERTING_OPERATIONS);
    }

    #[test]
    fn optimized_unit_clamp_not_longer_level3() {
        assert_not_longer(&MY_UNIT, OptLevel::O3, UNIT_ASSERTING_OPERATIONS);
    }

    #[test]
    fn optimized_unit_clamp_not_longer_level_s() {
        assert_not_longer(&MY_UNIT, OptLevel::Os, UNIT_ASSERTING_OPERATIONS);
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
#[cfg(feature = "test_slow_z")]
mod opt_level_z {
    use super::*;

    #[test]
    fn optimized_f64_asm_level_z() {
        assert_ok(&MY_F64, OptLevel::Oz, Expect::Match, F64_SIZE_OPERATIONS);
    }

    #[test]
    fn optimized_f32_asm_level_z() {
        assert_ok(&MY_F32, OptLevel::Oz, Expect::Match, F32_SIZE_OPERATIONS);
    }

    #[test]
    fn optimized_unit_asm_level_z() {
        assert_ok(&MY_UNIT, OptLevel::Oz, Expect::Match, UNIT_OPERATIONS);
    }

    #[test]
    fn optimized_unit_clamp_not_longer_level_z() {
        assert_not_longer(&MY_UNIT, OptLevel::Oz, UNIT_ASSERTING_OPERATIONS);
    }
}
