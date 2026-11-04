# ADR 0001: Hand-rolled bit packing vs a Serde bit-packing format

- Status: Proposed
- Date: 2026-11-04
- Related: `PLAN_PACKED.md`

## Context

`PLAN_PACKED.md` adds `#[typekin::packed]`: a named-field struct becomes a
transparent unsigned-integer wrapper whose fields are typed bit windows. Typekin
emits the shifts and masks, proves at compile time that every field fits its
window, and offers Serde on top (`with = serde`).

A consumer who already uses Serde sees two unrelated mechanisms. The question
was whether the packer could be a Serde data format instead: derive `Serialize`
and `Deserialize` on a plain struct and let a format pack the fields into an
integer. The stated concerns were the Serde dependency and the performance cost.

Options:

- **A. Hand-rolled (`packed`)**: the record's storage is the packed integer;
  typekin generates the shifts, masks, and proofs.
- **B. Serde format**: the record is an ordinary struct; a bit-packing
  `Serializer`/`Deserializer` converts it to and from an integer at the
  boundary.

## Decision

Keep bit packing hand-rolled in `packed` (option A). Serde stays an encoding
layered on the packed type through `with = serde`. Typekin does not implement
packing as a Serde data format and does not depend on Serde for it.

## Evidence

### Experiment

A throwaway crate implemented option B for the `PageHeader` of `PLAN_PACKED.md`
§0 and compared it against hand-written shifts and masks (setup, code, and
method in the appendix). The hand-written probes stand in for `packed`, which is
not implemented yet; `PLAN_PACKED.md` §7 requires its `make` and accessors to
match such code.

Instructions per probe, including every non-inlined callee, over the number of
functions:

| opt-level | pack, hand | pack, Serde | unpack, hand | unpack, Serde |
|-----------|------------|-------------|--------------|---------------|
| 0         | 24 / 2     | 471 / 20    | 84 / 4       | 1174 / 54     |
| 1         | 14 / 1     | 97 / 2      | 26 / 1       | 157 / 2       |
| 2         | 14 / 1     | 19 / 1      | 24 / 1       | 19 / 1        |
| 3         | 14 / 1     | 19 / 1      | 27 / 1       | 22 / 1        |
| s         | 14 / 1     | 80 / 3      | 26 / 1       | 173 / 6       |
| z         | 14 / 1     | 94 / 6      | 26 / 1       | 187 / 10      |

Best case for option B, with `#[inline(always)]` on every method of the format
(opt-levels 2 and 3 unchanged):

| opt-level | pack, Serde | unpack, Serde |
|-----------|-------------|---------------|
| 1         | 97 / 2      | 157 / 2       |
| s         | 94 / 2      | 149 / 3       |
| z         | 82 / 3      | 147 / 5       |

Findings:

- At opt-levels 2 and 3 the Serde path folds to straight-line code. Unpacking is
  equivalent to the hand-written version. Packing adds exactly one check,
  `cmp eax, 63; ja`, and an error return: Serde passes `serialize_u8(v)` and
  cannot know that `Version` is below 64.
- At opt-levels 1, s, and z calls remain even in the best case. At opt-level z
  the functions left out of line are serde_derive's `PageHeader::serialize` and
  `PageHeader::deserialize` and serde_core's `<u8 as Serialize>::serialize`,
  `<u8 as Deserialize>::deserialize`, `<bool as Deserialize>::deserialize`, and
  `SeqAccess::next_element`. A format author cannot annotate those.
- Typekin's erasure contract requires a match with hand-written code at
  opt-levels 1, 2, 3, and s; option B fails at 1 and s.

### Semantics

| Property                    | A: `packed`                             | B: Serde format                           |
|-----------------------------|-----------------------------------------|-------------------------------------------|
| In-memory value             | the packed `uN`                         | the unpacked struct                       |
| Bit positions               | `#[bits(..)]` / `#[bit(..)]` per field  | outside Serde's data model                |
| Narrow field fits window    | compile-time proof, infallible `make`   | runtime check, fallible packing           |
| Read one field              | shift and mask                          | decode the whole record                   |
| Update one field            | clear and or, infallible                | decode, modify, re-encode, fallible       |
| What the packer sees        | the field's bits                        | the field's Serde representation          |
| Dependencies                | none                                    | `serde` with `derive`                     |
| Erasure at opt-levels 1 / s | required by the asm fixtures            | not achieved                              |

Notes:

- **In-memory value**: only option A gives `size_of` of the integer, the
  `Option` niche with `with = non_zero`, raw `Eq`/`Hash`, atomics on the record,
  and arrays at the integer's size.
- **Bit positions**: Serde's data model carries field names and order, not
  widths or positions. The layout has to live beside the struct. A table keyed
  by field order silently moves bits when fields are reordered, which
  `PLAN_PACKED.md` DK2 rejects; the alternatives are per-field `serialize_with`
  adapters or magic newtype names.
- **What the packer sees**: `Serializer::is_human_readable` defaults to `true`,
  so a bitflag value field with default (auto) Serde emits text such as
  `"Dirty | Pinned"` into a bit packer unless the format overrides it. Every
  typekin wrapper field also needs `with = serde`.
- **Dependencies**: `serde_derive` builds on syn, quote, and proc-macro2, which
  typekin's build graph already contains, so the added compile cost is mostly
  Serde itself. Typekin's existing `with = serde` needs `serde` without its
  `derive` feature.

## Consequences

- Packing adds no runtime or build dependency; consumers without Serde pay
  nothing.
- Packed records keep the erasure guarantee at every checked opt-level, and a
  field that may not fit its window is a compile error.
- Typekin owns the bit-layout generator, its proofs, and its tests; nothing is
  reused from the Serde ecosystem.
