//! The assembly fixture crate. Every `asm_case_*.rs` is one of its modules;
//! one crate is built per target and opt-level, once per test run, under
//! `target/typekin-fixtures/`, and each test compares its own case's probes
//! symbol by symbol.
//!
//! A probe is an `#[unsafe(no_mangle)]`, `#[inline(never)]` function whose
//! parameters keep its inputs opaque to the optimizer. Probe names are unique
//! across cases, and each case declares its own types and gives its probes
//! bodies no other case has: LLVM merges identical functions of one crate
//! into an alias or a jump.

use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::path::{
    Path,
    PathBuf,
};
use std::process::Command;
use std::sync::{
    Arc,
    LazyLock,
    Mutex,
    OnceLock,
};

use crate::{
    Operation,
    OptLevel,
    exe,
    fixture_cargo,
};

/// A module of the assembly fixture crate.
pub struct AsmCase {
    /// The module name.
    pub name: &'static str,
    pub source: &'static str,
}

pub const MY_U32: AsmCase = AsmCase {
    name: "my_u32",
    source: include_str!("../asm_case_my_u32.rs"),
};

pub const MY_U64_CONST: AsmCase = AsmCase {
    name: "my_u64_const",
    source: include_str!("../asm_case_my_u64_const.rs"),
};

pub const MY_SEQUENCE: AsmCase = AsmCase {
    name: "my_sequence",
    source: include_str!("../asm_case_my_sequence.rs"),
};

pub const MY_F64: AsmCase = AsmCase {
    name: "my_f64",
    source: include_str!("../asm_case_my_f64.rs"),
};

pub const MY_F32: AsmCase = AsmCase {
    name: "my_f32",
    source: include_str!("../asm_case_my_f32.rs"),
};

pub const MY_UNIT: AsmCase = AsmCase {
    name: "my_unit",
    source: include_str!("../asm_case_my_unit.rs"),
};

pub const MY_I32: AsmCase = AsmCase {
    name: "my_i32",
    source: include_str!("../asm_case_my_i32.rs"),
};

pub const DOMAIN_INT: AsmCase = AsmCase {
    name: "domain_int",
    source: include_str!("../asm_case_domain_int.rs"),
};

pub const COMPILE_TIME: AsmCase = AsmCase {
    name: "compile_time",
    source: include_str!("../asm_case_compile_time.rs"),
};

pub const WITHIN: AsmCase = AsmCase {
    name: "within",
    source: include_str!("../asm_case_within.rs"),
};

pub const PACKED: AsmCase = AsmCase {
    name: "packed",
    source: include_str!("../asm_case_packed.rs"),
};

/// Every case; each build holds all of them.
const CASES: [&AsmCase; 11] = [
    &MY_U32,
    &MY_U64_CONST,
    &MY_SEQUENCE,
    &MY_F64,
    &MY_F32,
    &MY_UNIT,
    &MY_I32,
    &DOMAIN_INT,
    &COMPILE_TIME,
    &WITHIN,
    &PACKED,
];

/// Crate attributes; `my_u64_const` needs the const-trait features.
const HEADER: &str = "
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]
";

/// The case of every probe.
static PROBES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    let mut result = HashMap::new();
    for case in CASES {
        for probe in probes(case.source) {
            if let Some(other) = result.insert(probe, case.name) {
                panic!(
                    "probe `{probe}` is declared by `{other}` and `{}`",
                    case.name
                );
            }
        }
    }
    return result;
});

/// The `#[unsafe(no_mangle)]` functions of `source`.
fn probes(source: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut exported = false;
    for line in source.lines().map(str::trim) {
        if line == "#[unsafe(no_mangle)]" {
            exported = true;
        }
        else if exported && let Some((_, rest)) = line.split_once("fn ") {
            let end = rest
                .find(|it: char| !(it.is_ascii_alphanumeric() || it == '_'))
                .unwrap_or(rest.len());
            result.push(&rest[..end]);
            exported = false;
        }
    }
    return result;
}

