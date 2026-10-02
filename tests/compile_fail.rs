use std::{
    env,
    fs,
    path::{
        Path,
        PathBuf,
    },
    process::Command,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

struct FixtureDir {
    path: PathBuf,
}

impl FixtureDir {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before the Unix epoch")
            .as_nanos();
        let path = env::temp_dir().join(format!(
            "typekin-compile-fail-{}-{unique}",
            std::process::id()
        ));

        fs::create_dir_all(path.join("src"))
            .expect("create compile-fail fixture directory");

        return Self { path };
    }
}

impl Drop for FixtureDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn write(
    path: impl AsRef<Path>,
    content: &str,
) {
    fs::write(path, content).expect("write compile-fail fixture");
}

fn assert_rejected(
    name: &str,
    source: &str,
    expected_error: &str,
) {
    let fixture = FixtureDir::new();
    let manifest = fixture.path.join("Cargo.toml");
    let source_file = fixture.path.join("src/lib.rs");
    let typekin = Path::new(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .expect("canonicalize typekin root");
    let typekin = typekin.to_str().expect("typekin root must be valid UTF-8");

    write(
        &manifest,
        &format!(
            "[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies]\ntypekin = {{ path = {typekin:?} }}\n"
        ),
    );
    write(&source_file, source);

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

#[test]
#[ignore]
fn generated_api_boundaries_reject_invalid_consumers() {
    for (name, source, expected_error) in [
        (
            "validated-integral-make",
            r#"
                const fn is_even(value: u8) -> bool { value % 2 == 0 }

                #[typekin::integral(konst = false, valid = is_even)]
                #[repr(transparent)]
                struct Checked(u8);

                fn requires_make() {
                    let _ = Checked::make(2);
                }
            "#,
            "no associated function or constant named `make`",
        ),
        (
            "make-does-not-grant-math",
            r#"
                struct MakeSource(u8);
                fn into_u8(source: MakeSource) -> u8 { source.0 }

                #[typekin::integral(
                    konst = false,
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
        ),
        (
            "math-does-not-grant-make",
            r#"
                struct MathSource(u8);
                fn into_u8(source: MathSource) -> u8 { source.0 }

                #[typekin::integral(
                    konst = false,
                    friends = into_u8(MathSource) -> Math,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_make() {
                    let _ = Number::of(MathSource(2));
                }
            "#,
            "Make",
        ),
        (
            "bit-does-not-grant-math",
            r#"
                struct BitSource(u8);
                fn into_u8(source: BitSource) -> u8 { source.0 }

                #[typekin::integral(
                    konst = false,
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
        ),
        (
            "trust-does-not-grant-make",
            r#"
                struct TrustSource(u8);
                fn into_u8(source: TrustSource) -> u8 { source.0 }

                #[typekin::integral(
                    konst = false,
                    friends = into_u8(TrustSource) -> Trust,
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_make() {
                    let _ = Number::of(TrustSource(2));
                }
            "#,
            "Make",
        ),
        (
            "without-removes-raw",
            r#"
                #[typekin::integral(
                    konst = false,
                    without = [fn_conv_raw],
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
        ),
        (
            "without-removes-from-str",
            r#"
                #[typekin::integral(
                    konst = false,
                    without = [impl_from_str],
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
        ),
        (
            "without-removes-common-integral-methods",
            r#"
                #[typekin::integral(
                    konst = false,
                    without = [impl_core_int],
                )]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_count_ones() {
                    let _ = Number::make(2).count_ones();
                }
            "#,
            "no method named `count_ones`",
        ),
        (
            "core-integral-methods-exclude-unsafe-operations",
            r#"
                #[typekin::integral(konst = false)]
                #[repr(transparent)]
                #[derive(Copy, Clone)]
                struct Number(u8);

                fn requires_unchecked_add() {
                    let _ = Number::make(2).unchecked_add(Number::make(3));
                }
            "#,
            "no method named `unchecked_add`",
        ),
        (
            "validated-text-has-no-infallible-from",
            r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = false, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_infallible_from() {
                    let _: Checked = String::from("text").into();
                }
            "#,
            "the trait bound `Checked: From<String>` is not satisfied",
        ),
        (
            "validated-text-has-no-deref-mut",
            r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = false, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_deref_mut(value: &mut Checked) {
                    let _: &mut str = value;
                }
            "#,
            "DerefMut",
        ),
        (
            "validated-text-has-no-infallible-str-from",
            r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = false, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_infallible_from() {
                    let _: Checked = "text".into();
                }
            "#,
            "the trait bound `Checked: From<&str>` is not satisfied",
        ),
        (
            "validated-text-has-no-as-mut",
            r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = false, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_as_mut(value: &mut Checked) {
                    let _: &mut str = AsMut::<str>::as_mut(value);
                }
            "#,
            "AsMut<str>",
        ),
        (
            "validated-text-has-no-mutable-string",
            r#"
                extern crate alloc;
                use alloc::string::String;

                fn is_slug(value: &str) -> bool { !value.is_empty() }

                #[typekin::text(konst = false, valid = is_slug)]
                #[repr(transparent)]
                struct Checked(String);

                fn requires_mutable_string(_: &mut String) {}

                fn mutable_string_escape(value: &mut Checked) {
                    requires_mutable_string(value);
                }
            "#,
            "expected `&mut String`, found `&mut Checked`",
        ),
    ] {
        assert_rejected(name, source, expected_error);
    }
}
