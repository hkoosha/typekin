#[cfg(all(test, feature = "test_slow_only"))]
mod tests {
    use crate::compile_for_target;
    use std::path::Path;
    use std::process::Command;
    use std::sync::LazyLock;

    const TARGET: &str = "wasm32-unknown-unknown";

    fn target_installed() -> bool {
        static INSTALLED: LazyLock<bool> = LazyLock::new(|| {
            let output = Command::new("rustc")
                .args([
                    "+nightly",
                    "--print",
                    "target-libdir",
                    "--target",
                    TARGET,
                ])
                .output()
                .expect("failed to query nightly target library directory");
            assert!(
                output.status.success(),
                "rustc target query failed: {output:?}"
            );
            let directory = String::from_utf8(output.stdout)
                .expect("target library directory is not UTF-8");
            let Ok(entries) = std::fs::read_dir(Path::new(directory.trim()))
            else {
                eprintln!(
                    "skipping wasm32 fixture: nightly {TARGET} target is not installed"
                );
                return false;
            };
            let installed = entries.filter_map(Result::ok).any(|entry| {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                name.starts_with("libcore-") && name.ends_with(".rlib")
            });
            if !installed {
                eprintln!(
                    "skipping wasm32 fixture: nightly {TARGET} target is not installed"
                );
            }
            return installed;
        });
        return *INSTALLED;
    }

    fn accepts(
        name: &str,
        source: &str,
    ) {
        if !target_installed() {
            return;
        }
        let output = compile_for_target(name, "", source, true, Some(TARGET));
        assert!(output.status.success(), "{name}: {output:?}");
    }

    fn rejects(
        name: &str,
        expected: &str,
        source: &str,
    ) {
        if !target_installed() {
            return;
        }
        let output = compile_for_target(name, "", source, true, Some(TARGET));
        assert!(
            !output.status.success(),
            "unexpectedly compiled {name}: {output:?}"
        );
        assert!(
            output.stderr.contains(expected),
            "{name}: expected {expected:?}; {output:?}"
        );
    }

    #[test]
    fn wasm32_u64_to_usize_checked_conversion_rejects_truncation() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none)]
struct Wide(u64);
const LARGE: Wide = Wide(1u64 << 32);
const _: () = match LARGE.try_into_usize() {
    Err(()) => (),
    Ok(_) => panic!("u64 overflow accepted"),
};
const _: () = match Wide(u32::MAX as u64).try_into_usize() {
    Ok(value) if value == u32::MAX as usize => (),
    _ => panic!("representable u64 rejected"),
};
fn checked_trait(value: Wide) -> Result<usize, ()> { value.try_into() }
"#;
        accepts("wasm-u64-to-usize-checked", SRC);
    }

    #[test]
    fn wasm32_u32_to_isize_rejects_values_above_the_signed_maximum() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none)]
struct Number(u32);
const _: () = match Number(u32::MAX).try_into_isize() {
    Err(()) => (),
    Ok(_) => panic!("u32 overflow accepted"),
};
const _: () = match Number(i32::MAX as u32).try_into_isize() {
    Ok(value) if value == isize::MAX => (),
    _ => panic!("representable u32 rejected"),
};
fn checked_trait(value: Number) -> Result<isize, ()> { value.try_into() }
"#;
        accepts("wasm-u32-to-isize-checked", SRC);
    }

    #[test]
    fn wasm32_u32_to_usize_remains_infallible() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none)]
struct Number(u32);
const _: () = assert!(Number(u32::MAX).into_usize() == usize::MAX);
fn infallible_trait(value: Number) -> usize { value.into() }
"#;
        accepts("wasm-u32-to-usize-infallible", SRC);
    }

    #[test]
    fn wasm32_usize_has_no_fifth_byte_accessor() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none)]
struct Pointer(usize);
fn invalid(value: Pointer) { let _ = value.byte4(); }
"#;
        rejects("wasm-usize-byte4", "no method named `byte4`", SRC);
    }

    #[test]
    fn wasm32_usize_keeps_the_fourth_byte_accessor() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none)]
struct Pointer(usize);
const _: () = assert!(Pointer(0x1234_5678).byte3() == 0x12);
"#;
        accepts("wasm-usize-byte3", SRC);
    }

    #[test]
    fn wasm32_u64_has_no_infallible_usize_method() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none)]
struct Wide(u64);
fn invalid(value: Wide) { let _ = value.into_usize(); }
"#;
        rejects("wasm-u64-into-usize", "no method named `into_usize`", SRC);
    }

    #[test]
    fn wasm32_u64_has_no_infallible_usize_trait() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none)]
struct Wide(u64);
fn invalid(value: Wide) -> usize { value.into() }
"#;
        rejects("wasm-u64-from-usize", "From<Wide>", SRC);
    }

    #[test]
    fn wasm32_u64_is_not_an_automatic_usize_make_friend() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none)]
struct Pointer(usize);
fn invalid() { let _ = Pointer::of(1u64); }
"#;
        rejects("wasm-u64-implicit-friend", "Make", SRC);
    }

    #[test]
    fn wasm32_explicit_u64_friend_is_retained() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none, friend = _(u64) -> Make)]
