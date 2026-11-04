# ADR 0020: Test architecture

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `typekin_testing/` (`test_*.rs`, `fx_*.rs`, `src/machinery/`), `../src/unit`, `typekin_testing/Cargo.toml`, `typekin_testing/.config/nextest.toml`, root `justfile` (`z-test`), AGENTS.md (test rules), TODO.md; ADR 0019

## Context
Tests were spread across the root crate, per-test fixture crates and examples. Slow tests built a crate each, some passed on any failure or by skipping, and failures did not say what broke.

## Decision
- **Test at the layer where the behavior is observable.** Default: `typekin_testing`, as a downstream consumer. `../src/unit` keeps only diagnostics, crate-private analysis (closure proofs, domain normalization, float query analysis), token details and non-host pointer widths. Separate fixture crates only for what needs rustc to reject code, crate attributes, or another target.
- **Absence via macros, not crates:** `assert_not_impl!`, `assert_no_method!`, `assert_no_associated_fn!` (`machinery/absent.rs`). About 45 slow tests became fast. Changes to `absent.rs`/`fixture.rs` need a negative control.
- **Rejection fixtures match their own message:** a rejection passes only if rustc reports an error inside that case's code and every such error contains the expected text (mapped via `--message-format=json`). Compile-pass and rejection cases never share a crate; remaining fixtures batched into shared crates (~42 s → 8.6 s).
- **Never pass by skipping.** A missing target or tool fails the test.
- **A feature per environment requirement:** `test_slow_aarch64`, `test_slow_armv7`, `test_slow_i686` (umbrella `test_slow_cross`), `test_slow_wasm32`, all outside `test_slow`. `test_slow` = `test_fast` + `test_slow_only` + `test_slow_z`.
- **Fixtures and machinery always compile;** only tests are gated, and only whole modules (`mod opt_level_z`, `mod pointer_width_64`), never single `#[test]` fns. `lib.rs` stays attribute-free; `cfg`s live on modules in the test files, `#[global_allocator]` in `machinery/alloc.rs`. Helpers live in `src/machinery/` (exception to the flat-module rule).
- **`test_<name>.rs` holds tests, `fx_<name>.rs` the shared fixtures.** Validated fields stay private; fixtures provide helper fns instead of widening visibility.
- **No table-driven tests; one reason to fail per test.** Every case is a named `#[test]`; helpers allowed, loops over cases not. Not a single-assertion rule.
- **Dedup by code path:** rejections reached through the same shared parser path are duplicates regardless of surface syntax; one representative kept.
- **Fixture unification:** merge only semantically identical fixtures. Look-alikes stay when type identity or separate compilation is the point (`My32` vs the asm fixture type, `RootBeforeNested`/`NestedBeforeRoot`, friendship `DefaultSource`/`CheckedSource`/`TrustSource`, one diagnostic per compile-fail crate, konst=nightly vs default types).
- **nextest optional:** `z-test` uses it if installed; `.config/nextest.toml` puts fixture tests in a `fixture-crates` group with `max-threads = 1`.
- **Each fixture crate has its own target dir** under `target/typekin-fixtures/`.
- **Examples are not tests;** tests never depend on `examples/` (`my_tcp` replaced by test-only `Mark`/`Gap<L: Lane>`).

## Rejected alternatives
- **Separate crates for "type lacks method/impl".** Initially believed impossible without a crate; then found expressible at compile time.
- **Rejection tests that accept any build failure.** 18 tests passed on any failure; tightening immediately caught an extra non-const error in a new fixture.
- **Early return when wasm32 is missing.** Hid broken fixtures (private tuple construction under `hide_in`, non-const calls in `const` items).
- **Cross targets inside `test_slow`.** A `test_slow` run failed with "can't find crate for `std`" without the rustup target.
- **Gating fixtures on features.** "Compilation of test fixtures is quite fast"; gate only if enabling would need machinery the env lacks.
- **`cfg`s centralised in `lib.rs`.** User wanted `lib.rs` "attribute free".
- **Widening visibility for tests.** `Grade` was made `pub(crate)` and reverted: the test must exercise a *private* type, so it goes through `grade_by_path()`.
- **Table-driven tests for a shared contract.** User: "they HAVE to be individual tests, ALWAYS". Titles must say what broke.
- **Merging `ManualIntegral` into `ManuallyDerivedIntegral`.** User renamed it `IntegralWithCallerProvidedRepr` instead.
- **nextest as the runner.** ~90 s vs ~10 s with `cargo test`: each process repeats the `LazyLock` fixture builds. A group per fixture crate to regain parallelism was not done.
- **One shared fixture build directory.** Less disk (fixtures use ~407 MB) but builds would queue; user never answered, kept per-crate.
- **Mixing asm checks with other fixtures.** User: "They are just too important" (ADR 0019).

## Consequences
- Three fast proof tests (proven division, proven core-int method, unproven add keeps validation) were deleted in the move; only `test_slow` asm covers them. Tracked in TODO.md.
- `test-all` aliases `test-slow`, so it runs neither cross nor wasm32 tests; flagged, unanswered.
- `test_hide` may no longer cover a private type hidden by `hide_in` [INFERENCE, unverified].

## Sources
- omp session 01a1a5e0 (2026-11-04) (fx_/test_ split, gating, cross features, nextest)
- omp session 01a1a37f (2026-11-03) (layer move, deleted proof tests)
- omp session 01a1a328 (2026-11-03) (absence macros, rejection matching, no skipping, build dirs)
- omp sessions 01a14915 (2026-10-17), 01a152d3 (2026-10-19), 01a17312 (2026-10-25)
