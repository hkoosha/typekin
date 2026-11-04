# ADR 0014: `friends` and `constructor` composition

- Status: Accepted (implemented); `constructor` is `#[doc(hidden)]` and experimental
- Date: 2026-11-04
- Related: `src/friendship.rs` (`expand_with_constructor`, `resolve_scope`), `src/constructor.rs` (`Scope`, `Scope::wrap`, `ekran`), `src/lib.rs` (`friends`, `constructor`), ADR 0008 (friendship model), ADR 0016 (code-generation architecture)

## Context
The constructor (`of`) needs friendship's `Seal`/`Make` traits, and friendship emits its protocol into a scope, often an anonymous `const _`, which a separately expanded attribute cannot name. Two attribute macros on one item expand one after the other, each seeing only its own arguments. Something had to make the two compose regardless of order without exposing internal plumbing.

## Decision
- `constructor` stays its own attribute. It is not merged into `friendship`/`friends`. The **first typekin attribute to receive the tokens handles every typekin attribute on the item and strips them**. Siblings are matched by their last path segment. `friends` also works on its own.
- Composition goes through crate-private Rust calls, not generated attributes. If `constructor` is outermost, it parses the nested friendship config and calls `friendship::expand_with_constructor`. If `friends` is outermost, it builds the constructor config itself, so **`friends` generates `of` on its own** and accepts `scope` without exposing constructor options.
- Scope is explicit: `scope = self | _ | [visibility] module_name`. `self` emits into the enclosing module, `_` wraps the output in `const _: () = { ... };`, and a module name emits into that module. One expansion emits the friendship traits, conversions, constructor, relation conversion and `Make` impls, then wraps them once with the resolved scope. A friendship `mod` together with a non-`self` constructor scope is an error: "`constructor mod` conflicts with the friendship protocol module".
- `constructor` has no `capability =` argument. The capability is always `Make` (briefly named `Constructor`), and it is added if missing. The relation type implements `Seal` + `Make` as itself by returning `*self`, so it must be `Copy`. Why: direct construction from the relation should work without any friend.
- The relation argument is a full path and may include `Self::foo`. A version that forced an implicit `Self::` prefix was reverted: "accept a path, including Self::foo... don't assume foo is on Self". The argument was renamed `constructor` → `from_relation` → `of_relation`. Today the public key is `relationship = PATH` (internally `of_relation`), with `maker = Ident` (default `of`) and `friend = Type | [Type, ...]`.
- Each capability trait declares its own conversion and `Seal` is only a marker, so `of` calls `Make`'s conversion explicitly. Why: merging capabilities per source once let a later declaration overwrite the conversion used for `Make`.

## Rejected alternatives
- **Merge the constructor into friendship** (`friendship(constructor = ...)`). It was chosen because "a separate attribute macro cannot access another macro's anonymous-const traits". Reverted by the user: "I didn't say merge macros... the first macro who is handed the token stream should already handle all mentions of typekin macros, and it should remove them".
- **Forwarding attribute plus a public `typekin::__constructor` macro.** Generated attributes expand in the consumer crate, so the forwarding macro had to be publicly exported. `#[doc(hidden)]` hides it from docs but does not make it private. Rejected because it added an unacceptable `__` pseudo-API, and because it relied on generated attributes that `expandem` did not yet expand recursively.
- **Private generated protocol module** (e.g. `__typekin_document_friendship`) that the constructor could name. This hides only the trait *names*. `impl Make for Reader {}` is globally coherent and takes part in trait resolution across the crate wherever it is written, so the module does not contain the protocol's behavior. Explicitly called wrong.
- **Implicit `Self::` prefix on the relation path.** Reverted; see Decision.

## Consequences
- No proc-macro forwarding protocol is exposed, and the protocol's declarations and impls always share one scope.
- `constructor` remains `#[doc(hidden)] // Experimental AND unfinished!`. No reason for hiding it was recorded. The `doc(hidden)` rationale in the sessions applied only to the rejected `__constructor` macro.
- Supporting tooling: the forwarding experiment led to `expandem` recursively expanding attributes produced by an earlier selected expansion, with `--max-expansion-depth` (default 128) as a guard against self-reproducing macros. The final design no longer depends on it.
- `friendship` became the `friends` macro (commit e48d253 "do not expose contructor through friends", 5d21fb1).

## Sources
- omp session 01a10215 (2026-10-03) (forwarding macro, private module, scoped composition, expandem, doc-hidden)
- omp sessions 01a0be5d, 01a0bdfa (2026-09-20) (separate attribute, syntax, hardcoded `Make`, argument renames)
- omp sessions 01a10666 → 01a10857 (2026-10-04) (`Seal` marker, `friends` generates `of`)
- commits e48d253, 5d21fb1
