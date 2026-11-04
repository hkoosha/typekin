//! Proven `Within` friends (`raw_*` is the bare cast, `wrapped_*` typekin's
//! `of`). Optimized pairs must be identical; `WITHIN_CONTROL_OPERATIONS`
//! pairs the same cast with a validated friend of the same types, which
//! must still differ.

use std::num::NonZeroU16;

#[typekin::integral(domain = ..=100)]
pub struct Percent(u8);

#[typekin::integral(domain = ..=100, friend = _(Percent) -> Within)]
pub struct Score(u16);

#[typekin::integral(domain = ..=100, friend = _(Percent) -> Make)]
pub struct Validated(u16);

/// `NonZero` storage on both sides.
#[typekin::integral(domain = 1..=1023 | 49_152..)]
pub struct Port(u16);

#[typekin::integral(domain = 1.., friend = _(Port) -> Within)]
pub struct Positive(u16);

#[typekin::integral(domain = -128..=127, friend = _(i8) -> Within)]
pub struct Signed(i16);

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_within_score(percent: u8) -> u16 {
    return percent as u16;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_within_score(percent: Percent) -> Score {
    return Score::of(percent);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_within_validated(percent: Percent) -> Validated {
    return Validated::of(percent);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_within_positive(port: NonZeroU16) -> NonZeroU16 {
    return port;
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_within_positive(port: Port) -> Positive {
    return Positive::of(port);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_within_signed(raw: i8) -> i16 {
    return raw as i16;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_within_signed(raw: i8) -> Signed {
    return Signed::of(raw);
}

/// Through pointers: LLVM folds calls whose result it can compute, such as
/// the identity on `NonZeroU16`, and a linked binary drops the uncalled
/// functions.
pub fn main() {
    let raw_score: extern "C" fn(u8) -> u16 =
        std::hint::black_box(raw_within_score);
    let score: extern "C" fn(Percent) -> Score =
        std::hint::black_box(wrapped_within_score);
    let validated: extern "C" fn(Percent) -> Validated =
        std::hint::black_box(wrapped_within_validated);
    for raw in 0..=100u8 {
        let percent = Percent::try_make(raw).unwrap();
        assert_eq!(raw_score(raw), score(percent).raw());
        assert_eq!(raw_score(raw), validated(percent).raw());
    }

    let raw_positive: extern "C" fn(NonZeroU16) -> NonZeroU16 =
        std::hint::black_box(raw_within_positive);
    let positive: extern "C" fn(Port) -> Positive =
        std::hint::black_box(wrapped_within_positive);
    for raw in [1u16, 1023, 49_152, u16::MAX] {
        let port = Port::try_make(raw).unwrap();
        let non_zero = NonZeroU16::new(raw).unwrap();
        assert_eq!(raw_positive(non_zero).get(), positive(port).raw());
    }

    let raw_signed: extern "C" fn(i8) -> i16 =
        std::hint::black_box(raw_within_signed);
    let signed: extern "C" fn(i8) -> Signed =
        std::hint::black_box(wrapped_within_signed);
    for raw in i8::MIN..=i8::MAX {
        assert_eq!(raw_signed(raw), signed(raw).raw());
    }
}
