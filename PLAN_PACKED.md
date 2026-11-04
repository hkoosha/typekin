# typekin: packed records — implementation plan

Repo: `/root/typekin` @ `462d0c3` (v0.18.0, edition 2024, `rust-version` 1.97).
File and line references below were checked against that commit.

A new attribute macro `#[typekin::packed]` turns a named-field struct into a
transparent integer wrapper whose fields are typed bit windows. Every field is
already a valid value of its own type, so construction from fields and per-field
updates are infallible and never validate; only raw ingress (`try_make`, Serde)
validates.

Depends on `PLAN_WITHIN.md` (W4 source items and the W6 proof emitter). Design
choices confirmed by the user on 2026-10-31: named-field struct, explicit bit
ranges, a generated `Values` struct, and re-checking getters (§9).

---

## Implementation status

`[x]` done and verified, `[~]` in progress, `[ ]` not started.

- [ ] **K0** groundwork (§6)
- [ ] **K1** entry point, parsing, and layout checks (§3)
- [ ] **K2** integral configuration and raw validation (§5.1)
- [ ] **K3** `Values` struct and accessors (§5.2)
- [ ] **K4** window proofs and `non_zero` (§4)
- [ ] **K5** `Debug`, `Hash`, `Eq`, Serde (§5.3)
- [ ] **K6** tests (§7)
- [ ] **K7** example and documentation (§8)

---

## 0. TL;DR

```rust
#[typekin::integral(domain = 1..)]
pub struct PageId(u8);

#[typekin::integral(domain = ..64)]
pub struct Version(u8);

#[typekin::bitflag(repr = u8)]
pub enum PageState { Dirty = 0b01, Pinned = 0b10 }

#[typekin::packed(repr = u32, with = non_zero)]
pub struct PageHeader {
    /// Page number; 0 is reserved.
    #[bits(24..32)] pub id: PageId,
    #[bits(16..24)] pub state: PageStateValue,
    #[bits(10..16)] pub version: Version,   // 6 bits: `..64` is proven to fit
    #[bit(9)]       pub sealed: bool,
    // bit 8 unclaimed: always zero
    #[bits(0..8)]   pub data: u8,
}

fn demo(id: PageId, version: Version) {
    let header = PageHeader::make(PageHeaderValues {
        id,
        state: PageStateValue::Dirty,
        version,
        sealed: false,
        data: 0xFF,
    });                                                   // infallible, no validation
    let id: PageId = header.id();                         // typed read
    let header = header.with_state(PageState::Pinned.into_value());  // no validation
    let PageHeaderValues { data, .. } = header.fields();
    assert_eq!(data, 0xFF);
    assert!(PageHeader::try_make(0x0000_00FF).is_err());  // id bits 0
    assert!(PageHeader::try_make(0x0100_0100).is_err());  // unclaimed bit 8 set
    assert_eq!(size_of::<Option<PageHeader>>(), 4);       // proven via `id`
}
```

---

## 1. Evidence

### 1.1 The gap today

The README's flagship friendship example (`README.md:73-125`,
`examples/my_friend.rs`) packs `PageId`, `PageState`, and `PageData` into
`PageHeader(u32)` through hand-written shift functions and `|=` friends. It
documents the layout in a comment (`^ID......^ ^STATE.^ ^UNUSED^ ^DATA..^`),
cannot read a field back as its type, does not keep the unused byte zero, and
its sources are plain structs because a validated source would panic-check on
every `|=`. No bitfield or packing code exists in `src/` (searched for
`bitfield|packed|pack|layout`).

### 1.2 Reusable machinery

- `bitflag` already builds a wrapper by handing a configured `integral::Cfg` to
  `integral::ekran_with(ty, el, cfg, StoragePlan)` (`src/integral.rs:341-359`;
  caller `src/bitflag.rs:265-280`) and hides two exported types through
  `zz_hide::wrap` (`src/bitflag.rs:123-139`).
- `IntegralFlags` (`src/integral.rs:6490-6596`) switches off every operator,
  conversion, and formatting group individually; `RESTRICTED_MATH_FLAGS`
  (`:6443-6470`) is the profile precedent for a restricted surface.
