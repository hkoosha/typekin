# ADR 0016: Code-generation architecture and conventions

- Status: Accepted (implemented), except the prelude rule (see Consequences)
- Date: 2026-11-04 (updated 2026-11-09 by ADR 0033: data rows, `Named` tables,
  the order rule)
- Related: `AGENTS.md` (Implementation conventions), `PLAN.md` (D1–D11, App. B,
  G), `src/zz_flags.rs`, `src/zz_names.rs`, `src/zz_values.rs`,
  `src/zz_friend.rs`, `TODO.md`; ADR 0007, 0018, 0025, 0033

## Context

Typekin generates a lot of code from four families (integral/float, bitflag,
text, friends/constructor). These are the structural choices about how that code
is written: what models it, which macros exist, how names, files and emitters
are organised. Several of them were tried the other way first.

## Decision

- **Direct `Maker` + `quote!`, no code-model layer.** Emitters write `quote!`
  templates. Beside them, plain data rows say what to emit (ADR 0033): the
  name tables, `OpSpec` (operators), the forward rows, `PROFILES`, and the
  site table of rejected keys. A row is a constant, never a closure.
- **One macro per family, generating by default, with opt-out flags.**
  `integral`, `bitflag` and `text` generate `repr(transparent)`, `Clone`, `Copy`
  (not text), `Debug` and `Hash` by default.
  `without = clone|copy|hash|debug|repr` turns each one off
  (the `on` column of each family's `FlagRow` table).
- **`repr(transparent)` is generated, never required.** The macros check only
  the one-field tuple shape and never inspect caller `repr` attributes. The
  generated code makes no layout assumption (safe construction and field access
  only), so transparency is a layout/FFI choice. It is on by default because
  that is the established layout contract (AGENTS: "Preserve transparent
  representation by default and honor `without = repr`").
- **Names are types.** A family of names that logic compares, matches or looks
  up is an enum with one row table (`Named::TABLE`; ADR 0033): `FnName`,
  `IntegralFlag`/`TextFlag`/`BitFlag`, `Konsts`, `Capability`, `ProfileKind`,
  `Key`, `N`/`F`, operators. A name used in only a few places (2–4 when the rule
  was introduced; AGENTS now says "one or two") is a module `const` (`SEAL`,
  `MAKER`, …). Text stays inline only for diagnostics, docs, `Debug`, tokens
  inside `quote!`, an identifier emitted in one place, Rust keyword checks, and
  tables that are themselves the single boundary. Generated names come from
  `zz_names` key tables.
- **Flags are hand-written enums over a bitset.** Each family enum implements
  `Flag` with one `FlagRow { flag, name, on }` table. `Flags<F>` is a `u128`
  (at most 128 flags per family), parsed by one `FlagSettings<F>::parse`.
- **Tokens, not re-parsed strings.** Paths and types stay as tokens/`Ident` so
  they keep their spans (`value_name: Option<Ident>`, `lowered_type`). One
  exception is kept on purpose: `FacetSlotKey.rhs: String`
  (`src/zz_profile_integral.rs`) is "a normalized identity key for overlap
  detection, never re-parsed". Retyping it as `Option<PathIdentity>` would
  change sorting (`None` sorts differently from `"Self"`).
- **Flat modules, `zz_` = machinery.** Every module is a file directly in
  `src/`, declared from the crate root. A split-off part is named with a suffix
  (`zz_domain_closure.rs`). A non-`zz_` file is a macro entry point. A shared
  `zz_*` file never imports a family; one serving a single family carries that
  family's suffix (`zz_profile_integral`).
- **Emitter shape.** An emitter is a `&self` method returning one trait/impl
  group, with its flag check as an early return. Orchestrators are section
  lists, and inside a section the order follows the tables. Only "the
  annotated item comes first" is promised about order; the rest is
  deterministic but may change between versions (ADR 0033, D0.1; until
  2026-11-09 the orchestrators' order counted as public output). Logic
  functions stay under about 100 lines.
- **Per-kind domain code.** The integer and float normalizers/emitters are
  separate (`zz_domain_int.rs`, `zz_domain_float.rs`). The files are split by
  kind, not by stage. Only identical helpers are shared (`kleene`,
  `group_in_order`, `collapse`, `listing`).
- **Three protocol emitters** stay separate: `zz_friend::emit_protocol`,
  standalone `friendship::emit_friendship`, and text's `make_friendship`.
- **Annotated item first.** Every macro emits the item before anything it
  generates. Under `hide_in`, the private module comes first and the item is its
  first entry (`t_order.rs`).
- **No compatibility or migration code** for earlier behaviour unless explicitly
  asked.
- **Parameter naming:** generated and internal method parameters are named
  `that`, not `other`. This was the user's request; no reason recorded.

## Rejected alternatives

- **Typed code model ("lang" DSL).** `lang/{def,block,generate}.rs` and
  `value_ops.rs` (`Konst`, `Inline`, `Func`, `Trait`, `Where`, `BinOp`, `Merge`
  fold), plus `Generator*`/`Impl*` objects in `arena/dumpster`. Added in
  `455cb27`, deleted in `3468e9a` "cleanup". No reason recorded. [INFERENCE] It
  cost more than direct templates; it was half-commented-out from the start and
  has a copy-paste bug (`ImplPartialOrd` emits `PartialEq`). Copies remain in
  `arena/`.
- **Full/manual macro split** (`ty_integral`, `ty_bitflag`, `ty_text` beside
  manual versions). Lost to default-on generation plus `without`. Flags compose
  with profiles, facets, konst and hiding; an all-or-nothing entry point does
  not. Two more problems: plain derives impose bounds on generics, so generic
  integrals get hand-written `Copy`/`Clone`; and a bitflag enum's numeric `repr`
  sets its bit width and "cannot be safely guessed", so even a "full" bitflag
  still needed it from the caller (it remains required input).
- **Require caller-written `#[repr(transparent)]`.** It enforced a property the
  macros don't need and rejected valid wrappers. **Never generate it:** breaks
  the default layout contract. **Parse/reconcile caller `repr`:** extra policy,
  conflicts and edge cases. An incompatible caller `repr` is still left for
  rustc to reject; use `without = repr`.
- **`mk_flags!` structs of bools** (first trimmed in `2f0ccb7`, then replaced in
  `7f20e09`). Its `parse_from`, `get`, `&str` arms and `@str` arm were dead or
  broken. "the line count breaks even; the gain is that flags become data".
  AGENTS: "Do not create macros solely to reduce repetition."
- **Enums for every name.** User: "Don't go crazy though, for a single use case…
  maybe a global const is better than an enum which pollutes the namespace."
- **Nested module directories** (`src/zz_domain/closure.rs`). Flattened, and a
  rule was added so new ones need the user's permission. `PLAN_WITHIN.md`'s
  `src/zz_domain/within.rs` became `zz_domain_within.rs`.
- **Making `zz_profile` generic by injecting the integral grammar.** It would
  not have removed the coupling: `IntegralFlags` stays imported.
  **Renaming it `integral_profile.rs`:** non-`zz_` names are macro entry points,
  so readers would look there for the profile macros. The family suffix won.
- **`float.rs` via `#[path]`** while staying private under `integral/`. It
  "would keep everything private but let the file location disagree with the
  module path". Chosen instead: widen `Ctx`, `Maker`, `IntegralTarget`,
  `impl_header`, `fresh_ident` and `closed_result` to `pub(crate)`.
- **`ctx: &Ctx` on every emitter, and `let mut stream; if flag {…}`.** There is
  one `Ctx::new`, and the argument was always `&self.ctx`.
- **Shared domain normalizer (`BoundRules`, about −190 lines) and emitter
  skeleton (about −70).** "every line it touches is a tie-break that feeds the
  zero probe and therefore `NonZero` storage. Not worth it." The emitters must
  also stay token-identical. Stage split: "Split by kind rather than by stage,
  because floats are already split that way."
- **One configurable protocol emitter.** It "would create a mode matrix over
  observable expanded code. That is more fragile than the duplication it would
  remove" (`arena/DEDUP.md`).
- **Shared serde skeleton (`zz_serde.rs`).** It failed its own acceptance test:
  it saved 54 caller lines but cost at least 66 (PLAN.md D10). See ADR 0025.
- **Other sharing refused** to keep the output identical or readable: a central
  flag-check table (a closure per entry), an `#[inline(always)]` helper ("part
  of the visible item shape"), a cross-family fmt helper (four parameters for
  two two-line impls), and merged friend conversion bodies ("Same shape,
  different tokens").
- **Migration/compat framing.** User: "There is 'no current user'. Hence no
  migration."
- **Trailing impl block on the struct**
  (`#[integral] struct Foo(u32) { fn bar(){} }` → `struct` + `impl`).
  Implemented, then reverted at the user's request. No reason recorded.

## Consequences

- **Generated code should fully qualify prelude names**
  (`::core::option::Option`, …) so it compiles when the caller shadows `core`,
  `Option`, `Result`. This is not in force. A fix and `test_shadowed_prelude.rs`
  exist only in `stash@{0..2}`, and HEAD `bitflag.rs` still emits bare
  `Option`/`Result`/`Iterator` (`TODO.md`). Other families are unaudited.
- **128-flag ceiling** per family from the `u128` bitset.
- **Cost of the emitter rules:** duplication across the three protocol emitters
  and the two domain kinds is accepted. Changes are checked with `expandem`
  diffs (trailing commas and spacing ignored).
- **Widened visibility:** `float.rs`'s access means some integral internals are
  `pub(crate)` rather than private.
- **Open naming questions:** `zz_conv.rs` serves only integral but keeps its
  plain name ("`ty_out` is a cross-family config concept"). `IntegralTarget` was
  never split out to `integral_target.rs`.
- **No compat code:** breaking syntax changes ship with no migration
  diagnostics.

## Sources

- Commits `455cb27`, `3468e9a` (lang DSL); `arena/DEDUP.md` ("Deliberate
  non-candidate", protocol emitters)
- omp sessions 01a14f36, 01a14d66 (2026-10-18); 01a14f4f (2026-10-18)
- `ceaa744:PLAN_REFACTOR.md` → `PLAN.md` (B.3–B.4, G.2, G.6, D10)
- omp session 01a1a57b (2026-11-04)
- omp session 01a1a37f (2026-11-03)
- omp session 01a1a328 (2026-11-03)
- Claude Code sessions 93ccc8e4 (2026-10-28; prelude, tokens, `rhs: String`,
  `mk_flags!`), 354a3b22 (2026-11-03; `other` → `that`)
- omp session 01a0f766 (2026-10-01; trailing impl block); omp session 01a173f8
  (2026-10-25; no migration)
- Commits `2f0ccb7`, `7f20e09`
- AGENTS.md lines 101, 302, 351–353, 381–433; TODO.md (prelude shadowing)
