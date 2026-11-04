# ADR 0007: Generated trait impls (PartialOrd methods, Eq, Debug, inlining, Display)

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/integral_impls.rs` (`make_partial_ord`, `make_eq`,
  `make_debug`), `src/text.rs`, `src/bitflag.rs`, `src/integral_cfg.rs` /
  `text.rs` / `bitflag.rs` `DEFAULT_ON`, ADR 0004 (float `Eq`/`Ord`/`Hash`),
  ADR 0019 (asm verification), ADR 0015 (`konst`)

## Context

Every wrapper gets a fixed set of std trait impls. Each must cost nothing over
the raw type, work under `konst` and with generics, and print usefully. These
choices look like simplifiable boilerplate; this records why they are not.

## Decision

- **`PartialOrd` writes `lt`/`le`/`gt`/`ge` explicitly**, forwarding to the raw
  operator, for every kind (int, float, friends), not only floats. Comment at
  `integral_impls.rs:1092`.
- **`Eq` is a hand-written empty impl**, not `#[derive(Eq)]`: "a derive cannot
  emit `const impl` under `konst`, and it would bound the generic parameters"
  (`integral_impls.rs:600`).
- **`#[inline(always)]`** on generated helpers, methods and trait methods,
  including `Binary`/`Octal`/`LowerHex`/`UpperHex` `fmt`. **Exception:
  integral `Debug::fmt`** has no inline attribute ("Except for debug, inline
  fmt"; no reason recorded). Current deviations: text `Debug::fmt` is
  `#[inline(always)]`; bitflag's value `Display::fmt` is `#[inline]`; the
  bitflag enum uses `#[derive(Debug)]`.
- **`Debug` prints the type name**: `Foo(42)`, `Foo("abc")` (`debug_tuple`).
  User rule: "a `Foo(42)` must not only print `42` it must print `Foo(42)`."
- **Defaults with opt-out** (`without = [...]`): integral `clone`, `copy`,
  `debug`, `hash`, `repr` (`#[repr(transparent)]`); text `clone`, `debug`,
  `display`, `hash`, `repr` (no `copy`); bitflag `clone`, `copy`, `debug`,
  `hash`, `repr` (`repr` governs the value wrapper; the enum's integer repr is
  required user input). Debug/Hash default on "same way copy is implemented";
  Clone/Copy/repr were unconditional before and became opt-out flags.
- **Display**: off by default for integral, float and bitflag (`with =
  display`); **on by default for text** (README:840). Bitflag enum displays its
  flag name. Bitflag value displays set flag names joined by ` | `, then
  leftover bits in hex: `"Read | Write | 0x8"` (README:918,
  `flag_format_body`), and `FromStr` parses the same form.

## Rejected alternatives

- **Default `PartialOrd` methods through `partial_cmp`.** Default `ge` costs
  3→8 insns on aarch64 and 8→15 on armv7 at O1/Os/Oz (embedded/`no_std`
  levels); explicit forwarding is identical to raw on all four targets
  (`66ecab3:float-plan.md` §2.2, D5).
- **Explicit methods for floats only.** "One shape; protects ints from future
  LLVM changes" (D5).
- **`#[derive(Eq)]`.** Not const-capable; adds generic bounds.
- **`Debug` printing only the raw value** (text printed `"abc"`). Rejected by
  the user rule above.
- **Display opt-in for text too.** Text flipped to default-on with an opt-out
  (session 01a11d13); no further reason recorded.
- **Bitflag value Display "EXACTLY as the underlying type"** (padding, `#x`,
  `b`, `o`, checked on all 12 reprs; user request in 01a0fb72). **Reversed
  later**: values now display flag names plus hex leftovers, tests were updated
  to match. No reason recorded for the reversal.

## Consequences

- Removing the four `PartialOrd` methods or switching `Eq` to a derive is a
  regression, not a cleanup; asm fixtures catch the former.
- The inline exceptions above (text `Debug`, bitflag value `Display`) are
  inconsistent with the stated policy; unresolved.
- A bitflag value's `Display` is not interchangeable with its integer's; use
  `raw()` with integer formatting for that.

## Sources

- `66ecab3:float-plan.md` §2.2, §4.4, D5; `6a9e880:prev.md` §2.2, §4.4, D5
- `ceaa744:PLAN_REFACTOR.md` (`make_eq`); `src/integral_impls.rs:600`
- omp session 01a16f65 (2026-10-24)
- omp session 01a15048 (2026-10-18)
- omp session 01a14f4f (2026-10-18)
- omp sessions 01a11d13 (2026-10-08), 01a0fb72 (2026-10-02)
