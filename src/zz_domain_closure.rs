//! Closure proofs: whether every result of an operation on valid operands is
//! valid, decided per pointer width with interval arithmetic over the
//! resolved domain.
//!
//! A domain is eligible when, at every width, it is a union of intervals on
//! the value itself. Each operand interval is split at zero, every pair of
//! sub-intervals gets a sound result hull (exact corners where the operation
//! is monotone on the pair), the arithmetic policy maps the hull into the
//! type, and the operation is proven when every hull lies in one domain
//! interval. A wrong proof reaches `new_unchecked`, so every rule must stay a
//! sound over-approximation; anything else is "not proven". When the proof
//! fails, concrete operands near interval endpoints are evaluated exactly to
//! find a counterexample.

use proc_macro2::TokenStream;

use crate::zz_domain_int::{
    Bound,
    Operand,
    RNode,
    ResolvedDomain,
};
use crate::zz_values::{
    Int,
    N,
    POINTER_WIDTHS,
};

/// Above this many intervals per operand, no proof is attempted.
const MAX_INTERVALS: usize = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClosureOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    DivEuclid,
    RemEuclid,
    BitAnd,
    BitOr,
    BitXor,
    /// Any shift amount: `>>`, `checked_shr`, `wrapping_shr`, `strict_shr`,
    /// `overflowing_shr`.
    Shr,
    /// Amounts at or beyond the bit width give 0 (non-negative) or -1
    /// (negative).
    UnboundedShr,
    Neg,
    Not,
    Abs,
    Midpoint,
    Isqrt,
}

/// What happens when the exact mathematical result does not fit the type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Policy {
    /// Overflow yields no value (panic or `None`). Also: division/remainder
    /// by zero, signed `MIN / -1` and `MIN % -1`, `isqrt` of a negative,
    /// `abs`/`neg` of MIN, shift amount >= BITS.
    Checked,
    /// Results wrap modulo 2^BITS (`wrapping_*`, `Wrapping<T>`, and plain
    /// operators, which may wrap in release). Division by zero still yields
    /// no value; `MIN / -1` = MIN, `MIN % -1` = 0; shift amounts are masked.
    Wrapping,
    /// Results clamp to the type range (`saturating_*`). Division by zero
    /// yields no value. Shifts behave as under `Checked`.
    Saturating,
}

/// The right-hand operand's possible values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Rhs {
    /// Another value of the same wrapper: any value in the domain.
    SameDomain,
    /// A raw primitive or friend relation: any value of the subject type.
    Full,
    /// A shift amount (any non-negative amount); only for Shr/UnboundedShr.
    Shift,
    /// Unary operations (Neg, Not, Abs, Isqrt).
    Unary,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Closure {
    /// Every result of a valid operand is valid, at every pointer width.
    Proven,
    /// The domain is not a union of plain intervals on the value; holds the
    /// first opaque atom rendered approximately as written (e.g. `!{10}`,
    /// `mul(3)`, `!mul(3)`, `unsigned_abs in ..=1000`, `is_power_of_two`,
    /// `!(...)`).
    Ineligible(String),
    /// A concrete counterexample: operands in range, result outside the
    /// domain. `all_widths`: the same operands give the same invalid result
    /// at every pointer width.
    Refuted {
        lhs: Int,
        rhs: Option<Int>,
        result: Int,
        width: u32,
        all_widths: bool,
    },
    /// No proof and no counterexample found: results may reach `lo..=hi`.
    Unknown { lo: Int, hi: Int, width: u32 },
}

impl ClosureOp {
    fn is_unary(self) -> bool {
        return matches!(self, Self::Neg | Self::Not | Self::Abs | Self::Isqrt);
    }

    fn is_shift(self) -> bool {
        return matches!(self, Self::Shr | Self::UnboundedShr);
    }

    /// The divisor must not be zero.
    fn divides(self) -> bool {
        return matches!(
            self,
            Self::Div | Self::Rem | Self::DivEuclid | Self::RemEuclid
        );
    }

    /// The exact result can leave the type's range.
    fn overflows(self) -> bool {
        return matches!(
            self,
            Self::Add
                | Self::Sub
                | Self::Mul
                | Self::Div
                | Self::DivEuclid
                | Self::Neg
                | Self::Abs
        );
    }
}

// =============================================================================
// The subject type at one pointer width

