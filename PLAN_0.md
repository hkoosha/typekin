# PLAN_0: one operation model for code generation

Status (2026-11-08): design only, nothing implemented. This plan starts after
PLAN.md is fully implemented (check levels, operation forms, removing checks,
friends with a typekin domain; recorded in ADRs 0028-0032). It changes how
typekin is written. Generated code changes only as §5.1 decides: item order
(D0.1), the `assign_xor` flag (D0.2), diagnostic spans (D0.3), forward
attributes (D0.6), and bitflag value-type storage where PLAN.md chose
differently (D0.5). D0.8 changes typekin's own rustdoc.

The inventory in §3 comes from a read-only survey of the tree as it was on
2026-11-08, before PLAN.md. Nothing was built for it. References marked
"(pre-PLAN)" must be re-mapped in Phase 0 before any change.

## 1. Goal

- Every concept the generated API is built from gets one owner in typekin's
  source: a table row, an enum with one name table, or one emitter. Adding an
  operator, a forward, a flag, a wrapper-wide key, a check level or a storage
  kind then touches one place, plus its tests and docs.
- typekin gets smaller: about 1.3k-1.6k lines net, on top of what PLAN.md
  deletes itself (estimate from the pre-PLAN tree, §8).
- Generated behavior stays the same apart from the decisions in §5.1 (P1).
  AGENTS.md's public contract (diagnostics, generated impls, ownership,
  layout, constness, runtime cost) is the acceptance test. Item order is not
  part of it, except that the annotated item comes first (D0.1).

Non-goals:

- no new user-facing feature, key or flag, apart from `assign_xor` (D0.2);
- no typed code model; `quote!` stays (ADR 0016 rejected the "lang" DSL);
- no macros written only to reduce repetition, no new directories, no README
  change (AGENTS.md).

## 2. The model

Every generated function or trait impl is one operation that passes through
five stages:

| Stage           | Owner (target)                                                                                                    | Varies by                                                                                               | What PLAN.md added here                                                  |
|-----------------|-------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------|
| Operation       | an `OpSpec` row (operators), a `CoreFwd` row (named forms), an ingress constructor (`make`, `try_make`, `map`, …) | identity, arity, flags, `konst_*` flags, closure op, how it can panic                                   | `try_` forms, traits that fix `T` (R8), `pedantic` prechecks (§3.4)      |
| Right-hand side | a `Slot`                                                                                                          | same type, raw, friend (with or without a typekin domain), facet slot; arithmetic; `produce`            | `{Op}Rhs` dispatch for every wrapper; friends with a typekin domain      |
| Computation     | `compute(row, slot) -> Body { tokens, fails }`                                                                    | profile arithmetic, check level, proofs, `unsafe_code`                                                  | levels, prechecks, proven bodies, U3/U4                                  |
| Construction    | `Validation` (a list of conjuncts), `Check`, `StoragePlan`                                                        | validate, proven, `Within`, the `lax` guard, conjuncts implied by a friend's domain; the storage base   | `lax`, enum storage, conjunct skipping (Q31), U1/U2/U5                   |
| Form            | `translate(fails, panic, fixed_output) -> Option<Translation>` and one form emitter                               | regular form (trait impl or named method), `try_` form, assignment, `Step`                              | `panic`, R8, L11, L12                                                    |

Constructors are the identity operation. `make`, `try_make` and `map` go
through Construction and Form as `+` does; their Computation is empty or the
user's closure. Ingress keeps its own panic messages and stays outside the
`panic` setting (PLAN.md Q24).

Sketch, to be adjusted to the post-PLAN code in Phase 0:

```rust
// zz_values.rs: family-neutral
pub(crate) enum Op { Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr, Neg, Not }
pub(crate) enum OpKind { Arith, Bitwise, Shift, Unary }
pub(crate) struct OpSpec {
    pub(crate) op: Op,
    pub(crate) trait_name: &'static str,  // "Add"; the assignment is "AddAssign"
    pub(crate) method: &'static str,      // "add"
    pub(crate) kind: OpKind,
    pub(crate) family: Konsts,            // Math or Bit
    pub(crate) konst: Konsts,
    pub(crate) konst_assign: Option<Konsts>,
    pub(crate) closure: Option<ClosureOp>,
    pub(crate) symbol: &'static str,      // "+", for `prove` diagnostics
}

// integral_cfg.rs: integral-only columns (P5)
pub(crate) struct OpFlags { math: IntegralFlag, assign: Option<IntegralFlag> }

pub(crate) struct Slot { rhs: RhsKind, arithmetic: Arithmetic, produce: SelfConverter }
pub(crate) struct Body { tokens: TokenStream, fails: bool }
pub(crate) enum Translation { Pass, Unwrap(&'static str) }
pub(crate) fn translate(fails: bool, panic: Panic, fixed_output: bool) -> Option<Translation>;
```

## 3. Baseline (pre-PLAN, 2026-11-08)

### 3.1 Operator identity

The operator set is listed in about 17 places:

- `ArithmeticOp` (7 variants) and `Operator` (8 variants), with 3 and 6
  matches (`src/zz_values.rs:695-897`).
- `IntegralFlag::math` and `IntegralFlag::assign`
  (`src/integral_cfg.rs:1199-1226`). The `Math*`, `Assign*` and `FnMath*`
  variants are repeated in `ITEMS`, `DEFAULT_ON` and `name()`.
- 24 per-operator `Konsts` variants and their string match
  (`src/zz_lib.rs:113-286`).
- `SHIFTS` (`src/integral_impls.rs:63-91`) and the order in `impls_body`
  (`:114-141`).
- `MathHelper` and `FN_MATH_HELPERS` (`src/integral_items.rs:77-123`), which
  PLAN.md removes.
