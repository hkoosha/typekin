//! Consumer crates checked with `cargo check`, many cases per crate.
//!
//! Each case becomes `mod <case>` in one `src/lib.rs`, so one build checks
//! every case of a fixture. Rejections are attributed by line: a case is
//! rejected only by an error inside its own module whose message contains
//! the expected text, and an error outside every case fails them all.
//! Compile-pass and rejection cases never share a crate: rustc may skip
//! later checks (such as constant evaluation) once it has reported errors.
//! [`compile`] runs `cargo build` instead, for errors only monomorphization
//! reports, such as constants evaluated in generic code.
//!
//! Each fixture keeps its crate and target directory under the testing
//! crate's `target/`, so dependencies build once and later runs only
//! re-check what changed.

use std::ops::RangeInclusive;
use std::path::{
    Path,
    PathBuf,
};
use std::process::Command;

use crate::render_command;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Toolchain {
    Stable,
    Nightly,
}

/// A consumer crate: everything shared by its cases.
pub struct Fixture {
    /// Package name and directory; unique per fixture.
    pub name: &'static str,
    pub toolchain: Toolchain,
    /// `--target`; the host when `None`.
    pub target: Option<&'static str>,
    /// Crate-level lines before the cases: `#![...]` and `extern crate`.
    pub header: &'static str,
    /// `[dependencies]` lines besides `typekin`.
    pub dependencies: &'static str,
}

/// A module of a fixture crate that must compile.
pub struct Case {
    /// The module name.
    pub name: &'static str,
    pub source: &'static str,
}

/// A module of a fixture crate that rustc must reject with `expected`.
pub struct Rejection {
    /// The module name.
    pub name: &'static str,
    /// Text of the error's first line.
    pub expected: &'static str,
    pub source: &'static str,
}

/// An error rustc reported at a line of the fixture's `src/lib.rs`.
#[derive(Debug)]
struct Error {
    line: usize,
    message: String,
}

/// The outcome of checking a fixture crate once.
pub struct Checked {
    fixture: &'static str,
    success: bool,
    command: String,
    stderr: String,
    /// `(case, lines of its module)`.
    cases: Vec<(&'static str, RangeInclusive<usize>)>,
    /// Errors located in `src/lib.rs`.
    errors: Vec<Error>,
}

impl Checked {
    fn lines_of(
        &self,
        case: &str,
    ) -> &RangeInclusive<usize> {
        return self
            .cases
            .iter()
            .find(|(name, _)| *name == case)
            .map(|(_, lines)| lines)
            .unwrap_or_else(|| {
                panic!(
                    "case `{case}` is not part of fixture `{}`",
                    self.fixture
                )
            });
    }

    /// The case whose module holds `line`.
    fn case_at(
        &self,
        line: usize,
    ) -> Option<&'static str> {
        return self
            .cases
            .iter()
            .find(|(_, lines)| lines.contains(&line))
            .map(|(name, _)| *name);
    }

    fn report(&self) -> String {
        let errors = self
            .errors
            .iter()
            .map(|it| {
                let case = self.case_at(it.line).unwrap_or("<outside cases>");
                return format!("  {case} (line {}): {}", it.line, it.message);
            })
            .collect::<Vec<_>>()
            .join("\n");
        return format!(
            "fixture `{}`, command `{}`\nerrors:\n{errors}\nstderr:\n{}",
            self.fixture, self.command, self.stderr
        );
    }

    fn assert_no_error_outside_cases(&self) {
        let outside =
            self.errors.iter().any(|it| self.case_at(it.line).is_none());
        assert!(!outside, "error outside every case; {}", self.report());
    }
}

/// The directory holding fixture crates and their target directories.
fn root() -> PathBuf {
    return Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("typekin-fixtures");
}

/// `cargo` for a fixture crate, without the caller's rustc flags: the root
/// `justfile` exports `RUSTFLAGS = -Zmacro-backtrace`, which moves a const
/// panic's diagnostic from the case's line into `core`, and any other flag
/// could change the code under comparison.
pub fn fixture_cargo() -> Command {
    let mut command = Command::new("cargo");
    command
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_BUILD_RUSTFLAGS");
    return command;
}

/// Writes a crate depending on `typekin` and returns its directory.
pub fn write_crate(
    name: &str,
    dependencies: &str,
    source: &str,
) -> PathBuf {
    let typekin = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .expect("canonicalize the typekin root");
    let directory = root().join(name);
    std::fs::create_dir_all(directory.join("src"))
        .expect("cannot create the fixture directory");
    let manifest = format!(
        r#"[package]
name = "{name}"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
typekin = {{ path = "{}" }}
{dependencies}
"#,
        typekin.display(),
    );
    write_if_changed(&directory.join("Cargo.toml"), &manifest);
    write_if_changed(&directory.join("src").join("lib.rs"), source);
    return directory;
}

