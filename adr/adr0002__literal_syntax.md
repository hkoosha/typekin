# ADR 0002: Literal syntax: per-type macros vs a `lit::<RAW>()` method

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/zz_lit.rs`, `examples/my_lit.rs`, `AGENTS.md` (literal macro
  rule)

## Context

A validated wrapper needs a way to write a constant whose validity is checked at
compile time: `Port` with `domain = 1..=1023 | 49_152..=65_535` should accept
`443` and reject `0` in the build, not at runtime. `try_make` is already
`const`, so the check itself is const evaluation of `try_make(v)` and a panic on
`Err`; the question is the spelling and the expansion shape.

Options:

- **A. A generated method**: `Port::lit::<443>()`, a
  `const fn lit<const RAW: u16>() -> Self` whose body evaluates `try_make(RAW)`
  in an inline `const` block.
- **B. A typekin-exported macro**: `typekin::lit!(Port, 443)`.
- **C. A generated per-type macro**: `port!(443)`, named after the snake-cased
  type, expanding to a block-level `const` item.

## Decision

Option C. Every validated integral, float, and bitflag value type with a const
`try_make` gets a literal macro named after the snake-cased type (`lit_name`
overrides it; `without = lit_macro` removes it):

```rust
#[typekin::integral(domain = 1..=1023 | 49_152..=65_535)]
pub struct Port(u16);

const HTTPS: Port = port!(443); // checked by `cargo check`
let ssh = port!(22);            // also checked by `cargo check`

fn first<D: Direction>() -> Packet<D> {
    return packet!(const 1);    // inline `const` form for generic contexts
}
```

- `name!(v)` expands to a block-level `const` item; `name!(const v)` uses an
  inline `const` block where the type mentions generic parameters of the
  enclosing function.
- A literal macro is the only literal syntax. Typekin generates no
  `lit::<RAW>()` method.

## Evidence

Probes from 2026-10-31 used a `macro_rules!` stand-in with the planned expansion
against real typekin types:

| Probe                                | Inline `const { }`     | Block-item `const VALUE: T` |
|--------------------------------------|------------------------|-----------------------------|
| integral, valid                      | ok                     | ok                          |
| float `domain = 0.0..=1.0`, valid    | ok                     | ok                          |
| bitflag value, valid                 | ok                     | ok                          |
| generic `Tagged<D>` in `fn f<D>`     | ok                     | E0401: outer generic param  |
| invalid, `const` item, `cargo check` | E0080                  | E0080                       |
| invalid, fn body, `cargo check`      | **not reported**       | E0080                       |
| invalid, fn body, `cargo build`      | E0080                  | E0080                       |
| invalid, generic fn, `cargo build`   | E0080 at instantiation | n/a                         |

A prototype `Port::lit::<443>()` (`const fn lit<const RAW: u16>()` with an
inline `const` block) worked for integers.

Why option A was rejected:

- **Floats cannot use it.** `f32`/`f64` are not allowed as const-generic
  parameters, so `float` types would need a second spelling. A macro takes any
  expression and gives one spelling for integral, float, and bitflag values.
- **The error points into generated code.** A rejected `RAW` fails inside
  typekin's `lit` body; the macro reports E0080 at the call site ("evaluation
  panicked: 0 is outside the domain of Port").
- **`cargo check` misses it in function bodies.** Option A needs an inline
  `const` block (a const generic cannot reach a block-level item), and
  `cargo check`, editor flycheck, and the testing crate's compile harness do not
  evaluate inline `const` blocks in function bodies. Only `cargo build` reports
  the bad literal.

Why option B was rejected: typekin is a proc-macro crate and only generates
code. It cannot export a `macro_rules!` for consumers, and a function-like proc
macro has no type information, so it could only forward to `try_make` the same
way a generated macro does.

Also rejected: making `_or_panic` public. It is still a runtime panic, and it
conflicts with the naming and `hide_in` rules.

## Consequences

- Invalid literals fail `cargo check` at the call site, also inside functions.
- The block-level form cannot name generic parameters of the enclosing function
  (E0401); `name!(const v)` covers that case, at the price of being checked only
  by `cargo build` and at instantiation.
- Only types where a literal adds something get one. By default the macro exists
  when `try_make` is const and the type validates: unvalidated wrappers already
  have an infallible const `make`, bitflag enum variants are already constants
  (only the value type gets a macro), and `text` gets none because owned
  `String` construction is not const on stable. With `konst = none` (or
  `without = konst_try_make`) the macro is silently omitted; an explicit
  `with = lit_macro` then fails at expansion ("`lit_macro` needs a const
  `try_make`; …"), as do `lit_name` and `lit_path` without the macro.
- Each type gets a macro name in the caller's namespace. Typekin rejects names
  that are keywords or shadow prelude macros, because shadowing `line!` or
  `write!` would silently change unrelated code in the module; `lit_name`
  renames.
- The expansion names the type as written, resolved at the call site, unless
  `lit_path` gives a crate-relative path. A hidden guard method
  (`__typekin_lit_<hash>(&self)`) called on the built value rejects a different
  type of the same name. It is a method rather than the planned constant so it
  also works where generic arguments are inferred.
- Below `pub` the macro is a `macro_rules!` made path-addressable by a `use`;
  `pub` types export it with `#[macro_export]` under a mangled name and
  re-export it beside the type, because rustc rejects absolute paths to
  macro-expanded exports. The mangling hash is FNV-1a over the type name and its
  source location, so same-named `pub` types in different modules don't collide
  at the crate root.
- Without a macro, `try_make` in a `const` item stays available:

  ```rust
  const HTTPS: Port = match Port::try_make(443) {
      Ok(it) => it,
      Err(_) => panic!("443 is outside Port's domain"),
  };
  ```

- Revisit option A only as an integer-only addition beside the macro, accepting
  that its errors point into generated code and that `cargo check` misses them
  in function bodies.