#[derive(Clone, Copy)]
struct Ty {
    signed: bool,
    bits: u32,
    min: Int,
    max: Int,
}

impl Ty {
    fn new(
        subject: N,
        width: u32,
    ) -> Self {
        let (min, max) = subject.bounds_for_pointer_width(width);
        return Self {
            signed: subject.is_signed(),
            bits: subject.bits_for_pointer_width(width) as u32,
            min,
            max,
        };
    }

    fn contains(
        self,
        value: Int,
    ) -> bool {
        return self.min <= value && value <= self.max;
    }

    /// The value of a two's-complement bit pattern reduced to this type.
    fn wrap(
        self,
        pattern: u128,
    ) -> Int {
        let masked = match self.bits {
            u128::BITS => pattern,
            bits => pattern & ((1u128 << bits) - 1),
        };
        if !self.signed || (masked >> (self.bits - 1)) & 1 == 0 {
            return Int::NonNegative(masked);
        }
        return Int::Negative(match self.bits {
            u128::BITS => masked.wrapping_neg(),
            bits => (1u128 << bits) - masked,
        });
    }

    fn clamp(
        self,
        value: Ext,
    ) -> Int {
        return match value {
            Ext::Below => self.min,
            Ext::Above => self.max,
            Ext::Exact(it) => it.clamp(self.min, self.max),
        };
    }
}

// =============================================================================
// Exact semantics

/// A result bound: exact, or beyond every `Int` in one direction.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Ext {
    Below,
    Exact(Int),
    Above,
}

impl Ext {
    fn beyond(negative: bool) -> Self {
        return match negative {
            true => Self::Below,
            false => Self::Above,
        };
    }

    fn exact(self) -> Int {
        let Self::Exact(it) = self
        else {
            unreachable!("bound clamped to the type");
        };
        return it;
    }
}

/// The mathematical result of a non-shift operation, before the policy;
/// `None` when the operation has no value (zero divisor, `isqrt` of a
/// negative).
fn exact(
    op: ClosureOp,
    ty: Ty,
    lhs: Int,
    rhs: Option<Int>,
) -> Option<Ext> {
    let right = || rhs.expect("binary operation has a right-hand operand");
    return match op {
        ClosureOp::Add => Some(match lhs.checked_add(right()) {
            Some(it) => Ext::Exact(it),
            None => Ext::beyond(lhs.is_negative()),
        }),
        ClosureOp::Sub => Some(match lhs.checked_sub(right()) {
            Some(it) => Ext::Exact(it),
            None => Ext::beyond(lhs.is_negative()),
        }),
        ClosureOp::Mul => Some(match lhs.checked_mul(right()) {
            Some(it) => Ext::Exact(it),
            None => Ext::beyond(lhs.is_negative() != right().is_negative()),
        }),
        ClosureOp::Div => lhs.checked_div(right()).map(Ext::Exact),
        ClosureOp::Rem => lhs.checked_rem(right()).map(Ext::Exact),
        ClosureOp::DivEuclid => lhs.checked_div_euclid(right()).map(Ext::Exact),
        ClosureOp::RemEuclid => lhs.checked_rem_euclid(right()).map(Ext::Exact),
        ClosureOp::BitAnd => {
            Some(Ext::Exact(ty.wrap(lhs.pattern() & right().pattern())))
        }
        ClosureOp::BitOr => {
            Some(Ext::Exact(ty.wrap(lhs.pattern() | right().pattern())))
        }
        ClosureOp::BitXor => {
            Some(Ext::Exact(ty.wrap(lhs.pattern() ^ right().pattern())))
        }
        ClosureOp::Neg => Some(Ext::Exact(lhs.negate())),
        ClosureOp::Not => Some(Ext::Exact(ty.wrap(!lhs.pattern()))),
        ClosureOp::Abs => Some(Ext::Exact(Int::NonNegative(lhs.magnitude()))),
        // Signed rounds towards zero, unsigned down; never overflows.
        ClosureOp::Midpoint => Some(Ext::Exact(match ty.signed {
            true => Int::from_i128(lhs.to_i128().midpoint(right().to_i128())),
            false => {
                Int::NonNegative(lhs.magnitude().midpoint(right().magnitude()))
            }
        })),
        ClosureOp::Isqrt => lhs.isqrt().map(Ext::Exact),
        ClosureOp::Shr | ClosureOp::UnboundedShr => {
            unreachable!("shifts are evaluated by `shift`")
        }
    };
}

