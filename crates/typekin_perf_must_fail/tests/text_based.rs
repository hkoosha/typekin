use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const FIXTURE_BIN: &str = "typekin-perf-must-fail-fixture";
const BUILD_PROFILE: &str = "debug";

const OPERATIONS: &[Operation] = &[
    Operation::new("add", "raw_add_u32", "wrapped_add_u32"),
    Operation::new("sub", "raw_sub_u32", "wrapped_sub_u32"),
    Operation::new("mul", "raw_mul_u32", "wrapped_mul_u32"),
    Operation::new("div", "raw_div_u32", "wrapped_div_u32"),
    Operation::new("rem", "raw_rem_u32", "wrapped_rem_u32"),
    Operation::new("bitand", "raw_bitand_u32", "wrapped_bitand_u32"),
    Operation::new("bitor", "raw_bitor_u32", "wrapped_bitor_u32"),
    Operation::new("bitxor", "raw_bitxor_u32", "wrapped_bitxor_u32"),
    Operation::new("not", "raw_not_u32", "wrapped_not_u32"),
    Operation::new("shl", "raw_shl_u32", "wrapped_shl_u32"),
    Operation::new("shr", "raw_shr_u32", "wrapped_shr_u32"),
];

struct Operation {
    name: &'static str,
    raw: &'static str,
    wrapped: &'static str,
}

impl Operation {
    const fn new(
        name: &'static str,
        raw: &'static str,
        wrapped: &'static str,
    ) -> Self {
        return Self { name, raw, wrapped };
    }
}

#[test]
fn text_based_unoptimized_const_integral_operations_do_not_match_raw_assembly()
{
    let target_dir = fresh_target_dir();
    let binary = target_dir.join(BUILD_PROFILE).join(FIXTURE_BIN);

    let result = std::panic::catch_unwind(|| {
        build_fixture(&target_dir);
        run_fixture(&binary);
        assert_operations_differ(&binary);
    });

    let _ = std::fs::remove_dir_all(&target_dir);

    if let Err(error) = result {
        std::panic::resume_unwind(error);
    }
}

fn fresh_target_dir() -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before UNIX_EPOCH")
        .as_nanos();

    return env::temp_dir().join(format!(
        "typekin_perf_must_fail_text_asm_{}_{}",
        std::process::id(),
        now
    ));
}

fn build_fixture(target_dir: &Path) {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");

    let mut command = Command::new("cargo");
    command
        .arg("+nightly")
        .arg("build")
        .arg("--manifest-path")
        .arg(manifest)
        .arg("--bin")
        .arg(FIXTURE_BIN)
        .arg("--target-dir")
        .arg(target_dir)
        .env("CARGO_PROFILE_DEV_OPT_LEVEL", "0")
        .env("CARGO_PROFILE_DEV_LTO", "false")
        .env("CARGO_PROFILE_DEV_CODEGEN_UNITS", "256")
        .env("CARGO_PROFILE_DEV_INCREMENTAL", "false")
        .env("CARGO_PROFILE_DEV_OVERFLOW_CHECKS", "true")
        .env("CARGO_PROFILE_DEV_DEBUG", "false")
        .env("CARGO_PROFILE_DEV_PANIC", "abort");

    run(command, "build unoptimized fixture");
}

fn run_fixture(binary: &Path) {
    let command = Command::new(binary);
    run(command, "run unoptimized fixture");
}

fn assert_operations_differ(binary: &Path) {
    let symbols = symbols(binary);
    let mut diffs = Vec::new();
    let mut unexpected_matches = Vec::new();

    for operation in OPERATIONS {
        let raw_address = symbol_address(&symbols, operation.raw, binary);
        let wrapped_address =
            symbol_address(&symbols, operation.wrapped, binary);
        let raw = assembly_text(binary, operation.raw);
        let wrapped = assembly_text(binary, operation.wrapped);

        if raw == wrapped {
            unexpected_matches.push(assembly_match(
                operation,
                raw_address,
                wrapped_address,
                &raw,
            ));
        }
        else {
            diffs.push(text_diff(
                operation,
                raw_address,
                wrapped_address,
                &raw,
                &wrapped,
            ));
        }
    }

    if !diffs.is_empty() {
        println!(
            "expected unoptimized const integral assembly diffs:\n\n{}",
            diffs.join("\n")
        );
    }

    if !unexpected_matches.is_empty() {
        panic!(
            "unoptimized const integral wrapper operations unexpectedly matched raw assembly:\n\n{}",
            unexpected_matches.join("\n")
        );
    }
}