- `ProveOp` and `PROVE_OPERATORS` (`src/integral_proof.rs:384-441`), a third
  copy of the operator symbols.
- The xor special case `make_math_xor` (`src/integral_impls.rs:689-731`).
  There is no `AssignXor` flag, although `konst_xor_assign` exists
  (`src/zz_lib.rs:282`).
- Bitflag: two copied shift blocks and the bit operators
  (`src/bitflag.rs:1555-1640`).

Emission order differs from site to site; one canonical order replaces these
orders (D0.1):

- assignments: `BitAnd`, `Add`..`Rem`, `BitOr`;
- helpers: `Add`..`Rem`, `Xor`, `And`, `Or`, `Shr`, `Shl`;
- prove list: `Add`..`Rem`, `And`, `Or`, `Xor`, `Shr`, `Neg`, `Not`.

### 3.2 Named forms (forwards)

- **Integral:** `CoreForward { name, sig, sign, gate, proof }` with builder
  functions (`src/integral_forward.rs:519-737`) and the table at `:738-897`.
  Six `Custom` rows point at hand-written emitters (`:292-422`).
- **Float:** four tuple tables (`src/float.rs:30-149`) and 7 hand-written
  blocks (`:180-423`). `NIGHTLY_MATH` is an ordered subsequence of `STD_MATH`
  that lacks only `powf`.
- **Text:** closures (`src/text.rs:490-634`) and 5 hand-written blocks.
- 17 hand-written blocks in total. Each family rewrites the return shapes
  (raw, `Self`, `Option<Self>`, `(Self, bool)`, `(Self, Self)`).
- `abs` is special-cased 3 times: `src/integral_forward.rs:298-316`,
  `src/float.rs:336-342` and `src/integral_proof.rs:327`.
- `FnName` lists its 185 names 3 times (`src/zz_values.rs:905-1475`: enum,
  `ITEMS`, `name()`). Lens lists and arities repeat names
  (`src/zz_domain_float.rs:162-199`, `src/zz_domain_int.rs:258-262`).
- Integral and float forwards differ in `#[must_use]` and `#[track_caller]`
  (survey finding; re-check in Phase 0).

### 3.3 Construction and storage

- About 11 constructors each write "check the raw value, construct, else fail"
  by hand:
    - `try_make`, `_or_panic`, `_proven`, `_blind_trust` (removed by PLAN.md)
      and `map`/`try_map` (`src/integral_items.rs:479-530`, `:945-1081`);
    - `_within` (`src/integral_within.rs:378-410`);
    - `_result_or_panic`, `_make` and `_make_proven`
      (`src/integral_forward.rs:452-519`);
    - `FromStr` and `Deserialize` (`src/integral_impls.rs:255-361`);
    - packed `construct` (`src/packed.rs:767-780`).
- The "`Err` → panic" translation is copied 6 times:
  `src/integral_items.rs:519-528`, `src/bitflag.rs:520-533`,
  `src/text.rs:645-650` and `:907-927`, `src/packed.rs:946-954`,
  `src/zz_lit.rs:403`.
- `StoragePlan` (`src/integral.rs:741-874`) owns storage reads and writes,
  with exceptions:
    - bitflag builds `Self(bits)` and `#vl(bits)` directly
      (`src/bitflag.rs:1075`, `:1156`, `:1288`) and passes
      `StoragePlan::primitive`;
    - `_blind_trust` and `_within` spell out the construction by hand
      (`src/integral_items.rs:1059-1064`, `src/integral_within.rs:395-400`).
- `StoragePlan` holds two bools, `non_zero` and `wrapping`.

### 3.4 Configuration names and keys

- **Flag enums list each flag 3 to 4 times.** `IntegralFlag` has the enum,
  `ITEMS`, `DEFAULT_ON` and `name()` (`src/integral_cfg.rs:1356-1735`).
  `TextFlag`, `BitFlag` and `PackedFlag` follow the same pattern, with 4
  identical index-assert blocks.
- **`Key`:** the enum, `ITEMS: [Self; 33]` and `name()`
  (`src/zz_lib.rs:893-1006`).
- **`Konsts`** (`src/zz_lib.rs:113-286`): the enum, the `INTEGRAL` and `TEXT`
  lists, `is_stable` and a 50-arm string match. It does not follow the
  `ITEMS`/`name()` convention.
- **Single-word value parsers are hand-written.** `Konst::parse`
  (`src/zz_lib.rs:305-326`) and `MathProvider::parse`
  (`src/zz_domain_float.rs:60-70`) compare `to_string()` with string
  literals. `ProfileKind` and `HideIn` have their own parsers too.
- **Wrapper-wide settings are rejected by hand at each site:**
    - nested (`src/integral_cfg.rs:887-910`);
    - fixed profiles (`:922-925`);
    - facets (`src/zz_profile_integral.rs:529-531` and
      `src/integral_cfg.rs:177-221`, where the "wrapper-wide" text appears 4
      times);
    - packed (`src/packed.rs:1352-1357`, `:1404-1417`);
    - bitflag forwarding (`embed_konst`, `src/integral_cfg.rs:764-783`).

  One new wrapper-wide key touched 15-18 sites in 6 files, and PLAN.md adds
  three: `checks_level`, `panic` and `unsafe_code`.
- **Profiles:** six profile structs, five of which hold nothing but
  `settings` (`src/zz_profile_integral.rs:152-199`). Profile facts are spread
  over `arithmetic()`, `preset_off()`, `from_name`,
  `settings()`/`settings_mut()`, and the float and facet checks.
