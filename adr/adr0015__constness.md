# ADR 0015: Constness (`konst`)

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/zz_lib.rs` (`Konst`, `Konsts::is_stable`), `src/zz_flags.rs`
  (`konst_for`), `src/integral_cfg.rs` (`helper_konst`, `embed_konst`),
  `src/integral_forward.rs`, `Cargo.toml`, `TODO.md`, PHILOSOPHY.md "Constness
  Is Public API", ADR 0002, ADR 0009, ADR 0021

## Context

Generated items can be `const`, but what may be `const` depends on the
toolchain: some operations are stable-const, others need nightly const traits
(`[const] FnOnce`, `Destruct`, `funnel_shifts`). Callers also differ in what
they need: a non-const validator, a nightly-only crate, or a stable library.
Constness is public API, so typekin must not change it behind the caller's back.

## Decision

- `konst = none | stable | nightly`. Omitted means `stable`. With the crate's
  `nightly` Cargo feature, omitted means `nightly`. An explicit
  `konst = nightly` works without the feature.
- Three levels: the master `konst`, then families (`konst_math`, `konst_bit`,
  …), then operations (`konst_add`, `konst_map`, …). Each level defaults to the
  one above. Explicit `with`/`without = [konst_*]` wins over the master, e.g.
  `konst = none, with = [konst_math], without = [konst_add]`.
- What `stable` turns on is decided in one place: `Konsts::is_stable` (today
  `CoreInt`, `CoreFloat`, `TryMake`, `Value`, `TextAccess`). Single operations
  can still need a stricter gate. `funnel_shl`/`funnel_shr` inside `CoreInt`
  require `Konst::Nightly` (`Gate::NightlyKonst`).
- Typekin never drops `const` because of what validation calls, and adds no "not
  const" error of its own. It emits what was configured, and rustc reports the
  non-const call.
- Validation helpers are `const` only when a const item uses them, or when there
  is no validation (`helper_konst`, D16).
- `text` and `bitflag` use the same tri-state. Bitflag passes its `Konst` and
  every resolved `konst_*` on to its nested integral. `integral = [konst = …]`
  inside a bitflag is rejected, and conflicting forwarded flags get a diagnostic
  at the flag's span ("not compatible with bitflags' own konst config").
- `map`/`try_map` follow `konst_map` (nightly-only, because a const `map` needs
  `[const] FnOnce` + `Destruct`). The older `with_const_map_fn` flag is still
  accepted.
- `rust-version = "1.97"`.

## Rejected alternatives

- **`konst = true|false`.** `true` meant nightly-only, so there was no way to
  ask for stable const. "Stable and nightly capabilities must not be conflated."
- **Implicit default (`without = [konst]`, `with_const = false`) → explicit
  `konst` required.** Making it required (`5cb75a7`) was meant to keep the
  option of adding a default later "without a breaking change". That default
  later arrived as `stable` (`8deb1e7`), and `konst` became optional. No reason
  was recorded for picking `stable` as the default.
- **`text` silently defaulting to const, then `text` requiring an explicit
  `konst`.** Both were dropped when all macros moved to the same optional
  `konst`.
- **`konst = false` disabling everything.** Rejected in favor of "master is only
  the default". With `none`, individual `konst_*` flags can still opt in.
- **Treating all of `core_int` as stable.** A stable smoke test failed on the
  funnel shifts.
- **Adaptive downgrade or a const-safe subset** (drop `const` when a validator
  or `map` closure is non-const). User: "if `konst` is specified, it will so
  effect map and arbitrary validators as well. It's the library users' call,
  no?" Constness is the caller's decision, and a predictable API beats one that
  silently gets weaker.
- **Validation helpers always `const`.** That "makes `konst = none` useless for
  non-const validators".
- **Bitflag's internal integral always `nightly`.** This shortcut was reversed.
  The nested value now follows the caller's mode.
- **Stable policy repeated at each emitter / string registries `KONST_FLAGS`.**
  These were replaced by the single typed `Konsts` enum. Flags that belong to
  another macro are rejected.
- **`rust-version` 1.90 (also 1.86/1.94 considered).** "D8's 1.90 floor covered
  floats only — the default integral output (`core_int`'s `highest_one`, …)
  already fails on 1.96." The crate ships 1.97.

## Consequences

- Adding a const API means classifying it in `is_stable`, and gating any
  sub-operation that needs nightly.
- An invalid const configuration fails in rustc instead of quietly producing a
  weaker API.
- Open: the `nightly` feature changes the default for every crate in the build
  through Cargo feature unification. This seems to contradict PHILOSOPHY.md
  ("enabling an unrelated feature must not silently require a new nightly
  gate"). No reason was recorded for the feature flipping the default. Tracked
  in `TODO.md`.
- Open: `with_const_map_fn` remains next to `konst_map` only for compatibility,
  which goes against the no-compatibility rule (`TODO.md`).

## Sources

- omp session 01a148ff (2026-10-17)
- omp sessions 01a14166 (2026-10-15), 01a16f69 (2026-10-24), 01a17ef7
  (2026-10-27), 01a14f2a (2026-10-18); 01a14d75 (2026-10-18), 01a0f766
  (2026-10-01); 01a11d13 (2026-10-08)
- `66ecab3:float-plan.md` (D8, D16, §12), `8d41b61:PLAN.md` §2,
  `8d41b61:PHILOSOPHY.md`, `bfd34d0:crates/typekin/README.md`, `TODO.md`;
  commits da5904a, 5a5424d
- Commits 5cb75a7, 184d932, 8deb1e7, df99e66; `6a9e880:float-plan.md` D8, D16,
  §12
- `bfd34d0:crates/typekin/README.md` ("Const mode"), `5d21fb1:NUTYPE.md`
  (Surface table, "Const" row)
