use std::sync::LazyLock;

use crate::*;

// -------------------------------------------------------------------------
// core only

pub(crate) const CORE: Fixture = Fixture {
    name: "no_std_core",
    toolchain: Toolchain::Nightly,
    target: None,
    header: "
#![no_std]
#![feature(core_float_math)]
",
    dependencies: "",
};

pub(crate) const INTEGRAL: Case = Case {
    name: "integral_compiles_for_a_no_std_consumer",
    source: r#"
const fn is_even(value: u8) -> bool {
    return value % 2 == 0;
}

#[typekin::integral(valid = is_even)]
pub struct Even(u8);

pub fn build_even(value: u8) -> Result<Even, u8> {
    return Even::try_make(value);
}

pub fn parse_even(value: &str) -> Result<Even, ()> {
    return value.parse();
}
"#,
};

pub(crate) const BITFLAG: Case = Case {
    name: "bitflag_compiles_for_a_no_std_consumer",
    source: r#"
#[typekin::bitflag(repr = u8)]
pub enum Permission {
    Read = 0b001,
    Write = 0b010,
}

pub fn build_permission(value: u8) -> PermissionValue {
    return Permission::from_bits_truncate(value);
}
"#,
};

pub(crate) const FLOAT: Case = Case {
    name: "float_compiles_for_a_no_std_consumer",
    source: r#"
#[typekin::float(konst = none)]
pub struct Meters(f64);

pub fn scale(value: Meters, by: f64) -> Meters {
    return -(value * by);
}

pub fn parse_meters(value: &str) -> Result<Meters, ()> {
    return value.parse();
}
"#,
};

pub(crate) const VALIDATED_FLOAT: Case = Case {
    name: "validated_float_compiles_for_a_no_std_consumer",
    source: r#"
#[typekin::float(domain = 0.0..=core::f64::consts::TAU)]
pub struct Angle(f64);

pub fn build_angle(value: f64) -> Result<Angle, f64> {
    return Angle::try_make(value);
}

pub fn order(lhs: Angle, rhs: Angle) -> core::cmp::Ordering {
    return lhs.cmp(&rhs);
}
"#,
};

pub(crate) const FLOAT_POINT: Case = Case {
    name: "float_point_compiles_for_a_no_std_consumer",
    source: r#"
#[typekin::point]
pub struct Instant(f32);

pub fn elapsed(start: Instant, end: Instant) -> Instant {
    return end - start;
}
"#,
};

pub(crate) const NIGHTLY_MATH_FLOAT: Case = Case {
    name: "nightly_math_float_compiles_for_a_no_std_consumer",
    source: r#"
#[typekin::float(konst = none, math = nightly, domain = floor in {0.0})]
pub struct Fraction(f64);

pub fn build_fraction(value: f64) -> Result<Fraction, f64> {
    return Fraction::try_make(value);
}

pub fn truncate(value: Fraction) -> Fraction {
    return value.trunc();
}

pub fn root(value: Fraction) -> Fraction {
    return value.sqrt();
}
"#,
};

pub(crate) static CORE_CHECKED: LazyLock<Checked> = LazyLock::new(|| {
    return check(
        &CORE,
        &cases(&[
            &INTEGRAL,
            &BITFLAG,
            &FLOAT,
            &VALIDATED_FLOAT,
            &FLOAT_POINT,
            &NIGHTLY_MATH_FLOAT,
        ]),
    );
});

// -------------------------------------------------------------------------
// alloc

pub(crate) const ALLOC: Fixture = Fixture {
    name: "no_std_alloc",
    toolchain: Toolchain::Nightly,
    target: None,
    header: "
#![no_std]

extern crate alloc;
",
    dependencies: "",
};

pub(crate) const TEXT: Case = Case {
    name: "text_compiles_for_a_no_std_alloc_consumer",
    source: r#"
use alloc::string::String;

fn is_slug(value: &str) -> bool {
    return !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-');
}

#[typekin::text(std = false, valid = is_slug)]
pub struct Slug(String);

pub fn build_slug(value: &str) -> Result<Slug, ()> {
    return Slug::try_from_str(value);
}
"#,
};

pub(crate) static ALLOC_CHECKED: LazyLock<Checked> = LazyLock::new(|| {
    return check(&ALLOC, &cases(&[&TEXT]));
});

// -------------------------------------------------------------------------
// Serde with alloc

pub(crate) const SERDE: Fixture = Fixture {
    name: "no_std_serde",
    toolchain: Toolchain::Nightly,
    target: None,
    header: "
#![no_std]

extern crate alloc;
",
    dependencies: r#"serde = { version = "1", default-features = false, features = ["alloc"] }"#,
};

pub(crate) const SERDE_TYPES: Case = Case {
    name: "serde_compiles_for_a_no_std_alloc_consumer",
    source: r#"
use alloc::string::String;

const fn is_even(value: u8) -> bool {
    return value % 2 == 0;
}

#[typekin::integral(valid = is_even, with = serde)]
pub struct Even(u8);

#[typekin::text(std = false, with = serde)]
pub struct Text(String);

#[typekin::bitflag(
    repr = u8,
    with = [serde, serde_with_name],
)]
pub enum Permission {
    Read = 0b001,
    Write = 0b010,
}
"#,
};

pub(crate) static SERDE_CHECKED: LazyLock<Checked> = LazyLock::new(|| {
    return check(&SERDE, &cases(&[&SERDE_TYPES]));
});

// -------------------------------------------------------------------------
// Stable, Serde without alloc

pub(crate) const STABLE: Fixture = Fixture {
    name: "no_std_stable",
    toolchain: Toolchain::Stable,
    target: None,
    header: "#![no_std]",
    dependencies: r#"serde = { version = "1", default-features = false }"#,
};

pub(crate) const GENERIC: Case = Case {
    name: "generic_integrals_compile_in_no_std_on_stable",
    source: r#"
#[typekin::integral(
    profile = wrapping {},
    with = [serde, math_xor],
)]
pub struct Stored<'de, T: ?Sized, const SPACE: u8>(u32);

pub fn transform<'a, T: ?Sized, const SPACE: u8>(raw: u32) -> u32 {
    let initial = Stored::<'a, T, SPACE>::make(raw);
    let parsed: Stored<'a, T, SPACE> = "2".parse().unwrap();
    let mapped = initial.map(|value| value.wrapping_add(1));
    return (mapped + parsed).raw();
}
"#,
};

pub(crate) static STABLE_CHECKED: LazyLock<Checked> = LazyLock::new(|| {
    return check(&STABLE, &cases(&[&GENERIC]));
});
