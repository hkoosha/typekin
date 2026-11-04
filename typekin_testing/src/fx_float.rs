use crate::*;
use core::cmp::Ordering;
use core::marker::PhantomData;
use std::collections::BTreeSet;
use std::hash::{
    DefaultHasher,
    Hash,
    Hasher,
};

#[typekin::float(konst = none)]
pub struct Meters(f64);

#[typekin::float(konst = none)]
pub struct Small(f32);

#[typekin::float(konst = none, domain = 0.0..=1.0)]
pub struct Unit(f64);

#[typekin::float(konst = none, domain = 1.0..)]
pub struct AtLeastOne(f64);

#[typekin::float(konst = none, domain = !{0.5})]
pub struct NotHalf(f64);

#[typekin::float(konst = stable, domain = !is_nan)]
pub struct Stable(f64);

#[typekin::float(konst = none, domain = 0.0..=1.0, with = [display, serde])]
pub struct SerdeUnit(f64);

#[typekin::float(
    konst = none,
    friend = [
        _(f32) -> Rel,
        _(u32) -> Make,
        _(u64) -> Make,
    ],
)]
pub struct Friendly(f64);

#[typekin::point(
    konst = none,
    produce = Self::into_difference(_) -> f64,
)]
pub struct Instant(f64);

impl Instant {
    fn into_difference(self) -> f64 {
        return self.raw();
    }
}

#[derive(Clone, Copy)]
pub struct Offset(pub(crate) f64);

pub(crate) fn offset_raw(offset: Offset) -> f64 {
    return offset.0;
}

#[typekin::point(
    konst = none,
    produce = Self::into_difference(_) -> f64,
    facet = point {
        with = math_add,
        friend = offset_raw(Offset) -> Math,
        produce = Self,
    },
)]
pub struct Position(f64);

impl Position {
    fn into_difference(self) -> f64 {
        return self.raw();
    }
}

#[typekin::float(
    konst = none,
    tree = FloatDirection {
        FloatIn,
    },
)]
pub struct TreeFloat(f64);

#[typekin::float(konst = none, domain = 0.0..)]
pub struct Tagged<T>(f64);

#[typekin::integral(konst = none)]
pub struct Steps(u32);

#[typekin::float(konst = none, friend = _(Steps) -> Make)]
pub struct Distance(f64);

pub struct Feet(pub(crate) f64);

pub(crate) const fn feet_to_meters(feet: Feet) -> f64 {
    return feet.0 * 0.3048;
}

#[typekin::float(konst = none, friend = feet_to_meters(Feet) -> Make)]
pub struct Converted(f64);

#[typekin::float(konst = none, math = std)]
pub struct StdMath(f64);

#[typekin::float(konst = none, domain = -1.0..0.5)]
pub struct Lopsided(f64);

#[typekin::float(konst = none, domain = 0.0..)]
pub struct NonNegative(f64);

#[typekin::float(konst = none, domain = -1.0..=1.0)]
pub struct Signed(f64);

pub(crate) const fn is_not_one(value: f64) -> bool {
    return value != 1.0;
}

#[typekin::float(konst = none, valid = is_not_one, domain = 0.0..=2.0)]
pub struct Checked(f64);

#[typekin::float(konst = none, domain = 1.0e-40..=1.0)]
pub struct Tiny(f32);

#[typekin::float(konst = none, domain = 0.0..=1.0)]
pub struct ExposedUnit(pub f64);

#[typekin::float(konst = none, domain = 0.0..1.0)]
pub struct HalfOpen(f64);

#[typekin::float(konst = none, domain = ..1.5)]
pub struct BelowOneAndAHalf(f64);

#[typekin::float(konst = none, domain = ..=1.5)]
pub struct UpToOneAndAHalf(f64);

#[typekin::float(konst = none, domain = 0.0..1.0 | 10.0..)]
pub struct SplitRange(f64);

#[typekin::float(konst = none, domain = (0.0..1.0 | 10.0..) & !{0.5})]
pub struct Grouped(f64);

#[typekin::float(konst = none, domain = ..f64::MAX)]
pub struct BelowMax(f64);

