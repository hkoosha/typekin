#![allow(unused, dead_code)]

use std::path::{
    Path,
    PathBuf,
};
use std::process::Command;
use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

#[allow(unused_imports)]
pub(crate) mod tests;

pub(crate) struct Operation {
    pub name: &'static str,
    pub raw: &'static str,
    pub wrapped: &'static str,
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

pub(crate) const U32_OPERATIONS: &[Operation] = &[
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

pub(crate) fn make_dir(prefix: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    return std::env::temp_dir().join(format!(
        "typekin_{}__{}__{}",
        prefix,
        std::process::id(),
        now
    ));
}

pub(crate) fn build(
    level: &str,
    bin: &str,
    target_dir: &Path,
    manifest: &str,
) {
    let mut cmd = Command::new("cargo");
    cmd.arg("+nightly")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(Path::new(manifest).join("Cargo.toml"))
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
        .env("CARGO_PROFILE_RELEASE_OPT_LEVEL", level);
    let cmd = cmd;

    exe("build_bin", cmd);
}

pub(crate) fn exe(
    context: &str,
    mut command: Command,
) -> String {
    let rendered = my_proc::render_command(&command);
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
pub(crate) fn assert_match(
    binary: &Path,
    operations: &[Operation],
) -> (Vec<String>, Vec<String>) {
    let symbols = my_asm::symbols(binary);
    let mut expected = Vec::new();
    let mut unexpected = Vec::new();

    for it in operations {
        let raw_addr = my_asm::symbol_addr(&symbols, it.raw, binary);
        let wrapped_addr = my_asm::symbol_addr(&symbols, it.wrapped, binary);
        if raw_addr == wrapped_addr {
            continue;
        }

        let raw = my_asm::assembly_text(binary, it.raw);
        let wrapped = my_asm::assembly_text(binary, it.wrapped);

        if raw != wrapped {
            unexpected.push(my_asm::text_diff(
                it,
                raw_addr,
                wrapped_addr,
                &raw,
                &wrapped,
            ));
        }
        else {
            expected.push(raw);
        }
    }

    return (expected, unexpected);
}

#[must_use]
pub(crate) fn assert_normalized_diff(
    binary: &Path,
    operations: &[Operation],
) -> (Vec<String>, Vec<String>) {
    let symbols = my_asm::symbols(binary);
    let mut diffs: Vec<String> = Vec::new();
    let mut unexpected = Vec::new();

    for it in operations {
        let raw = my_asm::normalized_assembly(binary, it.raw);
        let raw_addr = my_asm::symbol_addr(&symbols, it.raw, binary);

        let wrapped_addr = my_asm::symbol_addr(&symbols, it.wrapped, binary);
        let wrapped = my_asm::normalized_assembly(binary, it.wrapped);

        if raw == wrapped {
            unexpected.push(my_asm::assembly_list_match(
                it,
                raw_addr,
                wrapped_addr,
                &raw,
            ));
        }
        else {
            diffs.push(my_asm::text_diff(
                it,
                raw_addr,
                wrapped_addr,
                &raw.join("\n"),
                &wrapped.join("\n"),
            ));
        }
    }

    return (diffs, unexpected);
}

#[must_use]
pub(crate) fn assert_diff(
    binary: &Path,
    operations: &[Operation],
) -> (Vec<String>, Vec<String>) {
    let symbols = my_asm::symbols(binary);
    let mut diffs = Vec::new();
    let mut unexpected = Vec::new();

    for it in operations {
        let raw_addr = my_asm::symbol_addr(&symbols, it.raw, binary);
        let raw = my_asm::assembly_text(binary, it.raw);

        let wrapped_addr = my_asm::symbol_addr(&symbols, it.wrapped, binary);
        let wrapped = my_asm::assembly_text(binary, it.wrapped);

        if raw == wrapped {
            unexpected.push(my_asm::assembly_match(
                it,
                raw_addr,
                wrapped_addr,
                &raw,
            ));
        }
        else {
            diffs.push(my_asm::text_diff(
                it,
                raw_addr,
                wrapped_addr,
                &raw,
                &wrapped,
            ));
        }
    }

    return (diffs, unexpected);
}

mod my_proc {
    use std::process::Command;

    pub(super) fn render_command(command: &Command) -> String {
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
}

mod my_asm {
    use crate::{
        Operation,
        exe,
    };
    use std::collections::HashMap;
    use std::fmt::Write;
    use std::path::Path;
    use std::process::Command;

    pub(crate) fn text_diff(
        operation: &Operation,
        raw_addr: &str,
        wrapped_addr: &str,
        raw: &str,
        wrapped: &str,
    ) -> String {
        let mut result = String::new();
        writeln!(
            result,
            "[OP_DIFF::{}] {} @ 0x{} != {} @ 0x{}",
            operation.name,
            operation.raw,
            raw_addr,
            operation.wrapped,
            wrapped_addr
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

    pub(super) fn assembly_list_match(
        operation: &Operation,
        raw_addr: &str,
        wrapped_addr: &str,
        assembly: &[String],
    ) -> String {
        let mut result = String::new();
        writeln!(
            result,
            "operation `{}` matched: {} at 0x{}, {} at 0x{}",
            operation.name,
            operation.raw,
            raw_addr,
            operation.wrapped,
            wrapped_addr
        )
        .unwrap();

        for instruction in assembly {
            writeln!(result, "  {instruction}").unwrap();
        }

        return result;
    }

    pub(super) fn assembly_match(
        operation: &Operation,
        raw_addr: &str,
        wrapped_addr: &str,
        assembly: &str,
    ) -> String {
        let mut result = String::new();
        writeln!(
            result,
            "operation `{}` matched: {} at 0x{}, {} at 0x{}",
            operation.name,
            operation.raw,
            raw_addr,
            operation.wrapped,
            wrapped_addr
        )
        .unwrap();
        writeln!(result, "{assembly}").unwrap();
        return result;
    }

    fn normalized_objdump_text(output: &str) -> String {
        let mut result = String::new();

        for line in output.lines() {
            let Some((address, instruction)) =
                line.trim_start().split_once(':')
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

    pub(super) fn symbol_addr<'a>(
        symbols: &'a HashMap<String, String>,
        symbol: &str,
        binary: &Path,
    ) -> &'a str {
        return symbols.get(symbol).unwrap_or_else(|| {
            panic!("missing symbol `{}` in {}", symbol, binary.display())
        });
    }

    pub(super) fn symbols(binary: &Path) -> HashMap<String, String> {
        let mut command = Command::new("nm");
        command.arg("-C").arg(binary);
        let output = exe("read_symbols", command);
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

    pub(super) fn assembly_text(
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

        let output = exe("disassemble_symbol", command);
        return normalized_objdump_text(&output);
    }

    pub(super) fn normalize_instruction(instruction: &str) -> String {
        let mut stripped = String::with_capacity(instruction.len());
        let mut in_annotation = false;
        for ch in instruction.chars() {
            match ch {
                '<' => in_annotation = true,
                '>' if in_annotation => in_annotation = false,
                _ if !in_annotation => stripped.push(ch),
                _ => {}
            }
        }

        let instruction = stripped
            .split('#')
            .next()
            .unwrap_or_default()
            .trim()
            .to_owned();

        if instruction.is_empty() {
            return instruction;
        }

        let Some(opcode_end) = instruction.find(char::is_whitespace)
        else {
            return instruction;
        };

        let opcode = &instruction[..opcode_end];
        if opcode == "call" || opcode.starts_with('j') {
            return format!("{opcode} <target>");
        }

        return instruction;
    }

    pub(super) fn normalized_assembly(
        binary: &Path,
        symbol: &str,
    ) -> Vec<String> {
        let mut command = Command::new("objdump");
        command
            .arg("-d")
            .arg("--demangle")
            .arg("--no-show-raw-insn")
            .arg(format!("--disassemble={symbol}"))
            .arg(binary);

        let output = exe("disassemble_symbol", command);
        let mut result = Vec::new();

        for line in output.lines() {
            let Some((address, instruction)) = line.split_once(':')
            else {
                continue;
            };
            if !address.trim().chars().all(|it| it.is_ascii_hexdigit()) {
                continue;
            }
            let instruction = normalize_instruction(instruction.trim());
            if !instruction.is_empty() {
                result.push(instruction);
            }
        }

        return result;
    }
}
