# Text Type Plan

## Goal

Add `#[typekin::text]` for validated transparent newtypes over
`alloc::string::String`. A text value must never escape in an invalid state
through normal APIs. A `trusted` friend is a deliberate contract exception:
its author guarantees validity externally while generated code skips validation.

```rust
#[typekin::text(valid = is_slug)]
#[repr(transparent)]
pub struct Slug(String);

fn is_slug(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
}
```

## Constraints

- `String` belongs to `alloc`, not `core`; generated text types require an
  allocator. `std` consumers get this through `std`, while `no_std` consumers
  need `alloc` available.
- The macro must accept only a transparent tuple struct with exactly one
  `String` field.
- Text validation callbacks take `&str` only, because a `String`-by-value
  callback would require cloning or consume the value being constructed.
- Text has its own `in` parser; it does not reuse integral ranges. Its grammar
  is `in = ["literal", ...]`, requires one or more string literals, rejects
  duplicates, and performs exact string membership checks.
- Callback validation and `in` membership compose with logical AND. Every
  normal construction and transformation path must apply both.
- `konst = true` follows integral's opt-in emission policy: apply the existing
  `const` / `[const]` generation to every relevant text path, including `map`
  and validation. The macro does not pre-reject callbacks, closures, allocation,
  or `String` operations based on const support; the caller's nightly toolchain
  and enabled features determine whether generated code compiles.
- Friendship consumes values. A friend conversion takes the friend by value and
  returns `String`; the annotated text type consumes that `String`.
- Friends default to untrusted. An untrusted `Make` friend validates its output
  and panics when it is invalid. `trusted = true` skips that validation and
  transfers the validity obligation to the friend author.
- Text permits only `Make` and `Rel` friendship capabilities. `Rel` means the
  normal `PartialEq`, `Eq`, `PartialOrd`, and `Ord` behavior over the text type's
  underlying string; it does not add math or bitwise operations.
- Unsupported attributes must be rejected rather than silently ignored.
- The invariant prevents exposing mutable aliases to the backing string.
  `DerefMut<Target = str>`, `AsMut<str>`, `as_mut_str`, and APIs returning
  `&mut String` cannot and MUST NOT be generated.
- The wrapper has transparent layout and no extra runtime state. Generated
  wrappers are marked `#[inline(always)]` where applicable.

## Friendship Ownership Migration

Before adding text friendship, change the shared friendship protocol from a
borrowed conversion to an owning one:

```rust
trait Seal {
    fn conversion(self) -> Relation;
}
```

Remove the blanket `Seal` and capability implementations for `&T` and `&mut T`.
Callers that own their input move it into the conversion; `Copy` values continue
to work naturally because passing them by value copies them. Update `friendship`,
`integral`, and `bitflag` together, including their generated examples and tests.

Rust's `PartialEq` and `PartialOrd` trait methods receive a borrowed right-hand
side and therefore cannot use an owning friendship conversion. The migration
removes friendship-based cross-type relation conversion from those traits. Text
`Rel` is same-type default ordering and equality only.

## `konst` Policy

`konst = true` requests consistent const emission rather than a conservative
supported subset. This includes generated `map` and paths that invoke `valid`
callbacks. Text does not preflight const compatibility or silently downgrade
generated items; callers opt into the required nightly features and receive the
compiler diagnostics for unsupported combinations. `konst = false` emits the
ordinary non-const forms.

## Public Configuration

Use the same root-level callback syntax as `integral`, with text-specific
literal membership and friendship configuration:

```rust
#[typekin::text(
    valid = [is_non_empty, is_slug],
    in = ["draft", "published"],
    friends = [
        SlugSource(conv = SlugSource::into_string, cap = [Make]),
        InternedSlug(conv = InternedSlug::into_string, cap = [Make], trusted = true),
    ],
    with = [display],
)]
#[repr(transparent)]
pub struct Slug(String);
```

A single callback is written as `valid = callback`; an array means every
callback must pass. Friend conversion functions consume their input and return
`String`, for example `fn into_string(self) -> String`. A friend does not
construct the text type itself.

`trusted` defaults to `false` and is meaningful only for `Make`. The shared
friend configuration gains that boolean flag. `display` is opt-in through the
existing flag convention: `with = [display]`.

Text-specific generation flags should follow the existing `with` / `without`
convention. Proposed initial flags group read-only traits, conversions, checked
mutation, and optional panic-on-invalid operator compatibility.

## Core Construction and Access API

Generate the following first:

- `try_make(String) -> Result<Self, ()>`
- `try_from_str(&str) -> Result<Self, ()>`
- `into_inner(self) -> String`
- `into_bytes(self) -> Vec<u8>` and other consuming escape hatches that leave
  no text wrapper behind
- `as_str`, `as_bytes`, `len`, `is_empty`, and `capacity`
- `AsRef<str>`, `AsRef<[u8]>`, `Borrow<str>`, and `Deref<Target = str>`
- `Debug`, `Hash`, equality, and ordering traits
- `From<Text> for String`
- `Display` only when enabled with `with = [display]`

Only generate infallible construction conversions such as `From<String>` and
`From<&str>` when no validators are configured. Validated text must use fallible
construction.

## Checked Mutation: `map`

`map` is the foundational mutation API:

