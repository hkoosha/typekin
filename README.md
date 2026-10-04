# Typekin

`typekin` is a set of proc-macros for defining new types with relations between
them.

Out of the box, it comes with integer and enum-backed bitflags without the need
to hand-write their conversions and operator implementations, validated text
newtypes over `alloc::string::String`, and runtime-erased transparent layouts.

The concept of [friendship](https://en.wikipedia.org/wiki/Friend_class) means
tight control over construction of values. Friend conversions consume their
input; non-`Copy` friends are moved into the conversion.

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
    let it: Quantity = this + that + 222usize;
    assert_eq!(it.into_u64(), 666u64);      // 321 + 123 + 222 = 666
}
```

## Example 1 - Friendship

With the concept of friendship, one can have full control over not only how
different types are cast to each other but also how they interact. For example,
if `PageId(u8)`, `PageState(u8)` and `PageData(u8)` are to be combined into a
single `PageHeader(u32)` before being written to disk,

```rust
#[repr(transparent)]
#[derive(Copy, Clone)]
#[typekin::integral(konst = false, friends = [
    id_to_header(PageId) -> [Make, Math, Bit, Relation],
    state_to_header(PageState) -> [Make, Math, Bit, Relation],
    PageData::to_header(PageData) -> [Make, Math, Bit, Relation],
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
fn state_to_header(it: PageState) -> u32 { (it.0 as u32) << 16 }
impl PageData { fn to_header(self) -> u32 { self.0 as u32 } }

fn main() {
    let id = PageId(0b0000_0101);
    let state = PageState(0b1010_1010);
    let data = PageData(0b1111_1111);

    // Each conversion places its source in a separate byte of the header:
    let mut header = PageHeader::make(0);
    header |= id;
    header |= state;
    header |= data;
    let expected = 0b00000101_10101010_00000000_11111111;
    assert_eq!(header.raw(), expected);
    // FORMAT:     ^ID......^ ^STATE.^ ^UNUSED^ ^DATA..^
}
```

## The Catch

Mathematical and bitwise operations can produce results rejected by a configured
validator. Instead of returning a `Result`, these operations panic on invalid
output. They can be disabled, leaving callers to extract raw values, operate on
them, and reconstruct the wrapper through checked construction.

These validation panics affect types that reject part of their underlying
domain; ordinary integer failures such as division by zero can also panic.
For instance, a callback can reject any `u32` less than 3:

```rust
#[repr(transparent)]
#[derive(Copy, Clone)]
#[typekin::integral(
  konst = false,
  valid = Self::is_gte_3,
)]
struct Foo(u32);

impl Foo {
    const fn is_gte_3(value: u32) -> bool { value >= 3 }
}

fn main() {
    let lhs = Foo::try_make(5).unwrap();
    let rhs = Foo::try_make(3).unwrap();

    // Panics: 5 - 3 = 2, which fails `is_gte_3`:
    let _: Foo = lhs - rhs;
}
```

________________________________________________________________________________

AI Disclaimer: The code is handwritten, but the rest of this README is AI
generated.

## Constness Status

`integral` and `bitflag` require explicit `konst = true|false`. Set `true` for
nightly-only const generation or `false` for ordinary stable implementations.
Text currently defaults to non-const generation when `konst` is omitted.

### Friendship is consuming

Friend capabilities gate construction and operations. Integral automatically
includes itself and its backing integer for operations, and grants raw/widening
construction when unvalidated; bitflag registers its enum/value pair. Additional
friends are declared explicitly. Conversions consume their inputs. Integral and
text equality and ordering remain same-type operations.

```rust
#[typekin::integral(konst = false)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Quantity(u32);

#[typekin::integral(
  friends = Quantity::raw(Quantity) -> Make,
  konst = false,
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Limit(u32);

fn main() {
    let quantity = Quantity::make(5);
    let limit = Limit::of(quantity);
    assert_eq!(limit.raw(), 5);
}
```

### Trusted friend construction

Friends without `Trust` are untrusted. Integral and text `of` validate their
converted values and panic if invalid. Granting both `Make` and `Trust` skips
only that friend-construction validation: the friend author must prove the
converted value is already valid. `Trust` alone does not grant `Make`.

```rust
struct Verified(u32);

impl Verified {
    fn try_make(raw: u32) -> Option<Self> {
        (1..=100).contains(&raw).then_some(Self(raw))
    }

    fn into_raw(self) -> u32 { self.0 }
}

#[typekin::integral(
    konst = false,
    in = 1..=100,
    friends = Verified::into_raw(Verified) -> [Make, Trust],
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Limited(u32);

fn main() {
    let verified = Verified::try_make(42).unwrap();
    let limited = Limited::of(verified); // Consumes the non-Copy friend.
    assert_eq!(limited.raw(), 42);
    assert_eq!(Limited::try_make(0), Err(0));
}
```

Trust grants no additional capabilities. Checked constructors and math/bitwise
operation results still validate, including operations with a trusted friend
as the right-hand operand.

Standalone `friendship` also accepts `Trust`, independently of `Make` (including
`Make` granted by `constructor`). It has no generated validator to bypass: the
configured `relationship` function always runs, including its own checks.
Relations and friend inputs can both be non-`Copy`.

### Validation

Place each constraint at the macro root. `valid` accepts either one callback
path or a comma-separated list in brackets; every listed callback must return `true`.

```rust
const fn valid_port(it: u16) -> bool { it != 0 }

