use std::error::Error;
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

const DISABLED: bool = true;

struct FixtureDir {
    keep: bool,
    path: PathBuf,
}

impl FixtureDir {
    fn new(suffix: &str) -> Result<Self, Box<dyn Error>> {
        let path = env::temp_dir().join(format!(
            "typekin-compile-fail-{}-{}-{}",
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos(),
            suffix,
            std::process::id()
        ));

        fs::create_dir_all(path.join("src"))?;

        return Ok(Self { keep: false, path });
    }
}

impl Drop for FixtureDir {
    fn drop(&mut self) {
        if !self.keep {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

//noinspection RsConstantConditionIf
fn assert_rejected(
    name: &str,
    source: &str,
    expected_error: &str,
) {
    if DISABLED {
        return;
    }

    let fixture = FixtureDir::new(name).unwrap();

    let path = &fixture.path.join("Cargo.toml");
    let content = &format!(
        "\
                [package]\n\
                name = \"{}\"\n\
                version = \"0.0.0\"\n\
                edition = \"2024\"\n\
                \n\
                [dependencies]\n\
                typekin = {{ path = {} }}\n\
                ",
        name,
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .expect("canonicalize typekin root")
            .to_str()
            .expect("typekin root must be valid UTF-8"),
    );
    fs::write(path, content).expect("write compile-fail fixture");

    let path = &fixture.path.join("src/lib.rs");
    fs::write(path, source).expect("write compile-fail fixture");

    let output = Command::new("cargo")
        .args([
            "+nightly",
            "check",
            "--offline",
            "--quiet",
            "--manifest-path",
        ])
        .arg(&fixture.path.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(fixture.path.join("target"))
        .output()
        .expect("run nightly compile-fail fixture");

    let diagnostics = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "{name} unexpectedly compiled:\n{diagnostics}"
    );
    assert!(
        diagnostics.contains(expected_error),
        "{name} rejected for an unexpected reason; expected {expected_error:?}:\n{diagnostics}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validated_integral_has_no_make() {
        assert_rejected(
            "validated-integral-make",
            r#"
                const fn is_even(value: u8) -> bool { value % 2 == 0 }

                #[typekin::integral(konst = none, valid = is_even)]
                #[repr(transparent)]
                struct Checked(u8);

                fn requires_make() {
                    let _ = Checked::make(2);
                }
            "#,
            "no associated function or constant named `make`",
        );
    }

    #[test]
    fn make_does_not_grant_math() {
        assert_rejected(
            "make-does-not-grant-math",
            r#"
                struct MakeSource(u8);
                fn into_u8(source: MakeSource) -> u8 { source.0 }

                #[typekin::integral(
                    konst = none,
                    friends = into_u8(MakeSource) -> Make,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_math() {
                    let _ = Number::make(1) + MakeSource(2);
                }
            "#,
            "Math",
        );
    }

    #[test]
    fn bit_does_not_grant_math() {
        assert_rejected(
            "bit-does-not-grant-math",
            r#"
                struct BitSource(u8);
                fn into_u8(source: BitSource) -> u8 { source.0 }

                #[typekin::integral(
                    konst = none,
                    friends = into_u8(BitSource) -> Bit,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_math() {
                    let _ = Number::make(1) + BitSource(2);
                }
            "#,
            "Math",
        );
    }

    #[test]
    fn trust_does_not_grant_make() {
        assert_rejected(
            "trust-does-not-grant-make",
            r#"
                struct TrustSource(u8);
                fn into_u8(source: TrustSource) -> u8 { source.0 }

                #[typekin::integral(
                    konst = none,
                    friends = into_u8(TrustSource) -> unsafe,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_make() {
                    let _ = Number::of(TrustSource(2));
                }
            "#,
            "Make",
        );
    }

    #[test]
    fn without_removes_raw() {
        assert_rejected(
            "without-removes-raw",
            r#"
                #[typekin::integral(
                    konst = none,
                    without = fn_conv_raw,
                    get_raw = Self::value,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                impl Number {
                    const fn value(self) -> u8 { self.0 }
                }

                fn requires_raw() {
                    let _ = Number::make(2).raw();
                }
            "#,
            "no method named `raw`",
        );
    }

    #[test]
    fn without_removes_self_math() {
        assert_rejected(
            "without-removes-self-math",
            r#"
                #[typekin::integral(
                    konst = none,
                    without = self_math,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_self_math() {
                    let _ = Number::make(2) + Number::make(3);
                }
            "#,
            "the trait `Math` is not implemented for `Number`",
        );
    }

    #[test]
    fn without_removes_self_bit() {
        assert_rejected(
            "without-removes-self-bit",
            r#"
                #[typekin::integral(
                    konst = none,
                    without = self_bit,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_self_bit() {
                    let _ = Number::make(2) | Number::make(3);
                }
            "#,
            "the trait `Bit` is not implemented for `Number`",
        );
    }

    #[test]
    fn self_operators_do_not_use_friendship() {
        assert_rejected(
            "self-operators-do-not-use-friendship",
            r#"
                #[typekin::integral(
                    konst = none,
                    friends = Number(Number) -> Math,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);
            "#,
            "self math operations use `with_self_math`, not `friends`",
        );
    }

    #[test]
    fn without_removes_from_str() {
        assert_rejected(
            "without-removes-from-str",
            r#"
                #[typekin::integral(
                    konst = none,
                    without = from_str,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_from_str<T>()
                where
                    T: core::str::FromStr<Err = ()>,
                {}

                fn requires_number_from_str() {
                    requires_from_str::<Number>();
                }
            "#,
            "the trait bound `Number: FromStr` is not satisfied",
        );
    }

    #[test]
    fn without_removes_common_integral_methods() {
        assert_rejected(
            "without-removes-common-integral-methods",
            r#"
                #[typekin::integral(
                    konst = none,
                    without = core_int,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_count_ones() {
                    let _ = Number::make(2).count_ones();
                }
            "#,
            "no method named `count_ones`",
        );
    }

    #[test]
    fn core_integral_methods_exclude_unsafe_operations() {
        assert_rejected(
            "core-integral-methods-exclude-unsafe-operations",
            r#"
                #[typekin::integral(konst = none)]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_unchecked_add() {
                    let _ = Number::make(2).unchecked_add(Number::make(3));
                }
            "#,
            "no method named `unchecked_add`",
        );
    }

    #[test]
    fn validated_text_has_no_infallible_from() {
        assert_rejected(
            "validated-text-has-no-infallible-from",
            r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = none, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_infallible_from() {
                    let _: Checked = String::from("text").into();
                }
            "#,
            "the trait bound `Checked: From<String>` is not satisfied",
        );
    }

    #[test]
    fn validated_text_has_no_deref_mut() {
        assert_rejected(
            "validated-text-has-no-deref-mut",
            r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = none, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_deref_mut(value: &mut Checked) {
                    let _: &mut str = value;
                }
            "#,
            "DerefMut",
        );
    }

    #[test]
    fn validated_text_has_no_infallible_str_from() {
        assert_rejected(
            "validated-text-has-no-infallible-str-from",
            r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = none, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_infallible_from() {
                    let _: Checked = "text".into();
                }
            "#,
            "the trait bound `Checked: From<&str>` is not satisfied",
        );
    }

    #[test]
    fn validated_text_has_no_as_mut() {
        assert_rejected(
            "validated-text-has-no-as-mut",
            r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = none, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_as_mut(value: &mut Checked) {
                    let _: &mut str = AsMut::<str>::as_mut(value);
                }
            "#,
            "AsMut<str>",
        );
    }

    #[test]
    fn validated_text_has_no_mutable_string() {
        assert_rejected(
            "validated-text-has-no-mutable-string",
            r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = none, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_mutable_string(_: &mut String) {}

                fn mutable_string_escape(value: &mut Checked) {
                    requires_mutable_string(value);
                }
            "#,
            "expected `&mut String`, found `&mut Checked`",
        );
    }
}