/// The case `symbol` (canonical) belongs to: one of its probes, or an item
/// of its module.
fn owner(symbol: &str) -> Option<&'static str> {
    if let Some(case) = PROBES.get(symbol) {
        return Some(case);
    }
    return CASES
        .iter()
        .map(|it| it.name)
        .find(|it| symbol.contains(&format!("crate::{it}::")));
}

type Builds = HashMap<
    (Option<&'static str>, OptLevel, Option<&'static str>),
    Arc<OnceLock<Arc<Build>>>,
>;

/// The build holding `case` for `target` (the host when `None`) at
/// `opt_level`: every case shares it, except where the build holds `case`
/// alone (`outlines`). The first caller builds it; concurrent callers wait
/// for it.
pub fn build(
    case: &'static AsmCase,
    target: Option<&'static str>,
    opt_level: OptLevel,
) -> Arc<Build> {
    static BUILDS: LazyLock<Mutex<Builds>> = LazyLock::new(Default::default);

    let alone = outlines(target, opt_level).then_some(case.name);
    let cell = Arc::clone(
        BUILDS
            .lock()
            .unwrap()
            .entry((target, opt_level, alone))
            .or_default(),
    );
    let build = cell.get_or_init(|| {
        let mut name = format!(
            "asm_{}_o{}",
            target.unwrap_or("host").replace('-', "_"),
            opt_level.cargo_value()
        );
        let build = match alone {
            Some(it) => {
                write!(name, "_{it}").unwrap();
                Build::new(&name, &[case], target, opt_level)
            }
            None => Build::new(&name, &CASES, target, opt_level),
        };
        return Arc::new(build);
    });
    return Arc::clone(build);
}

/// LLVM's machine outliner runs by default for AArch64 at opt-level z: it
/// moves instruction sequences that repeat anywhere in the crate into
/// shared functions, so other cases would change a case's code. Probes with
/// different IR can still compile to the same instructions, so distinct
/// bodies do not prevent it.
fn outlines(
    target: Option<&str>,
    opt_level: OptLevel,
) -> bool {
    return opt_level == OptLevel::Oz
        && target.is_some_and(|it| it.starts_with("aarch64"));
}

/// One build of a fixture crate: its function symbols and disassembly.
pub struct Build {
    /// The target and opt-level, for messages.
    label: String,
    /// Paths of the crate's own items start with it.
    crate_name: String,
    /// The linked host executable or the cross-target object.
    binary: PathBuf,
    /// The `(section, address)` of every function symbol.
    symbols: HashMap<String, (String, u64)>,
    /// Every function symbol at a `(section, address)`.
    aliases: HashMap<(String, u64), Vec<String>>,
    /// The size of the function at a `(section, address)`: alignment
    /// padding and the next function's data lie past it.
    sizes: HashMap<(String, u64), u64>,
    /// The `(address, line)` instructions of the function at a
    /// `(section, address)`, each followed by its relocations.
    functions: HashMap<(String, u64), Vec<(u64, String)>>,
}

impl fmt::Display for Build {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        return f.write_str(&self.label);
    }
}

