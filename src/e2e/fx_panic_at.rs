use crate::*;

/// A friend operand carrying any raw value, zero included.
#[derive(Clone, Copy)]
pub(crate) struct RawOperand(pub(crate) u8);

pub(crate) fn raw_operand(it: RawOperand) -> u8 {
    return it.0;
}

/// Validated; nothing panics.
#[typekin::integral(
    konst = none,
    panic_at = nothing,
    domain = 1..=100,
    friend = raw_operand(RawOperand) -> Math,
)]
pub(crate) struct Score(u8);

/// Unvalidated; nothing panics, so overflow is reported too.
#[typekin::integral(konst = none, panic_at = nothing)]
pub(crate) struct Tally(u8);

/// Only Rust's overflow checks may still panic.
#[typekin::integral(konst = none, panic_at = overflow)]
pub(crate) struct Gauge(u8);

/// Only Rust's arithmetic may still panic.
#[typekin::integral(konst = none, panic_at = arithmetic)]
pub(crate) struct Meter(u8);

/// `sequential` panics on overflow itself, which `arithmetic` reports.
#[typekin::sequential(konst = none, panic_at = arithmetic)]
pub(crate) struct Ticket(u8);

/// Wrapping arithmetic only has division by zero to report.
#[typekin::wrapping(konst = none, panic_at = nothing)]
pub(crate) struct Phase(u8);

/// Signed, for `-MIN`.
#[typekin::integral(konst = none, panic_at = nothing)]
pub(crate) struct Lean(i8);

/// A `Within` source of `Whole`.
#[typekin::integral(konst = none, domain = 1..=50)]
pub(crate) struct Half(u8);

#[typekin::integral(
    konst = none,
    panic_at = nothing,
    domain = 1..=100,
    friend = _(Half) -> Within,
)]
pub(crate) struct Whole(u8);

#[typekin::integral(
    konst = none,
    panic_at = nothing,
    domain = 1..=100,
    produce = Self::doubled(_) -> u16,
)]
pub(crate) struct Produced(u8);

impl Produced {
    fn doubled(self) -> u16 {
        return u16::from(self.raw()) * 2;
    }
}

#[typekin::integral(
    konst = none,
    panic_at = nothing,
    domain = 1..=100,
    facet = int {
        friend = raw_operand(RawOperand) -> Math,
    },
)]
pub(crate) struct Faceted(u8);

#[typekin::float(konst = none, panic_at = nothing, domain = 0.0..=1.0)]
pub(crate) struct Share(f64);

#[typekin::integral(konst = nightly, panic_at = nothing, domain = 1..=100)]
pub(crate) struct ConstScore(u8);

pub(crate) const CONST_SCORE_SUM: Result<ConstScore, Option<u8>> = {
    let score = match ConstScore::try_make(40) {
        Ok(it) => it,
        Err(_) => panic!("40 is a score"),
    };
    score + score
};
