# ADR 0011: Profiles and storage

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/zz_profile_integral.rs`, `src/integral_cfg.rs`
  (`resolve_layers`, `resolve_facet_layer`, `NonZeroPolicy`, `settle_flags`),
  `src/integral.rs` (`StoragePlan`), `src/zz_conv.rs` (`SelfConverter`),
  `src/integral_impls.rs` (`Step`), README "Integral production and profiles",
  AGENTS "Preserve profiles, facets, and storage"; ADR 0006 (domain), ADR 0010
  (facets), ADR 0012 (point/offset, `precedes`), ADR 0013 (zero exclusion across
  widths), ADR 0015 (const)

## Context

Integral wrappers need behaviors beyond "integer-like with primitive operators":
wrapping, checked, serial-number and affine-point semantics, plus physical
representations (`NonZero*`, `Wrapping`) that give niches and keep type
information. These must compose with the existing `with`/`without` flags and
with facets without a second configuration system.

## Decision

- **Profiles are presets over flags.** `profile = NAME { settings }` clears/sets
  flags; explicit root `with`/`without` always win, regardless of order. Profile
  bodies `{}` are optional.
- **Data model:** one
  `enum Profile { Int(Int), Circular(Circular), Delta(Circular), Sequential, Wrapping, Point, … }`
  with per-variant structs; each variant parses itself as if no other existed. A
  key a profile doesn't own (e.g. `precedes` on `int`) is the generic
  `unknown attribute`, not a special-cased error. Preset effects live in the
  `Profile::preset_off` table (replaced per-profile `profile_apply_*`
  functions).
- **Default ≠ `int`.** `Profiles(Vec<Profile>)`, and `is_default()` means "no
  profile given" (empty list), not "resolves to int". Bitflag's nested integral
  checks `is_default()` before resolving.
- **Ordered fold:** `profile = [a, b]` applies left to right, then root flags.
  Arithmetic is taken from the *last profile that carries arithmetic*
  (`Profiles::effective`), so `point` (no arithmetic) layers on top:
  `[circular, point]` stays primitive-wrapping.
- **Profiles:** `int` (primitive ops), `circular` (`wrapping_*`, no linear
  order, serial `precedes`), `delta` (= circular with `precedes` fixed off;
  `precedes =` rejected), `sequential` (`checked_*`, panic on overflow),
  `wrapping` (physical `Wrapping<P>`, linear order, ordinary API), `point`
  (surface only: no point+point, allows point−point).
- **Shorthands:**
  `#[typekin::{int,circular,delta,sequential,wrapping,point}(...)]` ≡
  `integral(profile = NAME, ...)`; they cannot take `profile =` (use `integral`
  for lists); profile settings go directly in the shorthand args.
- **`produce = conv(_) -> T`** (was `out_type`): one output policy per type
  (facets may override). The wrapper result is built and validated first, then
  converted. `_` is the required placeholder (`Self` argument rejected). Never
  absent: default is `Self(_) -> Self`, which calls nothing; any non-`Self`
  output drops assignment operators (`a op= b` can't change `a`'s type).
- **NonZero storage:** users declare the primitive; when the resolved domain
  proves zero impossible on every pointer width, the private field becomes
  `NonZero*`. `non_zero` is three-state (`NonZeroPolicy`): unset → infer;
  `with = non_zero` → add `value != 0` and require NonZero; `without = non_zero`
  → suppress storage only, domain unchanged. Facet-local `non_zero` is rejected.
- **Storage rewrite only for private fields.** `raw()`, parsing, Serde, friends
  and constructors keep primitive `P`. One
  `StoragePlan { raw, non_zero, wrapping }` is resolved once at the macro
  boundary and every storage read/construction goes through it. Combined:
  `Wrapping<NonZeroP>`.
