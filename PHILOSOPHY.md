# Typekin Philosophy

This document records the design principles behind Typekin. It explains which
features belong in the crate, what its APIs promise, and why some plausible
features are intentionally absent.

## Mission

Typekin defines transparent newtypes and explicit relationships between them.
Its purpose is not merely to reduce newtype boilerplate. Its distinguishing
purpose is to control:

- which owned values may construct a type;
- which operations are available between related types;
- where validation occurs;
- whether a conversion is an infallible contract or an expected fallible
  operation;
- how values move through a graph of validated types.

Once a value enters the validated graph through an ordinary safe path, generated
construction and transformation APIs must preserve its invariant.

Features are appropriate when they strengthen this model or make it practical to
use. Broad support for unrelated inner types, sanitization, regexes, or
arbitrary derives is not automatically aligned with the mission. A type that
only needs construction-time validation may be better served by a general
newtype crate.

## Friendship Is an Infallible Contract

Friendship declares permission and an infallible construction capability. It is
not an alternative spelling of Rust's standard conversion traits and is not
Typekin's fallible conversion framework.

### Friendship Converts Indirectly Through a Relation

`From<Source> for Target` describes one direct conversion:

```text
Source -> Target
```

The implementation knows both endpoints and owns the entire conversion. Rust
then supplies `Into<Target> for Source`. There may be separate `From`
implementations for different source types, but every implementation is still a
direct source-to-target edge.

Friendship separates conversion from target construction:

```text
Source -> Target's relation -> Target
```

For example, a target whose relation is `u32` defines how a `u32` becomes the
target. Each friend separately defines how it becomes that target's `u32`
relation. The target does not need a direct `From<FriendA>` or `From<FriendB>`
implementation, and neither friend needs a direct `Into<Target>` implementation.

This separation is useful even though Rust permits both
`From<FriendA> for Target` and `From<FriendB> for Target`. Friendship
centralizes the relation-to-target step and keeps each friend's responsibility
limited to producing the shared relation. Construction, validation, and
generated operations can then reuse that relation without repeating direct
conversions for every source-target pair.

The protocol is scoped to the target rather than globally to the relation. The
same source type can therefore convert to `u32` differently for two targets:

```text
Source -> TargetA's u32 relation -> TargetA
Source -> TargetB's u32 relation -> TargetB
```

Those two source-to-`u32` conversions may use different functions. A global
`Into<u32> for Source` cannot express that distinction because Rust permits only
one implementation for the `Source` and `u32` pair. Friendship needs no global
`Into<u32>`, `Into<TargetA>`, or `Into<TargetB>` implementation. The
target-specific protocol supplies the context that standard conversion traits do
not carry.

Friendship also governs more than construction. Capabilities such as `Make`,
`Math`, `Bit`, and `Rel` state how a friend may participate. A conversion may
also be specific to one capability, representing the same source differently for
different operations. `From` grants only a direct construction conversion and
does not model these permissions.

### Friendship Compared With `From` and `TryFrom`

`From` and friendship are both infallible at the type-signature level, but their
models differ:

- `From` is a direct, globally coherent source-to-target conversion.
- Friendship is a target-scoped, capability-gated conversion through a shared
  relation.
- `From` requires a direct implementation for each source-target pair.
- Friendship lets the target own relation-to-target construction while each
  friend only supplies its target-specific relation.

`TryFrom` is also a direct source-to-target conversion, but rejection is an
ordinary result represented by its associated `Error` type. It is the correct
choice when callers are expected to handle invalid input.

Friendship has no fallible result. A friend conversion consumes its source.
Construction through an untrusted friend validates the converted relation. If
validation rejects it, the operation panics because the declared infallible
contract was violated. A trusted friend may bypass that construction check and
carries the stronger obligation to produce a valid relation.

Expected rejection therefore does not belong in friendship. The caller should
implement `TryFrom` and use `TryInto`; Typekin does not generate a parallel
`try_of` friendship API. A friendship panic signals a broken programming
contract, not a routine parse or validation outcome.

## Validation Belongs on Every Generated Ingress

Validation is not limited to a named constructor. Every generated safe path that
can create a wrapper must either:

- run all configured validators;
- be statically incapable of creating an invalid value; or
- be an explicitly trusted friendship edge.

This includes arithmetic and bitwise results, text mutation, standard
conversions, parsing, and deserialization.

Validation after every integral operator is deliberate. It ensures every
intermediate wrapper is valid, but it also means an expression can panic on an
invalid intermediate result even when its mathematical final result would have
been valid.

For intentional final-state-only validation, integral wrappers provide a
separate mapping operation. Its fallible form returns the rejected final raw
value; its infallible form panics. This does not weaken ordinary operator
semantics.

## Panic and Fallibility Are Different Contracts

Generated operator traits and explicitly infallible friendship operations may
panic when their result violates the target invariant. Trait signatures leave no
place for a validation error, and invalid output represents a contract failure.

APIs designed for expected rejection return `Result` or `Option`. The choice is
made at the API boundary rather than hidden behind a global mode.

Typekin does not attempt to make every operation fallible. Parallel fallible and
infallible APIs are added only when they express materially different invariant
semantics, such as final-state integral mapping.

## Ownership Is Part of the Capability Model