fn shift(
    op: ClosureOp,
    policy: Policy,
    ty: Ty,
    lhs: Int,
    amount: Int,
) -> Option<Int> {
    let bits = Int::NonNegative(ty.bits as u128);
    let in_range = !amount.is_negative() && amount < bits;
    let amount = match (op, policy) {
        (ClosureOp::Shr, Policy::Wrapping) => amount
            .checked_rem_euclid(bits)
            .expect("nonzero width")
            .magnitude() as u32,
        (ClosureOp::Shr, _) => match in_range {
            true => amount.magnitude() as u32,
            false => return None,
        },
        // A negative amount converted to `u32` is at least the width.
        _ => match in_range {
            true => amount.magnitude() as u32,
            false => ty.bits,
        },
    };
    return Some(lhs.shr_floor(amount));
}

/// Exact semantics of one operation on concrete operands of `subject` at
/// `width`; `None` when the operation yields no value (panic or `None`).
/// `rhs` is `None` for unary operations and the amount for shifts.
pub(crate) fn eval(
    op: ClosureOp,
    policy: Policy,
    subject: N,
    width: u32,
    lhs: Int,
    rhs: Option<Int>,
) -> Option<Int> {
    let ty = Ty::new(subject, width);
    if op.is_shift() {
        return shift(op, policy, ty, lhs, rhs.expect("shift amount"));
    }
    if policy == Policy::Wrapping {
        let (left, right) = (lhs.pattern(), rhs.map(Int::pattern));
        let wrapped = match op {
            ClosureOp::Add => right.map(|it| left.wrapping_add(it)),
            ClosureOp::Sub => right.map(|it| left.wrapping_sub(it)),
            ClosureOp::Mul => right.map(|it| left.wrapping_mul(it)),
            _ => None,
        };
        if let Some(it) = wrapped {
            return Some(ty.wrap(it));
        }
    }

    let value = exact(op, ty, lhs, rhs)?;
    if policy == Policy::Checked
        && matches!(op, ClosureOp::Rem | ClosureOp::RemEuclid)
        && ty.signed
        && lhs == ty.min
        && rhs == Some(Int::Negative(1))
    {
        return None;
    }
    return match policy {
        Policy::Checked => match value {
            Ext::Exact(it) if ty.contains(it) => Some(it),
            _ => None,
        },
        Policy::Wrapping => Some(ty.wrap(value.exact().pattern())),
        Policy::Saturating => Some(ty.clamp(value)),
    };
}

// =============================================================================
// Eligibility

impl ResolvedDomain {
    /// Every result of `op` on valid operands is valid at every width, a
    /// counterexample, or neither.
    pub(crate) fn closure(
        &self,
        op: ClosureOp,
        rhs: Rhs,
        policy: Policy,
    ) -> Closure {
        match rhs {
            Rhs::Unary => assert!(op.is_unary(), "{op:?} is binary"),
            Rhs::Shift => assert!(op.is_shift(), "{op:?} is not a shift"),
            Rhs::SameDomain | Rhs::Full => {
                assert!(!op.is_unary(), "{op:?} is unary");
            }
        }

        let mut domains = Vec::with_capacity(POINTER_WIDTHS.len());
        for (index, width) in POINTER_WIDTHS.into_iter().enumerate() {
            let ty = Ty::new(self.subject, width);
            match intervals(&self.per_width[index], ty) {
                Ok(it) => domains.push(merge(it)),
                Err(atom) => return Closure::Ineligible(atom),
            }
        }

        let analysis = Analysis {
            op,
            rhs,
            policy,
            subject: self.subject,
            domains,
        };
        let failures = (0..POINTER_WIDTHS.len())
            .filter_map(|index| {
                return analysis.prove(index).err().map(|hull| (index, hull));
            })
            .collect::<Vec<_>>();
        if failures.is_empty() {
            return Closure::Proven;
        }
        for (index, _) in &failures {
            if let Some(refuted) = analysis.refute(*index) {
                return refuted;
            }
        }
        let (index, (lo, hi)) = failures[0];
        return Closure::Unknown {
            lo,
            hi,
            width: POINTER_WIDTHS[index],
        };
    }
}

