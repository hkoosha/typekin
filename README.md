# Typekin

`typekin` is a proc-macro crate for generating following no\_std code:

- `#[typekin::integral]`: integer new-types with explicit interoperability
- `#[typekin::bitflag]`: enum-backed flags plus a generated value type (modelled
  after pkg:cargo/bitflags@2.13.1)

The generated code will always be zero-dependency and is fully no\_std, and
everything is `const` by default. The dependency on typekin proc-macro itself
can be also dropped via expanding the generated code (cargo-expand) and
reformatting the output to a readable form via the `main.rs` in this macro.

The important rule is simple: The type only interoperates with the types listed
in `friends = [...]`.

This repository already contains some working examples:

- `crates/typekin/examples/my_u32.rs`
- `crates/typekin/examples/my_u32_non_const.rs`
- `crates/typekin/examples/my_flag.rs`

AI Disclaimer: The code is handwritten, but the README is AI generated, check
the AI prompts at the end.

## Constness Status

By default, the generated code is **nightly-only** due to `const` features, but
the `const impls` can be disabled with the proc-macro attribute, which makes the
code valid in stable rust without any unstable features:

```
without = [konst]
```

## `#[typekin::integral]`

Applied to a single-field tuple struct over an integral primitive (and annotated
with `#[repr(transparent)]`) turns into a bitflag very similar to
how https://crates.io/crates/bitflags works, albeit separating individual flags
as enum and keeping their combined values in a dedicated type.

### Demo

```rust
#[typekin::integral(
  friends = [
    u32,                   // Full friendship level
    Money(level=Rel),      // Only math [rel]ational ops (comparison) are allowed
    Foo(level=Custom)      // Custom relationship, does not add particular impl.
    Bar(conv=Bar::thingy)  // If a type requires particular conversion logic,
                           // and the default `Into` is not appropriate
  ]
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Amount(u32);

struct Bar(u64);
impl Bar {
    // Referenced by Bar friendship above. Must return the underlying type of
    // Amount (i.e. u32):
    // We could have also defined this on Amount
    const fn calculate_thingy(it: &Self) -> u32 {
        return it.0 * 2;
    }
}

fn main() {
    // Amount can be constructed directly from u32, since it's declared as a
    // friend:
    let a = Amount::of(10u32);

    // Otherwise one must handle failures:
    let b = Amount::try_make(20i32).unwrap();

    assert_eq!(a + b, Amount::of(30));
    assert_eq!((a + b).raw(), 30);
    assert!(b > a);

    // Since by default a fiend type is at `Full` level, mathematical operations
    // are allowed:
    assert_eq!(a + 5u32, Amount::of(15));

    let widened: u64 = a.into();
    assert_eq!(widened, 10);

    // All widening casts have handy aliases:
    assert_eq!(a.into_u128(), 10);

    // But narrowing casts must be checked:
    assert_eq!(a.try_into_u8(), Ok(10u8));
    assert_eq!(a.try_into_i32(), Ok(10i32));

    let money = Money::of(3u64);

    // This will not compile, as Money is not a friend of Amount:
    // let _ = amount + money;

    // But this friendship is declared and compiles:
    assert!(money < amount);
}
```

Some supported operations:

```rust
fn main() {
    // Construction and conversion:
    Amount::of(1u32);
    Amount::try_make(1i32)?;
    value.raw();
    value.into_u64();
    value.try_into_u8();

    // Generated operators, when friendship allows them.
    // A type is always friend of itself.
    value + other;
    value - other;
    value * other;
    value / other;
    value % other;
    value & other;
    value | other;
    value ^ other;
    !value;
    value << 1;
    value >> 1;
}
```

### Friendship is explicit

If a type is not listed in `friends`, it does not get to construct, compare
with, or operate on your new-type.

