# ADR 0003: Closure proofs at expansion time, and the `proven` default

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/zz_domain_closure.rs`, `src/integral_proof.rs`,
  `src/integral_cfg.rs` (`settle_proven`), `src/integral_items.rs` (`_proven`
  helpers), `src/tests/t_domain_closure.rs`, `examples/my_proven.rs`, README
  "The Catch", ADR 0013 (pointer widths), ADR 0024 (`Within`, cross-type proofs)

## Context

`Percent(u8)` with `domain = ..=100`: `Percent / Percent`, `%` and `midpoint`
can never leave the domain, yet the asm still had a redundant
`cmpb $101 … panic_fmt`. LLVM folds simple ranges but "cannot see a general
`domain`" (556c009 §1.4). Something has to know that the result is in range.

## Decision

- typekin proves closure itself at expansion time: interval arithmetic over the
  resolved domain (`zz_domain_closure.rs`). A proven operation calls a
  `_proven` helper that skips validation and documents it.
- `_proven` keeps `::core::debug_assert!(validation, "typekin closure proof
  violated")` and builds through the normal storage path. Free in release.
- Overflow policy per operation (`closure_policy`): checked profiles and
  `checked_*`/`strict_*` → `Checked`; `saturating_*` → `Saturating`; plain
  `+ - *` (may panic in debug or wrap in release), `wrapping_*`, bitwise ops
  and `!` → `Wrapping`, so the proof covers the wrapped results too. This is
  why `Percent + Percent` is never proven under the `int` profile.
- A proof must hold at **every pointer width**, and is made **per right-hand
  side**: `Rhs::SameDomain` vs `Rhs::Full` (raw/friend). `Port | Port` is
  proven, `Port | u16` is not (README:243).
- Scope: integer wrappers only ("closure proofs support integer wrappers
  only"), domains that are unions of plain intervals, at most 16 intervals per
  operand (`MAX_INTERVALS`); above that, no proof. Opaque atoms (`!=`, `%`
  /`mul`, lenses), `valid` callbacks and `with = non_zero` (adds an opaque
  `!= 0`; write `1..` instead) block proofs. Not proven: `<<` (`Shl` has no
  `ClosureOp`; `Shr`/`UnboundedShr` do), `pow`, `ilog*`.
- `prove = [...]` is a checked contract: it fails expansion with a
  counterexample and names a blocking atom. It **only checks**: emission never
  consults it, so `prove` cannot change which `_proven` paths exist (PLAN.md).
- Proven items carry the doc line "Cannot fail validation: every result lies in
  the domain `…` at every pointer width (proven by typekin at expansion)", with
  the domain as written. Reason: "Infallibility is caller-visible API" (DB5).
- The `proven` flag defaults **on only for `u8`/`i8`**; wider integers opt in
  with `with = proven`; `prove` implies it; `prove` with `without = proven` is
  an error (`settle_proven`, commit `5d14cb7`). This was the user's decision;
  **no reason recorded**. Measured facts nearby: exhaustive const evaluation of
  2^20 values trips the deny-by-default `long_running_const_eval` lint after
  2.35 s, while 65,536 values check in 0.22 s (PLAN_WITHIN §1.2, measured for
  `Within`'s rustc-side enumeration, not for these interval proofs), and the
  soundness sweeps against Rust's own operations are exhaustive only for `u8`
  and `i8` (boundary samples for `u128`/`i128`, `t_domain_closure.rs:872`).
  [INFERENCE] The default follows the width where soundness is exhaustively
  tested.

## Rejected alternatives

- **`core::hint::assert_unchecked(valid(raw))` on every field read**, letting
  LLVM find closure. Lost to "generated `unsafe` on every access, possible
  pessimization of unrelated code, and correctness invisible to typekin's
  tests" (556c009 §5). Same idea was already rejected for float `raw()` hints
  (unsound with safe in-module construction).
- **Always validating** (no proofs). Leaves the measured redundant
  compare-and-panic on operations that cannot fail.
- **Dropping the check entirely on proven paths.** Kept `debug_assert!` as a
  "safety net for analyzer bugs, as std debug-checks unchecked preconditions;
  free in release" (DB2). With `NonZero` storage a wrong proof reaches
  `new_unchecked`, i.e. UB, so proof code is held "to the same bar as the zero
  probe". `Within` chose a safe storage path instead (PLAN_WITHIN DW3).
- **One proof per operator / per single width.** Per RHS because "`Percent /
  Percent` may be proven while `Percent / u8` is not" (DB4); every width as the
  "same rule as domain folding and zero probes" (DB3).
- **Proofs on by default for every integer** (DB1: "Proofs apply
  automatically; `prove` only adds the requirement", reason "unobservable except
  removed impossible panics"). Reversed by `5d14cb7`; reason not recorded.
- **Float proofs.** "Rounding, NaN, and signed zeros need directed-rounding
  interval arithmetic; separate design"; `prove`/`proven` on floats is an
  error.
- **Cross-type proofs in the macro** ("the macro sees one type"). Done later
  by `Within` with rustc-evaluated const proofs (ADR 0024).

## Consequences

- Wider integers keep their checks unless the user writes `with = proven`.
- `Shl` never gets a proof or proof doc; only `Shr`/`ShrAssign` do.
- Xor's friend `BitXorAssign` now uses `binary_op`'s proof doc (PLAN.md P6.4),
  but no accepted domain proves xor for a raw right-hand side, so that doc is
  unreachable and untested.
- Three fast proof tests (proven division, proven core-int method, unproven
  add keeps validation) were deleted as covered by asm tests that only run under
  `test_slow`; not restored.

## Sources

- `556c009:PLAN_COMPILE_TIME.md` §1.4, §2, §B.2, §B.3, DB1–DB7, §5
- commit `5d14cb7` ("proven: closure proofs default on for u8/i8, opt-in for
  wider integers")
- `PLAN_WITHIN.md` §1.2, §10, DW3; `PLAN.md` P6.4
- `a03de93:PLAN_TODO.md`; omp sessions 01a1a37f (2026-11-03), 01a1a64e (2026-11-04);
  `66ecab3:float-plan.md`, `a2c7f6c:PLAN.md`, `8d41b61:PLAN.md`, `ceaa744:PLAN_REFACTOR.md`
