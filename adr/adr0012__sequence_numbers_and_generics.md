# ADR 0012: Sequence numbers, point/offset, generic markers

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `examples/my_tcp.rs`, `examples/my_sequence.rs`, `make_precedes`
  (`src/integral_items.rs`), `src/zz_profile_integral.rs` (`circular`, `delta`,
  `point`), `src/integral.rs` (phantom fields), README (generics around line
  362, profiles around line 575, TCP example around line 699), `arena/TEXT.md`,
  ADR 0008, ADR 0010, ADR 0011

## Context

The requirements come from Noah Kawaguchi's article "Encoding TCP in Rust's Type
System" ([Typenet](https://github.com/noahkawaguchi/typenet); copy in
`arena/TEXT.md`). It builds up TCP sequence numbers in stages. A comparison of
the article with typekin found: "Typekin starts by giving wrappers integer-like
behavior; the article progressively removes integer behavior and replaces it
with a specific algebra." Measured gaps at the time:

- generated `+`/`-` did not wrap TCP-style;
- `Point - Point -> Offset` had to be written by hand;
- generic `In`/`Out` markers were impossible;
- `Seq(Wrapping<u32>)` was rejected.

## Decision

- **Comparison:** circular wrappers get no `PartialOrd`/`Ord`. They get a
  `precedes` method that uses the strict half-range rule: equal and antipodal
  values return `false`. `precedes = false` suppresses it and is accepted only
  on `circular`. `delta` is `circular` with `precedes` fixed off.
- **Point/offset split:** positions (`point` profile: no point+point,
  point−point allowed, composable as `[circular, point]`) are separate from
  relative changes (`delta`). Offsets may be signed or unsigned.
- **Generic markers:** one generic declaration (`SeqPoint<D: Direction>(u32)`,
  `SeqOffset<D>(i32)`) instead of four concrete types. Typekin appends one
  private `PhantomData` field per parameter, in declaration order:
  `PhantomData<T>` for types, `PhantomData<&'a ()>` for lifetimes, private
  marker types for const parameters. There is no macro option for these fields.
- The value field must be one concrete primitive integer, and it must be private
  in generic declarations.
- `examples/my_tcp.rs` deliberately still uses `circular` +
  `without = math_add` + facets, not `point`.

## Rejected alternatives

- **`PartialOrd`/`Ord` for wrap-around comparison.** It "violates both the
  transitivity and duality (`a < b` if and only if `b > a`) requirements", so
  the article (and typekin) "stick with `a.precedes(b)`". (Before that it was a
  free function `seq_lt`, then a `SeqCmp` trait so that the name sits between
  its operands.)
- **`SeqDist` instead of an offset.** "A distance is symmetric and non-negative,
  while an offset … is signed and directional."
- **`Wrapping<u32>` as the model or as the field.** `Wrapping<u32>` is just as
  much a round-robin counter. It defines `<`, and sequence numbers must not have
  that. A `Seq(Wrapping<u32>)` field is still rejected: one primitive field
  only. `Wrapping` storage comes from the `wrapping` profile instead (ADR 0011).
- **Offsets always signed (like a signed `Duration`).** Window sizes and payload
  lengths are naturally unsigned, so both are allowed. `delta` was named for
  "signed or unsigned relative changes" after `offset` was wrongly argued to be
  signed-only.
- **Four concrete wrappers (`InSeqPoint`, …).** Friendship would be enough for
  them, but it duplicates declarations. "Friendship controls which *types* may
  interact; it cannot distinguish inbound and outbound values of the same type."
  Phantom generics separate directions. Generics were first deferred ("Let's
  skip generic for now").
- **One `PhantomData<(...)>` for all parameters.** Rejected so that field order
  mirrors declaration order.
- **`PhantomData<fn() -> T>`.** "`PhantomData<T>` is intentional: it gives …
  normal ownership, variance, drop-check, `Send`, and `Sync`. Do not substitute
  `PhantomData<fn() -> T>`."
- **Generic numeric field (`struct Value<T>(T)`).** Kept rejected so that the
  primitive, storage plan, profiles and domains are "statically resolved."
- **Public value field in generic declarations.** The appended phantom fields
  change the tuple constructor's arity, so the field is private and construction
  goes through `make`/`try_make`/`of`.
- **Converting `my_tcp` to `point`.** The assistant started the change and the
  user stopped it ("don't change tcp example yet"), then allowed only dropping
  redundant flags ("do NOT change any of its profiles"). No further reason
  recorded.
- **A generic `minimal`/`restricted` profile instead of `point`.** "Remove the
  redundant settings; use `point` if we want to name the remaining semantic
  distinction."

## Consequences

- `Foo<In>` and `Foo<Out>` get separate friendship instantiations but share one
  configuration. Behaviour that differs per marker needs concrete types.
- Typekin prevents mixing directions, but it cannot prove protocol-state facts
  such as `SND.UNA <= SND.NXT` or that live values stay within half a range.
  Level 4 of the article still does not catch `snd_una - snd_nxt`. The README
  says the TCP example shows "sequence arithmetic and stream separation, not a
  TCP transport."

## Sources

- `arena/TEXT.md` (Kawaguchi, Typenet)
- `6a9e880:prev.md`, `PLAN_PACKED.md`, `PLAN_WITHIN.md`; arena git commits
  (`git -C arena log`); omp sessions 01a178e8 (2026-10-26), 01a1737e
  (2026-10-25)
- omp sessions 01a1752b (2026-10-25), 01a16f65 (2026-10-24); arena `PLAN_2.md`
  (deleted)