/// The domain at one width as inclusive intervals on the value, or the first
/// atom that is not one.
fn intervals(
    node: &RNode,
    ty: Ty,
) -> Result<Vec<(Int, Int)>, String> {
    return match node {
        RNode::Const(true) => Ok(vec![(ty.min, ty.max)]),
        RNode::Const(false) => Ok(Vec::new()),
        RNode::Interval { operand, lo, hi } if operand.is_subject() => {
            Ok(vec![
                (
                    lo.as_ref().map_or(ty.min, |it| it.value),
                    hi.as_ref().map_or(ty.max, |it| it.value),
                ),
            ])
        }
        RNode::Or(items) => {
            let mut result = Vec::new();
            for item in items {
                result.extend(intervals(item, ty)?);
            }
            Ok(result)
        }
        RNode::And(items) => Err(items
            .iter()
            .find_map(|it| intervals(it, ty).err())
            .unwrap_or_else(|| render(node))),
        _ => Err(render(node)),
    };
}

/// Sorted, disjoint, non-adjacent intervals.
fn merge(mut intervals: Vec<(Int, Int)>) -> Vec<(Int, Int)> {
    intervals.retain(|(lo, hi)| lo <= hi);
    intervals.sort();
    let mut result = Vec::<(Int, Int)>::with_capacity(intervals.len());
    for (lo, hi) in intervals {
        if let Some(last) = result.last_mut()
            && last.1.successor().is_none_or(|it| lo <= it)
        {
            last.1 = last.1.max(hi);
            continue;
        }
        result.push((lo, hi));
    }
    return result;
}

fn covers(
    domain: &[(Int, Int)],
    value: Int,
) -> bool {
    return domain.iter().any(|(lo, hi)| *lo <= value && value <= *hi);
}

// -----------------------------------------------------------------------------
// Rendering of the blocking atom

fn text(tokens: &TokenStream) -> String {
    let text = tokens.to_string().replace(" :: ", "::");
    return match text.strip_prefix("- ") {
        Some(rest) => format!("-{rest}"),
        None => text,
    };
}

/// `lo..hi` as written; a lower end that `!{c}` made exclusive (N8) adds
/// `& !{c}`.
fn render_range(
    lo: &Option<Bound>,
    hi: &Option<Bound>,
) -> String {
    let hi_part = |it: &Bound| {
        let op = if it.inclusive { "..=" } else { ".." };
        return format!("{op}{}", text(&it.tokens));
    };
    return match (lo, hi) {
        (Some(lo), Some(hi)) if lo.value == hi.value && lo.inclusive => {
            format!("{{{}}}", text(&lo.tokens))
        }
        (Some(lo), hi) => {
            let range = format!(
                "{}{}",
                text(&lo.tokens),
                hi.as_ref().map_or(String::from(".."), hi_part)
            );
            match lo.inclusive {
                true => range,
                false => format!("{range} & !{{{}}}", text(&lo.tokens)),
            }
        }
        (None, Some(hi)) => hi_part(hi),
        (None, None) => String::from(".."),
    };
}

/// Lists parenthesize what would otherwise mix `|` and `&` or follow an
/// image: nested lists, images, and ranges rendered with `& !{c}`.
fn render_operand(node: &RNode) -> String {
    let compound = match node {
        RNode::Or(_) | RNode::And(_) => true,
        RNode::Interval { operand, lo, .. } => {
            !operand.is_subject() || lo.as_ref().is_some_and(|it| !it.inclusive)
        }
        RNode::Ne { operand, .. } | RNode::Divisible { operand, .. } => {
            !operand.is_subject()
        }
        _ => false,
    };
    return match compound {
        true => format!("({})", render(node)),
        false => render(node),
    };
}

/// A resolved node, approximately as written.
fn render(node: &RNode) -> String {
    let image = |operand: &Operand, values: String| {
        return match (operand.is_subject(), values.contains(" & ")) {
            (true, _) => values,
            (false, false) => format!("{} in {values}", operand.key.text()),
            (false, true) => format!("{} in ({values})", operand.key.text()),
        };
    };
    return match node {
        RNode::Const(value) => value.to_string(),
        RNode::Or(items) => items
            .iter()
            .map(render_operand)
            .collect::<Vec<_>>()
            .join(" | "),
        RNode::And(items) => items
            .iter()
            .map(render_operand)
            .collect::<Vec<_>>()
            .join(" & "),
        RNode::Not(inner) => format!("!({})", render(inner)),
        RNode::Interval { operand, lo, hi } => {
            image(operand, render_range(lo, hi))
        }
        RNode::Ne {
            operand, tokens, ..
        } => image(operand, format!("!{{{}}}", text(tokens))),
        RNode::Divisible {
            operand,
            tokens,
            negated,
            ..
        } => {
            let bang = if *negated { "!" } else { "" };
            image(operand, format!("{bang}mul({})", text(tokens)))
        }
        RNode::Predicate { name, .. } => name.to_string(),
    };
}