fn symbol_address<'a>(
    symbols: &'a HashMap<String, String>,
    symbol: &str,
    binary: &Path,
) -> &'a str {
    return symbols.get(symbol).unwrap_or_else(|| {
        panic!("missing symbol `{}` in {}", symbol, binary.display())
    });
}

fn symbols(binary: &Path) -> HashMap<String, String> {
    let mut command = Command::new("nm");
    command.arg("-C").arg(binary);
    let output = run(command, "read fixture symbols");
    let mut result = HashMap::new();

    for line in output.lines() {
        let mut fields = line.split_whitespace();
        let Some(address) = fields.next()
        else {
            continue;
        };
        let Some(kind) = fields.next()
        else {
            continue;
        };
        if !matches!(kind, "T" | "t") {
            continue;
        }
        let name = fields.collect::<Vec<_>>().join(" ");
        if name.starts_with("raw_") || name.starts_with("wrapped_") {
            result.insert(name, address.to_owned());
        }
    }

    return result;
}

fn assembly_text(
    binary: &Path,
    symbol: &str,
) -> String {
    let mut command = Command::new("objdump");
    command
        .arg("-d")
        .arg("--demangle")
        .arg("--no-show-raw-insn")
        .arg(format!("--disassemble={symbol}"))
        .arg(binary);

    let output = run(command, "disassemble fixture symbol");
    return normalized_objdump_text(&output);
}

fn normalized_objdump_text(output: &str) -> String {
    let mut result = String::new();

    for line in output.lines() {
        let Some((address, instruction)) = line.trim_start().split_once(':')
        else {
            continue;
        };
        if !address.trim().chars().all(|it| it.is_ascii_hexdigit()) {
            continue;
        }

        let instruction =
            instruction.split_whitespace().collect::<Vec<_>>().join(" ");
        if !instruction.is_empty() {
            result.push_str(&instruction);
            result.push('\n');
        }
    }

    return result;
}

fn text_diff(
    operation: &Operation,
    raw_address: &str,
    wrapped_address: &str,
    raw: &str,
    wrapped: &str,
) -> String {
    let mut result = String::new();
    writeln!(
        result,
        "operation `{}` differs: {} at 0x{}, {} at 0x{}",
        operation.name,
        operation.raw,
        raw_address,
        operation.wrapped,
        wrapped_address
    )
    .unwrap();
    writeln!(result, "--- {}", operation.raw).unwrap();
    writeln!(result, "+++ {}", operation.wrapped).unwrap();

    let raw = raw.lines().collect::<Vec<_>>();
    let wrapped = wrapped.lines().collect::<Vec<_>>();
    let count = raw.len().max(wrapped.len());

    for index in 0..count {
        match (raw.get(index), wrapped.get(index)) {
            (Some(lhs), Some(rhs)) if lhs == rhs => {
                writeln!(result, "  {lhs}").unwrap();
            }
            (Some(lhs), Some(rhs)) => {
                writeln!(result, "- {lhs}").unwrap();
                writeln!(result, "+ {rhs}").unwrap();
            }
            (Some(lhs), None) => writeln!(result, "- {lhs}").unwrap(),
            (None, Some(rhs)) => writeln!(result, "+ {rhs}").unwrap(),
            (None, None) => unreachable!(),
        }
    }

    return result;
}

fn assembly_match(
    operation: &Operation,
    raw_address: &str,
    wrapped_address: &str,
    assembly: &str,
) -> String {
    let mut result = String::new();
    writeln!(
        result,
        "operation `{}` matched: {} at 0x{}, {} at 0x{}",
        operation.name,
        operation.raw,
        raw_address,
        operation.wrapped,
        wrapped_address
    )
    .unwrap();
    writeln!(result, "{assembly}").unwrap();
    return result;
}

fn run(
    mut command: Command,
    context: &str,
) -> String {
    let rendered = render_command(&command);
    let output = command.output().unwrap_or_else(|error| {
        panic!("failed to {context} with `{rendered}`: {error}")
    });

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        panic!(
            "failed to {context} with `{rendered}`\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
            output.status, stdout, stderr
        );
    }

    return format!("{stdout}{stderr}");
}

fn render_command(command: &Command) -> String {
    let mut result = command.get_program().to_string_lossy().into_owned();
    for arg in command.get_args() {
        result.push(' ');
        result.push_str(&render_os(arg));
    }
    return result;
}

fn render_os(value: &OsStr) -> String {
    let value = value.to_string_lossy();
    if value.contains(char::is_whitespace) {
        return format!("{value:?}");
    }
    return value.into_owned();
}