- **Already shared, nothing to do:** `parse_inner`
  (`src/zz_lib.rs:1018-1042`), `FlagSettings::parse`, `expand_hideable` and
  `zz_hide::Envelope`.

### 3.5 Domain queries

- Four representations of an integer value set:
    - closure intervals as `Vec<(Int, Int)>`
      (`src/zz_domain_closure.rs:418-473`);
    - the `Within` form `Form { modulus, pieces }`
      (`src/zz_domain_within.rs:64-78`);
    - `RNode::Interval`;
    - point queries (`src/zz_domain_int.rs:1439-1472`).

  None of them offers a hull, min or max query.
- `intervals()` makes closure proofs ineligible for shapes like
  `..=10 & !{5}`.
- About 15 places handle the pointer width. APIs mix a width index with a width
  in bits, and closure's private `Ty` duplicates within's mask and pattern
  helpers.
- "The same tokens at every width are emitted once, otherwise once per `cfg`"
  is written 5 times: `src/zz_domain_int.rs:1476-1502`,
  `src/integral_within.rs:746-790` (twice), `src/integral.rs:69-84` and
  `src/integral_items.rs:739-768`.

### 3.6 Emitter boilerplate

- The `ty` / `impl_generics` / `where_clause` triple appears 16 times
  (`src/integral_impls.rs` 12 times, `src/float.rs` and
  `src/integral_forward.rs` twice each).
- 9 emitters use `let mut stream; if flag { … }` instead of the early return
  that AGENTS.md asks for (`src/integral_impls.rs:262`, `:302`, `:516`, `:545`,
  `:589`, `:612`, `:654`, `:965`, `:1033`).
- The generics rejection is copied 3 times (`src/text.rs:386-395`,
  `src/bitflag.rs:158-168`, `src/packed.rs:201-210`).

### 3.7 After PLAN.md: what to check in Phase 0

PLAN.md removes:

- the per-operator helpers and the `fn_math_*` flags (L16);
- `_eq`, `_cmp` and the `fn_op_*` flags (Q29);
- blind trust in every family (L17);
- inherent methods that duplicate trait impls (L11, Q25).

It adds:

- `{Op}Rhs` dispatch for every wrapper;
- one emitter for `_op` plus the translation (its Phase 2);
- `try_` forms;
- three wrapper-wide keys;
- enum storage and U1-U5;
- domain-aware friend construction.

Signs to look for, where PLAN.md may have left duplication:

- `try_` and regular forms written by separate emitters;
- level branches (the `lax` guard, `pedantic` prechecks) in several
  constructors and forwards instead of in one computation step;
- U forms or enum-storage branches outside `StoragePlan`;
- friend conjunct skipping (Q31) emitting validation apart from `_or_panic`;
- hand-written rejections for each of the three new keys;
- new `Try*` `FnName` variants, each listed 3 times.

## 4. Work items

Each item states a target, the baseline from §3, how it interacts with
PLAN.md, its effect on output, and a rough estimate of lines saved (pre-PLAN).

### W1. Name rows

- **Target:** `trait Named { const TABLE: &'static [(Self, &'static str)]; }`,
  with default `name()` and `of()`.
    - Flag enums get `FlagRow { flag, name, on }`, which replaces `ITEMS`,
      `DEFAULT_ON` and `name()`.
    - `Konsts` gets `KonstRow { konst, name, stable, integral, text }`. It
      replaces the string match and the `INTEGRAL`/`TEXT` lists.
      `Konsts::of` strips `konst_` the way removals strip `fn_`.
- **Applies to:** `FnName`, `IntegralFlag`, `TextFlag`, `BitFlag`,
  `PackedFlag`, `Konsts`, `Key`, `ProfileKind`, `Capability`, `N` and `F`,
  plus PLAN.md's level, `panic` and `unsafe_code` enums.
- **Invariant:** keep one compile-time index assertion per enum,
  `TABLE[i].0 as usize == i`. A generic `const fn` cannot call trait methods on
  stable, so each enum keeps its own short `const _` block.
- **Output:** none. Messages that join names ("expected `a`, `b`, or `c`")
  keep their order.
- **Estimate:** about −485 lines (`FnName` −185; flags, keys and `konst_*`
  −300). A new flag goes from 5 edits to 2, a new `konst_*` from 4 to 2.

### W2. `parse_choice`

- **Target:** `fn parse_choice<T: Named>(input: ParseStream) -> syn::Result<T>`.
  Its "expected …" message is built from `T::TABLE`.
- **Replaces:** `Konst::parse`, `MathProvider::parse`, `ProfileKind::parse`,
  `HideIn::parse`, and the PLAN.md parsers for `checks_level`, `panic` and
  `unsafe_code`.
- **Output:** messages and spans stay exactly as they are. `Konst` reports at
  `input.span()` when the next token is not an identifier, and `ProfileKind`
  applies `snake_case_of` first.
- **Estimate:** about −40 lines pre-PLAN, more after PLAN.md's three keys.

### W3. Wrapper-wide settings and a site table

- **Target:**
    - `WrapperWide { konst, konsts, checks, panic, unsafe_code }`;
    - its `parse_attr`;
    - `embed(&outer)`, which replaces `embed_konst` for bitflag forwarding;
    - `validate()` for rules across keys: `lax` with `unsafe_code` is an
      error (R11); floats reject `unsafe_code` (Q27); floats treat
      `pedantic` as `standard`.
- **Site table:** `fn rejection(site: Site, key: Key) -> Option<&'static str>`
  with `Site { Root, FixedProfile, Nested, ProfileBody, Facet, Packed, Text }`.
  `parse_inner` checks it before it calls the family closure, and before
  anything is consumed.