// =============================================================================
// Interval analysis

struct Analysis {
    op: ClosureOp,
    rhs: Rhs,
    policy: Policy,
    subject: N,
    /// Merged domain intervals per pointer width.
    domains: Vec<Vec<(Int, Int)>>,
}

impl Analysis {
    fn rhs_set(
        &self,
        index: usize,
        ty: Ty,
    ) -> Vec<(Int, Int)> {
        return match self.rhs {
            Rhs::SameDomain => self.domains[index].clone(),
            Rhs::Full => vec![(ty.min, ty.max)],
            Rhs::Shift => {
                vec![(Int::NonNegative(0), Int::NonNegative(u128::MAX))]
            }
            Rhs::Unary => Vec::new(),
        };
    }

    /// `Ok` when every result at this width is valid; otherwise the hull of
    /// the possible results.
    fn prove(
        &self,
        index: usize,
    ) -> Result<(), (Int, Int)> {
        let ty = Ty::new(self.subject, POINTER_WIDTHS[index]);
        let domain = &self.domains[index];
        // Every operation yields a value of the type, if any.
        if domain.is_empty() || domain[..] == [(ty.min, ty.max)] {
            return Ok(());
        }
        let rhs = self.rhs_set(index, ty);
        if domain.len() > MAX_INTERVALS || rhs.len() > MAX_INTERVALS {
            return Err((ty.min, ty.max));
        }

        let hulls = self.hulls(ty, domain, &rhs).ok_or((ty.min, ty.max))?;
        if hulls.iter().all(|(lo, hi)| {
            return domain.iter().any(|(dl, dh)| dl <= lo && hi <= dh);
        }) {
            return Ok(());
        }
        let lo = hulls.iter().map(|it| it.0).min().expect("a failing hull");
        let hi = hulls.iter().map(|it| it.1).max().expect("a failing hull");
        return Err((lo, hi));
    }

    /// Result hulls within the type, one or two per operand pair; `None`
    /// when results cannot be bounded (wrapping overflow).
    fn hulls(
        &self,
        ty: Ty,
        domain: &[(Int, Int)],
        rhs: &[(Int, Int)],
    ) -> Option<Vec<(Int, Int)>> {
        let lhs = split(domain);
        let mut result = Vec::new();

        if self.op.is_shift() {
            let Some((min, max)) = self.amounts(ty, rhs)
            else {
                return Some(result);
            };
            for (lo, hi) in lhs {
                result.push(match lo.is_negative() {
                    false => (lo.shr_floor(max), hi.shr_floor(min)),
                    true => (lo.shr_floor(min), hi.shr_floor(max)),
                });
            }
            return Some(result);
        }

        let rhs = match self.op.divides() {
            true => split(rhs)
                .into_iter()
                .filter_map(|(lo, hi)| match lo == Int::NonNegative(0) {
                    false => Some((lo, hi)),
                    true => (hi != Int::NonNegative(0))
                        .then_some((Int::NonNegative(1), hi)),
                })
                .collect::<Vec<_>>(),
            false => split(rhs),
        };
        for a in &lhs {
            if self.op.is_unary() {
                self.settle(ty, unary(self.op, ty, *a), &mut result)?;
                continue;
            }
            for b in &rhs {
                self.settle(ty, binary(self.op, ty, *a, *b), &mut result)?;
            }
        }
        return Some(result);
    }

