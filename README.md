# Typekin

`typekin` is a Rust proc-macro crate for building **zero-cost newtypes** around
integral primitives (`u8`..`u128`, `i8`..`i128`, `usize`, `isize`), with an API
modeled after **C++'s `friend` declarations**.

A newtype wrapping, say, a `u32` is normally an island: it doesn't compare, add,
or convert with anything but itself, and you have to hand-write every `impl` to
bridge it back to the outside world. `typekin` generates all of that boilerplate
for you, and — like a C++ class granting `friend` access to hand-picked types —
lets you explicitly declare which other types are "friends" of your newtype and
what they're allowed to do with it (construct, compare, do arithmetic/bitwise
ops with it, etc).

Everything is generated at compile time via a single attribute macro,
`#[typekin::integral(...)]`, applied to a single-field tuple struct.

By default, the macro requires nightly Rust as it relies on const-trait-impl and
related unstable features, but all const features can be turned off.

---

## Table of contents

- [Concept: friends, not open access](#concept-friends-not-open-access)
- [Quick start](#quick-start)
- [What gets generated](#what-gets-generated)
- [Friendship levels](#friendship-levels)
- [Macro configuration](#macro-configuration)
- [Requirements](#requirements)
- [Development](#development)
- [License](#license)

---

## Concept: friends, not open access

In C++, a class can `friend` another class or function, granting it access to
its private internals without opening those internals up to everyone.

`typekin` brings the same idea to Rust newtypes. By default, a `typekin` newtype
is sealed: nothing outside of it can construct it, compare it, or do arithmetic
with it except itself. You then explicitly grant **friendship** to specific
types (primitives or other `typekin` newtypes), at a chosen **level**, which
decides what that friend is allowed to do:

- Construct your type from theirs (`YourType::of(their_value)`).
- Compare against your type (`==`, `<`, `>`, ...).
- Do arithmetic with your type (`+`, `-`, `*`, `/`, `%`).
- Do bitwise ops with your type (`&`, `|`, `^`, `!`).

This is implemented under the hood with a sealed marker trait (`Seal`) plus
per-capability marker traits (`FriendMake`, `FriendMathOps`, `FriendMathBit`,
`FriendMathRel`) that only friended types implement — so the compiler enforces
the friendship, there's no runtime cost, and non-friended types simply fail to
type-check against your newtype.

## Quick start

```rust
// Enable the following nightly features, if building for const context.
// #![feature(const_cmp)]
// #![feature(const_trait_impl)]
// #![feature(const_ops)]
// #![feature(const_convert)]
// #![feature(const_clone)]
// #![feature(const_destruct)]

#[typekin::integral(
    friends = [u32],
    with_const = false,
)]
pub struct Bar(u32);

#[typekin::integral(
    friends = [
        u32(conv = self, level = Full),
        Bar(conv = Bar::raw, level = Full),
    ],
    with_const = false,
)]
pub struct Foo(u32);

fn main() {
    let foo = Foo::of(113u32);
    let bar = Bar::of(100u32);

    let sum: Foo = foo + bar;
    assert!(sum > foo);

    assert_eq!(sum.raw(), 213);

    let as_u64: u64 = bar.into();
    assert_eq!(as_u64, 213u64);

    // Same thing, different syntax:
    let as_u64: u64 = bar.into_u64();
    assert_eq!(as_u64, 213u64);

    // into_u8 is not generated at all as the case is not safe.
    // You get original value back as error if it fails.
    let try_as_u8: Result<u8, Foo> = bar.try_into_u8();
    assert!(try_as_u8.is_err());
}
```

Here, `Bar` is declared as a friend at the `Full` level, meaning `Foo` can be
constructed from a `Bar`, and arithmetic/relational/bitwise operators between
`Foo` and `Bar` are all generated. Any type that is *not* declared a friend
cannot be used to construct, compare, or operate on `Foo` — the sealed traits
prevent it at compile time.

## What gets generated

For a struct such as `pub struct MyExample(u32);` annotated with
`#[typekin::integral(...)]`, the macro generates (among other things):

- `Debug`, `Clone`, `Copy`, `Eq`, `Ord` implementations.
- `PartialEq<T>` / `PartialOrd<T>` for any friended `T` (via `FriendMathRel`).
- `Into<T>` for every integral type the wrapped value safely widens into (e.g.
  `u32` → `usize`, `u64`, `u128`, `i64`,
  `i128`, ...).
- `TryInto<T>` (fallible, checked) for every integral type it doesn't safely
  widen into.
- Arithmetic operators `Add`, `Sub`, `Mul`, `Div`, `Rem` (and their
  `*Assign` counterparts) for any friended `T` (via `FriendMathOps`).
- Bitwise operators `BitAnd`, `BitOr`, `BitXor`, `Not` (and `*Assign`
  counterparts) for any friended `T` (via `FriendMathBit`).
- `Shl<usize>` / `Shr<usize>` (and assign variants).
- A `raw()` accessor returning the wrapped primitive.
- An `of()` constructor generic over any friended type (via `FriendMake`).
- `try_make(...)` / an internal `_make(...)` constructor, optionally routed
  through a user-supplied validator function for fallible construction.
- A compile-time layout assertion ensuring the newtype has the same size as its
  wrapped primitive (i.e. it really is zero-cost).
- The private sealed friendship machinery (`Seal`, `FriendMake`,
  `FriendMathOps`, `FriendMathBit`, `FriendMathRel`) that backs all of the
  above.

Every one of these can be individually toggled off in the generator (see
`crates/typekin/src/integral/maker.rs`), though the public, documented surface
for doing so today is the `friends`/ `fn_get_raw`/`fn_validator`/ `with_const`
macro arguments described below.

TODO: Not all these flags are exposed through the macro.

## Friendship levels

Each entry in `friends = [...]` names a friend type and, optionally, its
friendship `level`. Levels are additive tiers of capability:

| Level     | Grants                                           |
|-----------|--------------------------------------------------|
| `None`    | No friendship (the default if unspecified).      |
| `Make`    | Can construct your type via `of(...)`.           |
| `Rel`     | Can be compared (`==`, `<`, `>`, ...).           |
| `Bit`     | Can do bitwise ops (`&`, `\|`, `^`, `!`).        |
| `Math`    | Can do arithmetic ops (`+`, `-`, `*`, `/`, `%`). |
| `MathRel` | `Math` + `Rel`.                                  |
| `MathBit` | `Math` + `Bit`.                                  |
| `Full`    | `Make` + `Math` + `Rel` + `Bit` (everything).    |

Each friend entry also accepts a `conv` argument describing how to convert a
value of the friend type into the newtype's wrapped element type:

- `conv = self` — the friend type *is* the element type (or trivially derefs to
  it), used as-is.
- `conv = some::path::to::fn` — an arbitrary function/path used to perform the
  conversion.
- Omitted — defaults to using the value as-is.

```rust
#[typekin::integral(friends = [
    u32(level = Full),
    u16(level = Make),
    OtherNewtype(level = MathRel, conv = OtherNewtype::raw),
])]
pub struct MyExample(u32);
```

## Macro configuration

`#[typekin::integral(...)]` accepts the following named arguments
(comma-separated, order-independent):

| Argument       | Type                   | Default     | Description                                                                                                                                |
|----------------|------------------------|-------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| `friends`      | `[FriendReq, ...]`     | `[]`        | List of friend declarations, see above.                                                                                                    |
| `fn_get_raw`   | path of form `Self::x` | `Self::raw` | Name of the generated raw-value accessor.                                                                                                  |
| `fn_validator` | path                   | none        | If set, construction (`of`, `try_make`) is routed through this validator; invalid values cause a panic (`_make`) or an `Err` (`try_make`). |
| `with_const`   | `bool`                 | `true`      | Whether generated impls are `const` (requires the unstable const-trait-impl features).                                                     |

The macro must be applied to a tuple struct with exactly one field whose type is
one of the supported integral primitives.

## Requirements

This crate currently relies on several **unstable** Rust features for its
default const mode (`with_const = true`):

- `const_trait_impl`
- `const_ops`
- `const_cmp`
- `const_convert`
- `const_clone`
- `const_destruct`

You'll need a nightly toolchain and to enable these features in any crate that
uses `#[typekin::integral(...)]` with its defaults (see the example
in [Quick start](#quick-start)). Setting `with_const = false` avoids the need
for these features at the cost of non-const generated code.

## Development

Common tasks are wired up via [`just`](https://github.com/casey/just):

```sh
just build   # cargo fmt && cargo build
just test    # cargo test
just fmt     # cargo fmt
just clean   # cargo clean

# Regenerate crates/typekin/examples/my_u32_expanded.rs from my_u32.rs
# (runs `cargo expand`, then pipes it through the typekin-unexpand binary).
just u32
```

## License

GPL-3, see the `license` field in `Cargo.toml`.
