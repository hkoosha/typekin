# AGENTS.md

## What this repository is

`typekin` is a Rust 2024 procedural-macro crate. It generates transparent
newtypes and the relationships between them:

- `integral`: integer newtypes, validation, conversions, math, and bitwise
  operations.
- `bitflag`: an enum plus a generated `{Enum}Value` type for combined or unknown
  bits.
- `text`: validated `String` newtypes, using `std` or `alloc` paths as
  configured.
- `friends`: capability protocols and optional generated constructors.
- `constructor`: augments a `friends` protocol with an explicit constructor.

The public contract is the code generated for downstream crates. Treat
compile-time diagnostics, generated trait implementations, ownership, layout,
constness, and runtime cost as API behavior.

## Repository map

- `src/lib.rs`: proc-macro entry points.
- `src/zz_runner.rs`: shared parsing, flag, diagnostic, and macro-runner
  helpers.
- `src/zz_value_type.rs`: primitive integral metadata used by generators.
- `src/{integral,bitflag,text,friendship,constructor}.rs`: parsing and emission
  for each macro family.
- `src/zz.rs`: shared validation grammar, list parsing, item-shape checks, and
  `repr` helpers.
- `src/tests/`: parser and token-generation unit tests.
- `tests/`: downstream-style behavioral tests against expanded macros.
- `examples/`: small source examples and checked-in `_exp.rs` expansions. Do not
  edit `_exp.rs` files directly: regenerate each one through its corresponding
  `justfile` target, or regenerate all of them with `just all`.
- `README.md`: practical public usage documentation.
- `NUTYPE.md`: a historical design investigation; it is not a backlog or API
  specification.
- `typekin_testing/`: separate assembly-comparison fixtures for integral
  operations.

## Development rules

- Keep public documentation practical, concise, and code-first.
- Preserve the optimized assembly comparison for changes that claim runtime
  erasure. `typekin_testing` includes both optimized comparisons and an
  unoptimized negative control; compilation or `#[inline]` alone is not that
  proof.
- Favor explicit capabilities, ownership, and constness. Reuse shared parsing
  and emission machinery rather than adding parallel syntax or generators.
- Do not create macros solely to reduce repetition.

## Design rules

### Preserve invariants on every path

- Generated constructors and transformations must not produce an invalid wrapper
  through their ordinary safe API.
- Apply all configured validators to every normal construction and
  transformation path, including friend construction and generated operations.
- `text` must not expose `DerefMut`, `AsMut<str>`, `&mut String`, or another
  alias that can leave a live wrapper invalid. Checked mutation consumes the
  wrapper, mutates its `String`, validates, and returns `Result`.
- For `integral` and `text`, `unsafe` paired with `Make` is the narrow
  exception: it bypasses only that friend's construction validation. It grants
  no other capability. Standalone `friends` has no generated validator to
  bypass.
- Unsupported attributes and combinations must produce a targeted macro
  diagnostic, not be ignored or silently downgraded.

### Make ownership and capability explicit

- Friend conversions consume the friend and return the relation by value. Do not
  restore blanket forwarding through `&T` or `&mut T`.
- Keep capabilities explicit. Integral recognizes `Make`, `Math`, `Bit`, `Rel`,
  and `unsafe`; `Math` also grants `Make` and `Bit`, while `Rel` enables
  comparisons with `Copy` friends. Text recognizes `Make`, `unsafe`, and
  self-only `Rel`. Bitflag's root friends are protocol markers; its nested
  integral configuration uses integral capabilities.
- Same-type integral arithmetic and bitwise operations are controlled by
  `self_math` and `self_bit`, not by declaring the wrapper as a friend.
- Prefer consuming transformations over cloning or rollback allocation.

### Keep wrappers zero-cost

- Preserve `#[repr(transparent)]` requirements and single-field shape checks.
- Generated wrappers must add no runtime state.
- Continue using `#[inline(always)]` on small generated forwarding and
  conversion paths where the existing generators do.
- Use `::core` in generated code where possible. Text chooses `::std` by default
  and `::alloc` with `std = false`; the latter must remain usable in a `no_std`
  consumer with `extern crate alloc`.
