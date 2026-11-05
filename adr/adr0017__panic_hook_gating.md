# ADR 0017: Panic-hook wrapper gating

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/zz_runner.rs` (`TYPEKIN_MACHINERY_DEBUG`,
  `typekin_catch_panics`, `catching`), `src/lib.rs` (`expand`,
  `expand_hideable`), `../src/unit`, AGENTS.md:452 ("Machinery and fixtures are
  unconditional"), ADR 0016, ADR 0020

## Context

Every macro entry point ran its expansion through `catching`. `catching`
installs a panic hook, runs the expansion under `catch_unwind`, captures the
message and a backtrace, and emits `compile_error!`. The request was to keep
this wrapper but switch it off by default, behind something hidden.

## Decision

```rust
// Can be enabled on demand, but MUST NOT be ever checked into the code,
// released, or published as enabled.
const TYPEKIN_MACHINERY_DEBUG: bool = false;
pub(crate) fn typekin_catch_panics() -> bool { return TYPEKIN_MACHINERY_DEBUG; }
```

- `catching` has a single signature and picks a branch with
  `match typekin_catch_panics()`. Off: it runs the closure and turns
  `syn::Error` into `compile_error!`. On: the panic-catching path.
- Both branches are compiled and type-checked in every build. The tests have no
  cfg gates (`t_runner.rs`).
- The user's reasons: keep the switch out of Cargo, no build script, and keep
  the machinery and tests unconditional with one switch point.
- At the end of the session the switch was `cfg!(typekin_hidden)` inside
  `typekin_catch_panics()`. Someone later replaced it with the hard-coded const.
  No reason recorded. [INFERENCE] The const drops the `unexpected_cfgs`
  allowance and the `RUSTFLAGS` traps (enabling the cfg rebuilt every
  dependency; `justfile` exports its own `RUSTFLAGS`; with `--target`,
  `RUSTFLAGS` doesn't reach proc macros).

## Rejected alternatives

- **Cargo feature `hidden`** (built). Rejected by "can the feature be hidden
  from cargo as well?" [INFERENCE] Features show up in `cargo metadata` and on
  docs.rs, and they unify across the dependency graph, so any dependent could
  turn it on for everyone. `nightly` stays the only feature. The built version
  also had two copies of `catching`, and the default one dropped the
  `UnwindSafe` bound.
- **`build.rs` emitting a cfg.** Vetoed before it was written: "no build.rs
  allowed btw". No reason given. The crate has no `build.rs`.
- **Crate/module-wide `#[cfg(typekin_hidden)]`** on imports, both `catching`
  copies and the tests (built). Rejected: "Don't use the feature globally. Keep
  the tests and machinery, only call the wrapper of some internal function
  returns true."
- Never considered: declaring the cfg through
  `[lints.rust] unexpected_cfgs = { check-cfg = [...] }` in Cargo.toml.

## Consequences

- Off by default. No reason recorded. [INFERENCE] The panic hook is global to
  the whole compiler process, and `take_hook()` restores the default hook rather
  than the previous one. rustc already reports proc-macro panics, so the wrapper
  is mainly a backtrace aid for typekin developers. `arena/judge` later called a
  global hook inside the compiler "a hacky pattern".
- By default, a panic inside typekin reaches rustc as a plain proc-macro panic,
  without typekin's `compile_error!` or backtrace. This change is undocumented.
- Every entry point must be `UnwindSafe` even when catching is off (`lib.rs`
  `expand`/`expand_hideable` bounds).
- Turning it on means editing the source. It must never be committed as `true`.
- Config and item parsing run before `catching`, so panics during parsing are
  never caught (PLAN.md: "Parse errors stay outside `catching`").
- Calling `catching` from unit tests requires `proc_macro2` tokens. `catching`
  now returns `proc_macro2::TokenStream`, and `lib.rs` converts the result.

## Sources

- omp session 01a179b2 (2026-10-26)
- `src/zz_runner.rs:16-22` (current const)