impl Build {
    /// Writes the crate `name` holding `cases` under
    /// `target/typekin-fixtures/` and builds it for `target` (the host when
    /// `None`) at `opt_level`. A cross target is compiled to one relocatable
    /// object and never linked, so it needs no cross linker or target C
    /// library.
    pub fn new(
        name: &str,
        cases: &[&AsmCase],
        target: Option<&str>,
        opt_level: OptLevel,
    ) -> Self {
        let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("typekin-fixtures")
            .join(name);
        write_if_changed(
            &directory.join("Cargo.toml"),
            &manifest(name, opt_level),
        );
        write_if_changed(
            &directory.join("src").join("main.rs"),
            &main_source(cases),
        );

        let mut command = fixture_cargo();
        command
            .arg("+nightly")
            .arg(if target.is_some() { "rustc" } else { "build" })
            .args(["--offline", "--quiet", "--release", "--manifest-path"])
            .arg(directory.join("Cargo.toml"))
            .arg("--target-dir")
            .arg(directory.join("target"));
        let binary = match target {
            None => directory.join("target").join("release").join(name),
            Some(target) => {
                let object = directory.join(format!("{name}.o"));
                command
                    .arg("--target")
                    .arg(target)
                    .arg("--")
                    .arg(format!("--emit=obj={}", object.display()))
                    // One `.text` section: merged functions keep sharing an
                    // address, as they do in a linked binary.
                    .arg("-Zfunction-sections=no")
                    // Cargo still requests a link; skip it.
                    .arg("-Clinker=true");
                object
            }
        };
        exe("build_asm_fixture", command);

        let label = format!(
            "{} at opt-level {}",
            target.unwrap_or("host"),
            opt_level.cargo_value()
        );
        return Self::read(label, name, binary);
    }

    /// Reads the function symbols and disassembly of `binary`, a linked
    /// executable or a relocatable `.o` object of the crate `crate_name`.
    pub fn read(
        label: String,
        crate_name: &str,
        binary: PathBuf,
    ) -> Self {
        let mut symbols = HashMap::new();
        let mut aliases: HashMap<_, Vec<_>> = HashMap::new();
        let mut sizes = HashMap::new();
        for (name, key, size) in function_symbols(&binary) {
            aliases.entry(key.clone()).or_default().push(name.clone());
            sizes.insert(key.clone(), size);
            symbols.insert(name, key);
        }
        let functions = functions(&disassemble(&binary));
        return Self {
            label,
            crate_name: crate_name.to_owned(),
            binary,
            symbols,
            aliases,
            sizes,
            functions,
        };
    }

    /// Runs `case`'s `main`, which checks that its probes compute the same
    /// results.
    pub fn run(
        &self,
        case: &AsmCase,
    ) {
        let mut command = Command::new(&self.binary);
        command.arg(case.name);
        exe("run_asm_case", command);
    }

    /// The normalized instructions of the function `symbol`; an alias reads
    /// the function at its address.
    #[must_use]
    pub fn disassembly(
        &self,
        symbol: &str,
    ) -> Disassembly {
        let key = self.symbols.get(symbol).unwrap_or_else(|| {
            panic!("missing symbol `{symbol}` in {self}");
        });
        // A size of zero leaves the function to the next symbol.
        let end = match self.sizes[key] {
            0 => u64::MAX,
            size => key.1 + size,
        };
        let lines = self
            .functions
            .get(key)
            .map(|it| {
                return it
                    .iter()
                    .filter(|(address, _)| *address < end)
                    .map(|(_, line)| line.as_str())
                    .collect::<Vec<_>>();
            })
            .filter(|it| !it.is_empty())
            .unwrap_or_else(|| {
                panic!("no instructions for `{symbol}` in {self}")
            });
        return normalize(
            &lines,
            &self.aliases[key],
            &self.crate_name,
            !is_object(&self.binary),
        );
    }

    /// Panics unless `operations` name probes of `case` and no probe of
    /// `case` shares code with another case: an alias at its address, or a
    /// reference to another case's code.
    pub fn assert_isolated(
        &self,
        case: &AsmCase,
        operations: &[Operation],
    ) {
        let mut violations = operations
            .iter()
            .flat_map(|it| [it.raw, it.wrapped])
            .filter(|it| owner(it) != Some(case.name))
            .map(|it| format!("`{it}` is not a probe of this case"))
            .collect::<Vec<_>>();
        for probe in probes(case.source) {
            // A linked binary drops the probes nothing calls.
            let Some(key) = self.symbols.get(probe)
            else {
                continue;
            };
            for alias in &self.aliases[key] {
                let alias = canonical(alias, &self.crate_name);
                if let Some(other) = owner(&alias).filter(|it| *it != case.name)
                {
                    violations.push(format!(
                        "`{probe}` shares its address with `{alias}` of `{other}`"
                    ));
                }
            }
            for reference in self.disassembly(probe).references {
                if let Some(other) =
                    owner(&reference).filter(|it| *it != case.name)
                {
                    violations.push(format!(
                        "`{probe}` refers to `{reference}` of `{other}`"
                    ));
                }
            }
        }
        assert!(
            violations.is_empty(),
            "case `{}` ({self}) shares code with other cases; give each case \
             its own types and probe bodies:\n{}",
            case.name,
            violations.join("\n")
        );
    }
}

