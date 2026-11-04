//! `konst = nightly` arithmetic on a primitive no other case uses, so its
//! probes cannot merge with `my_u32`'s.

#[typekin::integral(konst = nightly)]
pub struct MyU64Const(u64);

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_add_u64(
    lhs: u64,
    rhs: u64,
) -> u64 {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_add_u64(
    lhs: MyU64Const,
    rhs: MyU64Const,
) -> MyU64Const {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_sub_u64(
    lhs: u64,
    rhs: u64,
) -> u64 {
    return lhs - rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_sub_u64(
    lhs: MyU64Const,
    rhs: MyU64Const,
) -> MyU64Const {
    return lhs - rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_mul_u64(
    lhs: u64,
    rhs: u64,
) -> u64 {
    return lhs * rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_mul_u64(
    lhs: MyU64Const,
    rhs: MyU64Const,
) -> MyU64Const {
    return lhs * rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn raw_div_u64(
    lhs: u64,
    rhs: u64,
) -> u64 {
    // SAFETY: callers must keep the div/rem hot-path probe in its valid domain.
    unsafe { core::hint::assert_unchecked(rhs != 0) };
    return lhs / rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn wrapped_div_u64(
    lhs: MyU64Const,
    rhs: MyU64Const,
) -> MyU64Const {
    // SAFETY: callers must keep the div/rem hot-path probe in its valid domain.
    unsafe { core::hint::assert_unchecked(rhs.raw() != 0) };
    return lhs / rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn raw_rem_u64(
    lhs: u64,
    rhs: u64,
) -> u64 {
    // SAFETY: callers must keep the div/rem hot-path probe in its valid domain.
    unsafe { core::hint::assert_unchecked(rhs != 0) };
    return lhs % rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn wrapped_rem_u64(
    lhs: MyU64Const,
    rhs: MyU64Const,
) -> MyU64Const {
    // SAFETY: callers must keep the div/rem hot-path probe in its valid domain.
    unsafe { core::hint::assert_unchecked(rhs.raw() != 0) };
    return lhs % rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_bitand_u64(
    lhs: u64,
    rhs: u64,
) -> u64 {
    return lhs & rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_bitand_u64(
    lhs: MyU64Const,
    rhs: MyU64Const,
) -> MyU64Const {
    return lhs & rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_bitor_u64(
    lhs: u64,
    rhs: u64,
) -> u64 {
    return lhs | rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_bitor_u64(
    lhs: MyU64Const,
    rhs: MyU64Const,
) -> MyU64Const {
    return lhs | rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_bitxor_u64(
    lhs: u64,
    rhs: u64,
) -> u64 {
    return lhs ^ rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_bitxor_u64(
    lhs: MyU64Const,
    rhs: MyU64Const,
) -> MyU64Const {
    return lhs ^ rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_not_u64(value: u64) -> u64 {
    return !value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_not_u64(value: MyU64Const) -> MyU64Const {
    return !value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_shl_u64(
    lhs: u64,
    rhs: usize,
) -> u64 {
    return lhs << rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_shl_u64(
    lhs: MyU64Const,
    rhs: usize,
) -> MyU64Const {
    return lhs << rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_shr_u64(
    lhs: u64,
    rhs: usize,
) -> u64 {
    return lhs >> rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_shr_u64(
    lhs: MyU64Const,
    rhs: usize,
) -> MyU64Const {
    return lhs >> rhs;
}

pub fn main() {
    let lhs = 0b1101_0110u64 << 32;
    let rhs = 0b0011_1011u64;
    let shift = 3usize;
    let lhs_wrapped = MyU64Const::make(lhs);
    let rhs_wrapped = MyU64Const::make(rhs);

    assert_eq!(
        wrapped_add_u64(lhs_wrapped, rhs_wrapped).raw(),
        raw_add_u64(lhs, rhs)
    );
    assert_eq!(
        wrapped_sub_u64(lhs_wrapped, rhs_wrapped).raw(),
        raw_sub_u64(lhs, rhs)
    );
    assert_eq!(
        wrapped_mul_u64(lhs_wrapped, rhs_wrapped).raw(),
        raw_mul_u64(lhs, rhs)
    );
    // SAFETY: rhs is a nonzero constant in this fixture smoke check.
    unsafe {
        assert_eq!(
            wrapped_div_u64(lhs_wrapped, rhs_wrapped).raw(),
            raw_div_u64(lhs, rhs)
        );
        assert_eq!(
            wrapped_rem_u64(lhs_wrapped, rhs_wrapped).raw(),
            raw_rem_u64(lhs, rhs)
        );
    }
    assert_eq!(
        wrapped_bitand_u64(lhs_wrapped, rhs_wrapped).raw(),
        raw_bitand_u64(lhs, rhs)
    );
    assert_eq!(
        wrapped_bitor_u64(lhs_wrapped, rhs_wrapped).raw(),
        raw_bitor_u64(lhs, rhs)
    );
    assert_eq!(
        wrapped_bitxor_u64(lhs_wrapped, rhs_wrapped).raw(),
        raw_bitxor_u64(lhs, rhs)
    );
    assert_eq!(wrapped_not_u64(lhs_wrapped).raw(), raw_not_u64(lhs));
    assert_eq!(
        wrapped_shl_u64(lhs_wrapped, shift).raw(),
        raw_shl_u64(lhs, shift)
    );
    assert_eq!(
        wrapped_shr_u64(lhs_wrapped, shift).raw(),
        raw_shr_u64(lhs, shift)
    );
}
