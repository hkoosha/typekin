# Typekin

`typekin` is a set of proc-macros for defining new types with relations between
them.

Out of the box, it comes with integer and enum-backed bitflags without the need
to hand-write their conversions and operator implementations, and making sure
the types are inlined to plain numbers in the compiled binary and erased at
runtime.

The concept of [fiendship](https://en.wikipedia.org/wiki/Friend_class) means
tight control over construction of values, making sure invalid values are
never created at runtime.

You can find a set of working examples in the crate repository at
[typekin/examples](./examples) directory.

## Example 0 - Numbers

```rust
#[typekin::integral(konst = false)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Quantity(usize);

fn main() {
    let this: Quantity = Quantity::make(321);
    let that: Quantity = Quantity::make(123);
    let it: Quantity = this + that + 222;
    assert_eq!(it.into_u64(), 666u64);      // 321 + 123 + 222 = 666
}
```

## Example 1 - Friendship

With the concept of friendship, one can have full control over not only how
different types are cast to each other but also how they interact. For example,
if `PageId(u32)`, `PageState(u16)` and `PageData(u16)` are to be combined into a
single `PageHeader(u64)` before written to disk,

```rust
#[repr(transparent)]
#[derive(Copy, Clone)]
#[typekin::integral(konst = false, friends = [
    PageId(conv=id_to_header, cap=[Make, Math, Bit, Relation]),
    PageState(conv=state_to_header, cap=[Make, Math, Bit, Relation]),
    PageData(conv=PageData::to_header, cap=[Make, Math, Bit, Relation]),
])]
struct PageHeader(u32);
// VALUE:   0b00000000_00000000_00000000_00000000;
// FORMAT:  ^ID......^ ^STATE.^ ^UNUSED^ ^DATA..^

#[repr(transparent)]
#[derive(Copy, Clone)]
struct PageId(u8);
#[repr(transparent)]
#[derive(Copy, Clone)]
struct PageState(u8);
#[repr(transparent)]
#[derive(Copy, Clone)]
struct PageData(u8);

fn id_to_header(it: PageId) -> u32 { (it.0 as u32) << 24 }
fn state_to_header(it: PageState) -> u32 { (it.0 as u32) << 8 }
impl PageData { fn to_header(self) -> u32 { (self.0 as u32) << 0 } }

fn main() {
    let id = PageId(0b0000_0101);
    let state = PageState(0b1010_1010);
    let data = PageData(0b1111_1111);

    // While id, state & data all have value of 0b1111, they will not overwrite
    // each other; because their friendship relationship guards how they are
    // cast into a PageHeader before being bit-or-ed into header:
    let mut header = PageHeader::make(0);
    header |= id;
    header |= state;
    header |= data;
    let expected = 0b00000101_10101010_00000000_11111111;
    let expected = 0b00000101_10101010_00000000_11111111;
    // FORMAT:     ^ID......^ ^STATE.^ ^UNUSED^ ^DATA..^
}
```

## The Catch

Mathematical and bitwise operations (e.g. subtraction, bitwise and, ...) can
still lead to invalid result. Instead of making them fallible operations
producing a `Result`, they lead to runtime panics. It is still possible to opt
out of any bitwise or match operation, and require manual cast to raw type and a
reconstruction of concrete type with the result. However, it will be an
unpleasant API to use and choosing typekin's for the use case would be
questionable in the first place.

This only affects types that do not accept all values in the underlying type's
domain. For instance, if a custom validator fn is provided to reject any u32
less than 3:

```rust
#[repr(transparent)]
#[derive(Copy, Clone)]
#[typekin::integral(
  konst = false,
  validator = Self::is_gte_3,
)]
struct Foo(usize);

impl Foo {
    fn is_gte_3(self) -> bool { self.0 >= 3 }
}

fn main() {
    let lhs = Quantity::try_make(5).unwrap();
    let rhs = Quantity::try_make(20).unwrap();

    // Panics; As 5 - 20 = -15, and  the `is_gte_3` check fails:
    let _: Foo = lhs - rhs;
}
```

________________________________________________________________________________

AI Disclaimer: The code is handwritten, but the rest of this README is AI
generated. Some tests are also written with the help of AI.

## Constness Status

Generated constness is explicit. Set `konst = true` for nightly-only const
generation, or `konst = false` for plain implementations that compile on stable
Rust.

### Friendship is explicit

If a type is not listed in `friends`, it does not get to construct, compare
with, or operate on your new-type.

```rust
#[typekin::integral(konst = false)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Quantity(u32);

#[typekin::integral(
  friends = [Quantity(conv = Quantity::raw, cap = [Cmp])],
  konst = false,
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Limit(u32);

fn main() {
    let q = Quantity::make(5);
    let l = Limit::make(8);

    // comparison is allowed because Limit friended Quantity at [Cmp]
    assert!(l > q);

    // Arithmetic is not, this will fail at compile time:
    // let bad = l + q;
}
```

### Validation

Use `validator` when raw values are not always valid.

```rust
const fn valid_port(it: u16) -> bool { it != 0 }

#[typekin::integral(
  konst = false,
  validator = valid_port,
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Port(u16);

fn main() {
    assert_eq!(Port::try_make(8080).map(Port::raw), Ok(8080));

    // A rejected checked construction returns the raw value.
    assert_eq!(Port::try_make(0), Err(0));
}
```

### Configuration

Generation of individual features can be disabled (check
integral's [cfg](./src/integral.rs) and bitflag's [cfg](./src/bitflag.rs))

```rust
#[typekin::integral(
    // Required. `true` needs nightly const features; `false` generates plain impls.
  konst = false,

  friends = [
    u64(cap = [Bit]),                    // Only bitwise operations
    Foo(conv = Foo::to_u32, cap = [Make, Math, Bit, Cmp]),
  ],

  get_raw = Self::unwrap, // Use an existing accessor instead of generated `raw()`
  validator = is_valid,   // Reject invalid raw values in checked construction
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Example(u32);
```

Capabilities are explicit: `Make` gates `of`, `Relation` gates equality
and ordering, `Bit` gates bitwise operations, and `Math` gates arithmetic.

---

## `#[typekin::bitflag]`

Apply `bitflag` to a unit enum with an integral `repr`. It generates the enum, a
`{Enum}Value` type that represents combined or unknown bits, and flag/value
helpers such as `name()`, `items()`, `from_name()`, and `contains()`. It is
modeled after [bitflag](https://crates.io/crates/bitflag), but with a different
implementation.

`konst` is required and forwarded to `integral` for the generated value type.
Write it directly as `konst = true` or `konst = false`; use `integral = [...]`
when configuring other generated value-type behavior. `bitflag` has its own
`friends = [...]` list for enum-left operations; configure access with
`cap = [Make, Math, Bit, Relation]`. The enum
and generated value type are registered as friends automatically; arithmetic
friendship is not generated.

### Example

```rust
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[typekin::bitflag(konst = false)]
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

    let raw = PermValue::try_make(0b111).unwrap();
    assert!(raw.contains(Perm::Exec));
}
```

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

const START: Counter = Counter::make(10);
const NEXT: Counter = START + 1u32;
```

Use `konst = false` to generate plain implementations instead:

```rust
#[typekin::integral(konst = false)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Counter(u32);
```