- Avoid hidden allocation, copying, and dynamic dispatch. Validation is
  intentionally performed for each checked construction and operator result.

### Constness is a caller-visible choice

- `integral`, `bitflag`, and `text` default to `konst = stable`; callers may
  select `none` or `nightly` explicitly. Integral and bitflag `stable` enables
  their stable validation and value operations; `nightly` enables const trait
  implementations and nightly-only integer operations. Text `stable` enables
  read-only accessors, while `nightly` enables supported owned operations.
- Bitflag forwards its own const configuration to the generated integral value
  type. A conflicting nested integral `konst_*` flag is an error at that flag.
- Individual `konst_*` flags override an integral or text master setting. Keep
  non-const stable use working independently of nightly const support.

## Implementation conventions

- Reuse the shared parsers and emitters before adding macro-specific copies.
  Keep type-specific grammar in its type module.
- Parse with `syn`, emit with `quote`, and attach errors to the most relevant
  input span through the existing `MkErr`/runner machinery.
- Reject duplicate, unknown, and removed configuration explicitly. Preserve
  accepted syntax unless making an intentional breaking change.
- When making a breaking change, do not add compatibility checks, warnings,
  failures, migration code, or migration diagnostics for compatibility with the
  prior behavior unless explicitly asked to do so.
- `friends` accepts one declaration directly or a bracketed list. Specifications
  use `conversion(SourceType) -> Capability` or a bracketed capability list.
  Integral can use `self(PrimitiveType)` for a direct primitive relation; there
  is no `_` friend syntax and no standalone marker declaration. Constructor
  friend selectors accept one type or a list of types. Do not restore the
  obsolete `conv`/`cap` argument grammar.
- Keep emitted names and ordering deterministic. Use the existing primitive
  metadata and naming helpers rather than duplicating type tables.
- Follow the established entry-point shape: parse configuration and item,
  validate the item, build through a maker, and return the original item plus
  generated code.
- Match local style. Explicit `return` is intentional
  (`needless_return = "allow"`). rustfmt uses an 80-column limit, vertical
  imports and parameters, and `ClosingNextLine` braces.
- Share genuinely common grammar and emission rules. Do not force integral
  ranges and text literal membership through the same parser.

## Tests and verification

Test behavior at the layer where it is observable:

- Parser grammar, rejected legacy syntax, diagnostics, and token emission:
  `src/tests/`.
- Construction, ownership, validation, traits, layout, boundaries, and generated
  operations as a consumer sees them: `tests/`.
- User-facing API changes: update a compact source example, its checked-in
  expansion when applicable, and `README.md`.
- Bugs should get a regression test that fails for the old behavior.
- Keep unrelated behavior in separate tests. A test's name and scope should
  identify the single behavior that can make it fail. This is not a
  single-assertion rule: use every assertion useful to verify the smallest
  helpful unit of behavior.
- Do not use table-driven tests. Every case must be an individually named
  `#[test]`, even when cases share setup or assertions. Helper functions are
  allowed; loops over a table of test cases are not.
- Boundary tests matter: signed/unsigned conversion limits, ranges, invalid
  operation results, duplicate configuration, ownership of non-`Copy` friends,
  and invariant-preserving text mutation.

Use these commands from the repository root:

```sh
cargo +nightly fmt -- --check
cargo +nightly test
cargo +stable run --example my_text
cd typekin_testing && just test
```

Run a relevant example as a smoke test for the changed macro. For stable
integral behavior, choose a `konst = none` example. Text and bitflag examples
using the default should compile on stable; `konst = nightly` requires the
corresponding feature gates.

## Documentation and change discipline

- Documentation should be practical, concise, and code-first. Do not write
  advertising copy.
- Explain catches and deliberate omissions, especially panic-on-invalid
  operations, trusted friends, const toolchain requirements, and unavailable
  mutable text APIs.
- Keep configuration examples explicit and compilable.
- Public generated-API changes require coordinated updates to implementation,
  parser tests, consumer tests, source examples, and README documentation.
- Prefer a clean migration over compatibility aliases or parallel legacy syntax.
  Remove obsolete parsing and generated paths once callers and tests have moved.
