//! The `core` forwards as rows: every generated function that calls the
//! primitive function of its own name. `integral_forward` builds the
//! functions from the rows; `domain` typing (`zz_domain_int`,
//! `zz_domain_float`) reads which of them an expression may apply.

use crate::zz_domain_closure::{
    ClosureOp,
    Policy,
    Rhs,
};
use crate::zz_domain_int::LensKind;
use crate::zz_lib::ValidationCfg;
use crate::zz_values::FnName;

/// A generated function forwarding to the primitive function of the same
/// name.
pub(crate) struct CoreFwd {
    pub(crate) name: FnName,
    pub(crate) recv: Recv,
    /// The parameters after the receiver.
    pub(crate) args: &'static [Arg],
    pub(crate) ret: Ret,
    pub(crate) konst: FwdKonst,
    pub(crate) when: When,
    /// The closure-proof question; `None`: never proven.
    pub(crate) proof: Option<(ClosureOp, Policy)>,
    /// The domain shape that makes the result valid without a proof.
    pub(crate) structural: Option<Structural>,
    pub(crate) panics: Panics,
    pub(crate) lens: InDomain,
}

impl CoreFwd {
    /// The row of `rows` named `function`.
    pub(crate) fn of(
        rows: &'static [CoreFwd],
        function: FnName,
    ) -> Option<&'static CoreFwd> {
        return rows.iter().find(|it| it.name == function);
    }

    /// The right-hand side of the row's closure question; a row with a
    /// proof takes at most one argument.
    pub(crate) fn rhs(&self) -> Rhs {
        return match self.args {
            [] => Rhs::Unary,
            [Arg::Wrapped(_)] => Rhs::SameDomain,
            _ => Rhs::Shift,
        };
    }
}

/// The functions of `rows` a `domain` applies as lenses, or those it tests
/// as predicates, in table order: domain diagnostics list them so.
pub(crate) fn domain_functions(
    rows: &[CoreFwd],
    predicates: bool,
) -> Vec<FnName> {
    return rows
        .iter()
        .filter(|it| match it.lens {
            InDomain::No => false,
            InDomain::Int(_) | InDomain::Float => !predicates,
            InDomain::Predicate => predicates,
        })
        .map(|it| it.name)
        .collect();
}

/// What a forward is called on.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Recv {
    /// `self`.
    Value,
    /// `&self`.
    Ref,
    /// Nothing: an associated function, calling the primitive's.
    Assoc,
}