Friend conversions consume their source. Typekin does not automatically forward
friendship through references, clone values behind the caller's back, or retain
a rollback copy.

Text mutation also consumes the wrapper. Mutation occurs in the owned `String`,
then the final value is validated. No mutable reference to the backing `String`
may escape while it is wrapped.

Returning an invalid mutated `String` could be implemented without another heap
allocation by moving its existing buffer. It is intentionally not part of the
current API because an error carrying a `String` is ambiguous: callers may
reasonably expect either the original valid text or the mutated invalid
candidate. Recovering the original after arbitrary mutation generally requires
cloning, allocation, or an operation-specific rollback log.

The current text mutation API therefore returns `Err(())` and consumes rejected
state. If recovery is added in the future, it should use an explicitly named API
and a concrete rejected-value wrapper that makes the semantics unmistakable.

## Avoid Hidden Allocation

Generated operations should not allocate unless allocation is inherent in the
underlying operation requested by the caller.

- Moving an existing `String` is not an allocation.
- Formatting should write directly to a formatter.
- Parsing should inspect borrowed slices.
- Checked transformations should consume and reuse owned storage.
- Preserving both an original and a mutated owned string generally requires
  copying and is not free.

Allocation behavior is API behavior and should be covered by tests where it is
important.

## Error Types Stay Concrete

Typekin does not currently introduce generic validation-error types into every
generated API. Doing so would spread extra type parameters and bounds throughout
construction, friendship, operations, and const code.

The configuration accepts only `error = ()` for now. Any other error type is a
targeted macro error. This reserves a clean place in the grammar without
prematurely committing to a generic public design.

A possible future direction is a concrete caller-supplied error type named in
the macro configuration. That design would need to work coherently with ranges,
literal membership, multiple callbacks, ownership of rejected input, and const
validation before it is accepted.

## Serde Is an Explicit Call-Site Integration

Serde support is requested per type with `with = [serde]`.

There is no Typekin Cargo feature for Serde and no hidden transitive Serde
dependency. The consuming crate is responsible for:

- adding and configuring its own `serde` dependency;
- placing appropriate `cfg` attributes around macro use;
- avoiding conflicting manual or derived implementations.

This keeps dependency policy in the consuming crate and makes generated API
changes explicit at the annotated type.

Deserialization is a construction boundary and must apply complete validation.
Serialization exposes the wrapper's existing relation and does not mutate it.
Serde support must remain compatible with `no_std` when the caller selects
compatible Serde and allocator settings.

## Bitflags Follow an Established Text Model

Typekin's bitflag text behavior follows the minimum retain-mode grammar of the
`bitflags` crate instead of inventing another format:

- known names are separated by ` | `;
- unknown bits are represented as hexadecimal;
- parsing accepts names and hexadecimal entries;
- unknown bits are retained;
- formatting and parsing round-trip all bits;
- formatting and parsing do not allocate.

Typekin preserves its own enum-plus-value model, validation, friendship, and
const controls. Compatibility with the established text grammar does not mean
delegating its type model to another crate.

Serde bitflag representation is explicit:

- the default follows `bitflags`: text for human-readable serializers and raw
  bits for compact serializers;
- `serde_with_auto` explicitly selects the default;
- `serde_with_name` forces the textual form;
- `serde_with_value` forces the raw integral form;
- the three representation flags are mutually exclusive;
- the selected form applies to both serialization and deserialization.

Unknown bits are retained when the value type permits them. Deserialization
still applies nested integral validation.

## Opt-In Features Must Be Honest

An opt-in is not permission to silently weaken an invariant. It is permission to
select a clearly documented contract and accept its consequences.

Examples include:

- `with = [serde]` accepts a direct dependency on caller-visible Serde paths;
- trusted friendship accepts responsibility for proving converted values valid;
- operation flags select which generated behavior becomes part of the public
  API.

Unsupported combinations should fail during macro expansion with a diagnostic at
the relevant configuration span. They must not be ignored or silently
downgraded.

## Zero-Cost and `no_std` Are Core Constraints

Generated wrappers remain transparent and add no runtime state. Small forwarding
and conversion paths should inline, avoid dynamic dispatch, and use `::core`
where possible.

Text uses `::std` or `::alloc` according to explicit configuration. Optional
integrations must not force `std` into an otherwise `no_std` consumer.

Claims of zero runtime overhead require optimized assembly comparison where an
equivalent primitive implementation exists. Compilation and `#[inline]` alone
are not proof.

## Constness Is Public API

Const generation is selected explicitly. Stable and nightly capabilities must
not be conflated, and enabling an unrelated feature must not silently require a
new nightly gate.

Operations involving arbitrary closures, formatters, serializers, or
deserializers are non-const unless Rust provides the necessary language and
trait support. Typekin should not promise constness that downstream code cannot
actually use.

## Scope Discipline

New features should be evaluated against three questions:

1. Does this strengthen controlled construction, explicit relationships, or
   invariant-preserving transformation?
2. Can it preserve ownership, layout, allocation, constness, and `no_std`
   guarantees honestly?
3. Does it belong in Typekin rather than a general validation or newtype crate?

Features that fail these tests should not be added merely because they are
convenient elsewhere. Typekin should deepen its relationship-and-invariant model
rather than become a collection of unrelated derives.