- **Output:** diagnostics only. Every rejection points at the key itself
  (D0.3). Today the nested rejections (`src/integral_cfg.rs:887-910`) and
  the fixed-profile "profile is selected by this macro" (`:923-925`) point at
  `Span::call_site()`.
- **Estimate:** about −50 lines pre-PLAN, more after PLAN.md's three keys. A new
  wrapper-wide key then needs a `Key` row, a `WrapperWide` field and arm, and
  rows in the site table.

### W4. Profile table

- **Target:** `Profile { kind, settings, precedes }` and
  `const PROFILES: [ProfileSpec; 7]`. Each row holds:
    - the name;
    - the arithmetic;
    - the preset-off lists;
    - whether floats and facets accept the profile;
    - `Wrapping` storage;
    - the `precedes` default.

  The proof policy per arithmetic and level (R7) becomes a function of the row
  and the level.
- **Keep:** the "unknown attribute" error for `precedes` on profiles other than
  `circular`, and `delta` as `circular` with `precedes` off.
- **Estimate:** about −90 lines.

### W5. `OpSpec`

- **Target:** the table of §2.
- **Replaces:**
    - `ArithmeticOp` and the matches on `Operator`;
    - `SHIFTS`;
    - `ProveOp` and `PROVE_OPERATORS`;
    - `IntegralFlag::math`/`assign`, `Operator::is_konst_flags` and
      `Operator::assign_konst`;
    - bitflag's copied shift blocks;
    - every per-operator table PLAN.md added (its §3.4 precheck, `try_`
      names, R8's fixed output).
- **Order:** table order is the canonical operator order (D0.1), and every
  site iterates the table. No per-site order arrays.
- **Xor:** `make_math_xor` goes. Xor's assignment gets the `AssignXor` flag
  and follows the same rule as every other assignment (D0.2).
- **Location:** the family-neutral columns go in `zz_values.rs`. The integral
  flags stay in an integral file, `IntegralFlag::of_op(op) -> OpFlags`, because
  shared `zz_*` files never import a family (P5).
- **Kept separate:** `ClosureOp` is a superset with its own semantics
  (`DivEuclid`, `Midpoint`, `Isqrt`, `UnboundedShr`, `Abs`). Rows point to it.
- **Estimate:** about −150 to −200 lines, after PLAN.md's own removals.

### W6. Slots

- **Target:** for each operator, one ordered list of `Slot`s, built once from
  the facets or from the root configuration (same type, raw, friends). Every
  `{Op}Rhs` impl comes from that list. A slot carries:
    - the right-hand-side kind, with the friend conversion and the domain
      annotation;
    - the arithmetic;
    - `produce` and the output.
- **Replaces:**
    - the split between facet and non-facet operators
      (`with_global_operators`, `src/integral.rs:1085`, pre-PLAN);
    - the field-by-field copying of `arithmetic` and `ty_out` through
      `SlotSource`, `insert_slots`, `DefaultFacetCfg` and `FacetSlot`
      (`src/integral_cfg.rs:120-126`, `:290-318`, `:397-436`, `:734-739`;
      `src/zz_profile_integral.rs:575-591`).
- **PLAN.md:** its Phase 2 sends non-facet wrappers through `{Op}Rhs`. Check
  whether it already built a uniform slot list.
- **Estimate:** about −25 lines in configuration, plus whatever branching
  PLAN.md left behind.

### W7. `CoreFwd` rows

- **Target:**
  `CoreFwd { name, recv, args, ret, konst, when, proof, structural, panics, forms }`.
    - `ret`: `Raw(Out) | Wrap | Opt | Pair(RetEl, RetEl)`.
    - `panics`: `Never | Twin(FnName) | RhsNonZero | AmountBelowBits(&str)`,
      which is PLAN.md §3.4 as data.
    - `structural`: replaces the three `abs` special cases.
    - `when`: includes `Provider`, which merges `NIGHTLY_MATH` into
      `STD_MATH`.
    - `forms`: regular and/or `try_`.
- **Float and integral** share the row type and the builders for signature and
  body. The provider-specific call syntax stays a branch in the body builder.
  Domain-lens arities come from `args.len()`, with a `lens` column.
- **Text** forwards stay separate: they take a different receiver, have
  generic `where` clauses and mutate by consuming `self`. They share only
  `Forward`, `Removals` and W11.
- **Attributes** follow one rule (D0.6), derived from the form and from
  `Body::fails`, never set per row:
    - `#[track_caller]` on every generated function and trait method whose
      body can panic: validation, the translation under `panic = allow`,
      rustc's own checks, and the `NonZero` and hull tests;
    - `#[must_use]` on every inherent function that returns a value;
    - `#[inline(always)]` where the generators place it today.
- **Estimate:** about −350 lines. A new forward becomes a row, with no new
  `quote!` block.

### W8. `compute()`

- **Target:** `fn compute(&self, row: OpRef, slot: &Slot) -> Body`, where
  `OpRef` is either `&OpSpec` or `&CoreFwd`. It owns:
    - the profile arithmetic (pre-PLAN `Arithmetic::make_expr`,
      `make_const_expr` and `make_operator_expr`);
    - `Neg`, `Not` and the shift amount;
    - the level: the `lax` guard, and the `pedantic` prechecks taken from
      `Panics` and `OpKind`;
    - proofs: `prove` makes `fails` false (Q16), plus the proven forms of
      §4.1;
    - U3 and U4;
    - `Step::forward` and `Step::backward`.
- **PLAN.md:** its Phase 2 required one emitter. Check that every form reaches
  it (operators, `Neg`, `Not`, shifts, `Step`, `core_int` and float forwards,
  bitflag's enum forwarding), and that no level or proof branch lives anywhere
  else.
- **Estimate:** depends on Phase 0. The pre-PLAN baseline gives about −40 lines
  (the copy of the arithmetic match inside `_neg`, `rhs_for` matched twice).

### W9. `translate()`

- **Target:** a pure function implementing PLAN.md §3.6 and R8, returning
  `None` where no form is generated. Beside it, a token helper
  `unwrap_or_panic(expr, message)` is shared with the 6 ingress copies of §3.3.
  Their messages stay as they are.
- **Estimate:** about −30 lines. L7 asks for exactly this separation.

### W10. Construction

- **Validation as conjuncts.** A wrapper's validation is an ordered list of
  conjuncts: domain atoms and each callback, each marked pure or callback. One
  emitter writes it for:
    - `_or_panic`, `try_make`, `_result_or_panic`, `_make`, `map`/`try_map`;
    - parsing and Serde;
    - friend `Make`.

  Each site passes:
    - a `Check`: `Validate`, or `DebugOnly(message)` for proven, `Within` and
      the `lax` guard;
    - the conjuncts implied by a friend's domain (Q31's
      `IMPLIED || check(raw)`);
    - an `OnInvalid`: `Err(tokens)`, `None` or `Panic(message)`.

  U1 asserts only the facts of pure conjuncts.
- **One storage path.** `StoragePlan` becomes
  `{ base: Base, wrapping: bool }` with
  `enum Base { Plain, NonZero, Enum { .. } }`.
    - Reads (`extract`) carry U1.
    - Writes (`storage_expression`) carry the zero and hull tests, U2 and U5.
    - Every bypass goes through it: bitflag's direct `Self(bits)` and the
      hand-written construction in `_within`. Under primitive storage this
      emits the same tokens, or a reviewed token diff. Bitflag value types
      then choose their storage per D0.5.
- **Keep:** constness as a parameter of each constructor; panic messages; the
  public parameter names (`it`, `raw`, `value`).
- **Estimate:** about −70 to −100 lines pre-PLAN. The main gain is that `lax`,
  `unsafe_code`, enum storage and conjunct skipping each live in one place.

### W11. Removal keys by form

- **Target:** `Removals` keyed by `(Form, FnName)` with
  `Form { Regular, Try }`. `fn_try_pow` parses by stripping `fn_`, then `try_`.
  This replaces any `Try*` `FnName` variants that PLAN.md's Phase 4 added for
  removals. The internal key changes; the user's spelling does not.
- **Text:** its own `Try*` names (`TryPush`, `TryTruncate`, …,
  `src/zz_values.rs:1066-1075`) become `(Try, Push)` once PLAN.md's Phase 10
  has settled text. Users keep writing `without = fn_try_push`. If text then
  has regular forms too, `(Regular, Push)` fits beside them.
- **Caveat:** a core method literally named `try_*` would collide. None of the
  integral forwards has such a name; verify this in Phase 0.

### W12. Domain value sets

- **Target:**
    - `IntervalSet`: sorted, disjoint, inclusive intervals;
    - `ResolvedDomain::values(width) -> Result<Form, Opaque>`, `hull(width)`,
      `same_hull()` and `excludes(at)`;
    - `Width` (an index) and
      `IntTy { signed, bits, min, max }` in `zz_values.rs`;
    - `by_width(values) -> Vec<(cfg, T)>` with two modes, because today's
      sites differ: one item per width, or widths grouped in `any(...)`.
- **Consumers:**
    - closure proofs and PLAN.md §4.1's obligations;
    - the hull for enum storage;
    - U1 facts;
    - `Within` and conjunct implication for friends (Q31).
- **Output:** closure eligibility stays on today's `intervals()` gate, so
  output does not change. Widening it is future work (D0.7, §12.1).
- **Keep apart:** `IntervalSet` must not leak into
  `zz_domain_within_const.rs`, which is emitted verbatim.
- **Estimate:** about ±0 for the set itself, −40 for widths, −30 for
  `by_width`.

### W13. Emitter boilerplate

- **Target:**
    - `Maker::impl_trait(konst, path, body)`, plus a variant that takes extra
      generics (Serde, `PartialEq<T>`, operators);
    - early returns, following AGENTS.md's emitter shape;
    - `reject_generics(generics, message)` in `zz_lib.rs`.
- **Estimate:** about −80 lines.

## 5. Decisions

### 5.1 From the user, 2026-11-08

- **D0.1. One canonical order.** The only order typekin promises is that the
  annotated item comes first in the generated code. Under `hide_in`, the
  private module comes first and the item is its first item. Everything else
  may be reordered. This replaces the rule in AGENTS.md and ADR 0016 that the
  orchestrators' order is public output.
- **D0.2. `assign_xor`.** Xor's assignment gets its own flag, `assign_xor`,
  like every other assignment operator.
- **D0.3. Rejections point at the key.** Every rejected key, at any site,
  reports at the key's span, not at `Span::call_site()`.
- **D0.5. Storage of bitflag value types.** Bitflag's direct construction
  goes through `StoragePlan` (W10). The value types then follow the general
  storage rule, except that enum storage needs `with = enum_storage` in the
  nested configuration.
    - A bitflag value type exists to hold combined and unknown bits. Its
      domain must include zero, so it never gets `NonZero` storage, and it is
      the typekin type most likely to cross FFI.
    - With enum storage, a bit pattern outside the hull that arrives through
      FFI or `transmute` becomes UB instead of an invalid value. The gain (a
      niche, range facts for LLVM) does not justify that by default.
    - U1 applies only under `unsafe_code = allow`, which is opt-in already.
- **D0.6. Consistent attributes.** Generated functions follow one rule for
  `#[track_caller]` and `#[must_use]` (W7).
- **D0.7. Closure eligibility stays as it is in this plan.** W12 keeps
  today's `intervals()` gate. Widening it is future work (§12.1).
- **D0.8. Rustdoc of the fixed-profile macros.** `int`, `circular`, `delta`,
  `sequential`, `wrapping`, `point`, `nominal` and `float` stop repeating
  `integral`'s key list (`src/lib.rs:201-203`, `:283-410`, pre-PLAN). They
  refer to `integral`'s configuration and name only their own differences.
  The copies had already drifted: they omit `prove`, `lit_name` and
  `lit_path`, which the integral parser accepts for these macros
  (`src/integral_cfg.rs:928-931`).
- **Q0.4 was internal** (how removal keys name text's `try_` forms) and is
  settled in W11. Users see no difference.

### 5.2 Rules (proposed; confirm)

- **P1. Behavior-neutral by default.** Each phase leaves the set of generated
  items unchanged, compared without regard to order (§9), with the annotated
  item still first. Tokens inside each item stay the same apart from trailing
  commas and spacing, and diagnostics stay the same. The changes decided in
  §5.1 (D0.2, D0.3, D0.6, and D0.5 where PLAN.md chose differently) land in
  their own commits, with tests.
- **P2. Data, not a code model.**
    - Rows are plain constants, never a closure per entry (ADR 0016 rejected
      the closure-per-entry flag-check table).
    - `quote!` stays, and no macros are added.
- **P3. The canonical order.** After the annotated item, generated code
  follows one deterministic order, derived from the tables rather than kept
  by hand:
    - one section list in the orchestrator: declarations, protocol traits,
      inherent items, trait impls, macros and re-exports, as the post-PLAN
      code groups them;
    - inside a section: rows in table order (`OpSpec`, `CoreFwd`), each
      row's slots in slot order (same type, raw, then friends and facet slots
      in declaration order), and each slot's forms in form order (regular,
      assignment, `try_`).

  The order stays deterministic (AGENTS.md), but it is not stable across
  versions.
