use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unoptimized_closure_asm_level0() {
        assert_ok(
            &COMPILE_TIME,
            OptLevel::O0,
            Expect::DiffAny,
            CLOSURE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_closure_asm_level1() {
        assert_ok(
            &COMPILE_TIME,
            OptLevel::O1,
            Expect::Match,
            CLOSURE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_closure_asm_level2() {
        assert_ok(
            &COMPILE_TIME,
            OptLevel::O2,
            Expect::Match,
            CLOSURE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_closure_asm_level3() {
        assert_ok(
            &COMPILE_TIME,
            OptLevel::O3,
            Expect::Match,
            CLOSURE_OPERATIONS,
        );
    }

    #[test]
    fn optimized_closure_asm_level_s() {
        assert_ok(
            &COMPILE_TIME,
            OptLevel::Os,
            Expect::Match,
            CLOSURE_OPERATIONS,
        );
    }

    #[test]
    fn unproven_operation_keeps_validation_level2() {
        assert_ok(
            &COMPILE_TIME,
            OptLevel::O2,
            Expect::DiffEach,
            CLOSURE_CONTROL_OPERATIONS,
        );
    }
}

#[cfg(test)]
mod opt_level_z {
    use super::*;

    #[test]
    fn optimized_closure_asm_level_z() {
        assert_ok(
            &COMPILE_TIME,
            OptLevel::Oz,
            Expect::Match,
            CLOSURE_OPERATIONS,
        );
    }

    #[test]
    fn unproven_operation_keeps_validation_level_z() {
        assert_ok(
            &COMPILE_TIME,
            OptLevel::Oz,
            Expect::DiffEach,
            CLOSURE_CONTROL_OPERATIONS,
        );
    }
}
