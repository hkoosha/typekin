# AGENTS.md

## What this repository is

`typekin` is a single Rust 2024 procedural-macro crate. It generates
transparent newtypes and the relationships between them:

- `integral`: integer newtypes, validation, conversions, math, and bitwise
  operations.
- `bitflag`: an enum plus a generated `{Enum}Value` type for combined or
  unknown bits.
- `text`: validated `alloc::string::String` newtypes.
- `friendship` and `constructor`: explicit construction capabilities between
  types.

The public contract is the code generated for downstream crates. Treat
compile-time diagnostics, generated trait implementations, ownership, layout,
constness, and runtime cost as API behavior.

## Repository map

- `src/lib.rs`: proc-macro entry points.
- `src/runner.rs`: shared parsing, flag, diagnostic, and macro-runner helpers.
- `src/value_type.rs`: primitive integral metadata used by generators.
- `src/{integral,bitflag,text,friendship}.rs`: parsing and emission for each
  macro family.
- `src/tests/`: parser and token-generation unit tests.
- `tests/`: downstream-style behavioral tests against expanded macros.
- `examples/`: small source examples and checked-in `_exp.rs` expansions.
- `README.md`: practical public usage documentation.
- `PLAN.md`: text design rationale and maintenance backlog; verify design
  claims against current code and tests.
- `typekin_testing/`: assembly-comparison fixtures retained from the former
  workspace.

## Development rules

- **Directly recorded:** practical usage documentation, no advertising or fluff,
  and code examples rather than long explanations. These requests recur in
  historical `PROMPTS.md` (available at commit `5cb75a7`).
- **Directly recorded:** prove runtime erasure by comparing fully optimized
  nightly assembly for raw and wrapped math/bit operations on already-built
  values. Retain an unoptimized negative control; successful compilation or an
  inline annotation alone does not prove the performance claim.
- **[INFERENCE] From the implementation and migrations:** favor explicit
  capabilities, ownership, and constness; share actual common machinery and
  remove superseded paths instead of accumulating parallel implementations.

## Design rules

### Preserve invariants on every path

- A generated wrapper must not be constructible in an invalid state through
  its ordinary safe API.
- Apply all configured validators to every normal construction and
  transformation path, including friend construction and generated
  operations.
- `text` must not expose `DerefMut`, `AsMut<str>`, `&mut String`, or another
  alias that can leave a live wrapper invalid. Checked mutation consumes the
  wrapper, mutates its `String`, validates, and returns `Result`.
- A friend granted the `Trust` capability is the narrow documented exception
  across all type families. It grants no other capabilities; do not broaden
  its construction-only validation bypass.
- Unsupported attributes and combinations must produce a targeted macro
  diagnostic, not be ignored or silently downgraded.

### Make ownership and capability explicit

- Friend conversions consume the friend and return the relation by value.
  Do not restore blanket forwarding through `&T` or `&mut T`.
- Keep capabilities explicit (`Make`, `Math`, `Bit`, and the supported
  relation capability). Do not grant operations merely because a conversion
  exists.
- Equality and ordering for integral and text wrappers are same-type
  operations unless the current public contract explicitly says otherwise.
- Prefer consuming transformations over cloning or rollback allocation.

### Keep wrappers zero-cost

- Preserve `#[repr(transparent)]` requirements and single-field shape checks.
- Generated wrappers must add no runtime state.
- Continue using `#[inline(always)]` on small generated forwarding and
  conversion paths where the existing generators do.
- Use `::core` in generated code where possible and `::alloc` for owned text.
  Do not impose a downstream `std` dependency.
- Avoid hidden allocation, copying, dynamic dispatch, or repeated validation.
  Performance-sensitive changes should retain equivalent optimized assembly
  for already-valid wrapped and raw operations.

### Constness is a caller-visible choice

- `integral` and `bitflag` require explicit `konst = true|false`.
- `text` requires explicit `konst = true|false`, matching the other primary
  macro families.
- `konst = true` means consistent const emission. Do not silently emit a
  non-const subset when a requested combination is unsupported; let the
  caller's nightly compiler diagnose it.
- Keep stable, non-const use working independently of nightly const support.

## Implementation conventions

- Reuse the shared parsers and emitters before adding macro-specific copies.
  Keep type-specific grammar in its type module.
- Parse with `syn`, emit with `quote`, and attach errors to the most relevant
  input span through the existing `MkErr`/runner machinery.
- Reject duplicate, unknown, and removed configuration explicitly. Preserve
  accepted syntax unless making an intentional breaking change.
- `friends` accepts one declaration directly or a bracketed list. Specifications
  use `conversion(SourceType) -> Capability` or a bracketed capability list.
  `_(Type) -> Capabilities` preserves implicit conversion behavior; standalone
  `_ -> Capabilities` declares markers with no implementing type or dummy
  callback. Constructor friend selectors accept one type or a list of types.
  Do not restore the obsolete `conv`/`cap` argument grammar.
- Keep emitted names and ordering deterministic. Use the existing primitive
  metadata and naming helpers rather than duplicating type tables.
- Follow the established entry-point shape: parse configuration and item,
  validate the item, build through a maker, and return the original item plus
  generated code.
- Match local style. Explicit `return` is intentional
  (`needless_return = "allow"`). Rustfmt uses an 80-column limit, vertical
  imports and parameters, and `ClosingNextLine` braces.
- Share genuinely common grammar and emission rules. Do not force integral
  ranges and text literal membership through the same parser.

## Tests and verification

Test behavior at the layer where it is observable:

- Parser grammar, rejected legacy syntax, diagnostics, and token emission:
  `src/tests/`.
- Construction, ownership, validation, traits, layout, boundaries, and
  generated operations as a consumer sees them: `tests/`.
- User-facing API changes: update a compact source example and `README.md`.
- Bugs should get a regression test that fails for the old behavior.
- Boundary tests matter: signed/unsigned conversion limits, ranges, invalid
  operation results, duplicate configuration, ownership of non-`Copy`
  friends, and invariant-preserving text mutation.

Use these commands from the repository root:

```sh
cargo +nightly fmt -- --check
cargo +nightly test
cargo +stable run --example my_text
```

Run a relevant example as a smoke test for the changed macro. For stable
behavior, choose a `konst = false` source example; const examples require
nightly features.

Known tooling limitations and implementation gaps are tracked in the
[maintenance backlog](PLAN.md#maintenance-backlog). They are not supported
workflows or conventions to copy; do not repair them incidentally during an
unrelated task.

## Documentation and change discipline

- Documentation should be practical, concise, and code-first. Do not write
  advertising copy.
- Explain catches and deliberate omissions, especially panic-on-invalid
  operations, trusted friends, const toolchain requirements, and unavailable
  mutable text APIs.
- Keep configuration examples explicit and compilable.
- Public generated-API changes require coordinated updates to implementation,
  parser tests, consumer tests, source examples, and README documentation.
- Prefer a clean migration over compatibility aliases or parallel legacy
  syntax. Remove obsolete parsing and generated paths once callers and tests
  have moved.