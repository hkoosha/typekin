# Typekin

Typekin is a Rust procedural-macro crate for defining transparent newtypes and
explicit relationships between them. It makes construction, permitted
operations, and validation boundaries explicit while staying zero-cost.

Friendship controls construction and cross-type operations. Friend conversions
consume their input. Once a value enters the validated graph through an ordinary
safe path, generated construction and transformation APIs preserve its
invariant. Friendship is an infallible contract, not an expected-failure
conversion mechanism. If conversion failure is expected, Rust's `TryFrom` or
`TryInto` is likely better suited.

The generated code adds no runtime dependencies. Optimized assembly comparisons
for covered integral operations verify that wrapper code matches primitive
equivalents after inlining. The annotated types are not hidden and remain in
scope, so additional inherent implementations and derive attributes remain
available. Currently, Typekin provides:

- friendship protocols for capability-gated construction and operations;
- integer newtypes with validation and generated conversions, mathematical
  operations, and bitwise operations;
- enum-backed bitflags modeled after the
  [`bitflags`](https://crates.io/crates/bitflags) crate;
- validated `String` newtypes over `std` or `alloc`, which allocate only when
  their underlying `String` operation does;
- configurable const support. By default, all stable const features are enabled.
  Additional constness for math operations requires `konst = nightly`;
- allocation-free generated integral and bitflag code: it requires neither `std`
  nor `alloc` and does not allocate. Allocation in caller-provided callbacks,
  conversions, or Serde backends is outside Typekin's generated code;
- individual feature flags for disabling generation of specific features.

Working examples are available in the repository's [examples](./examples)
directory. This crate makes no backward-compatibility guarantees before version
1.0.0.

## Attribute Macro Inventory and Item Requirements

The supported public attribute macros are:

- `#[typekin::integral]`: over a one-field tuple struct whose field is a
  primitive integer.
- `#[typekin::bitflag]`: over a unit enum with an integral `repr`.
- `#[typekin::text]` over non-generic one-field tuple struct whose field is an
  unqualified `String`.
- `#[typekin::friends]`: over concrete, non-generic structs, enums, or unions.
  This is the underlying machinery of other macros.

## Examples

### Example 0 - Integers

```rust
#[typekin::integral]
struct Quantity(usize);

fn main() {
    let this: Quantity = Quantity::make(321);
    let that: Quantity = Quantity::make(123);
    let it: Quantity = this + that + 222usize;
    // 321 + 123 + 222 = 666
    assert_eq!(it.into_u64(), 666u64);
}
```

### Example 1 - Friendship

With friendships, one can have full control over _how_ different types are cast
to each other and how they interact. For example, if `PageId(u8)`,
`PageState(u8)` and `PageData(u8)` are to be combined into a single
`PageHeader(u32)` before being written to disk:

```rust
// The friend syntax is: CONVERSION_FN(FRIEND_TYPE) -> [CAPABILITY, ...].
// Use _(FRIEND_TYPE) when no conversion callback is needed.
// With no explicit capability, the default is assumed.
#[typekin::integral(friends = [
    id_to_header(PageId) -> [Math, Rel], 
    state_to_header(PageState),
    PageData::to_header(PageData),
])]
struct PageHeader(u32);
// VALUE:   0b00000000_00000000_00000000_00000000;
// FORMAT:  ^ID......^ ^STATE.^ ^UNUSED^ ^DATA..^

#[derive(Copy, Clone)]
struct PageId(u8);
#[inline(always)]
fn id_to_header(it: PageId) -> u32 { (it.0 as u32) << 24 }

#[derive(Copy, Clone)]
struct PageState(u8);
#[inline(always)]
fn state_to_header(it: PageState) -> u32 { (it.0 as u32) << 16 }

#[derive(Copy, Clone)]
struct PageData(u8);
impl PageData {
    #[inline(always)]
    fn to_header(self) -> u32 { self.0 as u32 }
}

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

    assert!(header > PageId(0));
}
```

### Example 2 - Friendship

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
    fn of_parts(relation: u8) -> Self { Self(relation) }
}

