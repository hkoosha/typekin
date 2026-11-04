# ADR 0008: Friendship model

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/zz_friend.rs` (`Capability`, `emit_protocol`), `src/friendship.rs` (`emit_friendship`), `src/text.rs` (`make_friendship`), `src/integral_friend.rs`, `src/constructor.rs`, AGENTS.md "Make ownership and capability explicit", PHILOSOPHY.md "Friendship Is an Infallible Contract"; ADR 0007, 0009, 0010, 0014, 0018, 0021, 0024

## Context
A wrapper needs to say which other types may construct it and take part in its operators, and how each of them becomes the wrapper's value. Integral and bitflag each had their own copy of this machinery, and external crates wanted it "outside of integers and flags". Rust's `From`/`Into` gives one global edge per type pair and carries no permissions.

## Decision
- **A full protocol, not a marker trait.** Each target gets a relation type (the value the friend converts to), a seal trait, and one capability trait per capability that extends the seal. Friends implement these traits. Everything is emitted inside `const _: () = { .. }` (`zz_lib::lint_scope`, `constructor::Scope::Const`), so nothing outside can name the traits or add impls.
- **Grammar:** `conversion(Source) -> Caps` ("as if defining an fn"). A single declaration or a bracketed list is accepted. `_(Source)` means the implicit conversion (integral, bitflag; text only `_(Self) -> Rel`). Integral also accepts `self(Primitive)`. A bare `_` is not a declaration.
- **Capabilities are explicit.** Integral root: `Make`, `Math`, `Bit`, `Rel`, `unsafe`, `Within`.
  - `Math` implies `Make` and `Bit`. There is no arithmetic-only capability.
  - `Rel` enables all six comparisons with `Copy` friends.
  - If the capability list is omitted, it is `Math`. `-> []` grants nothing.
  - Facet friends accept only `Math` and `Bit`, and `Math` is not expanded there.
  - Bitflag root accepts only `Bit` (ADR 0009). Text accepts `Make`, `unsafe` and self-only `Rel`.
- **`unsafe` implies `Make` only.** It skips that friend's construction validation and grants no `Math`, `Bit` or `Rel`. (Naming is out of scope here.)
- **Conversions consume.** The method is `fn conversion(self) -> Relation`. There is no blanket `&T`/`&mut T` forwarding.
- **Infallible.** Friendship has no `try_of`. Untrusted construction validates and panics if validation fails. Expected rejection belongs in caller-written `TryFrom`/`TryInto`.
- **Automatic raw friendship.** An integral automatically befriends its backing primitive. An unvalidated integral also gives `Make` to every primitive that safely widens to its repr (`can_safe_cast_to`, `as` cast). Literals go through the concrete `make(raw)`, which requires the repr's `Make` friend. With a validator there is no automatic `Make`, and the entry point is `try_make`.
- **Same-type ops come from `self_math`/`self_bit`.** Declaring the wrapper as its own friend is rejected ("self math operations use `self_math`, not `friend`").
- **Standalone `friends`:** `Seal` is an empty marker. Each capability trait declares and implements its own conversion. A source type may be listed more than once if the capability sets are disjoint; any overlap is an error ("friend capabilities overlap for a source type"). The default conversion name is `to_<snake(relation)>`, or `convert`.
- **Three emitters:** integral/bitflag/float (`emit_protocol`), standalone `friends` (`emit_friendship`) and text (`make_friendship`) stay separate. The shared `emit_protocol` puts the conversion on the seal; standalone puts it on each capability.

## Rejected alternatives
- **Marker-trait-only `#[friendship(friends=[..])] pub trait Marker {}`.** User: "Currently friendship macro does nothing". The constructor machinery and the seal impls belong in it.
- **Friendship as `From`/`Into`.** "A global `Into<u32> for Source` cannot express that distinction" (one source can feed different targets in different ways), and "`From` … does not model these permissions" (PHILOSOPHY.md).
- **Generated `From<Friend> for Target` (`with = [from]`).** It was accepted at first, then dropped ("it won't be implemented"). Friendship converts *indirectly through a relation*: FriendA and FriendB can each produce the target's `u32` in their own way, so no direct edge is needed. `From` also brings in the blanket `TryFrom<Error = Infallible>`, which blocks a real fallible `TryFrom` for that pair.
- **`try_of`.** Friendship is an infallible contract. User: "My goal was to force developers to use Friendship for infallible". A fallible variant would be a second conversion model.
- **Borrowed conversion + `&T`/`&mut T` forwarding.** This was the original protocol. Text friends produce a non-`Copy` `String`. User: friendship "consumes. This pushes allocation outside of this crate's level". `Copy` friends still work by copying. Side effect: `PartialEq`/`PartialOrd` take a borrowed RHS and "cannot use an owning friendship conversion". So text `Rel` is same-type only, and integral `Rel` requires `Copy` friends.
- **Friendship levels `Full`/`None`/`MathRel`/`XCustom(..)`, with full access by default.** Replaced by an explicit list (`cap = [..]`, later `-> [..]`). No reason was recorded. [INFERENCE] AGENTS: "Do not grant operations merely because a conversion exists." Root `Math ⇒ Make + Bit` brings back a narrower bundle.
- **`Numeric` alias (Math+Bit+Relation+Make) and `Relation`.** `Relation` was a no-op ("grants no observable integral behavior"). Both were folded into `Math` and `Rel`.
- **`Ord` separate from `Eq`.** Rust does not allow it: "`PartialOrd<Rhs>` requires `PartialEq<Rhs>`".
- **Other syntaxes.** Rejected forms: `Type(conv = f, cap = [..], trusted = ..)`, `Source: Inspect + Trust => conv`, `Source: [..] => conv`, `Source => conv with [..]`, and `Source { conv:, cap: }`. The user chose the fn-like form. Old syntax is a hard error with no aliases or migration diagnostics ("Just make the breaking changes"). AGENTS: "Do not restore the obsolete `conv`/`cap` argument grammar."
- **`conv = self` on primitive friends.** Replaced by automatic raw friendship. No reason was recorded; [INFERENCE] it removes boilerplate.
- **`Self(T)` as the implicit spelling.** Rejected with a diagnostic pointing to `_(T)`. No reason was recorded.
- **Capability-only `_ -> Caps` markers.** Added to replace a dummy `conv = x`, then removed (5d21fb1). AGENTS: "no standalone marker declaration". **Typed `_(T)`** was removed in 5d21fb1 in favour of `self(Prim)`, then restored in 3beaaef. No reason was recorded for either change.
- **Single conversion on `Seal` for standalone `friends`.** When disjoint declarations of the same source were merged, the later conversion overwrote the earlier one, so `Make` ran the wrong function. User: "each trait extending Seal must do it on its own."
- **Making the literal `of(123)` work by renaming the generic `of` to `of_friend`.** That is an API cutover. **Befriending `i32`** was also rejected: "`i32 → u32` is not a safe cast". `make(raw)` was added instead.
- **`unsafe` independent of `Make`.** An earlier README said "`unsafe` alone does not grant `Make`". This was reversed in d1640cd. No reason was recorded; [INFERENCE] without `Make` there is nothing to bypass.
- **Self-friendship for `Foo + Foo`.** User: "the same concept of friendship is reused… This needs to change." Friend capabilities apply only to other operand types.
- **Inverse relation / `Project` capability.** The destination already declares the reverse edge (`header_to_id(PageHeader) -> Make`), so `Project` "would only rename `Make`".
- **One-to-many split capability.** Each destination declares its own `Make`. One macro constructing several types would couple their ingress.
- **Generated reverse operators (`u32 + Wrapper`).** Coherence forbids them because both `Add` and `u32` are foreign. For two local wrappers, an automatic reverse "would pick the wrong owner and the wrong meaning".
- **Text `Math`/`Add<ValidatedText>`.** Concatenation is not an implied relation. Mutation goes through `map`.
- **One configurable protocol emitter.** "A single configurable emitter would create a mode matrix over observable expanded code. That is more fragile than the duplication it would remove." (arena/DEDUP.md)

