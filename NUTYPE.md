I asked LLM to assess if I could drop and deprecate this package in favor of
nutype, or at least drop all validation machinery and reuse nutype for that
part:

# typekin 0.12.0 and nutype 0.8.0

Source of truth for typekin is `src`, not the README examples. Friend syntax in
`src` is `conversion(Type) -> [Capability, ...]`. The README still shows the
removed `conv` / `cap` form.

## Verdict

Do not deprecate typekin. Nutype already covers the plain validated newtype, and
covers it more completely. It does not cover friendship, revalidated arithmetic
and bitwise ops, bitflags, or consuming text mutation.

The overlap is wide enough that typekin should stop emitting its own check
expressions and use nutype for that step. It is not wide enough to drop the
crate. A type that only needs construction-time guarantees belongs in nutype.

## Products

typekin generates relations between transparent wrappers: who may construct a
value, which operations exist, and which results stay inside the invariant.
Validation is a `bool` reused on those paths.

nutype generates one newtype whose invariant is enforced at construction and at
serde deserialization. Named rules produce a dedicated error enum. Sanitizers
may rewrite the value before the check. Derives are allow-listed. The only
bypass is `unsafe` `new_unchecked`, behind a crate feature and a per-type flag.

## Surface

|                      | typekin                                                                                                                                                                                                                                            | nutype 0.8                                                                                                                                                        |
|----------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Entry                | `integral`, `bitflag`, `text`, `friendship`, `constructor`                                                                                                                                                                                         | `#[nutype]`                                                                                                                                                       |
| Inners               | 12 integers; `String`                                                                                                                                                                                                                              | `String`, those integers, `f32`/`f64`, `rust_decimal`, generics, other types                                                                                      |
| Validation           | `valid =` callbacks, all AND. Integral `in` is a `+` union of ranges (OR). Text `in` is an exact string-literal set (OR). The two groups AND.                                                                                                      | `validate(...)`: bounds, `finite`, `not_empty`, `len_char_*`, `len_utf16_*`, `regex`, one `predicate`, or `with` + custom `error`. All AND. One use of each kind. |
| Sanitization         | none                                                                                                                                                                                                                                               | `trim`, `lowercase`, `uppercase`, `with`, before validation                                                                                                       |
| Failure              | integral `Result<Self, Inner>`; text `Result<Self, ()>`                                                                                                                                                                                            | generated `TypeError`, or the caller’s error from `with`                                                                                                          |
| Where the check runs | `try_make`; integral `_unchecked` (panic), which math and bit ops call; untrusted `Make`; text `map` and the consuming mutators; `Add<&str>` (panic). Bitflag has no validator of its own: nested `integral = [...]` and `from_bits` → `try_make`. | `try_new` / infallible `new` when there is no validator; serde deserialize                                                                                        |
| Bypass               | safe `Trust` on a `Make` friend: `Self(raw)` for that construction only. Ops and other constructors still check.                                                                                                                                   | `unsafe { Type::new_unchecked(raw) }`, skips sanitizers and validators                                                                                            |
| Construction         | consuming friend conversion. Capabilities are explicit: integral `Make`, `Math`, `Bit`, `Relation`, `Trust`; text `Make`, `Trust` (`Rel` parses on `Self` and emits nothing); bitflag protocol traits are `Flag*`                                  | public `try_new` / `new`, or `constructor(visibility = ...)`                                                                                                      |
| Ops                  | arithmetic, bitwise, shifts, same-type ord. Result goes through `_unchecked`.                                                                                                                                                                      | none                                                                                                                                                              |
| Text mutation        | consuming `map` / `try_*`. No `DerefMut`, `AsMut`, or `&mut String`.                                                                                                                                                                               | no mutation API; field is private                                                                                                                                 |
| Const                | `konst` required on integral and bitflag. Integral validating `_unchecked` is always `const fn`, so integral callbacks are already const. Text const is opt-in per function.                                                                       | `const_fn` on `new` / `try_new`, stack values only. Not `String`.                                                                                                 |
| no_std               | generated code uses `::core` and `::alloc`                                                                                                                                                                                                         | `std` default; `default-features = false` exists. `regex` is a separate feature and needs the `regex` crate in the consuming crate                                |
| Other                | bitflag enum plus `{Enum}Value`, unknown bits, flag iteration                                                                                                                                                                                      | serde, schemars, arbitrary, valuable, guarded derives, `#[repr]` passthrough                                                                                      |

Standalone `friendship` does not generate a validator. `relationship` always
runs, `Trust` included.

## What nutype cannot already do

- Capability traits and consuming cross-type construction.
- A safe, friend-scoped skip of construction validation.
- Re-checking arithmetic, bitwise ops, or consuming string edits.
- Enum bitflags and a distinct value type.
- Const wrappers around those operations.
- typekin’s error shapes (`Err(raw)`, `Err(())`).
- OR of ranges or string literals, except by hand inside one `predicate`.

What typekin cannot already do: sanitizers, per-rule errors, floats,
decimals, generics, arbitrary inners, regex, length rules, serde that
revalidates, derive allow-listing.