- **P4. Names stay enums** (AGENTS.md). A row table is the only place their
  text lives, guarded by the index assertions.
- **P5. Shared `zz_*` files never import a family.** Family-neutral columns go
  in `zz_values.rs` or `zz_lib.rs`. Integral-only columns stay in integral
  files, or in a new `zz_*_integral.rs` directly in `src/`.
- **P6. AGENTS.md's code shape holds:**
    - flat modules;
    - functions under about 100 lines;
    - each emitter a `&self` method returning one impl group, with its flag
      check as an early return;
    - each orchestrator one section list (P3).
- **P7. One concern per commit.** No commit mixes a refactor with an output
  change.

## 6. Not abstracted

- **The int and float domain normalizers and emitters.** ADR 0016 rejected
  `BoundRules`: every tie-break feeds the zero probe and with it `NonZero`
  storage.
- **The three protocol emitters and the `of<T>` makers.** Their public trait
  shapes differ (ADR 0016).
- **A shared serde skeleton, and Debug/Hash/Display/Serde "view" emitters
  shared by integral and text.** ADR 0016 recorded that the serde skeleton
  failed its own acceptance test.
- **Folding `ClosureOp` into `OpSpec`**, and generating the
  `checked_*`/`wrapping_*` forward rows from operator × policy.
