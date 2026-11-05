# ADR 0006: The `domain` language

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/zz_domain.rs` (EBNF module doc), `src/zz_domain_int.rs`,
  `src/zz_domain_float.rs`, `src/zz_lib.rs` (`set`, D12 order), `src/text.rs`,
  README domain section, AGENTS.md:121-132; ADR 0002 (literals), ADR 0003
  (closure proofs), ADR 0004 (floats), ADR 0021 (text)

## Context

`domain` states which values of the field's type are valid. typekin must be able
to evaluate it at expansion time: to merge ranges, to probe for zero (which
decides `NonZero` storage), and to prove closure. The emitted check must also
cost no more than the hand-written one. Earlier notations either did not read
like what they meant or prevented some of that analysis.

## Decision

- The domain is a set expression with no variable:
  `domain = (1..12 | 87..) & !{10} & !mul(3)`. The atoms are Rust ranges,
  rosters `{…}`, `mul(c)`, predicates, and images `lens in S` / `lens !in S`,
  e.g. `abs in ..=1000` or `rem_euclid(16) in {3}`. The operators are `|`, `&`
  and `!`, the operators `HashSet` already uses.
- There are no comparisons and no arithmetic. `>c` is written `c.. & !{c}`, and
  a lone value needs braces: `{5}`.
- Constants are literals, `<prim>::MIN`/`MAX` (`u32::MAX` is accepted, a bare
  `MAX` is not) and the float constants. User constants, parenthesized constants
  (`(1)..5`, D17) and arithmetic (`1..10 - 2`) are errors. There is no
  difference operator; write `& !{…}`.
- Precedence: `&` binds tighter than `|`, as in Rust and in logic. That is the
  only precedence rule; every other ambiguity is an error that asks for
  parentheses.
- An `in` set is one value-unary. `|` or `&` right after it is an error, because
  it could extend either the image's values or the surrounding set. `!` before a
  `closed` operand is unambiguous, so `!1..5` is accepted.
- Normalization never adds a comparison. Redundant `!{c}` are dropped, and rule
  N8 turns a `!{c}` at an interval end into an exclusive end.
- A domain that admits every value or none is an error (D11).
- The domain is checked before `valid` callbacks (D12): callbacks only ever see
  in-domain values, and the cheap rejection runs first.
- Lenses are a whitelist of pure, total functions (`abs`, `count_ones`,
  `rem_euclid(c)`, `isqrt` for unsigned, …). `pow`, `ilog*`, signed `isqrt`,
  `checked_*`/`wrapping_*`/ `saturating_*`, `powi`, transcendentals and
  arbitrary calls are rejected; use `valid = path`.
- Float divisors (`mul`, `rem_euclid`, `div_euclid`) must be exactly the written
  decimal (D13); `mul(0.1)` is an error.
- The grammar is an EBNF in the `//!` module doc of `zz_domain.rs`. It is the
  source of truth; the "In plain words" section after it may lag behind.
- Text domains use braces too (`{"a", "b"}`) but keep their own parser
  (`zz::set`). They never go through the integral parser.

## History

1. Comma-separated ranges were rejected because "commas are reserved for ANDed
   lists".
