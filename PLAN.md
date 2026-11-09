# PLAN: check levels, operator forms, and removing checks

Status (2026-11-08): design only, nothing implemented. This replaces the
2026-11-06 plan (`checks_level = typekin | core | none`); §9 lists what changed.
Revised the same day:

- regular forms, how each `enforcement` value handles failure, and no operation
  helpers (§3.3, §3.6, L7-L9, L11-L13, L16, L21-L27);
- storage that tells LLVM the domain (§4.4, L10);
- opt-in `unsafe` and where undefined behavior can arise (§4.5, §4.6, L14,
  L15);
- blind trust replaced by friends with a typekin domain (§3.7, L17-L20);
- two domains, the API domain and the accepted domain (§3.8, L28, L29);
- answers on `refine`, the error type, panic messages, construction at
  `total`, duplicates, `_eq`/`_cmp` and `of()` (L30-L36), on `lax` and
  `strict_*` (L37-L40), and on checked forms (L41-L43), and future work (§13).

§7 lists the open questions by the phase that needs them.

## 1. Goal

Each type gets one `enforcement` (L21, L25):

- `lax`: rustc's checks are left to rustc, as the build configures them.
  Typekin's own checks run only when `debug_assertions` is on. Failures panic.
- `invariant` (the user's "auto"; the default): rustc's checks are left to
  rustc. Typekin's own checks always run. Failures panic.
- `strict`: typekin assumes rustc checks nothing, overflow and `/ 0` included.
  It checks every precondition itself, before the operation, in every build.
  Typekin's own checks always run. Failures panic.
- `total`: like `strict`, but a failure is returned as `Result`; nothing panics.

Every operation that can fail, before proofs, has a checked form named and
typed as in core: `checked_op()` returns `Option<T>`, where T is the operation's
value (`Self`, or the target of the `produce` callback), and typekin never
nests it (L42, L43). `with = checked_all` adds a checked form to every other
operation too. Beside it there is exactly one regular form:

- where a trait exists (`a + b`, `-a`, `a += b`, …), the trait impl. No inherent
  method duplicates it, whatever its signature (`add()` is not generated);
- otherwise (`pow()`, `abs()`, `ilog2()`, …), the named method. It keeps its
  own value type (`ilog2()` still returns `u32`); the only change to its output
  is that it may return `Result` (L12).

A regular form's body is the level's checked computation, followed by a
translation to the form's output. One emitter writes both into the form, and no
helper function sits between them (L16). The level also decides the
translation, that is, whether a failure panics or is returned as `Result` (L24,
L25). That is not a profile, and it does not depend on what `checked_op()`
returns (§3.6).

Stage 2: typekin knows the domain, rustc does not. Proofs with formal evidence
remove as many checks as they can. That covers typekin's validation at every
level, and at `strict` and `total` also the prechecks typekin adds in place of
rustc's. Generated code is also rewritten wherever a different shape lets rustc
and LLVM see the domain, so that they remove checks themselves (§4.4).

Blind trust goes. Instead, a friend can be marked as a typekin type with a
generated domain. Construction from it is validated `Make` that skips the
checks this domain already guarantees, and the friend's value is always
trusted to lie inside its domain (§3.7).

## 2. Two kinds of checks

### 2.1 rustc's checks

Probe (throwaway, deleted): stable 1.99.1, `-O`,
`-C overflow-checks=on|off`, `-C debug-assertions=off`, on `i32`/`u32` values
passed through `black_box`, with each panic caught by `catch_unwind`.

| Operation                                                      | `overflow-checks=on`                                | `off`               |
|----------------------------------------------------------------|-----------------------------------------------------|---------------------|
| `-MIN`, `MIN.abs()`                                            | panic "attempt to negate with overflow"             | `MIN`               |
| `MIN * -1`, `MIN - 1`, `MIN.pow(2)`                            | panic                                               | `MIN`, `MAX`, `0`   |
| `0u32 - 1`                                                     | panic                                               | `u32::MAX`          |
| `1i32 << 32`, `1i32 >> 32`                                     | panic "attempt to shift … with overflow"            | `1` (amount masked) |
| `u32::MAX.next_power_of_two()`, `u32::MAX.next_multiple_of(2)` | panic "attempt to add with overflow"                | `0`                 |
| `x / 0`, `x % 0`                                               | panic                                               | panic               |
| `MIN / -1`, `MIN.div_euclid(-1)`                               | panic "attempt to divide with overflow"             | panic               |
| `MIN % -1`, `MIN.rem_euclid(-1)`                               | panic "… remainder with overflow" (although 0 fits) | panic               |
| `MIN.isqrt()`, `MIN.ilog2()`, `0.ilog2()`                      | panic                                               | panic               |
| `7u32.div_ceil(0)`, `7u32.next_multiple_of(0)`                 | panic                                               | panic               |

The following never panicked in either build:

- `MIN.wrapping_div(-1)` = `MIN` and `MIN.wrapping_rem(-1)` = `0`.
- `MIN.wrapping_neg()` = `MIN.wrapping_abs()` = `MIN`.
- `checked_div`, `checked_rem`, `checked_neg` and `checked_abs` return `None`
  at `MIN` with `-1`.
- `overflowing_div` = `(MIN, true)`, `overflowing_rem` = `(0, true)`, and
  `saturating_div` = `MAX`.
- `unsigned_abs`, `abs_diff`, `midpoint` and `signum`.
- `1.wrapping_shl(33)` = `2`.
- `u32 / NonZeroU32` and `u32 % NonZeroU32`.

So there are two classes. **Class B** panics only with `overflow-checks`;
**class A** panics in every build.

**What `MIN` needs.** No special treatment:

- `lax`, `invariant`: rustc catches `MIN / -1` and `MIN % -1` in every build, as
  it catches `/ 0`. `-MIN`, `abs(MIN)` and `MIN * -1` are ordinary class-B
  overflow.
- `strict`, `total`: `checked_div`, `checked_rem` and `checked_{div,rem}_euclid`
  return `None` for `/ 0` and for `MIN / -1` alike. `checked_neg`,
  `checked_abs`, `checked_mul` and `checked_pow` do the same for `MIN`. One
  `checked_*` per operation covers every case.
- `MIN % -1` panics in rustc and is `None` in `checked_rem`, even though the
  mathematical result 0 fits. `strict` and `total` treat it as a failure too,
  consistent with both.
- Wrapping profiles define the `MIN` results (`MIN`, `0`). Only `/ 0` is left.
- `isize::MIN` depends on the pointer width. `checked_*` handles that per
  target, and proofs hold at every width, as today.
- The closure analysis already models these cases exactly (`settle`,
  `src/zz_domain_closure.rs:731-773`).

`checked_*` tests once and then computes without rustc's check. At opt-level 2,
`a + b` and `a.checked_add(b)` both compile to `add; jb` (2026-11-06
observation). At opt-level 0 `checked_*` costs a call.

**Reading the build.** `cfg(overflow_checks)` is unstable (E0658,
rust-lang/rust#111466). `cfg!(debug_assertions)` is stable but independent of
`overflow-checks`; `[profile.dev] overflow-checks = false` is a valid setting.
So `lax` keys on `debug_assertions`, and no level reads `overflow-checks`.

### 2.2 Typekin's checks

| Check                                               | Where today                                                                                                                                                                                                                      |
|-----------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Validation (`domain`, `valid` callbacks) of a value | `_or_panic` "invalid value" (`src/integral_items.rs:1017`), `_result_or_panic` "operation produced an invalid value" (`src/integral_forward.rs:473`), `map` (`integral_items.rs:524`), `try_make`, parsing, Serde, friend `Make` |
| Zero in `NonZero` storage                           | `NonZeroX::new` on the storage path                                                                                                                                                                                              |
| `sequential`'s overflow check                       | `Arithmetic::Checked` (`src/zz_profile_integral.rs:91-104`; `Neg` at `integral_items.rs:806-812`), "operation overflowed"                                                                                                        |
| Proof safety nets                                   | `debug_assert!` in `_proven` (`integral_items.rs:1039`) and in `_within`                                                                                                                                                         |
| Compile time                                        | literal macros (through `try_make`, `src/zz_lit.rs:353`), `Within` and packed constants, `prove = [...]`                                                                                                                         |

### 2.3 Today's emission (audit, `agent://ChecksLevelAudit`)

- Every `+ - * / % << >>`, including the `*Assign` forms, friend right-hand
  sides and facet slots, goes through one emitter, `Arithmetic::make_expr`
  (`src/zz_profile_integral.rs:61-150`). Bitwise operators and `!` cannot panic.
- `Arithmetic` is chosen by profile (`:296-306`): `int` → `Primitive`,
  `circular`/`delta` → `PrimitiveWrapping`, `wrapping` → `WrapperWrapping`
  (`Wrapping` storage), `sequential` → `Checked`. `point` and `nominal` keep the
  previous profile's choice.
    - The choice applies to the whole wrapper: helpers, shifts, `Neg`,
      `core_int` and the proof policy.
    - Only facet operator slots carry their own choice
      (`src/integral_cfg.rs:397-420`).
- `Neg` has its own match (`integral_items.rs:790-822`). `abs` is a `core_int`
  forward, `raw.abs()`, under every profile.
- `core_int` forwards (table at `src/integral_forward.rs:738-897`) validate only
  the result. None of them checks a precondition before the call.
    - Class B: `pow`, `abs`, `next_power_of_two`, `next_multiple_of` (overflow).
    - Class A: `ilog*`, signed `isqrt`, `div_euclid`, `rem_euclid`, `div_ceil`,
      `next_multiple_of` by 0, `saturating_div`,
      `{wrapping,overflowing}_{div,rem}{,_euclid}` by 0, `strict_*`, and the
      nightly `funnel_*`.
- `Step` (`with = range`, `src/integral_impls.rs:153-238`) does not override
  `forward` or `backward`. Core's default
  `forward_checked(..).expect("overflow in Step::forward")` therefore panics in
  every build, while the primitive's own `Step` panics only under class B.
- Integers have no `Sum`/`Product`; those flags are float-only
  (`integral_cfg.rs:1105-1109`).
- bitflag's nested integral always uses `Primitive`
  (`src/bitflag.rs:2093-2097`). The enum's `Shl`/`Shr<usize>` forward to the
  value type's shifts.
- **Bug:** the shift helpers take a `usize` amount and emit
  `checked_shl(rhs as u32)` (`zz_profile_integral.rs:142`). On 64-bit targets,
  `sequential` therefore accepts an amount of 2^32 + k as k. The wrapping forms
  are unaffected, because BITS divides 2^32.
- **Bug:** `without = fn_math_neg` (or `fn_math_add` and the rest) with its
  operator still on expands to a call of a helper that is not generated. rustc,
  not typekin, reports it: E0599 "no method named `_neg` found" (verified with a
  throwaway crate). L16 removes these flags.

## 3. The levels

### 3.1 Who checks what

| Check                                                                             | `lax`                        | `invariant`  | `strict`, `total`          |
|-----------------------------------------------------------------------------------|------------------------------|--------------|----------------------------|
| Validation on infallible paths                                                    | debug builds only            | always       | always                     |
| Validation on fallible paths (`checked_*`, parsing, Serde, literal macros)        | always                       | always       | always                     |
| `sequential`'s overflow check                                                     | debug builds only            | always       | always                     |
| rustc class B (overflow, shift ≥ BITS, `neg`/`abs`/`pow`, `next_power_of_two`)    | rustc, per `overflow-checks` | same         | typekin first, every build |
| rustc class A (`/ 0`, `% 0`, `MIN / -1`, `MIN % -1`, `ilog*` ≤ 0, `isqrt` < 0, …) | rustc, every build           | same         | typekin first, every build |
| Zero in `NonZero` storage                                                         | always                       | always       | always                     |
| Compile time (literal macros, `Within`/packed constants, `prove`)                 | always                       | always       | always                     |
| `_proven`/`_within` safety net                                                    | debug builds                 | debug builds | debug builds               |

### 3.2 Levels and profiles are separate axes

The profile decides what overflow means. The level decides who checks it, and
when.

| Profile                         | Overflow means           | `lax`                                                                    | `invariant`             | `strict`, `total`                         |
|---------------------------------|--------------------------|--------------------------------------------------------------------------|-------------------------|-------------------------------------------|
| `int` (and `point` after it)    | an error, rustc's        | `lhs + rhs`; validation in debug builds                                  | `lhs + rhs`; validation | `checked_add` → failure; validation       |
| `sequential`                    | an error, typekin's      | `if cfg!(debug_assertions) { checked_add → failure } else { lhs + rhs }` | `checked_add` → failure | `checked_add` → failure                   |
| `circular`, `delta`, `wrapping` | wrapping, never an error | `wrapping_add`                                                           | `wrapping_add`          | `wrapping_add`                            |
| `int`, `/`                      |                          | `lhs / rhs`                                                              | `lhs / rhs`             | `checked_div` → failure                   |
| wrapping profiles, `/`          |                          | `wrapping_div`; rustc panics on 0                                        | same                    | `rhs == 0` → failure, else `wrapping_div` |

- At `strict` and `total`, `int` and `sequential` arithmetic emit the same
  checks. The two profiles still differ in their preset flags: `sequential`
  turns off `Mul`, `Div` and the others.
- No level contradicts a profile. The 2026-11-06 idea of profiles as presets of
  the level (A2) is dropped.

### 3.3 Forms of an operation

An operation has two forms for each right-hand-side kind (same type, raw or
friend, facet slot). Both start from the level's checked computation (§3.2,
§3.4), which gives `T` or `Result<T, ()>`. The computation is written into each
body, not into a function of its own (L16):

| Form           | Returns     | Behavior                                                                                                                                       |
|----------------|-------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| Regular form   | §3.6        | The trait impl (`impl Add`) where a trait exists, the named method (`pow()`) otherwise: the computation, then the translation of §3.6.         |
| `checked_op()` | `Option<T>` | Checks every precondition first and validates, at every level, and never panics (R2). Named as in core: `checked_add`, `checked_pow` (L42). |

- **T** is the form's value type. For an operator it is `Self`, the type the
  operator already produces (a bitflag's `Flag | Flag` gives the value type), or
  the `produce` target (`src/zz_conv.rs:45-115`, root and facet slots). A named
  form keeps its own type: `Self` for `pow`, `u32` for `ilog2`, `(Self, bool)`
  for `overflowing_add` (L12).
- **An operation can fail** exactly when its computation still contains a
  typekin check that reports failure; the computation then gives
  `Result<T, ()>`. Checks left to rustc at `lax` and `invariant` are panics
  inside the computation and do not count. Proofs over the API domain count
  (L29, §3.8). `lax`'s guarded checks are assertions and do not count (L40).
- **One regular form (L11).** Where a trait exists, the trait impl is the
  regular form, and no inherent method duplicates it, whatever its signature:
  there is no `add()` beside `impl Add`. Method-call syntax prefers inherent
  methods, so a duplicate would make `a.add(b)` and `a + b` behave differently.
  L34 keeps bitflag's stable `const` set methods as an exception.
- **No nesting.** `checked_op` returns `Option<T>` at every level (R9). At
  `total`, a regular form whose operation can fail returns `Result<T, ()>`
  (L31); `.ok()` gives the checked form's type.
- **Stability.** Whether a checked form exists never depends on a proof (R4).
- **Constness.** Trait impls are `const` only with `konst = nightly`. Today's
  `checked_*` forwards are stable `const fn`s that take `Self`; whether they
  stay so once they dispatch on every right-hand side is Q42.
- **Dispatch.** Operators dispatch on the right-hand-side type through trait
  impls, so a `checked_add` for every right-hand side is a generic
  `checked_add<R>` with a trait per right-hand side (Q42). Facets already
  generate `{Op}Rhs` traits with `fn op(lhs, rhs) -> Self::Output`
  (`src/integral_friend.rs:475-600`). Plan: the `{Op}Rhs` impl for each
  right-hand side holds the body of the regular form and of `checked_op`, and
  `Add<T>` and `AddAssign<T>` forward to it. Non-facet wrappers go through the
  same machinery.
- **Removal** follows the existing convention: each named form is a
  `zz_lib::Forward`, removed with `without = fn_pow` or
  `without = fn_checked_add`.
- **No operation helpers (L16).** Today each operator's body sits in a private
  helper: `_add(&self, raw)`, `_neg`, `_not`, `_{op}_wrapping` and the
  `_proven` siblings (`src/integral_items.rs:85-291`, `:771-822`). Three impls
  per operator (`Self`, raw and friend right-hand sides) and their assignment
  forms call that helper.
    - The helpers only deduplicate. They have validated every call since the
      first commit (`ce1d57a`), so chaining them never deferred validation.
      ADR 0022 records "validate only the final value of a chain" as rejected,
      with `map`/`try_map` covering that need.
    - The per-right-hand-side `{Op}Rhs` bodies now hold the shared code, and
      §3.6 needs separate bodies per right-hand side anyway.
    - The helpers go, and so do the `fn_math_*` flags that switch them (§2.3).

### 3.4 `strict` and `total`: check first, per operation

| Operation                                                                  | Check-first form                                        |
|----------------------------------------------------------------------------|---------------------------------------------------------|
| `+ - *`                                                                    | `checked_*`                                             |
| `/ %` (`int`, `sequential`)                                                | `checked_div`, `checked_rem`                            |
| `/ %` (wrapping profiles)                                                  | `rhs == 0` → failure, else `wrapping_*`                 |
| `<< >>` (`usize` amount)                                                   | `u32::try_from(rhs)`, then `checked_shl`/`checked_shr`  |
| `Neg`, `abs`, `pow`                                                        | `checked_neg`, `checked_abs`, `checked_pow`             |
| signed `isqrt`, `ilog`, `ilog2`, `ilog10`                                  | `checked_isqrt`, `checked_ilog*`                        |
| `div_euclid`, `rem_euclid`                                                 | `checked_*_euclid`                                      |
| `next_power_of_two`, `next_multiple_of`                                    | `checked_next_power_of_two`, `checked_next_multiple_of` |
| `div_ceil`, `saturating_div`, `{wrapping,overflowing}_{div,rem}{,_euclid}` | `rhs == 0` → failure first                              |
| `strict_*`                                                                 | `checked_*` → typekin failure (L39)                     |
| `funnel_shl`, `funnel_shr` (nightly)                                       | `n < BITS` first                                        |
| `Step::forward`, `Step::backward`                                          | override: `forward_checked` → typekin failure           |

These cannot panic: bitwise operators and `!`, `wrapping_*` without division,
`saturating_*` without division, `checked_*`, `overflowing_*` without division,
`midpoint`, `unbounded_sh*`, `rotate_*`, `abs_diff`, `unsigned_abs`,
`carrying_*`, and conversions. Validation still applies to all of them.

Floats: rustc's float arithmetic never panics, so for floats `strict` behaves
like `invariant`, and `total` differs from it only in returning validation
failures as `Result`.

### 3.5 What `lax` guards

- **Guarded (debug builds only):** validation on infallible paths. That covers
  operators, assignment operators, `Neg`, `core_int` forwards, friend
  `Make`/`of`, `make`, `map` and `From`-style conversions. `sequential`'s
  overflow check is guarded too.
- **Never guarded:** every fallible API (`checked_*` forms, `try_make`,
  `try_map`, parsing, Serde, literal macros), zero in `NonZero` storage
  (`NonZeroX::new` is the only safe path), a NaN-free float's NaN rejection
  (L37), and compile-time checks.
- **Emission:** `cfg!(debug_assertions)`, not `#[cfg]`, so both branches
  type-check in every build. Under `lax`, `_or_panic` takes `_proven`'s shape:
  `debug_assert!`, then construct.
- `cfg!` expands in the crate that declares the type, so the guard follows that
  crate's settings. Cargo builds dependencies with the same profile unless
  per-package overrides say otherwise.

In release builds, a `lax` value can lie outside its domain. That is not UB,
because without `unsafe_code = allow` generated code calls no `unsafe` (§4.6,
R11). But anything derived from the domain can then be wrong:

- float `core_float` query constants (`is_normal`, `is_finite`, …);
- closure-proof results.

Two consequences are closed off. A NaN-free float rejects NaN in every build
(L37), so `Eq`, `Ord` and `Hash` stay sound. A `lax` type publishes no domain
(L38), so `Within` targets, friends at `total` and packed windows never rely on
it. Reading a `lax` field of a packed record is Q40.

### 3.6 From the computation to the regular form

This covers every operator trait typekin implements (`Add`, `Sub`, `Mul`, `Div`,
`Rem`, `BitAnd`, `BitOr`, `BitXor`, `Shl`, `Shr`, `Neg`, `Not`, each `*Assign`,
and `Step`) for every right-hand side. It also covers every named form without a
trait, such as the `core_int` forwards `pow`, `abs`, `ilog2`, `rotate_left` and
`wrapping_add`. For those, the translation only decides between their own type
and `Result` of it (L12). At `total` it covers construction too (L33): `make`
and `map` return `Result`, and `From` conversions, whose trait fixes `T`, are
not generated where construction can fail. A friend's `of()` keeps returning
`T` (L36).

```rust
// One impl per right-hand side; `Add<T>` and `AddAssign<T>` forward to it.
impl AddRhs<W> for Rhs {
    type Output = Out;

    #[inline(always)]
    fn add(lhs: W, rhs: Rhs) -> Out {
        // The level's computation (§3.2, §3.4), giving `T` or
        // `Result<T, ()>`; then the translation to `Out`.
    }
}

impl W {
    #[inline(always)]
    pub fn pow(self, exp: u32) -> Out {
        // The computation, then the translation, as above.
    }
}
```

One emitter writes the computation and the translation into each form's body.
`Out` follows from whether the operation can fail and from the `enforcement`
value (L25):

| Operation                      | `lax`, `invariant`, `strict` | `total`                                                                   |
|--------------------------------|------------------------|---------------------------------------------------------------------------|
| cannot fail (gives `T`)        | `T`, no translation    | `T`, no translation                                                       |
| can fail (`Result<T, ()>`)     | `T`, panic on `Err`    | `Result<T, ()>`, no translation; not generated where the trait fixes `T`  |

- **T** is the form's value type (§3.3): only operators can have a type other
  than their natural one, through `produce` or a type they already produce.
- **Traits that fix the output to `T`:** each `*Assign`, which returns `()`, and
  `Step::forward`/`Step::backward`. At `total` they exist only where the
  operation cannot fail (R8).
- No setting asks for `Result` from an operation that cannot fail, so L8's `Ok`
  case does not arise.
- **Not a profile.** Profiles describe numbers: wrapping, a delta, a sequence.
  Panicking is runtime and language behavior (L13). Today `sequential`
  hard-codes a panic ("operation overflowed", §2.3). Under this plan it only
  makes overflow a failure of the computation (R1), and the level decides how
  that failure surfaces.
- The level decides both what the computation checks, and so whether the
  operation can fail, and how a failure surfaces (L24).
- **Per right-hand side.** `_div` for `Percent / Percent` and for
  `Percent / u8` are separate functions with separate return types, so
  `Div<Percent>` and `Div<u8>` can have different `Output`s (L29). Facet slots
  work the same way.
- **Bitflag.** The enum's `Shl`/`Shr<usize>` forward to the value type's
  operators and take their `Output`.
- **Constness.** The translation (`match`, panic) is valid in `const`.
  Constness follows the `konst_*` flags, as today.
- [INFERENCE] The unwrap of a computation that always gives `Ok` folds away at
  opt-level ≥ 1. Phase 2's assembly tests check it.
- **No panics.** At `total`, typekin's generated operations never panic; user
  callbacks are the exception. At `lax` and `invariant`, rustc's own checks
  panic as the build sets them; at `strict`, typekin panics in their place.

Worked example: `struct Foo(u32)` with no domain and no callbacks, profile
`int`:

| Operation            | `lax` and `invariant`                               | `strict`                                       | `total`                              |
|----------------------|-----------------------------------------------------|------------------------------------------------|--------------------------------------|
| `a / b`              | `Foo`; rustc panics on `/ 0` in every build         | `Foo`; typekin panics on `/ 0`                 | `Result<Foo, ()>`; `Err` on `/ 0`    |
| `a + b`              | `Foo`; rustc panics on overflow or wraps, per build | `Foo`; typekin panics on overflow, every build | `Result<Foo, ()>`; `Err` on overflow |
| `a & b`, `!a`        | `Foo`                                               | `Foo`                                          | `Foo`: nothing can fail              |
| `a /= b`, `a += b`   | present                                             | present                                        | absent (R8)                          |
| `a.ilog2()`          | `u32`; rustc panics on 0                            | `u32`; typekin panics on 0                     | `Result<u32, ()>`; `Err` on 0        |
| `a.checked_div(b)`   | `Option<Foo>`; `None` on `/ 0`                      | the same                                       | the same                             |

Without a domain there is nothing to validate, so at `lax` and `invariant` no
operation can fail in typekin's sense. A `Result` appears only at `total`, and
only where typekin itself checks.

**Default API.** The default is `invariant` (L22), so a regular form keeps
returning `T` and panics on failure, as today, and a wrapper behaves like its
raw value (L27, PHILOSOPHY.md). Only `total` changes signatures:

- every regular form whose operation can fail returns `Result<T, ()>`
  (`a + b`, `-a`, `pow()`, `rotate_left()` and the rest);
- `+=` and `Step` disappear where the operation can fail.

README, the examples and the `src/e2e` tests keep their `a + b`.

### 3.7 Friends with a typekin domain, replacing blind trust

Blind trust goes (L17):

- the `unsafe` friend capability in every family: integral, the bitflag value
  type, text, and standalone `friends` and `constructor` with their `Unsafe`
  protocol trait;
- `with_blind_trust` (integral, text) and the `_blind_trust` constructor
  (`src/integral_items.rs:1048-1079`, `src/text.rs:929-951`), with
  `zz_hide::blind_trust_visibility`;
- `examples/my_blind_trust.rs` and its expansion, and the fixtures and tests
  built on blind trust.

The replacement is a per-friend annotation that says "this friend is a typekin
type with a typekin-generated domain" (L18; spelling Q32):

- **A property of the friend, not a capability.** It applies to whatever the
  friend can do: construction (`Make`) and, where Q31 finds a use, the
  right-hand side of `Math` and `Bit`.
- **Construction is domain-aware validated `Make`.** The target skips every
  check the friend's domain already guarantees and validates the rest. Nothing
  has to be contained, and partial overlap is not an error. The exception is
  `total`: there `of()` cannot report a failure, so the friend qualifies only
  when nothing is left to check (L36).
- **The friend's domain is always trusted (L19).** Typekin never checks what
  that domain guarantees, even when user `unsafe` code built an invalid friend
  value: with a friend domain of `1..4`, the target never tests for 0. Two
  limits:
    - the safe storage path still tests zero for `NonZero` storage
      (`NonZeroX::new`). LLVM folds that test where the friend's storage
      tells it the value is not zero (§4.4), and under `unsafe_code = allow`,
      U2 removes it;
    - a `lax` friend publishes no domain (L38), so the annotation rejects it.
- **Where the domain comes from.** Every integral wrapper, bitflag value types
  included, already publishes hidden `__TYPEKIN_WITHIN_V1_*` constants for
  `Within` (ADR 0024).
    - A friend without them (not a typekin type, or `without = within_source`)
      fails to compile at the annotation.
    - A friend whose form is opaque (callbacks, or beyond the form caps) gets
      full validation, apart from what its 8-bit table proves.
- **How checks are skipped** is open (Q31).
- **`Within`** is the case where nothing is left to check, plus the
  requirement that nothing is (Q33).
- **`Make` only where it makes sense (L20).** A capability implies `Make` only
  when the capability can already reach the friend's value through valid
  operations, so that `Make` adds no power. Q30 confirms each row:

  | Capability                           | Today            | Proposal       | Reason                                                                                                                                                                                                                  |
  |--------------------------------------|------------------|----------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
  | root `Math` (integral, float)        | implies `Make`   | keep           | `0 + friend` or `1 * friend` already builds the friend's value; `Make` validates either way                                                                                                                             |
  | root `Bit` (integral, bitflag value) | does not         | imply `Make`   | `0 \| friend`, `0 ^ friend` or `!0 & friend` builds it the same way; a bitflag value's domain always contains 0                                                                                                          |
  | `Rel`                                | does not         | do not imply   | comparing is not producing: a type that compares against the wrapper need not be one it is built from. ADR 0008: "Do not grant operations merely because a conversion exists"                                           |
  | `Within`                             | implies `Make`   | keep           | it only changes how `Make` constructs                                                                                                                                                                                   |
  | facet `Math`/`Bit`                   | do not           | do not imply   | `Make` is wrapper-wide while facets are per operator slot (ADR 0010). Facets serve pairs like a point and its offset, where building a point from an offset is the mistake they prevent                                |
  | bitflag root `Bit` (enum-left)       | does not         | do not imply   | the enum's values are its variants; building the enum from a friend has no meaning                                                                                                                                      |
  | text self-only `Rel`                 | n/a              | no             | the friend is the type itself                                                                                                                                                                                           |
  | the typekin-domain annotation        | n/a              | no             | it is a property of the friend: it changes how `Make` works where a capability provides `Make`                                                                                                                          |

  The identity argument for `Math` and `Bit` needs the identity (0, 1 or all
  ones) to be valid in the target's domain. `Math` has implied `Make` without
  that condition since ADR 0008.
- **Text** loses blind trust without a replacement until Phase 10. A text
  friend with a typekin text domain is part of that phase.

### 3.8 Two domains: the API domain and the accepted domain

A type has two domains (L28): the API domain, written `domain`, and the
accepted domain, written `refine` (L30).

| Domain          | Decides                                                                                                                                                                                                       |
|-----------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| API domain      | the public API: which operations cannot fail (L29), and so their signatures at `total`, whether `+=` and `Step` exist there (R8), and the "Cannot fail" doc line at every value; what other types see (Q38) |
| accepted domain | validation and optimization: every ingress (`try_make`, `make`, parsing, Serde, literal macros), result validation, which runtime checks proofs remove, enum storage's hull, `NonZero` inference, U1's assumptions |

- **Subset rule.** `refine` must be a subset of `domain`, equal or proper.
  Expansion checks it at every pointer width and fails with a counterexample
  otherwise (L30). Without `refine` the two are equal, which is today's single
  `domain`.
- **Callbacks.** `valid` callbacks belong to `domain`, where they block proofs
  as today. A value must pass them in both domains.
- **Migration.** A consumer can narrow the accepted domain to validate more
  strictly and remove more runtime checks without changing a single signature.
  The API changes only when the API domain does.
- **The catch, documented as the consumer's decision.** Values the API domain
  admits but the accepted domain rejects are no longer accepted: `try_make`
  returns `Err` for them, `make` panics, and deserializing them fails.
- **An operation the API declares infallible must stay infallible** under the
  accepted domain. Its `T` signature has no way to report a result that only
  the narrowing rules out; Q37 decides what happens when narrowing breaks this.
- **An operation that can fail in the API, but is proven closed over the
  accepted domain,** keeps its `Result` signature (always `Ok`) and loses its
  runtime check. That is the optimization before the API change.

Worked example: `struct Pct(u8)` at `total`, with `domain = ..=100`:

| `refine`            | `a \| b`                                                       | `a & b`                                                  |
|---------------------|----------------------------------------------------------------|----------------------------------------------------------|
| none (`..=100`)     | `Result<Pct, ()>`: results reach `127`, so it can fail         | `Pct`: at most `min(a, b)`, proven infallible            |
| `..=63`             | `Result<Pct, ()>`, always `Ok`, no runtime check: closed       | `Pct`                                                    |
| `1..=100`           | `Result<Pct, ()>`                                              | breaks: `1 & 2 = 0` is outside `1..=100` (Q37)           |

Changing the API domain to `..=63` later is the public API change that makes
`a | b` return `Pct`.

## 4. Stage 2: removing checks

### 4.1 What a proof can remove

All obligations must hold at every pointer width, for valid operands.

| Check                           | Proof obligation                                                 | Emitted once proven                                                                                | Today                                    |
|---------------------------------|------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|------------------------------------------|
| Result validation               | closure (`ResolvedDomain::closure`)                              | `_proven`                                                                                          | yes; on by default for `u8`/`i8`         |
| Overflow precheck               | the hull of exact results fits the type                          | `wrapping_*`: same value on the proven range, no rustc check in any build                          | no                                       |
| `/ 0`, `% 0` precheck           | the divisor's domain excludes 0 (same-type right-hand side only) | unsigned `NonZero` storage: `lhs / rhs_nonzero` (`u32: Div<NonZeroU32>`); otherwise `wrapping_div` | no                                       |
| `MIN / -1`, `MIN % -1` precheck | lhs excludes `MIN`, or rhs excludes `-1`                         | as the row above                                                                                   | no                                       |
| `-x`, `abs(x)` overflow         | the domain excludes `MIN`                                        | `wrapping_neg`, `wrapping_abs`                                                                     | structural proofs remove validation only |
| `ilog*`, `isqrt` argument       | domain ⊆ `1..` (`ilog*`) or `0..` (`isqrt`)                      | the plain call                                                                                     | no                                       |
| `next_power_of_two` overflow    | maximum ≤ 2^(BITS-1)                                             | the plain call                                                                                     | no                                       |
| Shift amount, `pow` exponent    | the right-hand side is a raw `usize`/`u32`: never provable       | n/a                                                                                                | n/a                                      |

- Where the emitted form keeps rustc's own class-A test (signed `/`, `ilog*`,
  `isqrt`), that test can no longer fire. LLVM removes it only when the storage
  type tells it why (§4.4):
    - `NonZero` storage removes the zero test;
    - the signed `MIN / -1` test goes only when the storage excludes `MIN` from
      the dividend or `-1` from the divisor.
- Raw and friend right-hand sides only have `Rhs::Full`, as today: a friend's
  domain is not visible at expansion.
- At `invariant`, rustc's checks stay as written even when no overflow is proven
  (Q13).

### 4.2 Evidence

Proofs need formal evidence; tests serve only as regression checks (D9). Q10
chooses the method.

- The new obligations in rows 3-7 of §4.1 are membership and sign tests on
  intervals, far simpler than the hull rules.
- Row 2 reuses the hull rules (`corners`, `rem_hull`, the bit hulls, `unary`,
  `settle`).
- The analysis computes on exact integers: `Int` is a sign plus a `u128`
  (`src/zz_values.rs:441`), and type bounds enter only through `Ty`.
- Tools on this machine: z3, cvc5, bitwuzla and Kani are not installed.

### 4.3 Budget

Cost depends on interval count, not value count (§8). Proposal:

- Proofs are on by default at every width when each operand's domain has at most
  8 intervals at every width.
- `with = proven` goes up to `MAX_INTERVALS = 16`.
- `without = proven` turns proofs off; `prove = [...]` implies them.
- Memoize closure questions per type only if measurements justify it.

### 4.4 Storage that tells rustc and LLVM the domain

Typekin can also rewrite generated code so that rustc and LLVM see part of the
domain, and LLVM then removes checks by itself. That includes rustc's own
class-A tests, which no safe code can remove otherwise (§4.1), and bounds
checks in the user's code, which typekin's proofs never reach.

Without `unsafe` (`assert_unchecked`, `unreachable_unchecked`, `new_unchecked`;
§4.5 covers the opt-in), there are two channels:

- **The storage type's validity range.** rustc passes it to LLVM as `range` on
  parameters and `!range` on loads, so it survives calls and memory.
- **A branch that dominates the use inside one inlined body**, such as a
  validation that has just run. This already happens today, and the fact is
  lost at the next store or call.

So the knowledge has to live in the storage type. It is chosen where `NonZero`
storage is chosen today (`Cfg::requires_non_zero_storage`,
`src/integral_cfg.rs:164-174`; emission in `src/integral.rs:654-852`).

Probe (throwaway, deleted): stable 1.99.1, x86-64, `#[no_mangle]` functions,
assembly read at opt-levels 1 and 2, which gave the same results:

| Storage                                                       | Code                                                     | Result                                       |
|---------------------------------------------------------------|----------------------------------------------------------|----------------------------------------------|
| enum `#[repr(u8)]`, discriminants `1..=100`, passed by value  | `1 <= raw && raw <= 100`                                 | folded to `true`                             |
| same, loaded from `&[Pct]`                                    | same                                                     | folded; only the slice bounds check remains  |
| same                                                          | `a + b <= 200` (as `u16`), `a / b <= 100`                | folded to `true`                             |
| same                                                          | `x / raw`                                                | no zero test (`divb` alone)                  |
| same                                                          | `[u8; 101][raw]` in user code                            | no bounds check                              |
| same                                                          | safe construction `match raw { 1 => V1, …, _ => None }`  | `lea; cmp; cmov`: one range test             |
| enum `#[repr(u32)]`, `1..=34`                                 | `x / raw`                                                | no zero test; `size_of` stays 4              |
| enum `#[repr(i8)]`, `-127..=0`                                | `raw / b` after testing `b != 0`                          | the `MIN / -1` test is gone                  |
| `NonZeroU32`, `NonZeroI32`                                    | `x / b.get()`                                            | no zero test; signed keeps `MIN / -1`        |
| `NonZeroI32` holding `raw ^ MIN`, divisor `NonZeroI32`        | `raw / b`                                                | no test at all                               |
| `NonZeroU32` holding `raw - 1000`                             | `raw != 1000`                                            | folded to `true`                             |
| `u8` field widened to `u32`                                   | `a + b <= 510`                                           | folded to `true`                             |
| plain `u32`                                                   | `a / b`                                                  | zero test and panic call remain              |

`size_of::<Option<Pct>>()` is 1: the values outside the enum become niches.

Compile cost of enum storage, measured with rustc only (no typekin expansion).
Each of 20 types has `make`, `raw` and one use; the enum derives only `Clone`
and `Copy`. Extra time per type over plain storage:

| Values in the enum | opt-level 0 | opt-level 2 |
|--------------------|-------------|-------------|
| 256                | 7.4 ms      | 7.0 ms      |
| 1 024              | 32 ms       | 45 ms       |
| 4 096              | 172 ms      | 315 ms      |

For comparison, typekin's expansion of one type takes about 6 ms (§8).
Deriving `Debug`, `Hash`, `Ord` and the like on the enum roughly doubled the
4 096 row, so traits go through `raw()` instead.

Encodings:

| Encoding                                                                    | LLVM learns                        | Cost                                                  | Layout                                                    |
|-----------------------------------------------------------------------------|------------------------------------|-------------------------------------------------------|-----------------------------------------------------------|
| Enum, `#[repr(raw)]`, one variant per value of the hull `[lo, hi]`          | the exact hull                     | compile time grows with the hull; reads free (`as`)   | size, alignment and scalar ABI unchanged; niches appear   |
| `NonZero` (today)                                                           | `raw != 0`                         | none                                                  | unchanged                                                 |
| `NonZero` holding `raw ^ k`                                                 | `raw != k`                         | one `xor` per read and write, often folded            | unchanged                                                 |
| Narrower primitive field (`u8` for a `u32` domain in `0..=255`)             | the narrow type's range            | none                                                  | **`size_of` changes**                                     |

Rules if adopted:

- **One storage type per wrapper.** Proposed precedence: enum, which subsumes
  `NonZero` when the hull excludes 0; then `NonZero`; then `Wrapping`.
  Wrapping arithmetic works on the raw value either way.
- **The enum covers every value of the hull, holes included.**
    - Holes stay validation's job.
    - A value outside the hull cannot be stored at all. That test runs on every
      construction path, domain-aware friends included, and folds where the
      friend's domain implies it. It is never guarded (as for zero in `NonZero`
      storage under `lax`, §3.5).
