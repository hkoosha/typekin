//! `Within` proofs: every value a source can hold lies in a target's
//! domain, so the target constructs from it without validation.
//!
//! Each side reduces its validation, per pointer width, to a [`Form`]:
//! residue classes modulo one modulus, each a union of disjoint ranges of
//! members. Forms are built by breakpoints and residue sampling. Every leaf
//! of the resolved domain contributes the values where its truth may change
//! (breakpoints) and the period its truth repeats with; between consecutive
//! breakpoints every leaf, and so the whole domain, is constant on each
//! residue class of the lcm of the periods, and one exact evaluation per
//! segment and class decides it. A form must be exact (a source's holds
//! every valid value, a target's only valid ones), so the differential
//! tests compare forms with `ResolvedDomain::contains` value by value.
//!
//! The subset check over two forms lives in `zz_domain_within_const`, which
//! typekin compiles (primitive sources, decided at expansion) and emits
//! verbatim (wrapper sources, decided by rustc's const evaluator).

use std::collections::BTreeSet;
use std::ops::Bound as Limit;

use proc_macro2::{
    Literal,
    TokenStream,
};
use quote::{
    ToTokens,
    quote,
};

use crate::zz_domain_int::{
    LensKind,
    Operand,
    RNode,
    ResolvedDomain,
    eval,
    test_holds,
};
use crate::zz_domain_within_const::{
    __TypekinWithin,
    __TypekinWithinPiece,
    __typekin_within_subset,
};
use crate::zz_lib::ValidationCfg;
use crate::zz_values::{
    FnName,
    Int,
    N,
    POINTER_WIDTHS,
};

/// Above this modulus a domain has no form: the target-side check walks one
/// progression per residue.
pub(crate) const MAX_MODULUS: u128 = 1024;
/// Above this many evaluations (segments times modulus) a domain has no
/// form, which bounds expansion time.
pub(crate) const MAX_EVALUATIONS: u128 = 65_536;
/// Above this many pieces a domain has no form, which bounds the size of the
/// published constant.
pub(crate) const MAX_PIECES: usize = 256;

/// The members `lo, lo + modulus, …, hi` of one residue class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Piece {
    pub(crate) residue: u128,
    pub(crate) lo: Int,
    pub(crate) hi: Int,
}

/// A set of values at one pointer width: `x` is a member iff some piece has
/// `x ≡ residue (mod modulus)` and `lo ≤ x ≤ hi`. Pieces are sorted by
/// `(residue, lo)`, disjoint and not adjacent within a residue, and their
/// ends are members.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Form {
    pub(crate) modulus: u128,
    pub(crate) pieces: Vec<Piece>,
}

/// Why a validation has no form.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Opaque {
    /// `valid` callbacks, as written.
    Callbacks(String),
    /// A lens whose results follow no interval or period: `count_ones`.
    Lens(FnName),
    /// `isqrt in mul(d)`.
    IsqrtMultiples,
    /// The lcm of the periods, above `MAX_MODULUS`.
    Modulus(u128),
    /// Segments times modulus, above `MAX_EVALUATIONS`.
    Evaluations(u128),
    /// Pieces after merging, above `MAX_PIECES`.
    Pieces(usize),
}

impl Opaque {
    /// Why `ty`'s validation has no form, for diagnostics.
    pub(crate) fn describe(
        &self,
        ty: &str,
    ) -> String {
        return match self {
            Self::Callbacks(callbacks) => format!(
                "`{ty}`'s `valid = {callbacks}` is opaque to subdomain proofs"
            ),
            Self::Lens(function) => format!(
                "`{ty}`'s domain uses `{}`, which is opaque to subdomain proofs",
                function.name()
            ),
            Self::IsqrtMultiples => format!(
                "`{ty}`'s domain uses `isqrt` with `mul`, which is opaque to \
                subdomain proofs"
            ),
            Self::Modulus(modulus) => format!(
                "`{ty}`'s domain repeats with period {modulus}, above the \
                subdomain proof limit of {MAX_MODULUS}"
            ),
            Self::Evaluations(evaluations) => format!(
                "`{ty}`'s domain needs {evaluations} evaluations, above the \
                subdomain proof limit of {MAX_EVALUATIONS}"
            ),
            Self::Pieces(pieces) => format!(
                "`{ty}`'s domain has {pieces} pieces, above the subdomain \
                proof limit of {MAX_PIECES}"
            ),
        };
    }
}

