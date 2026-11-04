use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unoptimized_within_asm_level0() {
        assert_ok(&WITHIN, OptLevel::O0, Expect::DiffAny, WITHIN_OPERATIONS);
    }

    #[test]
    fn optimized_within_asm_level1() {
        assert_ok(&WITHIN, OptLevel::O1, Expect::Match, WITHIN_OPERATIONS);
    }

    #[test]
    fn optimized_within_asm_level2() {
        assert_ok(&WITHIN, OptLevel::O2, Expect::Match, WITHIN_OPERATIONS);
    }

    #[test]
    fn optimized_within_asm_level3() {
        assert_ok(&WITHIN, OptLevel::O3, Expect::Match, WITHIN_OPERATIONS);
    }

    #[test]
    fn optimized_within_asm_level_s() {
        assert_ok(&WITHIN, OptLevel::Os, Expect::Match, WITHIN_OPERATIONS);
    }

    #[test]
    fn validated_friend_keeps_validation_level2() {
        assert_ok(
            &WITHIN,
            OptLevel::O2,
            Expect::DiffEach,
            WITHIN_CONTROL_OPERATIONS,
        );
    }
}

#[cfg(test)]
mod opt_level_z {
    use super::*;

    #[test]
    fn optimized_within_asm_level_z() {
        assert_ok(&WITHIN, OptLevel::Oz, Expect::Match, WITHIN_OPERATIONS);
    }

    #[test]
    fn validated_friend_keeps_validation_level_z() {
        assert_ok(
            &WITHIN,
            OptLevel::Oz,
            Expect::DiffEach,
            WITHIN_CONTROL_OPERATIONS,
        );
    }
}
