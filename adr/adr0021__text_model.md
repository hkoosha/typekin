# ADR 0021: Text model

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/text.rs`, `src/lib.rs` (`text` docs), README, PHILOSOPHY.md,
  AGENTS.md; ADR 0002, 0006, 0007, 0008, 0015

## Context

`#[text]` wraps a `String` with a validation invariant. Any mutable alias to the
inner `String` would let callers break that invariant, and mutation that can
fail has to decide what happens to the value.

## Decision

- No mutable escape hatches: no `DerefMut`, `AsMut<str>`, `as_mut_str`,
  `&mut String`, `drain` or other guard APIs. Only read-only access (`&str`,
  `Deref<str>`, `AsRef`, `Borrow`).
- Validators take `&str`, so validation needs no ownership transfer or clone.
- The base mutation is `map(self, FnOnce(&mut String)) -> Result<Self, ()>`: it
  consumes self, mutates, revalidates. On failure the candidate is dropped;
  there is no clone and no rollback. If the closure panics, the consumed value
  is dropped.
- Failure is `Err(())` (`src/text.rs:457-560`).
- Validation is `valid = …` **or** `domain = {"a", "b"}`, never both
  (`"valid" and "domain" cannot be used together`, `src/text.rs:251`).
- Literal `domain` uses the set braces of ADR 0006 but keeps its own parser; it
  does not go through the integral range parser. Duplicates are rejected;
  `domain = {}` admits nothing.
- Infallible `From<String>`/`From<&str>` only without validators.
- `Display` is on by default (`without = [display]` opts out).
- No literal macro for text.
- `std = true` (default) emits `::std` paths; `std = false` emits `::alloc`
  (`src/text.rs:195-202`).
- Rustdoc on `typekin::text` warns that text is not an enum replacement
  (`src/lib.rs:473`): heap `String`, not `Copy`, string comparisons, typos in
  match arms still compile.
- Allocation claim (README:27) is limited: text allocates only when the
  underlying `String` operation does.

## Rejected alternatives

- **`DerefMut` / `AsMut<str>` / `&mut String` / `drain`.** They would expose an
  invalid state; such APIs "cannot and MUST NOT be generated".
- **Validators taking `String`.** Would "require cloning or consume the value
  being constructed".
- **`Err(String)` (return the invalid candidate).** For construction it would
  mirror integral's `Err(raw)`. For mutation the original valid value cannot be
  kept without clone or rollback, so the meaning is ambiguous. User: "It's
  confusing what's being returned. So no." A separately named recovery API with
  a concrete `RejectedText`-style wrapper is deferred, not built.
- **Restoring the original on failure.** Needs "cloning, allocation, or an
  operation-specific rollback log".
- **`valid` AND `domain` composed.** The first text plan composed them with
  logical AND; `184d932` made them exclusive ("Choose exactly one validation
  mode"). No reason recorded for the reversal.
- **Text `domain` through the integral range/expression grammar.** Text
  membership got its own parser so it does not disturb integral's; later kept as
  a literal list (now `{…}`) rather than adopting the expression grammar.
- **`Display` opt-in (`with = [display]`).** Was the earlier rule; flipped in
  `048618f` ("Make display behave as core"). No further reason recorded.
- **`AddAssign` for text.** Deferred: a transactional version "must leave the
  live wrapper valid after validation failure or panic", which needs a clone.
- **Literal macro for text.** "Owned `String` construction is not const on
  stable."
- **"Text doesn't allocate itself" in README.** Too broad:
  `try_from_str`/`From<&str>` call `String::from`, `map` and push may
  reallocate.

## Consequences

- Every mutation pays a full revalidation; a failed mutation loses the value.
- Callers cannot recover a rejected candidate until a named recovery API exists.
- Operators like `Add<&str>` panic via `map` (see ADR 0022 for the
  panic-vs-`Result` rule).

## Sources

- omp sessions 01a14b74 (2026-10-17), 01a14d75 (2026-10-18), 01a0f766
  (2026-10-01)
- omp session 01a14b3f (2026-10-17)
- omp sessions 01a11d13 (2026-10-08), 01a0fb72 (2026-10-02)
- omp session 01a1a5e0 (2026-11-04) (enum warning)
- commit `184d932`; old text plan `a2c7f6c:PLAN.md`
- `a2c7f6c:PLAN.md`, `8d41b61:PHILOSOPHY.md`, `a03de93:PLAN_TODO.md`,
  `556c009:PLAN_COMPILE_TIME.md`; PHILOSOPHY.md (rejected-text rationale)
