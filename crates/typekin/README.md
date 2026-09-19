# Typekin

`typekin` is a proc-macro for defining integer newtype and enum-backed bitflags
without hand-writing their conversions and operator implementations, and making
sure the type is inlined to plain numbers and erased at runtime.

The concept of [fiendship](https://en.wikipedia.org/wiki/Friend_class) means
tight control over construction of values, making sure invalid values are
never created at runtime.

- `#[typekin::integral]` generates an integral newtype API.
- `#[typekin::bitflag]` generates enum flags plus a value type for combined
  and / or unknown bit patterns.

Working examples:

- [typekin/examples/my_u32.rs](./examples/my_u32.rs)
- [typekin/examples/my_i128.rs](./examples/my_i128.rs)
- [typekin/examples/my_flag.rs](./examples/my_flag.rs)

## Example 0 - Numbers

```rust
#[typekin::integral(konst = false)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Quantity(usize);

fn main() {
    let this: Quantity = Quantity::of(321);
    let that: Quantity = Quantity::of(123);
    let it: Quantity = this + that + 222;
    assert_eq!(it.into_u64(), 666u64);      // 321 + 123 + 222 = 666
}
```

## Example 1 - Friendship

With concept of friendship, one can have full control over not only how
different types are cast to each other but also how they interact. For example,
if `PageId(u32)`, `PageState(u16)` and `PageData(u16)` are to be combined into a
single `PageHeader(u64)` before written to disk, 

```rust
#[repr(transparent)]
#[derive(Copy, Clone)]
#[typekin::integral(konst = false, friends = [
    PageId(conv=id_to_header),
    PageState(conv=state_to_header),
    PageData(conv=PageData::to_header),
])]
struct PageHeader(u32);

#[repr(transparent)]
#[derive(Copy, Clone)]
struct PageId(u8);

#[repr(transparent)]
#[derive(Copy, Clone)]
struct PageState(u8);

#[repr(transparent)]
#[derive(Copy, Clone)]
struct PageData(u8);

fn id_to_header(it: PageId) -> u32 { return it.0; }
fn state_to_header(it: PageState) -> u32 { return it.0; }
impl PageData { fn to_header(self) -> u32 { return self.0; } }

fn main() {
    let id =       PageId::of(0b0000_0101);
    let state = PageState::of(0b1010_1010);
    let data =   PageData::of(0b1111_1111);
  
    // While id, state & data all have value of 0b1111, they will not overwrite
    // each other; because their friendship relationship guards how they are
    // cast into a PageHeader before being bit-or-ed into header:
    let mut header = PageHeader::of(0);
    header |= id;
    header |= state;
    header |= data;
  
    let expected = 0b00000101_10101010_00000000_11111111;
    //               ^..ID..^ ^......STATE....^ ^.DATA.^
  
    assert_eq!(header.raw(), expected);
}
```

________________________________________________________________________________

AI Disclaimer: The code is handwritten, but the rest of this README is AI
generated. Some tests are also written with the help of AI.

## Constness Status

Generated constness is explicit. Set `konst = true` for nightly-only const
generation, or `konst = false` for plain implementations that compile on stable
Rust.

## `#[typekin::integral]`

Applied to a single-field tuple struct over a primitive integer and annotated
with `#[repr(transparent)]`, it generates construction, conversion, comparison,
and operator APIs. Interoperation is deny-by-default: add a type to `friends`
only for operations that are valid for the domain.

### Minimal use

```rust
#[typekin::integral(konst = false)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Quantity(u32);

fn main() {
    let quantity = Quantity::of(23u32);

    assert_eq!(quantity.raw(), 23);
    assert_eq!((quantity + 1u32).raw(), 24);
}
```

### Friendship is explicit

If a type is not listed in `friends`, it does not get to construct, compare
with, or operate on your new-type.

```rust
#[typekin::integral(konst = false)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Quantity(u32);

#[typekin::integral(
  friends = [Quantity(conv = Quantity::raw, level = [Rel])],
  konst = false,
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Limit(u32);

fn main() {
    let q = Quantity::of(5u32);
    let l = Limit::of(8u32);

    // comparison is allowed because Limit friended Quantity at [Rel]
    assert!(l > q);

    // Arithmetic is not, this will fail at compile time:
    // let bad = l + q;
}
```

### Validation

Use `fn_validator` when raw values are not always valid.

```rust
const fn valid_port(
    it: u16
) -> bool {
    it != 0
}

#[typekin::integral(
  konst = false,
  fn_validator = valid_port,
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Port(u16);

fn main() {
    assert_eq!(Port::try_make(8080), Ok(Port::of(8080u16)));

    // A rejected checked construction returns the raw value.
    assert_eq!(Port::try_make(0), Err(0));
}
```

### Configuration

Generation of individual features can be disabled (check integral's
[cfg](./src/integral.rs) and bitflag's [cfg](./src/bitflag.rs))

```rust
#[typekin::integral(
    // Required. `true` needs nightly const features; `false` generates plain impls.
  konst = false,

  friends = [
    u64(level = [Bit]),      // Only bitwise operations
    Foo(conv = Foo::to_u32), // Full access with custom conversion
  ],

  fn_get_raw = Self::unwrap, // Use an existing accessor instead of generated `raw()`
  fn_validator = is_valid,   // Reject invalid raw values in checked construction
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Example(u32);
```

Friendship levels: `Full` expands to `Make`, `Rel`, `Bit`, and `Math`; use an
individual level to narrow access. `None` grants no generated operations.

---

## `#[typekin::bitflag]`

Apply `bitflag` to a unit enum with an integral `repr`. It generates the enum,
a `{Enum}Value` type that represents combined or unknown bits, and flag/value
helpers such as `name()`, `items()`, `from_name()`, and `contains()`.

`konst` is required inside `integral = [...]`; it controls generated code for
the value type. `bitflag` has its own `friends = [...]` list for enum-left
bitwise operations. The enum and generated value type are registered as friends
automatically; arithmetic friendship is not generated.

### Example

```rust
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[typekin::bitflag(
  integral = [konst = false],
)]
pub enum Perm {
    None = 0,
    Read = 0b001,
    Write = 0b010,
    Exec = 0b100,
}

fn main() {
    let rw = Perm::Read.inserted(Perm::Write);

    assert_eq!(rw.raw(), 0b011);
    assert!(rw.contains(Perm::Read));
    assert!(rw.contains(Perm::Write));
    assert!(!rw.contains(Perm::Exec));

    assert_eq!(Perm::from_name("Exec"), Some(Perm::Exec));
    assert_eq!(Perm::Read.name(), "Read");

    let names: Vec<_> = rw.iter_known_flags().map(Perm::name).collect();
    assert_eq!(names, vec!["Read", "Write"]);

    let raw = PermValue::try_make(0b111u8).unwrap();
    assert!(raw.contains(Perm::Exec));
}
```

### Unknown bits stay representable

The generated value type is still an integral new-type, so raw bit patterns are
kept even when they do not map to a named flag. For this to work, the named
flags (i.e. enum variants) are separated from values (value bits).

```rust
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[typekin::bitflag(
  integral = [
    // This configuration is passed to `typekin::integral` for `ModeValue`.
    konst = false
  ],
)]
pub enum Mode {
    None = 0,
    A = 0b0001,
    B = 0b0010,
}

fn main() {
    let value = ModeValue::of(0b1000u8);

    assert!(value.contains_unknown_bits());
    assert_eq!(value.into_known_bits().raw(), 0);
    assert_eq!(value.raw(), 0b1000);
}
```

---

## Const mode

Const generation requires `konst = true`. Until the relevant const features
stabilize, it remains nightly-only:

```rust
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_destruct)]
#![feature(derive_const)]

#[typekin::integral(konst = true)]
#[repr(transparent)]
#[derive(Copy)]
#[derive_const(Clone)]
pub struct Counter(u32);

const START: Counter = Counter::of(10u32);
const NEXT: Counter = START + 1u32;
```

Use `konst = false` to generate plain implementations instead:

```rust
#[typekin::integral(konst = false)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Counter(u32);
```

---

## Development

```sh
just build
just test
just fmt

# regenerate expanded examples
just u32
just plain
just flag
# Or more simply:
just all
```