fn manifest(
    name: &str,
    opt_level: OptLevel,
) -> String {
    let typekin = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()
        .expect("canonicalize the typekin root");
    let level = match opt_level {
        OptLevel::Os | OptLevel::Oz => {
            format!("\"{}\"", opt_level.cargo_value())
        }
        _ => opt_level.cargo_value().to_owned(),
    };
    return format!(
        r#"[package]
name = "{name}"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
typekin = {{ path = "{}" }}

[profile.release]
opt-level = {level}
codegen-units = 1
debug = false
incremental = false
lto = false
overflow-checks = false
panic = "abort"
"#,
        typekin.display(),
    );
}

/// One module per case, and a `main` running the case its first argument
/// names.
fn main_source(cases: &[&AsmCase]) -> String {
    let mut result = String::from(HEADER);
    for case in cases {
        write!(result, "\nmod {} {{\n{}}}\n", case.name, case.source).unwrap();
    }
    result.push_str(
        "
fn main() {
    let case = std::env::args().nth(1).unwrap_or_default();
    match case.as_str() {
",
    );
    for case in cases {
        writeln!(result, "        \"{0}\" => {0}::main(),", case.name).unwrap();
    }
    result.push_str(
        "        _ => panic!(\"unknown case `{case}`\"),
    }
}
",
    );
    return result;
}

/// Leaves unchanged files untouched so cargo can reuse its results.
fn write_if_changed(
    path: &Path,
    content: &str,
) {
    if std::fs::read_to_string(path).is_ok_and(|it| it == content) {
        return;
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap_or_else(|error| {
            panic!("cannot create {}: {error}", parent.display())
        });
    }
    std::fs::write(path, content).unwrap_or_else(|error| {
        panic!("cannot write {}: {error}", path.display())
    });
}

/// Cross-target objects need LLVM's tools: the host's GNU binutils read only
/// the host architecture.
fn is_object(binary: &Path) -> bool {
    return binary.extension().is_some_and(|it| it == "o");
}

/// `name` itself when it is on `PATH`, otherwise the copy that rustup's
/// `llvm-tools` component installs in the nightly sysroot (not on `PATH`).
/// A tool found in neither place keeps its bare name, so running it fails.
fn llvm_tool(name: &str) -> PathBuf {
    let on_path = std::env::var_os("PATH").is_some_and(|path| {
        return std::env::split_paths(&path).any(|it| it.join(name).is_file());
    });
    if on_path {
        return PathBuf::from(name);
    }
    let installed = LLVM_TOOLS.join(name);
    if installed.is_file() {
        return installed;
    }
    return PathBuf::from(name);
}

/// `<sysroot>/lib/rustlib/<host>/bin` of the nightly toolchain.
static LLVM_TOOLS: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut sysroot = Command::new("rustc");
    sysroot.args(["+nightly", "--print", "sysroot"]);
    let mut version = Command::new("rustc");
    version.args(["+nightly", "-vV"]);
    let version = exe("rustc_version", version);
    let host = version
        .lines()
        .find_map(|it| it.strip_prefix("host: "))
        .expect("rustc -vV names the host");
    let sysroot = exe("rustc_sysroot", sysroot);
    let sysroot = sysroot.lines().next().expect("rustc prints its sysroot");
    return PathBuf::from(sysroot)
        .join("lib")
        .join("rustlib")
        .join(host)
        .join("bin");
});

