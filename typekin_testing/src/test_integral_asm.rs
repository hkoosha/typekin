use crate::*;
use std::process::Command;

fn assert_ok(
    name: &str,
    bin_name: &str,
    opt_level: usize,
    must_match: bool,
    operations: &[Operation],
) {
    let dir = make_dir(name);

    let bin = dir.join("release").join(bin_name);
    build(opt_level, &bin_name, &dir);
    exe("bin", Command::new(&bin));

    let (expected, unexpected) = match must_match {
        true => assert_match(&bin, operations),
        false => assert_diff(&bin, operations),
    };

    if !expected.is_empty() {
        let it = format!("expected found:\n\n{}", expected.join("\n"));
        eprintln!("{}", it);
    }
    if !unexpected.is_empty() {
        let it = format!("unexpected:\n\n{}", unexpected.join("\n"));
        panic!("{}", it);
    }
}

#[cfg(feature = "slow_tests")]
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
}