#[typekin::integral(
  konst = false,
  valid = valid_port,
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

Integral types also implement `FromStr`. Parsing first uses the wrapped
primitive's parser and then applies the same validation; either failure returns
`Err(())`. Use `without = [impl_from_str]` to omit this implementation.

`in` accepts either one Rust range expression or a `+`-separated union:
`in = 1..=1023 + 49_152..=65_535`. Commas are reserved for ANDed lists, so
they are not valid range separators. `in` and `valid` conditions are ANDed;
generated construction and enabled mathematical or bitwise operations reject
values that fail them.

```rust
#[typekin::integral(
  konst = false,
  in = 1..=1023 + 49_152..=65_535,
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Port(u16);

fn main() {
    assert_eq!(Port::try_make(443).map(Port::raw), Ok(443));
    assert_eq!(Port::try_make(1024), Err(1024));
}
```

## `#[typekin::text]`

`text` creates a transparent, validated `String` newtype. Text needs the
allocator crate in scope, even in a `std` crate:

```rust
extern crate alloc;

use alloc::string::String;

fn is_slug(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
}

#[typekin::text(
    konst = false,
    valid = is_slug,
    in = ["draft", "published"],
    with = [display],
)]
#[repr(transparent)]
struct Slug(String);

fn main() {
    let draft = Slug::try_from_str("draft").unwrap();
    let published = draft
        .map(|value| value.replace_range(.., "published"))
        .unwrap();
    assert_eq!(published.as_str(), "published");
}
```

`valid` accepts one callback or a bracketed list; every callback receives
`&str` and must return `bool`. `in` accepts a duplicate-free list of string
literals. The two constraints compose with logical AND.

Membership compares exact strings without case or Unicode normalization.
`in = []` rejects every normal input; `in = [""]` admits the empty string.
A listed value must still pass every callback.

The generated type offers fallible `try_make` / `try_from_str`, consuming
`into_inner` / `into_bytes`, read-only string access, `Deref<Target = str>`,
`AsRef`, `Borrow`, hash, equality, and ordering. It deliberately does not offer
mutable string aliases such as `DerefMut`, `as_mut_str`, or `&mut String`.
`map(self, FnOnce(&mut String)) -> Result<Self, ()>` is the checked mutation
entry point: invalid output is dropped instead of rewrapped.

Text friends use `conversion(Source) -> Make`, with an owning function returning
`String`. Missing conversions are rejected during macro expansion. Friend output
is validated by default and panics if invalid. `-> [Make, Trust]` bypasses only
that construction validation; use it only when validity is proved independently.

`with = [display]` opts into `Display`. `konst = true` propagates const
generation through text APIs, including validation and `map`; unsupported
callbacks, allocation, or closure combinations are rejected by the caller's
nightly compiler configuration.

### Configuration

Generation of individual features can be disabled (check
integral's [cfg](./src/integral.rs) and bitflag's [cfg](./src/bitflag.rs))

```rust
struct Foo(u32);
impl Foo { fn to_u32(self) -> u32 { self.0 } }
const fn is_valid(value: u32) -> bool { value != 0 }

#[typekin::integral(
    // Required. `true` needs nightly const features; `false` generates plain impls.
  konst = false,

  friends = [
    _(u64) -> Bit,                    // Only bitwise operations
    Foo::to_u32(Foo) -> [Make, Math, Bit],
  ],
  without = [fn_conv_raw], // Do not generate the existing accessor below

  get_raw = Self::unwrap, // Use an existing accessor instead of generated `raw()`
  valid = is_valid, // Reject invalid raw values in checked construction
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Example(u32);

impl Example {
    const fn unwrap(self) -> u32 { self.0 }
}
```

Capabilities are explicit: `Make` gates `of`, `Bit` gates bitwise operations,
and `Math` gates arithmetic. Integral equality and ordering are same-type only.

---

## `#[typekin::bitflag]`

Apply `bitflag` to a unit enum with an integral `repr`. It generates the enum, a
`{Enum}Value` type that represents combined or unknown bits, and flag/value
helpers such as `name()`, `items()`, `from_name()`, and `contains()`. It is
modeled after [bitflag](https://crates.io/crates/bitflag), but with a different
implementation.

Every enum variant is mirrored as a same-named associated constant on its
generated `{Enum}Value` type.
They are constructed through generated integral validation, so each enum
discriminant must be valid.

`konst` is required and forwarded to `integral` for the generated value type.
Write it directly as `konst = true` or `konst = false`; use `integral = [...]`
when configuring other generated value-type behavior. `bitflag` has its own
`friends` option for enum-left operations; for example,
`friends = _(u8) -> Bit` or a bracketed list of such declarations. The enum and
generated value type are registered as friends automatically; enum arithmetic
operators are not generated.

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
    assert_eq!(names, vec!["None", "Read", "Write"]);

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

fn main() { assert_eq!(NEXT.raw(), 11); }
```

Use `konst = false` to generate plain implementations instead:

```rust
#[typekin::integral(konst = false)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Counter(u32);
```

