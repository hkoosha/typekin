# ADR 0004: Float totality and float semantics

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/float.rs`, `src/integral_impls.rs` (Ord/Hash),
  `src/integral_items.rs` (`float_cmp`, conversions), `src/integral_cfg.rs`,
  `src/integral_friend.rs`; ADR 0003 (proofs), ADR 0007 (PartialOrd methods,
  Eq), ADR 0008 (friendship), ADR 0011 (profiles), ADR 0015 (konst); README
  float section, PHILOSOPHY.md, AGENTS.md

## Context

Float newtypes reuse the integral generator, but `f32`/`f64` have NaN, two
zeros, infinities and rounding. Each integral habit (`Ord` via `unwrap`, `as`
round trips, bit ops, `NonZero`) had to be checked against IEEE semantics and
zero-cost asm. This ADR records those per-feature decisions; the entry point and
the `math` provider are out of scope.

## Decision

- **Totality only for NaN-free domains with a private field.**
  `Eq`/`Ord`/`Hash`/`_cmp` exist only when the domain proves NaN is rejected
  *and* the value field is private. Requesting them otherwise is an error
  (`"{name} requires a NaN-free float type"`,
  `"{name} on a float type requires a private value field"`,
  `src/integral_cfg.rs`), not a silent skip. PHILOSOPHY: "a diagnostic, not a
  silently weaker `Ord` that panics or lies".
- **`Ord::cmp` is branchy:**
  `if l < r {Less} else if l > r {Greater} else {Equal}` (`float_cmp`). 7/4/10
  insns (x86_64/aarch64/armv7, O3), no panic path, const on every konst mode.
  Integers keep `partial_cmp().unwrap()`.
- **No assume hints in `raw()`** (`assert_unchecked` etc.); none emitted.
- **Hash** = `(raw + 0.0).to_bits()` so `-0.0` and `+0.0` hash alike (4 insns,
  x86_64/aarch64). The `+ 0.0` is dropped when the domain admits at most one
  zero.
- **No inherent `min`/`max`/`clamp` on `Ord` types**: `Ord`'s versions select
  valid values without validation. Types without `Ord` get inherent forwards
  that validate (`CORE_FLOAT_SELECTIONS`). Note `Ord::max(+0.0, -0.0)` follows
  `cmp` (equal → returns the second argument), not IEEE `max`.
- **`f64 → f32` narrowing keeps NaN and ±inf (D3):**
  `t as f64 == r || r.is_nan()`; a finite value must round-trip.
- **Float → int `try_into_*`:** `as` round trip alone when the target's `MAX` is
  exact in the float; otherwise round trip plus `r < 2^bits` with folded exact
  bounds (`f32::INFINITY` stands in for 2^128). Per-pointer-width variants for
  `usize`/`isize`. Rejects rounding, NaN, inf, out of range; `-0.0` is ok.
- **Lossy primitive friends (D6):** `Make` friends convert with `as` and may
  round; `Rel`/`Math` primitive friends must convert exactly
  (`"… is lossy; float Rel and Math friends must convert exactly"`).
- **`Sum`/`Product`:** fold raw from `-0.0`/`1.0` exactly as `core` does,
  validate the result once. On by default (D4). Empty sum is `-0.0`, so it
  panics when the domain excludes `-0.0`.
- **`is_*` query folding:** each query depends only on sign × category (10
  classes). If every class the domain admits answers alike, the forward becomes
  a `const` `true`/`false` with a doc line. Exact for NaN/±inf/±0, conservative
  for subnormal/normal. Only with a private field (`Cfg::visible_value` disables
  it, so `ExposedUnit(f64::NAN).is_nan()` stays true); `valid` callbacks don't
  block it since they only narrow. `classify()` is not folded.
- **Consts:** only format metadata is forwarded (`RADIX`, `MANTISSA_DIGITS`,
  `DIGITS`, `MIN/MAX_EXP`, `MIN/MAX_10_EXP`, typed `u32`/`i32`,
  `CORE_FLOAT_CONSTS`). These describe the storage format, so no domain can
  invalidate them. `Self`-typed consts (`MAX`, `NAN`, `EPSILON`, `PI`) are not
  forwarded.
- **Non-goals:** no bit ops, `non_zero` storage, `Step`/`range`, or
  circular/delta/sequential/wrapping profiles; each is a targeted error.

## Rejected alternatives

- **`Eq`/`Ord`/`Hash` on NaN-able types via `total_cmp`/bits (ordered-float
  style).** "Semantically surprising, diverges from `PartialEq`."
- **`cmp` via `partial_cmp().unwrap()`.** 15/14 insns plus a panic path.
- **`cmp` via `total_cmp`.** 15/10 insns and wrong: `-0.0 < +0.0` contradicts
  `Eq`.
- **`cmp` via `unreachable_unchecked` / assume hints in `raw()`.** Same speed as
  branchy but unsound: in-module code can construct `Unit(f64::NAN)` safely. The
  same idea was rejected again for proofs (ADR 0003).
- **Hashing raw bits.** Breaks `Hash`/`Eq` agreement on ±0.
- **Inherent NaN-propagating `min`/`max`/`clamp` on NaN-free types.** They would
  shadow `Ord`'s faster ones: x86 `max` 2 vs 9 insns, `min` 3 vs 10, `clamp` 12
  vs 20.
- **`Err` on NaN in `try_into_f32`.** Plan recommended `Ok`; no further reason
  recorded.
- **Integral's plain `as` round trip for float→int.** Accepts `2^63` (saturates
  to `i64::MAX`), verified.
- **Bounds checked first (`r >= LO && r < HI` before the round trip).** The
  plan's own first form; measured 1–2 extra insns at O1/Os/Oz.
- **Unfolded `-(MIN as F)` bound.** Changed LLVM's instruction order at O2/O3.
- **Lossy `Rel`/`Math` friends.** `src/integral_friend.rs:399-400`: "comparisons
  and arithmetic must not silently round, so `Rel` and `Math` need an exact
  source."
- **Per-element validation in `Sum`/`Product`.** "Slower, no semantic gain."
  Unvalidated fold is identical to raw `iter().sum::<f64>()`.
- **`Sum`/`Product` off by default.** Plan recommended on (D4); no further
  reason recorded.
- **Forwarding `Self`-typed consts.** A domain can make them invalid (e.g.
  `NAN`, `MAX`).
- **Folding queries on `pub`-field types.** Users can store any value, so the
  answer would lie.

## Consequences

- `Ord` and `Hash` cost nothing beyond raw comparisons; NaN-able types simply
  lack them.
- Below O2 the `#[inline(always)]` layer alone changes instruction selection for
  checked conversions, so asm tests compare O1/Os/Oz against an inline-layer
  control, O2/O3 against raw (`typekin_testing/src/asm_case_my_i32.rs`).
- Integral↔float newtype friends fail with rustc's missing-`Into` error; the
  macro cannot see a friend's kind, so no targeted error.
- README's "Checked conversions reject … NaN" applies to float→int, not to
  `try_into_f32`.
- Code comments cite `(D3)` from the deleted plan; this ADR is where that now
  lives.
- Open: fold `classify()`; doc line that forwarded consts are "of the wrapped
  primitive float, independent of `domain`".

## Sources

- `git show 66ecab3:float-plan.md` §2.4, §2.5, §4.1, §4.5, §4.8, §12, §13 (D3,
  D4, D6, D7), P4/P7 status
- `6a9e880:prev.md` (Copilot session transcript, 2026-10-31); omp session
  01a1a328 (2026-11-03) (const forwarding, query folding)
- `66ecab3:float-plan.md`, `8d41b61:PLAN.md`, `8d41b61:PHILOSOPHY.md`,
  `bfd34d0:crates/typekin/README.md`
- `8f83c7f:PHILOSOPHY.md`