#[typekin::float(konst = none, domain = 0.0..=f64::INFINITY)]
pub struct UpToInfinity(f64);

#[typekin::float(konst = none, domain = 0.0..std::f64::consts::TAU)]
pub struct Angle(f64);

#[typekin::float(konst = none, domain = is_finite)]
pub struct Finite(f64);

#[typekin::float(konst = none, domain = ..=0.1)]
pub struct TenthOrLess(f32);

#[typekin::float(konst = none, domain = mul(0.25))]
pub struct Quarters(f64);

#[typekin::float(konst = none, math = std, domain = sqrt in ..=2.0)]
pub struct RootAtMostTwo(f64);

#[typekin::float(konst = none, domain = ..0.0 | 1.0..)]
pub struct OutsideUnit(f64);

#[typekin::float(konst = none, domain = !(..5.0))]
pub struct NotBelowFive(f64);

pub(crate) fn is_not_two(value: f64) -> bool {
    return value != 2.0;
}

#[typekin::float(konst = none, valid = is_not_two)]
pub struct CallbackOnly(f64);

#[typekin::float(konst = none, domain = !is_nan, without = ord)]
pub struct Unordered(f64);

#[typekin::float(konst = none, without = bit_access)]
pub struct WithoutBitAccess(f64);

#[typekin::float(konst = none, math = core)]
pub struct CoreMath(f64);

#[typekin::float(konst = none, math = std, without = float_math)]
pub struct StdWithoutFloatMath(f64);

#[typekin::float(
    konst = none,
    math = std,
    without = [fn_round_ties_even, fn_is_nan, fn_from_bits, fn_sin_cos],
)]
pub struct WithoutSomeFloatFns(f64);

#[typekin::float(konst = none, without = fn_signum)]
pub struct ZeroSignum(f64);

impl ZeroSignum {
    /// `0.0` for either zero, unlike `f64::signum`.
    pub fn signum(self) -> Self {
        let raw = self.raw();
        return match raw == 0.0 {
            true => Self::make(0.0),
            false => Self::make(raw.signum()),
        };
    }
}

#[typekin::float(konst = none, profile = int)]
pub struct IntProfile(f64);

#[typekin::float(konst = none, profile = point)]
pub struct PointProfile(f64);

#[derive(Clone, Copy)]
pub struct Boost(pub(crate) f64);

pub(crate) fn boost_raw(boost: Boost) -> f64 {
    return boost.0;
}

#[typekin::float(konst = none, friend = boost_raw(Boost) -> Math)]
pub struct Boosted(f64);

pub struct Reading(pub(crate) f64);

pub(crate) fn reading_raw(reading: Reading) -> f64 {
    return reading.0;
}

#[typekin::friends(
    relation = f64,
    mod = gauge_protocol,
    friend = reading_raw(Reading) -> Make,
)]
pub struct Gauge {
    level: f64,
}

impl Gauge {
    pub fn of_parts(level: f64) -> Self {
        return Self { level };
    }

    pub(crate) fn level(&self) -> f64 {
        return self.level;
    }
}

#[typekin::integral(konst = none)]
pub struct Short(u16);

#[typekin::integral(konst = none)]
pub struct SignedShort(i16);

#[typekin::integral(konst = none)]
pub struct SignedCount(i32);

#[typekin::integral(konst = none)]
pub struct Large(u64);

#[typekin::integral(konst = none)]
pub struct SignedLarge(i64);

#[typekin::integral(konst = none)]
pub struct Pointer(usize);

#[typekin::integral(konst = none, profile = point)]
pub struct Tick(i32);

#[typekin::integral(konst = none, with = fmt_exp_lower)]
pub struct Scientific(u32);

pub(crate) const STABLE: Stable = match Stable::try_make(1.5) {
    Ok(it) => it,
    Err(_) => panic!("1.5 is not NaN"),
};

pub(crate) fn stable(value: f64) -> Stable {
    return Stable::try_make(value).unwrap();
}

pub(crate) fn hash_of<T: Hash>(value: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    return hasher.finish();
}
