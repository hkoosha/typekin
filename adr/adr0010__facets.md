# ADR 0010: Facets

- Status: Accepted (implemented)
- Date: 2026-11-04
- Related: `src/integral_friend.rs` (`make_facet_dispatch`,
  `facet_rhs_trait_decl`, `facet_rhs_impl`, `facet_operator_impl`,
  `facet_assign_impl`),
  `src/integral_cfg.rs` (`FacetSlotKey`, `resolve_facets`), `src/integral.rs`
  (`with_global_operators`), README §facets, AGENTS.md "Preserve profiles,
  facets, and storage", ADR 0008, ADR 0011, ADR 0012, ADR 0015

## Context

A type-wide output policy (`out_type`, now `produce`) cannot express "`A + B`
and `A + A` have different output types" (for example `Point - Point -> Offset`
but `Point - Offset -> Point`). The unit of configuration has to be one
`(operator, RHS type)` slot: each slot needs its own arithmetic policy, friends
and output conversion.

## Decision

- `facet = FACET | [FACET, ...]` on one `integral` invocation. Each facet is an
  independent identity that owns its profile behaviour, friends (only
  `Math`/`Bit` operand slots), operation surface and `produce`. It inherits root
  settings unless it overrides them.
- The root configuration is the **implicit default facet**. Explicit facets
  claim their exact slots first, and the root fills the slots nobody claimed.
  Two explicit facets claiming the same slot is an error ("duplicate explicit
  facet slot"). Root `Make`/`Rel` stay wrapper-wide.
- Omitting `facet` and writing `facet = []` mean different things. Omitted uses
  the global operator path (`with_global_operators()` is `!facets_present()`).
  An empty list turns on facet dispatch with no explicit facets. A nested
  integral config inside a bitflag rejects `facet` entirely.
- Codegen: one RHS dispatcher trait per operator (`{Op}Rhs`), **one** blanket
  std-operator impl per operator, and one concrete dispatch impl (with an
  associated `Output`) per RHS slot.
- The dispatcher trait is `#[doc(hidden)] pub`, not private. A public wrapper
  exposes the associated-`Output` projection, so Rust's privacy check requires
  the trait to be public. It still cannot be named, because it lives in the
  anonymous expansion scope.
- A facet's result is validated by the left-hand wrapper before the output
  conversion runs.
- A slot whose output is `Self` gets a concrete `{Op}Assign<Rhs>` impl while
  its owner has the `assign_*` flag on, forwarding to the slot's `{Op}Rhs`
  impl. Facets inherit the root's flags, so the root's assignments survive
  turning facets on. An explicit `assign_*` needs a `Self` output and an owned
  slot of its operator; shift assignments stay wrapper-wide. The assignment is
  const only with its operator, and an explicit `konst_*` splitting them is an
  error. (Amended: assignments were first rejected; see below.) Below
  `panic_at = anything` (ADR 0028) slots output a `Result` and get no
  assignments.
- Constness (`konst`), `non_zero`, `precedes` and removals are wrapper-wide and
  cannot be set per facet. One exception: a `wrapping` profile on any facet
  switches the whole wrapper to `Wrapping` storage (ADR 0011), and each slot
  keeps its own arithmetic.

## Rejected alternatives

- **TCP-specific `affine = point` mode (generic phantom fields plus generated
  point/offset algebra).** It encoded one application instead of solving per-RHS
  outputs in general. The user's verdict: "Nope. I don't like this." `point` now
  exists only as a composable surface profile (ADR 0012).
- **Stacking several `#[integral]` attributes (core/self/binary profiles, named
  kernels or namespaces).** "No, this is horrible. It can be simpler given that
  we have friendship." The plan required that facets "must not require stacking
  multiple `#[integral]` attributes or introduce a special-purpose affine
  macro."
- **One blanket impl per facet (`impl<T: FacetMath> Add<T> for A`).**
  "Implementations are not scope bound, they are global." Rust treats
  independently generated blanket impls as potentially overlapping even when
  their private friendship traits are distinct, and "anonymous `const` blocks do
  not scope trait or inherent implementations."
- **Private dispatcher trait.** Rejected by the compiler's privacy check (see
  Decision).
- **Naming it `profiles`.** `profile = [P1, P2]` is already a one-or-list
  *pipeline*, while facets are a *set of identities*. "Calling both 'profiles'
  obscures this semantic distinction."
- **Naming it `overloads`.** The construct "has more identity, identity of its
  own." `domains` and `contexts` were considered too. The draft's plural
  `facets` became singular `facet` with one-or-list syntax.
- **Moving the root syntax into an explicit `self` facet.** The user first asked
  for it, then reversed: "Actually changed my mind. You were right. Syntax stays
  the same, at resolution time it goes into default facet." Public syntax stays
  unchanged; root config becomes the implicit default facet during resolution
  (omp session 01a173b7, 2026-10-25).
- **Blanket assignment operators on facets.** "A dispatcher may return a
  different associated type, so a blanket `AddAssign<T>` is not sound." Facets
  first generated no assignments at all, which also dropped the root's `+=`
  once facets were on, and users wrote `AddAssign` by hand. Concrete per-slot
  impls, emitted only where the output is known to be `Self`, avoid both the
  unsoundness and blanket overlap.
- **Converting before validation.** Validating first is intentional: "a result
  rejected by the left-hand wrapper's validator cannot be converted into an
  otherwise valid target." A consumer test shows `2+3=5` rejected through the
  facet path.
- **Name `legacy_operators`.** Renamed to `global_operators`. "Legacy" was
  misleading: it is the non-deprecated path used whenever `facet` is omitted.
  There is no migration or compatibility framing ("There is 'no current user'.
  Hence no migration.").

## Consequences

- One generic declaration still produces one configuration. Behaviour that
  differs per marker needs concrete types (ADR 0012).
- Facet friends select operand slots only. Wrapper-wide capabilities stay at the
  root.
- Facet and global paths share `Arithmetic::make_operator_expr` with an
  `ArithmeticOp` enum (the user rejected `operation: &str`: "It must be enum").

## Sources

- omp session 01a1737e (2026-10-25); arena git commits (`git -C arena log`)
- arena git `1847020`, `d74f084`, `d2a8c3e`, `d01798b` (`PLAN.md`; the implicit
  default facet appears in `d2a8c3e`)
- omp sessions 01a173f8 (2026-10-25), 01a16f65 (2026-10-24), 01a173b7
  (2026-10-25)