- `ValidationCfg.callbacks: Vec<Path>` (`src/zz_lib.rs:362-367`) is ANDed into
  every constructor; a generated `const fn` can be one of them.
- `PLAN_WITHIN.md` W4 publishes per wrapper `__typekin_within_v1_raw`, the
  domain form, and the 8-bit table; W6 emits a const proof that a source lies
  inside a target set. A bit window is such a target set: one interval.

---

## 2. Non-negotiables

- No path may create an invalid record or an invalid field value. Raw ingress
  validates every field and the unclaimed bits; `make` and `with_*` accept only
  already-valid field values.
- No bit is ever silently truncated: a field type that may not fit its window is
  a compile error, not a mask.
- No new runtime dependency, allocation, `std`, `unsafe` in macro output, or
  dynamic dispatch. Accessors are shifts and masks.
- Constness follows `konst`/`konst_*`; typekin never drops `const`.
- Unsupported combinations fail during expansion at the relevant span.

---

## 3. Surface (K1)

### 3.1 Item

- A non-generic struct with named fields, each carrying exactly one of
  `#[bits(LO..HI)]`, `#[bits(LO..=HI)]`, or `#[bit(N)]`. Bit 0 is the least
  significant bit of the raw value.
- Field visibility and doc comments carry over to the getter, the `with_`
  setter, and the `Values` struct field.
- Other field attributes are rejected (the struct is rewritten; derives and
  `serde` attributes on fields would not apply).

### 3.2 Configuration

| Key / flag                                       | Meaning                                                 | Default           |
|--------------------------------------------------|---------------------------------------------------------|-------------------|
| `repr = uN`                                      | backing unsigned primitive (`u8` … `u128`, not `usize`) | required          |
| `konst`                                          | `none`, `stable`, `nightly`, as for integral            | `stable`          |
| `hide_in`                                        | as for integral                                         | `__` + snake name |
| `values_name = Ident`                            | name of the generated values struct                     | `{Type}Values`    |
| `with = non_zero`                                | `NonZero` storage, proven (§4.2)                        | off               |
| `with = ord`                                     | `Ord`/`PartialOrd` over the raw value                   | off               |
| `with = serde`                                   | raw-integer Serde, validating on deserialize            | off               |
| `without = debug \| eq \| hash \| values_struct` | omit those items                                        | on                |

Everything else integral accepts (`domain`, `valid`, `friend`, `profile`,
`facet`, `produce`, `tree`, `math`, operator flags) is rejected with
`` `packed` does not support `KEY` ``. `usize` is rejected because the layout
would depend on the target.

### 3.3 Field types

| Type                                                                      | Window                                          | Read                                       | Write                     |
|---------------------------------------------------------------------------|-------------------------------------------------|--------------------------------------------|---------------------------|
| `bool`                                                                    | exactly 1 bit (`#[bit(N)]`)                     | `bit != 0`                                 | `as`                      |
| primitive integer                                                         | exactly its own bit width                       | cast (sign-preserving)                     | cast                      |
| typekin integral wrapper, bitflag `{Enum}Value`, or another `packed` type | any width that provably holds its domain (§4.1) | `Field::try_make(bits)`, unreachable `Err` | `__typekin_within_v1_raw` |

Every field uses the natural encoding: the window holds the low `w` bits of the
field's raw value, and a read restores the raw value as `as` widens it,
zero-extending unsigned raw types and sign-extending signed ones. Encodings are
a closed set built into typekin (DK9), and v1 provides only natural; an encoding
the set lacks (BCD, Gray, sign-magnitude) is modeled by a field type whose raw
value is the encoded pattern. Floats, `text`, and arbitrary types are rejected:
the first two by name at expansion, others by rustc's missing hidden items
(documented).

### 3.4 Expansion-time diagnostics

