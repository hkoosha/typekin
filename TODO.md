# TODO

Loose ends found while reconstructing design history (see `adr/`). Each item
says what is missing, not how to fix it.

- **Prelude shadowing in generated code.** Bitflag emits bare `Option`,
  `Result`, and `Iterator` (`src/bitflag.rs`, e.g. `from_bits`,
  `try_as_known_bits_only`, `iter*`), so a caller type with one of those names
  breaks or changes the expansion. Other families are unaudited. A fix and a
  `test_shadowed_prelude.rs` test exist only in `git stash list`
  (`stash@{0..2}`, based on an old tree); never landed.
- **Fast-suite proof coverage.** Three `t_proof` tests were deleted when tests
  moved to `typekin_testing` (proven division, proven core-int method, unproven
  add keeps validation). Only the `test_slow` assembly tests cover them now.
- **Bitflag `Neg`.** Since P1.1 a signed bitflag value type gets `Neg` by
  default; `PLAN.md` §7 says to reject `Neg` on bitflag values and ship both
  together. Not done.
- **`konst` default vs Cargo feature unification.** The `nightly` feature flips
  the default `konst` for every crate in the build, which seems to contradict
  PHILOSOPHY.md ("enabling an unrelated feature must not silently require a new
  nightly gate"). Needs a decision (ADR 0015).
- **Dangling decision labels.** Code comments cite `(D3)`, `(D12)`, `(D13)`,
  `(D16)`, `(D17)` from plans that were never committed; point them at ADR 0004
  (D3), ADR 0006 (D12, D13, D17) and ADR 0015 (D16).
- **`#[typekin::constructor]`** is still `#[doc(hidden)]` "experimental and
  unfinished"; finish or remove.
- **`PLAN_WITHIN.md`** paths and checkboxes are stale (README section and
  `my_within` exist).
- **Stale notes.** `arena/DEDUP.md` §3 still says the lossless-cast tables were
  kept (code now uses range inclusion per pointer width) and its file paths are
  outdated.
- **Test recipes.** `just test-all` is an alias of `test-slow`, so it runs
  neither the cross-target nor the wasm32 features.
- **AArch64 opt-level z per-case crates** (`outlines()` in
  `typekin_testing/src/machinery/asm.rs`): kept when z was opt-in; z is now part
  of `test_slow`. Confirm it is still wanted.
- **`test_hide`** may no longer exercise a private type hidden by `hide_in`
  after the fixture move (flagged, unverified).
- **`with_const_map_fn`** survives beside `konst_map` only for compatibility
  (kept when `konst = nightly` started covering `map`), against the
  no-compatibility rule in AGENTS.md.
- **`packed` opaque-source message.** When a packed record is a field in a
  window narrower than its `repr`, or a `Within` source, the "cannot prove"
  reason names the internal callback
  (`` `Inner`'s `valid = __typekin_packed_valid` is opaque to subdomain proofs ``).
- **Optimization: unchecked `NonZero` construction.** Generated code calls no
  `unsafe`, so storing into `NonZero` always goes through `NonZeroX::new(raw)`
  and panics on zero (`StoragePlan::storage_expression`, `src/integral.rs`).
  Validated construction (`try_make`, operators, `_proven`) used
  `new_unchecked`. LLVM drops the new check only where it can derive `raw != 0`
  from the inlined validation: `Code64` in `src/e2e/asm_case_domain_int.rs`
  (`domain = (-5..12 | 87..) & !{10} & !mul(3)` on `i64`) keeps a `test`/`je`
  and a panic block, so its exact comparison against a hand-written probe was
  dropped from `DOMAIN_INT_OPERATIONS`. `_proven` results have no runtime check
  at all and keep it whenever the operands' ranges don't show `raw != 0`.
  Options: route zero into the existing rejection (`Err`, `_or_panic`) instead
  of a separate panic, or `new_unchecked` behind an opt-in for generated
  `unsafe`; measure with the `asm_case_*` probes.
- **Optimization: `AsRef<raw>` on `NonZero` storage.** Borrowing the primitive
  out of `NonZero` (or `Wrapping<NonZero>`) took an `unsafe` pointer cast, so
  `Cfg::settle_as_ref` now turns `AsRef<raw>` off for that storage and rejects
  an explicit `with = as_ref_`; `Step::steps_between` borrows a copy instead
  (`&start.0.get()`). `impl From<&NonZero<T>> for &T` (`more_from_nonzero`,
  marked stable in nightly 1.101) can restore `AsRef` without `unsafe` once
  `rust-version` reaches the release that ships it.
- swap max with 0, zero it fits on Option
