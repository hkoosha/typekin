# ADR 0022: Integral API surface

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/integral_items.rs`, `src/integral_impls.rs`, `src/integral_cfg.rs`, README "The Catch", PHILOSOPHY.md; ADR 0006, 0008, 0011, 0018

## Context
`#[integral]` newtypes should feel like the core integer they wrap without ever producing a value outside the declared domain, and without spreading error types or generics through the API.

## Decision
- **Core methods mirrored** (`IntegralFlag::CoreInt`, default on): stable inherent methods of core integers, signed-only/unsigned-only by repr. Methods returning `Self` validate: they panic on an invalid result, `checked_*` returns `None`. Stable-but-non-const core methods stay non-const, "matching core's stability exactly".
- **`unsafe` core methods are never forwarded** ("Do not replicate or implement 'unsafe' stuff from core"); a compile-fail test asserts `unchecked_add` is absent.
- **`FromStr`** (`IntegralFlag::FromStr`, `Err = ()`): parses the primitive and applies the validation condition directly, not via `try_make`, because `try_make` can be disabled. [INFERENCE] `try_make` could therefore be removed without breaking `FromStr`.
- **Every operator validates its own result.** `Foo(0) + 1 + 2` fails if `1` is forbidden: "Each `Add` call must immediately produce its declared `Output`". For final-only validation use `try_map`/`map` (closure on the raw value, validated once; `src/integral_items.rs:473/486`).
- **Operators panic** on an invalid result instead of returning `Result`.
- **Errors:** `try_make` returns the rejected raw value; only `error = ()` is accepted, reserving the grammar.
- **Empty domain** keeps the normal return types; every ordinary construction is rejected.
- **Bit-part accessors** (`get_lo/hi{N}`, `get_byte/word/dword{i}`): unsigned reprs only (`src/integral_items.rs:260`), numbered least-significant first, endian-independent, none for `u8`, per-pointer-width for `usize`.
- **`make(raw)`** and automatic `Make` friendship from the repr and every safe-widening primitive (`can_safe_cast_to`), only for unvalidated types; see ADR 0008.
- `with = range` `Step` restrictions: see ADR 0011. Float consts are not covered here (ADR 0004).

## Rejected alternatives
- **Validate only the final value of a chain.** Impossible with per-call `Output`. An accumulator/builder type was offered and not built; `try_map`/`map` cover the need.
- **Names `map_raw`/`try_map_raw`.** Renamed to `map`/`try_map` by the user; no reason recorded.
- **Fallible operators, or opting out to require manual raw casts.** "It will be an unpleasant API." Panic and fallibility are separate contracts (PHILOSOPHY.md).
- **Generic error parameter.** "Adds generics to the API, making everything more painful"; would spread parameters and bounds through construction, friendship, operations and const code. "An associated type would've been ok, but since structs cannot currently have one, it's a no go."
- **Concrete caller error (`error = MyError`, `Result<(), E>` callbacks).** Deferred: "would create a second validation model" next to ranges, literal membership, multiple callbacks and const validation.
- **`!` return type for an empty domain.** `From::from` cannot return `!`, `Result<!, ()>` is unstable, and a bare `!` breaks `.unwrap()` callers.
- **Signed bit-part accessors (two's-complement, unsigned results).** Explored, not taken: "callers may mistake two's-complement fragments for numeric decomposition".
- **Forwarding `unsafe` core methods.** Excluded by the user (above).

## Consequences
- Chains with a forbidden intermediate panic even when the final value is valid; use `map`/`try_map`.
- Validation cost on every operator; README suggests feature-gating validators in non-release builds.
- No structured validation errors until a concrete error model is designed.

## Sources
- omp sessions 01a11d13 (2026-10-08), 01a0fb72 (2026-10-02)
- omp sessions 01a0bdfa (2026-09-20), 01a0b441, 01a0b559 (2026-09-18)
- omp session 01a14b3f (2026-10-17)
- omp sessions 01a14b74 (2026-10-17), 01a14d75 (2026-10-18)
- `bfd34d0:crates/typekin/README.md` "The Catch"
- `8d41b61:PHILOSOPHY.md`, `bfd34d0:crates/typekin/README.md`