## Consequences
- Non-`Copy` friends work and allocation stays with the caller. A friend is spent when it is used, and cross-type `Rel` exists only for `Copy` friends.
- An untrusted friend that produces an invalid value panics. The contract has to be documented ("infallible by contract, not mathematically total").
- The integral/bitflag protocol and the standalone `friends` protocol differ in where the conversion lives. Any change to one has to be checked against the others.
- Several syntax reversals have no recorded reason (`_(T)`, `Self(T)`). Do not re-open them without new evidence. The fn form does: user, omp session 01a0fb72: "I want this: `inspector_text(InspectorSource) -> [Inspect, Trust]`, as if defining an fn."

## Sources
- omp sessions 01a0be5d, 01a0bdfa (2026-09-20), 01a0b441 (2026-09-18): marker trait → protocol, `make`, safe-widening.
- omp sessions 01a10a9f (2026-10-05), 01a10666 → 01a10857 (2026-10-04), 01a0fb72 (2026-10-02): Numeric/Rel, per-capability conversion, fn syntax.
- omp sessions 01a12c31 (2026-10-11, `self_math`), 01a15306 (2026-10-19, `unsafe` ⇒ `Make`).
- omp session 01a150a9 (2026-10-18): `_(T)` not `Self(T)`.
- omp sessions 01a14b74 (2026-10-17), 01a0f766 (2026-10-01): `From` dropped; consuming friends.
- Commits `4200870`, `5cb75a7`, `40139a0`, `97b850a`, `fb7ec18`, `3e4b0f1`, `048618f`, `5d21fb1`, `6c7405e`, `3beaaef`, `0892e78`, `d1640cd`, `9021592`, `da5904a`, `f36e8d5`, `38bf76f`, `83131e7`, `7471731`, `200af1a`; `a2c7f6c:PLAN.md` "Friendship Ownership Migration".
- omp sessions 01a14b3f / 01a14b3e (2026-10-17): `try_of`, `From`, Project, split, reverse ops, text Math.
- Three emitters: `arena/DEDUP.md`. History: `66ecab3:float-plan.md`, `8d41b61:PLAN.md`, `8d41b61:PHILOSOPHY.md`, `2936856:PLAN.md`, `ceaa744:PLAN_REFACTOR.md`, `bfd34d0:crates/typekin/README.md`.