/// A parameter after the receiver.
#[derive(Clone, Copy)]
pub(crate) enum Arg {
    /// `name: Self`, passed on as its raw value.
    Wrapped(&'static str),
    /// `name: &Self`, passed on as a reference to its raw value.
    Ref(&'static str),
    /// `name: ty`, passed on as it is.
    Plain(&'static str, Plain),
}

/// A type other than the wrapper: its values pass through unvalidated.
#[derive(Clone, Copy)]
pub(crate) enum Plain {
    Bool,
    U32,
    I32,
    OptionU32,
    /// `[u8; size_of::<el>()]`.
    Bytes,
    /// The unsigned primitive of the same width: an integer's unsigned
    /// counterpart (itself when unsigned), a float's bits.
    Unsigned,
    /// The signed primitive of the same width.
    Signed,
    /// `::core::num::FpCategory`.
    Category,
    /// `::core::cmp::Ordering`.
    Ordering,
}

/// What a forward returns.
#[derive(Clone, Copy)]
pub(crate) enum Ret {
    /// The primitive function's result as it is.
    Raw(Plain),
    /// `Self`: the validated result.
    Wrap,
    /// `Option<Self>`: `None` from the primitive function, or for an
    /// invalid value.
    Opt,
    /// A tuple of two results, at least one a validated `Self`.
    Pair(RetEl, RetEl),
}

impl Ret {
    /// An invalid result panics: a validated `Self` cannot report it.
    pub(crate) fn panics(self) -> bool {
        return matches!(self, Self::Wrap | Self::Pair(..));
    }
}

/// One result of a `Ret::Pair`, with the name the generated code binds it
/// to.
#[derive(Clone, Copy)]
pub(crate) enum RetEl {
    /// A validated `Self`.
    Wrap(&'static str),
    /// A `bool`, passed through.
    Bool(&'static str),
}

/// Whether a forward is `const`.
#[derive(Clone, Copy)]
pub(crate) enum FwdKonst {
    /// Whatever the constness settings.
    Always,
    /// By `konst_core_int` or `konst_core_float`.
    Setting,
    /// The primitive function is not `const`.
    Never,
}

/// When a forward is generated, beyond the flag of its table.
#[derive(Clone, Copy)]
pub(crate) struct When {
    pub(crate) sign: Sign,
    pub(crate) gate: Gate,
}

/// The integer signs a forward exists for.
#[derive(Clone, Copy)]
pub(crate) enum Sign {
    Any,
    Signed,
    Unsigned,
}

/// A setting a forward depends on.
#[derive(Clone, Copy)]
pub(crate) enum Gate {
    Always,
    /// Only with `konst = nightly` (not `konst_core_int`): the primitive
    /// function is unstable.
    NightlyKonst,
    /// Only with `math = std`: `::core::fNN::math` lacks the function.
    Std,
    /// Only without `Ord`, whose `min`, `max`, and `clamp` select valid
    /// values and need no validation.
    Unordered,
}

/// A domain shape that makes an operation closed without a closure proof.
#[derive(Clone, Copy)]
pub(crate) enum Structural {
    /// The `-` operator on a symmetric domain.
    Neg,
    /// Plain `abs`.
    Abs,
}

impl Structural {
    pub(crate) fn closed(
        self,
        validation: &ValidationCfg,
    ) -> bool {
        return match self {
            Self::Neg => validation.neg_closed(),
            Self::Abs => validation.abs_closed(),
        };
    }
}

/// Whether the primitive function itself panics. `panic_at` asks only for
/// a raw result: below `anything` a validated `Self` is left out anyway.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Panics {
    Never,
    /// On some inputs, as `ilog2` on a non-positive value does: left out
    /// where `panic_at` checks division.
    Arithmetic,
}

/// How a `domain` expression may use a forwarded function.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum InDomain {
    No,
    /// An integer lens (`abs in ..=3`), read as its `LensKind`.
    Int(LensKind),
    /// A float lens.
    Float,
    /// A test by itself: `is_nan`, `!is_nan`.
    Predicate,
}

const UNARY: &[Arg] = &[];
const BYTES: &[Arg] = &[Arg::Plain("bytes", Plain::Bytes)];

/// `fn name(self, args) -> ret`, `const` by the settings.
const fn row(
    name: FnName,
    ret: Ret,
    args: &'static [Arg],
) -> CoreFwd {
    return CoreFwd {
        name,
        recv: Recv::Value,
        args,
        ret,
        konst: FwdKonst::Setting,
        when: When {
            sign: Sign::Any,
            gate: Gate::Always,
        },
        proof: None,
        structural: None,
        panics: Panics::Never,
        lens: InDomain::No,
    };
}

/// `const fn name(self) -> out`, whatever the constness settings.
const fn read(
    name: FnName,
    out: Plain,
) -> CoreFwd {
    return CoreFwd {
        konst: FwdKonst::Always,
        ..row(name, Ret::Raw(out), UNARY)
    };
}

/// A validated result, proven when `op` under `policy` is closed.
const fn proven(
    name: FnName,
    ret: Ret,
    args: &'static [Arg],
    op: ClosureOp,
    policy: Policy,
) -> CoreFwd {
    return CoreFwd {
        proof: Some((op, policy)),
        ..row(name, ret, args)
    };
}

/// An associated function building a value from the primitive's.
const fn from(
    name: FnName,
    args: &'static [Arg],
) -> CoreFwd {
    return CoreFwd {
        recv: Recv::Assoc,
        ..row(name, Ret::Wrap, args)
    };
}

/// A `float_math` function only `math = std` has; none of them is `const`.
const fn std_only(
    name: FnName,
    ret: Ret,
    args: &'static [Arg],
) -> CoreFwd {
    return gated(Gate::Std, not_const(row(name, ret, args)));
}

const fn signed(it: CoreFwd) -> CoreFwd {
    return CoreFwd {
        when: When {
            sign: Sign::Signed,
            ..it.when
        },
        ..it
    };
}

const fn unsigned(it: CoreFwd) -> CoreFwd {
    return CoreFwd {
        when: When {
            sign: Sign::Unsigned,
            ..it.when
        },
        ..it
    };
}

const fn gated(
    gate: Gate,
    it: CoreFwd,
) -> CoreFwd {
    return CoreFwd {
        when: When { gate, ..it.when },
        ..it
    };
}

const fn not_const(it: CoreFwd) -> CoreFwd {
    return CoreFwd {
        konst: FwdKonst::Never,
        ..it
    };
}

const fn lens(
    lens: InDomain,
    it: CoreFwd,
) -> CoreFwd {
    return CoreFwd { lens, ..it };
}

/// The `core_int` forwards, in emission order: the functions a `domain`
/// applies and its predicates (domain diagnostics list both in this
/// order), then the common rows, the signed, and the unsigned ones.
pub(crate) const CORE_INT: &[CoreFwd] = {
    use ClosureOp::*;
    use InDomain::*;
    use LensKind as L;
    use Plain::*;
    use Ret::*;
    const SAME: &[Arg] = &[Arg::Wrapped("rhs")];
    const BASE: &[Arg] = &[Arg::Wrapped("base")];
    const VALUE: &[Arg] = &[Arg::Wrapped("value")];
    const ROTATE: &[Arg] = &[Arg::Plain("n", U32)];
    const RHS: &[Arg] = &[Arg::Plain("rhs", U32)];
    const EXP: &[Arg] = &[Arg::Plain("exp", U32)];
    const FUNNEL: &[Arg] = &[
        Arg::Wrapped("right"),
        Arg::Plain("n", U32),
    ];
    const CARRY: &[Arg] = &[
        Arg::Wrapped("rhs"),
        Arg::Plain("carry", Bool),
    ];
    const BORROW: &[Arg] = &[
        Arg::Wrapped("rhs"),
        Arg::Plain("borrow", Bool),
    ];
    const MUL: &[Arg] = &[
        Arg::Wrapped("rhs"),
        Arg::Wrapped("carry"),
    ];
    const MUL_ADD: &[Arg] = &[
        Arg::Wrapped("rhs"),
        Arg::Wrapped("carry"),
        Arg::Wrapped("add"),
    ];
    /// `(Self, bool)`: the value wraps.
    const OVERFLOWING: Ret =
        Ret::Pair(RetEl::Wrap("value"), RetEl::Bool("overflowed"));
    const CARRIED: Ret = Ret::Pair(RetEl::Wrap("value"), RetEl::Bool("carry"));
    const BORROWED: Ret =
        Ret::Pair(RetEl::Wrap("value"), RetEl::Bool("borrow"));
    const WIDE: Ret = Ret::Pair(RetEl::Wrap("low"), RetEl::Wrap("high"));
    const C: Policy = Policy::Checked;
    const S: Policy = Policy::Saturating;
    // Overflow panics with overflow checks and wraps without: wrapped
    // results cover both.
    const W: Policy = Policy::Wrapping;
    &[
        signed(lens(
            Int(L::UnsignedAbs),
            CoreFwd {
                structural: Some(Structural::Abs),
                ..proven(FnName::Abs, Wrap, UNARY, Abs, W)
            },
        )),
        signed(lens(
            Int(L::UnsignedAbs),
            row(FnName::UnsignedAbs, Raw(Unsigned), UNARY),
        )),
        lens(Int(L::AbsDiff), row(FnName::AbsDiff, Raw(Unsigned), SAME)),
        signed(lens(Int(L::Signum), row(FnName::Signum, Wrap, UNARY))),
        lens(Int(L::CountOnes), read(FnName::CountOnes, U32)),
        lens(Int(L::CountZeros), read(FnName::CountZeros, U32)),
        lens(Int(L::LeadingZeros), read(FnName::LeadingZeros, U32)),
        lens(Int(L::TrailingZeros), read(FnName::TrailingZeros, U32)),
        lens(Int(L::LeadingOnes), read(FnName::LeadingOnes, U32)),
        lens(Int(L::TrailingOnes), read(FnName::TrailingOnes, U32)),
        lens(
            Int(L::RemEuclid),
            proven(FnName::RemEuclid, Wrap, SAME, RemEuclid, W),
        ),
        lens(
            Int(L::DivEuclid),
            proven(FnName::DivEuclid, Wrap, SAME, DivEuclid, W),
        ),
        lens(Int(L::Isqrt), proven(FnName::Isqrt, Wrap, UNARY, Isqrt, C)),
        unsigned(lens(Predicate, row(FnName::IsPowerOfTwo, Raw(Bool), UNARY))),
        signed(lens(Predicate, row(FnName::IsPositive, Raw(Bool), UNARY))),
        signed(lens(Predicate, row(FnName::IsNegative, Raw(Bool), UNARY))),
        read(FnName::HighestOne, OptionU32),
        read(FnName::LowestOne, OptionU32),
        CoreFwd {
            args: BASE,
            panics: Panics::Arithmetic,
            ..read(FnName::Ilog, U32)
        },
        CoreFwd {
            panics: Panics::Arithmetic,
            ..read(FnName::Ilog2, U32)
        },
        CoreFwd {
            panics: Panics::Arithmetic,
            ..read(FnName::Ilog10, U32)
        },
        CoreFwd {
            args: BASE,
            ..read(FnName::CheckedIlog, OptionU32)
        },
        read(FnName::CheckedIlog2, OptionU32),
        read(FnName::CheckedIlog10, OptionU32),
        read(FnName::ToBeBytes, Bytes),
        read(FnName::ToLeBytes, Bytes),
        read(FnName::ToNeBytes, Bytes),
        row(FnName::RotateLeft, Wrap, ROTATE),
        row(FnName::RotateRight, Wrap, ROTATE),
        row(FnName::SwapBytes, Wrap, UNARY),
        row(FnName::ReverseBits, Wrap, UNARY),
        row(FnName::IsolateHighestOne, Wrap, UNARY),
        row(FnName::IsolateLowestOne, Wrap, UNARY),
        row(FnName::ToBe, Wrap, UNARY),
        row(FnName::ToLe, Wrap, UNARY),
        from(FnName::FromBe, VALUE),
        from(FnName::FromLe, VALUE),
        from(FnName::FromNeBytes, BYTES),
        from(FnName::FromBeBytes, BYTES),
        from(FnName::FromLeBytes, BYTES),
        proven(FnName::CheckedAdd, Opt, SAME, Add, C),
        proven(FnName::CheckedSub, Opt, SAME, Sub, C),
        proven(FnName::CheckedNeg, Opt, UNARY, Neg, C),
        proven(FnName::CheckedMul, Opt, SAME, Mul, C),
        proven(FnName::CheckedDiv, Opt, SAME, Div, C),
        proven(FnName::CheckedRem, Opt, SAME, Rem, C),
        proven(FnName::CheckedDivEuclid, Opt, SAME, DivEuclid, C),
        proven(FnName::CheckedRemEuclid, Opt, SAME, RemEuclid, C),
        row(FnName::CheckedShl, Opt, RHS),
        proven(FnName::CheckedShr, Opt, RHS, Shr, C),
        row(FnName::CheckedPow, Opt, EXP),
        proven(FnName::Midpoint, Wrap, SAME, Midpoint, C),
        row(FnName::UnboundedShl, Wrap, RHS),
        proven(FnName::UnboundedShr, Wrap, RHS, UnboundedShr, C),
        proven(FnName::SaturatingAdd, Wrap, SAME, Add, S),
        proven(FnName::SaturatingSub, Wrap, SAME, Sub, S),
        proven(FnName::SaturatingMul, Wrap, SAME, Mul, S),
        proven(FnName::SaturatingDiv, Wrap, SAME, Div, S),
        row(FnName::SaturatingPow, Wrap, EXP),
        proven(FnName::WrappingAdd, Wrap, SAME, Add, W),
        proven(FnName::WrappingSub, Wrap, SAME, Sub, W),
        proven(FnName::WrappingNeg, Wrap, UNARY, Neg, W),
        proven(FnName::WrappingMul, Wrap, SAME, Mul, W),
        proven(FnName::WrappingDiv, Wrap, SAME, Div, W),
        proven(FnName::WrappingRem, Wrap, SAME, Rem, W),
        proven(FnName::WrappingDivEuclid, Wrap, SAME, DivEuclid, W),
        proven(FnName::WrappingRemEuclid, Wrap, SAME, RemEuclid, W),
        row(FnName::WrappingShl, Wrap, RHS),
        proven(FnName::WrappingShr, Wrap, RHS, Shr, W),
        row(FnName::WrappingPow, Wrap, EXP),
        row(FnName::Pow, Wrap, EXP),
        proven(FnName::StrictAdd, Wrap, SAME, Add, C),
        proven(FnName::StrictSub, Wrap, SAME, Sub, C),
        proven(FnName::StrictNeg, Wrap, UNARY, Neg, C),
        proven(FnName::StrictMul, Wrap, SAME, Mul, C),
        proven(FnName::StrictDiv, Wrap, SAME, Div, C),
        proven(FnName::StrictRem, Wrap, SAME, Rem, C),
        proven(FnName::StrictDivEuclid, Wrap, SAME, DivEuclid, C),
        proven(FnName::StrictRemEuclid, Wrap, SAME, RemEuclid, C),
        row(FnName::StrictShl, Wrap, RHS),
        proven(FnName::StrictShr, Wrap, RHS, Shr, C),
        row(FnName::StrictPow, Wrap, EXP),
        proven(FnName::OverflowingAdd, OVERFLOWING, SAME, Add, W),
        proven(FnName::OverflowingSub, OVERFLOWING, SAME, Sub, W),
        proven(FnName::OverflowingNeg, OVERFLOWING, UNARY, Neg, W),
        proven(FnName::OverflowingMul, OVERFLOWING, SAME, Mul, W),
        proven(FnName::OverflowingDiv, OVERFLOWING, SAME, Div, W),
        proven(FnName::OverflowingRem, OVERFLOWING, SAME, Rem, W),
        proven(
            FnName::OverflowingDivEuclid,
            OVERFLOWING,
            SAME,
            DivEuclid,
            W,
        ),
        proven(
            FnName::OverflowingRemEuclid,
            OVERFLOWING,
            SAME,
            RemEuclid,
            W,
        ),
        row(FnName::OverflowingShl, OVERFLOWING, RHS),
        proven(FnName::OverflowingShr, OVERFLOWING, RHS, Shr, W),
        row(FnName::OverflowingPow, OVERFLOWING, EXP),
        signed(row(FnName::CastUnsigned, Raw(Unsigned), UNARY)),
        signed(proven(FnName::CheckedAbs, Opt, UNARY, Abs, C)),
        signed(proven(FnName::CheckedIsqrt, Opt, UNARY, Isqrt, C)),
        signed(proven(FnName::StrictAbs, Wrap, UNARY, Abs, C)),
        signed(proven(FnName::SaturatingNeg, Wrap, UNARY, Neg, S)),
        signed(proven(FnName::SaturatingAbs, Wrap, UNARY, Abs, S)),
        signed(proven(FnName::WrappingAbs, Wrap, UNARY, Abs, W)),
        signed(proven(FnName::OverflowingAbs, OVERFLOWING, UNARY, Abs, W)),
        unsigned(row(FnName::CastSigned, Raw(Signed), UNARY)),
        unsigned(row(FnName::BitWidth, Raw(U32), UNARY)),
        unsigned(gated(
            Gate::NightlyKonst,
            row(FnName::FunnelShl, Wrap, FUNNEL),
        )),
        unsigned(gated(
            Gate::NightlyKonst,
            row(FnName::FunnelShr, Wrap, FUNNEL),
        )),
        unsigned(row(FnName::IsMultipleOf, Raw(Bool), SAME)),
        unsigned(row(FnName::NextPowerOfTwo, Wrap, UNARY)),
        unsigned(row(FnName::CheckedNextPowerOfTwo, Opt, UNARY)),
        unsigned(row(FnName::DivCeil, Wrap, SAME)),
        unsigned(row(FnName::NextMultipleOf, Wrap, SAME)),
        unsigned(row(FnName::CheckedNextMultipleOf, Opt, SAME)),
        unsigned(not_const(row(FnName::CarryingAdd, CARRIED, CARRY))),
        unsigned(not_const(row(FnName::BorrowingSub, BORROWED, BORROW))),
        unsigned(not_const(row(FnName::CarryingMul, WIDE, MUL))),
        unsigned(not_const(row(FnName::CarryingMulAdd, WIDE, MUL_ADD))),
    ]
};

/// The `core_float` forwards: the queries a `domain` tests and the
/// functions it applies come first, in the order its diagnostics list them.
pub(crate) const CORE_FLOAT: &[CoreFwd] = {
    use InDomain::*;
    use Plain::*;
    use Ret::*;
    const BITS: &[Arg] = &[Arg::Plain("bits", Unsigned)];
    const SIGN: &[Arg] = &[Arg::Wrapped("sign")];
    const THAT: &[Arg] = &[Arg::Wrapped("that")];
    const THAT_REF: &[Arg] = &[Arg::Ref("that")];
    const BOUNDS: &[Arg] = &[
        Arg::Wrapped("min"),
        Arg::Wrapped("max"),
    ];
    &[
        lens(Predicate, read(FnName::IsNan, Bool)),
        lens(Predicate, read(FnName::IsFinite, Bool)),
        lens(Predicate, read(FnName::IsInfinite, Bool)),
        lens(Predicate, read(FnName::IsNormal, Bool)),
        lens(Predicate, read(FnName::IsSubnormal, Bool)),
        lens(Predicate, read(FnName::IsSignPositive, Bool)),
        lens(Predicate, read(FnName::IsSignNegative, Bool)),
        lens(
            Float,
            CoreFwd {
                structural: Some(Structural::Abs),
                ..row(FnName::Abs, Wrap, UNARY)
            },
        ),
        lens(Float, row(FnName::Signum, Wrap, UNARY)),
        lens(Float, row(FnName::Copysign, Wrap, SIGN)),
        lens(Float, row(FnName::Recip, Wrap, UNARY)),
        lens(Float, row(FnName::ToDegrees, Wrap, UNARY)),
        lens(Float, row(FnName::ToRadians, Wrap, UNARY)),
        row(FnName::NextUp, Wrap, UNARY),
        row(FnName::NextDown, Wrap, UNARY),
        row(FnName::Midpoint, Wrap, THAT),
        gated(Gate::Unordered, row(FnName::Min, Wrap, THAT)),
        gated(Gate::Unordered, row(FnName::Max, Wrap, THAT)),
        gated(Gate::Unordered, row(FnName::Clamp, Wrap, BOUNDS)),
        read(FnName::Classify, Category),
        read(FnName::ToBits, Unsigned),
        from(FnName::FromBits, BITS),
        read(FnName::ToNeBytes, Bytes),
        from(FnName::FromNeBytes, BYTES),
        read(FnName::ToBeBytes, Bytes),
        from(FnName::FromBeBytes, BYTES),
        read(FnName::ToLeBytes, Bytes),
        from(FnName::FromLeBytes, BYTES),
        CoreFwd {
            recv: Recv::Ref,
            ..not_const(row(FnName::TotalCmp, Raw(Ordering), THAT_REF))
        },
    ]
};

/// The `float_math` forwards, from the `math` provider: the functions a
/// `domain` applies come first, in the order its diagnostics list them.
/// The rounding family and `mul_add` are `const` with either provider.
pub(crate) const FLOAT_MATH: &[CoreFwd] = {
    use InDomain::Float;
    use Ret::Wrap;
    const A: &[Arg] = &[Arg::Wrapped("a")];
    const A_B: &[Arg] = &[
        Arg::Wrapped("a"),
        Arg::Wrapped("b"),
    ];
    /// An `i32` exponent.
    const EXPONENT: &[Arg] = &[Arg::Plain("a", Plain::I32)];
    const SIN_COS: Ret = Ret::Pair(RetEl::Wrap("sin"), RetEl::Wrap("cos"));
    &[
        lens(Float, row(FnName::Floor, Wrap, UNARY)),
        lens(Float, row(FnName::Ceil, Wrap, UNARY)),
        lens(Float, row(FnName::Round, Wrap, UNARY)),
        lens(Float, row(FnName::RoundTiesEven, Wrap, UNARY)),
        lens(Float, row(FnName::Trunc, Wrap, UNARY)),
        lens(Float, row(FnName::Fract, Wrap, UNARY)),
        lens(Float, row(FnName::MulAdd, Wrap, A_B)),
        lens(Float, not_const(row(FnName::Sqrt, Wrap, UNARY))),
        lens(Float, not_const(row(FnName::Cbrt, Wrap, UNARY))),
        lens(Float, not_const(row(FnName::RemEuclid, Wrap, A))),
        lens(Float, not_const(row(FnName::DivEuclid, Wrap, A))),
        not_const(row(FnName::Powi, Wrap, EXPONENT)),
        std_only(FnName::Powf, Wrap, A),
        std_only(FnName::Exp, Wrap, UNARY),
        std_only(FnName::Exp2, Wrap, UNARY),
        std_only(FnName::ExpM1, Wrap, UNARY),
        std_only(FnName::Ln, Wrap, UNARY),
        std_only(FnName::Ln1p, Wrap, UNARY),
        std_only(FnName::Log, Wrap, A),
        std_only(FnName::Log2, Wrap, UNARY),
        std_only(FnName::Log10, Wrap, UNARY),
        std_only(FnName::Hypot, Wrap, A),
        std_only(FnName::Sin, Wrap, UNARY),
        std_only(FnName::Cos, Wrap, UNARY),
        std_only(FnName::Tan, Wrap, UNARY),
        std_only(FnName::Asin, Wrap, UNARY),
        std_only(FnName::Acos, Wrap, UNARY),
        std_only(FnName::Atan, Wrap, UNARY),
        std_only(FnName::Atan2, Wrap, A),
        std_only(FnName::Sinh, Wrap, UNARY),
        std_only(FnName::Cosh, Wrap, UNARY),
        std_only(FnName::Tanh, Wrap, UNARY),
        std_only(FnName::Asinh, Wrap, UNARY),
        std_only(FnName::Acosh, Wrap, UNARY),
        std_only(FnName::Atanh, Wrap, UNARY),
        std_only(FnName::SinCos, SIN_COS, UNARY),
    ]
};