    /// Effective shift amounts `min..=max`; `None` when no amount yields a
    /// value.
    fn amounts(
        &self,
        ty: Ty,
        rhs: &[(Int, Int)],
    ) -> Option<(u32, u32)> {
        let bits = Int::NonNegative(ty.bits as u128);
        let last = Int::NonNegative(ty.bits as u128 - 1);
        let in_range = |(lo, hi): &(Int, Int)| {
            return !lo.is_negative() && *hi <= last;
        };
        let small = |it: Int| it.magnitude() as u32;
        let ranges = match (self.op, self.policy) {
            // Masked: any amount below the width unless all are already.
            (ClosureOp::Shr, Policy::Wrapping) => {
                match rhs.iter().all(in_range) {
                    true => rhs.to_vec(),
                    false => vec![(Int::NonNegative(0), last)],
                }
            }
            (ClosureOp::Shr, _) => rhs
                .iter()
                .map(|(lo, hi)| {
                    ((*lo).max(Int::NonNegative(0)), (*hi).min(last))
                })
                .filter(|(lo, hi)| lo <= hi)
                .collect(),
            // Amounts at or beyond the width, and negative ones, act as the
            // width itself.
            _ => rhs
                .iter()
                .flat_map(|(lo, hi)| {
                    let mut parts = Vec::with_capacity(2);
                    if lo.is_negative() {
                        parts.push((bits, bits));
                    }
                    if !hi.is_negative() {
                        parts.push((
                            (*lo).max(Int::NonNegative(0)).min(bits),
                            (*hi).min(bits),
                        ));
                    }
                    return parts;
                })
                .collect(),
        };
        let min = ranges.iter().map(|it| it.0).min()?;
        let max = ranges.iter().map(|it| it.1).max()?;
        return Some((small(min), small(max)));
    }

    /// Applies the policy to a raw hull; `None` when wrapping makes it
    /// unbounded.
    fn settle(
        &self,
        ty: Ty,
        hull: Option<(Ext, Ext)>,
        result: &mut Vec<(Int, Int)>,
    ) -> Option<()> {
        let Some((lo, hi)) = hull
        else {
            return Some(());
        };
        let (min, max) = (Ext::Exact(ty.min), Ext::Exact(ty.max));
        if !self.op.overflows() || self.policy == Policy::Checked {
            let (lo, hi) = (lo.max(min), hi.min(max));
            if lo <= hi {
                result.push((lo.exact(), hi.exact()));
            }
            return Some(());
        }
        if self.policy == Policy::Saturating || (min <= lo && hi <= max) {
            result.push((ty.clamp(lo), ty.clamp(hi)));
            return Some(());
        }
        // Signed `MIN / -1`, `-MIN`, and `abs(MIN)` are exactly MAX + 1 and
        // wrap to MIN; nothing else of these operations overflows.
        let above = ty.max.successor().map(Ext::Exact);
        if matches!(
            self.op,
            ClosureOp::Div
                | ClosureOp::DivEuclid
                | ClosureOp::Neg
                | ClosureOp::Abs
        ) && ty.signed
            && min <= lo
            && Some(hi) == above
        {
            if lo <= max {
                result.push((lo.exact(), ty.max));
            }
            result.push((ty.min, ty.min));
            return Some(());
        }
        return None;
    }

    // -------------------------------------------------------------------------
    // Counterexamples

    fn candidates(
        &self,
        index: usize,
        ty: Ty,
        set: &[(Int, Int)],
        neighbors: bool,
    ) -> Vec<Int> {
        let mut values = vec![
            Int::NonNegative(0),
            Int::NonNegative(1),
            Int::NonNegative(2),
            Int::Negative(1),
            Int::Negative(2),
            ty.min,
            ty.max,
        ];
        values.extend(ty.min.successor());
        values.extend(ty.max.predecessor());
        let sources = match neighbors {
            true => &self.domains[index],
            false => set,
        };
        for (lo, hi) in sources.iter().take(MAX_INTERVALS) {
            values.extend([
                *lo,
                *hi,
            ]);
            values.extend(lo.successor());
            values.extend(hi.predecessor());
            if neighbors {
                values.extend(lo.predecessor());
                values.extend(hi.successor());
            }
        }
        values.retain(|it| covers(set, *it));
        values.sort();
        values.dedup();
        return values;
    }

