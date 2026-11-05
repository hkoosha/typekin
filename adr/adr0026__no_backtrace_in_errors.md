# ADR 0026: No backtraces in generated error values

- Status: Accepted
- Date: 2026-11-04
- Related: `examples/my_u32_exp.rs` (`try_make`, `try_map`, `try_into_*`,
  `FromStr`, `_make`, `_or_panic`, `_result_or_panic`), AGENTS.md ("Only
  `error = ()` is currently supported"), ADR 0015, ADR 0019, ADR 0022, ADR 0025

## Context

The question was whether the integral macro's generated error paths could
capture a `std::backtrace::Backtrace`, and what that would cost at runtime. The
integral paths that can fail:

- `try_make`, `try_map`: `Result<Self, Raw>`, where the error is the rejected
  raw value.
- `try_into_*`, `TryInto`, `FromStr`: `Result<_, ()>`.
- `checked_*`, `Step::forward_checked`/`backward_checked`: `Option<Self>`
  through `_make`.
- Serde deserialization, with `with = serde`.

Operators, `make`, `map`, and friend construction panic through `_or_panic` or
`_result_or_panic` instead of returning an error.

## Decision

Generated error values do not capture a `Backtrace`. Error types stay `Copy`,
without destructors, and available in `core` (`()` or the rejected raw value).
Code that needs context captures it at its own boundary.

## Evidence

A throwaway crate (stable, release, x86-64, `debug = 1`) declared `Even(u32)`.
Its `try_make` returned either `Result<Even, u32>` or
`Result<Even, (u32, Backtrace)>`, and the probe called `.ok()` on the result, as
`checked_*` would.

| Case | ns/call |
|---|---|
| Today (`Result<_, u32>`), 50% errors | 1.0 |
| Backtrace, never errors | 1.5 |
| Backtrace, 50% errors, backtraces off | 4.3 |
| Backtrace, 50% errors, `RUST_BACKTRACE=1` | 640 |
| Same, 40 frames deeper | 2,580 |
| Capture plus `Display` of the backtrace | 31,500 |

- Size: `Result<Even, u32>` is 8 bytes. `Backtrace` is 48 bytes, so the result
  becomes 56. It no longer fits in registers and is returned through memory.
- Code: today's probe compiles to `mov; not; and; ret`. The backtrace version
  has about 30 instructions: a stack frame, a 48-byte copy, a call to
  `Backtrace::capture`, and `drop_glue::<Backtrace>`.
- The capture survives `.ok()`. It's an opaque call with side effects, so LLVM
  cannot remove it when the caller discards the error. Every failed
  `checked_add` would capture a backtrace.
- `RUST_BACKTRACE=""` enables capture. std treats only `"0"` as off.

## Rejected alternatives

- **Capture a `Backtrace` on every `Err`.** Besides the runtime cost:
  - `Backtrace::capture()` is not a `const fn`. `try_make`, `try_into_*`, and
    `TryInto` are `const` under `konst = stable`/`nightly`. Literal macros
    (`name!(v)` runs `try_make` in a `const` item) and `Within` tables need that
    constness (ADR 0015).
  - `std` only. Generated integral code uses `::core` and must stay `no_std`.
  - `Backtrace` is neither `Copy` nor `Clone` and has drop glue. The current
    errors are free to copy and discard.
  - For validated types, the `test_slow` assembly comparison against
    hand-written raw code would no longer match on any path that returns `Err`,
    because the capture call stays in (ADR 0019).
  - The error type would change, which AGENTS.md does not allow.
- **Capture on panic paths.** Not needed: under `RUST_BACKTRACE` the panic hook
  already prints a backtrace, and `_result_or_panic` and `map` are
  `#[track_caller]`.

## Consequences

- Error paths stay zero-cost and keep both their constness and `no_std` support.
- Errors carry no origin. Callers that want one capture it themselves, for
  example `anyhow` converts the error and captures a backtrace on `?`.
- Open option, not measured: a `#[track_caller]` constructor that stores
  `&'static Location<'static>`. It costs one pointer, works in `const fn`, and
  has no drop glue. It still grows `Result<Self, (u32, &Location)>` past 16
  bytes, so x86-64 SysV returns it through memory, and it changes the error
  type.

## Sources

- omp session 2026-11-04 (this investigation; the benchmark crate was deleted
  after measuring)
- `examples/my_u32_exp.rs:1700-1846`, `1963-1998`
