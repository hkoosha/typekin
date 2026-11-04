//! The existing fixtures compiled for other architectures. The host suites
//! cover x86_64; these never link or run the fixture. Each target has its
//! own feature because building for it needs that rustup target.

use crate::fx_cross_asm::*;
use crate::*;

#[cfg(test)]
#[cfg(feature = "test_slow_aarch64")]
mod aarch64 {
    use super::*;

    #[test]
    fn unoptimized_integral_aarch64_level0() {
        assert_cross(
            &MY_U32,
            AARCH64,
            OptLevel::O0,
            Expect::DiffEach,
            U32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_integral_aarch64_level1() {
        assert_cross(
            &MY_U32,
            AARCH64,
            OptLevel::O1,
            Expect::Match,
            U32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_integral_aarch64_level2() {
        assert_cross(
            &MY_U32,
            AARCH64,
            OptLevel::O2,
            Expect::Match,
            U32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_integral_aarch64_level3() {
        assert_cross(
            &MY_U32,
            AARCH64,
            OptLevel::O3,
            Expect::Match,
            U32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_integral_aarch64_level_s() {
        assert_cross(
            &MY_U32,
            AARCH64,
            OptLevel::Os,
            Expect::Match,
            U32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_integral_const_aarch64_level3() {
        assert_cross(
            &MY_U64_CONST,
            AARCH64,
            OptLevel::O3,
            Expect::Match,
            U64_CONST_OPERATIONS,
        );
    }

    #[test]
    fn optimized_map_aarch64_level3() {
        assert_cross(
            &MY_U32,
            AARCH64,
            OptLevel::O3,
            Expect::Match,
            U32_MAP_OPERATIONS,
        );
    }

    #[test]
    fn optimized_circular_output_aarch64_level3() {
        assert_cross(
            &MY_SEQUENCE,
            AARCH64,
            OptLevel::O3,
            Expect::Match,
            SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_generic_sequence_aarch64_level3() {
        assert_cross(
            &MY_SEQUENCE,
            AARCH64,
            OptLevel::O3,
            Expect::Match,
            GENERIC_SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn unoptimized_domain_int_aarch64_level0() {
        assert_cross(
            &DOMAIN_INT,
            AARCH64,
            OptLevel::O0,
            Expect::DiffAny,
            DOMAIN_INT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_domain_int_aarch64_level3() {
        assert_cross(
            &DOMAIN_INT,
            AARCH64,
            OptLevel::O3,
            Expect::Match,
            DOMAIN_INT_OPERATIONS,
        );
    }

    #[test]
    fn unoptimized_f64_aarch64_level0() {
        assert_cross(
            &MY_F64,
            AARCH64,
            OptLevel::O0,
            Expect::DiffAny,
            F64_OPERATIONS,
        );
    }

    #[test]
    fn optimized_f64_aarch64_level1() {
        assert_cross(
            &MY_F64,
            AARCH64,
            OptLevel::O1,
            Expect::Match,
            F64_SIZE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_f64_aarch64_level2() {
        assert_cross(
            &MY_F64,
            AARCH64,
            OptLevel::O2,
            Expect::Match,
            F64_OPERATIONS,
        );
    }

    #[test]
    fn optimized_f64_aarch64_level3() {
        assert_cross(
            &MY_F64,
            AARCH64,
            OptLevel::O3,
            Expect::Match,
            F64_OPERATIONS,
        );
    }

    #[test]
    fn optimized_f64_aarch64_level_s() {
        assert_cross(
            &MY_F64,
            AARCH64,
            OptLevel::Os,
            Expect::Match,
            F64_SIZE_OPERATIONS,
        );
    }

    #[test]
    fn unoptimized_f32_aarch64_level0() {
        assert_cross(
            &MY_F32,
            AARCH64,
            OptLevel::O0,
            Expect::DiffAny,
            F32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_f32_aarch64_level3() {
        assert_cross(
            &MY_F32,
            AARCH64,
            OptLevel::O3,
            Expect::Match,
            F32_OPERATIONS,
        );
    }

    #[test]
    fn unoptimized_unit_aarch64_level0() {
        assert_cross(
            &MY_UNIT,
            AARCH64,
            OptLevel::O0,
            Expect::DiffAny,
            UNIT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_unit_aarch64_level1() {
        assert_cross(
            &MY_UNIT,
            AARCH64,
            OptLevel::O1,
            Expect::Match,
            UNIT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_unit_aarch64_level2() {
        assert_cross(
            &MY_UNIT,
            AARCH64,
            OptLevel::O2,
            Expect::Match,
            UNIT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_unit_aarch64_level3() {
        assert_cross(
            &MY_UNIT,
            AARCH64,
            OptLevel::O3,
            Expect::Match,
            UNIT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_unit_aarch64_level_s() {
        assert_cross(
            &MY_UNIT,
            AARCH64,
            OptLevel::Os,
            Expect::Match,
            UNIT_OPERATIONS,
        );
    }

    #[test]
    fn unoptimized_i32_aarch64_level0() {
        assert_cross(
            &MY_I32,
            AARCH64,
            OptLevel::O0,
            Expect::DiffAny,
            I32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_i32_aarch64_level1() {
        assert_cross(
            &MY_I32,
            AARCH64,
            OptLevel::O1,
            Expect::Match,
            I32_SIZE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_i32_aarch64_level2() {
        assert_cross(
            &MY_I32,
            AARCH64,
            OptLevel::O2,
            Expect::Match,
            I32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_i32_aarch64_level3() {
        assert_cross(
            &MY_I32,
            AARCH64,
            OptLevel::O3,
            Expect::Match,
            I32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_i32_aarch64_level_s() {
        assert_cross(
            &MY_I32,
            AARCH64,
            OptLevel::Os,
            Expect::Match,
            I32_SIZE_OPERATIONS,
        );
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_armv7")]
mod armv7 {
    use super::*;

    #[test]
    fn optimized_integral_armv7_level3() {
        assert_cross(
            &MY_U32,
            ARMV7,
            OptLevel::O3,
            Expect::Match,
            U32_OPERATIONS,
        );
    }

    #[test]
    fn unoptimized_f64_armv7_level0() {
        assert_cross(
            &MY_F64,
            ARMV7,
            OptLevel::O0,
            Expect::DiffAny,
            F64_OPERATIONS,
        );
    }

    #[test]
    fn optimized_f64_armv7_level1() {
        assert_cross(
            &MY_F64,
            ARMV7,
            OptLevel::O1,
            Expect::Match,
            F64_SIZE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_f64_armv7_level2() {
        assert_cross(
            &MY_F64,
            ARMV7,
            OptLevel::O2,
            Expect::Match,
            F64_OPERATIONS,
        );
    }

    #[test]
    fn optimized_f64_armv7_level3() {
        assert_cross(
            &MY_F64,
            ARMV7,
            OptLevel::O3,
            Expect::Match,
            F64_OPERATIONS,
        );
    }

    #[test]
    fn optimized_f64_armv7_level_s() {
        assert_cross(
            &MY_F64,
            ARMV7,
            OptLevel::Os,
            Expect::Match,
            F64_SIZE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_unit_armv7_level3() {
        assert_cross(
            &MY_UNIT,
            ARMV7,
            OptLevel::O3,
            Expect::Match,
            UNIT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_i32_armv7_level3() {
        assert_cross(
            &MY_I32,
            ARMV7,
            OptLevel::O3,
            Expect::Match,
            I32_OPERATIONS,
        );
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_i686")]
mod i686 {
    use super::*;

    #[test]
    fn optimized_integral_i686_level3() {
        assert_cross(
            &MY_U32,
            I686,
            OptLevel::O3,
            Expect::Match,
            U32_OPERATIONS,
        );
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_aarch64")]
#[cfg(feature = "test_slow_z")]
mod aarch64_opt_level_z {
    use super::*;

    #[test]
    fn optimized_integral_aarch64_level_z() {
        assert_cross(
            &MY_U32,
            AARCH64,
            OptLevel::Oz,
            Expect::Match,
            U32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_f64_aarch64_level_z() {
        assert_cross(
            &MY_F64,
            AARCH64,
            OptLevel::Oz,
            Expect::Match,
            F64_SIZE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_f32_aarch64_level_z() {
        assert_cross(
            &MY_F32,
            AARCH64,
            OptLevel::Oz,
            Expect::Match,
            F32_SIZE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_unit_aarch64_level_z() {
        assert_cross(
            &MY_UNIT,
            AARCH64,
            OptLevel::Oz,
            Expect::Match,
            UNIT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_i32_aarch64_level_z() {
        assert_cross(
            &MY_I32,
            AARCH64,
            OptLevel::Oz,
            Expect::Match,
            I32_SIZE_OPERATIONS,
        );
    }
}

#[cfg(test)]
#[cfg(feature = "test_slow_armv7")]
#[cfg(feature = "test_slow_z")]
mod armv7_opt_level_z {
    use super::*;

    #[test]
    fn optimized_integral_armv7_level_z() {
        assert_cross(
            &MY_U32,
            ARMV7,
            OptLevel::Oz,
            Expect::Match,
            U32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_domain_int_armv7_level_z() {
        assert_cross(
            &DOMAIN_INT,
            ARMV7,
            OptLevel::Oz,
            Expect::Match,
            DOMAIN_INT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_f64_armv7_level_z() {
        assert_cross(
            &MY_F64,
            ARMV7,
            OptLevel::Oz,
            Expect::Match,
            F64_SIZE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_unit_armv7_level_z() {
        assert_cross(
            &MY_UNIT,
            ARMV7,
            OptLevel::Oz,
            Expect::Match,
            UNIT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_i32_armv7_level_z() {
        assert_cross(
            &MY_I32,
            ARMV7,
            OptLevel::Oz,
            Expect::Match,
            I32_SIZE_OPERATIONS,
        );
    }
}
