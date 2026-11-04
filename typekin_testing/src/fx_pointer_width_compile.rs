use crate::*;
use std::process::Command;
use std::sync::LazyLock;

pub(crate) const TARGET: &str = "wasm32-unknown-unknown";

/// Fails instead of skipping: a missing target must not pass.
pub(crate) fn assert_target_installed() {
    let output = Command::new("rustup")
        .args(["+nightly", "target", "list", "--installed"])
        .output()
        .expect("cannot run rustup");
    let installed = String::from_utf8_lossy(&output.stdout);
    assert!(
        installed.lines().any(|it| it.trim() == TARGET),
        "{TARGET} is not installed; run `rustup +nightly target add {TARGET}`"
    );
}

// -------------------------------------------------------------------------
// Accepted

pub(crate) const ACCEPTED: Fixture = Fixture {
    name: "wasm32_accepted",
    toolchain: Toolchain::Nightly,
    target: Some(TARGET),
    header: concat!("#![no_std]\n", include_str!("machinery/absent.rs")),
    dependencies: "",
};

pub(crate) static ACCEPTED_CHECKED: LazyLock<Checked> = LazyLock::new(|| {
    assert_target_installed();
    return check(
        &ACCEPTED,
        &cases(&[
            &U64_TO_USIZE_CHECKED,
            &U32_TO_ISIZE_CHECKED,
            &U32_TO_USIZE_INFALLIBLE,
            &USIZE_NO_FIFTH_BYTE,
            &USIZE_FOURTH_BYTE,
            &U64_NO_USIZE_METHOD,
            &U64_NO_USIZE_TRAIT,
            &EXPLICIT_U64_FRIEND,
            &BITFLAG_NO_FIFTH_BYTE,
            &BITFLAG_U32_CONVERSION,
            &WITHIN_USIZE_INTO_U32,
        ]),
    );
});

pub(crate) const U64_TO_USIZE_CHECKED: Case = Case {
    name: "wasm32_u64_to_usize_checked_conversion_rejects_truncation",
    source: r#"
#[typekin::integral(konst = stable)]
struct Wide(u64);

const fn wide(raw: u64) -> Wide {
    return match Wide::try_make(raw) {
        Ok(value) => value,
        Err(_) => panic!("unvalidated value rejected"),
    };
}

const _: () = match wide(1u64 << 32).try_into_usize() {
    Err(()) => (),
    Ok(_) => panic!("u64 overflow accepted"),
};
const _: () = match wide(u32::MAX as u64).try_into_usize() {
    Ok(value) if value == u32::MAX as usize => (),
    _ => panic!("representable u64 rejected"),
};

fn checked_trait(value: Wide) -> Result<usize, ()> {
    return value.try_into();
}
"#,
};

pub(crate) const U32_TO_ISIZE_CHECKED: Case = Case {
    name: "wasm32_u32_to_isize_rejects_values_above_the_signed_maximum",
    source: r#"
#[typekin::integral(konst = stable)]
struct Number(u32);

const fn number(raw: u32) -> Number {
    return match Number::try_make(raw) {
        Ok(value) => value,
        Err(_) => panic!("unvalidated value rejected"),
    };
}

const _: () = match number(u32::MAX).try_into_isize() {
    Err(()) => (),
    Ok(_) => panic!("u32 overflow accepted"),
};
const _: () = match number(i32::MAX as u32).try_into_isize() {
    Ok(value) if value == isize::MAX => (),
    _ => panic!("representable u32 rejected"),
};

fn checked_trait(value: Number) -> Result<isize, ()> {
    return value.try_into();
}
"#,
};

pub(crate) const U32_TO_USIZE_INFALLIBLE: Case = Case {
    name: "wasm32_u32_to_usize_remains_infallible",
    source: r#"
#[typekin::integral(konst = stable)]
struct Number(u32);

const MAX: Number = match Number::try_make(u32::MAX) {
    Ok(value) => value,
    Err(_) => panic!("unvalidated value rejected"),
};

const _: () = assert!(MAX.into_usize() == usize::MAX);

fn infallible_trait(value: Number) -> usize {
    return value.into();
}
"#,
};

pub(crate) const USIZE_NO_FIFTH_BYTE: Case = Case {
    name: "wasm32_usize_has_no_fifth_byte_accessor",
    source: r#"
#[typekin::integral(konst = none)]
struct Pointer(usize);

fn absent() {
    assert_no_method!(Pointer, byte4);
}
"#,
};

