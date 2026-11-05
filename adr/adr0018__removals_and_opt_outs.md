# ADR 0018: Removals and opt-outs (`without = friend`, `without = fn_*`)

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/zz_lib.rs` (`Forward`, `Removals::keep`),
  `src/integral_items.rs` (`el_makes`), `src/integral_cfg.rs`
  (`IntegralFlag::DEFAULT_ON`), AGENTS.md "Every generated function that
  forwards to a `core` function…", `examples/my_f64.rs`, ADR 0008, ADR 0010, ADR
  0011, ADR 0016

## Context

Callers need to remove parts of the generated API: a forwarded `core` function
they want to write themselves, a derive they implement by hand, the friend
protocol. Every removal flag has to stay sound, so turning something off can't
break code typekin itself generates.

## Decision

- **No `friend` flag.** `without = friend` is an "unknown flag". To get no
  friends, write `friend = []` plus `without = self_friend_*`. `make`/`of` are
  emitted only when the primitive has a `Make` friend.
- **`without = fn_<name>`** removes one generated function that forwards to
  `core` (or `String` for text). It only removes things: `with = fn_*` is an
  error.
  - Generated code never calls these public forwards. `Deref`, `AsRef`, `Debug`,
    `Hash`, `Eq`/`Ord`, serde and `Add` read the field directly, so every
    removal compiles.
  - Removing a known function the type doesn't generate is an error ("`fn_floor`
    removes nothing"). An unknown `fn_<typo>` is reported as "unknown flag" when
    the attribute is parsed.
  - Allowed at the root, in profile bodies and in a bitflag's
    `integral = [...]`. Rejected in facets.
  - Typekin's own functions (`make`, `map`, …) and constants such as `BITS` are
    not covered. Constants go with their family flag.
- **Default-on derives are opt-out flags.** `clone`, `copy`, `hash`, `debug` and
  `repr` (plus `display` for text) are on by default and turned off with
  `without = …`. There is one macro per family.
- **Unused friend callbacks are still type-checked.** A friend callback that
  nothing calls is referenced in a `const _` block (input/output types and
  constness are asserted) without being called. That way it won't stop compiling
  if generated code starts calling it later.

## Rejected alternatives

- **Fixing `without = friend`.** The flag only skipped the friend trait impls,
  but comparisons and `make`/`of` go through those traits. Every use failed to
  compile (missing `PartialEq`/`PartialOrd`, "`u16: Make` is not satisfied").
  "No program using it could have compiled", so removing it breaks nobody, and
  the same result can already be written another way. [INFERENCE] A real fix
  would have required deciding what `Eq`/`Ord`/`of` mean without a friend
  protocol, and that was never explored.
- **Letting rustc fail when a removed function is still used.** The original
  TODO said "We don't care if some other code uses it, since rustc will just
  fail compilation". The implementation went further and made it impossible:
  with `as_str` removed, text's `Deref` resolved to the unstable `str::as_str`,
  and "with that feature enabled it would recurse forever".
- **`with = fn_*` (opting back in).** These flags only remove.
- **Silently accepting a removal that removes nothing.** That would hide typos
  and stale config. (Compare `without = core_int` on a float, which is still
  silently accepted.)
- **Separate manual/full macros** (`ty_integral`, `ty_bitflag`, `ty_text`
  injecting repr and derives). Flags compose with profiles, facets, `konst` and
  hiding, keep the macro list small, and avoid duplicate expansion paths. See
  ADR 0016.

## Consequences

- Every new forward must be a `Forward` that passes through `Removals::keep`,
  and generated impls must not call public forwards.
- Facets can't remove functions. Use a profile or the root.
- An unused callback must still have the type the protocol expects.

## Sources

- omp session 01a1a5e0 (2026-11-04); `a03de93:PLAN_TODO.md` item 2
- omp session 01a1a37f (2026-11-03)
- Commits 404be48, 2f0ccb7
- `a03de93:PLAN_TODO.md` item 3
- omp session 01a14f36 (2026-10-18)
- omp session 01a17f3f (2026-10-27; `examples/my_u32_exp.rs`)
