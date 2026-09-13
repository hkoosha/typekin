use typekin_perf_types::Number;

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_add_u32(
    lhs: u32,
    rhs: u32,
) -> u32 {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_add_u32(
    lhs: Number,
    rhs: Number,
) -> Number {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_sub_u32(
    lhs: u32,
    rhs: u32,
) -> u32 {
    return lhs - rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_sub_u32(
    lhs: Number,
    rhs: Number,
) -> Number {
    return lhs - rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_mul_u32(
    lhs: u32,
    rhs: u32,
) -> u32 {
    return lhs * rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_mul_u32(
    lhs: Number,
    rhs: Number,
) -> Number {
    return lhs * rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn raw_div_u32(
    lhs: u32,
    rhs: u32,
) -> u32 {
    // SAFETY: callers must keep the div/rem hot-path probe in its valid domain.
    unsafe { core::hint::assert_unchecked(rhs != 0) };
    return lhs / rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn wrapped_div_u32(
    lhs: Number,
    rhs: Number,
) -> Number {
    // SAFETY: callers must keep the div/rem hot-path probe in its valid domain.
    unsafe { core::hint::assert_unchecked(rhs.0 != 0) };
    return lhs / rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn raw_rem_u32(
    lhs: u32,
    rhs: u32,
) -> u32 {
    // SAFETY: callers must keep the div/rem hot-path probe in its valid domain.
    unsafe { core::hint::assert_unchecked(rhs != 0) };
    return lhs % rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn wrapped_rem_u32(
    lhs: Number,
    rhs: Number,
) -> Number {
    // SAFETY: callers must keep the div/rem hot-path probe in its valid domain.
    unsafe { core::hint::assert_unchecked(rhs.0 != 0) };
    return lhs % rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_bitand_u32(
    lhs: u32,
    rhs: u32,
) -> u32 {
    return lhs & rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_bitand_u32(
    lhs: Number,
    rhs: Number,
) -> Number {
    return lhs & rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_bitor_u32(
    lhs: u32,
    rhs: u32,
) -> u32 {
    return lhs | rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_bitor_u32(
    lhs: Number,
    rhs: Number,
) -> Number {
    return lhs | rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_bitxor_u32(
    lhs: u32,
    rhs: u32,
) -> u32 {
    return lhs ^ rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_bitxor_u32(
    lhs: Number,
    rhs: Number,
) -> Number {
    return lhs ^ rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_not_u32(value: u32) -> u32 {
    return !value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_not_u32(value: Number) -> Number {
    return !value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_shl_u32(
    lhs: u32,
    rhs: usize,
) -> u32 {
    return lhs << rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_shl_u32(
    lhs: Number,
    rhs: usize,
) -> Number {
    return lhs << rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_shr_u32(
    lhs: u32,
    rhs: usize,
) -> u32 {
    return lhs >> rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_shr_u32(
    lhs: Number,
    rhs: usize,
) -> Number {
    return lhs >> rhs;
}

fn main() {
    let lhs = 0b1101_0110u32;
    let rhs = 0b0011_1011u32;
    let shift = 3usize;
    let lhs_wrapped = Number(lhs);
    let rhs_wrapped = Number(rhs);

    assert_eq!(
        wrapped_add_u32(lhs_wrapped, rhs_wrapped).0,
        raw_add_u32(lhs, rhs)
    );
    assert_eq!(
        wrapped_sub_u32(lhs_wrapped, rhs_wrapped).0,
        raw_sub_u32(lhs, rhs)
    );
    assert_eq!(
        wrapped_mul_u32(lhs_wrapped, rhs_wrapped).0,
        raw_mul_u32(lhs, rhs)
    );
    // SAFETY: rhs is a nonzero constant in this fixture smoke check.
    unsafe {
        assert_eq!(
            wrapped_div_u32(lhs_wrapped, rhs_wrapped).0,
            raw_div_u32(lhs, rhs)
        );
        assert_eq!(
            wrapped_rem_u32(lhs_wrapped, rhs_wrapped).0,
            raw_rem_u32(lhs, rhs)
        );
    }
    assert_eq!(
        wrapped_bitand_u32(lhs_wrapped, rhs_wrapped).0,
        raw_bitand_u32(lhs, rhs)
    );
    assert_eq!(
        wrapped_bitor_u32(lhs_wrapped, rhs_wrapped).0,
        raw_bitor_u32(lhs, rhs)
    );
    assert_eq!(
        wrapped_bitxor_u32(lhs_wrapped, rhs_wrapped).0,
        raw_bitxor_u32(lhs, rhs)
    );
    assert_eq!(wrapped_not_u32(lhs_wrapped).0, raw_not_u32(lhs));
    assert_eq!(
        wrapped_shl_u32(lhs_wrapped, shift).0,
        raw_shl_u32(lhs, shift)
    );
    assert_eq!(
        wrapped_shr_u32(lhs_wrapped, shift).0,
        raw_shr_u32(lhs, shift)
    );
}
