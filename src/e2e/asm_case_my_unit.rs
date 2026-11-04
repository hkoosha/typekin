//! NaN-free floats. `eq`, `lt`, `neg`, `abs`, and hashing without zero read
//! their operands through references: by value their bodies are `my_f64`'s,
//! and identical probes of different cases merge in the batched build.

use core::cmp::Ordering;
use core::hash::{
    Hash,
    Hasher,
};

/// Both zeros are valid: hashing canonicalizes `-0.0`.
#[typekin::float(domain = 0.0..=1.0)]
pub struct MyUnit(f64);

/// No zero is valid: hashing uses the bits directly.
#[typekin::float(domain = 1.0..)]
pub struct MyAtLeastOne(f64);

/// Symmetric: `Neg` needs no check.
#[typekin::float(domain = -1.0..=1.0)]
pub struct MySigned(f64);

/// Keeps the hashed value observable without hashing work of its own.
pub struct LastWord(u64);

impl Hasher for LastWord {
    fn finish(&self) -> u64 {
        return self.0;
    }

    fn write(
        &mut self,
        _: &[u8],
    ) {
        unreachable!("only u64 is hashed");
    }

    fn write_u64(
        &mut self,
        value: u64,
    ) {
        self.0 = value;
    }
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_make_unit(value: f64) -> Result<f64, f64> {
    return if value >= 0.0 && value <= 1.0 {
        Ok(value)
    }
    else {
        Err(value)
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_make_unit(value: f64) -> Result<MyUnit, f64> {
    return MyUnit::try_make(value);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_eq_unit(
    lhs: &f64,
    rhs: &f64,
) -> bool {
    return lhs == rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_eq_unit(
    lhs: &MyUnit,
    rhs: &MyUnit,
) -> bool {
    return lhs == rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_lt_unit(
    lhs: &f64,
    rhs: &f64,
) -> bool {
    return lhs < rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_lt_unit(
    lhs: &MyUnit,
    rhs: &MyUnit,
) -> bool {
    return lhs < rhs;
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_cmp_unit(
    lhs: f64,
    rhs: f64,
) -> Ordering {
    return if lhs < rhs {
        Ordering::Less
    }
    else if lhs > rhs {
        Ordering::Greater
    }
    else {
        Ordering::Equal
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_cmp_unit(
    lhs: MyUnit,
    rhs: MyUnit,
) -> Ordering {
    return lhs.cmp(&rhs);
}

/// `Ord::max` returns the second argument when both compare equal; written
/// as `core`'s `max_by` orders the comparison.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_max_unit(
    lhs: f64,
    rhs: f64,
) -> f64 {
    return if rhs < lhs { lhs } else { rhs };
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_max_unit(
    lhs: MyUnit,
    rhs: MyUnit,
) -> MyUnit {
    return Ord::max(lhs, rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_min_unit(
    lhs: f64,
    rhs: f64,
) -> f64 {
    return if rhs < lhs { rhs } else { lhs };
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_min_unit(
    lhs: MyUnit,
    rhs: MyUnit,
) -> MyUnit {
    return Ord::min(lhs, rhs);
}

/// `Ord::clamp` asserts `min <= max`; so does the raw version.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_clamp_unit(
    value: f64,
    min: f64,
    max: f64,
) -> f64 {
    assert!(min <= max);
    return if value < min {
        min
    }
    else if value > max {
        max
    }
    else {
        value
    };
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_clamp_unit(
    value: MyUnit,
    min: MyUnit,
    max: MyUnit,
) -> MyUnit {
    return Ord::clamp(value, min, max);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_hash_unit(value: f64) -> u64 {
    let mut hasher = LastWord(0);
    (value + 0.0).to_bits().hash(&mut hasher);
    return hasher.finish();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_hash_unit(value: MyUnit) -> u64 {
    let mut hasher = LastWord(0);
    value.hash(&mut hasher);
    return hasher.finish();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_hash_at_least_one(value: &f64) -> u64 {
    let mut hasher = LastWord(0);
    value.to_bits().hash(&mut hasher);
    return hasher.finish();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_hash_at_least_one(value: &MyAtLeastOne) -> u64 {
    let mut hasher = LastWord(0);
    value.hash(&mut hasher);
    return hasher.finish();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_neg_signed(value: &f64) -> f64 {
    return -*value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_neg_signed(value: &MySigned) -> MySigned {
    return -*value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_abs_unit(value: &f64) -> f64 {
    return value.abs();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_abs_unit(value: &MyUnit) -> MyUnit {
    return value.abs();
}

/// The domain rejects NaN: the query is a constant.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_is_nan_unit(_value: f64) -> bool {
    return false;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_is_nan_unit(value: MyUnit) -> bool {
    return value.is_nan();
}

/// The domain rejects NaN and both infinities: the query is a constant.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_is_finite_unit(_value: f64) -> bool {
    return true;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_is_finite_unit(value: MyUnit) -> bool {
    return value.is_finite();
}

pub fn main() {
    let values = [
        0.0,
        -0.0,
        0.25,
        0.5,
        1.0,
    ];
    for lhs in values {
        let lhs_w = MyUnit::try_make(lhs).unwrap();
        assert_eq!(wrapped_hash_unit(lhs_w), raw_hash_unit(lhs), "{lhs}");
        for rhs in values {
            let rhs_w = MyUnit::try_make(rhs).unwrap();
            assert_eq!(
                wrapped_eq_unit(&lhs_w, &rhs_w),
                raw_eq_unit(&lhs, &rhs)
            );
            assert_eq!(
                wrapped_lt_unit(&lhs_w, &rhs_w),
                raw_lt_unit(&lhs, &rhs)
            );
            assert_eq!(wrapped_cmp_unit(lhs_w, rhs_w), raw_cmp_unit(lhs, rhs));
            assert_eq!(
                wrapped_max_unit(lhs_w, rhs_w).raw().to_bits(),
                raw_max_unit(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_min_unit(lhs_w, rhs_w).raw().to_bits(),
                raw_min_unit(lhs, rhs).to_bits()
            );
            if lhs <= rhs {
                let value = MyUnit::try_make(0.5).unwrap();
                assert_eq!(
                    wrapped_clamp_unit(value, lhs_w, rhs_w).raw().to_bits(),
                    raw_clamp_unit(0.5, lhs, rhs).to_bits()
                );
            }
        }
    }
    for value in [
        -1.0,
        -0.0,
        0.0,
        0.5,
        1.0,
        2.0,
        f64::NAN,
    ] {
        assert_eq!(
            wrapped_try_make_unit(value)
                .map(MyUnit::raw)
                .map(f64::to_bits)
                .map_err(f64::to_bits),
            raw_try_make_unit(value)
                .map(f64::to_bits)
                .map_err(f64::to_bits),
        );
    }
    for value in [
        -1.0,
        -0.0,
        0.5,
        1.0,
    ] {
        let wrapped = MySigned::try_make(value).unwrap();
        assert_eq!(
            wrapped_neg_signed(&wrapped).raw().to_bits(),
            raw_neg_signed(&value).to_bits()
        );
    }
    for value in [
        -0.0,
        0.0,
        0.5,
    ] {
        let wrapped = MyUnit::try_make(value).unwrap();
        assert_eq!(
            wrapped_abs_unit(&wrapped).raw().to_bits(),
            raw_abs_unit(&value).to_bits()
        );
    }
    // Through pointers: the constant results would otherwise be folded into
    // `main` and the functions dropped.
    let raw_is_nan: extern "C" fn(f64) -> bool =
        std::hint::black_box(raw_is_nan_unit);
    let wrapped_is_nan: extern "C" fn(MyUnit) -> bool =
        std::hint::black_box(wrapped_is_nan_unit);
    let raw_is_finite: extern "C" fn(f64) -> bool =
        std::hint::black_box(raw_is_finite_unit);
    let wrapped_is_finite: extern "C" fn(MyUnit) -> bool =
        std::hint::black_box(wrapped_is_finite_unit);
    for value in [
        -0.0,
        0.0,
        f64::from_bits(1),
        0.5,
        1.0,
    ] {
        let wrapped = MyUnit::try_make(value).unwrap();
        assert_eq!(wrapped_is_nan(wrapped), raw_is_nan(value));
        assert_eq!(wrapped_is_finite(wrapped), raw_is_finite(value));
    }
    for value in [
        1.0,
        2.5,
        f64::INFINITY,
    ] {
        let wrapped = MyAtLeastOne::try_make(value).unwrap();
        assert_eq!(
            wrapped_hash_at_least_one(&wrapped),
            raw_hash_at_least_one(&value)
        );
    }
}