- **Analyses that only look alike:** the int and float structural `Neg`/`abs`
  proofs, `clamp`, the untyped probes and the float `Class` query analysis.
- **`zz_domain_within_const.rs`**, which stays plain stable `const fn` code.
- **Configuration that only looks alike:**
    - the per-family flag enums and the `NameKey` impls;
    - the `deny_flags` rules, which run at different times;
    - bitflag's `parse_integral` checks;
    - the friend grammars;
    - text's value-list membership next to integral domains (AGENTS.md).
- **An `_op` helper under another name.** `AddAssign<T>` forwards to the
  `{Op}Rhs` impl (PLAN.md L16).

## 7. Open questions

None; §5.1 records every answer. Phase 0 re-checks D0.2 and D0.5 against
what PLAN.md implemented.

## 8. Estimates

Estimates come from the pre-PLAN tree and do not count what PLAN.md deletes
itself (the helper layer and the non-facet operator path, roughly 500 lines).

| Item                    | Lines (estimate)                                     |
|-------------------------|------------------------------------------------------|
| W1 name rows            | −485                                                 |
| W2 `parse_choice`       | −40                                                  |
| W3 wrapper-wide keys    | −50                                                  |
| W4 profile table        | −90                                                  |
| W5 `OpSpec`             | −150 to −200                                         |
| W6 slots                | −25                                                  |
| W7 `CoreFwd` rows       | −350                                                 |
| W8 `compute()`          | −40 (more if PLAN.md left level branches scattered)  |
| W9 `translate()`        | −30                                                  |
| W10 construction        | −70 to −100                                          |
| W11 removal keys        | avoids about +120 for `Try*` variants                |
| W12 domain value sets   | −70                                                  |
| W13 boilerplate         | −80                                                  |

Where a change lands, before and after:

| Change                         | Before (pre-PLAN)                                                       | After                                                    |
|--------------------------------|-------------------------------------------------------------------------|----------------------------------------------------------|
| A new integral flag            | 5 places in `integral_cfg.rs`, plus the emitter and docs               | a variant and a row (plus category lists), emitter, docs |
| A new `konst_*`                | 4 places in `zz_lib.rs`                                                 | a variant and a row                                      |
| A new wrapper-wide key         | 15-18 sites in 6 files                                                  | a `Key` row, a `WrapperWide` field and arm, site rows    |
| A new operator                 | about 10 tables                                                         | an `OpSpec` row, plus its flags and `konst_*` variants   |
| A new forward                  | a row, or a hand-written `quote!` block for a new shape                 | a row                                                    |
| A level-dependent computation  | every operator and forward site                                         | `compute()`                                              |
| A new storage kind             | `StoragePlan` and its bypasses                                          | a `Base` arm                                             |

