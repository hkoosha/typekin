#[cfg(test)]
mod tests {
    use std::path::{
        Path,
        PathBuf,
    };
    use std::process::Command;
    use std::time::{
        SystemTime,
        UNIX_EPOCH,
    };
    use std::{
        env,
        fs,
    };

    struct FixtureDir {
        path: PathBuf,
    }

    impl FixtureDir {
        fn new(suffix: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock before the Unix epoch")
                .as_nanos();
            let path = env::temp_dir().join(format!(
                "typekin-no-std-{suffix}-{}-{unique}",
                std::process::id()
            ));

            fs::create_dir_all(path.join("src"))
                .expect("create no-std fixture directory");

            return Self { path };
        }
    }

    impl Drop for FixtureDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn assert_no_std_compiles(
        name: &str,
        source: &str,
    ) {
        let fixture = FixtureDir::new(name);
        let manifest = fixture.path.join("Cargo.toml");
        let source_path = fixture.path.join("src/lib.rs");
        let typekin = Path::new(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .expect("canonicalize typekin root");
        let typekin =
            typekin.to_str().expect("typekin root must be valid UTF-8");

        fs::write(
            &manifest,
            format!(
                "[package]\nname = \"typekin_no_std_fixture\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies]\ntypekin = {{ path = {typekin:?} }}\n"
            ),
        )
            .expect("write no-std fixture manifest");
        fs::write(&source_path, source).expect("write no-std fixture source");

        let output = Command::new("cargo")
            .args([
                "+nightly",
                "check",
                "--offline",
                "--quiet",
                "--manifest-path",
            ])
            .arg(&manifest)
            .arg("--target-dir")
            .arg(fixture.path.join("target"))
            .output()
            .expect("run nightly no-std fixture");

        assert!(
            output.status.success(),
            "{name} no-std fixture failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn integral_compiles_for_a_no_std_consumer() {
        assert_no_std_compiles(
            "integral",
            r#"#![no_std]

const fn is_even(value: u8) -> bool {
    return value % 2 == 0;
}

#[typekin::integral(konst = none, valid = is_even)]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Even(u8);

pub fn build_even(value: u8) -> Result<Even, u8> {
    return Even::try_make(value);
}

pub fn parse_even(value: &str) -> Result<Even, ()> {
    return value.parse();
}
"#,
        );
    }

    #[test]
    fn bitflag_compiles_for_a_no_std_consumer() {
        assert_no_std_compiles(
            "bitflag",
            r#"#![no_std]
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_iter)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]

#[typekin::bitflag(konst = none)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Permission {
    Read = 0b001,
    Write = 0b010,
}

pub fn build_permission(value: u8) -> PermissionValue {
    return Permission::from_bits_truncate(value);
}
"#,
        );
    }

    #[test]
    fn text_compiles_for_a_no_std_alloc_consumer() {
        assert_no_std_compiles(
            "text",
            r#"#![no_std]

extern crate alloc;

use alloc::string::String;

fn is_slug(value: &str) -> bool {
    return !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-');
}

#[typekin::text(konst = none, std = false, valid = is_slug)]
#[repr(transparent)]
pub struct Slug(String);

pub fn build_slug(value: &str) -> Result<Slug, ()> {
    return Slug::try_from_str(value);
}
"#,
        );
    }
}
