# ADR 0023: packed: design decisions

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `PLAN_PACKED.md` (K0–K7 done), `src/packed.rs`, `examples/my_packed.rs`, README "Packed records", ADR 0001 (why hand-rolled packing at all), ADR 0024 (`Within`, which proves field windows), ADR 0006, ADR 0009

## Context
ADR 0001 decided to pack records by hand instead of using a Serde format. This ADR records how the `packed` record macro looks, which alternatives were rejected while planning it (DK1–DK9, §10, §11 of `PLAN_PACKED.md`), and where the implementation refined the plan.

## Decision
- Named-field struct; each field carries an explicit bit range, bit 0 = LSB (DK1, DK2).
- `make` takes a generated `{Type}Values` struct (named arguments) (DK3).
- `repr` is unsigned `u8..u128` only (DK7).
- Unclaimed bits must be zero; the representation is canonical, so `Eq`/`Hash` work on the raw value (DK5, §5.3).
- A value that does not fit its window is a compile error, proven through `Within` (ADR 0024, W6).
- Getters re-check via `F::try_make`; the `Err` arm is unreachable (DK4).
- `non_zero` storage is opt-in and proven (DK6).
- `make`/`with_*` skip the validator, use the safe storage path, keep `debug_assert!` (same as ADR 0024 DW3/DW4).
- Integral keys are rejected on records with "`packed` does not support `KEY`" (DK8).
- Encodings are a closed built-in set; v1 has natural encoding only (DK9).
- Every wrapper field gets a fit-proof constant that rustc evaluates during `cargo check`. Only rustc knows whether the window equals the field's raw width, and it skips the check in that case. The proof reuses the `Within` subset check and message helpers (`zz_domain_within::emit_check`, `emit_messages`) against the window's range, which the constant picks from the field's published signedness.
- A window wider than the field's raw type is a const-evaluation error, because the representation would stop being canonical.
- With `with = non_zero`, validation also rejects zero. `try_make`'s unchecked `NonZero` construction therefore never depends on the proof; the proof only guarantees that `make` cannot panic.
- Unsigned wrapper fields are written without a mask, since their fit is proven; signed fields mask off their sign extension. This makes `make` compile to the same code as the hand-written version (asm `packed` case).
- Only `konst` is accepted; `konst_*` flags are rejected like integral flags. `bool` also accepts a one-bit `#[bits(..)]`.

## Rejected alternatives
- **Tuple struct + `field = [...]` list (DK1).** "Loses field docs and per-field visibility."
- **Sequential widths (DK2).** "Reordering fields would silently change the layout"; explicit ranges also "match wire-format specs".
- **Positional `make(a, b, c)` (DK3).** The values struct gives "named arguments at the call site; no swapped same-typed arguments".
- **Name `{Type}Fields` / `fields_name` / `without = fields_struct`.** Renamed to `{Type}Values` / `values_name` / `values_struct` in 13703fb. No reason recorded.
- **Public hidden unchecked constructor on field types, making getters free (DK4).** "Any code could call it, breaking the field type's invariant." Cost accepted: one compare + cold panic for fields whose domain the window does not imply, "documented as the price".
- **Retaining unclaimed bits (DK5).** "Two records with equal fields could compare unequal." May come back later as explicit `gaps = retain`.
- **Inferring `non_zero` storage (DK6).** "Storage type is chosen at expansion, the proof needs rustc."
- **Signed `repr` or `usize` (DK7).** "Layout must not depend on sign or target."
- **Record-level `domain`/`valid` (DK8).** "It would make `make`/`with_*` fallible." Not in v1.
- **Other integral keys on records (`friend`, math, operators, `facet`, …).** Rejected with "`packed` does not support `KEY`".
- **Arbitrary field attributes (derives, `serde`).** "The struct is rewritten; derives and `serde` attributes on fields would not apply."
- **User `encode`/`decode` per field (DK9).** "Typekin cannot see into a codec": no fit proof, so `make`/`with_*` would need runtime checks; a non-injective `decode` makes raw `Eq`/`Hash` disagree with the fields; asm fixtures could not back the zero-cost claims. Exotic encodings (BCD, Gray, sign-magnitude) should be field types whose raw value is the bit pattern.
- **Truncating/masking oversized values.** "Silently changes data"; compile error instead.
- **`offset` K inferred from the domain minimum (§11).** "A domain change would otherwise move the wire encoding" (the same argument as DK2 for positions). K must be explicit.
- **`offset` on `bool`, bitflag values, nested packed records (§11).** They are "bit patterns, not numbers".
- **`scale` now (§11).** Deferred. If added, offset applies before scale. Open issues: wrap collisions, folding the `%s` re-check, the signed definition.

## Consequences
- Implemented in `src/packed.rs` on top of `integral::ekran_with`. The window proofs depend on the `Within` source constants (ADR 0024, W4).
- Getters of fields whose domain the window does not imply keep a branch.
- Layout is stable under field reordering and domain edits, at the cost of explicit bit ranges.

## Sources
- `PLAN_PACKED.md` (HEAD; history 0c3e9aa `packed-plan.md` → a03de93 → 13703fb, which added DK9, §10 codec entry, §11 and the `Values` rename)
- `PLAN_PACKED.md` §10; omp session 01a14b3e (2026-10-17)
