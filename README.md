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
for covered integral and float operations verify that wrapper code matches
primitive equivalents after inlining. The annotated type is re-exported where it
is declared, so additional inherent implementations and derives remain available
(see [Hiding](#hiding-hide_in)). Currently, Typekin provides:

- friendship protocols for capability-gated construction and operations;
- integer newtypes with validation and generated conversions, mathematical
  operations, and bitwise operations;
- `f32`/`f64` newtypes with the same configuration, NaN-aware validation, and
  `Eq`/`Ord`/`Hash` when validation rules NaN out;
- enum-backed bitflags modeled after the
  [`bitflags`](https://crates.io/crates/bitflags) crate;
- validated `String` newtypes over `std` or `alloc`, which allocate only when
  their underlying `String` operation does;
- configurable const support. By default, all stable const features are enabled.
  Additional constness for math operations requires `konst = nightly`;
- allocation-free generated integral, float, and bitflag code: it requires
  neither `std` nor `alloc` (except float `math = std`) and does not allocate.
  Allocation in caller-provided callbacks, conversions, or Serde backends is
  outside Typekin's generated code;
- individual feature flags for disabling generation of specific features.

Working examples are available in the repository's [examples](./examples)
directory. This crate makes no backward-compatibility guarantees before version
1.0.0.

## Attribute Macro Inventory and Item Requirements

The supported public attribute macros are:

- `#[typekin::integral]`: over a one-field tuple struct whose field is a
  primitive integer. Generic declarations require a private value field.
- `#[typekin::int]`, `#[typekin::sequential]`, `#[typekin::wrapping]`,
  `#[typekin::circular]`, and `#[typekin::delta]`: `integral` with a fixed
  profile.
- `#[typekin::float]`: over a one-field tuple struct whose field is `f32` or
  `f64`. Generic declarations require a private value field.
- `#[typekin::point]`: the fixed `point` profile over either an integer or a
  float field.
- `#[typekin::bitflag]`: over a non-generic unit enum with an integral `repr`.
- `#[typekin::text]`: over a non-generic one-field tuple struct whose field is
  an unqualified `String`.
- `#[typekin::tree]`: over an empty, non-generic marker trait; it emits branch
  traits and zero-sized marker leaves.
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
// With no explicit capability, the default is assumed.
#[typekin::integral(friend = [
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
and `Target::of` passes the resulting relation to `Target::of_parts`. The
`friend` argument accepts one conversion declaration or a bracketed list:

```rust
#[typekin::friends(
    // The two types talk to each other over `u8`-typed values.
    relation = u8,
    // `Source` is a friend of `Target`; it can participate in `Make`
    // operations. That is, its value can be used to construct new `Target`
    // instances. The conversion happens through `Source::into_relation`.
    friend = Source::into_relation(Source) -> Make,
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

Validation has a runtime cost on every checked construction and generated
operation. When the invariant is a development-time diagnostic rather than a
release guarantee, the validator can compile to a no-op outside debug builds,
just as Rust disables overflow checks in release:

```rust
#[inline(always)]
const fn is_div_by_3(raw: usize) -> bool {
    // In debug builds, every checked value must be divisible by 3:
    #[cfg(debug_assertions)] return raw % 3 == 0;

    // Inlines to a no-op and vanishes in release:
    #[cfg(not(debug_assertions))] return true;
}

#[typekin::integral(valid = is_div_by_3)]
struct Foo(usize);

fn main() {
    let lhs = Foo::try_make(6).unwrap();
    let rhs = Foo::try_make(3).unwrap();

    // Panics in debug builds: 6 / 3 = 2, which fails `is_div_by_3`:
    let _: Foo = lhs / rhs;
}
```

Integral wrappers generate `try_map`, which returns the rejected final raw
value, and `map`, which panics when final validation fails. The closure runs
once and intermediate primitive values are not validated. Primitive overflow and
closure panics retain their normal Rust behavior. Use `without = [map]` to omit
both methods.

Operations that cannot leave the domain skip the check. For an integral `domain`
made only of ranges and listed values (no `!{…}` holes, `mul`, functions, or
`valid` callbacks), typekin decides at expansion, per operation and
right-hand-side type and at every pointer width, whether every result is valid.
Proven operations compile to the bare primitive operation and document "Cannot
fail validation"; the rest keep their check. The `proven` flag runs this
analysis: it is on by default for `u8` and `i8`, and wider integers opt in with
`with = [proven]` (`without = [proven]` turns it off for 8-bit types). `prove`
makes this a contract, implies `proven`, and fails expansion with a
counterexample:

```rust
#[typekin::integral(domain = ..=100, prove = [math_div, math_rem, midpoint])]
pub struct Percent(u8);

#[typekin::integral(with = [proven], domain = 1..=1023 | 49_152..=65_535)]
pub struct Port(u16);
```

```text
// with `math_add` added to `prove`:
error: `Percent + Percent` is not closed: 1 + 100 = 101 is outside `..=100`
```

`Port | Port` is proven here, but `Port | u16` is not, so it keeps its check.
Without `with = [proven]`, both validate. The runnable
[proofs example](examples/my_proven.rs) uses both types.

## Friendship

### Friendship is consuming

Friend capabilities gate construction and operations. Integral automatically
includes itself and its backing integer for operations, and grants raw/widening
construction when unvalidated; bitflag registers its enum/value pair. Additional
friends are declared explicitly. Conversions consume their inputs. Text equality
and ordering remain same-type operations; integral comparisons against another
type require a `Rel` friend.

### Blind-trust friend construction

Friends without `unsafe` are untrusted. Integral and text `of` validate their
converted values and panic if invalid. `unsafe` implies `Make` and skips only
that friend-construction validation, through the generated `_blind_trust`
constructor: the friend author must prove the converted value is already valid.
`with = with_blind_trust` emits `_blind_trust` without an `unsafe` friend, for
code in the defining module that has already established validity.

`unsafe` does not imply `Math`, `Bit`, or `Rel`. Validating constructors and
math/bitwise operation results still validate, including operations with a
blind-trust friend as the right-hand operand. Text `unsafe` friends require an
explicit conversion returning `String`, just like explicit `Make` friends.

Standalone `friends` generates `Self::of` for the relationship and every
declared friend; targets provide `fn of_parts(relation) -> Self`, and `scope`
controls where the generated protocol is placed. It has no generated validator
to bypass: the configured `of_parts` function always runs, including its own
checks. Relations and friend inputs can both be non-`Copy`.

### Proven friendship (`Within`)

`_(Source) -> Within` implies `Make` and builds the target without validation,
because rustc proves at compile time that every value of the source lies in the
target's domain. `of` compiles to a plain cast, and a failed proof is a compile
error naming the least counterexample:

```rust
#[typekin::integral(domain = ..=100)]
pub struct Percent(u8);

#[typekin::integral(
    domain = ..=255,
    friend = [_(Percent) -> Within, _(u8) -> Within],
)]
pub struct Byte(u16);

// `_(Byte) -> Within` on `domain = 1..` fails with:
// `Byte` is not within `Positive`: 0 is outside the domain of `Positive`
```

Sources are integral wrappers, bitflag value types, or primitive integers.
Wrappers publish hidden constants for the proof unless
`without = within_source`. `Within` converts through the raw value, so it takes
no explicit conversion and cannot be combined with `unsafe`. `valid` callbacks
are opaque to the proof except for 8-bit sources, whose validation table rustc
checks against the target's own validation. Generic targets prove where they are
instantiated, so `cargo build`, not `cargo check`, reports their failures.

## Hiding (`hide_in`)

By default, `integral` (and its profile entry points), `float`, `text`, and
`bitflag` place the annotated type and everything generated for it in a private
module and re-export the type where the macro was written. The tuple constructor
and a private value field are then unreachable even from the defining module, so
the only ways in are the generated, validating constructors.

```rust
#[typekin::integral(domain = 1..=1023)]
pub struct Port(u16);
// expands roughly to:
//   mod __port { use super::*; pub struct Port(u16); /* impls */ }
//   pub use __port::Port;

fn main() {
    let port = Port::try_make(443).unwrap();
    assert_eq!(port.raw(), 443);
    // `Port(5)` and `port.0` do not compile, even in this module.
}
```

- The module is `__` plus the snake-cased type name (`PageHeader` →
  `__page_header`); `hide_in = name` picks another. `hide_in = none` keeps the
  type where it is, exactly as without the option.
- The module starts with `use super::*`, so callbacks, friend types, and private
  helpers of the defining module resolve as usual. `self::` and `super::` paths
  in the configuration and item are adjusted for the extra level; paths inside
  string literals are not, so a non-doc attribute string starting with `super::`
  is rejected.
- `get_raw` and a custom `_or_panic` constructor need direct field access and
  are rejected while hiding is on.
- A declaration inside a function body cannot see items declared in that
  function from the module; move them to module level or use `hide_in = none`.
- Place additional `derive` attributes below the Typekin attribute. A derive
  written above it cannot reach the private field.

## Integral - `#[typekin::integral]`

### Generic integral identities

```rust
#[typekin::integral]
struct Tagged<'a, T: ?Sized = (), const SPACE: u8 = 7>(u32)
where
    T: 'a;

fn main() {
    let value = Tagged::<'static, str, 7>::make(42);
    let copied = value;
    assert_eq!(copied.raw(), value.raw());
    assert_eq!(core::mem::size_of_val(&value), core::mem::size_of::<u32>());
}
```

Integral declarations accept lifetimes, types, and const parameters, preserving
their attributes, bounds, defaults, and `where` clauses. The source must still
have exactly one concrete primitive integer field, not a generic numeric field.
For generic declarations that field must be private: Typekin appends one private
`core::marker::PhantomData` tuple field per parameter, in declaration order, so
direct tuple construction has a different arity. Use `make`, `try_make`, or `of`
rather than the tuple constructor.

Lifetime markers use `PhantomData<&'a ()>`, type markers use `PhantomData<T>`,
and const parameters use private marker types with the declared const type.
Const-parameter types requiring nightly features need those gates in the
consumer. Type markers have Rust's normal ownership, variance, drop-check,
`Send`, and `Sync` behavior. Configured generated `Copy` and `Clone` impls do
not require marker types to implement either trait; user-written bounds still
apply. Formatting, hashing, parsing, and Serde likewise operate only on the
numeric value.

Markers add no runtime state, size, alignment, allocation, or wire data. Storage
selection still affects only field zero, and `raw()` returns the primitive even
with `NonZero` or `Wrapping` storage. Friendship protocols and operator facets
retain all generic arguments, so a friend of one instantiation is not implicitly
a friend of another. Non-generic declarations acquire no marker fields. This
generic support is specific to `integral` and `float`, not `text`, standalone
`friends`, or `bitflag`.

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

An unvalidated integral has raw `make`. Configuring `domain` or `valid`
suppresses that constructor and exposes `try_make(raw) -> Result<Self, Raw>`
instead.

`domain` is the set of accepted raw values, written as a set expression. Its
parts are ranges (`2..=4`, `87..`, `..10`), listed values (`{5}`, `{1, 3, 7}`),
multiples (`mul(3)`), predicates (`is_power_of_two`, `is_positive`,
`is_negative`), and the values for which a pure integer function lies in a set
(`abs in ..=1000`, `count_ones in {1}`, `rem_euclid(16) in {3}`) or outside it
(`abs !in ..=1000`). Combine them with `!` (complement), `&` (intersection), `|`
(union), and parentheses. One precedence rule applies: `&` binds tighter than
`|`, as in Rust, so `..5 | 10.. & ..20` is `..5 | (10.. & ..20)`. Wherever else
two readings are possible, the domain is rejected and asks for parentheses: an
`in` set is never followed by `|` or `&` (`abs in (..5 | 10..)` or
`(abs in ..5) | 10..`, not `abs in ..5 | 10..`). `!` takes the set right after
it, so `!1..5` needs no parentheses. A function's set is excluded with `!in`;
a leading `!`, as in `!abs in ..3`, is rejected.

```rust
// In 1..12 or 87.., but not 10, and not a multiple of 3:
#[typekin::integral(domain = (1..12 | 87..) & !{10} & !mul(3))]
struct Code(u32);

fn main() {
    assert!(Code::try_make(7).is_ok());
    assert!(Code::try_make(9).is_err());
    // Zero is outside the domain, so `Code` uses `NonZeroU32` storage:
    assert_eq!(size_of::<Option<Code>>(), size_of::<Code>());
}
```

Constants are integer literals (an optional suffix must match the operand type)
or the operand type's `MIN`/`MAX`, optionally negated; arithmetic, parenthesized
constants, other paths, and calls are rejected, so `1..10 - 2` is an error, not
`1..8`. A single value is a set only in braces: `{5}`. A function's result type
types its constants: `abs` on `i32` yields `u32`. Divisors must be at least 2.
Constants outside the type fold away per pointer width, so `-5..10` on `u8`
checks only `< 10`. Typekin merges and sorts overlapping or adjacent ranges and
drops redundant exclusions (`!{c}`), never adding a comparison, and rejects a
domain that admits every value or none. The domain is checked before `valid`
callbacks; both must accept the value. Generated construction and enabled
mathematical or bitwise operations reject values that fail them.

When the domain excludes zero, the generated private field uses the matching
`core::num::NonZero*` type. `with = [non_zero]` adds a wrapper-wide nonzero
constraint and selects that storage even when no `domain` is declared.
`without = [non_zero]` opts out of `NonZero*` storage only: it leaves the
declared domain and its validation unchanged.

```rust
#[typekin::integral(domain = 1..=1023 | 49_152..=65_535)]
struct Port(u16);

fn main() {
    assert_eq!(Port::try_make(443).map(Port::raw), Ok(443));
    assert_eq!(Port::try_make(1024), Err(1024));
}
```

`with = range` emits the nightly `core::iter::Step` implementation, allowing
ordinary wrapper ranges to iterate values. It requires
`#![feature(step_trait)]`; `konst = nightly` also makes that implementation
const. `Step` cannot represent validation holes, so `range` is rejected with
`domain`, `valid`, `non_zero`, or a circular profile.

```rust
#![feature(step_trait)]

#[typekin::integral(konst = none, with = range)]
struct Ordinal(u8);

fn main() {
    let values: Vec<_> = (Ordinal::make(2)..=Ordinal::make(4))
        .map(Ordinal::raw)
        .collect();
    assert_eq!(values, [2, 3, 4]);
}
```

`error = ()` is accepted by `integral`, `float`, `text`, and a bitflag's nested
`integral` configuration. Unit is currently the only supported error type; the
option reserves the configuration surface without changing existing constructor
error values.

### Compile-time literals

Every validated `integral`, `float`, and bitflag value type with a const
`try_make` gets a literal macro named after its snake-cased type (`Port` →
`port!`, `PermValue` → `perm_value!`). It builds the wrapper from a constant; an
invalid constant fails the build, including under `cargo check`, and a valid one
leaves no panic path.

```rust
#[typekin::integral(domain = 1..=1023 | 49_152..=65_535)]
pub struct Port(u16);

const HTTPS: Port = port!(443);

fn main() {
    let ssh = port!(22);
    assert_eq!((HTTPS.raw(), ssh.raw()), (443, 22));
}
```

```text
error[E0080]: evaluation panicked: 0 is outside the domain of Port
```

- `port!(EXPR)` checks `EXPR` in a block-level `const` item. `port!(const EXPR)`
  uses an inline `const` block, which generic code needs: `packet!(const 1)`
  infers `Packet<D>`'s arguments, `packet!([Out] 7)` names them. `cargo check`
  does not report an invalid inline form in a fn body; `cargo build` does.
- The expansion names the type as written, so import the type beside the macro,
  or set `lit_path = crate::net::Port` to make the macro self-contained.
- `lit_name = name` renames the macro; a type whose macro name would be a
  keyword or shadow a prelude macro (`Ref`, `Line`) must set it.
  `without = lit_macro` omits the macro.

### Integral production and profiles

`#[typekin::integral]` can use `produce` to configure one result type for all
non-assignment operators:

```rust
#[typekin::delta(konst = none)]
struct Offset(i64);

#[typekin::integral(
    konst = none,
    produce = Point::into_offset(_) -> Offset,
)]
struct Point(u32);

impl Point {
    fn into_offset(self) -> Offset {
        return Offset::make(self.raw().into());
    }
}

fn main() {
    let difference: Offset = Point::make(7) - Point::make(3);
    assert_eq!(difference.raw(), 4);
}
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
| `int` | Primitive Rust arithmetic | Default `Ord` / `PartialOrd` behavior |
| `sequential` | Checked; panics on overflow or underflow | Ordinary linear ordering |
| `wrapping` | `Wrapping` arithmetic | Ordinary linear ordering |
| `circular` | Primitive `wrapping_*` arithmetic | No `Ord` / `PartialOrd`; serial `precedes` |
| `delta` | Primitive `wrapping_*` arithmetic | No `Ord` / `PartialOrd`; no `precedes` |
| `point` | Unchanged; primitive arithmetic when used alone | Unchanged |

Each profile also has a fixed-profile attribute macro: `#[typekin::int(...)]`,
`#[typekin::sequential(...)]`, `#[typekin::wrapping(...)]`,
`#[typekin::circular(...)]`, `#[typekin::delta(...)]`, and
`#[typekin::point(...)]`. Each is equivalent to
`#[typekin::integral(profile = NAME, ...)]`; use `integral` for profile lists.
Its arguments are ordinary integral arguments, while `circular` additionally
accepts `precedes` directly. The selected profile cannot be overridden with
`profile = ...`.

For `circular`, `delta`, and `sequential`, same-type `+`, `-`, and their
assignments remain enabled. Primitive arithmetic, bitwise operators, shifts,
`core_int`, and bit access are disabled by default. `wrapping` retains the
normal integral API surface. A profile body can use `with` and `without` to
change its operation set; re-enabled arithmetic uses that profile's wrapping or
checked policy. Bitwise operations retain ordinary bitwise semantics.

`point` models positions rather than offsets. It uses the same restricted API as
`circular`, `delta`, and `sequential`, additionally disables `+` and `+=`, and
leaves subtraction enabled. Compose it with an arithmetic profile, such as
`[circular, point]` or `[sequential, point]`. It preserves that profile's
arithmetic, storage, ordering, and `precedes` setting. Configure `produce` for
point differences and explicit facets for point-plus-offset and
point-minus-offset; the offset type and conversions are not inferred.

`delta` inherits `circular` and fixes `precedes = false`. Use it for signed or
unsigned relative changes, where serial precedence is meaningless. It keeps
circular's primitive storage and wrapping arithmetic while omitting the
`precedes` method.

`wrapping` stores its private field as `core::num::Wrapping<P>` and keeps
ordinary `Ord` and `PartialOrd`. `circular` keeps primitive storage, removes
linear ordering, and emits `precedes`. `delta` keeps the same arithmetic and
storage but omits serial precedence. All three profiles wrap at the primitive
width, but they intentionally model different APIs. A nonzero domain combined
with `wrapping` uses `Wrapping<NonZeroP>` while `raw()` and all public
conversion, parsing, and serialization APIs continue to use `P`.

Physical storage rewriting requires a private tuple field. A configured
`get_raw` accessor must still return `P`. Blind-trust friends may bypass
semantic validation but cannot construct zero in `NonZero*` storage. A custom
`_or_panic` constructor (`without = fn_or_panic`) is therefore unavailable when
nonzero storage is selected.

`profile` accepts one profile or an ordered list. Resolution starts from the
default integral profile, then applies each profile's defaults and body in list
order. Untouched settings carry forward, later settings win, and root-level
`with` and `without` are applied last. The last arithmetic profile selects the
arithmetic policy; a following `point` profile does not replace it.

Empty bodies are optional for profiles and facets: `profile = circular`,
`profile = [circular, point]`, `profile = delta`, and `facet = circular` use
defaults. Keep braces for configuration, such as `delta { with = math_mul }`.

`facet` accepts one declaration directly or a bracketed list. Facets select
arithmetic and output conversion by the exact RHS type. Slots not claimed by an
explicit facet use the root profile. A facet inherits root flags; use `with` to
re-enable an operation disabled at the root. A `point` facet also inherits root
arithmetic because it does not select an arithmetic policy.

`tree` uses its annotated trait as a seal rather than a marker node. It emits
branch traits and zero-sized leaves that implement every ancestor branch. Each
branch extends its semantic parent and the seal.

```rust
#[typekin::tree(
    Direction {
        In,
        Out,
    },
)]
trait MakerSeal {}

#[typekin::circular(
    konst = none,
    without = math_add,
    produce = Self::into_offset(_) -> SeqOffset<D>,
    facet = circular {
        with = math_add,
        friend = Self::offset_raw(SeqOffset<D>) -> Math,
        produce = Self,
    },
)]
struct SeqPoint<D: Direction>(u32);

#[typekin::delta(konst = none)]
struct SeqOffset<D: Direction>(i32);

impl<D: Direction> SeqPoint<D> {
    fn into_offset(self) -> SeqOffset<D> {
        return SeqOffset::make(self.raw() as i32);
    }

    fn offset_raw(offset: SeqOffset<D>) -> u32 {
        return offset.raw() as u32;
    }
}

fn main() {
    let maximum = SeqPoint::<Out>::make(u32::MAX);
    let next = maximum + SeqOffset::<Out>::make(1);
    let difference: SeqOffset<Out> = next - maximum;
    assert_eq!(next.raw(), 0);
    assert_eq!(difference.raw(), 1);
}
```

`precedes` compares the backing bit patterns around the wrapping circle.
Equality and values exactly half the numeric space apart do not precede each
other. Use `raw()` for deliberate linear bit-pattern ordering. Matching `In`
operations work independently; mixing `SeqPoint<Out>` with `SeqOffset<In>` or
`SeqPoint<In>` does not compile. `Self`-qualified callbacks refer to the LHS
wrapper even when dispatched through an RHS implementation.

Omitted node visibility inherits its parent; a top-level node inherits the
seal's visibility. Add a visibility only where it differs, such as
`pub(crate) In`.

The runnable [TCP sequence example](examples/my_tcp.rs) models directional
sequence points and offsets, including typed `SND.NXT`, `RCV.NXT`, and `SND.UNA`
state. Run `cargo +stable run --example my_tcp`. It demonstrates sequence
arithmetic and stream separation, not a TCP transport.

`precedes` is only valid inside `circular`; it defaults to `true`. `delta`
always omits it, while `int`, `sequential`, and `wrapping` reject it. Circular
and delta profiles also reject explicit `ord` or `partial_ord`. Profiles do not
apply to generated bitflag value types; nested bitflag integral configuration
supports only the default profile and no `produce`. Bitflag domains must include
zero, and nested bitflag configuration cannot enable `non_zero`.

### Combined configuration

```rust
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_destruct)]
#![feature(derive_const)]

struct Foo(u32);
impl Foo { const fn to_u32(self) -> u32 { self.0 } }
const fn is_valid(value: u32) -> bool { value != 0 }

#[typekin::integral(
  konst = nightly,         // Enables all const flags.
  friend = [
    self(u16) -> Bit,      // Only bitwise operations, using `Into<u32>`
    Foo::to_u32(Foo),      // Omitted capabilities default to Math
  ],
  without = [fn_conv_raw], // Do not generate the existing accessor below
  get_raw = Self::unwrap,  // Use an existing accessor instead of generated `raw()`
  hide_in = none,          // `unwrap` reads the field, so the type stays unhidden
  valid = is_valid,        // Reject invalid raw values in checked construction
)]
struct Example(u32);

impl Example {
    const fn unwrap(self) -> u32 { self.0 }
}
```

Capabilities are explicit: `Make` gates `of`, `Bit` gates bitwise operations,
and `Math` enables construction, arithmetic, and bitwise operations. `Rel`
enables `==`, `!=`, `<`, `<=`, `>`, and `>=` against `Copy` friends. `Within`
grants `Make` without validation once proven (see "Proven friendship"). Integral
friends with no `->` capability clause default to `Math`; write `-> []` to grant
no capabilities.

### Embedded marker trees

`integral`, `float`, `bitflag`, and `text` accept `tree = NODE` for one root
node or `tree = [NODE, ...]` for several. `NODE` uses `tree`'s node grammar. An
embedded tree creates a private `MarkerTreeRoot` seal in a hidden generated
module and re-exports its nodes beside the annotated type, so the nodes can be
used in the rest of the enclosing module while remaining sealed.

Omitted root-node visibility inherits the annotated type's visibility; omitted
descendants inherit their parent. Explicit visibility overrides that rule.

```rust
#[typekin::integral(
    tree = Direction {
        In,
        pub(crate) Out,
    },
)]
pub struct PacketNumber<D: Direction>(u32);
```

## Floats - `#[typekin::float]`

`float` takes the integral configuration (`konst`, `valid`, `domain`, `error`,
`friend`, `with`, `without`, `get_raw`, `produce`, `facet`, `tree`, `hide_in`,
and the `int` or `point` profile), adds `math`, and applies float semantics:

```rust
#[typekin::float(konst = none)]
pub struct Meters(f64);

fn main() {
    let width = Meters::make(2.5);
    assert_eq!((-width * 2.0).raw(), -5.0);

    // Checked conversions reject rounding, NaN, and out-of-range values.
    assert_eq!(Meters::make(4.0).try_into_u8(), Ok(4));
    assert_eq!(Meters::make(4.5).try_into_u8(), Err(()));
    assert_eq!(Meters::make(f64::NAN).try_into_i64(), Err(()));

    let total: Meters = [width, width].iter().sum();
    assert_eq!(format!("{total:e}"), "5e0");
}
```

Ranges, listed values, and multiples in a float `domain` reject NaN; their
complements admit it (`!{0.5}`). There is no `>`: `0.0.. & !{0.0}` accepts the
positive values and compiles to `> 0.0`. A domain that rejects NaN makes the
type totally ordered: with a private value field it gets `Eq`, `Ord`, and `Hash`
(`-0.0 == 0.0`, and both hash alike).

```rust
use std::collections::BTreeSet;

#[typekin::float(domain = 0.0..=1.0)]
pub struct Unit(f64);

fn main() {
    assert!(Unit::try_make(f64::NAN).is_err());

    let half = Unit::try_make(0.5).unwrap();
    let set = [half, half].into_iter().collect::<BTreeSet<_>>();
    assert_eq!(set.len(), 1);
}
```

The catches:

- A type that can hold NaN (`domain = !{0.5}`, callbacks alone, no domain) has
  no `Eq`, `Ord`, or `Hash`; requesting them is an error, as is requesting them
  with a public value field.
- `Sum` and `Product` fold raw values like `f64`'s own implementations and
  validate once, at the end. An empty sum is `-0.0`.
- There are no bitwise operations, `non_zero` storage, `Step`, or circular,
  delta, sequential, and wrapping profiles. `Math` friends grant arithmetic and
  `Make` only. Primitive `Make` friends convert with `as` and may round; `Rel`
  and `Math` friends must convert exactly.
- `Neg` and `abs` skip validation when the domain makes the result valid by
  construction (`-1.0..=1.0` for `Neg`; also `0.0..` for `abs`). Signed integers
  get `Neg` the same way; unsigned integers have none.
- Integers convert to floats: `into_f64` where every value is exact,
  `try_into_f32` (and the like) otherwise, which rejects rounding.
- `floor`, `sqrt`, and friends are not in stable `core`. `math = std` enables
  them in domains and as methods through `std`; `math = nightly` uses
  `core::f64::math` (no_std, `#![feature(core_float_math)]`). Divisors such as
  `mul(0.25)` must be exactly representable; `mul(0.1)` is an error.

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
    domain = {"draft", "news", "medical-news"},
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

## Serde

Use `with = serde` on `integral`, `float`, `text`, or `bitflag` to generate
`Serialize` and `Deserialize`. Deserialization applies every configured
validator. Typekin has no Serde Cargo feature and does not provide Serde as a
normal dependency: the consuming crate must add `serde` directly and apply any
desired `cfg` gating to the macro invocation.

Integral and float wrappers serialize as their raw number, while text wrappers
serialize as strings. Do not also derive the same Serde traits on a wrapper
using this option.

## Configuration

Generation of individual features can be disabled with `without`; see the flag
lists in [integral](src/integral.rs), [bitflag](src/bitflag.rs), and
[text](src/text.rs).

## Const mode

`integral`, `float`, `bitflag`, and `text` default to `konst = stable`. Set
`konst = none` to disable optional const flags, or `konst = nightly` to enable
const trait implementations and nightly-only integer operations. For integral,
float, and bitflag, `stable` makes checked construction, raw value access, and
core integer and float methods const. For text, it enables const read-only
accessors; `nightly` additionally enables supported owned construction and
conversion. Enabling Typekin's `nightly` Cargo feature changes the default to
`konst = nightly`, and `with`/`without` accept individual `konst_*` flags.

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

#[typekin::integral(konst = none, valid = is_valid)]
struct Counter(u32);
```
