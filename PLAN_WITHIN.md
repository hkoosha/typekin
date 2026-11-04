# typekin: proven subdomain friendship — implementation plan

Repo: `/root/typekin` @ `462d0c3` (v0.18.0, edition 2024, `rust-version` 1.97).
File and line references below were checked against that commit.

One feature: a friend capability `Within` that lets a target build itself from a
source wrapper (or primitive) **without runtime validation**, because rustc has
proven at compile time that every value of the source's domain lies in the
target's domain. A failed proof is a compile error with a counterexample.

It closes the gap that `PLAN_COMPILE_TIME.md` §B.2 leaves open ("Cross-type
proofs … the macro sees one type"): the macro still sees one type, but each
wrapper publishes its domain as hidden constants and rustc's const evaluator
does the cross-type comparison.

---

## Implementation status

`[x]` done and verified, `[~]` in progress, `[ ]` not started.

- [x] **W0** groundwork (§6). Baseline: root 922, `test_fast` 805, and
  `test_slow` 904 tests. The `Within` fixtures themselves pin the §1.2 facts
  (proofs in hidden modules evaluated by `cargo check`, cross-module hidden
  constants, const-built messages); the cross-crate case reuses
  `literal_provider`.
- [x] **W1** exact domain evaluator (§4.1): `contains` returns `bool`.
- [x] **W2** normal form: intervals (§4.2)
- [x] **W3** normal form: residue classes (§4.2)
- [x] **W4** source-side constants (§5.1)
- [x] **W5** `Within` grammar and expansion-time diagnostics (§3)
- [x] **W6** target-side proof and construction (§5.2–§5.4)
- [x] **W7** tests (§7). After: root 1007, `test_fast` 817, and `test_slow`
  935 tests (995 with `test_slow_wasm32` and `test_slow_cross`).
- [x] **W8** example and documentation (§8): `my_within`, re-expanded
  examples, README, `lib.rs`, `AGENTS.md`, and `PHILOSOPHY.md`.

Where the implementation departs from the text below, written against an
older tree:

- Modules stay flat: `src/zz_domain_within.rs` (forms),
  `src/zz_domain_within_const.rs` (the subset check), and
  `src/integral_within.rs` (source constants, planning, proofs, `_within`).
- The subset check has one implementation, not two (§5.3): the
  `zz_domain_within_const.rs` code is plain stable `const fn` over
  sign-magnitude values, which typekin compiles for primitive sources and emits
  verbatim for rustc. Its counterexample is the least failing member.
- A wrapper with a visible value field publishes every raw value as its form,
  and the raw accessor reads the field, so it is always `const`.
- A generic target forces its proof in the friend's seal conversion, which
  `Make`, `Rel`, and `Math` all use, rather than in `make` alone.
- Tests follow the testing crate's layout: `test_within.rs`/`fx_within.rs`,
  cases in `fx_compile_fail.rs`, `fx_compile_pass.rs`, and
  `fx_pointer_width_compile.rs`, and the asm case `asm_case_within.rs` in the
  shared assembly crate. `fixture.rs` gained `compile` (`cargo build`); its
  negative control is the same generic case accepted by `cargo check`.

---

## 0. TL;DR

```rust
#[typekin::integral(domain = ..=100)]
pub struct Percent(u8);

#[typekin::integral(domain = 1..=1023 | 49_152..)]
pub struct Port(u16);

#[typekin::integral(domain = ..=255, friend = [_(Percent) -> Within, _(u8) -> Within])]
pub struct Byte(u16);
// Byte::of(percent) and Byte::of(7u8) compile to a plain widening cast:
// no validation, no panic path, no `unsafe`.

#[typekin::integral(domain = 1.., friend = _(Byte) -> Within)]
pub struct Positive(u16);
// error[E0080]: evaluation panicked: `Byte` is not within `Positive`:
//               0 is outside the domain of `Positive`

#[typekin::integral(domain = mul(6) & ..=60, friend = _(Even) -> Within)]
pub struct Sixes(u32);
#[typekin::integral(domain = mul(2))]
pub struct Even(u32);
// error[E0080]: … `Even` is not within `Sixes`: 2 is outside the domain of `Sixes`
```

Proof tiers, chosen per edge inside rustc:

| Tier | When | How |
| --- | --- | --- |
| Table | source raw type is `u8` or `i8` and its validation is const-callable | rustc evaluates source and target validators for all 256 raw values; covers callbacks and every atom |
| Form | both domains have a structural normal form | interval-plus-residue subset check in const code |
| none | otherwise | compile error naming the blocking atom or callback |

Exhaustive enumeration is limited to 8-bit sources (decision DW1).

---

## 1. Evidence (probes, 2026-10-31, x86_64)

All probes used throwaway crates, deleted afterwards. W0 re-establishes each
fact as a permanent compile test before code relies on it.

### 1.1 The gap today

A friend's `Make` impl routes to exactly two constructors
(`src/friendship.rs:549-555`):

| Capability | Constructor | Consequence |
| --- | --- | --- |
| no `Unsafe` | `Self::_or_panic` (`src/integral.rs:1981-1982`) | runtime validation and a panic path, even when the edge cannot fail |
| `Unsafe` | `Self::_blind_trust` (`src/integral.rs:1983-1984`, `5334-5356`) | no check; the friend author's proof is unchecked |

`fix_friendship` removes `Make` from primitive friends once the target has
validation (`src/integral.rs:1621-1642`), so `Byte` cannot accept a `u8`
infallibly today either.

### 1.2 rustc as the prover

| Probe | Toolchain | Result |
| --- | --- | --- |
| `const _: () = assert!(subset(A::DOMAIN, B::DOMAIN))`, valid and invalid pair | stable | valid passes, invalid is E0080 in **`cargo check`** |
| `const _` nested inside typekin's `const _: () = { … };` wrapper, inside a `hide_in` module | 1.97 | evaluated by `cargo check` (E0080 reported) |
| `#[doc(hidden)] pub const` on a type in `mod __percent`, read from `mod __byte` | 1.97 | resolves through the re-export |
| `panic!("{}", text)` where `text: &str` is built in const from a byte buffer via `core::str::from_utf8` (decimal counterexample) | 1.97 | E0080 `evaluation panicked: \`Byte\` -> \`Port\`: 0 is outside \`Port\`` |
| exhaustive loop over 65,536 values calling two const validators (popcount, `rem_euclid`, `%`) | stable | `cargo check` 0.22 s |
| same over 2²⁰ values | stable | stopped by the deny-by-default `long_running_const_eval` lint after 2.35 s |

So: compile-time proofs are visible to editors, counterexamples can be formatted
on the MSRV, and enumeration is only practical for small sources.

### 1.3 What the proc macro already has

- `ResolvedDomain { subject: N, per_width: [RNode; 3] }`
  (`src/zz_domain.rs:1114-1117`), one normalized tree per pointer width in
  `POINTER_WIDTHS = [16, 32, 64]` (`src/zz_value_type.rs:12`).
- Leaf kinds after normalization: `Interval`, `Ne`, `Divisible { negated }`,
  `Predicate`, under `Not`/`And`/`Or`/`Const` (`src/zz_domain.rs:1083-1108`).
  Lenses: `UnsignedAbs` (also `abs`), `AbsDiff`, `Signum`, `CountOnes`,
  `CountZeros`, `LeadingZeros`, `TrailingZeros`, `LeadingOnes`, `TrailingOnes`,
  `RemEuclid`, `DivEuclid`, `Isqrt` (`:819-832`).
- `ResolvedDomain::contains(value, width) -> Option<bool>` (`:1826-1833`) is
  exact except that most lenses are modeled only at zero (`lens_value`,
  `:1761-1814`).
- Conservative structural proofs exist as precedent: `symmetric` and
  `abs_closed` (`:1991-2035`), feeding `closed_result`
  (`src/integral.rs:5086-5095`).
- `Int` is sign-magnitude `u128` with total order, `successor`, `predecessor`,
  and no arithmetic (`src/zz_value_type.rs:464-545`).

---

## 2. Non-negotiables

- No path may create an invalid wrapper. A `Within` edge exists only when a
  proof succeeded at compile time; otherwise the crate does not compile.
- Construction after a proof uses the **safe** storage path
  (`storage_expression(raw, false)`, `src/integral.rs:802-812`): a wrong proof
  can break typekin's semantic invariant but cannot reach `new_unchecked`.
- Proven paths keep `debug_assert!(#validation)` (same rule as
  `PLAN_COMPILE_TIME.md` DB2).
- No new runtime dependency, allocation, `std`, `unsafe` in macro output, or
  dynamic dispatch. Hidden items add no runtime code unless used.
- Constness follows `konst`/`konst_*`; typekin never drops `const`.
- Unsupported combinations fail during expansion at the relevant span; proofs
  that need the other type fail in rustc with a typekin-authored message.
- Typekin exports no runtime crate: every helper the const proof needs is
  emitted at the target site.

---

## 3. Surface (W5)

### 3.1 Grammar

`Within` is a new integral friend capability, valid at the root `friend` list
and in a bitflag's nested `integral = [...]`.

```text
friend = _(Source) -> Within
friend = _(Source) -> [Within, Rel]
friend = self(u8) -> Within          // primitive source; `_(u8)` is equivalent
```

- `Within` implies `Make`, exactly as `Unsafe` does
  (`src/friendship.rs:163-165`), and changes only how `Make` constructs. `Math`,
  `Bit`, and `Rel` keep their semantics; operator results are still validated.
- The conversion is fixed: the source's raw value, cast with `as` to the
  target's raw type. The cast is lossless because the proof includes the
  target's raw range.
- Accepted sources: any `integral` wrapper (including generic instantiations
  such as `Tagged<In>` and the target's own generic parameters), a bitflag
  `{Enum}Value`, or a primitive integer.

### 3.2 Expansion-time diagnostics (at the friend span)

| Input | Message |
| --- | --- |
| explicit conversion `f(Percent) -> Within` | `` `Within` converts through the source's raw value; write `_(Percent)` `` |
| `[Within, unsafe]` | `` `Within` proves what `unsafe` trusts; use one `` |
| `_(Self) -> Within` | `` a type is always within itself; remove this friend `` |
| `Within` in a facet | existing `facet friends only support `Math` and `Bit`` (`src/integral.rs:5619-5627`) |
| `Within` on `float`/`point` over a float | `` `Within` supports integer wrappers only `` |
| `Within` on a bitflag root `friend` | `` `Within` belongs in the nested `integral = [...]` friend list `` |
| primitive source not contained (decided at expansion, §5.3) | `` `u16` is not within `Byte`: 256 is outside the domain `..=255` `` (adds `at 16-bit pointer width` when width-specific) |
| primitive source, target opaque and not table-provable | `` cannot prove `u16` within `Byte`: `valid = is_even` is opaque to subdomain proofs `` |
| source wrapper built with `without = within_source` | rustc: `no associated item named __TYPEKIN_WITHIN_V1_…` plus a typekin `note` in the README section |

`text` friends reject `Within` through the existing `unknown capability` path
(`src/friendship.rs:199-201`).

### 3.3 Const-evaluation diagnostics (E0080, at the generated proof)

Messages are built in const code (§1.2) and name both types:

```text
`Byte` is not within `Positive`: 0 is outside the domain of `Positive`
`Word` is not within `Small`: 300 does not fit `u8`
cannot prove `Wide` within `Narrow`: `Wide`'s domain uses `count_ones`,
    which is opaque to subdomain proofs
cannot prove `Wide` within `Narrow`: moduli 997 and 1009 exceed the proof
    budget
```

`rustc` points at the macro invocation of the target, which carries the friend
list.

---

## 4. Proof model

### 4.1 Exact evaluator (W1)

Make `lens_value` (`src/zz_domain.rs:1761-1814`) and `lens_or_bits`
(`:1910-1931`) exact for every lens at every value and width, replacing the
"only zero is ever probed" contract (`:1788`). Bit lenses evaluate the two's
complement pattern at `bits_for_pointer_width`. `contains` then returns `Some`
for every input; keep the `Option` return only if a caller still needs it,
otherwise make it `bool`.

This evaluator is the oracle for W2/W3 and for their tests.

### 4.2 Normal form (W2, W3)

```rust
/// Per pointer width: x ∈ set ⇔ some piece has x ≡ residue (mod modulus)
/// and lo ≤ x ≤ hi. lo and hi are members (≡ residue), pieces are sorted by
/// (residue, lo) and disjoint within a residue.
pub(crate) struct Form { modulus: u128, pieces: Vec<Piece> }
pub(crate) struct Piece { residue: u128, lo: Int, hi: Int }
pub(crate) enum FormResult { Form(Form), Opaque(Reason) }
```

Construction by **breakpoints plus residue sampling**, per width index:

1. Walk the `RNode`; every leaf contributes breakpoints (values where its truth
   may change) and a period. `modulus` is the lcm of all periods (`1` for pure
   interval domains: W2 handles exactly that case).
2. Sort breakpoints; consecutive breakpoints delimit segments.
3. For each segment and residue `r < modulus`, evaluate `contains` at the first
   member of the segment congruent to `r`. Within a segment every leaf is
   constant per residue class, so one sample decides the whole class.
4. Emit a piece per (segment, residue) that is in the set, then merge pieces of
   the same residue across adjacent segments.

Leaf rules (`c` is a lens-space constant from the leaf's test):

| Leaf | Breakpoints | Period |
| --- | --- | --- |
| subject interval, `{v}`, `!{v}` | `lo`, `hi + 1` / `v`, `v + 1` | 1 |
| `mul(d)`, `!mul(d)` on the subject | — | `d` |
| `is_positive`, `is_negative` | `0`, `1` | 1 |
| `is_power_of_two` | `2^k`, `2^k + 1` for every `k` (≤ 128 points) | 1 |
| `abs`/`unsigned_abs` test | `±c`, `±(c + 1)`, `0` | `d` if the test is `mul(d)` |
| `abs_diff(a)` test | `a ± c`, `a ± (c + 1)`, `a` | `d` if the test is `mul(d)` |
| `signum` test | `0`, `1` | 1 |
| `div_euclid(k)` test | `c·k`, `(c + 1)·k` | `k·d` if the test is `mul(d)` |
| `rem_euclid(m)` test | — | `m` |
| `isqrt` range or roster | `c²`, `(c + 1)²` | — |
| `leading_zeros`, `leading_ones` (any test) | every power-of-two boundary of the bit pattern (≤ 2·bits + 2) | 1 |
| `trailing_zeros`, `trailing_ones` (any test) | `0` and the all-ones value | `2^(k+1)`, `k` = the largest lens value below `bits` where the test changes |
| `isqrt` with `mul(d)`, `count_ones`, `count_zeros` | — | **opaque** |

Breakpoints outside the width's subject range are dropped; arithmetic that
overflows `Int` saturates to the range edge.

Caps (constants in the new module, enforced per width; exceeding one makes the
form `Opaque` with a reason naming the cap):

| Cap | Value | Reason |
| --- | --- | --- |
| `modulus` | ≤ 1024 | bounds the target-side loop (§4.3) |
| evaluations (segments × modulus) | ≤ 65,536 | expansion time |
| pieces after merging | ≤ 256 | size of the emitted constant |

A domain with any `valid` callback is `Opaque` (callbacks are paths, not
bodies). A missing domain is the full raw range (`modulus` 1, one piece).

### 4.3 Form subset check (emitted at the target)

Inputs: the source's `Form` (read from its constant), the target's `Form`
(literal, known at expansion), and the target raw type `T`.

1. Decode each source piece endpoint (sign-magnitude) into the work type `W`
   (`u128` if `T` is `u128`, else `i128`); an endpoint outside `T`'s range is
   the counterexample (endpoints are members, so it is real).
2. `L = lcm(source.modulus, target.modulus)`. Split each source piece into
   `L / source.modulus` progressions with step `L`; each has one residue modulo
   `target.modulus`.
3. Walk each progression against the target pieces of that residue, jumping from
   the end of a covering piece to the next progression point (`checked_add`;
   overflow ends the walk). The first uncovered point is the counterexample.
4. A step counter guards `L/m_s · |S| · |T| ≤ 65,536`; exceeding it panics with
   the budget message, keeping well below the `long_running_const_eval`
   threshold measured in §1.2.

### 4.4 Table tier

Only for sources whose raw type is `u8` or `i8` (DW1).

- The source publishes `[bool; 256]`: entry `i` is its full validation (domain
  and callbacks) at raw `i` (`u8`) or `i - 128` (`i8`), computed by a const
  block from the same `ValidationCfg::emit` expression its constructors use
  (`src/zz_lib.rs:371-389`).
- The target checks each admitted value: first against its raw range (bounds
  clamped to `-128..=255`, emitted as literals), then against its own validation
  expression emitted inline with `value: T`.
- Emitted only where the validation is const-callable: a domain alone always is;
  callbacks are when `try_make` or the validating helpers are const
  (`konst_for(&[Konsts::TryMake])` with `fn_try_make`, or `helper_konst()`,
  `src/integral.rs:6219-6223`), because rustc already requires the callbacks to
  be const there.

### 4.5 Tier selection (inside the generated const)

```text
if source table is Some and target validation is const-callable  -> Table
else if source form is Some and target form is Some             -> Form
else panic: reason from source (constant) and/or target (literal)
```

Table is preferred when available: it is exact by construction and does not
depend on the analyzer.

### 4.6 Pointer widths

Forms of `usize`/`isize` wrappers are emitted under
`#[cfg(target_pointer_width = "…")]` per width, as `ResolvedDomain::emit`
already does (`src/zz_domain.rs:1963-1988`). rustc checks only the width it
compiles for; `test_pointer_width_compile.rs` covers the others (§7).

---

## 5. Emission

### 5.1 Source side (W4)

Every integral wrapper, including bitflag value types (through
`integral::ekran_with`, `src/bitflag.rs:265-280`), gets one hidden inherent
impl. Names are versioned so a mismatch between typekin versions across crates
fails to resolve instead of being misread.

| Item | Type | Present when |
| --- | --- | --- |
| `__TYPEKIN_WITHIN_V1_RAW` | `(bool, u32)`: signed, bits (per width) | always |
| `__TYPEKIN_WITHIN_V1_FORM` | `Option<(u128, &'static [(u128, bool, u128, bool, u128)])>`: modulus, pieces `(residue, lo_negative, lo_magnitude, hi_negative, hi_magnitude)` (per width) | always; `None` when opaque |
| `__TYPEKIN_WITHIN_V1_WHY` | `&'static str`: `` `Wide`'s domain uses `count_ones`, … `` | always; `""` when the form exists |
| `__TYPEKIN_WITHIN_V1_TABLE` | `Option<[bool; 256]>` | raw `u8`/`i8` and const-callable validation; `None` otherwise |
| `__typekin_within_v1_raw(self) -> Raw` | `const fn` iff the raw getter is | the wrapper has a raw getter (`fn_conv_raw` or `get_raw`) |

- All items are `#[doc(hidden)] pub`; the inherent impl travels with the
  re-exported type, so `zz_hide::wrap` (`src/zz_hide.rs:192-217`) needs no
  change, and a `pub` item cannot exceed the type's own visibility.
- Generic wrappers emit them in the generic inherent impl.
- New flag `within_source` (default on) in `IntegralFlags`
  (`src/integral.rs:6490-6596`); `without = within_source` omits all five items,
  e.g. for a wrapper that must not hand its raw value to friends.
- Code: new `src/zz_domain/within.rs` (form construction, caps, reasons, token
  emission for §5.1 and §5.2), included from `src/zz_domain.rs`; `Int` helpers
  (`gcd`, `lcm`, checked add/sub/mul, `square`) in `src/zz_values.rs`, shared
  with `PLAN_COMPILE_TIME.md` B1.

### 5.2 Target side (W6)

For each `Within` friend `S`:

- **Proof.** Non-generic target and source: statements emitted directly into the
  outer `const _: () = { … };` block (`src/integral.rs:303-312`), like the
  layout assertions (`:867-891`). Generic: an associated
  `const __TYPEKIN_WITHIN_PROOF_<n>: () = { … };` in the generic inherent impl,
  forced from the friend's `make` with `let () = …;`, following
  `__TYPEKIN_LAYOUT_ASSERT` (`:614-616`, `3731-3737`). Generic proofs run at
  monomorphization (`cargo build`), as documented for the literal macro's inline
  form.
- **Proof body.** A local `const fn` returning `Option<counterexample>`
  implementing §4.3–§4.5 over `W`, plus a message builder writing decimals into
  a `[u8; 160]` buffer and `core::str::from_utf8` (§1.2).
- **Construction helper.**
  `#[doc(hidden)] pub(self) fn _within(raw: #el) -> Self`, `const` iff
  `helper_konst()`, body `debug_assert!(#validation); return #constructed;` with
  `construct_blind_trust`'s storage path (`src/integral.rs:618-680`). Emitted
  beside `_blind_trust` in `make_fn_make` (`:5285-5359`) when any friend has
  `Within`.
- **Routing.** `friendship::Construction` gains `within_capability` and
  `within`; the `make` body at `src/friendship.rs:549-561` picks `_within` for
  friends holding the capability. `Within` itself gets no capability trait
  (filtered where `ekran_friendship` builds them, `src/integral.rs:1886-1909`).
- **Conversion.** New branch in `friend_conversion_expression`
  (`src/integral.rs:1146-1188`): wrapper source →
  `<S>::__typekin_within_v1_raw(#value) as #el`; primitive → `#value as #el`.
- **Docs.** The `Make` impl for `S` carries the doc line "Cannot fail
  validation: every value of S lies in the domain of T (proven by rustc at
  compile time)."

### 5.3 Primitive sources

The proc macro knows both sides, so it decides at expansion: the source's form
is its full range per width. Structural target → compare `Form`s in the proc
macro (same algorithm as §4.3, implemented once over `Int` in `within.rs`) and
report the counterexample at the friend span; no const is emitted. Opaque target
with an 8-bit primitive and const-callable validation → emit the Table check
(all 256 entries true). Otherwise → the expansion error of §3.2.

The emitted const check in §5.2 and the expansion-time check share one algorithm
but two implementations (Rust over `Int`, emitted tokens over `W`). W7's
differential tests pin them to each other and to `contains`.

### 5.4 Bitflag

- Nested `integral = [friend = …]` goes through `integral::Cfg` parsing
  (`src/bitflag.rs:1869-1906`), so `Within` works for value-type targets with no
  new code beyond W5.
- Value types publish §5.1 items, so masks such as `domain = ..=0b111` are valid
  sources.
- Root bitflag friends (`src/bitflag.rs:1852-1855`) reject `Within` (§3.2).

---

## 6. Phases and code locations

- **W0 groundwork.**
  - Baseline: root `cargo test` and `typekin_testing --features test_slow`;
    record counts here.
  - Permanent compile tests (cases in the fixture crates of
    `typekin_testing/src/fixture.rs`) for the §1.2 facts: nested `const _`
    evaluated by `cargo check` inside a hidden module; cross-module hidden
    constant; const-built panic message.
  - Cross-crate fixture: `write_crate` plus a `../<crate>` path dependency, as
    `typekin_testing/src/test_compile_pass.rs` does for literal macros.
- **W1** `src/zz_domain.rs:1761-1932`: exact lenses; tests in §7.
- **W2** `src/zz_domain/within.rs`: `Form` with modulus 1; leaf rules without
  periods; caps; `Reason`.
- **W3** periods and residue sampling; `Int` helpers.
- **W4** source emission in `src/integral.rs` beside `make_fn_make_try`
  (`:5223-5270`); `within_source` flag; table constness rule (§4.4).
- **W5** `WITHIN` beside `src/integral.rs:87-91`; valid set and `Within → Make`
  in `Cfg::parse_friends` (`:6225-6264`); §3.2 diagnostics; bitflag root
  rejection.
- **W6** §5.2–§5.4.

W1–W3 have no consumer-visible effect and can land first. W4 changes every
expansion (hidden items), so re-expand all examples in W8.

---

## 7. Tests (W7)

`src/tests/t_domain.rs` (W1), one named `#[test]` per lens, each enumerating
every `u8`, `i8`, `u16`, and `i16` value at every pointer width and comparing
`contains` with Rust's own method (`count_ones`, `leading_zeros`, `rem_euclid`,
…) applied to the value.

`src/tests/t_domain_within.rs` (W2/W3), one named `#[test]` each:

- Differential soundness: for each leaf rule in §4.2 and for mixed domains
  (`(1..12 | 87..) & !{10} & !mul(3)`, `(abs in ..=100) & !mul(7)`,
  `(trailing_zeros in 3..) | (leading_ones in {2})`, …), enumerate every 8- and
  16-bit value and assert form membership equals `contains`.
- Wide types: at every breakpoint ±1 and at `MIN`, `MAX` for `u64`, `i64`,
  `u128`, `i128`, form membership equals `contains`, without overflow.
- Opaque: `count_ones`, `isqrt in mul(2)`, a `valid` callback, and each cap,
  each with its reason text.
- Expansion-time primitive check: `u8` within `..=255` proven; `u16` within
  `..=255` refuted with counterexample 256; `usize` refuted only at 64 bits
  reports the width.
- Tokens: a `Within` friend's `make` calls `_within`, not `_or_panic`; `_within`
  contains `debug_assert!` and no `unsafe`; `without = within_source` emits no
  `__TYPEKIN_WITHIN_V1_` item.
- Grammar: one test per §3.2 diagnostic.

`typekin_testing/src/test_within.rs` (consumer):

- Proven edges compile and convert correctly, enumerating every source value:
  `Percent → Byte`, `Port → Positive16`, `i8` table source with a const callback
  into a wider target with a callback, `mul(6) → mul(2)`, `NonZero` source into
  `NonZero` target, bitflag value mask into a nested-integral target, generic
  `Tagged<In>` source, generic target.
- `Within` combined with `Rel` and `Math`: results still validated (a `Math`
  result outside the target domain panics).

`typekin_testing/src/test_compile_fail.rs` via `assert_rejected`:

- Form refutation with counterexample; raw-range refutation; table refutation
  for an `i8` source with a callback; opaque source reason; budget reason;
  generic edge fails at `cargo build` instantiation (`compile` with build, not
  check).
- Cross-crate source through the W0 dependency fixture: proven and refuted.

`test_pointer_width_compile.rs`: a `usize` source within a `u32` target compiles
for a 32-bit target and is rejected for `x86_64`.

Asm (`test_slow`): new bin fixture `within` (`fixture_within.rs`, registered in
`typekin_testing/Cargo.toml` like `domain_int`) with `raw_*` and `wrapped_*`
pairs for `Percent → Byte`, `Port → Positive16` (`NonZero` storage on both
sides), and a primitive `u8 → Byte`; `Expect::Match` at O1/O2/O3/Os/Oz,
`DiffAny` at O0, and an untrusted friend of the same pair as `DiffEach` negative
control.

---

## 8. Example and documentation (W8)

- `just new within within` → `examples/my_within.rs` (+ `_exp.rs`, `[[example]]`
  entries, recipe); then `just all` to re-expand every example (§6).
- README: section "Proven friendship (`Within`)" after "Blind-trust friend
  construction", and the capability list at `README.md:761-765`. `AGENTS.md`
  restricts README edits to explicit requests: confirm before editing.
- `PHILOSOPHY.md` "Validation Belongs on Every Generated Ingress": a proven
  `Within` edge is the "statically incapable" option across types, not a bypass.
- `AGENTS.md`: `Within` in the capability rules (`AGENTS.md:121-127`,
  `140-142`); a soundness rule beside the zero-probe rule ("form, table, and
  `contains` must agree; keep the differential tests exhaustive"); repository
  map entry for `src/zz_domain/within.rs`.

---

## 9. Decisions

| ID | Decision | Rationale |
| --- | --- | --- |
| DW1 | Exhaustive table only for `u8`/`i8` sources | user decision; 2¹⁶ is cheap (§1.2) but excluded to keep compile cost flat |
| DW2 | A friend capability, not a new key | reuses friend parsing, `of`, `Make` routing; `Within` reads as "source is within target" and avoids `core::ops::Sub` |
| DW3 | Safe storage path after the proof | a proof bug cannot become UB |
| DW4 | `debug_assert!` on proven construction | safety net, free in release |
| DW5 | Prefer the table tier when available | exact by construction, independent of the analyzer |
| DW6 | Versioned hidden items, `without = within_source` opt-out | cross-crate mismatch fails loudly; wrappers can refuse to publish their raw value |
| DW7 | `of` only; no generated `From` | friendship is not `From` (`PHILOSOPHY.md`); callers can write a one-line `From` that calls `of` |
| DW8 | Integers only | float rounding and NaN need a separate design, as in `PLAN_COMPILE_TIME.md` §B.2 |

## 10. Rejected alternatives

- **Exhaustive enumeration for 16-bit or larger sources**: DW1; 2²⁰ already
  trips `long_running_const_eval`.
- **Proc macro reading the source's tokens**: it cannot see other items; a copy
  of the source domain in the target's configuration would drift.
- **`unsafe` friend plus documentation**: that is the status quo the feature
  replaces.
- **Runtime check with `#[cold]` panic**: still a branch and a panic path; the
  asm fixtures would stay `DiffEach`.
- **Generated narrowing `TryFrom` in the other direction**: expected rejection
  belongs to user-written `TryFrom` (`PHILOSOPHY.md`).

## 11. Relation to `PLAN_COMPILE_TIME.md`

- Part B lists cross-type proofs as a non-goal; this plan covers them.
- W1 (exact evaluator) and the `Int` helpers serve B1 as well; B1's
  `ResolvedDomain::intervals` is `Form` with modulus 1. Whichever lands first
  owns them.
- Part A is independent; `port!(…)` values are ordinary wrappers and work as
  `Within` sources.