```rust
pub fn map(
    self,
    f: impl FnOnce(&mut String),
) -> Result<Self, ()>
```

Its generated logic is equivalent to:

```rust
let mut value = self.0;
f(&mut value);

if is_valid(&value) {
    Ok(Self(value))
} else {
    Err(())
}
```

Properties:

- `self` is consumed, so no clone or rollback allocation is required.
- The closure receives unrestricted `&mut String` access.
- Validation, including `in` membership, happens after the closure returns.
- An invalid result is dropped and reported as `Err(())`; no invalid text value
  can escape.
- A panic consumes and drops the original wrapper during unwinding; it cannot
  leave an externally accessible invalid text value.

Checked convenience mutators (`try_push`, `try_push_str`, `try_insert`,
`try_replace_range`, and similar APIs) should use the same
consume-mutate-validate model where their ownership signatures permit it.

## String Surface Policy

Catalog the stable `alloc::string::String` API at the workspace MSRV and place
each item in one of these groups:

1. **Direct read-only forwarding** — methods and traits that cannot change the
   value, including `str` APIs reached through `Deref<Target = str>`.
2. **Checked construction or transformation** — methods that yield a new
   string and must validate it before wrapping it.
3. **Consuming escape hatches** — methods such as `into_inner` that return the
   backing data and leave no validated wrapper alive.
4. **Unavailable invariant breakers** — mutable-reference escape hatches and
   guard-based mutation APIs such as `drain`, unless a transactional design can
   prove they cannot expose an invalid wrapper.

For String-shaped infallible operations such as `Add<&str>` and `AddAssign`,
the implementation may provide integral-style panic-on-invalid behavior only
when it can preserve the existing wrapper's validity transactionally. Checked
APIs remain the canonical interface.

## Implementation Phases

1. **Friendship ownership migration**
   - Change shared friendship conversion traits and conversion callbacks to take
     their source by value and return the relation by value.
   - Remove `&T` / `&mut T` friendship forwarding and migrate `friendship`,
     `integral`, and `bitflag` call sites, tests, and expanded examples.
   - Add `trusted = true` to shared friend configuration, preserving `false` as
     the default. Restrict its text behavior to `Make`.

2. **Text validation support**
   - Extract only the root `valid` callback-list parsing that text can share
     with integral; leave integral range parsing unchanged.
   - Add a text-only `in` parser for non-empty string-literal sets and reject
     duplicate literals during macro expansion.
   - Generate an AND-composed validation condition for callbacks and membership.

3. **Macro skeleton**
   - Add `crates/typekin/src/text.rs`.
   - Register the module and `#[proc_macro_attribute] text` in `src/lib.rs`.
   - Parse `#[repr(transparent)] struct Name(String);` and emit targeted
     diagnostics for unsupported shapes and attributes.

4. **Construction, friendship, and read-only behavior**
   - Implement validation, fallible construction, consuming conversions,
     immutable accessors, and read-only traits.
   - Implement `Make` friends by consuming a friend into `String`; validate and
     panic for untrusted output, or bypass validation only for trusted output.
   - Generate same-type default equality and ordering for `Rel`, and opt-in
     `Display` through `with = [display]`.

5. **Mutation and transforms**
   - Implement `map` first with `Result<Self, ()>`.
   - Add checked convenience mutation methods only where their error and
     ownership contracts preserve the invariant without hidden invalid state.
   - Add compatible `Add` / `AddAssign` behavior after the transactional rules
     are established.

6. **Test and documentation coverage**
   - Add parser tests in `src/tests/text.rs`.
   - Add end-to-end tests in `tests/text.rs` for construction, callback chains,
     literal membership, duplicate-literal rejection, `map`, failed-map errors,
     read-only trait behavior, Unicode boundaries, friendship trust behavior,
     and String operation coverage.
   - Cover `konst` token emission for `map` and validation paths; any generated
     combination unsupported by the caller's nightly configuration is diagnosed
     by Rust rather than filtered by the macro.
   - Update friendship, integral, and bitflag tests to prove consuming friend
     conversion and the absence of reference forwarding.
   - Add a text example, its `Cargo.toml` example entry, and README guidance on
     `alloc`, validation, `map`, friendship trust, and intentionally unavailable
     mutable APIs.

## Acceptance Criteria

- Valid values can be constructed and used through ordinary read-only
  `String` / `str` operations.
- `in` accepts only a non-empty, duplicate-free list of string literals, and
  membership composes with callback validation using logical AND.
- Every normal generated path that creates or transforms a text value validates
  before producing `Self`. A trusted `Make` friend is the documented exception
  and assumes the friend author already established validity.
- `map` returns `Ok(Self)` only for validated output and returns `Err(())` for
  invalid output after dropping its backing string.
- `konst = true` preserves const emission for `map` and validation paths; the
  macro does not silently downgrade unsupported caller configurations.
- Friendship conversion consumes its source, produces its relation by value,
  and never forwards capabilities through `&T` or `&mut T`.
- Text supports `Make` and same-type `Rel` only; it has no generated math or
  bitwise friendship capabilities.
- No generated safe API gives callers a mutable alias capable of leaving a live
  text wrapper invalid.
- The stable supported String surface is documented, with omissions explicitly
  justified by the invariant.
