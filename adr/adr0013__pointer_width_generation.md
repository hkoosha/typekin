# ADR 0013: Pointer-width code generation

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/integral.rs` (`pointer_width_cfg`, `cast_targets`),
  `src/zz_values.rs` (`can_safe_cast_to`), `src/integral_items.rs`,
  `src/integral_friend.rs`, `src/zz_domain_int.rs`, `../src/unit`,
  `typekin_testing/src/fx_pointer_width_compile.rs`, `src/lib.rs`
  cross-compilation docs, `PLAN_WITHIN.md`; ADR 0006 (domain), ADR 0011 (NonZero
  storage), ADR 0022 (integral API)

## Context

`usize`/`isize` widths depend on the consumer's target, but a proc macro runs on
the host. Code like `N::USIZ => usize::BITS` inside the macro baked in the host
width: a 64-bit host building for wasm32 produced silently truncating
conversions and wrong domain checks.

## Decision

- Generate the applicable 16-, 32- and 64-bit variants in one expansion and put
  `#[cfg(target_pointer_width = "...")]` on the **generated items**.
  Width-independent items stay unconditional; items valid on no width are not
  emitted. User: "all code paths are generated regardless of the target or host
  bit width."
- Applies to accessors, conversion methods/traits, and *automatic* `Make`
  friends. Explicitly declared friends stay available.
- Domains are normalized per width (ranges merged), emitting cfg-selected
  validation; endpoint expressions are preserved and unselected variants aren't
  type-checked.
- Integer conversion losslessness is computed by range inclusion of source and
  destination at each width (`can_safe_cast_to(to, pointer_width)`), not by hand
  tables. Checked (`try_into_*`) targets are the complement.
- The public API varies by target by design: `u32 → isize` is `Into` on 64-bit,
  `TryInto<Error = ()>` on 16/32-bit.
- Zero exclusion (for NonZero storage, ADR 0011) must hold on every supported
  width.

## Rejected alternatives

- **Host-sized constants or `#[cfg]` inside the proc macro.** The proc-macro
  crate is built for the host, so both select the host width.
- **Reading `CARGO_CFG_TARGET_POINTER_WIDTH` in the macro.** Cargo documents it
  for build scripts, not as a proc-macro target API.
- **Keeping symbolic ranges unmerged.** Sound fallback, but per-width
  normalization was preferred "because range merging avoids runtime
  comparisons."
- **Hand-written lossless-cast tables.** They had an independent bug:
  `SAFE_I016_32` classed `i16 → u16` as safe, so `-1i16` converted to `65535`.
  The DEDUP cleanup first kept the tables deliberately ("correcting them changes
  the generated API and behavior"); they were later replaced by range inclusion.
  `arena/DEDUP.md` §3 is out of date on this.

## Consequences

- Observed before the fix (64-bit host → wasm32): `Wide(2^32).try_into_usize()`
  returned `Ok(0)`; automatic `Word::of(2^32u64)` produced raw `0`; const
  `usize` `byte4()` failed with E0080 (shift overflow); domain
  `[isize::MIN..-2_147_483_648, -isize::MAX..]` accepted `isize::MIN`.
- After: native-64 and wasm32 runtime smoke, AVR 16-bit compile, 432
  width-specific conversion probes (12×12 pairs × 3 widths), downstream slow
  suite 461 passed (historical, not rerun).
- Expansions are larger (up to three cfg'd copies); code that compiles on one
  target may not have the same conversion traits on another.
- wasm32 fixtures need the `test_slow_wasm32` feature.

## Sources

- omp session 01a17f6a (2026-10-27); commit lead `abfef808` "fix corss
  compiling" [INFERENCE: association unconfirmed]
- `arena/DEDUP.md` §3
- omp session 01a17f3e (2026-10-27)