```rust
#[typekin::integral(
  friends = [u32(conv = self, level = [Full])],
  without = [konst],
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Quantity(u32);

#[typekin::integral(
  friends = [
    u32(conv = self, level = [Make]),
    Quantity(conv = Quantity::raw, level = [Rel]),
  ],
  without = [konst],
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
  friends = [u16(conv = self, level = [Make, Rel])],
  fn_validator = valid_port,
)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct Port(u16);

fn main() {
    assert_eq!(Port::try_make(8080), Ok(Port::of(8080u16)));

    // You get same value back as error.
    // Not having error messages is explicit design choice, to keep the types simple
    // and encourage more meaningful and domain specific names (that is, one should
    // typically grasp what's an allowed value from the name):
    assert_eq!(Port::try_make(0), Err(0));

    // Since we declared u16 as a friend on Make level, `of(u16)` is possible,
    // but it still may panic due to the validator func.
    let _ = Port::of(0u16);
    panic!("unreachable");
}
```

### Attribute shape

The syntax you will most likely need looks like this:

```rust
#[typekin::integral(
  friends = [
    u32,                     // Full level by default
    u64(level = [Bit]),      // Only bit operations allowed
    Foo(conv = Foo::to_u32), // Full level, but custom value conversion
  ],
  
  // One can rename default name to other than `raw()`.
  fn_get_raw = Self::unwrap, 
  
  // If the type has a validator, in which case, infallible constructors
  // are not exposed
  fn_validator = is_valid,
  
  // Currently needed to build for stable rust
  without = [konst],
)]
#[repr(transparent)]
#[Derive(Copy, Clone)]
struct Example(u32);
```

Friend levels used by the macro:

```
- Full: Shorthand for Make + Rel + Bit + Math
- Rel
- Bit
- Math
- Make
- None
- AnythingElse: Not used by Typekin, but available at type level for custom logic

TODO: prevent feature clash, either use Custom(Anything) or prefix it with Custom
```

---

## `#[typekin::bitflag]`

It is modeled (and mostly copied) from pkg:cargo/bitflags@2.13.1, but with some
different design decisions. Ihe macro is applied to a unit enum with an integral
`repr`. The macro generates:

- the enum you wrote
- a value type, by default named as `<EnumName>Value`
- flag/value helpers such as `name()`, `items()`, `from\_name()`, `contains()`,
  `iter()`

The enum is a bitwise and relational friend of its generated value type. Mixed
bitwise operations (`Perm::Read | value`, `value | Perm::Read`) return
`PermValue`; a combined bit pattern is never coerced back into the enum.
Arithmetic friendship is intentionally not generated.

`bitflag` also accepts its own `friends = [...]` list, independent from
`integral = [friends = ...]`. These friends are available to enum-left bitwise
operations, so `Perm::Read & friend` returns `PermValue`. The generated enum
and value are registered automatically.

### Example

```rust
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[typekin::bitflag]
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
    // Anything inside this attribute is directly passed to typekin::integral
  integral = [
    friends = [u8],
    without = [konst]
  ]
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

Const generation is on by default. Eventually when rust stabilizes all const
features, the generated code will work on stable rust out of the box, but until
then, enabling these features (as of writing) are required:

```rust
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_destruct)]
#![feature(derive_const)]

#[typekin::integral]
#[repr(transparent)]
#[derive(Copy)]
#[derive_const(Clone)]
pub struct Counter(u32);

const START: Counter = Counter::of(10u32);
const NEXT: Counter = START + 1u32;
```

But `const impl` can be skipped in favor of plain implementations, by disabling
the const flag:

```rust
#[typekin::integral(
  friends = [u32(conv = self, level = [Full])],
  without = [konst],
)]
struct Foo;
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
```

## Development

```
2026-08-01: Read this project and create comprehensive readme. This is crate for
the new-type pattern, borrowing the concept of `friend classes` from c++.
---
2026-08-09: This project has evolved, rewrite the readme, and do not make it
fluff or an ad. Just say what's important, and more importantly try to
demonstrate with code example rather than text.                                                                                           
NOTE: this version is manually modified, might have typos.
```

