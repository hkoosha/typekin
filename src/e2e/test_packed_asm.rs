use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unoptimized_packed_asm_level0() {
        assert_ok(&PACKED, OptLevel::O0, Expect::DiffAny, PACKED_OPERATIONS);
    }

    #[test]
    fn optimized_packed_asm_level1() {
        assert_ok(&PACKED, OptLevel::O1, Expect::Match, PACKED_OPERATIONS);
    }

    #[test]
    fn optimized_packed_asm_level2() {
        assert_ok(&PACKED, OptLevel::O2, Expect::Match, PACKED_OPERATIONS);
    }

    #[test]
    fn optimized_packed_asm_level3() {
        assert_ok(&PACKED, OptLevel::O3, Expect::Match, PACKED_OPERATIONS);
    }

    #[test]
    fn optimized_packed_asm_level_s() {
        assert_ok(&PACKED, OptLevel::Os, Expect::Match, PACKED_OPERATIONS);
    }

    #[test]
    fn checked_getter_keeps_check_level2() {
        assert_ok(
            &PACKED,
            OptLevel::O2,
            Expect::DiffEach,
            PACKED_CONTROL_OPERATIONS,
        );
    }
}

#[cfg(test)]
mod opt_level_z {
    use super::*;

    #[test]
    fn optimized_packed_asm_level_z() {
        assert_ok(&PACKED, OptLevel::Oz, Expect::Match, PACKED_OPERATIONS);
    }

    #[test]
    fn checked_getter_keeps_check_level_z() {
        assert_ok(
            &PACKED,
            OptLevel::Oz,
            Expect::DiffEach,
            PACKED_CONTROL_OPERATIONS,
        );
    }
}
