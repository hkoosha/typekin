use crate::*;
use std::collections::HashMap;
use std::fmt::Write;
use std::path::Path;
use std::process::Command;

#[must_use]
pub fn text_diff(
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

#[must_use]
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

#[must_use]
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

#[must_use]
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

#[must_use]
pub(super) fn symbol_addr<'a>(
    symbols: &'a HashMap<String, String>,
    symbol: &str,
    binary: &Path,
) -> &'a str {
    return symbols.get(symbol).unwrap_or_else(|| {
        panic!("missing symbol `{}` in {}", symbol, binary.display())
    });
}

#[must_use]
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

#[must_use]
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

#[must_use]
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

#[must_use]
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
