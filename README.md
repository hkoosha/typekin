# Typekin

`typekin` is a set of proc-macros for defining new types with relations between
them.

Out of the box, it comes with integer and enum-backed bitflags without the need
to hand-write their conversions and operator implementations, validated text
newtypes over `alloc::string::String`, and runtime-erased transparent layouts.

The concept of [friendship](https://en.wikipedia.org/wiki/Friend_class) means
tight control over construction of values. Friend conversions consume their
input; non-`Copy` friends are moved into the conversion. Once a validated value
enters the validated graph, it stays validated while it moves between different
types and can be relied upon.

You can find a set of working examples in the crate repository at
[typekin/examples](./examples) directory.

## Attribute Macro Inventory and Item Requirements

The public attribute macros are:

- `#[typekin::integral]`
- `#[typekin::bitflag]`
- `#[typekin::text]`
- `#[typekin::friends]`

`integral` requires a `#[repr(transparent)]` one-field tuple struct whose field
is a primitive integer. `text` requires a nongeneric, `#[repr(transparent)]`
one-field tuple struct whose field is an unqualified `String`. `bitflag`
requires a unit enum with an integral `repr`. `friends` accepts concrete,
nongeneric structs, enums, or unions.


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

## Example 2 - Friendship

`friends` declares a conversion protocol. The conversion consumes its source,
and `Target::of` passes the resulting relation to `Target::of_parts`:

```rust

#[typekin::friends(
    // The two types talk to each other over `u8`-typed values.
    relation = u8,
    // `Source` is a friend of target`, it can participate in `Make` operations
    // That is, its value can be used to construct new Target instances.
    // The conversion happens by the function/method Source::into_relation
    friends = Source::into_relation(Source) -> Make,
)]
struct Target(u8);

impl Target {
    #[inline(always)]
    fn of_parts(relation: u8) -> Self { Self(relation) }
}

struct Source(u8);
impl Source {
    #[inline(always)]
    fn into_relation(self) -> u8 { self.0 }
}

fn main() {
    // The generated `of` method accepts instances of `Source` and knows how to
    // extract the relation value and construct a Target using the defined
    // `of_parts`
    assert_eq!(Target::of(Source(7)).0, 7);
}
```

## The Catch

Mathematical and bitwise operations can produce results rejected by a configured
validator. Instead of returning a `Result`, these operations panic on invalid
output. They can be disabled, leaving callers to extract raw values, operate on
them, and reconstruct the wrapper through checked construction.

These validation panics affect types that reject part of their underlying
domain; ordinary integer failures such as division by zero can also panic.
Validation occurs after every generated operator call, not only after a whole
expression. Rust evaluates `foo + 1 + 2` as `(foo + 1) + 2`, and each `+` must
produce a validated wrapper. Consequently, an intermediate result rejected by
the validator panics even if a later operation would produce an accepted final
value. To validate only a final result, extract the raw value, do the primitive
operations (using checked arithmetic when overflow must be handled), and
reconstruct with `try_make`.

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
    #[inline(always)]
    const fn is_gte_3(value: u32) -> bool { value >= 3 }
}

fn main() {
    let lhs = Foo::try_make(5).unwrap();
    let rhs = Foo::try_make(3).unwrap();

    // Panics: 5 - 3 = 2, which fails `is_gte_3`:
    let _: Foo = lhs - rhs;
}
```

Similarly, a type that accepts `0` but rejects `1` and `2` cannot evaluate this
chain through the generated operators, even though its final raw result would
be `3`:

```rust
#[typekin::integral(konst = false, valid = Self::is_valid)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct SkipOneAndTwo(u32);

impl SkipOneAndTwo {
    const fn is_valid(value: u32) -> bool { value != 1 && value != 2 }
}

let start = SkipOneAndTwo::try_make(0).unwrap();
// Parses as `(start + 1) + 2`; the first `+` must create `SkipOneAndTwo(1)`.
let _: SkipOneAndTwo = start + 1 + 2; // Panics.

// Validate only the final raw result instead.
let value = SkipOneAndTwo::try_make(start.raw() + 1 + 2).unwrap();
```

Validation has a runtime cost on every checked construction and generated
operation. When the invariant is a development-time diagnostic rather than a
release guarantee, a validator can be enabled only by an opt-in Cargo feature:

```rust
#[typekin::integral(konst = false, valid = Self::is_valid)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Counter(u32);

impl Counter {
    #[cfg(feature = "validate")]
    const fn is_valid(value: u32) -> bool { value <= 1_000 }

    #[cfg(not(feature = "validate"))]
    #[inline(always)]
    const fn is_valid(_: u32) -> bool { true }
}
```

This is analogous to the usual profile-dependent integer-overflow checking:
it can reduce release overhead, but it also means invalid values can be
constructed in builds without the feature. It is not suitable when the
invariant is required for safety, correctness, or an external contract.

## Constness Status

`integral` and `bitflag` require explicit `konst = true|false`. Set `true` for
nightly-only const generation or `false` for ordinary stable implementations.
`text` also requires explicit `konst = true|false`.

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
    #[inline(always)]
    fn try_make(raw: u32) -> Option<Self> {
        (1..=100).contains(&raw).then_some(Self(raw))
    }

    #[inline(always)]
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

