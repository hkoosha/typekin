use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unoptimized_domain_int_asm_level0() {
        assert_ok(
            &DOMAIN_INT,
            OptLevel::O0,
            Expect::DiffAny,
            DOMAIN_INT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_domain_int_asm_level1() {
        assert_ok(
            &DOMAIN_INT,
            OptLevel::O1,
            Expect::Match,
            DOMAIN_INT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_domain_int_asm_level2() {
        assert_ok(
            &DOMAIN_INT,
            OptLevel::O2,
            Expect::Match,
            DOMAIN_INT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_domain_int_asm_level3() {
        assert_ok(
            &DOMAIN_INT,
            OptLevel::O3,
            Expect::Match,
            DOMAIN_INT_OPERATIONS,
        );
    }

    #[test]
    fn optimized_domain_int_asm_level_s() {
        assert_ok(
            &DOMAIN_INT,
            OptLevel::Os,
            Expect::Match,
            DOMAIN_INT_OPERATIONS,
        );
    }

    #[test]
    fn domain_int_is_never_longer_than_written_level1() {
        assert_not_longer(&DOMAIN_INT, OptLevel::O1, DOMAIN_INT_WRITTEN);
    }

    #[test]
    fn domain_int_is_never_longer_than_written_level3() {
        assert_not_longer(&DOMAIN_INT, OptLevel::O3, DOMAIN_INT_WRITTEN);
    }
}

#[cfg(test)]
mod opt_level_z {
    use super::*;

    #[test]
    fn optimized_domain_int_asm_level_z() {
        assert_ok(
            &DOMAIN_INT,
            OptLevel::Oz,
            Expect::Match,
            DOMAIN_INT_OPERATIONS,
        );
    }

    #[test]
    fn domain_int_is_never_longer_than_written_level_z() {
        assert_not_longer(&DOMAIN_INT, OptLevel::Oz, DOMAIN_INT_WRITTEN);
    }
}
