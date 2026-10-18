# Typekin Feature Plan

This plan implements the decisions recorded in `PHILOSOPHY.md`. It is an
implementation checklist, not a compatibility promise. Typekin is pre-1.0, so
clean API changes are preferred over compatibility aliases.

## Scope

The work contains four related additions:

1. final-state integral mapping;
2. opt-in Serde implementations that preserve validation;
3. a reserved `error = ()` configuration;
4. bitflag text round-tripping and selectable Serde representations.

It deliberately does not add standard friendship conversion implementations,
fallible friendship constructors, recoverable text mutation errors, generic
validation errors, or a Typekin Cargo feature for Serde.

## 1. Configuration

Extend the existing parsers and flag machinery before emitting new code.

### Common rules

- Recognize `serde` in `with` and `without` where the corresponding macro can
  emit those implementations.
- Keep the feature disabled by default. In particular, no existing type gains a
  dependency on `serde` merely by upgrading.
- Reject duplicates and contradictory `with`/`without` declarations through the
  existing flag diagnostics.
- Emit `::core` paths except for the caller-provided `::serde` path.

### `error`

- Accept `error = ()` on `integral` and `text`, including an `integral`
  configuration nested inside `bitflag`.
- Reject every non-unit type at the type's span with a targeted diagnostic.
- Reject duplicate `error` declarations.
- Treat this as reserved syntax for a future concrete validation-error design.
  It does not change today's `try_make` error shapes: integral still returns the
  rejected raw integer and text still returns `()`.
- Do not accept `error` on standalone `friends` or `constructor`, which do not
  own a validator.

### Bitflag representation flags

Add `serde_with_auto`, `serde_with_name`, and `serde_with_value` as bitflag-only
flags.

- They are mutually exclusive.
- Each representation flag enables Serde generation. Typekin does not validate
  the caller's dependencies; rustc reports a missing `serde` crate.
- `serde_with_auto`, serde follows the `bitflags` crate's normal behavior: text
  for human-readable serializers and raw bits for compact serializers.
- `serde_with_name` forces the textual representation for every serializer and
  deserializer.
- `serde_with_value` forces the raw integral representation for every serializer
  and deserializer.
- Default to `serde_with_auto`.

Add parser tests for every accepted form, unknown flag, duplicate, dependency,
and mutual-exclusion error.

## 2. Final-State Integral Mapping

Generate two consuming methods on integral wrappers:

```rust
pub fn try_map_raw<F>(self, function: F) -> Result<Self, Raw>
where
    F: FnOnce(Raw) -> Raw;

pub fn map_raw<F>(self, function: F) -> Self
where
    F: FnOnce(Raw) -> Raw;
```

Behavior:

- Extract the relation through the configured raw accessor.
- Invoke the closure exactly once.
- Validate only the closure's final result.
- `try_map_raw` returns the rejected final raw value.
- `map_raw` uses the same path and panics with a stable, operation-specific
  message when validation rejects the final value.
- Closure panics and primitive overflow retain ordinary Rust behavior.
- Neither method is const by default because stable Rust cannot call an
  arbitrary closure in const code.
- Add `with_const_map_fn` to mark both functions const.
- Both methods remain zero-allocation and inline.

Expose one normal `map_raw` generation flag, enabled by default, that controls
both methods. Test validated and unvalidated wrappers, final-only validation,
single closure invocation, rejection, panic behavior, and configured raw
accessors.

## 3. Opt-In Serde

`with = [serde]` directly emits implementations referring to `::serde`.

- Do not add a Typekin Cargo feature.
- Do not add `serde` as a normal Typekin dependency.
- The consuming crate must add and configure `serde` itself.
- The consuming crate is responsible for placing matching `cfg` attributes
  around macro use when Serde is conditional.
- A caller must not also derive the same Serde traits on the wrapper.

### Integral

- Serialize the exposed raw integer.
- Deserialize the primitive integer and pass it through the wrapper's complete
  validation condition.
