# ADR 0009: Bitflag model

- Status: Accepted (implemented), except the `Neg` rejection (planned)
- Date: 2026-11-04
- Related: `src/bitflag.rs`, `src/zz_friend.rs` (`BITFLAG_ROOT`,
  `ProtocolCapability`), PLAN.md §6.1–6.3 and §7, PHILOSOPHY.md "Bitflags Follow
  an Established Text Model", ADR 0008, 0015, 0018, 0021, 0022, 0025

## Context

`#[typekin::bitflag]` turns a unit enum into a flag set. It has to decide the
type shape, how the integer width is chosen, which friendship and integral
features apply to flags, and how values print, parse and serialize.

## Decision

- **Shape:** the user's unit enum holds the flags; a separate `{Enum}Value`
  wrapper (suffix configurable, `value_name = ".."`) holds the set. The value
  type gets one named `const` per variant, built through `_or_panic`.
- **Repr is required:** `repr = T` or `#[repr(T)]` on the enum. Neither is a
  `compile_error!`; both is an error even if they agree. There is no default.
- **Root friends accept only `Bit`.** General rule for capabilities: one that
  does nothing because its code is generated anyway stays accepted (text
  `_(Self) -> Rel`, since text always emits `Eq`/`Ord` for Self); one that is
  accepted while its code is deliberately not generated is rejected. Bitflag
  root `Make`/`Math`/`Cmp`/`unsafe` only implemented private marker traits that
  nothing bounds on, so they are rejected with a pointer to
  `integral = [friend = ..]`. Unknown root capabilities are validated too.
- **Typed protocol:** whether a protocol trait is `unsafe` and whether `make`
  calls `_blind_trust` is decided by the capability (`ProtocolCapability`),
  never by comparing the emitted trait name.
- **Nested integral:** the value type is an integral configured by
  `integral = [..]`; omitting it resolves exactly like `integral = []`. The
  nested config rejects facets, `produce`, non-default profiles, `non_zero`,
  NonZero storage and any domain that excludes zero (the empty set is 0).
  `konst` is forwarded into it.
- **Value-type storage (ADR 0033 D0.5, 2026-11-09):** the value type is
  constructed through `StoragePlan` like every wrapper (it used to write
  `Self(bits)` itself), and its storage stays the primitive. It exists to
  hold combined and unknown bits, its domain must include zero, and it is the
  typekin type most likely to cross FFI, where a rewritten bit pattern turns
  an out-of-domain value into undefined behavior. A storage that rewrites
  the field (`PLAN.md`'s enum storage, ADR 0034's niche storage) will
  therefore need an explicit opt-in in the nested configuration.
- **`Not` is bitflag's own:** `!value` is `complemented()`, masked to the
  declared flags, instead of the integral bitwise `Not`.
- **Text:** the `bitflags` retain grammar: names joined by ` | `, unknown bits
  as `0x..` hex, unknown bits retained, round-trips all bits, no allocation.
- **Serde:** `serde_with_auto` (default) / `serde_with_name` /
  `serde_with_value`, mutually exclusive; see ADR 0025.

## Rejected alternatives

- **`bitflags`-style struct with associated consts.** A `bitflags!` reference
  expansion (`my_thingy`, `ef7c6e9`, removed in `455cb27`) was compared; the
  enum + `{Enum}Value` design was chosen. No reason recorded.
- **Optional repr with `isize` fallback.** User: "Do not use isize as fallback.
  Require either `bitflag(repr = <N>)` or `#[repr(...)]`."
- **Accepting both repr forms when equal.** Rejected "Even if their value
  matches".
- **Root `Make`/`Math`/`Cmp`/`unsafe`.** "Root Make, Math, Cmp and unsafe only
  implemented private marker traits that nothing bounds on" (`10abfea`); a test
  even pinned that root `unsafe` grants nothing. Before `404be48` an unknown
  root capability (`-> Inspector`) and `without = friend` were accepted and then
  failed in rustc.
- **Name-based `unsafe` detection (`is_blind_trust_capability`).** It misread
  two inputs: a generic parameter named `Unsafe` renamed the trait to
  `__TypekinUnsafe`, which lost `unsafe` while `make` still called
  `_blind_trust`; a `friends` capability named `FlagUnsafe` became an
  `unsafe trait`.
- **Skipping nested `resolve()` when `integral = [..]` is absent.** Two paths
  for one meaning; with one path `packed` inherits it. `expandem` showed the
  only difference was `Neg`/`_neg` for a signed repr (13 lines).
- **Integral `Not` on the value.** `!flags` must stay within the declared flags.
- **Decimal unknown bits.** An early sketch; switched to `bitflags`-compatible
  hex rather than inventing another text format.

## Consequences

- Signed-repr bitflags without `integral = [..]` now gain `Neg`. PLAN.md asked
  to ship the `Neg` rejection ("two's-complement negation has no set meaning;
  the meaningful complement is `Not`") in the same release; it is not
  implemented yet.
- Value-level capabilities live only under `integral = [friend = ..]`.
- Text compatibility with `bitflags` is grammar only; the type model stays
  Typekin's.

## Sources

- `ceaa744:PLAN_REFACTOR.md` §6.1–6.3, C.3
- omp session 01a15048 (2026-10-18)
- omp session 01a0b451 (2026-09-18)
- omp session 01a0f766 (2026-10-01)
- omp session 01a14b3f (2026-10-17); `my_thingy`: arena git commits `ef7c6e9`,
  `455cb27`
- commits `10abfea`, `404be48`
- PLAN.md §6.1–6.3; `src/bitflag.rs` `resolve_repr`, `parse_integral`,
  `complemented`
