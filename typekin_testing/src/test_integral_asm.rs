use crate::*;
use std::process::Command;

#[test]
fn unoptimized_integral() {
    let dir = make_dir("unoptimized_asm");
    let bin = dir.join("release").join("my_u32");

    build(0, "my_u32", &dir);

    exe("bin", Command::new(&bin));

    let (expected, unexpected) = assert_diff(&bin, U32_OPERATIONS);
    if !expected.is_empty() {
        let it = format!("expected found:\n\n{}", expected.join("\n"));
        eprintln!("{}", it);
    }
    if !unexpected.is_empty() {
        let it = format!("unexpected:\n\n{}", unexpected.join("\n"));
        panic!("{}", it);
    }
}

#[test]
fn optimized_integral() {
    let dir = make_dir("optimized_asm");
    let bin = dir.join("release").join("my_u32");

    build(3, "my_u32", &dir);
    exe("bin", Command::new(&bin));

    let (expected, unexpected) = assert_match(&bin, U32_OPERATIONS);
    if !expected.is_empty() {
        let it = format!("expected found:\n\n{}", expected.join("\n"));
        eprintln!("{}", it);
    }
    if !unexpected.is_empty() {
        let it = format!("unexpected:\n\n{}", unexpected.join("\n"));
        panic!("{}", it);
    }
}

#[test]
fn optimized_integral_mapping() {
    let dir = make_dir("optimized_map_asm");
    let bin = dir.join("release").join("my_u32");
    build(3, "my_u32", &dir);
    exe("bin", Command::new(&bin));

    let (expected, unexpected) = assert_match(&bin, U32_MAP_OPERATIONS);
    if !expected.is_empty() {
        let it = format!("expected found:\n\n{}", expected.join("\n"));
        eprintln!("{}", it);
    }
    if !unexpected.is_empty() {
        let it = format!("unexpected:\n\n{}", unexpected.join("\n"));
        panic!("{}", it);
    }
}

#[test]
fn optimized_integral_const() {
    let dir = make_dir("optimized_asm_const");
    let bin = dir.join("release").join("my_u32_const");

    build(3, "my_u32_const", &dir);
    exe("bin", Command::new(&bin));

    let (expected, unexpected) = assert_match(&bin, U32_OPERATIONS);
    if !expected.is_empty() {
        let it = format!("expected found:\n\n{}", expected.join("\n"));
        eprintln!("{}", it);
    }
    if !unexpected.is_empty() {
        let it = format!("unexpected:\n\n{}", unexpected.join("\n"));
        panic!("{}", it);
    }
}