| Input                                                                                            | Message (at the span)                                                   |
|--------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------|
| missing or duplicate position attribute                                                          | `` each field needs one `#[bits(..)]` or `#[bit(..)]` ``                |
| anything besides the bit range in `#[bits(..)]` or `#[bit(..)]`                                  | `` `#[bits(..)]` takes only a bit range ``                              |
| empty, reversed, or past-`repr` range                                                            | `` bits 30..34 exceed `u32` (32 bits) ``                                |
| overlap                                                                                          | `` bits 20..26 of `state` overlap bits 24..32 of `id` ``                |
| `bool` wider than 1 bit                                                                          | `` `bool` takes exactly one bit; use `#[bit(N)]` ``                     |
| primitive not matching its width                                                                 | `` `u8` needs exactly 8 bits; use a wrapper with a narrower `domain` `` |
| field name colliding with a generated item (`raw`, `make`, `fields`, `try_make`, `with_…` clash) | `` field `raw` collides with the generated `raw()` ``                   |
| generics, tuple or unit struct, enum                                                             | `` `packed` needs a non-generic struct with named fields ``             |

### 3.5 Const-evaluation diagnostics

Built with the `PLAN_WITHIN.md` §4.3 message builder:

```text
`Version` does not fit bits 10..16 of `PageHeader`: 64 needs more than 6 bits
cannot prove `non_zero` for `PageHeader`: every field admits zero
```

---

## 4. Proofs (K4)

### 4.1 Field fits its window

For each wrapper field `F` with window width `w`, emit a `Within` proof with `F`
as source and the window as target set: `0..=2^w - 1` for unsigned raw types,
`-2^(w-1)..=2^(w-1) - 1` for signed ones, target raw type `F`'s raw type. This
reuses the W6 emitter unchanged except that the target form is the window
instead of a wrapper domain. A window equal to `F`'s raw width needs no proof
and emits none.

### 4.2 `with = non_zero`

Record raw zero means every field reads zero. Emit a const check that at least
one field's set excludes zero (its form has no piece containing 0, or its table
has `false` at 0); `bool` and primitives admit zero. On success the integral
storage plan uses `NonZero` (`StoragePlan`, `src/integral.rs:719-724`), and
`make` stays infallible because the field value already excludes zero.

Because the storage type must be chosen at expansion while the proof runs in
rustc, `non_zero` is opt-in rather than inferred.

### 4.3 Getter cost

Getters construct through `Field::try_make` and treat `Err` as unreachable
(decision DK4). The check is present in the source; whether it costs anything is
up to LLVM:

- Window width equal to the field's raw width and an unvalidated field: no check
  exists.
- Interval domains whose window bound implies validity (`Version` in 6 bits):
  the masked value's known range folds the comparison. Asm fixtures require
  `Match` (§7).
