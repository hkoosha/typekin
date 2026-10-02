use std::process::Command;

use crate::*;

const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");
// let _ = std::fs::remove_dir_all(&dir);

#[test]
fn unoptimized_integral() {
    const CTX: &str = "unoptimized_asm";
    const FIX: &str = "my_u32";
    const OPS: &[Operation] = U32_OPERATIONS;
    const OPZ: &str = "0";

    let dir = make_dir(CTX);
    let bin = dir.join("release").join(FIX);
    build(OPZ, FIX, &dir, MANIFEST);
    exe("bin", Command::new(&bin));

    let (expected, unexpected) = assert_diff(&bin, OPS);
    // if !expected.is_empty() {
    //     let it = format!("expected found:\n\n{}", expected.join("\n"));
    //     eprintln!("{}", it);
    // }
    if !unexpected.is_empty() {
        let it = format!("unexpected:\n\n{}", unexpected.join("\n"));
        panic!("{}", it);
    }
}

#[test]
fn optimized_integral() {
    const CTX: &str = "optimized_asm";
    const FIX: &str = "my_u32";
    const OPS: &[Operation] = U32_OPERATIONS;
    const OPZ: &str = "3";

    let dir = make_dir(CTX);
    let bin = dir.join("release").join(FIX);
    build(OPZ, FIX, &dir, MANIFEST);
    exe("bin", Command::new(&bin));

    let (expected, unexpected) = assert_match(&bin, OPS);
    // if !expected.is_empty() {
    //     let it = format!("expected found:\n\n{}", expected.join("\n"));
    //     eprintln!("{}", it);
    // }
    if !unexpected.is_empty() {
        let it = format!("unexpected:\n\n{}", unexpected.join("\n"));
        panic!("{}", it);
    }
}

#[test]
fn optimized_integral_const() {
    const CTX: &str = "optimized_asm_const";
    const FIX: &str = "my_u32_const";
    const OPS: &[Operation] = U32_OPERATIONS;
    const OPZ: &str = "3";

    let dir = make_dir(CTX);
    let bin = dir.join("release").join(FIX);
    build(OPZ, FIX, &dir, MANIFEST);
    exe("bin", Command::new(&bin));

    let (expected, unexpected) = assert_match(&bin, OPS);
    // if !expected.is_empty() {
    //     let it = format!("expected found:\n\n{}", expected.join("\n"));
    //     eprintln!("{}", it);
    // }
    if !unexpected.is_empty() {
        let it = format!("unexpected:\n\n{}", unexpected.join("\n"));
        panic!("{}", it);
    }
}
