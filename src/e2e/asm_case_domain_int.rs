//! Domain codegen: `raw_*` is the hand-written *normalized* condition (with
//! the same `NonZero` storage where typekin infers it, built as typekin
//! builds it: the safe constructor, panicking on zero), `written_*` the
//! condition exactly as written, `wrapped_*` typekin's `try_make`. Optimized
//! `raw_*` and `wrapped_*` must be identical, and `wrapped_*` never longer
//! than `written_*`.

#[typekin::integral(domain = (1..12 | 87..) & !{10} & !mul(3))]
pub struct Code(u32);

#[typekin::integral(domain = (-5..12 | 87..) & !{10} & !mul(3))]
pub struct Code64(i64);

#[typekin::integral(domain = mul(2) | 100..=200)]
pub struct EvenOrBand(u8);

#[typekin::integral(domain = (abs in ..=1000) & !mul(7))]
pub struct Bounded(i32);

#[typekin::integral(domain = (count_ones in {1}) | (rem_euclid(16) in {3}))]
pub struct Bits(u32);

#[typekin::integral(domain = 8..10 | 2..=4)]
pub struct Unsorted(u32);

#[typekin::integral(domain = 1..5 | 3..10)]
pub struct Overlapping(u32);

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_code(it: u32) -> bool {
    // The wrapper stores `NonZeroU32` (the domain excludes zero), so the
    // hand-written equivalent builds the same storage.
    let checked: Result<core::num::NonZeroU32, u32> =
        if ((it >= 1 && it < 12) || it >= 87) && it != 10 && it % 3 != 0 {
            Ok(match core::num::NonZeroU32::new(it) {
                Some(value) => value,
                None => panic!("zero cannot be stored by this integral type"),
            })
        }
        else {
            Err(it)
        };
    return checked.is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_code(it: u32) -> bool {
    let checked: Result<core::num::NonZeroU32, u32> =
        if ((it >= 1 && it < 12) || it >= 87) && it != 10 && it % 3 != 0 {
            Ok(match core::num::NonZeroU32::new(it) {
                Some(value) => value,
                None => panic!("zero cannot be stored by this integral type"),
            })
        }
        else {
            Err(it)
        };
    return checked.is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_code(raw: u32) -> bool {
    return Code::try_make(raw).is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_code64(it: i64) -> bool {
    let checked: Result<core::num::NonZeroI64, i64> =
        if ((it >= -5 && it < 12) || it >= 87) && it != 10 && it % 3 != 0 {
            Ok(match core::num::NonZeroI64::new(it) {
                Some(value) => value,
                None => panic!("zero cannot be stored by this integral type"),
            })
        }
        else {
            Err(it)
        };
    return checked.is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_code64(raw: i64) -> bool {
    return Code64::try_make(raw).is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_even_or_band(it: u8) -> bool {
    return it % 2 == 0 || (it >= 100 && it <= 200);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_even_or_band(it: u8) -> bool {
    return it % 2 == 0 || (it >= 100 && it <= 200);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_even_or_band(raw: u8) -> bool {
    return EvenOrBand::try_make(raw).is_ok();
}

/// The normalized check behind one `#[inline(always)]` layer, as in
/// `try_make`: the zero check left after it compiles differently inline.
#[inline(always)]
const fn raw_bounded_body(it: i32) -> Result<core::num::NonZeroI32, i32> {
    // The wrapper stores `NonZeroI32` (the domain excludes zero), so the
    // hand-written equivalent builds the same storage.
    return if it.unsigned_abs() <= 1000 && it % 7 != 0 {
        Ok(match core::num::NonZeroI32::new(it) {
            Some(value) => value,
            None => panic!("zero cannot be stored by this integral type"),
        })
    }
    else {
        Err(it)
    };
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_bounded(it: i32) -> bool {
    return raw_bounded_body(it).is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_bounded(it: i32) -> bool {
    let checked: Result<core::num::NonZeroI32, i32> =
        if it.unsigned_abs() <= 1000 && it % 7 != 0 {
            Ok(match core::num::NonZeroI32::new(it) {
                Some(value) => value,
                None => panic!("zero cannot be stored by this integral type"),
            })
        }
        else {
            Err(it)
        };
    return checked.is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_bounded(raw: i32) -> bool {
    return Bounded::try_make(raw).is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_bits(it: u32) -> bool {
    // The wrapper stores `NonZeroU32` (the domain excludes zero), so the
    // hand-written equivalent builds the same storage.
    let checked: Result<core::num::NonZeroU32, u32> =
        if it.count_ones() == 1 || it.rem_euclid(16) == 3 {
            Ok(match core::num::NonZeroU32::new(it) {
                Some(value) => value,
                None => panic!("zero cannot be stored by this integral type"),
            })
        }
        else {
            Err(it)
        };
    return checked.is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_bits(it: u32) -> bool {
    let checked: Result<core::num::NonZeroU32, u32> =
        if it.count_ones() == 1 || it.rem_euclid(16) == 3 {
            Ok(match core::num::NonZeroU32::new(it) {
                Some(value) => value,
                None => panic!("zero cannot be stored by this integral type"),
            })
        }
        else {
            Err(it)
        };
    return checked.is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_bits(raw: u32) -> bool {
    return Bits::try_make(raw).is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_unsorted(it: u32) -> bool {
    // The wrapper stores `NonZeroU32` (the domain excludes zero), so the
    // hand-written equivalent builds the same storage.
    let checked: Result<core::num::NonZeroU32, u32> =
        if (it >= 2 && it <= 4) || (it >= 8 && it < 10) {
            Ok(match core::num::NonZeroU32::new(it) {
                Some(value) => value,
                None => panic!("zero cannot be stored by this integral type"),
            })
        }
        else {
            Err(it)
        };
    return checked.is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_unsorted(it: u32) -> bool {
    let checked: Result<core::num::NonZeroU32, u32> =
        if (it >= 8 && it < 10) || (it >= 2 && it <= 4) {
            Ok(match core::num::NonZeroU32::new(it) {
                Some(value) => value,
                None => panic!("zero cannot be stored by this integral type"),
            })
        }
        else {
            Err(it)
        };
    return checked.is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_unsorted(raw: u32) -> bool {
    return Unsorted::try_make(raw).is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_overlapping(it: u32) -> bool {
    // The wrapper stores `NonZeroU32` (the domain excludes zero), so the
    // hand-written equivalent builds the same storage.
    let checked: Result<core::num::NonZeroU32, u32> = if it >= 1 && it < 10 {
        Ok(match core::num::NonZeroU32::new(it) {
            Some(value) => value,
            None => panic!("zero cannot be stored by this integral type"),
        })
    }
    else {
        Err(it)
    };
    return checked.is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_overlapping(it: u32) -> bool {
    let checked: Result<core::num::NonZeroU32, u32> =
        if (it >= 1 && it < 5) || (it >= 3 && it < 10) {
            Ok(match core::num::NonZeroU32::new(it) {
                Some(value) => value,
                None => panic!("zero cannot be stored by this integral type"),
            })
        }
        else {
            Err(it)
        };
    return checked.is_ok();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_overlapping(raw: u32) -> bool {
    return Overlapping::try_make(raw).is_ok();
}

pub fn main() {
    for value in 0..=u8::MAX {
        let wrapped = wrapped_even_or_band(value);
        assert_eq!(raw_even_or_band(value), wrapped);
        assert_eq!(written_even_or_band(value), wrapped);

        let wide = u32::from(value);
        let wrapped = wrapped_code(wide);
        assert_eq!(raw_code(wide), wrapped);
        assert_eq!(written_code(wide), wrapped);
        let wrapped = wrapped_bits(wide);
        assert_eq!(raw_bits(wide), wrapped);
        assert_eq!(written_bits(wide), wrapped);
        let wrapped = wrapped_unsorted(wide);
        assert_eq!(raw_unsorted(wide), wrapped);
        assert_eq!(written_unsorted(wide), wrapped);
        let wrapped = wrapped_overlapping(wide);
        assert_eq!(raw_overlapping(wide), wrapped);
        assert_eq!(written_overlapping(wide), wrapped);

        let signed = i64::from(value) - 128;
        let wrapped = wrapped_code64(signed);
        assert_eq!(written_code64(signed), wrapped);

        let bounded = i32::from(value) * 9 - 1100;
        let wrapped = wrapped_bounded(bounded);
        assert_eq!(raw_bounded(bounded), wrapped);
        assert_eq!(written_bounded(bounded), wrapped);
    }
}