    fn refute(
        &self,
        index: usize,
    ) -> Option<Closure> {
        let width = POINTER_WIDTHS[index];
        let ty = Ty::new(self.subject, width);
        let domain = &self.domains[index];
        let lhs = self.candidates(index, ty, domain, false);
        let rhs = match self.rhs {
            Rhs::Unary => vec![None],
            Rhs::Shift => (0..=ty.bits as u128 + 1)
                .map(|it| Some(Int::NonNegative(it)))
                .collect(),
            Rhs::SameDomain => self
                .candidates(index, ty, domain, false)
                .into_iter()
                .map(Some)
                .collect(),
            Rhs::Full => self
                .candidates(index, ty, &[(ty.min, ty.max)], true)
                .into_iter()
                .map(Some)
                .collect(),
        };

        let mut first = None;
        for l in &lhs {
            for r in &rhs {
                let Some(result) =
                    eval(self.op, self.policy, self.subject, width, *l, *r)
                else {
                    continue;
                };
                if covers(domain, result) {
                    continue;
                }
                let all_widths = self.refutes_everywhere(*l, *r, result);
                let refuted = Closure::Refuted {
                    lhs: *l,
                    rhs: *r,
                    result,
                    width,
                    all_widths,
                };
                if all_widths {
                    return Some(refuted);
                }
                first.get_or_insert(refuted);
            }
        }
        return first;
    }

    fn refutes_everywhere(
        &self,
        lhs: Int,
        rhs: Option<Int>,
        result: Int,
    ) -> bool {
        return POINTER_WIDTHS
            .into_iter()
            .enumerate()
            .all(|(index, width)| {
                let ty = Ty::new(self.subject, width);
                let domain = &self.domains[index];
                let rhs_valid = match (self.rhs, rhs) {
                    (Rhs::SameDomain, Some(it)) => covers(domain, it),
                    (Rhs::Full, Some(it)) => ty.contains(it),
                    (Rhs::Shift, Some(it)) => !it.is_negative(),
                    (Rhs::Unary, None) => true,
                    _ => false,
                };
                return covers(domain, lhs)
                    && rhs_valid
                    && eval(
                        self.op,
                        self.policy,
                        self.subject,
                        width,
                        lhs,
                        rhs,
                    ) == Some(result)
                    && !covers(domain, result);
            });
    }
}

/// Splits intervals at zero so every part has one sign.
fn split(intervals: &[(Int, Int)]) -> Vec<(Int, Int)> {
    let mut result = Vec::with_capacity(intervals.len() + 1);
    for (lo, hi) in intervals {
        match lo.is_negative() && !hi.is_negative() {
            true => {
                result.push((*lo, Int::Negative(1)));
                result.push((Int::NonNegative(0), *hi));
            }
            false => result.push((*lo, *hi)),
        }
    }
    return result;
}

/// Exact results at the corners of a sign-split pair.
fn corners(
    op: ClosureOp,
    ty: Ty,
    (al, ah): (Int, Int),
    (bl, bh): (Int, Int),
) -> Option<(Ext, Ext)> {
    let values = [
        (al, bl),
        (al, bh),
        (ah, bl),
        (ah, bh),
    ]
    .map(|(a, b)| exact(op, ty, a, Some(b)).expect("nonzero divisor"));
    return Some((
        *values.iter().min().expect("four corners"),
        *values.iter().max().expect("four corners"),
    ));
}

/// Raw result hull of a binary operation on a sign-split pair; `None` when
/// the pair yields no value.
fn binary(
    op: ClosureOp,
    ty: Ty,
    a: (Int, Int),
    b: (Int, Int),
) -> Option<(Ext, Ext)> {
    return match op {
        // Monotone in each operand (bilinear for `*`): corners are exact.
        ClosureOp::Add
        | ClosureOp::Sub
        | ClosureOp::Mul
        | ClosureOp::Div
        | ClosureOp::DivEuclid
        | ClosureOp::Midpoint => corners(op, ty, a, b),
        ClosureOp::Rem | ClosureOp::RemEuclid => rem_hull(op, a, b),
        ClosureOp::BitAnd => Some(bitand_hull(a, b)),
        ClosureOp::BitOr => Some(bitor_hull(a, b)),
        ClosureOp::BitXor => Some(bitxor_hull(a, b)),
        _ => unreachable!("{op:?} is not a binary operation"),
    };
}