- Domains the window does not imply (`PageId`'s `1..` in 8 bits) or non-interval
  domains: one compare and a cold panic remain. Documented as the price of not
  adding a validation-skipping path to field types.

---

## 5. Emission

### 5.1 Integral core (K2)

`src/packed.rs` (new) builds an `integral::Cfg` the way bitflag does
(`integral::Cfg::without_konst()` then `apply_konst`,
`src/bitflag.rs:1812-1832`) and calls
`integral::ekran_with(ty, repr, cfg, storage)`:

- Kept integral items: storage and layout assertions, `try_make`, `raw`,
  `Clone`/`Copy`, `repr(transparent)`, `PartialEq`/`Eq`, `Hash`, opt-in `Ord`
  and Serde, the W4 within-source items (so a packed type can be a field of
  another packed type, or a `Within` source).
- Disabled: every `math_*`, `assign_*`, `self_math`, `self_bit`, `bit_access`,
  `core_int`, `map`, `into`, `try_into`, conversions, `from_str`, `display`,
  `fmt_*`, the integral `Debug`, and friend generation.
- Validation: one callback, a generated
  `const fn __typekin_packed_valid(raw: uN) -> bool` in the hidden module:
  unclaimed bits are zero, and every wrapper field's bits pass
  `F::try_make(..).is_ok()`. Its constness follows the record's `konst`; rustc
  reports a field type whose `try_make` is not const.

### 5.2 Values struct and accessors (K3)

```rust
#[derive(Clone, Copy)]                    // + Debug/PartialEq/Eq/Hash per flags
pub struct PageHeaderValues {
    pub id: PageId, /* … */
}

impl PageHeader {
    pub const fn make(values: PageHeaderValues) -> Self;      // shifts and ors
    pub const fn fields(self) -> PageHeaderValues;
    pub const fn id(self) -> PageId;                          // per field
    pub const fn with_id(self, id: PageId) -> Self;           // clear, then or
}
```

- `make` and `with_*` skip the validator: every input is a valid field value,
  and the window proof (§4.1) guarantees no bits spill. They build storage
  through `IntegralTarget::construct_blind_trust`'s safe path
  (`storage_expression(raw, false)`, `src/integral.rs:802-812`), so a bug cannot
  reach `new_unchecked`, and emit `debug_assert!(__typekin_packed_valid(raw))`
  as the safety net (`PLAN_WITHIN.md` DW4).
- `values_name` sets the struct name; `without = values_struct` removes it,
  `make`, and `fields`, keeping getters and setters.
- `zz_hide::wrap` exports the record and the values struct (pattern of
  `src/bitflag.rs:133-138`).

### 5.3 Traits (K5)

- `Debug`: `PageHeader { id: PageId(5), state: …, … }` through the getters.
- `PartialEq`/`Eq`/`Hash`: over the raw value (field-wise equality is identical
  because the representation is canonical: unclaimed bits are zero).
- Serde: the raw integer, deserialized through `try_make`.

---

## 6. Phases and code locations

- **K0** baseline test counts; confirm `PLAN_WITHIN.md` W4 and W6 have landed;
  generalize the W6 proof emitter to accept an interval target set with a
  caller-supplied message prefix.
- **K1** `src/lib.rs`: `#[proc_macro_attribute] pub fn packed` beside `bitflag`
  (`src/lib.rs:408-412`), through `zz_runner::catching`; `src/packed.rs`: `Cfg`
  parser (reusing `Konst`, `hide_in`, flag parsing from `zz_runner`), item and
  field-attribute parsing, §3.4 checks.
- **K2** §5.1.
- **K3** §5.2.
- **K4** §4.1, §4.2.
- **K5** §5.3.

---

## 7. Tests (K6)

`src/tests/t_packed.rs`, one named `#[test]` each:

- One per §3.4 diagnostic and per rejected configuration key.
- Tokens: no `math_`/operator impls; `make` and `with_*` contain no call to
  `__typekin_packed_valid` outside `debug_assert!`; no `unsafe`; one window
  proof per narrowed wrapper field and none for full-width fields.

`typekin_testing/src/test_packed.rs` (consumer):

- Round trip for every field value of small windows: `make` then getter returns
  the input; `with_x` changes only `x`'s window (all other bits compared before
  and after).
- `try_make` agrees with a hand-written predicate for every `u16` raw value of a
  `u16` record with a validated field, a signed field, a `bool`, and a gap.
- Signed field sign extension at window boundaries.
- Nested packed record as a field; bitflag value as a field.
- `non_zero`: `size_of::<Option<T>>() == size_of::<T>()`.
- Serde round trip and rejection of invalid raw input.

`test_compile_fail.rs` via `assert_rejected`: field domain too wide for its
window (counterexample message); `non_zero` without a zero-excluding field;
non-const field `try_make` under `konst = stable` (rustc E0015).

Asm (`test_slow`): fixture bin `packed` with `raw_*` (hand-written shifts and
masks) and `wrapped_*` pairs for `make`, `with_state`, `version()`, `data()`,
and `sealed()`: `Match` at O1/O2/O3/Os/Oz, `DiffAny` at O0; `id()` is `DiffEach`
against the unchecked shift as the documented cost of §4.3.

---

## 8. Example and documentation (K7)

- `just new packed packed` → `examples/my_packed.rs` with the §0 header;
  register and expand.
- README section "Packed records" after "Bitflag" (user approval required per
  `AGENTS.md`); the PageHeader friendship example stays as the friendship
  demonstration.
- `AGENTS.md`: entry point list, repository map entry for `src/packed.rs`, and
  the rules "packed fields are typed, windows are proven, unclaimed bits are
  zero, `make`/`with_*` never validate, encodings are typekin built-ins".

---

## 9. Decisions