`friends` also accepts `Trust`, independently of `Make`. It generates
`Self::of` for the relationship and every declared friend; targets provide
`fn of_parts(relation) -> Self`, and `scope` controls where the generated
protocol is placed. It has no generated validator to bypass: the configured
`of_parts` function always runs, including its own checks. Relations and friend
inputs can both be non-`Copy`.

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

An unvalidated integral has raw `make`. Configuring `in` or `valid` suppresses
that constructor and exposes `try_make(raw) -> Result<Self, Raw>` instead.

`in` accepts one range or a bracketed, comma-separated list:
`in = [2..=4, 8..10]`. Bounds must be integer constants (including primitive
`MIN`/`MAX` constants), not arbitrary expressions. `in` and `valid` conditions
are ANDed; generated construction and enabled mathematical or bitwise
operations reject values that fail them.

```rust
#[typekin::integral(
  konst = false,
  in = [1..=1023, 49_152..=65_535],
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

`text` creates a transparent, validated `String` newtype. By default
(`std = true`), generated paths use `::alloc::{string::String, vec::Vec}`.
Set `std = false` to use the corresponding `::alloc` paths. Text needs the
allocator crate in scope when not using std:

```rust
extern crate alloc;

use alloc::string::String;

#[typekin::text(
    konst = false,
    in = ["draft", "news", "medical-news"],
    // `valid = some_fn` is the alternative to literal membership.
)]
#[repr(transparent)]
struct Tag(String);

fn main() {
    let draft = Tag::try_from_str("draft").unwrap();
    let news = draft
        .map(|it| it.replace_range(.., "news"))
        .unwrap();
    let medical_news = news
        .clone()
        .map(|it| "medical-" + it)
        .unwrap();
    assert_eq!(news.as_str(), "news");

    let draft_news = news
        .map(|it| "draft-" + it);
    assert!(draft_news.is_err()); // There is no "draft-news" in allow-list.
}
```

`konst` is required. Choose exactly one validation mode: `valid` accepts one
callback path or a bracketed list of callbacks, while `in` accepts a
duplicate-free list of string literals. Membership compares exact strings
without case or Unicode normalization. `in = []` rejects every normal input;
`in = [""]` admits the empty string.

`Display` is generated by default; use `without = [display]` to opt out.

Text friends use `conversion(Source) -> Make`, with an owning function returning
`String`. Missing conversions are rejected during macro expansion. Friend output
is validated by default and panics if invalid. `-> [Make, Trust]` bypasses only
that construction validation; use it only when validity is proved independently.

### Configuration

Generation of individual features can be disabled (check
integral's [cfg](./src/integral.rs) and bitflag's [cfg](./src/bitflag.rs))

```rust
struct Foo(u32);
impl Foo { fn to_u32(self) -> u32 { self.0 } }
const fn is_valid(value: u32) -> bool { value != 0 }

#[typekin::integral(
  konst = false, // Required, and `true` needs nightly const features.
  friends = [
    _(u64) -> Bit,         // Only bitwise operations, no conversion needed
    Foo::to_u32(Foo) -> [Make, Math, Bit],
  ],
  without = [fn_conv_raw], // Do not generate the existing accessor below
  get_raw = Self::unwrap,  // Use an existing accessor instead of generated `raw()`
  valid = is_valid,        // Reject invalid raw values in checked construction
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
generated `{Enum}Value` type. They are constructed through generated integral
validation, so each enum discriminant must be valid.

`konst` is required and forwarded to `integral` for the generated value type.
Write it directly as `konst = true` or `konst = false`; use `integral = [...]`
when configuring other generated value-type behavior. `bitflag` has its own
`friends` option for enum-left operations; for example,
`friends = _(u8) -> Bit` or a bracketed list of such declarations. The enum and
generated value type are registered as friends automatically; enum arithmetic
operators are not generated.

`suffix` and `value_name` control the generated value-type name. In the nested
`integral = [...]` configuration, `without = [impl_core_int]` removes the large
forwarded primitive-integral-method group.

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

    let mixed = PermValue::from_bits_retain(0b1011);
    assert_eq!(PermValue::from_bits(0b1011), None);
    assert_eq!(mixed.raw(), 0b1011);
    assert_eq!(PermValue::from_bits_truncate(0b1011).raw(), 0b0011);
    assert_eq!(PermValue::Read.into_flag(), Ok(Perm::Read));
    assert_eq!(mixed.into_flag(), Err(mixed));
}
```

## Const mode

Const generation requires `konst = true`. Until the relevant const features
stabilize, it remains nightly-only. These gates are sufficient for the
`integral` example below:

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

`bitflag(konst = true)` needs that set plus const iterator support:

```rust
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_destruct)]
#![feature(derive_const)]
#![feature(const_iter)]

#[repr(u8)]
#[derive(Copy)]
#[derive_const(Clone, Eq, PartialEq, Ord, PartialOrd)]
#[typekin::bitflag(konst = true)]
enum Permission {
    Read = 0b001,
    Write = 0b010,
}
```

Use `konst = false` to generate plain implementations instead:

```rust
#[typekin::integral(konst = false)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Counter(u32);
```