/// `(name, (section, address), size)` of every function symbol, from the
/// SysV listing `name|value|class|type|size|line|section`.
fn function_symbols(binary: &Path) -> Vec<(String, (String, u64), u64)> {
    let mut command = match is_object(binary) {
        true => Command::new(llvm_tool("llvm-nm")),
        false => Command::new("nm"),
    };
    command
        .args(["--demangle", "--defined-only", "--format=sysv"])
        .arg(binary);
    return exe("read_symbols", command)
        .lines()
        .filter_map(|line| {
            let fields = line.split('|').map(str::trim).collect::<Vec<_>>();
            let [name, value, _, "FUNC", size, _, section] = fields[..]
            else {
                return None;
            };
            let address = u64::from_str_radix(value, 16).ok()?;
            let size = u64::from_str_radix(size, 16).ok()?;
            return Some((
                name.to_owned(),
                (section.to_owned(), address),
                size,
            ));
        })
        .collect();
}

fn disassemble(binary: &Path) -> String {
    let mut command = match is_object(binary) {
        true => Command::new(llvm_tool("llvm-objdump")),
        false => Command::new("objdump"),
    };
    command.args(["--disassemble", "--demangle", "--no-show-raw-insn"]);
    if is_object(binary) {
        command.arg("--reloc");
    }
    command.arg(binary);
    return exe("disassemble", command);
}

/// The `(address, line)`s of every function in `disassembly`, by
/// `(section, address)`: both tools stop a function at the next symbol.
fn functions(disassembly: &str) -> HashMap<(String, u64), Vec<(u64, String)>> {
    let mut result: HashMap<_, Vec<_>> = HashMap::new();
    let mut section = "";
    let mut current = None;
    for line in disassembly.lines() {
        if let Some(it) = line.strip_prefix("Disassembly of section ") {
            section = it.trim_end_matches(':');
            current = None;
        }
        else if let Some((address, rest)) = line.split_once(" <")
            && rest.ends_with(">:")
            && let Ok(address) = u64::from_str_radix(address, 16)
        {
            let key = (section.to_owned(), address);
            result.entry(key.clone()).or_default();
            current = Some(key);
        }
        else if let Some(key) = &current
            && let Some((offset, text)) = line.trim_start().split_once(':')
            && let Ok(address) = u64::from_str_radix(offset, 16)
            && !text.trim().is_empty()
        {
            let line = (address, text.trim().to_owned());
            result.get_mut(key).unwrap().push(line);
        }
    }
    return result;
}

/// One function's instructions, comparable across builds.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Disassembly {
    /// One line per instruction, its relocations appended. Addresses inside
    /// the function are relative to its start; other symbols lose the crate
    /// name and crate-wide numbering; trailing alignment padding is dropped.
    pub lines: Vec<String>,
    /// The addresses a linked binary's instructions refer to outside the
    /// function, so the same instructions referring to different code or
    /// data still differ. An object refers through relocations instead.
    pub targets: Vec<u64>,
    /// The symbols the instructions refer to outside the function.
    pub references: Vec<String>,
}

fn normalize(
    raw: &[&str],
    own: &[String],
    crate_name: &str,
    linked: bool,
) -> Disassembly {
    let mut result = Disassembly::default();
    let mut lines = raw.iter().copied().peekable();
    while let Some(instruction) = lines.next() {
        let mut relocations = Vec::new();
        while let Some(it) = lines.next_if(|it| it.starts_with("R_")) {
            let (kind, target) =
                it.split_once(char::is_whitespace).unwrap_or((it, ""));
            let (symbol, addend) = split_addend(target.trim());
            let symbol = canonical(symbol, crate_name);
            // A section's addend places the item among the others.
            let addend = if symbol.starts_with('.') { "" } else { addend };
            relocations.push(format!("{kind} {symbol}{addend}"));
            result.references.push(symbol);
        }
        let mut line = operands(
            instruction,
            own,
            crate_name,
            linked,
            !relocations.is_empty(),
            &mut result,
        );
        for it in relocations {
            write!(line, " ; {it}").unwrap();
        }
        result.lines.push(line);
    }
    while result.lines.last().is_some_and(|it| is_padding(it)) {
        result.lines.pop();
    }
    return result;
}

