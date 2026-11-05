# ADR 0025: Serde integration

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: README §Serde, PHILOSOPHY.md "Serde Is an Explicit Call-Site
  Integration", `src/integral_impls.rs`, `src/text.rs`, `src/bitflag.rs`
  (`ekran_serde`, `SerdeRepr`), PLAN.md D10, ADR 0009, 0016

## Context

Wrappers need `Serialize`/`Deserialize`, but Typekin wants no runtime
dependencies and every ingress must keep the invariant.

## Decision

- Per-type opt-in: `with = serde` on `integral`, `float`, `text`, `bitflag`.
- No Typekin Cargo feature and no Serde dependency: the consuming crate adds
  `serde` itself and puts any `cfg` gating on the macro call.
- Deserialization is a construction boundary: it runs every configured
  validator. Serialization exposes the existing value.
- Integral and float serialize as the raw number; text as a string.
- Bitflag: `serde_with_auto` (default: names for human-readable formats, raw
  bits otherwise, like `bitflags`), `serde_with_name`, `serde_with_value`;
  mutually exclusive, symmetric for both directions.
- Standalone `friendship` generates no Serde.
- The user must not also derive the Serde traits the option generates (the one
  exception to "does not hide the type").

## Rejected alternatives

- **Cargo feature plus per-type opt-in.** Narrowed by the user: "The only opt-in
  is `with = [serde]`. It's caller's responsibility to gate the macro call(s)
  with cfg and to also add the dependency." Dependency policy belongs to the
  consuming crate.
- **Serde for standalone friendship.** It cannot infer a canonical
  representation.
- **Shared serde skeleton (`zz_serde.rs`, P5.3).** Saved 54 caller lines but
  cost at least 66: net +13 lines at best (5,049 → 5,062); the earlier −60
  estimate was an inference. Dropped by its own acceptance (PLAN.md D10); each
  family keeps its own Serde emitter.

## Consequences

- Each type opting in needs a direct `serde` dependency in the caller.
- Allocation in Serde backends is outside the no-allocation claim.
- `no_std` works if the caller picks compatible Serde/allocator settings.

## Sources

- omp session 01a14b3f (2026-10-17)
- PLAN.md D10; omp session 01a14d75 (2026-10-18) (README claims)
- PHILOSOPHY.md lines 218–264; README.md §Serde