- Consumers see two mechanisms with distinct jobs: `packed` decides the
  in-memory layout and Serde the encoding. They meet through `with = serde` on
  the packed type: the raw integer per `PLAN_PACKED.md` §3.2. A field-wise or
  auto mode, which decodes each field through its own `Deserialize` and builds
  the record through the infallible `make`, remains open.
- A Serde bit format remains a reasonable consumer-side choice when a record
  only crosses an I/O boundary, an unpacked in-memory value is acceptable, and
  fit errors may surface at runtime. Typekin does not provide one.

## Appendix: experiment

### Setup

- x86-64 Linux, `rustc 1.101.0-nightly (708c94ef0 2026-11-03)`, edition 2024.
- `serde`, `serde_core`, and `serde_derive` 1.0.229; `serde` with `derive`.
- Assembly per opt-level `L`:

  ```sh
  cargo +nightly rustc --offline --release --lib --target-dir target/oL -- \
    --emit=asm -Copt-level=L -Cllvm-args=-x86-asm-syntax=intel
  ```

- Count: the probe's instructions plus those of every function in the same
  assembly file it references, transitively; labels and directives excluded.
- A unit test checked that both pairs of probes agree on the raw values `0`,
  `0x0100_0000`, `0x0100_0100`, `0xFFFF_FEFF`, `0x1234_5678 & !(1 << 8)`, and
  `0x8000_0200`.

### Format

`Error` is a zero-sized type implementing `ser::Error` and `de::Error` that
discards messages, which favors option B: error paths never allocate.

```rust
/// (lowest bit, width) per field, in declaration order.
pub trait Layout {
    const WINDOWS: &'static [(u32, u32)];
    const CLAIMED: u32;
}

impl<L: Layout> Packer<L> {
    #[inline(always)]
    fn put(&mut self, value: u32, bits: u32) -> Result<(), Error> {
        let Some(&(lo, width)) = L::WINDOWS.get(self.index) else {
            return Err(Error);
        };
        self.index += 1;
        if width < bits && (value >> width) != 0 {
            return Err(Error); // does not fit: runtime, not compile time
        }
        self.acc |= value << lo;
        Ok(())
    }
}

impl<L: Layout> Unpacker<L> {
    #[inline(always)]
    fn take(&mut self, bits: u32) -> Result<u32, Error> {
        let Some(&(lo, width)) = L::WINDOWS.get(self.index) else {
            return Err(Error);
        };
        self.index += 1;
        if width > bits {
            return Err(Error);
        }
        let mask = if width == 32 { !0 } else { (1u32 << width) - 1 };
        Ok((self.raw >> lo) & mask)
    }
}
```

- `&mut Packer<L>` accepts only `serialize_struct`. Its `serialize_field` hands
  each value to a field serializer whose `serialize_bool`, `serialize_u8`,
  `serialize_u16`, and `serialize_u32` call `put` with 1, 8, 16, and 32 bits and
  whose `serialize_newtype_struct` forwards. Every other data-model method
  returns `Err(Error)` (`ser::Impossible` for compound types).
- `&mut Unpacker<L>` implements `deserialize_struct` as
  `visitor.visit_seq(self)`. Its `next_element_seed` returns `None` after the
  last window and otherwise deserializes through a field deserializer whose
  `deserialize_bool`, `deserialize_u8`, `deserialize_u16`, and `deserialize_u32`
  call `take` and the matching `visit_*`. Everything else forwards to a
  `deserialize_any` that returns `Err(Error)`.
- `to_u32::<L, T: Serialize>` serializes and requires every window to be used.
  `from_u32::<L, T: for<'de> Deserialize<'de>>` rejects unclaimed bits,
  deserializes, and requires every window to be used.

### Record

`PageId` (`domain = 1..`, `NonZeroU8` storage) and `Version` (`domain = ..64`)
implement Serde by hand with `#[inline(always)]`, as typekin's `with = serde`
does: serialize the raw `u8`; deserialize a `u8`, then validate (`!= 0`, `< 64`)
and fail with `custom("invalid value")`. `state` is a plain `u8`.

```rust
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageHeader {
    pub id: PageId,       // 24..32
    pub state: u8,        // 16..24
    pub version: Version, // 10..16
    pub sealed: bool,     // 9
    pub data: u8,         // 0..8
}

pub struct PageHeaderLayout;

impl Layout for PageHeaderLayout {
    const WINDOWS: &'static [(u32, u32)] =
        &[(24, 8), (16, 8), (10, 6), (9, 1), (0, 8)];
    const CLAIMED: u32 = !(1 << 8);
}
```

### Probes

```rust
#[unsafe(no_mangle)]
#[inline(never)]
pub fn pack_serde(h: &PageHeader) -> Result<u32, Error> {
    to_u32::<PageHeaderLayout, _>(h)
}

/// What a proven `packed::make` amounts to: no check, infallible.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn pack_hand(h: &PageHeader) -> u32 {
    (h.id.0.get() as u32) << 24
        | (h.state as u32) << 16
        | (h.version.0 as u32) << 10
        | (h.sealed as u32) << 9
        | h.data as u32
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn unpack_serde(raw: u32) -> Result<PageHeader, Error> {
    from_u32::<PageHeaderLayout, _>(raw)
}

/// What `packed::try_make` followed by `fields()` amounts to.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn unpack_hand(raw: u32) -> Option<PageHeader> {
    if raw & (1 << 8) != 0 {
        return None;
    }
    let id = PageId(NonZeroU8::new((raw >> 24) as u8)?);
    Some(PageHeader {
        id,
        state: (raw >> 16) as u8,
        version: Version(((raw >> 10) & 0x3f) as u8),
        sealed: (raw >> 9) & 1 != 0,
        data: raw as u8,
    })
}
```