- **Same hull at every pointer width**, or no enum storage.
- **Existing rewritten-storage rules carry over:** a private field, no
  `AsRef<raw>`, and `raw()` returns the primitive. Traits on the wrapper go
  through `raw()`; the enum derives only `Clone` and `Copy`.
- **`repr` support.** `#[repr(u128)]`, `#[repr(i128)]` and `#[repr(usize)]`
  enums compile on stable 1.99.1. Check against `rust-version` 1.97.
- **Catch.** User `unsafe` code that transmutes an out-of-hull primitive into
  the wrapper (FFI, `bytemuck`, `transmute`) becomes UB; today it is only an
  invalid value. `Option<Wrapper>` can shrink. Both are observable layout facts
  and need documenting.
- **Limits.** LLVM learns one interval plus single excluded values, and
  nothing at opt-level 0. Holes, `mul`, callbacks and closure under an
  operation remain typekin's proofs. The API (L29) never depends on what LLVM
  folds.

### 4.5 Opt-in `unsafe` (`unsafe_code = allow`)

This is off by default (Q26). With it, typekin emits `unsafe` code wherever
`unsafe` buys an optimization that no safe form gives. Each use rests on an
invariant that holds in every build: either a validation that has run, or a
proof with formal evidence (Phase 5). `lax` cannot be combined with it (R11).

