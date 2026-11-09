# ADR 0024: `Within`: design decisions

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/integral_within.rs`, `src/zz_domain_within.rs`,
  `src/zz_domain_within_const.rs`, `PLAN_WITHIN.md`, ADR 0006 (domain), ADR 0008
  (friendship), ADR 0018 (`without`), ADR 0019 (asm), ADR 0023 (packed uses it)

## Context

Converting wrapper `S` into wrapper `T` when every value of `S` is valid in `T`
used to need an `unsafe` friend plus documentation. `Within` proves the subset
at compile time, so the conversion has no branch and no panic path.

## Decision

- `Within` is a friend capability: `friend = [Within(Source)]`. `T::of(s)`
  converts through `raw as T`. Only `Make` routing changes; there is no `Within`
  trait (§5.2).
- Each wrapper publishes hidden versioned consts `__TYPEKIN_WITHIN_V1_*`
  describing its domain. The target emits a const check that rustc evaluates.
  `without = within_source` stops publication (`IntegralFlag::WithinSource`).
- Proof tiers: an exhaustive table for `u8`/`i8` sources (used even when a form
  exists, DW5); otherwise a form analysis with caps `MAX_MODULUS = 1024`,
  `MAX_EVALUATIONS = 65_536`, `MAX_PIECES = 256`. Beyond the caps the domain is
  `Opaque` with a reason. The const walk budget is
  `__TYPEKIN_WITHIN_BUDGET = 65_536` steps.
- An exact evaluator handles every lens, value and width; `contains` returns
  `bool` (W1).
- `ResolvedDomain::values(width) -> Result<Form, Opaque>` is the exact
  per-width value set (ADR 0033, 2026-11-09): the form is no longer private
  to `Within`, and widths are a typed `Width` with `IntTy` bounds shared with
  closure proofs. The emitted subset check (`zz_domain_within_const.rs`) is
  untouched and knows none of these types.
- One plain stable `const fn` subset check over sign-magnitude values. Typekin
  compiles it in for primitive sources and emits it verbatim for rustc.
- Primitive sources are decided at expansion (the macro sees both sides). Errors
  point at the friend span, and no const is emitted.
- The proven constructor skips validation but uses the safe storage path. It
  keeps `debug_assert!(…, "typekin within proof violated")`.

## Rejected alternatives

- **Read the source's domain in the macro, or copy it into the target config.**
  A proc macro "cannot see other items; a copy of the source domain in the
  target's configuration would drift."
- **`unsafe` friend + docs.** "That is the status quo the feature replaces."
- **Runtime check with a `#[cold]` panic.** "Still a branch and a panic path;
  the asm fixtures would stay `DiffEach`."
- **Exhaustive tables for 16-bit+ sources (DW1).** "2²⁰ already trips
  `long_running_const_eval`". 2¹⁶ is cheap (0.22 s) but excluded "to keep
  compile cost flat".
- **Form proof when a table is available (DW5).** The table is "exact by
  construction, independent of the analyzer".
- **Lenses modeled only at zero (old `lens_value`).** Replaced by the exact
  evaluator, which is the oracle for forms and tests.
- **Unbounded forms.** Caps bound the target loop, expansion time and constant
  size, and stay "well below the `long_running_const_eval` threshold".
- **Proving through `valid` callbacks, `count_ones`/`count_zeros`,
  `isqrt`∘`mul(d)`.** These stay opaque, because "callbacks are paths, not
  bodies". The only way through is the 8-bit table when the callback is
  const-callable.
- **Two subset checks (proc-macro over `Int` + emitted tokens), pinned by
  differential tests.** Planned in §5.3. The implementation replaced it with the
  single `const fn` (reversal noted in `PLAN_WITHIN.md` "Where the
  implementation departs").
- **Runtime helper crate.** "Every helper the const proof needs is emitted at
  the target site."
- **`new_unchecked` after the proof (DW3).** "A proof bug cannot become UB."
- **No check on proven paths (DW4).** `debug_assert!` stays as a "safety net,
  free in release".
- **A new configuration key (DW2).** As a capability it "reuses friend parsing,
  `of`, `Make` routing". The name `Within` reads as "source is within target"
  and avoids `core::ops::Sub`.
- **Generated `From` (DW7).** "Friendship is not `From`" (`PHILOSOPHY.md`).
  Users can write a one-line `From` that calls `of`.
- **Generated narrowing `TryFrom` the other way.** "Expected rejection belongs
  to user-written `TryFrom`."
- **User conversion function.** Rejected: "`Within` converts through the
  source's raw value". The cast is lossless because the proof includes the
  target's raw range.
- **Floats / `point` over float (DW8).** "Float rounding and NaN need a separate
  design."
- **`Within` in facets, on bitflag root friends, or `_(Self)`.** Rejected with
  diagnostics. Facet friends are only `Math`/`Bit`. On bitflags it "belongs in
  the nested `integral = [...]`". `_(Self)` fails with "a type is always within
  itself; remove this friend".
- **A capability trait for `Within`.** None. It only alters `Make` routing.
- **Unversioned or mandatory publication (DW6).** With versioning, a
  "cross-crate mismatch fails loudly". The opt-out exists because "wrappers can
  refuse to publish their raw value".
- **`[Within, unsafe]` together.** Rejected: "`Within` proves what `unsafe`
  trusts; use one."

## Consequences

- 16-bit+ sources with callback or very irregular domains cannot be proven
  (`Opaque`). Users fall back to `unsafe` or checked conversion.
- The implementation departs from the plan: flat modules instead of
  `src/zz_domain/within.rs`; the proof is forced in the friend's seal conversion
  (shared by `Make`, `Rel`, `Math`), not only in `make`; fixture-crate tests
  (`fx_*.rs`).

## Sources

- `PLAN_WITHIN.md` (a1c61ba; history a626315 → ae4a235 → 155f6e6
  `within-plan.md`, a03de93 → 556c009 → 13703fb)
- `PLAN_WITHIN.md` §10