/// Alignment after the last instruction depends on what follows the
/// function, not on the function.
fn is_padding(line: &str) -> bool {
    return line == "int3"
        || line.split(' ').any(|token| token.starts_with("nop"));
}

/// Rewrites every address annotated with a symbol: `<self+0x10>` inside the
/// function, `<symbol>` for another function, and `<address>` inside other
/// code or data of a linked binary. A relocated field holds a placeholder
/// until linking, `<reloc>`, and on targets that keep the addend in the
/// instruction its numbers place the target among the crate's items:
/// `0x?`.
fn operands(
    instruction: &str,
    own: &[String],
    crate_name: &str,
    linked: bool,
    relocated: bool,
    result: &mut Disassembly,
) -> String {
    let collapsed = without_rip_displacement(
        &instruction.split_whitespace().collect::<Vec<_>>().join(" "),
    );
    let mut line = String::with_capacity(collapsed.len());
    let mut rest = collapsed.as_str();
    while let Some(open) = rest.find(" <")
        && let Some(length) = annotation_length(&rest[open + 1..])
    {
        let end = open + 1 + length;
        let before = &rest[..open];
        let start = before.rfind(' ').map_or(0, |it| it + 1);
        let digits = before[start..].trim_start_matches("0x");
        let Ok(address) = u64::from_str_radix(digits, 16)
        else {
            line.push_str(&rest[..end]);
            rest = &rest[end..];
            continue;
        };
        let (symbol, offset) = split_offset(&rest[open + 2..end - 1]);
        line.push_str(&before[..start]);
        if relocated {
            line.push_str("<reloc>");
        }
        else if own.iter().any(|it| it == symbol) {
            write!(line, "<self{offset}>").unwrap();
        }
        else if linked && !offset.is_empty() {
            result.targets.push(address);
            line.push_str("<address>");
        }
        else {
            let symbol = canonical(symbol, crate_name);
            if linked {
                result.targets.push(address);
            }
            write!(line, "<{symbol}{offset}>").unwrap();
            result.references.push(symbol);
        }
        rest = &rest[end..];
    }
    line.push_str(rest);
    if relocated {
        // A zero displacement is not printed: `0x14(%ebx)` and `(%ebx)` are
        // the same placeholder.
        line = without_numbers(&line).replace("0x?(", "(");
    }
    return line;
}

/// `line` with every `0x…` number replaced by `0x?`.
fn without_numbers(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(start) = rest.find("0x") {
        result.push_str(&rest[..start + 2]);
        let tail = &rest[start + 2..];
        let digits = tail
            .find(|it: char| !it.is_ascii_hexdigit())
            .unwrap_or(tail.len());
        result.push_str(if digits == 0 { "" } else { "?" });
        rest = &tail[digits..];
    }
    result.push_str(rest);
    return result;
}

/// The length of the `<…>` annotation `text` starts with; symbols nest
/// angle brackets, and `->` closes none.
fn annotation_length(text: &str) -> Option<usize> {
    let mut depth = 0usize;
    let mut previous = ' ';
    for (index, it) in text.char_indices() {
        match it {
            '<' => depth += 1,
            '>' if previous != '-' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            _ => {}
        }
        previous = it;
    }
    return None;
}

/// `("symbol", "+0x10")` from `symbol+0x10`.
fn split_offset(annotation: &str) -> (&str, &str) {
    return match annotation.rfind("+0x") {
        Some(index)
            if annotation[index + 3..]
                .chars()
                .all(|it| it.is_ascii_hexdigit()) =>
        {
            (&annotation[..index], &annotation[index..])
        }
        _ => (annotation, ""),
    };
}