- **Custom `or_panic` constructor + NonZero storage is an error** ("safe
  generated APIs cannot delegate structural validity to an opaque custom
  constructor").
- **Wrapping storage is wrapper-wide:** used if the effective root profile or
  any explicit facet's effective profile is `wrapping`; each slot keeps its own
  arithmetic. An overridden earlier `wrapping` layer does not force it.
  `circular`/`delta` use `Arithmetic::PrimitiveWrapping` on primitive storage;
  `wrapping` uses `WrapperWrapping`.
- **Runtime vs const:** runtime `WrapperWrapping` emits
  `Wrapping(lhs) op Wrapping(rhs)`; const lowering falls back to primitive
  `wrapping_*` (`Arithmetic::make_const_expr`).
- **`with = range`** (nightly `Step`) is rejected with `domain`, `valid`,
  `non_zero`, or a circular profile; Wrapping storage is allowed.

## Rejected alternatives

- **`ProfileConfig` + `IntProfile` split.** User: "do not make sense. Each
  IntProfile instance must be a struct instance with the fields it needs."
- **Special-case error for keys from other profiles.** User: "must give the feel
  that each enum instance has its own unique parser unaware of others."
- **Making resolved `int` report `is_default()`, or a separate
  `enum Default { Integral }` tag.** "the whole point … was that
  `default != integral`. You can't cheat by having them equate." Default will
  mean different things for bitflag and text.
- **Parse a list, select one profile.** "Do not model `profile` internally as
  one enum selected from a list; model it as an ordered sequence of profile
  applications."
- **Last profile wins wholesale.** Would make `[circular, point]` revert to
  primitive arithmetic; replaced by last-arithmetic-profile-wins.
- **Moving root `profile` into a special `self` facet.** User reverted: "Syntax
  stays the same, at resolution time it goes into default facet."
- **Generic `minimal`/`restricted` profile** to shorten TCP `without` lists.
  Most entries were redundant with `circular`; `point` names the remaining
  semantic distinction. `examples/my_tcp.rs` still deliberately uses
  `circular` + `without`, per user ("do NOT change any of its profiles").
- **Names for `delta`:** `unordered` (describes an omission, not the role),
  `no_precedes` (exposes a generator switch), `circular_offset` (repeats
  inherited behavior). `offset` was picked first on the claim `delta` doesn't
  fit unsigned; user objected, assistant conceded ("My distinction was wrong").
  `point` = position, `delta` = relative change.
- **Required `{}` bodies.** Dropped as noise; no further reason recorded.
- **`Option<SelfConverter>` for no output policy.** Replaced by required
  converter defaulting to `Self(_)`, treated as identity.
- **`Self` as converter argument.** Replaced by `_`; "the code is correct not
  the plan".
- **Declaring `struct Id(NonZeroU32)`, or inferring nonzero from `valid`
  callbacks.** Callbacks are opaque; a primitive field plus validation gives no
  `Option<T>` niche. Requirement: "if the effective domain excludes zero, we
  will use a NonZero* type."
- **`non_zero` as plain boolean / `without = non_zero` weakening the domain.**
  Storage and semantics are separate concerns.
- **Rewriting public fields.** Silently changes a public type and lets consumers
  depend on storage internals.
- **Mandatory `Wrapping` storage for `circular`.** First plan did this; user
  correction: "No, not mandatory Wrapping storage for circular profile. Only for
  wrapping profile." Forced the `PrimitiveWrapping`/`WrapperWrapping` split.
- **`wrapping` as an alias of circular, or omitting physical `Wrapping`.**
  Keeping real `Wrapping` preserves semantic type information and leaves room
  for Rust-specific optimizations.
- **Per-slot storage.** One Rust type has one field representation.
  **Sticky wrapping** (any historical wrapping layer forces storage): layering
  decides effectiveness.
- **Decorative `Wrapping` field with primitive `wrapping_*` everywhere.** Would
  not express the standard-wrapper semantics. Const keeps primitive lowering
  [INFERENCE: std `Wrapping` operators are not const].
- **`Step` for validated/nonzero/circular types.** "`Step`'s successor laws
  cannot be upheld" with validation holes or wraparound.

## Consequences

- One global output policy per type at root; mixed point/offset outputs need
  facets (ADR 0010).
- Bitflag's nested `integral` rejects `produce` and non-default profiles.
- Internal names lag public ones (`ty_out` for `produce`).
- NonZero inference depends on the domain prover being exact across 16/32/64-bit
  (ADR 0013); unprovable domains silently keep primitive storage.
- README macro inventory near line 47 omits `point`.

## Sources

- omp sessions 01a173b7 (2026-10-25), 01a178e8 (2026-10-26)
- omp session 01a16f65 (2026-10-24)
- omp sessions 01a173f8 (2026-10-25), 01a15306 (2026-10-19), 01a179dd
  (2026-10-26)
- commits f36e8d5 (singular keys, optional bodies), 05ab17b (`produce`),
  3cf8dc1, c790a42 ("profiles accepts list"), e9b5728/74f88d8 (`preset_off`)