impl Form {
    /// Every value of `subject` at width `index`.
    pub(crate) fn full(
        subject: N,
        index: usize,
    ) -> Self {
        let (lo, hi) = subject.bounds_for_pointer_width(POINTER_WIDTHS[index]);
        return Self {
            modulus: 1,
            pieces: vec![Piece { residue: 0, lo, hi }],
        };
    }

    /// The pieces as the shared check takes them.
    pub(crate) fn data(&self) -> Vec<__TypekinWithinPiece> {
        return self
            .pieces
            .iter()
            .map(|it| {
                return (
                    it.residue,
                    it.lo.is_negative(),
                    it.lo.magnitude(),
                    it.hi.is_negative(),
                    it.hi.magnitude(),
                );
            })
            .collect();
    }

    /// `(modulus, &[(residue, lo_negative, lo, hi_negative, hi), ...])`.
    pub(crate) fn tokens(&self) -> TokenStream {
        let modulus = Literal::u128_suffixed(self.modulus);
        let pieces = self.data().into_iter().map(|(residue, a, b, c, d)| {
            let residue = Literal::u128_suffixed(residue);
            let (b, d) = (Literal::u128_suffixed(b), Literal::u128_suffixed(d));
            return quote! { (#residue, #a, #b, #c, #d) };
        });
        return quote! { (#modulus, &[#(#pieces),*]) };
    }
}

/// The shared check over two forms, with the target's raw range.
pub(crate) fn subset(
    source: &Form,
    target: &Form,
    (min, max): (Int, Int),
) -> __TypekinWithin {
    let value = |it: Int| (it.is_negative(), it.magnitude());
    return __typekin_within_subset(
        (source.modulus, &source.data()),
        (target.modulus, &target.data()),
        value(min),
        value(max),
    );
}

/// The shared check's items (`zz_domain_within_const`), verbatim, for a
/// target's generated scope.
pub(crate) fn emit_check() -> TokenStream {
    return include_str!("zz_domain_within_const.rs")
        .parse()
        .expect("the shared check is valid Rust");
}

impl ValidationCfg {
    /// The values this validation admits at width `index`: every value
    /// without validation, the resolved domain's form without callbacks.
    pub(crate) fn form(
        &self,
        subject: N,
        index: usize,
    ) -> Result<Form, Opaque> {
        if !self.callbacks.is_empty() {
            let callbacks = self
                .callbacks
                .iter()
                .map(|it| it.to_token_stream().to_string().replace(' ', ""))
                .collect::<Vec<_>>();
            return Err(Opaque::Callbacks(match callbacks.as_slice() {
                [callback] => callback.clone(),
                _ => format!("[{}]", callbacks.join(", ")),
            }));
        }
        return match self.resolved() {
            Some(domain) => domain.form(index),
            None => Ok(Form::full(subject, index)),
        };
    }
}

impl ResolvedDomain {
    /// The domain at width `index` as a form, or why it has none.
    pub(crate) fn form(
        &self,
        index: usize,
    ) -> Result<Form, Opaque> {
        let (starts, modulus) = self.segments(index)?;
        let (_, max) =
            self.subject.bounds_for_pointer_width(POINTER_WIDTHS[index]);
        return sample(&self.per_width[index], index, (&starts, max), modulus);
    }

    /// The segment starts at width `index`, from the type's minimum, and
    /// the modulus every leaf repeats with inside a segment.
    pub(crate) fn segments(
        &self,
        index: usize,
    ) -> Result<(Vec<Int>, u128), Opaque> {
        let mut shape = Shape {
            breakpoints: BTreeSet::new(),
            period: 1,
            subject: self.subject,
            index,
        };
        shape.visit(&self.per_width[index])?;
        let (min, max) =
            self.subject.bounds_for_pointer_width(POINTER_WIDTHS[index]);
        let mut starts = vec![min];
        starts.extend(
            shape
                .breakpoints
                .range((Limit::Excluded(min), Limit::Included(max)))
                .copied(),
        );
        return Ok((starts, shape.period));
    }
}

/// Where a domain's leaves may change truth at one width, and the period
/// they repeat with in between.
struct Shape {
    breakpoints: BTreeSet<Int>,
    period: u128,
    subject: N,
    index: usize,
}

impl Shape {
    fn bits(&self) -> u32 {
        return self
            .subject
            .bits_for_pointer_width(POINTER_WIDTHS[self.index])
            as u32;
    }

    /// A breakpoint; `None` (an overflowing one) lies beyond every value.
    fn at(
        &mut self,
        value: Option<Int>,
    ) {
        if let Some(value) = value {
            self.breakpoints.insert(value);
        }
    }

    fn repeat(
        &mut self,
        period: u128,
    ) -> Result<(), Opaque> {
        if period > MAX_MODULUS {
            return Err(Opaque::Modulus(period));
        }
        let mut divisor = (self.period, period);
        while divisor.1 != 0 {
            divisor = (divisor.1, divisor.0 % divisor.1);
        }
        self.period = self.period / divisor.0 * period;
        if self.period > MAX_MODULUS {
            return Err(Opaque::Modulus(self.period));
        }
        return Ok(());
    }

    fn visit(
        &mut self,
        node: &RNode,
    ) -> Result<(), Opaque> {
        return match node {
            RNode::Const(_) => Ok(()),
            RNode::Or(items) | RNode::And(items) => {
                items.iter().try_for_each(|it| self.visit(it))
            }
            RNode::Not(inner) => self.visit(inner),
            RNode::Predicate { function, .. } => {
                self.predicate(*function);
                Ok(())
            }
            RNode::Interval { operand, .. }
            | RNode::Ne { operand, .. }
            | RNode::Divisible { operand, .. } => self.leaf(node, operand),
        };
    }

    fn predicate(
        &mut self,
        function: FnName,
    ) {
        let zero = Int::NonNegative(0);
        if function == FnName::IsPowerOfTwo {
            for exponent in 0..self.bits() {
                let power = Int::NonNegative(1 << exponent);
                self.at(Some(power));
                self.at(power.successor());
            }
            return;
        }
        // `is_positive`, `is_negative`.
        self.at(Some(zero));
        self.at(zero.successor());
    }

    /// A comparison on the value or on one lens application.
    fn leaf(
        &mut self,
        node: &RNode,
        operand: &Operand,
    ) -> Result<(), Opaque> {
        let Some(lens) = operand.lens()
        else {
            match boundaries(node) {
                Some(values) => values.into_iter().for_each(|it| self.at(it)),
                None => self.repeat(divisor(node))?,
            }
            return Ok(());
        };
        match lens {
            LensKind::CountOnes | LensKind::CountZeros => {
                return Err(Opaque::Lens(lens.function()));
            }
            LensKind::Signum => {
                let zero = Int::NonNegative(0);
                self.at(Some(zero));
                self.at(zero.successor());
            }
            LensKind::RemEuclid => {
                self.repeat(operand.arg(self.index).magnitude())?;
            }
            LensKind::LeadingZeros | LensKind::LeadingOnes => self.leading(),
            LensKind::TrailingZeros | LensKind::TrailingOnes => {
                self.trailing(lens, node)?;
            }
            LensKind::UnsignedAbs
            | LensKind::AbsDiff
            | LensKind::DivEuclid
            | LensKind::Isqrt => self.monotone(lens, node, operand)?,
        }
        return Ok(());
    }

    /// Lenses monotone on each side of a point: `abs`, `abs_diff(a)` (around
    /// `0` and `a`), `div_euclid(k)`, and `isqrt` (non-decreasing). A lens
    /// boundary `c` (where the test changes between `c - 1` and `c`) maps to
    /// the values where the lens reaches `c`.
    fn monotone(
        &mut self,
        lens: LensKind,
        node: &RNode,
        operand: &Operand,
    ) -> Result<(), Opaque> {
        let one = Int::NonNegative(1);
        let Some(values) = boundaries(node)
        else {
            let divisor = divisor(node);
            return match lens {
                LensKind::Isqrt => Err(Opaque::IsqrtMultiples),
                // `d | floor(x / k)` depends on `x mod k·d`.
                LensKind::DivEuclid => {
                    let divisor = operand
                        .arg(self.index)
                        .magnitude()
                        .saturating_mul(divisor);
                    self.repeat(divisor)
                }
                // `d | |x - a|` iff `x ≡ a (mod d)`.
                _ => self.repeat(divisor),
            };
        };
        let center = match lens {
            LensKind::AbsDiff => operand.arg(self.index),
            _ => Int::NonNegative(0),
        };
        self.at(Some(center));
        for value in values.into_iter().flatten() {
            match lens {
                // `|x - a| >= c` iff `x >= a + c` or `x <= a - c`.
                LensKind::UnsignedAbs | LensKind::AbsDiff => {
                    self.at(center.checked_add(value));
                    self.at(center
                        .checked_sub(value)
                        .and_then(|it| it.checked_add(one)));
                }
                // `x.div_euclid(k) >= c` iff `x >= c * k`.
                LensKind::DivEuclid => {
                    self.at(value.checked_mul(operand.arg(self.index)));
                }
                // `x.isqrt() >= c` iff `x >= c * c`, for `c >= 0`.
                _ => self.at(value.checked_mul(value)),
            }
        }
        return Ok(());
    }

    /// `leading_zeros` and `leading_ones` are constant between the
    /// power-of-two boundaries of the bit pattern: `2^j` and `2^bits - 2^j`.
    fn leading(&mut self) {
        let bits = self.bits();
        let mask = mask(bits);
        let signed = self.subject.is_signed();
        self.at(Some(Int::NonNegative(0)));
        for exponent in 0..bits {
            let low = 1u128 << exponent;
            for pattern in [
                low,
                mask - (low - 1),
            ] {
                self.at(Some(from_pattern(pattern, bits, signed)));
            }
        }
    }

    /// Away from its special value (zero, or all ones), `trailing_zeros(x)`
    /// is `k` iff `x ≡ 2^k (mod 2^(k + 1))`, and `trailing_ones` likewise for
    /// `!x`: when the test agrees on every result from `K + 1` up, the truth
    /// depends on `x mod 2^(K + 1)` alone.
    fn trailing(
        &mut self,
        lens: LensKind,
        node: &RNode,
    ) -> Result<(), Opaque> {
        let bits = self.bits();
        let special = match (lens, self.subject.is_signed()) {
            (LensKind::TrailingZeros, _) => Int::NonNegative(0),
            (_, true) => Int::Negative(1),
            (_, false) => Int::NonNegative(mask(bits)),
        };
        self.at(Some(special));
        self.at(special.successor());
        let holds =
            |it: u32| test_holds(node, Int::NonNegative(u128::from(it)));
        let last_change =
            (0..bits - 1).rev().find(|it| holds(*it) != holds(*it + 1));
        if let Some(last_change) = last_change {
            self.repeat(1 << (last_change + 1))?;
        }
        return Ok(());
    }
}

/// The form of `node` at width `index`, whose values end at `max`: one
/// evaluation per segment and residue class decides the class on the
/// segment, and same-class pieces of adjacent segments merge.
fn sample(
    node: &RNode,
    index: usize,
    (starts, max): (&[Int], Int),
    modulus: u128,
) -> Result<Form, Opaque> {
    let evaluations = (starts.len() as u128).saturating_mul(modulus);
    if evaluations > MAX_EVALUATIONS {
        return Err(Opaque::Evaluations(evaluations));
    }

    let mut classes = vec![Vec::<(Int, Int)>::new(); modulus as usize];
    for (position, start) in starts.iter().enumerate() {
        let end = match starts.get(position + 1) {
            Some(next) => next.predecessor().expect("a later start"),
            None => max,
        };
        for (class, residue) in classes.iter_mut().zip(0..) {
            let Some(first) =
                first_member(*start, residue, modulus).filter(|it| *it <= end)
            else {
                continue;
            };
            if !eval(node, first, index) {
                continue;
            }
            let last = last_member(end, residue, modulus);
            match class.last_mut() {
                Some(previous)
                    if previous
                        .1
                        .checked_add(Int::NonNegative(modulus))
                        .is_some_and(|it| it == first) =>
                {
                    previous.1 = last;
                }
                _ => class.push((first, last)),
            }
        }
    }

    let pieces = classes
        .into_iter()
        .zip(0..)
        .flat_map(|(class, residue)| {
            return class.into_iter().map(move |(lo, hi)| Piece {
                residue,
                lo,
                hi,
            });
        })
        .collect::<Vec<_>>();
    if pieces.len() > MAX_PIECES {
        return Err(Opaque::Pieces(pieces.len()));
    }
    return Ok(Form { modulus, pieces });
}

/// The values `c` where an interval or `!{c}` test changes between `c - 1`
/// and `c`; `None` (overflowing) ones lie beyond every value. `None` for a
/// `mul(d)` test, whose truth is periodic.
fn boundaries(node: &RNode) -> Option<Vec<Option<Int>>> {
    return match node {
        RNode::Interval { lo, hi, .. } => Some(vec![
            lo.as_ref().map(|it| it.value),
            hi.as_ref().and_then(|it| it.value.successor()),
        ]),
        RNode::Ne { value, .. } => Some(vec![
            Some(*value),
            value.successor(),
        ]),
        _ => None,
    };
}

/// The divisor of a `mul(d)` test.
fn divisor(node: &RNode) -> u128 {
    let RNode::Divisible { divisor, .. } = node
    else {
        unreachable!("only `mul(d)` tests are periodic");
    };
    return divisor.magnitude();
}

fn mask(bits: u32) -> u128 {
    return match bits {
        u128::BITS => u128::MAX,
        bits => (1 << bits) - 1,
    };
}

/// The value of a `bits`-bit two's-complement pattern.
fn from_pattern(
    pattern: u128,
    bits: u32,
    signed: bool,
) -> Int {
    if !signed || (pattern >> (bits - 1)) & 1 == 0 {
        return Int::NonNegative(pattern);
    }
    return Int::Negative((!pattern & mask(bits)) + 1);
}

/// The least member of `residue`'s class from `start`; `None` beyond every
/// `Int`.
fn first_member(
    start: Int,
    residue: u128,
    modulus: u128,
) -> Option<Int> {
    let offset = Int::NonNegative(residue)
        .checked_sub(start)?
        .checked_rem_euclid(Int::NonNegative(modulus))?;
    return start.checked_add(offset);
}

/// The greatest member of `residue`'s class up to `end`.
fn last_member(
    end: Int,
    residue: u128,
    modulus: u128,
) -> Int {
    let offset = end
        .checked_sub(Int::NonNegative(residue))
        .and_then(|it| it.checked_rem_euclid(Int::NonNegative(modulus)))
        .expect("type values minus a residue fit");
    return end.checked_sub(offset).expect("type values fit");
}
