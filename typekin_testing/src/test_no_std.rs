use crate::*;

const DEP_NONE: &'static str = "";
const DEP_SERDE: &'static str = r#"
serde = { version = "1", default-features = false, features = ["alloc"] }
"#;

#[cfg(test)]
#[cfg(feature = "test_slow")]
mod tests {
    use super::*;

    #[test]
    fn integral_compiles_for_a_no_std_consumer() {
        const SRC: &str = r#"
#![no_std]

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
"#;

        assert_crate_compiles("integral", DEP_NONE, SRC)
    }

    #[test]
    fn bitflag_compiles_for_a_no_std_consumer() {
        const SRC: &str = r#"
#![no_std]
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_iter)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]

#[typekin::bitflag(repr = u8)]
pub enum Permission {
    Read = 0b001,
    Write = 0b010,
}

pub fn build_permission(value: u8) -> PermissionValue {
    return Permission::from_bits_truncate(value);
}
"#;

        assert_crate_compiles("bitflag", DEP_NONE, SRC)
    }

    #[test]
    fn text_compiles_for_a_no_std_alloc_consumer() {
        const SRC: &str = r#"
#![no_std]

extern crate alloc;

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
"#;

        assert_crate_compiles("text", DEP_NONE, SRC)
    }

    #[test]
    fn serde_compiles_for_a_no_std_alloc_consumer() {
        const SRC: &str = r#"
#![no_std]

extern crate alloc;

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
"#;

        assert_crate_compiles("serde", DEP_SERDE, SRC);
    }
}
