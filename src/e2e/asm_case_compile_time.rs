//! Compile-time validation: operations proven closed over their domain
//! (`raw_*` is the bare primitive operation, `wrapped_*` typekin's) and
//! literal macros (`raw_*` returns the constant). Optimized pairs must be
//! identical; `CLOSURE_CONTROL_OPERATIONS` keeps a validated operation that
//! must still differ.

#[typekin::integral(domain = ..=100)]
pub struct Percent(u8);

#[typekin::integral(domain = 1..=1023 | 49_152..=65_535)]
pub struct Port(u16);

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn raw_percent_div(
    lhs: u8,
    rhs: u8,
) -> u8 {
    // SAFETY: callers keep the divisor nonzero, so the division-by-zero
    // panic (whose location differs per function) is not compared.
    unsafe { core::hint::assert_unchecked(rhs != 0) };
    return lhs / rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn wrapped_percent_div(
    lhs: Percent,
    rhs: Percent,
) -> Percent {
    // SAFETY: as in `raw_percent_div`.
    unsafe { core::hint::assert_unchecked(rhs.raw() != 0) };
    return lhs / rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn raw_percent_rem(
    lhs: u8,
    rhs: u8,
) -> u8 {
    // SAFETY: as in `raw_percent_div`.
    unsafe { core::hint::assert_unchecked(rhs != 0) };
    return lhs % rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn wrapped_percent_rem(
    lhs: Percent,
    rhs: Percent,
) -> Percent {
    // SAFETY: as in `raw_percent_div`.
    unsafe { core::hint::assert_unchecked(rhs.raw() != 0) };
    return lhs % rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_percent_bitand(
    lhs: u8,
    rhs: u8,
) -> u8 {
    return lhs & rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_percent_bitand(
    lhs: Percent,
    rhs: Percent,
) -> Percent {
    return lhs & rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_percent_shr(
    lhs: u8,
    rhs: usize,
) -> u8 {
    return lhs >> rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_percent_shr(
    lhs: Percent,
    rhs: usize,
) -> Percent {
    return lhs >> rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_percent_midpoint(
    lhs: u8,
    rhs: u8,
) -> u8 {
    return lhs.midpoint(rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_percent_midpoint(
    lhs: Percent,
    rhs: Percent,
) -> Percent {
    return lhs.midpoint(rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_percent_add(
    lhs: u8,
    rhs: u8,
) -> u8 {
    return lhs.wrapping_add(rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_percent_add(
    lhs: Percent,
    rhs: Percent,
) -> Percent {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_lit_port() -> u16 {
    return 443;
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_lit_port() -> Port {
    return port!(443);
}

pub fn main() {
    for lhs in 0..=100u8 {
        let wrapped = Percent::try_make(lhs).unwrap();
        for rhs in 1..=100u8 {
            let other = Percent::try_make(rhs).unwrap();
            // SAFETY: `rhs` starts at 1.
            unsafe {
                assert_eq!(
                    raw_percent_div(lhs, rhs),
                    wrapped_percent_div(wrapped, other).raw()
                );
                assert_eq!(
                    raw_percent_rem(lhs, rhs),
                    wrapped_percent_rem(wrapped, other).raw()
                );
            }
            assert_eq!(
                raw_percent_bitand(lhs, rhs),
                wrapped_percent_bitand(wrapped, other).raw()
            );
            assert_eq!(
                raw_percent_midpoint(lhs, rhs),
                wrapped_percent_midpoint(wrapped, other).raw()
            );
            if lhs.checked_add(rhs).is_some_and(|it| it <= 100) {
                assert_eq!(
                    raw_percent_add(lhs, rhs),
                    wrapped_percent_add(wrapped, other).raw()
                );
            }
        }
        for amount in 0..8usize {
            assert_eq!(
                raw_percent_shr(lhs, amount),
                wrapped_percent_shr(wrapped, amount).raw()
            );
        }
    }
    // Through pointers: the constant results would otherwise be folded into
    // `main` and the functions dropped.
    let raw: extern "C" fn() -> u16 = std::hint::black_box(raw_lit_port);
    let wrapped: extern "C" fn() -> Port =
        std::hint::black_box(wrapped_lit_port);
    assert_eq!(raw(), wrapped().raw());
}