| ID  | Decision                                                   | Rationale                                                                |
|-----|------------------------------------------------------------|--------------------------------------------------------------------------|
| DK1 | Named-field struct with field attributes (user)            | names, docs, and visibility map to accessors                             |
| DK2 | Explicit bit ranges, bit 0 least significant (user)        | matches wire-format specs; reordering fields cannot move bits            |
| DK3 | Generated `{Type}Values` struct for `make`/`fields` (user) | named arguments at the call site; no swapped same-typed arguments        |
| DK4 | Getters re-check through `F::try_make` (user)              | any field type allowed; no validation-skipping path added to field types |
| DK5 | Unclaimed bits must be zero                                | canonical representation makes raw equality field equality               |
| DK6 | `non_zero` is opt-in and proven                            | storage type is chosen at expansion, the proof needs rustc               |
| DK7 | Unsigned `repr`, no `usize`                                | layout must not depend on sign or target                                 |
| DK8 | No record-level `domain`/`valid` in v1                     | it would make `make`/`with_*` fallible                                   |
| DK9 | Encodings are built into typekin; no user codecs (user)    | a codec is opaque: no fit proof, canonical form, or erasure claim (§10)  |

## 10. Rejected alternatives

- **Truncating or masking oversized values**: silently changes data.
- **Tuple struct with `field = [...]` configuration**: loses field docs and
  per-field visibility (user choice DK1).
- **Sequential widths**: reordering fields would silently change the layout
  (user choice DK2).
- **A public hidden unchecked constructor on field types for free getters**: any
  code could call it, breaking the field type's invariant.
- **Retaining unclaimed bits**: two records with equal fields could compare
  unequal; can be added later as an explicit `gaps = retain`.
- **User-supplied `encode`/`decode` per field**: typekin cannot see into a
  codec. It could not prove that encoded values fit their window, so `make` and
  `with_*` would need a runtime check; a `decode` mapping two patterns to one
  value would make raw `Eq`/`Hash` disagree with the fields; and asm fixtures
  could not back erasure claims for user code. Encodings are typekin built-ins
  or do not exist (DK9).

## 11. Deferred to a follow-up plan

Encodings beyond natural, in this order. Both extend `#[bits(..)]` additively
(`#[bits(9..16, offset = 1980)]`), so existing layouts and their wire formats do
not change.

- **`offset = K`**: stores `v - K`, reads `bits + K`. Uses: length-minus-one
  fields, FAT years since 1980, exponent bias, 8-bit PCM (`offset = -128` on
  `i8`).
  - Fit proof: the field's domain lies in `K..=K + 2^w - 1`, one interval for
    the §4.1 emitter.
  - A bijection, so the representation stays canonical and `make`/`with_*` stay
    infallible; one add or subtract per access. A decode that wraps in the raw
    type lands below `K`, outside the domain, so raw ingress rejects it.
  - `K` is explicit, never inferred from the domain's minimum: a domain change
    would otherwise move the wire encoding (as DK2 argues for positions).
  - Ripple: §4.2 tests whether a field's domain excludes `K` (raw zero decodes
    to `K`); §5.1 validates `F::try_make(bits + K)`; new diagnostics reject
    `offset` on `bool`, bitflag values, and nested packed records (bit patterns,
    not numbers) and a `K` outside the raw type; §7 adds round trips at both
    ends of the window and an asm pair against hand-written `bits + K`.
- **`scale = s`** (with an optional `offset`): stores `(v - K) / s`, reads
  `bits * s + K`. Uses: FAT seconds / 2, sizes in 4 KiB pages.
  - Fit proof: the target set is one residue piece (modulus `s`, residue
    `K mod s`), which the W6 form check already handles (`PLAN_WITHIN.md` §4.3,
    modulus cap 1024).
  - Ripple beyond offset: a decode that wraps can land back on the progression
    (`u8`, `s = 2`, an 8-bit window: bits 0 and 128 both decode to 0), so a
    const check must require `K + s * (2^w - 1)` to fit the field's raw type,
    which only rustc knows; encoding divides by `s`; whether LLVM folds the
    getter's `%s` re-check is open until asm fixtures exist; and scaling a
    signed natural window (sign-extend, then multiply) needs its own definition.