Probe (throwaway, deleted): stable 1.99.1 unless noted, x86-64, opt-level 2:

| Code                                                                                                       | Safe form                                     | With `unsafe`                                                        |
|------------------------------------------------------------------------------------------------------------|-----------------------------------------------|----------------------------------------------------------------------|
| `u32` domain `1..=1_000_000` (too wide for enum storage): `a / b <= 1_000_000`, `a.checked_add(b).is_some()` | LLVM cannot know it (plain `u32`: `not; cmp; setbe`) | folded to `true`, with `assert_unchecked` on each `raw()` read |
| `x / b`, `b` from that domain                                                                              | zero test and panic                           | `divl` alone                                                         |
| `NonZero` result of `a * 4`, `a` proven in `1..=1000`                                                      | `shl; test; je` and a panic path              | `lea` alone (`new_unchecked`, or the assumption on `a`)              |
| `a.unchecked_add(1) > a`, no overflow proven                                                               | `wrapping_add`: `cmp; setne`                  | folded to `true` (no signed wrap)                                    |
| signed `a / b`, `a != MIN` and `b != 0` proven                                                             | two tests and two panic paths                 | `cltd; idivl` alone                                                  |
| `x % 2 == 0` with `x % 4 == 0` assumed (`mul(4)`)                                                          | n/a                                           | folded to `true`                                                     |
| `x != 3` with `x % 3 != 0` assumed                                                                         | n/a                                           | not folded: LLVM ignores this fact                                   |
| `unchecked_shl(n)` with `n < 32`                                                                           | n/a                                           | `shll` alone; compiles on stable                                     |
| nightly 1.102 `pattern_type!(u32 is 1..=100)`                                                              | n/a                                           | range folded at parameters; `Option<P>` is 4 bytes; building and reading it needs `transmute` |