2. Next came `in = 1..=1023 + 49_152..=65_535`, using `+` as the union.
3. `+` was replaced by a single range or a bracket list `[a.., b..]`. Bounds had
   to be constants (user: "`u32::MAX` is ok, `MAX` is not … `foo..` is also not
   ok as foo is a var"). Ranges were sorted and merged, with bounds stored as
   sign+magnitude over `u128`, because "`i128` cannot represent `u128::MAX`;
   `u128` cannot store any negative value". Today's
   `Int::{Negative, NonNegative}` still stores bounds this way.
4. A logical expression replaced the lists: `(1..12 || 87..) && !=10 && !%3`
   (float-plan, user-approved breaking change).
5. Commit `13703fb` introduced today's set notation. The user had called the
   logical syntax "made up". The complaints:
   - comparisons had no left side (`!=10`);
   - `%3` reads backwards (in Rust, `x % 3` nonzero means *not* divisible), so
     `!%3` double-negates;
   - `||` between sets matches neither Rust nor math.

   The tree already had set semantics, so only the parser changed. Resolution,
   the zero probe and the proofs did not. Every migration diagnostic was removed
   on the user's order.

## Rejected alternatives

- **Interval brackets `[1, 12)`, `∪`, `∈`.** rustc does not lex them
  ("unexpected closing delimiter", "unknown start of token"). Rust ranges are
  the interval notation that is available.
- **Set-builder notation.** Too verbose (user).
- **Rust with an explicit hole (`_ % 3 != 0 && _ != 10`).** The longest option,
  and ranges would still need an invented `_ in 1..12`.
- **`3Z` / `16Z + 3`.** Rust readers know suffixes as types (`3u8`). User: "3Z
  is a no go, use mul(3)". Residues are written `rem_euclid(16) in {3}`.
- **A difference operator `-` / constant arithmetic.** "`1..10 - 2` should in no
  case be `1..8`" (user). `& !` is preferred.
- **User constants in bounds.** Constants are evaluated at expansion time for
  the `NonZero` probe and the closure proofs. Allowing user constants would give
  up those analyses for any atom that used one.
- **No precedence at all.** Tried: mixing `|` and `&` required parentheses, on
  the argument that ∪/∩ have no agreed precedence. Reverted by the user
  ("AndOverOr was exception, let's bring that one back"). Two things were kept
  from the attempt: the renderer still parenthesizes, and the NaN hint still
  reads `(domain) & !is_nan`.
- **A leading `!abs in S`.** It reads as complementing `abs`; `abs !in S` reads
  as "not in", as in Kotlin. `abs in !S` stays because `!` inside value sets is
  needed anyway.
- **`in` sets running to the end of the expression.** The first implementation
  did this, and it needed parentheses even where the reading was unambiguous. It
  was replaced by the single-value-unary rule.
- **`let`-binding lens results.** Measured +2 instructions on armv7 `Os` (23 vs
  21); the call is duplicated instead, and LLVM CSEs it.
- **Canonicalizing `!=` into intervals.** Measured 8 → 16 instructions.
  Exclusions stay opaque.
- **De Morgan, distribution, expanding float negations.** Measured 4 → 8
  instructions, and wrong without `is_nan`.
- **Arbitrary paths, closures and calls.** "Purity and totality are
  unverifiable; `valid = path` already exists."
- **`powi` and transcendentals.** "Membership would depend on the opt-level or
  the platform."
- **`pow` as a lens.** It can overflow.
- **Parenthesized constants (accepted with lookahead, D17).** Ambiguous with
  grouping, and with no arithmetic nothing needs them.
- **Accepting tautologies and contradictions silently (D11).** Rejected;
  `domain = ..` is an error, while `..` inside a larger expression is fine.
- **Keeping `[..]` beside the expression syntax, or migration diagnostics.**
  AGENTS.md forbids compatibility shims and migration diagnostics, and the user
  declined them.
- **Routing text membership through the integral range parser.** AGENTS.md: "Do
  not force integral ranges and text literal membership through the same
  parser."

## Consequences

- Some meanings are longer to write: `abs in ..=1000` for `abs <= 1000`,
  `c.. & !{c}` for `>c`. N8 keeps them free at runtime.
- A named constant cannot be used in a domain; the value has to be repeated as a
  literal.
- The ambiguity errors quote the user's spelling and name both readings (tested
  in `t_domain.rs`).

## Sources

- `git show 66ecab3:float-plan.md`: §2.7, §5.5, §5.7, §12, D10–D17.
- Commit `13703fb` "domain syntax".
- omp session 01a1a64c (2026-11-04): set-notation session, turns 1–7.
- `66ecab3:float-plan.md`, `8d41b61:PLAN.md`, `ceaa744:PLAN_REFACTOR.md`,
  `bfd34d0:crates/typekin/README.md`.
- omp session 01a107be (2026-10-04): `+` removal, const-only bounds,
  sign+magnitude.
- Commits `6a9e880`, `13703fb`, `c819096`, `5d21fb1`, `e9b5728`.