## 9. Phases

Verification is the same for every phase:

- **Expansions** match the Phase 0 baseline under the order-insensitive
  comparison (P1; Phase 0, step 5): the annotated item first, the same set of
  items after it, and the same tokens inside each item.
- **Examples** are regenerated in the same commit (`just <name>-expand`, then
  `cargo +nightly fmt`) and pass the same comparison against their previous
  checked-in versions.
- These commands pass. Run them directly, not through `just test`, whose
  `fmt` step formats the root `*.md` files, README included:

```sh
cargo +nightly fmt -- --check
cargo +nightly clippy --locked --lib -- -D warnings
cargo +nightly test --locked --no-default-features
cargo +nightly test --locked
cargo +nightly test --locked --lib --features nightly
```

Refactor phases add no tests. The existing suites and the expansion diff
show that output did not change, and the rows are internal (AGENTS.md: tests
catch consumer-visible bugs). The compile-time index assertions are code
invariants, not tests. Phase 7 adds a regression test for each intended
change.

### Phase 0: re-survey and record

1. Re-map §3 and §10 against the post-PLAN tree. Mark each work item as done
   by PLAN.md, partly done, or open, and update this plan. Drop items that
   PLAN.md made obsolete, and add the findings of §3.7.
2. Check §5.1 against the post-PLAN tree: D0.2 and D0.5 depend on what
   PLAN.md implemented.
3. Write `adr/adr0033__operation_model.md` with status "Accepted (not
   implemented)", and update ADR 0016 (§11).
4. Record the baseline outside the tracked tree, for example in
   `target/plan0/`:
    - `expandem <file> 'typekin::**'` for every `src/e2e` file with a typekin
      attribute;
    - the same for every example;
    - the test results.
5. Build the order-insensitive comparison there as well, as a throwaway that
   is never committed. It parses two expansions with `syn` and checks that
   both start with the same annotated item (or the same `hide_in` module with
   that item first). Then it sorts the remaining items of every module, impl
   and trait by their tokens and compares the rest.

### Phase 1: names and configuration

W1, W2, W3 and W4. W1 comes first: W2 needs `Named`, and W3 uses W2.

### Phase 2: operation rows

W5, W7 and W11. Each row table replaces its old tables in one commit.
Operators and forwards move to the canonical order here (D0.1).

### Phase 3: slots, computation and translation

W6, W8 and W9. Once every form goes through `compute()` and `translate()`,
remove the old paths in the same phase. Each orchestrator becomes one section
list (P3).

### Phase 4: construction and storage

W10. Route the storage bypasses through `StoragePlan` first, then add the
conjunct model. If D0.5 differs from what PLAN.md emits, that change lands
in Phase 7.

### Phase 5: domain queries

W12. Move the consumers over one at a time: closure, the §4.1 obligations,
enum storage, U1, `Within`, conjunct implication.

### Phase 6: emitter boilerplate

W13.

### Phase 7: intended output changes

One commit per decided change, each with a regression test that fails on the
old behavior, and with the regenerated `_exp.rs` files:

- **D0.2:** add `IntegralFlag::AssignXor` (`assign_xor`), on by default like
  the other `assign_*` flags.
    - It joins every category list that holds them: `DEFAULT_ON`,
      `ASSIGN_FLAGS`, `INT_ONLY_FLAGS`, `RESTRICTED_MATH_FLAGS` and the
      profile presets (pre-PLAN names). It also gets its `OpSpec` column.
    - `^=` then follows the same assignment rule as the other operators.
      Pre-PLAN, `+=` depends on `assign_add` alone
      (`src/integral_impls.rs:498-508`), so `without = math_xor` would no
      longer remove `^=` on its own.
    - Tests: `BitXorAssign` is present by default; with
      `without = assign_xor` it is absent (`assert_not_impl!`) while
      `BitXor` stays; the bitflag value type follows the same rule.
    - Docs: the flag lists in `src/lib.rs`.
- **D0.3:** move every rejection in W3's site table to the key's span, and
  update the `src/unit` diagnostic tests that pin the old spans.
- **D0.6:** apply W7's attribute rule.
    - Test: a panicking integral forward and a panicking operator report the
      caller's file and line; a panic hook captures
      `PanicHookInfo::location()`.
    - [INFERENCE] rustc's own overflow panics inside a `#[track_caller]`
      function also report the caller; the test covers one such case.
    - New `#[must_use]` attributes can raise `unused_must_use` warnings in
      user code. That is intended.
- **D0.5**, only where PLAN.md's storage choice for bitflag value types
  differs. Enum storage then needs `with = enum_storage` in the nested
  configuration, which accepts the flag.
    - Tests: a bitflag value type with a small restricted domain keeps
      primitive storage by default (`Option<Value>` is larger than `Value`)
      and gets the niche with `with = enum_storage`. A value type whose
      domain is the whole type never gets enum storage.

### Phase 8: documentation

- **AGENTS.md:**
    - the repository map;
    - "Names are types", now pointing at the row tables;
    - how to add an operator, a forward or a wrapper-wide key;
    - the order rule: only "the annotated item comes first" is promised
      (D0.1); drop "that order is part of the public output" from the
      emitter conventions;
    - the pre-PLAN statements that were already out of date: tests live in
      `src/e2e` with `--no-default-features` for the fast set, not in
      `typekin_testing/` with `test_*` features, and the justfile has no
      `fn-lengths` recipe.

  Fix them unless PLAN.md's Phase 9 already did.