struct Source(u8);
impl Source {
    fn into_relation(self) -> u8 { self.0 }
}

fn main() {
    // The generated `of` method accepts instances of `Source` and knows how to
    // extract the relation value and construct a Target using the defined
    // `of_parts`
    let source = Source(7);
    let target = Target::of(source);
    assert_eq!(target.0, 7);
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

The validations can still incur a runtime cost, hence the validation function
can be disabled in certain profiles such as release; just as how rust disables
overflow checks at runtime:

```rust
#[inline(always)]
const fn is_div_by_3(raw: usize) -> bool {
    // In debug profile, raw is always validated before creating the type `Foo`:
    #[cfg(debug_assertions)] return raw % 3 == 0;

    // Inlines to NO-OP and vanishes in release:
    #[cfg(not(debug_assertions))] return true;
}

#[derive(Copy, Clone)]
#[typekin::integral(valid = is_div_by_3)]
struct Foo(usize);

fn main() {
    let lhs = Foo::try_make(5).unwrap();
    let rhs = Foo::try_make(3).unwrap();

    // Panics: 5 - 3 = 2, which fails `is_div_by_3`:
    let _: Foo = lhs - rhs;
}
```

Integral wrappers generate `try_map`, which returns the rejected final raw
value, and `map`, which panics when final validation fails. The closure runs
once and intermediate primitive values are not validated. Primitive overflow and
closure panics retain their normal Rust behavior. Use `without = [map]` to omit
both methods.

Validation has a runtime cost on every checked construction and generated
operation. When the invariant is a development-time diagnostic rather than a
release guarantee, a validator can be enabled only by an opt-in Cargo feature:

## Constness Status

`integral`, `bitflag`, and `text` default to `konst = stable`. Set
`konst = none` to disable optional const flags, or `konst = nightly` to enable
const trait implementations and nightly-only integer operations. For integral
and bitflag, `stable` enables stable validation and value operations. For text,
it enables const read-only accessors; `nightly` additionally enables supported
owned construction and conversion.

### Friendship is consuming

Friend capabilities gate construction and operations. Integral automatically
includes itself and its backing integer for operations, and grants raw/widening
construction when unvalidated; bitflag registers its enum/value pair. Additional
friends are declared explicitly. Conversions consume their inputs. Integral and
text equality and ordering remain same-type operations.

### Trusted friend construction

Friends without `unsafe` are untrusted. Integral and text `of` validate their
converted values and panic if invalid. `unsafe` implies `Make` and skips only
that friend-construction validation: the friend author must prove the converted
value is already valid.

`unsafe` does not imply `Math`, `Bit`, or `Rel`. Checked constructors and
math/bitwise operation results still validate, including operations with a
trusted friend as the right-hand operand. Text `unsafe` friends require an
explicit conversion returning `String`, just like explicit `Make` friends.

Standalone `friends` generates `Self::of` for the relationship and every
declared friend; targets provide `fn of_parts(relation) -> Self`, and `scope`
controls where the generated protocol is placed. It has no generated validator
to bypass: the configured `of_parts` function always runs, including its own
checks. Relations and friend inputs can both be non-`Copy`.

### Validation

Place each constraint at the macro root. `valid` accepts either one callback
path or a comma-separated list in brackets; every listed callback must return
`true` for the value to be considered valid.

```rust
const fn is_valid_port(it: u16) -> bool { it != 0 }

#[typekin::integral(valid = is_valid_port)]
struct Port(u16);

fn main() {
    assert_eq!(Port::try_make(8080).map(Port::raw), Ok(8080));

    // A rejected checked construction returns the raw value.
    assert_eq!(Port::try_make(0), Err(0));
}
```

Integral types also implement `FromStr`. Parsing first uses the wrapped
primitive's parser and then applies the same validation; either failure returns
`Err(())`. Use `without = [from_str]` to omit this implementation.

An unvalidated integral has raw `make`. Configuring `in` or `valid` suppresses
that constructor and exposes `try_make(raw) -> Result<Self, Raw>` instead.

`in` accepts one range or a bracketed, comma-separated list:
`in = [2..=4, 8..10]`. Bounds must be integer constants (including primitive
`MIN`/`MAX` constants), not arbitrary expressions. `in` and `valid` conditions
are ANDed; generated construction and enabled mathematical or bitwise operations
reject values that fail them.

```rust
#[typekin::integral(in = [1..=1023, 49_152..=65_535])]
struct Port(u16);

fn main() {
    assert_eq!(Port::try_make(443).map(Port::raw), Ok(443));
    assert_eq!(Port::try_make(1024), Err(1024));
}
```

`error = ()` is accepted by `integral`, `text`, and a bitflag's nested
`integral` configuration. Unit is currently the only supported error type; the
option reserves the configuration surface without changing existing constructor
error values.

## Text - `#[typekin::text]`

`text` creates a validated `String` newtype. By default (`std = true`),
generated paths use `::std::{string::String, vec::Vec}`. Set `std = false` to
use the corresponding `::alloc` paths. Text needs the allocator crate in scope
when not using std. Unlike other types, `Display` is generated by default for
`text`; use `without = [display]` to opt out.

```rust
extern crate alloc;

use alloc::string::String;

#[typekin::text(
    std = false,
    in = ["draft", "news", "medical-news"],
    // `valid = some_fn` is the alternative to literal membership.
)]
struct Tag(String);

fn main() {
    let draft = Tag::try_from_str("draft").unwrap();
    let news = draft
        .map(|it| it.replace_range(.., "news"))
        .unwrap();
    let medical_news = news
        .clone()
        .map(|it| it.insert_str(0, "medical-"))
        .unwrap();
    assert_eq!(news.as_str(), "news");
    assert_eq!(medical_news.as_str(), "medical-news");

    let draft_news = news
        .map(|it| it.insert_str(0, "draft-"));
    assert!(draft_news.is_err()); // There is no "draft-news" in allow-list.
}
```

### Serde

Use `with = serde` on `integral`, `text`, or `bitflag` to generate `Serialize`
and `Deserialize`. Deserialization applies every configured validator. Typekin
has no Serde Cargo feature and does not provide Serde as a normal dependency:
the consuming crate must add `serde` directly and apply any desired `cfg` gating
to the macro invocation.

Integral wrappers serialize as their raw integer, while text wrappers serialize
as strings. Do not also derive the same Serde traits on a wrapper using this
option.

### Configuration

Generation of individual features can be disabled (check integral's
[cfg](src/integral.rs) and bitflag's [cfg](src/bitflag.rs))

### Integral output types and profiles

`#[typekin::integral]` can configure one result type for all generated
non-assignment operators:

```rust
#[typekin::integral(
    konst = none,
    out_type = Point::into_offset(_) -> Offset,
)]
struct Point(u32);
```

The callback receives the validated `Point` result exactly once. Its target need
not be `Copy`, an integral wrapper, or a friend. Constructors, raw access,
comparisons, conversions, `map`, `try_map`, and explicitly named integer methods
keep their normal result types. Assignment operators are unavailable because
they must leave `Self` in place.

The callback runs after the source result has been constructed and validated. An
operation whose primitive result is invalid for the source wrapper cannot be
recovered by mapping it to a valid target. Rust checks the callback's
visibility, signature, target validation, and constness. In a const operator,
the callback must itself be const-callable.

Profiles select the ordinary arithmetic policy and default API surface:

| Profile | Arithmetic | Ordering |
| --- | --- | --- |
| `default {}` | Primitive Rust arithmetic | Default `Ord` / `PartialOrd` behavior |
| `circular {}` | Native-width wrapping | No `Ord` / `PartialOrd`; optional `precedes` |
| `sequential {}` | Checked; panics on overflow or underflow | Ordinary linear ordering |

For `circular` and `sequential`, same-type `+`, `-`, and their assignments
remain enabled. Primitive arithmetic, bitwise operators, shifts, `core_int`, and
bit access are disabled by default. A profile body can use `with` and `without`
to change its operation set; re-enabled arithmetic uses that profile's wrapping
or checked policy. Bitwise operations retain ordinary bitwise semantics.

```rust
#[typekin::integral(
    konst = none,
    profile = circular {
        precedes = true,
        without = math_add,
    },
    out_type = SeqPoint::into_offset(_) -> SeqOffset,
)]
struct SeqPoint(u32);

#[typekin::integral(
    konst = none,
    profile = circular {
        precedes = false,
    },
)]
struct SeqOffset(i32);
```

`precedes` compares the backing bit patterns around the wrapping circle.
Equality and values exactly half the numeric space apart do not precede each
other. Use `raw()` for deliberate linear bit-pattern ordering. The one
`out_type` policy cannot express a full mixed point/offset algebra: write mixed
operations such as `Point + Offset -> Point` by hand.

`precedes` is only valid inside `circular`; it defaults to `true`. `default` and
`sequential` reject it. Circular profiles also reject explicit `ord` or
`partial_ord`. Profiles do not apply to generated bitflag value types; nested
bitflag integral configuration supports only the default profile and no
`out_type`.

```rust
struct Foo(u32);
impl Foo { fn to_u32(self) -> u32 { self.0 } }
const fn is_valid(value: u32) -> bool { value != 0 }

#[typekin::integral(
  konst = nightly,         // Enables all const flags.
  friends = [
    self(u16) -> Bit,      // Only bitwise operations, using `Into<u32>`
    Foo::to_u32(Foo),      // Omitted capabilities default to Math
  ],
  without = [fn_conv_raw], // Do not generate the existing accessor below
  get_raw = Self::unwrap,  // Use an existing accessor instead of generated `raw()`
  valid = is_valid,        // Reject invalid raw values in checked construction
)]
struct Example(u32);

impl Example {
    const fn unwrap(self) -> u32 { self.0 }
}
```

Capabilities are explicit: `Make` gates `of`, `Bit` gates bitwise operations,
and `Math` enables construction, arithmetic, and bitwise operations. `Rel`
enables `==`, `!=`, `<`, `<=`, `>`, and `>=` against `Copy` friends. Integral
friends with no `->` capability clause default to `Math`; write `-> []` to grant
no capabilities.

---

## Bitflag - `#[typekin::bitflag]`

Apply `bitflag` to a unit enum with an integral `repr`. It generates the enum, a
`{Enum}Value` type that represents combined or unknown bits, and flag/value
helpers such as `name()`, `items()`, `from_name()`, and `contains()`. It is
modeled after [bitflags](https://crates.io/crates/bitflags), but with a
different implementation.

`konst` defaults to `stable`. `suffix` and `value_name` control the generated
value-type name.

### Example

```rust
#[typekin::bitflag(repr = u8, with = [display])]
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
    assert_eq!(mixed.to_string(), "Read | Write | 0x8");
    assert_eq!("Read | Write | 0x8".parse::<PermValue>(), Ok(mixed));
    assert_eq!(PermValue::Read.into_flag(), Ok(Perm::Read));
    assert_eq!(mixed.into_flag(), Err(mixed));
}
```

## Const mode

Full integral const generation requires `konst = nightly`. Until the relevant
const features stabilize, it remains nightly-only. These gates are sufficient
for the integral example below:

```rust
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_destruct)]
#![feature(derive_const)]

#[typekin::integral(konst = nightly)]
pub struct Counter(u32);

const START: Counter = Counter::make(10);
const NEXT: Counter = START + 1u32;

fn main() { assert_eq!(NEXT.raw(), 11); }
```

`bitflag` needs that set plus const iterator support:

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
#[typekin::bitflag(konst = nightly)]
enum Permission {
    Read = 0b001,
    Write = 0b010,
}
```

Use `konst = none` to generate plain implementations by default, for instance
when the validator cannot be const:

```rust
// Not a const fn:
fn is_valid(it: u32) -> bool {
  return true;
}

#[typekin::integral(konst = none)]
struct Counter(u32);
```
