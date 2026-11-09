# ADR 0028: `panic_at`

- Status: Accepted (implemented)
- Date: 2026-11-06
- Related: `src/integral_cfg.rs` (`PanicAt`, `settle_panic_at`, `translate`),
  `src/integral_compute.rs` (`Maker::compute`), `src/integral_items.rs`
  (`_or_err`, `try_of`), `src/zz_conv.rs` (`OperatorOutput`),
  `src/zz_friend.rs` (`Make::try_make`), `src/integral_forward.rs`
  (the forwards kept below `anything`), `src/float.rs` (`Sum`/`Product`),
  `examples/my_panic_at.rs`, ADR 0010, ADR 0026, ADR 0033

## Context

Generated integral code panics in three ways:

- typekin's own validation (`_or_panic`, `_result_or_panic`, `map`) and
  `sequential`'s `panic!("operation overflowed")`, in every build;
- Rust arithmetic that panics in every build: integer division or remainder by
  zero (also through `wrapping_div`), and `MIN / -1`;
- Rust's `overflow-checks`, which release builds turn off.

The user wanted wrappers whose generated code does not panic.

## Decision

- A wrapper-wide key, `panic_at = anything | arithmetic | overflow | nothing`,
  names what may still panic. The default `anything` is today's expansion.

  | Level | Still panics | Reported as `Err(None)` |
  |---|---|---|
  | `arithmetic` | Rust's arithmetic | `sequential` overflow |
  | `overflow` | Rust's `overflow-checks` | also division by zero, `MIN / -1` |
  | `nothing` | nothing | also any overflow (`checked_*`) |

  Wrapping profiles keep wrapping: only division by zero is theirs to report.
  Floats never panic in arithmetic.
- Below `anything`, operators (and facet `{Op}Rhs::Output`) return
  `Result<Output, Option<Raw>>`: `Err(Some(raw))` for a result that fails
  validation, `Err(None)` when there is no result. The error is an `Option`
  because "div by zero [has] no error value", and holds the raw value because
  an invalid value is not a `Self`. The user chose `Result` over `Option`.
- Nothing relies on the domain ("I must NOT rely on `domain`"): every result is
  validated, so closure proofs, `_proven`, and `_within` are off, and `proven`
  or `prove` is an error.
- A function never changes its signature with the level: "if `of()` always
  returned value, it should not start returning Result, rather the fn should be
  omitted". So `of` gives way to `try_of -> Result<Self, Raw>` (the friend
  `Make` trait has `try_make` instead of `make`), `map` is omitted for
  `try_map`, and `core_int`/`core_float`/`float_math` functions returning a
  validated `Self` are omitted, while `checked_*` and reads stay. `ilog*`, which
  panic in `core`, are omitted where division is checked.
- `Sum` and `Product` are implemented on the operators' `Result`, folding step
  by step: "summation of a and b and c becomes `((a + b)? + c)?`".
- Assignment operators cannot report failure and are omitted; an explicit
  `assign_*` is an error. `unsafe` friends and `with_blind_trust` construct
  without validation and are errors.
- Bitflag's nested `integral = [..]` rejects `panic_at`.

## Rejected alternatives

- **A `panic` flag (`without = panic`) and a `no_panic` profile.** Replaced by
  levels ("make panic_level an enum, aka a config"), so it is not a profile.
- **Level names `lax | domain | core | none`.** The user's first names; the
  chosen ones say what may still panic, so `panic_at = nothing` reads as is.
- **`Option` outputs.** The user switched to `Result` with an `Option` error.
- **Leaving division by zero to Rust at every level.** The premise was that
  runtime checks are "removed from release builds by rustc". That holds for
  overflow checks, not for integer division by zero, so `overflow` checks it.
- **Changing `of`, `map`, or forwards to return `Result` in place.** Rejected
  for signature consistency (above).

## Consequences

- Still in the expansion below `anything`: the `NonZero` storage arm after
  validation (unreachable), compile-time layout, literal, and `Within`
  assertions, and the integer `partial_cmp(..).unwrap()` (infallible).
- `arithmetic` and `overflow` leave Rust's panics in place by design.

## Sources

- omp session 2026-11-06 (this design; the user's choices are quoted)