Forms:

| Form                                                                                                   | Where                                                              | Replaces                                                                                        |
|--------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------|-------------------------------------------------------------------------------------------------|
| U1 `assert_unchecked(fact)` on every `raw()`/`get_raw` read; pure facts only: the hull, single excluded values, `mul(2^k)` | every validated integer wrapper                    | enum storage's role for LLVM (§4.4), at any hull size; enum storage remains only for niches     |
| U2 `NonZero::new_unchecked`                                                                            | construction after validation, or of a proven result               | the safe constructor's zero test (`src/integral.rs:792-822`)                                    |
| U3 `unchecked_add`/`sub`/`mul`/`shl`/`shr`                                                             | operations proven free of overflow, shift amounts proven < BITS    | `wrapping_*`; adds no-wrap facts                                                                |
| U4 `assert_unchecked` before signed `/` and `%`                                                        | divisor proven ≠ 0 and `(MIN, -1)` proven impossible               | rustc's class-A tests, which no safe form removes (§4.1)                                        |
| U5 `transmute` from the raw value                                                                      | enum storage construction after validation                         | the `match`                                                                                     |
| U6 pattern-type storage                                                                                | `nightly` Cargo feature only; an incomplete internal rustc feature | enum storage, at any hull size                                                                  |

- **U1 assumes only pure, cheap facts, never callbacks.** An assumed
  expression may not be removed when it contains a call.
- **Debug builds still check.** The `debug_assert!` in `_proven` and `_within`
  stays, and each U form gets a `debug_assert!` of its fact, so debug builds
  panic before reaching UB. [INFERENCE] std's own precondition checks on
  `assert_unchecked`, `new_unchecked` and `unchecked_*` also fire when debug
  assertions are on.
- **ADR 0003's objections** to `assert_unchecked` were `unsafe` on every
  access, possible pessimization of unrelated code, and correctness invisible
  to typekin's tests. The answers here:
    - the setting is opt-in;
    - the assembly tests compare it against the safe output, so a
      pessimization shows up;
    - Miri runs over it (Phase 8).

### 4.6 Undefined behavior: today and with the opt-in

Today generated code is safe, and no typekin path is UB. An invalid state ends
one of these ways:

| Source of an invalid value                                              | Today                                                                                                                                                                       | With `unsafe_code = allow`                                    |
|-------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------|
| Wrong closure, precheck or `Within` proof (an analyzer bug)             | debug builds: the `debug_assert!` panics. Release: the invalid value is stored silently; zero into `NonZero` storage panics ("zero cannot be stored by this integral type") | UB in release; debug builds still panic first                 |
| A friend's value outside its declared domain (possible only through user `unsafe`), taken through a domain-aware friendship | trusted (L19): the target may store an invalid value; zero into `NonZero` storage still panics                                                                              | UB, at construction (U2, U5) or at the first read (U1)        |
| `lax` in release (planned, §3.5)                                        | the invalid value is stored; zero into `NonZero` storage panics                                                                                                              | not allowed: `lax` with the opt-in is an error (R11)          |
| User `unsafe` (`transmute`, FFI, `bytemuck`) of an invalid raw          | primitive storage: an invalid but defined value. `NonZero` storage: zero is already UB (Rust's validity invariant). Enum storage (§4.4): any value outside the hull is UB | UB for any invalid value, at the first read (U1)              |

- **Correction.** ADR 0003 ("Rejected alternatives") says that a wrong proof
  with `NonZero` storage "reaches `new_unchecked`, i.e. UB". That is stale.
  `storage_expression` (`src/integral.rs:792-822`) builds `NonZero` storage
  with `NonZeroX::new` and panics on zero, so today a wrong proof panics there.
  Phase 9 fixes the ADR.
- **Zero in `NonZero` storage today** is UB only through user `unsafe` code;
  every typekin path panics instead. With the opt-in, typekin's own paths can
  reach UB, as the table lists.

## 5. Decisions

From the user, 2026-11-08:

- **L1.** Levels: `lax`, `invariant` (the user's "auto"), and a strict end, the
  user's "pedantic", which L25 splits into `strict` and `total`.
- **L2.** The middle level leaves rustc's checks to rustc and always runs
  typekin's checks.
- **L3.** `lax` treats rustc's checks like the middle level and guards typekin's
  checks with `debug_assertions`.
- **L4.** `strict` and `total` (the user's "pedantic") always run typekin's
  checks, assume rustc checks nothing (overflow and `/ 0` included), and do
  those checks themselves.
- **L5.** Every operation has a regular form; every operation that can fail
  also has a checked form (L42, L43), which does not nest `Option`/`Result`.
  (Its last clause, "where the regular form returns a `Result`, both forms have
  the same signature", is superseded by L42: the checked form returns
  `Option`, as in core.)
- **L6.** Stage 2 removes as many checks as possible, with evidence and closure
  proofs: typekin's checks, and at `strict` and `total` also the stand-ins for
  rustc's.
- **L7.** A regular form's body is the level's computation followed by a
  translation to the form's output. The translation is kept separate from
  everything else. (L16: no helper function sits between them.)
- **L8.** The translation works as follows:
    - same type on both sides: nothing to translate;
    - `T` to `Result<T, ()>`: wrap in `Ok`;
    - `Result<T, ()>` to `T`: no impl by default; panic on `Err` only when the
      user asks for it. L22 supersedes the default: at `invariant` a failing
      operation panics, and `Result` comes only at `total`.
- **L9.** Whether a regular form returns an error or panics does not depend on
  what `checked_op()` returns. (Its first half, "does not depend on the
  level", is superseded by L24: the level now implies it.)
- **L10.** Generated code is rewritten wherever a different shape shares the
  domain with rustc and, through it, LLVM, so that more optimizations apply.
- **L11.** No inherent method duplicates a trait impl, with or without a
  different signature: `add()` is dropped, leaving `impl Add` and
  `checked_add()`.
- **L12.** A named form without a trait (`pow()`, `ilog2()`) keeps its own
  value type: `Self` for `pow`, `u32` for `ilog2`, `(Self, bool)` for
  `overflowing_add`. The only change to its output is the failure translation
  of §3.6, `T` or `Result<T, ()>`. It never takes an arbitrary output type:
  that belongs to operators, through `produce` or a type the operator already
  produces (a bitflag's `Flag | Flag` gives the value type).
- **L13.** There is no free-form output-type setting. Whether panics are
  allowed comes from the level (L24). It is not a profile: profiles describe
  numerical behavior (wrapping, a delta, a sequence), while panicking is
  runtime and language behavior.
- **L14.** An off-by-default setting lets typekin emit `unsafe` code where it
  gives faster code (§4.5).
- **L15.** The plan documents where undefined behavior can arise, today and
  with that setting (§4.6).
- **L16.** There are no operation helper functions. Today's private helpers only
  deduplicate a body that several impls share; chaining them never deferred
  validation. Each form carries its own body, shared per right-hand side
  through the `{Op}Rhs` dispatch.
- **L17.** Blind trust goes: the `unsafe` friend capability, `with_blind_trust`
  and `_blind_trust`, in every family.
- **L18.** It is replaced by a per-friend annotation saying the friend is a
  typekin type with a generated domain. Construction from such a friend is
  validated `Make` that may skip the checks the friend's domain already
  guarantees.
- **L19.** A friend's value is always trusted to lie inside the friend's
  declared domain. Typekin never checks it, even when user `unsafe` code
  produced an invalid value: with a friend domain of `1..4`, typekin never
  assumes or tests for 0.
- **L20.** A capability implies `Make` only where that makes sense, decided per
  capability, never as a blanket rule. `Math` qualifies, because
  `0 + friend` already constructs.
- **L21.** The key is `enforcement`, with the values `lax`, `invariant`, `strict`
  and `total`. `invariant` adds over `lax` that the domain invariant is checked
  in every build, and nothing beyond.
- **L22.** The default is `invariant`. Otherwise `a + b` is too painful and no
  one will use the library: a wrapped value must superficially behave as its
  raw value behaves in the language (L27). This reverses L8's default.
- **L23.** Scope of `enforcement`:
    - floats accept all four values. rustc's float arithmetic never panics, so
      `strict` acts as `invariant`, and `total` differs only in returning
      validation failures as `Result` (§3.4);
    - bitflag accepts the key at the root and forwards it to its nested value
      type, which rejects its own `enforcement`, as with `konst`;
    - text takes the same four values as integral. Their meanings for text are
      defined in Phase 10, where some values may end up behaving alike. Until
      then text rejects the key rather than ignoring it.
    - packed accepts `enforcement` and `unsafe_code` (Q26) and ignores both, so
      its behavior stays the same. A packed record has no operators or
      forwards for these keys to act on; `try_make` and Serde always validate,
      and `make` and `with_*` build from fields that are valid by
      construction. This is a deliberate exception to AGENTS.md's rule that
      unsupported configuration gets a targeted diagnostic, and Phase 9
      records it there.
- **L24.** Panicking is not configured separately: the `enforcement` value
  implies it. Checking and failure handling are too interleaved to be set in
  two places.
- **L25.** The four values, as this table states them. Phase 0 copies the table
  verbatim into ADR 0028, and Phase 9 copies it verbatim into the docs:

  | Value       | rustc's preconditions (overflow, `/ 0`, …) | typekin's validation | On failure             | Same in every build |
  |-------------|--------------------------------------------|----------------------|------------------------|---------------------|
  | `lax`       | rustc, per build                           | debug builds only    | panic                  | no                  |
  | `invariant` | rustc, per build                           | always               | panic                  | no                  |
  | `strict`    | typekin, before the operation              | always               | panic                  | yes                 |
  | `total`     | typekin, before the operation              | always               | `Result`, never panics | yes                 |

    - `strict` matches core's `strict_*` methods, which panic in every build.
    - At `total`, a regular form that can fail returns `Result<T, ()>`, and
      forms whose trait requires `T` (`+=`, `Step`) are dropped where the
      operation can fail (R8).
    - There is no value for typekin's failures as `Result` while rustc's `/ 0`
      and overflow still panic: it would promise no panics and still panic.
    - checked forms are the same at every value (R2). `produce` stays the value
      conversion, and `unsafe_code` stays a separate key.
- **L26.** The docs stop calling `prove = [...]` a contract and call it a
  checked requirement. This was decided while the key was briefly named
  `contract`, and stays after its rename to `enforcement`. PHILOSOPHY.md
  already does; README, ADR 0003 and `examples/my_proven.rs` follow (Phase 0
  for the ADR, Phase 9 for the rest).
- **L27.** By default, a wrapped value behaves on the surface as its raw value
  behaves in the language (PHILOSOPHY.md, "Wrapped Values Behave Like Their Raw
  Values by Default").
- **L28.** A type has two domains (§3.8):
    - the API domain decides the public API, that is, which operations are
      infallible;
    - the accepted domain decides validation and optimizations. It is a subset
      of the API domain, never a proper superset, and equal to it by default.

  Narrowing the accepted domain lets consumers optimize before they change
  the public API. That it can reject values that used to be accepted is the
  consumer's decision, not typekin's, and the docs say so.
- **L29.** A proof makes an operation infallible: an operation cannot fail in
  the API when a proof over the API domain shows it, including its prechecks
  at `strict` and `total`. This settles Q16. `prove = [...]` stays a checked
  requirement over the same proofs.
- **L30.** The API domain is `domain`, and the accepted domain is
  `refine = <domain expression>`. Expansion checks that `refine` is a subset of
  `domain`, equal or proper, at every pointer width, and fails with a
  counterexample otherwise. `valid` callbacks belong to `domain`, not `refine`.
  Const-callable callbacks in `refine` are future work (§13).
- **L31.** The error type of regular forms that can fail is `()`, as in
  `Result<T, ()>`, following today's `error = ()`. Checked forms return
  `Option` (L42). An error enum whose variants may carry a value is future work
  (§13).
- **L32.** Every runtime panic in generated code has one of exactly two
  messages:
    - "precondition failed", when a check before the operation fails: a
      precheck, or the validation of an input;
    - "postcondition failed", when the validation of a computed result fails.

  This includes the `debug_assert!`s in `_proven` and `_within`. The messages
  are constant strings, never formatted and never carrying the bad value, so
  the panic path stays small in assembly. The generator fixes one per panic
  site. This is a hard requirement. rustc's own panics, left to rustc at `lax`
  and `invariant`, keep rustc's messages. The final wording and an opt-in
  formatting callback are future work (§13).
- **L33.** `total` covers construction too, in this plan: `make` and `map`
  return `Result`, and `From` conversions, whose trait fixes `T`, are not
  generated where construction can fail. A friend's `of()` is the exception
  (L36).
- **L34.** Inherent methods that shadow a trait method go: the float `min`,
  `max` and `clamp` forwards wherever `Ord` exists. Bitflag's `union`,
  `intersection` and `symmetric_difference` stay as an exception to L11,
  because they are existing stable `const fn`s and trait methods cannot be on
  stable.
- **L35.** `_eq` and `_cmp`, with their flags `fn_op_eq` and `fn_op_cmp`, go
  together with the operation helpers (L16).
- **L36.** A friend's `of()` returns `T` at every `enforcement` value, never
  `Result`, so friendship stays an infallible contract (PHILOSOPHY.md). At
  `total`, where it may not panic either, the problem moves to the friend:
  `of()` accepts only a friend whose domain is known and proven to fit the
  accepted domain.
    - It accepts a typekin friend whose published domain is a subset, through
      the `Within` proof.
    - It accepts a primitive friend whose whole range fits, decided at
      expansion. The backing primitive `#el` fits only when the type has no
      domain.
    - Every other `Make` friend is rejected at `total`, by a diagnostic at the
      friend's declaration or by the failing `Within` proof. Construction from
      it goes through `try_make` or `TryFrom`.
    - At `lax`, `invariant` and `strict`, `of()` validates as today and panics
      with "precondition failed".
    - Checking the domain of other friends is future work (§13).
- **L37.** Under `lax`, a NaN-free float rejects NaN in every build; only the
  rest of its domain is guarded. `Eq`, `Ord` and `Hash` are generated on that
  promise, and `Ord`'s `partial_cmp().unwrap()` would panic on NaN.
- **L38.** A `lax` type publishes no `Within` form and no 8-bit table; its
  constants give `lax` as the reason. A `lax` domain does not hold in release
  builds, so publishing it would spread `lax`'s risk to targets that are not
  `lax`. Hence these fail to compile:
    - `Within` from a `lax` source;
    - `of()` from a `lax` friend at `total` (L36);
    - the typekin-domain annotation on a `lax` friend (§3.7);
    - a packed window that holds a `lax` field and is narrower than the field's
      raw type.
- **L39.** At `strict` and `total`, the `strict_*` forwards become `checked_*`
  with a typekin failure. Turning rustc's panics into typekin failures is what
  those levels are for.
- **L40.** `lax`'s debug-only checks are assertions, not failures. They panic
  in debug builds and vanish in release, so at `lax` the regular forms keep
  returning `T`, like the `debug_assert!` in `_proven`.
- **L41.** A `produce` target that is itself `Option` or `Result` is an
  ordinary value type. Typekin does not flatten it, so with
  `produce = Some(_) -> Option<Pct>`, `checked_add` returns
  `Option<Option<Pct>>`. It is the user's own type.
- **L42.** The fallible form of an operation is named and typed as in core:
  `checked_op()` returns `Option<T>`. There is no `try_op`, so typekin adds no
  convention of its own.
    - Today's `checked_*` forwards (`checked_add`, `checked_pow`,
      `checked_ilog2`, …) already are these forms, validating and returning
      `None` on an invalid result, and they stay.
    - Operations without a core counterpart follow the same pattern:
      `checked_bitor`, `checked_not`, `checked_wrapping_div`.
    - An operator's checked form exists with the operator, no longer with
      `core_int`.
    - `strict_*` get none: `checked_*` already is their checked form.
    - At `total`, a fallible `a + b` returns `Result<T, ()>` (L31) and
      `a.checked_add(b)` returns `Option<T>`. This supersedes L5's last
      clause.
    - Construction and conversion keep std's `try_` convention: `try_make`,
      `try_map`, `try_into_*` and `TryFrom`.
- **L43.** Which operations get a checked form:
    - by default, every operation that can fail before proofs: operators,
      `Neg`, and the `core_int` forwards that can panic;
    - `with = checked_all` (spelling to confirm) adds one to every other
      operation too, infallible ones included;
    - never: assignment operators, and forms that already return `Option`,
      `Result` or a tuple (`checked_*`, `overflowing_*`).

Carried over from 2026-11-06, not revisited:

- **D5.** Compile-time checks always run in full.
- **D6.** The `debug_assert!` in `_proven` stays.
- **D7.** `NonZero` storage stays the default.
- **D9.** Proof evidence must be formal.
- Generated code calls no `unsafe`.

## 6. Proposed rules (derived; confirm)

- **R1.** Levels and profiles are independent (§3.2). `sequential`'s overflow
  check is a typekin check, so `lax` guards it.
- **R2.** Checked forms run every check at every level and are never guarded.
  The same holds for every fallible ingress: `try_make`, parsing, Serde and
  literal macros.
- **R3.** `enforcement` is set once per wrapper, like `konst`, `non_zero` and
  `precedes`, not per facet: validation is a single function per wrapper.
- **R4.** Whether a checked form exists is decided from the configuration,
  before proofs.
- **R5.** `MIN` needs no code of its own (§2.1).
- **R6.** Fix the `sequential` shift truncation (§2.3) on its own, first, with a
  regression test.
- **R7.** Proof policy per (arithmetic, level): `sequential` → `Checked`;
  `int` at `strict` and `total` → `Checked`; `int` at `lax`/`invariant` →
  `Wrapping`, which covers both builds; wrapping profiles → `Wrapping`. `lax`
  uses `invariant`'s policy, because its validation only runs in debug builds,
  where `sequential`'s check is on as well.
- **R8.** Traits that fix the output to `T` (`*Assign`, `Step::forward`,
  `Step::backward`) are generated only where the operation cannot fail or
  panics are allowed (§3.6).
- **R9.** `checked_op` returns `Option<T>`, with T the value type, at every
  `enforcement` value, and typekin never nests it (L42).
- **R10.** Without `unsafe`, the storage type's validity range is the only
  channel that survives calls and memory (§4.4). Typekin's knowledge reaches
  LLVM through the choice of storage.
- **R11.** `unsafe_code = allow` uses `unsafe` only where an invariant holds in
  every build: a validation that has run, or a proof with formal evidence.
  Hence:
    - `lax` with it is an error;
    - a domain-aware friend's value outside its domain (possible only through
      user `unsafe`, L19) becomes UB in the target;
    - the U forms wait for Phase 5's evidence table.

## 7. Open questions

Needed before Phase 3 (levels):

- **Q40. Reading a `lax` field of a packed record.** Packed getters re-check
  the stored bits through the field's `try_make`, which is never guarded (R2),
  and panic when it fails ("typekin packed field invariant violated" today,
  `src/packed.rs:951`). `make` and `with_*` store a `lax` field's value as
  given, and in release builds that value can lie outside its domain. Options:
    - read a `lax` field through its `make`, whose check `lax` guards, so in
      release the stored value comes back unchanged, like its raw value
      (recommended);
    - keep `try_make`: an out-of-domain value written in a release build
      panics when it is read.

Needed before Phase 4 (checked forms):

- **Q42. Right-hand sides and constness of `checked_op`.** Today's `checked_*`
  forwards take `Self` (or a `u32` amount or exponent) and are stable
  `const fn`s under the default `konst = stable`. An operator dispatches on its
  right-hand side (raw, friends, facet slots) through traits, and calling a
  trait method is `const` only on nightly (`konst = nightly`). One method name
  cannot be both. Options:
    - `checked_op<R>` takes every right-hand side its operator takes, through
      `{Op}Rhs`, and is `const` exactly when its operator is (recommended:
      every right-hand side gets its checked form under core's name, as L5
      asks). Stable `const` code loses today's `checked_*`;
    - `checked_op` keeps core's signature and stays a stable `const fn`. Raw,
      friend and facet right-hand sides get no checked form, which narrows L5.

Needed before Phase 4b (friends):

- **Q30. Which capabilities imply `Make`.** Confirm the per-capability
  proposals in §3.7's table: `Math` and `Within` keep it, `Bit` gains it, and
  `Rel`, facet friends and the bitflag root do not. One choice is open:
    - `Math` and `Bit` imply `Make` unconditionally (recommended). This
      matches today's `Math`, and the API does not depend on the domain;
    - they imply it only when the target's domain admits the identity (0 for
      `+`, `|` and `^`; 1 for `*`; all ones for `&`), decided at expansion.
      This is exact, but the API then changes with the domain: `Math` on
      `2..=100` would lose `Make`.
- **Q31. How the target skips checks.** Options:
    - per check (recommended):
        - the target's validation splits into its top-level conjuncts: domain
          atoms and each callback;
        - for each conjunct, rustc computes a constant from the friend's
          published form, using the `Within` subset check, that says whether
          the friend's domain implies it;
        - each conjunct is emitted as `IMPLIED || check(raw)`, which rustc
          folds;
        - the 8-bit table gives exact per-conjunct answers for `u8`/`i8`
          friends;
        - Phase 5 then needs evidence for the subset check and the forms too;
    - storage only: no typekin analysis. LLVM folds what the friend's storage
      tells it (§4.4), and under the opt-in, what U1 tells it;
    - all or nothing: skip validation when `Within` would succeed, otherwise
      validate fully.
- **Q32. Spelling of the annotation.** Options:
    - a marker on the friend declaration, outside the capability list, such as
      `friend = typekin _(Percent) -> Math` (recommended: it describes the
      friend, not what the friend may do);
    - a capability, such as `_(Percent) -> [Math, Domain]`.

  Also: reject the annotation on primitive friends, whose domain is the whole
  type (recommended), or accept it as a no-op.
- **Q33. `Within` and the annotation.** Options:
    - `Within` becomes the annotation plus the requirement that nothing is left
      to check, and it fails to compile otherwise, as today (recommended);
    - drop `Within`. Domain-aware construction with nothing left to check
      covers its runtime behavior, but the compile-time guarantee goes.

Needed before Phases 5-10:

- **Q10. Evidence method.** Options:
    - Kani on the real hull and `settle` code, per width, with one or two
      symbolic intervals, plus a written lemma lifting it to n intervals; SMT
      or a written proof where Kani cannot finish (likely 128-bit `*`, `/`,
      `%`). Recommended; it starts with a spike.
    - SMT only (z3/cvc5): symbolic bounds over the integers, bit-vectors per
      width for `& | ^`.
    - Written proofs only.
- **Q11. Budget.** Options:
    - default ≤ 8 intervals, `with = proven` up to a hard cap of 16
      (recommended; at worst about 3× the expansion without proofs);
    - lift the cap on opt-in (about 16× the cost at 64 intervals).
- **Q12. README.** May Phase 9 edit README.md? Four changes are involved:
    - "The Catch" (`README.md:191-250`): its `cfg(not(debug_assertions))`
      validator recipe becomes `enforcement = lax`, and its proof default
      changes;
    - the table of L25, verbatim;
    - `prove` no longer called a contract (`README.md:232`, L26);
    - the blind-trust (`unsafe` friend) material.
- **Q13. `invariant` with no overflow proven.** Options:
    - keep `lhs + rhs` (recommended; L2 leaves rustc's checks to rustc);
    - emit `wrapping_add`, which drops rustc's dead check in debug builds.
- **Q21. Enum storage.** Options:
    - default when the hull has at most 256 values at every width and the
      domain is not the whole type (recommended):
        - `without = enum_storage` turns it off, and `with = enum_storage`
          raises the limit to 4 096;
        - every validated `u8`/`i8` qualifies;
        - it costs about 7 ms of compile time per type at 256 values;
    - opt-in only;
    - none.

  With any enum storage, confirm the §4.4 rules: one storage type per wrapper,
  with the precedence enum, then `NonZero`, then `Wrapping`; the same hull at
  every pointer width, or no enum storage; the enum derives only `Clone` and
  `Copy`, and traits go through `raw()`; and `Option<Wrapper>` may shrink.
- **Q22. `NonZero` holding `raw ^ k`.** Options:
    - only where it removes a check nothing else does (recommended): signed
      division's `MIN / -1`, when the domain excludes `MIN` or `-1` and the hull
      is too large for an enum. Measure the `xor` cost first;
    - never.
- **Q23. Narrower primitive field.** It changes `size_of`, which is API.
  Options:
    - not now (recommended): an enum with `#[repr(raw)]` gives the exact hull
      without changing the size;
    - an opt-in flag.
- **Q26. Spelling of the `unsafe` opt-in.** Options:
    - a root key `unsafe_code = allow | deny`, default `deny` (recommended). It
      mirrors rustc's `unsafe_code` lint and stays out of profiles;
    - a `with = unsafe_code` flag, which profile bodies would also accept.
- **Q27. Which `unsafe` forms.** Recommendation:
    - U1–U5 for integers;
    - U6 later, since it rests on an incomplete internal feature;
    - floats reject the key with a targeted diagnostic until a float form shows
      a measured gain.
    - `packed` accepts the key as a no-op (L23).

  Also confirm §4.5's rule that each U form gets a `debug_assert!` of its fact,
  and R11's consequences (`lax` with the opt-in is an error).
- **Q28. `#![forbid(unsafe_code)]` in the consumer.** [INFERENCE] rustc
  suppresses most lints inside external macro expansions, so `unsafe` carrying
  typekin's own spans may not trigger the consumer's `unsafe_code` lint.
  Options:
    - give every generated `unsafe` the span of the user's
      `unsafe_code = allow` token, so the consumer's lint level applies, and
      verify it with a fixture crate (recommended);
    - leave it as is; the opt-in key is then the only record.
- **Q34. ADR layout (§12).** Six new ADRs (0028-0033, one per topic), with
  proof evidence and the budget extending ADR 0003. The alternative is one ADR
  for the whole plan.
- **Q37. Narrowing that breaks an infallible operation.** Options:
    - expansion fails with a counterexample, such as "`refine` breaks
      `Pct & Pct`: 1 & 2 = 0 is outside `1..=100`". The consumer then widens
      the accepted domain or changes the API domain (recommended: signatures
      and `total`'s no-panic promise stay honest);
    - the operation keeps `T` and panics on such a result, even at `total`.
      That would be the consumer's decision, and the docs would say so.
- **Q38. Which domain other types see.** `Within` forms and the friend
  annotation (§3.7) publish a domain. Options:
    - the API domain (recommended). It is a sound over-approximation of every
      value the type holds, and narrowing the accepted domain never changes
      what dependents compile against;
    - the accepted domain. Dependents prove more, but a source's narrowing or
      widening changes their proofs.
- **Q39. Typekin releases that prove more.** A release with more proof rules
  can show more operations infallible over an unchanged API domain. That turns
  a `Result` into `T` at `total`, a breaking change for the consumer's
  dependents. Options:
    - accept it, and have each release list the operations it newly proves
      (recommended);
    - make only operations listed in `prove = [...]` infallible (the old Q16
      (c)), so the API never moves without a configuration change.

## 8. Measurements (2026-11-06, still valid)

Measured on an i9-12900K at opt-level 0, Cargo's default for proc macros.
"Proof cost" is the whole-expansion time with `with = [proven]` minus
`without = [proven]`.

Value count makes no difference:

| Domain                                   | Proof cost |
|------------------------------------------|------------|
| `u32` `1..34` (33 values)                | 0.60 ms    |
| `u32` `1..=4_000_000_000`                | 0.74 ms    |
| `u128` `1..34`, 84 closure questions     | 0.46 ms    |
| `u128` `1..=2^127`, 84 closure questions | 0.45 ms    |

Interval count drives cost, roughly quadratically:

| Intervals | Proofs, `u64` | Proofs, `isize` | Expansion without proofs | Proofs ÷ expansion |
|-----------|---------------|-----------------|--------------------------|--------------------|
| 1         | 0.95 ms       | 0.74 ms         | 5.9 ms                   | 16%                |
| 2         | 1.1 ms        | 1.2 ms          | 6.1 ms                   | 20%                |
| 4         | 2.0 ms        | 2.3 ms          | 6.4 ms                   | 36%                |
| 8         | 5.4 ms        | 6.1 ms          | 6.9 ms                   | 88%                |
| 16        | 16.6 ms       | 22.4 ms         | 8.1 ms                   | 277%               |

Reproduction (throwaway; debug test profile):

- **Per question:** parse a `Domain`, call `resolve(subject, false)`, then time
  `ResolvedDomain::closure(op, rhs, policy)` over 84 questions for 20 rounds.
- **Per type:** time `integral::ekran_with` with and without `proven`, as the
  helper in `src/unit/t_proof.rs:12-26` does.
- **Domains:** pieces spaced 1 000 apart, with every other piece negated for
  signed types.
- **rustc classes:** the §2.1 probe.

## 9. Changes from the 2026-11-06 plan

- Levels `typekin | core | none` become `enforcement = lax | invariant |
  strict | total`. `none` (wrapping everywhere) is dropped: wrapping is what a
  profile means, not a check level.
- D2 (no cfg guards) is overturned by `lax`. D12 (no debug-only shortcut)
  still holds for `invariant`, `strict` and `total`.
- D8 (proofs never remove rustc's checks) is overturned for `strict` and
  `total`.
- D1 (a value is always valid) holds at `invariant`, `strict` and `total`. `lax`
  relaxes it in release builds.
- D4 (ingress is always validated) now covers only fallible ingress under
  `lax`.
- A2 (profiles as presets; contradicting them is an error) is dropped.
- A1 (operators that cannot return a failure panic with typekin's panic) holds
  at `lax`, `invariant` and `strict`, the default included. At `total`, a
  regular form that can fail returns `Result` instead (§3.6).
- AGENTS.md's "generated code calls no `unsafe` code" gains an opt-in exception
  (§4.5). ADR 0003's claim that a wrong proof reaches `new_unchecked` is stale
  (§4.6).
- Blind trust (the `unsafe` friend capability, `with_blind_trust`,
  `_blind_trust`) goes (L17). AGENTS.md's blind-trust rules go with it, and its
  naming rule ("`blind_trust`, never `trusted`") needs another example.
- ADR 0022 rejected fallible operators ("It will be an unpleasant API").
  Operators that return `Result` at `total` (L25) reverse that, and the ADR
  needs the new decision.
- Corrections to the old plan:
    - under the wrapping profiles, `MIN / -1` never panicked; they emit
      `wrapping_div`;
    - integers have no `Sum`/`Product`.

## 10. Phases

Tests follow AGENTS.md: consumer-visible behavior in `src/e2e` (`fx_*`/`test_*`
inline modules, `test_fast`), diagnostics in `src/unit`, and assembly in
`test_slow`. Every phase ends by updating the ADRs it implements (§12): status,
consequences and code references.

### Phase 0: align and record

1. Answers to §7.
2. Record the decisions before any code changes (§12):
    - write the new ADRs, with status "Accepted (not implemented)". ADR 0028
      contains the table of L25 verbatim, and ADR 0003 stops calling `prove` a
      contract (L26);
    - update each affected existing ADR: mark the decisions this plan replaces,
      and point to the new ADR.
3. R6: fix the shift truncation. Regression test: a `sequential` wrapper with
   shifts re-enabled rejects an amount of 2^32 + 1 on 64-bit
   (`mod pointer_width_64`).

### Phase 1: parse and resolve

1. Add a `Key` entry (`src/zz_lib.rs:893-990`) and an `Enforcement` enum with
   one name table (`of()`, `name()`), converted once at the parse boundary.
2. The `enforcement` value also decides panicking (L24, L25); there is no
   separate `panic` key. Packed accepts the key as a no-op (L23).
3. Reject duplicates and unknown values. Apply the scope of L23 with targeted
   diagnostics: bitflag forwarding, the nested and text rejections, and the
   accepted no-ops on packed, facets per R3. Tests cover that packed output is
   unchanged with every value of the key.
4. Thread the level into `Cfg` beside `profile`. Keep `Arithmetic` as the
   profile's overflow meaning, and add the level as the second input to every
   `make_expr` caller.
5. Tests (`src/unit`): every diagnostic.
6. Parse `refine` (L30) and resolve both domains (§3.8). Check at expansion
   that `refine` is a subset of `domain` at every pointer width, reusing the
   subset check typekin already compiles in for primitive `Within` sources
   (`src/zz_domain_within_const.rs`). Fail with a counterexample otherwise, and
   with a targeted diagnostic where the forms cannot decide it. Tests
   (`src/unit`): the subset diagnostic, equal domains accepted, and `valid`
   beside `refine`.

### Phase 2: the regular forms (at `invariant`)

1. Write each body of an operator, `Neg`, `Not`, shift, `Step` method and
   `core_int` forward from one emitter: the level's computation (which can fail
   per L29, with L31's error type), then the translation of §3.6.
    - Operators get one body per right-hand-side kind, in its `{Op}Rhs` impl
      (`src/integral_friend.rs:501-561`). `Add<T>` and `AddAssign<T>` forward
      to it.
    - Named forms without a trait carry their own body.
2. Remove the private helpers (`src/integral_items.rs:85-291`, `_neg`, `_not`,
   `_{op}_wrapping`, the `_proven` siblings) and the `fn_math_*` flags (L16),
   and `_eq` and `_cmp` with `fn_op_eq` and `fn_op_cmp` (L35).
3. Remove inherent duplicates of trait impls (L11), keeping bitflag's set
   methods (L34).
4. Replace every runtime panic message in generated code with "precondition
   failed" or "postcondition failed", fixed per site (L32).
5. Tests:
    - each panic site's message (`should_panic(expected = …)`); the assembly
      shows one constant string per site and no formatting (L32);
    - each cell of §3.6 at `invariant` and `total`: the output type (through a
      typed binding), the value, and the panic (`should_panic`) or the `Err`;
    - `pow()` and `+` follow the same rule;
    - per right-hand side: `Div<Percent>` and `Div<u8>` with different
      `Output`s per L29;
    - assignment operators and `Step`: absent at `total` where the operation
      can fail (`assert_not_impl!`), present at the other values;
    - facets and friend right-hand sides;
    - the removed duplicates are gone;
    - the existing e2e tests keep their `a + b`, because the default
      `invariant` keeps today's signatures;
    - assembly: `invariant` equals today's output.

### Phase 3: `lax`, `strict` and `total` in the computation

1. `lax`: guard validation and `sequential`'s check with
   `cfg!(debug_assertions)`, as assertions that leave the regular forms' types
   alone (L40). Leave every path §3.5 lists as never guarded alone, including a
   NaN-free float's NaN rejection (L37). `lax` types publish `lax` as the
   reason for having no form and no table (L38, `src/integral_within.rs`).
   Packed getters of `lax` fields per Q40.
2. `strict` and `total`: the §3.4 forms for operators, shifts, `Neg`,
   `core_int` forwards and `Step`. Every failure becomes the computation's
   `Err`. At `total`, construction too (L33): `make` and `map` return `Result`,
   and `From` impls that can fail are not generated. `of()` keeps `T` and
   accepts only friends proven to fit (L36).
3. Proof policy per R7 (`closure_policy`, `src/integral_proof.rs:125-139`).
4. Tests:
    - e2e per level: overflow, `/ 0`, `MIN / -1`, `MIN % -1`, shifts and `ilog`;
      `lax` panics on an invalid result in-crate (tests build with debug
      assertions), and the assembly row below shows its release behavior;
    - update `test_integral_neg.rs`, which pins "operation overflowed" (now
      "precondition failed", L32);
    - at `total`, `make` and `map` return `Result`, and the `From` impls that
      can fail are absent (`assert_not_impl!`);
    - at `total`, `of()` returns `T` for a `Within` friend and for a primitive
      friend that fits, and a friend that is not proven to fit is rejected: a
      `src/unit` diagnostic for primitives, and a fixture crate for the failing
      `Within` proof (L36);
    - at `lax`, the regular forms return `T` (typed bindings, L40);
    - `strict_*` panics with "precondition failed" at `strict` and returns
      `Err` at `total` (L39);
    - fixture crate, rejections (L38): `Within` from a `lax` source, and a
      packed window narrower than a `lax` field's raw type;
    - assembly: `invariant` unchanged; `strict` equal to hand-written
      `checked_*` plus a panic, and `total` to `checked_*` mapped to `Err`;
      `lax` at opt-level ≥ 1 without debug assertions equal to the raw
      operation, and for a NaN-free float equal to the raw operation plus the
      NaN check (L37).

### Phase 4: checked forms

1. Move the checked forms into the `{Op}Rhs` dispatch (§3.3), per Q42.
   Today's `checked_*` forwards (`src/integral_forward.rs:795-891`) become the
   checked forms of their operations (L42).
2. Apply R9, L41 and L43, with the `checked_all` flag. Each form is a
   `Forward`, removable with `without = fn_checked_*`. Constness follows the
   operator's `konst_*` flags.
3. Tests:
    - a checked form returns `None` on failure at every level and never
      panics;
    - at `total`, a fallible `+` returns `Result<T, ()>`, and `checked_add`
      returns `Option<T>` (typed bindings);
    - an operator's checked form exists under `without = core_int`;
    - a `produce` target that is itself `Option` comes back nested (L41);
    - with `checked_all`, an infallible operation has a checked form, always
      `Some`; without it, the form is absent (`assert_no_method!`);
    - ownership of non-`Copy` friends;
    - absence after removal (`assert_no_method!`).

### Phase 4b: friends with a typekin domain (Q30-Q33)

Removing blind trust can land at any time after Phase 1. The annotation needs
Q31's design.

1. Remove blind trust (L17) in every family, with its example, fixtures and
   tests, and the `Within`-beside-`unsafe` diagnostic
   (`src/zz_friend.rs:506-508`).
2. Apply Q30: `Bit` implies `Make`, and the rest of §3.7's table stays as is.
3. Parse the annotation (Q32). Reject it with a targeted diagnostic on
   primitive friends, whose domain is the whole type.
4. Emit domain-aware construction per Q31, on top of `src/integral_within.rs`
   and `src/zz_domain_within*.rs`; `Within` per Q33.
5. Tests:
    - e2e: a friend with domain `1..4` into a target `1..10 & !{3}` rejects
      `3`. A friend value outside its domain, built with `transmute` over plain
      storage, passes through unchecked (L19);
    - fixture crate: the annotation on a non-typekin friend fails to compile;
    - unit: the diagnostics;
    - assembly: the skipped checks are absent, and validation stays without
      the annotation (the negative control).

### Phase 5: formal evidence (Q10)

1. A spike: install the chosen tools and time `+` at `u64` and `*` at `u128`.
2. One lemma per rule:
    - the hull rules: corners for `+ - * /`, `div_euclid` and `midpoint`; `%`
      and `rem_euclid`; `& | ^`; shifts; `neg`, `!`, `abs`, `isqrt`;
    - `settle` per policy, `MIN / -1` included;
    - `split` and `merge`;
    - the new obligations of §4.1.
    - the `Within` subset check and the published forms
      (`src/zz_domain_within*.rs`), if Q31 relies on them.
3. Record which `ClosureOp` × `Policy` × `Rhs` combinations have evidence. Only
   those may return `Closure::Proven`; the rest return "not proven".
4. Keep the exhaustive 8-bit sweeps (`src/unit/t_domain_closure.rs:871-1208`)
   as regression tests.

### Phase 6: proofs remove checks

1. Add queries on `ResolvedDomain` for the §4.1 obligations, per domain:
   proofs over the API domain decide whether an operation can fail (L29), and
   proofs over the accepted domain remove runtime checks (§3.8). Narrowing
   follows Q37.
2. `settle_proven` (`src/integral_cfg.rs:593-615`): the §4.3 budget defaults,
   per Q11.
3. Tests:
    - expansion tokens in `src/unit/t_proof.rs` for a wide type within budget;
    - the budget boundary (8 against 9 intervals);
    - a `prove`-listed operator with a plain `Output`, and an expansion failure
      when its proof fails;
    - the §3.8 worked example: `a | b` keeps its `Result` without a runtime
      check (assembly), and an accepted domain of `1..=100` fails expansion
      per Q37;
    - assembly: a proven `strict` or `total` operation equal to the bare
      `wrapping_*`, and an unsigned `NonZero` divisor without a zero test.

### Phase 7: storage that tells LLVM the domain (Q21-Q23)

1. Choose the storage per §4.4 beside `NonZero` and `Wrapping`
   (`src/integral_cfg.rs:164-174`).
2. Emit it in `src/integral.rs:654-852`:
    - the enum declaration, after the item as the ordering rule requires,
      inside the `hide_in` module;
    - safe `match` construction and `as` reads, both valid in `const`.
3. Tests:
    - e2e:
        - `size_of` unchanged, and the `Option<W>` niche;
        - construction and rejection at the hull's edges and at holes;
        - a value outside the hull panics on every construction path;
        - generic wrappers;
    - unit: `AsRef<raw>` rejected with enum storage;
    - assembly:
        - validation of `Pct / Pct` folds with enum storage, and stays with
          `without = enum_storage` (the negative control);
        - `x / divisor` without a zero test;
        - a user-side array index without a bounds check;
    - compile-time measurements at the Q21 limits.

### Phase 8: opt-in `unsafe` (Q26-Q28)

This needs Phase 5's evidence table: U2–U4 apply only to proofs with evidence.

1. Parse the key (Q26) at the root. Reject it with `lax` (R11) and per Q27;
   packed accepts it as a no-op (L23), with a test that its output is
   unchanged.
2. Emit U1–U5 through the existing emitters, with spans per Q28:
    - raw reads (`get_raw`);
    - storage construction (`src/integral.rs:792-822`);
    - `_proven` and `_within`;
    - the proven forms of §4.1.
3. Tests:
    - e2e: every result unchanged for valid values (the existing suites, run
      with the opt-in);
    - unit: the `lax` and float rejections;
    - fixture crate: `#![forbid(unsafe_code)]` with the opt-in fails to compile
      (Q28);
    - assembly: each U form equal to hand-written `unsafe` code, and the
      opt-in never larger than the safe output; without the opt-in the checks
      stay (the negative control);
    - Miri over the opt-in suites, behind its own feature (`test_slow_miri`),
      because the environment may lack Miri.

### Phase 9: documentation

- `src/lib.rs` and AGENTS.md: profiles, arithmetic, operators, friendship,
  storage, the packed no-op exception, the `unsafe` opt-in, constness, the
  proof default and the two domains;
- AGENTS.md's naming rule, which says "checked" is not used for typekin
  validation: `checked_op` is now the fallible form of every operation, as in
  core, and validates its result (L42);
- the catch of narrowing the accepted domain (§3.8), documented as the
  consumer's decision;
- the table of L25, verbatim, in `src/lib.rs` and in README per Q12;
- `prove` no longer called a contract (L26): README per Q12,
  `examples/my_proven.rs` and its expansion;
- a final pass over the ADRs of §12;
- examples with their `_exp.rs` (`just <name>-expand`, then
  `cargo +nightly fmt`).
- PHILOSOPHY.md's "Friendship Is an Infallible Contract": at `total`, `of()`
  accepts only friends proven to fit (L36).

### Phase 10: text (L23)

Text accepts `enforcement` from this phase on, with meanings defined here: `lax`
guards panicking ingress, `strict` and `total` add char-boundary and index
prechecks to the regular string operations, and some values may end up behaving
alike. Text friends with a typekin text domain (§3.7).

## 11. Code map

- Arithmetic emitter: `src/zz_profile_integral.rs:52-150`; per profile
  `:296-306`; per wrapper `:404-426`.
- Facet slots: `src/integral_cfg.rs:397-420`; `{Op}Rhs` traits:
  `src/integral_friend.rs:475-600`.
- Helpers, `Neg`, `_or_panic`, `_proven`: `src/integral_items.rs:85-291`,
  `:790-822`, `:987-1046`.
- `core_int` forward table: `src/integral_forward.rs:738-897`;
  `_result_or_panic`: `:473`.
- `Step`: `src/integral_impls.rs:153-238`.
- Output type: `src/zz_conv.rs:24-121`.
- Proof gate and policy: `src/integral_proof.rs:99-149`. `proven` default:
  `src/integral_cfg.rs:593-615`.
- Closure analysis: `src/zz_domain_closure.rs` (`MAX_INTERVALS` :30, `Policy`
  :59, `settle` :731-773, hull rules :905-1085). Sweeps:
  `src/unit/t_domain_closure.rs:871-1208`.
- Storage choice: `src/integral_cfg.rs:164-174`; storage emission:
  `src/integral.rs:654-852`.
- Literal macros: `src/zz_lit.rs:353-381` (through `try_make`).
- bitflag nested profile: `src/bitflag.rs:2080-2117`; enum shifts
  `:1559-1590`.
- ADRs: see §12.

## 12. ADRs

Phase 0 records every decision of this plan in ADRs before any code changes,
and each later phase updates the ADRs it implements.

### 12.1 New ADRs

| ADR                          | Covers                                                                                                                                                                                                                     |
|------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `adr0028__enforcement.md`    | the `enforcement` key: values, the table of L25 verbatim, default and scope (L1-L4, L21-L25, L27); rustc's check classes and `MIN` (§2.1); levels against profiles (R1, §3.2); `strict`/`total` prechecks, `strict_*` included (§3.4, L39); `lax` guards as assertions, what stays unguarded, and why `lax` types publish no domain (§3.5, L37, L38, L40); proof policy per level (R7). Q13 |
| `adr0029__operation_forms.md` | regular and checked forms, the translation, no inherent duplicates, no operation helpers, the error type, the two panic messages, construction at `total` (L5, L7-L9, L11-L13, L16, L31-L35, L41-L43, R4, R8, R9, §3.3, §3.6). Q42 |
| `adr0030__domain_storage.md` | storage that tells rustc and LLVM the domain (L10, R10, §4.4). Q21-Q23                                                                                                                                                     |
| `adr0031__unsafe_opt_in.md`  | `unsafe_code = allow`, and where UB can arise (L14, L15, R11, §4.5, §4.6). Q26-Q28                                                                                                                                         |
| `adr0032__typekin_friends.md` | blind trust removed; friends with a typekin domain, domain-aware construction and trust; which capabilities imply `Make`, and why; `of()` returns `T` and accepts only proven friends at `total` (L17-L20, L36, §3.7). Q30-Q33                |
| `adr0033__two_domains.md`    | the API domain and the accepted domain: `domain` and `refine`, the subset check, the default, proofs making operations infallible, and the catch of narrowing (L28-L30, §3.8). Q37-Q39                                  |

Proof evidence and the budget extend ADR 0003 instead of getting their own ADR.

### 12.2 Updated ADRs

| ADR                          | Update                                                                                                                                                                                                                                                                     |
|------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0003 closure proofs          | the `proven` default by interval budget at every width (§4.3, Q11); formal evidence (D9, Q10) and its table; proofs over the API domain decide whether an operation can fail (L29), and proofs over the accepted domain remove checks (L28); `prove` is a checked requirement, no longer "a checked contract" (L26); the new obligations (§4.1); policy per level (R7); the stale `new_unchecked` claim (§4.6) |
| 0004 float semantics         | the `enforcement` values on floats (`strict` acts as `invariant`, L23); a NaN-free float rejects NaN in every build, even at `lax` (L37); float `min`/`max`/`clamp` go where `Ord` exists (L34); floats reject `unsafe_code` (Q27)                                    |
| 0005 `hide_in`               | `blind_trust_visibility` goes with blind trust (L17)                                                                                                                                                                                                                       |
| 0006 domain language         | `refine` and its subset check (L30), and the two-domain model (L28)                                                                                                                                                                                                       |
| 0007 generated trait impls   | operator `Output` from the translation (§3.6); impls whose trait fixes `T` are dropped at `total` (R8); the `Step` overrides                                                                                                                                              |
| 0008 friendship model        | friend right-hand sides through `{Op}Rhs` for every wrapper; the `unsafe` capability goes, replaced by friends with a typekin domain (§3.7); `Bit` implies `Make`, with the per-capability reasoning (L20, Q30); `of()` always returns `T`, and at `total` accepts only friends proven to fit (L36) |
| 0009 bitflag model           | `enforcement` and the other new keys forwarded to the nested value type (L23); the set methods stay as stable `const fn`s (L34); the enum's shifts take the value type's `Output`; nested `unsafe` friends and `with_blind_trust` go (L17)                             |
| 0010 facets                  | every wrapper dispatches through `{Op}Rhs`; `enforcement` and `unsafe_code` are wrapper-wide (R3)                                                                                                                                                                          |
| 0011 profiles and storage    | profiles describe numbers only (L13, R1); `sequential` no longer panics by itself; storage precedence: enum, `NonZero`, `Wrapping` (§4.4)                                                                                                                                 |
| 0012 sequence numbers        | `sequential`'s overflow surfaces through the computation and the `enforcement` value (§3.6)                                                                                                                                                                               |
| 0013 pointer widths          | enum storage needs the same hull at every width (§4.4)                                                                                                                                                                                                                    |
| 0014 `friends` and `constructor` | the `unsafe` capability and its `Unsafe` protocol trait go (L17)                                                                                                                                                                                                       |
| 0015 constness               | whether checked forms stay stable `const` (Q42); `helper_konst` no longer covers operation helpers (L16)                                                                                                                                                                  |
| 0016 codegen architecture    | one emitter writes the computation and the translation; the helper layer goes (L16)                                                                                                                                                                                        |
| 0018 removals and opt-outs   | the `fn_math_*` flags are removed (L16), and `fn_op_eq`/`fn_op_cmp` (L35); removal of checked and named forms (`fn_checked_*`); the `checked_all` flag (L43)                                                                                                             |
| 0019 assembly verification   | comparisons for the levels, the translation, enum storage and the `unsafe` forms                                                                                                                                                                                         |
| 0020 test architecture       | the `test_slow_miri` feature (Phase 8)                                                                                                                                                                                                                                     |
| 0022 integral API surface    | fallible operators, reversing the rejection; checked forms for every fallible operation, merged with the `checked_*` forwards (L42); construction at `total` (L33); no inherent duplicates (L11, L34); `unsafe` core methods are still never forwarded, even where typekin uses them internally under the opt-in |
| 0023 packed                  | packed accepts `enforcement` and `unsafe_code` as no-ops (L23); a window holding a `lax` field is as wide as its raw type (L38); reading a `lax` field back (Q40)                                                                                                       |
| 0024 `Within`                | `lax` sources publish no form or table, so `Within` from them fails to compile (L38); DW3 ("a proof bug cannot become UB") holds only without the `unsafe` opt-in; `Within` and the friend annotation (Q33); its forms now also serve domain-aware construction (Q31), and publish the API domain (Q38) |

ADR 0021 (text) loses blind trust now (L17) and gains the `enforcement` values
and text friends with Phase 10. ADR 0025 (Serde) is unchanged: deserialization
already returns its failure.

## 13. Future work

Not part of this plan. Recorded so that the decisions above leave room for it.

- **Callbacks in `refine` (L30).** Const-callable `valid` callbacks could also
  narrow the accepted domain, if rustc can evaluate them to prove the subset.
- **An error enum (L31).** Typekin will need an error enum whose variants may
  carry a value, such as the rejected raw value. That is how precondition
  failures report a value without an `Option`.
- **Panic messages (L32).** The final wording of "precondition failed" and
  "postcondition failed" is decided later. Formatting the bad value may come as
  an opt-in callback outside the inline code, never by default.
- **Friends' domains (L36).** At `total`, how `of()` learns and checks the
  domain of a friend that is neither a typekin type nor a primitive, so that
  such a friend can qualify too.