fn rem_hull(
    op: ClosureOp,
    (al, ah): (Int, Int),
    (bl, bh): (Int, Int),
) -> Option<(Ext, Ext)> {
    let point = |it: Int| Ext::Exact(it);
    let sum = |it: Option<Int>| point(it.expect("in range of Int"));
    let (a_negative, b_negative) = (al.is_negative(), bl.is_negative());
    let (b_min, b_max) = match b_negative {
        false => (bl.magnitude(), bh.magnitude()),
        true => (bh.magnitude(), bl.magnitude()),
    };
    let b_min_int = Int::NonNegative(b_min);
    let b_max_less = Int::NonNegative(b_max - 1);
    return match (a_negative, op) {
        // |a| < |b|: the remainder is `a` itself.
        (false, _) if ah.magnitude() < b_min => Some((point(al), point(ah))),
        (false, _) => {
            Some((point(Int::NonNegative(0)), point(ah.min(b_max_less))))
        }
        (true, ClosureOp::Rem) if al.magnitude() < b_min => {
            Some((point(al), point(ah)))
        }
        (true, ClosureOp::Rem) => Some((
            point(Int::from_sign_magnitude(
                true,
                al.magnitude().min(b_max - 1),
            )),
            point(Int::NonNegative(0)),
        )),
        // |a| <= |b|: the remainder is `a + |b|`.
        (true, _) if al.magnitude() <= b_min => Some((
            sum(al.checked_add(b_min_int)),
            sum(ah.checked_add(Int::NonNegative(b_max))),
        )),
        (true, _) => Some((point(Int::NonNegative(0)), point(b_max_less))),
    };
}

fn bitand_hull(
    (al, ah): (Int, Int),
    (bl, bh): (Int, Int),
) -> (Ext, Ext) {
    let point = |it: Int| Ext::Exact(it);
    return match (al.is_negative(), bl.is_negative()) {
        (false, false) => (point(Int::NonNegative(0)), point(ah.min(bh))),
        (false, true) => (point(Int::NonNegative(0)), point(ah)),
        (true, false) => (point(Int::NonNegative(0)), point(bh)),
        // Bits above both lower bounds' ones stay set.
        (true, true) => {
            let bits = al
                .bitwise_not()
                .bitlen()
                .max(bl.bitwise_not().bitlen())
                .expect("negated negative values are non-negative");
            (point(Int::Negative(1u128 << bits)), point(ah.min(bh)))
        }
    };
}

fn bitor_hull(
    (al, ah): (Int, Int),
    (bl, bh): (Int, Int),
) -> (Ext, Ext) {
    let point = |it: Int| Ext::Exact(it);
    return match (al.is_negative(), bl.is_negative()) {
        (false, false) => (point(al.max(bl)), point(ah.max(bh).mask())),
        (false, true) => (point(bl), point(Int::Negative(1))),
        (true, false) => (point(al), point(Int::Negative(1))),
        (true, true) => (point(al.max(bl)), point(Int::Negative(1))),
    };
}

/// `a ^ b = !(a ^ !b)`, and `!x` of a negative is non-negative.
fn bitxor_hull(
    (al, ah): (Int, Int),
    (bl, bh): (Int, Int),
) -> (Ext, Ext) {
    let point = |it: Int| Ext::Exact(it);
    return match (al.is_negative(), bl.is_negative()) {
        (false, false) => {
            (point(Int::NonNegative(0)), point(ah.max(bh).mask()))
        }
        (true, true) => (
            point(Int::NonNegative(0)),
            point(al.bitwise_not().max(bl.bitwise_not()).mask()),
        ),
        (false, true) => (
            point(ah.max(bl.bitwise_not()).mask().bitwise_not()),
            point(Int::Negative(1)),
        ),
        (true, false) => (
            point(bh.max(al.bitwise_not()).mask().bitwise_not()),
            point(Int::Negative(1)),
        ),
    };
}

/// Raw result hull of a unary operation on a sign-split interval.
fn unary(
    op: ClosureOp,
    ty: Ty,
    (lo, hi): (Int, Int),
) -> Option<(Ext, Ext)> {
    return match op {
        ClosureOp::Neg => {
            Some((Ext::Exact(hi.negate()), Ext::Exact(lo.negate())))
        }
        ClosureOp::Not => Some((
            Ext::Exact(ty.wrap(!hi.pattern())),
            Ext::Exact(ty.wrap(!lo.pattern())),
        )),
        ClosureOp::Abs => Some(match lo.is_negative() {
            false => (Ext::Exact(lo), Ext::Exact(hi)),
            true => (Ext::Exact(hi.negate()), Ext::Exact(lo.negate())),
        }),
        ClosureOp::Isqrt => {
            Some((Ext::Exact(lo.isqrt()?), Ext::Exact(hi.isqrt()?)))
        }
        _ => unreachable!("{op:?} is not a unary operation"),
    };
}
