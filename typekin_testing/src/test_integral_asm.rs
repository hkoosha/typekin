use crate::*;

#[cfg(test)]
#[cfg(feature = "test_slow_only")]
mod tests {
    use super::*;

    #[test]
    fn unoptimized_integral_asm_level0() {
        assert_ok(
            "unoptimized_integral_asm_level0",
            "my_u32",
            0,
            false,
            U32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_integral_asm_level3() {
        assert_ok(
            "optimized_integral_asm_level3",
            "my_u32",
            3,
            true,
            U32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_integral_const_arm_level3() {
        assert_ok(
            "optimized_integral_const_arm_level3",
            "my_u32_const",
            3,
            true,
            U32_OPERATIONS,
        );
    }

    #[test]
    fn optimized_map_asm_level3() {
        assert_ok(
            "optimized_map_asm_level3",
            "my_u32",
            3,
            true,
            U32_MAP_OPERATIONS,
        );
    }

    #[test]
    fn optimized_circular_output_asm_level3() {
        assert_ok(
            "optimized_circular_output_asm_level3",
            "my_sequence",
            3,
            true,
            SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn unoptimized_generic_sequence_asm_level0() {
        assert_ok(
            "unoptimized_generic_sequence_asm_level0",
            "my_sequence",
            0,
            false,
            GENERIC_SEQUENCE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_generic_sequence_asm_level3() {
        assert_ok(
            "optimized_generic_sequence_asm_level3",
            "my_sequence",
            3,
            true,
            GENERIC_SEQUENCE_OPERATIONS,
        );
    }
}