- **`src/lib.rs` (D0.8):** the seven fixed-profile macros and `float` refer to
  `integral`'s configuration instead of repeating its key list, and name only
  their own differences (`profile`, `precedes`, `math`,
  `profile = int | point`). Take each difference from the parser, not from
  the old lists; for example, check whether `float` accepts `prove`.
- **ADRs:** the final pass of §11.

## 10. Code map (pre-PLAN; re-map in Phase 0)

- Operator tables: `src/zz_values.rs:695-897`,
  `src/integral_cfg.rs:1199-1226`, `src/integral_impls.rs:63-141`,
  `src/integral_items.rs:77-123`, `src/integral_proof.rs:384-441`,
  `src/bitflag.rs:1555-1640`.
- Arithmetic emitter: `src/zz_profile_integral.rs:52-150`; `_neg`:
  `src/integral_items.rs:790-820`.
- Facet dispatch: `src/integral_friend.rs:468-600`
  (`facet_rhs_impl` `:501-561`); slots: `src/integral_cfg.rs:120-126`,
  `:290-318`, `:397-436`, `:734-739`.
- Forwards: `src/integral_forward.rs:31-963`, `src/float.rs:30-423`,
  `src/text.rs:490-634`; `Forward` and `Removals`: `src/zz_lib.rs:779-860`;
  `FnName`: `src/zz_values.rs:905-1475`.
- Constructors: `src/integral_items.rs:479-530`, `:945-1081`;
  `src/integral_within.rs:378-410`; `src/integral_forward.rs:452-519`;
  `src/packed.rs:767-780`.
- Storage: `StoragePlan`, `src/integral.rs:741-874`; bitflag bypasses:
  `src/bitflag.rs:1075`, `:1156`, `:1288`.
- Configuration: `Key` and `Konsts`, `src/zz_lib.rs:113-330` and `:893-1042`;
  `IntegralFlag`, `src/integral_cfg.rs:1031-1735`; profiles,
  `src/zz_profile_integral.rs:152-483`.
- Domains: `src/zz_domain_closure.rs:145-205`, `:418-473`;
  `src/zz_domain_within.rs:64-78`, `:251-304`, `:596-614`;
  `src/zz_domain_int.rs:1431-1502`.

## 11. ADRs

New:

| ADR                             | Covers                                                                                                   |
|---------------------------------|----------------------------------------------------------------------------------------------------------|
| `adr0033__operation_model.md`   | the five stages (§2), the row tables, the decisions of §5.1, P1-P7, what stays separate (§6), and the future work of §12 |

Updated:

| ADR                          | Update                                                                                                             |
|------------------------------|--------------------------------------------------------------------------------------------------------------------|
| 0016 codegen architecture    | data rows accepted beside direct `quote!` templates; the typed code model stays rejected; `Named` row tables; only "the annotated item comes first" is promised about order (D0.1) |
| 0018 removals and opt-outs   | removal keys by `(Form, FnName)` (W11)                                                                             |
| 0010 facets                  | one slot list per operator for facet and non-facet wrappers (W6)                                                   |
| 0011 profiles and storage    | the profile table (W4); `StoragePlan` with `Base` as the only storage path (W10)                                   |
| 0003 closure proofs          | the value-set queries (W12); wider eligibility as future work (§12.1)                                              |
| 0024 `Within`                | `Form` as the shared per-width value set (W12)                                                                     |
| 0009 bitflag model           | storage of bitflag value types (D0.5)                                                                              |
| 0022 integral API surface    | `assign_xor` (D0.2); the `#[track_caller]` and `#[must_use]` rule (D0.6)                                           |

## 12. Future work

### 12.1 Wider closure eligibility (D0.7)

Today a closure proof is attempted only when the domain normalizes to plain
intervals on the value (`intervals()`, `src/zz_domain_closure.rs:418-446`,
pre-PLAN). `..=10 & !{5}` and shapes with lens atoms are "ineligible",
although their exact value set is still a union of intervals.

**Change.** Feed closure the exact per-width set from W12 wherever its
`Form` has modulus 1.

**Prerequisites:**

- W12, the value-set query;
- formal evidence (PLAN.md's Phase 5) for the conversion from `Form` to
  intervals, as for the hull rules. Without it, these sets stay "not proven".

**What changes:**

- **Generated code.** More operators construct through `_proven` (no
  validation in release builds; the `debug_assert!` stays) and carry the
  "Cannot fail validation" doc line. Values, layout and ABI do not change.
- **`prove = [...]`.** Lists that fail today ("not eligible") may succeed,
  and the `prove` diagnostics change.
- **API.** It changes only through `prove`, under PLAN.md's Q16 (c). If
  PLAN.md settled Q16 on (b), every proof decides a return type, and this
  becomes an API change.
- **Cost.** `Form` is computed per width under `MAX_EVALUATIONS`, and holes
  add intervals to PLAN.md's §4.3 budget.

**Work:**

1. `ResolvedDomain::closure` takes its intervals from `values(width)` when
   `intervals()` finds the domain ineligible.
2. Count the extra intervals against the §4.3 budget.
3. Tests:
    - `src/unit`: extend the exhaustive 8-bit soundness sweeps
      (`src/unit/t_domain_closure.rs:871-1208`, pre-PLAN) to `Ne` atoms and
      lens atoms whose `Form` has modulus 1;
    - `src/unit`: expansion tokens for a wide type whose domain becomes
      eligible;
    - e2e: a `prove = [...]` list that fails today now expands, and an
      operation that is still not closed keeps failing with its
      counterexample.
4. Update ADR 0003.