pub(crate) const USIZE_FOURTH_BYTE: Case = Case {
    name: "wasm32_usize_keeps_the_fourth_byte_accessor",
    source: r#"
#[typekin::integral(konst = stable)]
struct Pointer(usize);

const POINTER: Pointer = match Pointer::try_make(0x1234_5678) {
    Ok(value) => value,
    Err(_) => panic!("unvalidated value rejected"),
};

const _: () = assert!(POINTER.byte3() == 0x12);
"#,
};

pub(crate) const U64_NO_USIZE_METHOD: Case = Case {
    name: "wasm32_u64_has_no_infallible_usize_method",
    source: r#"
#[typekin::integral(konst = none)]
struct Wide(u64);

fn absent() {
    assert_no_method!(Wide, into_usize);
}
"#,
};

pub(crate) const U64_NO_USIZE_TRAIT: Case = Case {
    name: "wasm32_u64_has_no_infallible_usize_trait",
    source: r#"
#[typekin::integral(konst = none)]
struct Wide(u64);

fn absent() {
    assert_not_impl!(Wide: Into<usize>);
}
"#,
};

pub(crate) const EXPLICIT_U64_FRIEND: Case = Case {
    name: "wasm32_explicit_u64_friend_is_retained",
    source: r#"
#[typekin::integral(konst = none, friend = _(u64) -> Make)]
struct Pointer(usize);

fn explicit() -> usize {
    return Pointer::of(23u64).raw();
}
"#,
};

pub(crate) const BITFLAG_NO_FIFTH_BYTE: Case = Case {
    name: "wasm32_bitflag_nested_usize_has_no_fifth_byte_accessor",
    source: r#"
#[typekin::bitflag(konst = none)]
#[repr(usize)]
enum Flag {
    One = 1,
}

fn absent() {
    assert_no_method!(FlagValue, byte4);
}
"#,
};

pub(crate) const BITFLAG_U32_CONVERSION: Case = Case {
    name: "wasm32_bitflag_nested_usize_has_infallible_u32_conversion",
    source: r#"
#[typekin::bitflag(konst = stable)]
#[repr(usize)]
enum Flag {
    One = 1,
}

const _: () = assert!(FlagValue::One.into_u32() == 1);

fn infallible_trait(value: FlagValue) -> u32 {
    return value.into();
}
"#,
};

/// A `usize` source fits `u32` at 32 bits; `test_compile_fail` rejects the
/// same edge on the 64-bit host.
pub(crate) const WITHIN_USIZE_INTO_U32: Case = Case {
    name: "wasm32_usize_source_is_within_u32",
    source: r#"
#[typekin::integral]
pub struct Index(usize);

#[typekin::integral(friend = _(Index) -> Within)]
pub struct Small(u32);

pub fn small(index: Index) -> Small {
    return Small::of(index);
}
"#,
};

// -------------------------------------------------------------------------
// Rejected: `of` is bounded by the private `Make` protocol, which no
// assertion in the accepted crate can name.

pub(crate) const REJECTED: Fixture = Fixture {
    name: "wasm32_rejected",
    toolchain: Toolchain::Nightly,
    target: Some(TARGET),
    header: "#![no_std]",
    dependencies: "",
};

pub(crate) static REJECTED_CHECKED: LazyLock<Checked> = LazyLock::new(|| {
    assert_target_installed();
    return check(
        &REJECTED,
        &rejections(&[
            &U64_NOT_AUTOMATIC_FRIEND,
            &BITFLAG_U64_NOT_AUTOMATIC_FRIEND,
        ]),
    );
});

pub(crate) const U64_NOT_AUTOMATIC_FRIEND: Rejection = Rejection {
    name: "wasm32_u64_is_not_an_automatic_usize_make_friend",
    expected: "the trait `__pointer::_::Make` is not implemented for `u64`",
    source: r#"
#[typekin::integral(konst = none)]
struct Pointer(usize);

fn invalid() {
    let _ = Pointer::of(1u64);
}
"#,
};

pub(crate) const BITFLAG_U64_NOT_AUTOMATIC_FRIEND: Rejection = Rejection {
    name: "wasm32_bitflag_nested_usize_has_no_automatic_u64_friend",
    expected: "the trait `__flag::_::Make` is not implemented for `u64`",
    source: r#"
#[typekin::bitflag(konst = none)]
#[repr(usize)]
enum Flag {
    One = 1,
}

fn invalid() {
    let _ = FlagValue::of(1u64);
}
"#,
};
