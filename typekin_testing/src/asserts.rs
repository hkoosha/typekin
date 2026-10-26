use crate::ops::Operation;
use crate::{
    asm,
    compile,
};
use std::fmt::Debug;
use std::hash::Hash;
use std::path::Path;

#[must_use]
pub fn assert_match(
    binary: &Path,
    operations: &[Operation],
) -> (Vec<String>, Vec<String>) {
    let symbols = asm::symbols(binary);
    let mut expected = Vec::new();
    let mut unexpected = Vec::new();

    for it in operations {
        let raw_addr = asm::symbol_addr(&symbols, it.raw, binary);
        let wrapped_addr = asm::symbol_addr(&symbols, it.wrapped, binary);
        if raw_addr == wrapped_addr {
            continue;
        }

        let raw = asm::assembly_text(binary, it.raw);
        let wrapped = asm::assembly_text(binary, it.wrapped);

        if raw != wrapped {
            unexpected.push(asm::text_diff(
                it,
                raw_addr,
                wrapped_addr,
                &raw,
                &wrapped,
            ));
        }
        else {
            expected.push(raw);
        }
    }

    return (expected, unexpected);
}

#[must_use]
pub fn assert_normalized_diff(
    binary: &Path,
    operations: &[Operation],
) -> (Vec<String>, Vec<String>) {
    let symbols = asm::symbols(binary);
    let mut diffs: Vec<String> = Vec::new();
    let mut unexpected = Vec::new();

    for it in operations {
        let raw = asm::normalized_assembly(binary, it.raw);
        let raw_addr = asm::symbol_addr(&symbols, it.raw, binary);

        let wrapped_addr = asm::symbol_addr(&symbols, it.wrapped, binary);
        let wrapped = asm::normalized_assembly(binary, it.wrapped);

        if raw == wrapped {
            unexpected.push(asm::assembly_list_match(
                it,
                raw_addr,
                wrapped_addr,
                &raw,
            ));
        }
        else {
            diffs.push(asm::text_diff(
                it,
                raw_addr,
                wrapped_addr,
                &raw.join("\n"),
                &wrapped.join("\n"),
            ));
        }
    }

    return (diffs, unexpected);
}

#[must_use]
pub fn assert_diff(
    binary: &Path,
    operations: &[Operation],
) -> (Vec<String>, Vec<String>) {
    let symbols = asm::symbols(binary);
    let mut diffs = Vec::new();
    let mut unexpected = Vec::new();

    for it in operations {
        let raw_addr = asm::symbol_addr(&symbols, it.raw, binary);
        let raw = asm::assembly_text(binary, it.raw);

        let wrapped_addr = asm::symbol_addr(&symbols, it.wrapped, binary);
        let wrapped = asm::assembly_text(binary, it.wrapped);

        if raw == wrapped {
            unexpected.push(asm::assembly_match(
                it,
                raw_addr,
                wrapped_addr,
                &raw,
            ));
        }
        else {
            diffs.push(asm::text_diff(
                it,
                raw_addr,
                wrapped_addr,
                &raw,
                &wrapped,
            ));
        }
    }

    return (diffs, unexpected);
}

pub fn assert_panics<T>(f: impl FnOnce() -> T + std::panic::UnwindSafe) {
    std::panic::set_hook(Box::new(move |_| {}));
    let result = std::panic::catch_unwind(f);
    let _ = std::panic::take_hook();
    assert!(result.is_err());
}

pub fn unsafe_assert_panics<T>(f: impl FnOnce() -> T) {
    assert_panics(std::panic::AssertUnwindSafe(f))
}

pub fn accepts_copy_clone<T: Copy + Clone>() {}

pub fn accepts_ord<T: Ord>() {}

pub fn accepts_eq<T: Eq>() {}

pub fn accepts_hash<T: Hash>() {}

pub fn accepts_copy<T: Copy>() {}

pub fn accepts_clone<T: Clone>() {}

pub fn accepts_debug<T: Debug>() {}

pub fn assert_crate_compiles(
    name: &str,
    dependencies: &str,
    source: &str,
) {
    let output = compile(name, dependencies, source, true);
    assert!(
        output.status.success(),
        "failed, {name}:\n{}",
        output.stderr
    );
}
