# ADR 0033: One operation model for code generation

- Status: Accepted (implemented for what exists today; see "Left for
  `PLAN.md`")
- Date: 2026-11-09
- Related: `PLAN_0.md` (the plan this records), `PLAN.md` (check levels; not
  implemented when this landed), `AGENTS.md` (Implementation conventions),
  `src/zz_lib.rs` (`Named`, `parse_choice`, `Site`, `rejection`, `Removals`,
  `Validation`), `src/zz_flags.rs` (`FlagRow`), `src/zz_values.rs` (`OpSpec`,
  `Width`, `IntTy`, `IntervalSet`), `src/zz_forward.rs` (`CoreFwd`),
  `src/zz_profile_integral.rs` (`PROFILES`, `Slot`), `src/integral_cfg.rs`
  (`WrapperWide`, `OpFlags`, `translate`), `src/integral_compute.rs`
  (`compute`), `src/integral.rs` (`StoragePlan`); ADR 0003, 0009, 0010, 0011,
  0016, 0018, 0022, 0024, 0028, 0034

## Context

Every concept the generated API is built from was written down several times
in typekin's source. On 2026-11-08 the operator set was listed in about 17
places, the 185 function names three times each, every flag three to four
times, and a new wrapper-wide key touched 15 to 18 sites in six files. About
eleven constructors each spelled out "check the raw value, construct, else
fail", four representations of an integer value set coexisted, and the
emission order differed from site to site while being treated as public
output.

`PLAN.md` (check levels, `try_` forms, enum storage, opt-in `unsafe`) adds
more of each. `PLAN_0.md` was written to follow it. The user had it applied
first instead, to the tree as it stood with `panic_at` (ADR 0028), so that
`PLAN.md` lands on one owner per concept.

## Decision

- **Five stages.** Every generated function or trait impl is one operation
  passing through five stages, each with one owner in the source:

  | Stage | Owner | Varies by |
  |---|---|---|
  | Operation | an `OpSpec` row (operators), a `CoreFwd` row (named forms), an ingress constructor | identity, arity, flags, `konst_*` flags, closure op, how it can panic |
  | Right-hand side | a `Slot` of the wrapper's `Slots` | same type, raw amount, friend, facet slot; arithmetic; `produce` |
  | Computation | `Maker::compute(row: OpRef) -> Body { tokens, fails, panics }` (an operator with its slot, a forward row, a `Step`, a fold) | profile arithmetic, `panic_at`, proofs |
  | Construction | `Validation` (the conjuncts), `Check`, `OnInvalid`, `StoragePlan` | validate, proven, `Within`; the storage base |
  | Form | `translate(fails, panic_at, fixed_output)` and one form emitter | trait impl or named method, assignment, `Step` |

  Constructors are the identity operation: `make`, `try_make` and `map` go
  through Construction and Form as `+` does. Ingress keeps its own panic
  messages.
- **Data rows beside `quote!`.** A concept is a table row, an enum with one
  name table, or one emitter. Rows are plain constants, never a closure per
  row; `quote!` templates stay, and no macro is added (ADR 0016).
    - `Named` (`TABLE` of `(variant, text)`, default `name()` and `of()`):
      `FnName`, `Key`, `Konst`, `N`, `F`, `Capability`, `ProfileKind`,
      `PanicAt`, `MathProvider`. `parse_choice::<T: Named>` parses a
      single-word value and builds "expected `a`, `b`, or `c`" from the table.
    - `FlagRow { flag, name, on }` per flag enum; `KonstRow { konst, name,
      stable, integral, text }` for the `konst_*` names.
    - `OpSpec` rows for the twelve operators (`OPS`), with the integral-only
      flag columns in `IntegralFlag::of_op`.
    - `CoreFwd` rows (`CORE_INT`, `CORE_FLOAT`, `FLOAT_MATH`) shared by the
      integral and float `core` forwards and read by `domain` typing.
    - `PROFILES`: one `ProfileSpec` row per profile.
    - `rejection(site, key)`: the keys a configuration site rules out.
  Each table keeps one compile-time assertion that row `i` holds variant `i`.
  A new flag, `konst_*`, operator, forward, profile or wrapper-wide key is a
  variant plus its row (and, for a key, a `WrapperWide` field and arm).
- **D0.1, one canonical order.** Typekin promises only that the annotated item
  comes first (under `hide_in`: the private module first, the item first in
  it). Everything after it follows the tables: rows in table order, each
  row's slots in slot order, each slot's forms in form order. The order is
  deterministic but may change between versions. This replaces ADR 0016's
  "that order is public output".
- **D0.2, `assign_xor`.** `^=` gets its own flag like every other assignment
  operator, on by default; `without = math_xor` alone no longer removes it.
- **D0.3, rejections point at the key.** A key a site rules out is reported at
  the key, not at the call site or the value. A facet rejects `precedes` with
  the wrapper-wide message whatever its profile.
- **D0.5, bitflag value-type storage.** Bitflag constructs through
  `StoragePlan` like every wrapper. Its value types keep primitive storage:
  they hold combined and unknown bits, must include zero, and are the type
  most likely to cross FFI, so a rewritten storage (`PLAN.md`'s enum storage,
  ADR 0034's niche storage) will need an explicit opt-in in the nested
  configuration.
- **D0.6, one attribute rule** (`zz_lib::FnAttrs`). `#[track_caller]` on every
  generated function and trait method whose body can panic: the computation
  panics (rustc's overflow, division and shift checks, `sequential`'s panic, a
  forward row's `panics` column), the translation is `Unwrap`, or the storage
  write tests for zero; transitively over generated callers. A proof's
  `debug_assert!` does not count, and an unvalidated type loses the attribute
  where nothing can panic. `#[must_use]` on every inherent function that
  returns a value, unless the return type is itself `#[must_use]` (`Result`,
  `impl Iterator`): measured on expanded examples, marking those too raises
  `clippy::double_must_use`. `#[inline(always)]` stays where it was. The rule
  is derived from the form, its `Body` and its `Translation`, never set per
  row.
- **D0.7, closure eligibility is unchanged.** Proofs stay gated by the plain
  interval shape; widening it is future work (`PLAN_0.md` §12.1).
- **D0.8, rustdoc.** The fixed-profile macros and `float` refer to
  `integral`'s configuration and name only their differences.
- **Rules.** Refactors leave the set of generated items and the tokens inside
  each item unchanged (compared order-insensitively with `expandem`); intended
  output changes are the decisions above. Names stay enums. Shared `zz_*`
  files never import a family; family-only columns live in family files.

## Rejected alternatives

- **A typed code model** stays rejected (ADR 0016): rows describe what to
  emit, templates emit it.
- **Not abstracted, on purpose:** the int and float domain normalizers and
  emitters (every tie-break feeds the zero probe and `NonZero` storage); the
  three protocol emitters and the `of<T>` makers; a shared serde skeleton and
  cross-family "view" emitters; folding `ClosureOp` into `OpSpec`, or
  generating the `checked_*`/`wrapping_*` forward rows from operator × policy;
  analyses that only look alike (structural `Neg`/`abs` proofs, `clamp`, the
  untyped probes, the float class analysis); `zz_domain_within_const.rs`,
  which is emitted verbatim; the per-family flag enums, `deny_flags` rules,
  friend grammars, and text's value-list membership.
- **`HideIn` through `parse_choice`.** Its value is `none` or any module name,
  not a closed set of names.
- **A per-site list of the keys `packed` takes.** `packed` names every key it
  does not take; the site only says that such a key is reported by name.
- **Text forwards as forward rows.** They take a different receiver, have
  generic `where` clauses and mutate by consuming `self`; they share only
  `Forward` and `Removals`.

## Consequences

- Where a change lands:

  | Change | Before | After |
  |---|---|---|
  | A new integral flag | 5 places in `integral_cfg.rs` | a variant and a row (plus category lists) |
  | A new `konst_*` | 4 places in `zz_lib.rs` | a variant and a row |
  | A new wrapper-wide key | 15-18 sites in 6 files | a `Key` row, a `WrapperWide` field and arm, site rows |
  | A new operator | about 10 tables | an `OpSpec` row, plus its flags and `konst_*` variants |
  | A new forward | a row, or a hand-written `quote!` block | a row |
  | A new storage kind | `StoragePlan` and its bypasses | a `Base` arm |

- `name()` and `of()` are trait methods: a file that calls them imports
  `zz_lib::Named`.
- A table no longer gets the exhaustiveness check a `match` had. The index
  assertion catches a misplaced row; a variant added at the end without a row
  surfaces as an unknown name.
- Removal keys are `(Form, FnName)` (ADR 0018): `fn_try_push` is the `Try`
  form of `push`. A name that exists only as a `try_` form now answers
  `without = fn_push` with "removes nothing" instead of "unknown flag".
- Generated items moved relative to each other once (D0.1); the checked-in
  `_exp.rs` examples were regenerated.
- The source did not get smaller, against the plan's estimate (about 29.2k
  lines in `src/*.rs` before and after). The tables and shared emitters
  removed about 750 lines; the slot/`compute`/`translate` model and the
  attribute rule added them back, because the helper layer and both operator
  paths stay until `PLAN.md` removes them.
- Constructor bodies took one shape (`return if … { … } else { … };`); the
  token changes are listed in `PLAN_0.md` §13.
- Not verified when this landed: the slow suite (fixture crates, assembly
  comparisons) was last run before `assign_xor`, the slot model and the
  attribute rule; `#[track_caller]` in particular needs the assembly
  comparisons.
- Known gaps of the attribute rule: text's `try_insert`, `try_insert_str`,
  `try_replace_range` and `try_truncate` carry `#[track_caller]`, but their
  `String` call sits in a closure, so a panic there still reports generated
  code; packed's `__typekin_packed_valid` is unmarked.

## Left for `PLAN.md`

These parts of `PLAN_0.md` have no subject until `PLAN.md` is implemented;
each already has its owner:

- check levels (`lax` guard, `pedantic` prechecks) and `unsafe_code` (U1-U5):
  `compute()` and the `Check` passed to construction; `WrapperWide` gains the
  keys and its cross-key `validate()`;
- `try_` forms for integral forwards: `Form::Try` and a `forms` column;
- `{Op}Rhs` dispatch for non-facet wrappers: the slot list already describes
  both paths;
- enum storage and the hull query: a `Base` arm and a `ResolvedDomain` query
  over `IntervalSet`;
- friends with a typekin domain and implied conjuncts (Q31): the conjunct
  list.

## Sources

- `PLAN_0.md` (2026-11-08) and the omp session of 2026-11-09 that applied it
  (the user's choices: apply to the current tree, include the intended output
  changes, no commits)
