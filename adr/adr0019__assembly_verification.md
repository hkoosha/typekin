# ADR 0019: Assembly verification of zero cost

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `typekin_testing/src/machinery/asm.rs`, `typekin_testing/src/asm_case_*.rs`, `typekin_testing/src/machinery/fixture.rs` (`fixture_cargo`), `typekin_testing/Cargo.toml` (features), AGENTS.md (assembly rules), PHILOSOPHY.md; ADR 0003, 0004, 0007, 0020

## Context
typekin promises that a wrapper vanishes: generated operations cost the same as hand-written code on the primitive. That claim needs evidence that rustc and LLVM actually deliver it, at the opt-levels and on the targets consumers use.

## Decision
- **Proof is assembly.** Each probe of a typekin operation is compared against a hand-written probe on the raw primitive. Opt-levels 1, 2, 3 and s must match; opt-level 0 must differ (negative control). "Compilation or `#[inline]` alone is not proof" (AGENTS.md, PHILOSOPHY.md).
- **O0 control per fixture.** `#[inline(always)]` makes 6 of 9 float domain pairs byte-identical even at O0 (`6a9e880:prev.md`), so a global "O0 differs" assumption is false; each fixture must show its own O0 difference, otherwise the comparison has not shown it can detect anything.
- **Origin.** Originally three crates from the user's prompts: `typekin_perf_types` (definitions), `typekin_perf_ok` (opt-level 3, must match) and `typekin_perf_must_fail` (unoptimized, "We expect the assembly to differ"). Folded into `typekin_testing` in `a2c7f6c`.
- **Batching:** one fixture crate per (target, opt-level), one module per case (`asm_case_*.rs`), built once per run under `target/typekin-fixtures/asm_<target>_o<level>`. Safe because the checked paths are `#[inline(always)]`: `alwaysinline` ignores caller count and cost, and inline items are copied into every codegen unit that uses them, so neither other cases nor the CGU split change inlining. Effects within one case (below-O2 instruction selection, the extra `inline(always)` layer of checked float conversions) look the same batched or not.
- **Cross-case merging** is prevented in the source, not by flags: each case declares its own types and gives its probes distinct bodies (`my_u32_const` became `my_u64_const`, `my_sequence` uses `u16`/`i16`, five `my_unit` pairs take references). `Build::assert_isolated` fails a case whose probe shares an address with, or refers to, another case's code.
- **Normalization:** crate-wide label, anonymous-constant and panic-location numbering; trailing `nop`/`int3` padding dropped; crate names in generic paths replaced by `crate::` (only at segment start); functions cut at their `nm --format=sysv` size (armv7 literal pools); relocated numbers masked (`0x?`) and relocations shown with `llvm-objdump --reloc` (i686 REL addends).
- **AArch64 at opt-level z builds one crate per case** (`fn outlines()` in `asm.rs`), because of the machine outliner (below).
- **Opt-level z** runs under `test_slow_z`, which today is part of `test_slow`.
- **Fixture builds strip `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS` and `CARGO_BUILD_RUSTFLAGS`** (`fixture_cargo`). The justfile's `-Zmacro-backtrace` moved const-panic diagnostics into `core/panic.rs` and broke three rejection tests; and "any other flag could change the code under comparison."

## Rejected alternatives
- **Trusting compilation or `#[inline]`.** Neither shows the optimizer removed the wrapper.
- **One `[[bin]]` crate per test in a fresh `/tmp` directory** (the former "never batch" rule). `test_slow` 121.6 s → 18.2 s batched (warm 3.5 s); `test_slow_cross` 204.9 s → 24.3 s (warm 4.8 s). The `/tmp` dirs had grown to about 15 GB (197 × ~79 MB).
- **`-Zmerge-functions=disabled`.** Forbidden by the user up front; AGENTS records only "merging is not disabled with compiler flags". [INFERENCE] Same reason as the outliner flag: it would no longer test the code consumers get. A prototype found 16 merged groups across cases at every level ≥ O2; renaming probes alone did nothing because merging works on bodies.
- **`-Cllvm-args=-enable-machine-outliner=never`** (option b). "A compiler flag that turns off a cross-function pass, close to the rule against disabling merging, and aarch64 at opt-level z would then no longer test the code consumers actually get."
- **Accept outlining and normalize outlined bodies** (option c). "Batched still won't be identical to per-case."
- **Revert batching** (option d). Loses the measured speedup. Option (a), per-case crates for AArch64 z only, was chosen ("Keep aarch64 per-case"). Reason for it: 36 probes differed batched vs per-case at aarch64 z (21 outlined only when batched, 15 branching to differently numbered `OUTLINED_FUNCTION_*`); distinct IR can still produce identical machine code.
- **Opt-level z on by default → opt-in → back in `test_slow`.** The user made z opt-in: "-z is inherently at odds with promises of `type` vanishes … Even hand written code for plain numbers might be outlined too if pattern is repeated enough." Later `test_slow` was changed to include `test_slow_z`; no reason recorded.
- **Dropping the `RUSTFLAGS` export from the justfile** instead of stripping it: not considered.

## Consequences
- Per-case types change coverage: `konst = nightly` arithmetic is now tested on u64, not u32.
- After z became opt-in, removing `outlines()` was offered and never answered; it remains. Outlined-function numbers are not normalized.
- The batched-vs-per-case equivalence check (269 probes, 4 targets, 6 levels) was a throwaway; nothing in the repo reproduces it. Changed cases were never compared with the pre-batching builds.
- Proof of zero cost lives only in `test_slow`; see ADR 0020 for the fast-suite gap.

## Sources
- omp session 01a1a596 (2026-11-04; batching, outliner options, z opt-in, drift)
- `5cb75a7:PROMPTS.md` (same text in `fb7ec18:README_PROMPTS.md`); commit `a2c7f6c` (fold-in)
- omp session 01a1a5e0 (2026-11-04; `RUSTFLAGS` stripping)
- `6a9e880:prev.md` L28942 (Copilot session transcript, 2026-10-31; O0 control per fixture)
- AGENTS.md (assembly rule text)