/// Leaves unchanged files untouched so cargo can reuse its results.
fn write_if_changed(
    path: &Path,
    content: &str,
) {
    if std::fs::read_to_string(path).is_ok_and(|it| it == content) {
        return;
    }
    std::fs::write(path, content).unwrap_or_else(|error| {
        panic!("cannot write {}: {error}", path.display())
    });
}

/// Checks one crate holding every case of `fixture`.
pub fn check(
    fixture: &Fixture,
    cases: &[(&'static str, &'static str)],
) -> Checked {
    return run(fixture, cases, "check");
}

/// Builds one crate holding every case of `fixture`: unlike [`check`], this
/// instantiates generic code and evaluates the constants it uses.
pub fn compile(
    fixture: &Fixture,
    cases: &[(&'static str, &'static str)],
) -> Checked {
    return run(fixture, cases, "build");
}

/// Runs `cargo <subcommand>` on one crate holding every case of `fixture`.
fn run(
    fixture: &Fixture,
    cases: &[(&'static str, &'static str)],
    subcommand: &str,
) -> Checked {
    let mut source = String::from(fixture.header);
    if !source.ends_with('\n') {
        source.push('\n');
    }
    let mut ranges = Vec::new();
    for (name, body) in cases {
        assert!(
            ranges.iter().all(|(it, _)| it != name),
            "duplicate case `{name}` in fixture `{}`",
            fixture.name
        );
        let start = source.lines().count() + 1;
        source.push_str(&format!("mod {name} {{\n{body}\n}}\n"));
        let end = source.lines().count();
        ranges.push((*name, start..=end));
    }

    let directory = write_crate(fixture.name, fixture.dependencies, &source);
    let mut command = fixture_cargo();
    command
        .arg(match fixture.toolchain {
            Toolchain::Stable => "+stable",
            Toolchain::Nightly => "+nightly",
        })
        .args([subcommand, "--offline", "--quiet", "--message-format=short"])
        .arg("--manifest-path")
        .arg(directory.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(directory.join("target"));
    if let Some(target) = fixture.target {
        command.arg("--target").arg(target);
    }
    let rendered = render_command(&command);
    let output = command
        .output()
        .unwrap_or_else(|error| panic!("cannot run `{rendered}`: {error}"));
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    return Checked {
        fixture: fixture.name,
        success: output.status.success(),
        command: rendered,
        errors: errors_of(&stderr),
        stderr,
        cases: ranges,
    };
}

/// `src/lib.rs:12:5: error[E0423]: message` lines of short diagnostics.
fn errors_of(stderr: &str) -> Vec<Error> {
    return stderr
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("src/lib.rs:")?;
            let (number, rest) = rest.split_once(':')?;
            let (_, rest) = rest.split_once(": ")?;
            let message = rest
                .strip_prefix("error")?
                .split_once(": ")
                .map(|(_, it)| it)?;
            return Some(Error {
                line: number.parse().ok()?,
                message: message.to_owned(),
            });
        })
        .collect();
}

/// The whole fixture compiled, and `case` is part of it.
pub fn assert_compiles(
    checked: &Checked,
    case: &Case,
) {
    let _ = checked.lines_of(case.name);
    assert!(checked.success, "fixture failed; {}", checked.report());
}

/// rustc rejected `case`: its module has errors, and each of them contains
/// `case.expected`, so a broken fixture cannot pass for the wrong reason.
pub fn assert_rejected(
    checked: &Checked,
    case: &Rejection,
) {
    let lines = checked.lines_of(case.name);
    assert!(!checked.success, "fixture compiled; {}", checked.report());
    checked.assert_no_error_outside_cases();
    let mut errors = checked
        .errors
        .iter()
        .filter(|it| lines.contains(&it.line))
        .peekable();
    assert!(
        errors.peek().is_some(),
        "`{}` was not rejected; {}",
        case.name,
        checked.report()
    );
    let unexpected = errors.find(|it| !it.message.contains(case.expected));
    assert!(
        unexpected.is_none(),
        "`{}` was rejected for another reason than `{}`: {:?}; {}",
        case.name,
        case.expected,
        unexpected,
        checked.report()
    );
}

/// The `(name, source)` pairs of compile-pass cases.
#[must_use]
pub fn cases(cases: &[&Case]) -> Vec<(&'static str, &'static str)> {
    return cases.iter().map(|it| (it.name, it.source)).collect();
}

/// The `(name, source)` pairs of rejection cases.
#[must_use]
pub fn rejections(cases: &[&Rejection]) -> Vec<(&'static str, &'static str)> {
    return cases.iter().map(|it| (it.name, it.source)).collect();
}
