# ADR 0005: `hide_in`: types live in a private module by default

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/zz_hide.rs`, `src/integral.rs` (conflict errors), README "Hiding (`hide_in`)", AGENTS.md `hide_in` rule; ADR 0004 (float totality), ADR 0008 (friendship), ADR 0014 (`friends`/`constructor`)

## Context
A private value field is still reachable from the defining module: it can call the
tuple constructor `Port(0)` or write `.0` and skip validation. Earlier, typekin sealed
only its own generated traits and impls in an anonymous `const _: () = { … }` scope
("so they cannot be manually implemented"). Later, `friends` gained an optional named
module (`mod = [vis] name`, today also `scope = …`), where user paths were rewritten
to `super::X`. Neither of these protected the type's own field.

## Decision
- On by default for `integral` (and its profile entry points), `text`, and the
  `bitflag` root. The item and all generated code move into `#[doc(hidden)] mod
  __snake_name`, holding the item, then `use super::*;`, then the generated code.
  `$vis use` re-exports the type (for bitflag: the enum and the value type).
  The user decided this (D19); hiding is on by default and no other reason was recorded.
- `hide_in = name` sets the module name. `hide_in = none` turns hiding off and gives
  exactly the unhidden expansion. A module named `none` is written `r#none`.
- The item's visibility moves one level deeper (private → `pub(super)`,
  `pub(super)` → `pub(in super::super)`, `pub`/`pub(crate)` unchanged), so privacy
  checks see the same nominal visibility as without hiding.
- `use super::*` resolves relative names. Paths starting with `self::` or `super::`
  gain one level, in both the configuration and the item. `crate::` and `::` paths
  are not changed.
- String literals are not rewritten, because typekin "cannot know which strings are
  paths". A non-doc attribute string starting with `"super::…"` is an error, because
  it would silently resolve one module lower.
- While hidden, these are targeted errors: `get_raw = path`, and a user-written
  `_or_panic` (`without = fn_or_panic`). Both need field access, and both errors name
  `hide_in = none` as the fix.
- `_blind_trust` is `pub(super)` when hidden: the one explicit way in for the
  defining module.

## Rejected alternatives
- **Item `pub` inside the module (the embedded-tree shape).** Tried first. It is a
  hard E0446 ("private type in public interface") for private `produce` outputs and
  other private types in associated types, e.g.
  `impl Add for OutputSource { type Output = OperatorOutput; }`. The visibility is
  re-anchored instead.
- **Per-path `super::` prefixing (`friendship::parent_path`).** It "cannot reach
  bounds, `where` clauses or attribute tokens". `use super::*` covers them all.
- **Callback bridges for fn-local declarations** (trait impls at the call site, which
  can name fn-local items). The struct's own bounds and the friend types would still
  have to be named inside the module. Also, "every bridge callable from outside the
  module is also callable by the defining module". Fn-local declarations are
  therefore unsupported with hiding on. A module inside a function cannot reach the
  function's items (`use super::*` gives E0425; `super::super` gives E0433). A macro
  cannot tell it is inside a function, so the error comes from rustc, not typekin.
- **`hide_in` on `friends`, `tree`, `constructor`.** These have "no storage to
  seal", and `friends` needs a user-written `of_parts` that constructs `Self`.
- **String form `hide_in = "x"` / `""`.** "There is one grammar (identifier; `none`
  reserved), not a string form next to a keyword." Any string is rejected with
  `hide_in takes a module name or none`.
- **Rewriting paths inside string literals.** Not possible to do reliably (see above).
  The `"super::…"` error is used instead.

## Consequences
- Hiding is a privacy boundary. It does not license optimizer assumptions: in-module
  code can still build an invalid value safely, which is why assume hints stay
  rejected (ADR 0004, ADR 0003).
- The change broke code that used `.0` or the tuple constructor (E0423); `raw()`,
  `make`, `try_make` and `of` replace them. Doctests without an explicit `fn main`
  are wrapped by rustdoc into a function and hit the fn-local limit.
- A `pub` field stays constructible. That is the caller's choice.
- Embedded `tree` output stays beside the module, and the module sees it through
  `use super::*`.

## Sources
- `git show 66ecab3:float-plan.md`: §3.4, D19, §12 (rejected list), §2.8 (`parent_path`), risk 19.
- `prev.md` at `6a9e880`: the user's decision (D19).
- Early `const _` sealing and `mod =`: omp session 01a0be5d (2026-09-20).
- `66ecab3:float-plan.md`, `8d41b61:PLAN.md`, `ceaa744:PLAN_REFACTOR.md`, `bfd34d0:crates/typekin/README.md`;
  `6a9e880:float-plan.md` §3.4 (D19), §12.