- Convert rejection into `serde::de::Error::custom` without requiring the
  wrapper or a custom error to implement additional traits.
- Preserve `no_std` compatibility when the caller configures Serde accordingly.

### Text

- Serialize as a string.
- Deserialize into the selected `std` or `alloc` `String`.
- Pass the owned value through complete text validation before constructing the
  wrapper.
- Do not expose mutable access or bypass validation.

### Standalone friendship

Do not infer a serialization format from an arbitrary relation. Standalone
`friends` therefore does not accept `serde`; callers can implement Serde using
their chosen relation and constructor.

### Bitflag

The generated value type uses the representation selected in section 1:

- auto (default): textual flags for human-readable serializers and raw bits for
  compact serializers;
- name: textual flags for every serializer;
- value: raw bits for every serializer.

Raw-bit deserialization retains unknown bits but still applies any nested
integral validation. Text deserialization parses the retain-mode grammar from
section 4 and then applies the same validation.

The flag enum uses its exact variant name in textual mode and its repr value in
raw mode. Raw values that do not equal a declared variant are rejected when
deserializing the enum.

Use development-only Serde test dependencies. Test readable and compact
serializers, all three representation modes, validation failures, unknown bits,
text allocation mode, and absence of generated Serde code when the flag is not
enabled.

## 4. Bitflag Text Format

Match the minimum retain-mode text behavior of the `bitflags` crate rather than
inventing a Typekin-specific grammar.

Grammar and formatting:

- Separate entries with ` | ` when formatting.
- Parse optional whitespace around `|`.
- Accept exact declared names.
- Accept unknown or remaining bits as `0x` hexadecimal values.
- Retain unknown bits.
- Format known contained flags by name and append remaining bits in hexadecimal.
- Format an empty value as the empty string.
- Reject unknown names, malformed hexadecimal values, empty interior entries,
  overflow, and a final value rejected by nested integral validation.
- Preserve deterministic declaration order and the existing iteration semantics
  for overlapping or composite flags.

Implement allocation-free formatting directly through `fmt::Formatter` and
allocation-free parsing over borrowed string slices.

Integrate with the existing API carefully:

- `Display` for the enum continues to emit its exact name.
- `FromStr` for the enum accepts one exact declared name.
- `Display` for the generated value emits the retain-mode flag text.
- `FromStr` for the generated value parses retain-mode flag text.
- Replace, rather than conflict with, the generated integral value type's
  numeric `Display` or `FromStr` implementation when bitflag text behavior is
  enabled.
- Continue honoring the existing `display` and nested `from_str` generation
  controls.

Test empty, one-name, multiple-name, composite, unknown-only, mixed known and
unknown, whitespace, malformed, overflow, and validated-value cases. Verify that
formatting followed by parsing preserves every bit.

## 5. Documentation and Examples

- Update `README.md` with concise examples for `map_raw`, `try_map_raw`,
  `with = [serde]`, bitflag text, and each Serde representation mode.
- State prominently that friendship is an infallible contract.
- State that expected conversion failure belongs in caller-written
  `TryFrom`/`TryInto`.
- State that Typekin provides no Serde Cargo feature or transitive Serde path.
- Document `error = ()` as reserved and intentionally limited.
- Update a compact source example and regenerate, rather than manually editing,
  each affected `_exp.rs` file.

## 6. Verification

Run the complete required checks:

```sh
cargo +nightly fmt -- --check
cargo +nightly test
cargo +stable run --example my_text
cd typekin_testing && just test
```

Also run:

- relevant stable integral and bitflag examples;
- `no_std` consumer tests without Serde;
- a `no_std` Serde consumer configured by the fixture itself;
- Markdown linting for `README.md`, `PLAN.md`, and `PHILOSOPHY.md`;
- optimized assembly comparisons after integral mapping.

No runtime-erasure claim is complete until the optimized assembly fixtures
remain equivalent.