/// `("symbol", "-0x4")` from the relocation target `symbol-0x4`.
fn split_addend(target: &str) -> (&str, &str) {
    let index = target
        .rfind("+0x")
        .into_iter()
        .chain(target.rfind("-0x"))
        .max();
    return match index {
        Some(index)
            if target[index + 3..].chars().all(|it| it.is_ascii_hexdigit()) =>
        {
            (&target[..index], &target[index..])
        }
        _ => (target, ""),
    };
}

/// `symbol` without what depends on the crate it was built in: the crate
/// name, and the crate-wide numbering of anonymous constants
/// (`anon.<hash>.<n>`) and local labels (`.LCPI3_0`).
fn canonical(
    symbol: &str,
    crate_name: &str,
) -> String {
    let prefix = format!("{crate_name}::");
    let mut result = String::with_capacity(symbol.len());
    let mut rest = symbol;
    while let Some(start) = rest.find(&prefix) {
        let segment = !rest[..start]
            .ends_with(|it: char| it.is_ascii_alphanumeric() || it == '_');
        result.push_str(&rest[..start]);
        result.push_str(if segment { "crate::" } else { &prefix });
        rest = &rest[start + prefix.len()..];
    }
    result.push_str(rest);

    let symbol = std::mem::take(&mut result);
    let mut rest = symbol.as_str();
    while let Some(start) = rest.find("anon.") {
        result.push_str(&rest[..start + 4]);
        let tail = &rest[start + 5..];
        let numbering = tail
            .find(|it: char| !(it.is_ascii_alphanumeric() || it == '.'))
            .unwrap_or(tail.len());
        rest = &tail[numbering..];
    }
    result.push_str(rest);
    if result.starts_with(".L") {
        let end = result
            .trim_end_matches(|it: char| it.is_ascii_digit() || it == '_')
            .len();
        result.truncate(end);
    }
    return result;
}

/// `-0x107e3(%rip) # 5240 <…>` depends on where the instruction is; the
/// disassembler's comment already names the absolute target, so the
/// displacement is dropped.
#[must_use]
fn without_rip_displacement(instruction: &str) -> String {
    if !instruction.contains("(%rip)") || !instruction.contains('#') {
        return instruction.to_owned();
    }
    return instruction
        .split(' ')
        .map(|token| {
            let Some(index) = token.find("(%rip)")
            else {
                return token.to_owned();
            };
            let displacement = &token[..index];
            let start = displacement
                .rfind(|it: char| {
                    !(it.is_ascii_hexdigit() || it == 'x' || it == '-')
                })
                .map_or(0, |it| it + 1);
            return format!("{}{}", &token[..start], &token[index..]);
        })
        .collect::<Vec<_>>()
        .join(" ");
}

/// `operation`'s two sides, line by line.
#[must_use]
pub fn text_diff(
    operation: &Operation,
    raw: &Disassembly,
    wrapped: &Disassembly,
) -> String {
    let mut result = String::new();
    writeln!(
        result,
        "[OP_DIFF::{}] {} != {}",
        operation.name, operation.raw, operation.wrapped
    )
    .unwrap();
    writeln!(result, "--- {}", operation.raw).unwrap();
    writeln!(result, "+++ {}", operation.wrapped).unwrap();

    let count = raw.lines.len().max(wrapped.lines.len());
    for index in 0..count {
        match (raw.lines.get(index), wrapped.lines.get(index)) {
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
    if raw.targets != wrapped.targets {
        writeln!(result, "- targets {:x?}", raw.targets).unwrap();
        writeln!(result, "+ targets {:x?}", wrapped.targets).unwrap();
    }

    return result;
}

/// `operation`'s two sides, identical.
#[must_use]
pub fn assembly_match(
    operation: &Operation,
    assembly: &Disassembly,
) -> String {
    let mut result = String::new();
    writeln!(
        result,
        "operation `{}` matched: {} and {}",
        operation.name, operation.raw, operation.wrapped
    )
    .unwrap();
    for line in &assembly.lines {
        writeln!(result, "  {line}").unwrap();
    }
    return result;
}