## Using nutype for typekin validation

Both crates are `proc-macro = true`. `nutype_macros` is also `proc-macro = true`
and tells callers not to use it directly. typekin cannot call nutype’s parser
or emitter as a library. syn is 3 here and 2 in nutype; do not share AST types.

Do not implement this by emitting a private `#[nutype]` wrapper and mapping
`try_new`:

- nutype owns that struct, its derives, and its error enum.
- `try_new` is not `inline(always)`; `into_inner` is an extra move on `String`.
- `const_fn` does not cover `String`, while typekin text validation is const
  when `konst = true`.
- Trust would have to call `new_unchecked`. That is `unsafe`, feature-gated,
  and also skips sanitizers. typekin’s bypass is safe and check-only.
- A sanitizer would rewrite the value. Integral `Err(it)` must be the input
  the caller passed.

### nutype

Add one function-like macro. No new crate. No change to `#[nutype]` types,
errors, sanitizers, `const_fn`, or `new_unchecked`.

```rust
nutype::check!(int, $value:expr, $($rule:tt)*)
nutype::check!(str, $value:expr, $($rule:tt)*)
```

- `int` accepts the current integer validators except `with`. `str` accepts
  the current string validators except `with`. No sanitizers, no `error =`.
- Rules AND. `predicate` may repeat. Every other kind stays unique.
- Expansion is one parenthesized `bool` expression, `::core` paths only,
  pasted at the call site. No generated type.
- Integer bounds and `predicate` paths must be valid inside a `const fn`.
- `regex` stays behind the `regex` feature and still requires the consuming
  crate to depend on `regex`.
- Integer and `str` rules must expand with `default-features = false`.
- Float, decimal, and `with` are out of this macro. typekin has no such
  values, and `with` exists to build nutype’s `Result` error.

`#[nutype(validate(...))]` keeps a single `predicate`. Repeated predicates are
a `check!` difference only.

Implement it by reusing the integer and string validator parsers and the
boolean half of the existing `try_new` emitter. `nutype` forwards to
`nutype_macros` the same way `nutype` already forwards.

Rejected alternative: move the emitter into a normal library crate so typekin
can call it during expansion. That avoids a downstream `nutype` dependency,
and it is a larger split than this macro.

### typekin

Consuming crates that generate a validated type must depend on `nutype`
directly. A proc-macro dependency of typekin is not visible to them. Use
`default-features = false`. Enable `regex` only in crates that pass a regex
rule. Unvalidated types must not mention `::nutype` in their expansion.

Replace the bodies of integral `validation_condition` and text
`validation_condition`. Leave every signature and call site. Ops keep calling
`_unchecked`. Text mutators keep calling `try_make`. Trust keeps building
`Self(...)` and does not call `check!`.

Lowering, integral:

- `valid = [a, b]` → one `check!(int, it, predicate = a, predicate = b)`.
- One range → the matching bounds inside that same `check!`:
  `start..=end` is `greater_or_equal` + `less_or_equal`;
  `start..end` is `greater_or_equal` + `less`; open ends drop a bound;
  `..` emits no bounds.
- `in = r1 + r2` stays OR outside the macro:
  `(check!(int, it, <r1>)) || (check!(int, it, <r2>))`,
  then AND the predicate check. nutype has no OR group. Do not add one.
- Endpoints stay expressions, as they are now.

Lowering, text:

- Callbacks → `check!(str, value.as_str(), predicate = ...)`.
- `in = ["draft", "published"]` stays a typekin `==` disjunction, AND-ed with
  the `check!` result. Do not add a string-set validator to nutype.
- Empty `in` still fails closed (`false`).

Add optional `rules = (...)` on integral and text, forwarded as more terms in
the same `check!` and AND-ed with `valid` / `in`. This is the only new syntax,
and it is how `len_char_*`, `len_utf16_*`, `not_empty`, and `regex` show up.
Reject sanitizers, `with`, `error`, float rules, and unknown keys with a
span error.

Const:

- Integral `_unchecked` is already an unconditional `const fn` whenever
  validation exists. Integral `rules` may contain only const integer rules.
  That is the whole integer subset of `check!`.
- Text with `konst = false` may use `len_char_*`, `len_utf16_*`, `not_empty`,
  and `regex`.
- Text with `konst = true` rejects those four at expansion time. User
  `predicate` paths remain, and fail later if the function is not const, as
  today.

Public behavior that does not move:

- `Result<Self, Inner>` and `Result<Self, ()>`.
- Panic strings on `_unchecked`, untrusted text friends, and `Add<&str>`.
- No sanitizer phase and no rewrite of the rejected value.
- Friendship, bitflag, and operator generation.
- `no_std`. `regex` is the exception: that rule pulls `std` via the `regex`
  crate, matching nutype.

Tests: `tests/` construction and op behavior should not change for existing
`valid` / `in` inputs. Add parser coverage for `rules`, for repeated
predicates, and for a range union lowered as OR of two `check!` invocations.
Re-run the existing optimized assembly comparison. The check is still an
inlined expression on the op result, not a call to `try_new`.
