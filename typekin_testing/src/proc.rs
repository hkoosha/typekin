use crate::FixtureDir;
use std::path::Path;
use std::process::{
    Command,
    ExitStatus,
};
use std::{
    env,
    fs,
};

const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");

pub const DEP_NONE: &'static str = "";

pub const DEP_SERDE: &'static str = r#"
serde = { version = "1", default-features = false, features = ["alloc"] }
"#;

pub fn build(
    optimization_level: usize,
    bin: &str,
    target_dir: &Path,
) {
    let mut cmd = Command::new("cargo");
    cmd.arg("+nightly")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(Path::new(MANIFEST).join("Cargo.toml"))
        .arg("--bin")
        .arg(bin)
        .arg("--target-dir")
        .arg(target_dir)
        .env("CARGO_PROFILE_RELEASE_INCREMENTAL", "false")
        .env("CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS", "true")
        .env("CARGO_PROFILE_RELEASE_DEBUG", "false")
        .env("CARGO_PROFILE_RELEASE_PANIC", "abort")
        .env("CARGO_PROFILE_RELEASE_LTO", "false")
        .env("CARGO_PROFILE_RELEASE_CODEGEN_UNITS", "1")
        .env("CARGO_PROFILE_RELEASE_INCREMENTAL", "false")
        .env("CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS", "false")
        .env(
            "CARGO_PROFILE_RELEASE_OPT_LEVEL",
            optimization_level.to_string(),
        );
    let cmd = cmd;

    exe("build_bin", cmd);
}

pub fn exe(
    context: &str,
    mut command: Command,
) -> String {
    let rendered = render_command(&command);
    let output = command.output().unwrap_or_else(|error| {
        panic!(
            "failed, context=`{context}`, command=`{rendered}`, error={error}"
        )
    });

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        panic!(
            "failed context=`{context}`, command=`{rendered}`, status={}\nstdout:\n{}\nstderr:\n{}",
            output.status, stdout, stderr
        );
    }

    return format!("{stdout}{stderr}");
}

#[must_use]
pub fn render_command(command: &Command) -> String {
    let mut result = command.get_program().to_string_lossy().into_owned();
    for arg in command.get_args() {
        result.push(' ');

        let value = arg.to_string_lossy();
        result.push_str(&if value.contains(char::is_whitespace) {
            format!("{value:?}")
        }
        else {
            value.into_owned()
        });
    }
    return result;
}

pub fn compile(
    name: &str,
    dependencies: &str,
    source: &str,
    nightly: bool,
) -> Output {
    let fixture = FixtureDir::mkdir_suffixed(name);

    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .expect("canonicalize root");
    let root = root.to_str().expect("root must be valid UTF-8");

    let cargo_toml = format!(
        r#"
[package]
name = "{}"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
typekin = {{ path = "{}" }}
{}
"#,
        name, root, dependencies,
    );

    let manifest = fixture.path.join("Cargo.toml");
    fs::write(&manifest, cargo_toml).expect("cannot write manifest");

    let lib_rs = fixture.path.join("src/lib.rs");
    fs::write(&lib_rs, source).expect("cannot write source");

    let output = Command::new("cargo")
        .args([
            match nightly {
                true => "+nightly",
                false => "+stable",
            },
            "check",
            "--offline",
            "--quiet",
        ])
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--target-dir")
        .arg(fixture.path.join("target"))
        .output()
        .unwrap();

    return Output {
        status: output.status,
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    };
}

#[derive(Debug)]
pub struct Output {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}