struct Pointer(usize);
fn explicit() -> usize { Pointer::of(23u64).raw() }
"#;
        accepts("wasm-explicit-u64-friend", SRC);
    }

    #[test]
    fn wasm32_signed_to_unsigned_rejects_negative() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none)]
struct Signed(isize);
const _: () = match Signed(-1).try_into_u64() {
    Err(()) => (),
    Ok(_) => panic!("negative value accepted"),
};
fn checked_trait(value: Signed) -> Result<u64, ()> { value.try_into() }
"#;
        accepts("wasm-negative-to-unsigned", SRC);
    }

    #[test]
    fn wasm32_isize_domain_does_not_merge_an_empty_minimum_range() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none, with = konst_try_make,
    domain = [isize::MIN..-2_147_483_648, -isize::MAX..])]
struct Domain(isize);
const _: () = match Domain::try_make(isize::MIN) {
    Err(_) => (),
    Ok(_) => panic!("minimum incorrectly included"),
};
const _: () = match Domain::try_make(-isize::MAX) {
    Ok(_) => (),
    Err(_) => panic!("adjacent value rejected"),
};
"#;
        accepts("wasm-domain-excludes-minimum", SRC);
    }

    #[test]
    fn wasm32_isize_domain_retains_a_singleton_minimum_range() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none, with = konst_try_make,
    domain = [-isize::MAX.., -2_147_483_648..=-2_147_483_648])]
struct Domain(isize);
const _: () = match Domain::try_make(isize::MIN) {
    Ok(_) => (),
    Err(_) => panic!("minimum incorrectly excluded"),
};
"#;
        accepts("wasm-domain-includes-minimum", SRC);
    }

    #[test]
    fn wasm32_pointer_bounds_validate_nonzero_domain() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = stable, domain = [isize::MIN..0, 1..=isize::MAX])]
struct Number(isize);
const _: () = match Number::try_make(0) {
    Err(0) => (),
    _ => panic!("zero accepted"),
};
const _: () = match Number::try_make(isize::MIN) {
    Ok(_) => (),
    Err(_) => panic!("minimum rejected"),
};
"#;
        accepts("wasm-pointer-nonzero-validation", SRC);
    }

    #[test]
    fn wasm32_pointer_bounds_preserve_nonzero_niche() {
        const SRC: &str = r#"
#![no_std]
#[typekin::integral(konst = none, domain = [isize::MIN..0, 1..=isize::MAX])]
struct Number(isize);
const _: () = assert!(
    core::mem::size_of::<Option<Number>>() == core::mem::size_of::<isize>()
);
"#;
        accepts("wasm-pointer-nonzero-niche", SRC);
    }

    #[test]
    fn wasm32_bitflag_nested_usize_has_no_fifth_byte_accessor() {
        const SRC: &str = r#"
#![no_std]
#[typekin::bitflag(konst = none)]
#[repr(usize)]
enum Flag { One = 1 }
fn invalid(value: FlagValue) { let _ = value.byte4(); }
"#;
        rejects("wasm-bitflag-byte4", "no method named `byte4`", SRC);
    }

    #[test]
    fn wasm32_bitflag_nested_usize_has_no_automatic_u64_friend() {
        const SRC: &str = r#"
#![no_std]
#[typekin::bitflag(konst = none)]
#[repr(usize)]
enum Flag { One = 1 }
fn invalid() { let _ = FlagValue::of(1u64); }
"#;
        rejects("wasm-bitflag-implicit-u64-friend", "Make", SRC);
    }

    #[test]
    fn wasm32_bitflag_nested_usize_has_infallible_u32_conversion() {
        const SRC: &str = r#"
#![no_std]
#[typekin::bitflag(konst = none)]
#[repr(usize)]
enum Flag { One = 1 }
const _: () = assert!(FlagValue::One.into_u32() == 1);
fn infallible_trait(value: FlagValue) -> u32 { value.into() }
"#;
        accepts("wasm-bitflag-usize-to-u32", SRC);
    }

    #[test]
    fn wasm32_bitflag_nested_domain_includes_zero_but_excludes_minimum() {
        const SRC: &str = r#"
#![no_std]
#[typekin::bitflag(konst = stable, integral = [
    domain = [isize::MIN..-2_147_483_648, -isize::MAX..],
])]
#[repr(isize)]
enum Flag { One = 1 }
const _: () = match FlagValue::try_make(0) {
    Ok(_) => (),
    Err(_) => panic!("zero rejected"),
};
const _: () = match FlagValue::try_make(isize::MIN) {
    Err(_) => (),
    Ok(_) => panic!("minimum incorrectly included"),
};
"#;
        accepts("wasm-bitflag-domain-includes-zero", SRC);
    }

    #[test]
    fn wasm32_bitflag_nested_domain_cannot_exclude_zero() {
        const SRC: &str = r#"
#![no_std]
#[typekin::bitflag(konst = none, integral = [
    domain = [isize::MIN..-2_147_483_648, -isize::MAX..0],
])]
#[repr(isize)]
enum Flag { One = 1 }
"#;
        rejects(
            "wasm-bitflag-domain-excludes-zero",
            "bitflag integral domain must include zero",
            SRC,
        );
    }
}
