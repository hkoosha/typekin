#[typekin::float(math = std)]
pub struct MyF64(f64);

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_add_f64(
    lhs: f64,
    rhs: f64,
) -> f64 {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_add_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> MyF64 {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_sub_f64(
    lhs: f64,
    rhs: f64,
) -> f64 {
    return lhs - rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_sub_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> MyF64 {
    return lhs - rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_mul_f64(
    lhs: f64,
    rhs: f64,
) -> f64 {
    return lhs * rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_mul_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> MyF64 {
    return lhs * rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_div_f64(
    lhs: f64,
    rhs: f64,
) -> f64 {
    return lhs / rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_div_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> MyF64 {
    return lhs / rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_rem_f64(
    lhs: f64,
    rhs: f64,
) -> f64 {
    return lhs % rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_rem_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> MyF64 {
    return lhs % rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_add_assign_f64(
    mut lhs: f64,
    rhs: f64,
) -> f64 {
    lhs += rhs;
    return lhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_add_assign_f64(
    mut lhs: MyF64,
    rhs: MyF64,
) -> MyF64 {
    lhs += rhs;
    return lhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_mul_assign_f64(
    mut lhs: f64,
    rhs: f64,
) -> f64 {
    lhs *= rhs;
    return lhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_mul_assign_f64(
    mut lhs: MyF64,
    rhs: MyF64,
) -> MyF64 {
    lhs *= rhs;
    return lhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_neg_f64(value: f64) -> f64 {
    return -value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_neg_f64(value: MyF64) -> MyF64 {
    return -value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_add_prim_f64(
    lhs: f64,
    rhs: f64,
) -> f64 {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_add_prim_f64(
    lhs: MyF64,
    rhs: f64,
) -> MyF64 {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_eq_f64(
    lhs: f64,
    rhs: f64,
) -> bool {
    return lhs == rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_eq_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> bool {
    return lhs == rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_ne_f64(
    lhs: f64,
    rhs: f64,
) -> bool {
    return lhs != rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_ne_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> bool {
    return lhs != rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_lt_f64(
    lhs: f64,
    rhs: f64,
) -> bool {
    return lhs < rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_lt_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> bool {
    return lhs < rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_le_f64(
    lhs: f64,
    rhs: f64,
) -> bool {
    return lhs <= rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_le_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> bool {
    return lhs <= rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_gt_f64(
    lhs: f64,
    rhs: f64,
) -> bool {
    return lhs > rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_gt_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> bool {
    return lhs > rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_ge_f64(
    lhs: f64,
    rhs: f64,
) -> bool {
    return lhs >= rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_ge_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> bool {
    return lhs >= rhs;
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_partial_cmp_f64(
    lhs: f64,
    rhs: f64,
) -> Option<core::cmp::Ordering> {
    return lhs.partial_cmp(&rhs);
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_partial_cmp_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> Option<core::cmp::Ordering> {
    return lhs.partial_cmp(&rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_map_f64(
    value: f64,
    rhs: f64,
) -> f64 {
    return value * rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_map_f64(
    value: MyF64,
    rhs: f64,
) -> MyF64 {
    return value.map(|raw| raw * rhs);
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_f32_f64(value: f64) -> Result<f32, ()> {
    let narrowed = value as f32;
    return if narrowed as f64 == value || value.is_nan() {
        Ok(narrowed)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_f32_f64(value: MyF64) -> Result<f32, ()> {
    return value.try_into_f32();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_u8_f64(value: f64) -> Result<u8, ()> {
    let converted = value as u8;
    return if converted as f64 == value {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_u8_f64(value: MyF64) -> Result<u8, ()> {
    return value.try_into_u8();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_i32_f64(value: f64) -> Result<i32, ()> {
    let converted = value as i32;
    return if converted as f64 == value {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_i32_f64(value: MyF64) -> Result<i32, ()> {
    return value.try_into_i32();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_u32_f64(value: f64) -> Result<u32, ()> {
    let converted = value as u32;
    return if converted as f64 == value {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_u32_f64(value: MyF64) -> Result<u32, ()> {
    return value.try_into_u32();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_i64_f64(value: f64) -> Result<i64, ()> {
    let converted = value as i64;
    return if converted as f64 == value && value < 9223372036854775808.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_i64_f64(value: MyF64) -> Result<i64, ()> {
    return value.try_into_i64();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_u64_f64(value: f64) -> Result<u64, ()> {
    let converted = value as u64;
    return if converted as f64 == value && value < 18446744073709551616.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_u64_f64(value: MyF64) -> Result<u64, ()> {
    return value.try_into_u64();
}

/// The same check as `raw_try_into_f32_f64` behind one `#[inline(always)]` layer:
/// below O2 that layer alone can change instruction selection.
#[inline(always)]
const fn written_try_into_f32_f64_body(value: f64) -> Result<f32, ()> {
    let narrowed = value as f32;
    return if narrowed as f64 == value || value.is_nan() {
        Ok(narrowed)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_f32_f64(value: f64) -> Result<f32, ()> {
    return written_try_into_f32_f64_body(value);
}

/// The same check as `raw_try_into_u8_f64` behind one `#[inline(always)]` layer:
/// below O2 that layer alone can change instruction selection.
#[inline(always)]
const fn written_try_into_u8_f64_body(value: f64) -> Result<u8, ()> {
    let converted = value as u8;
    return if converted as f64 == value {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_u8_f64(value: f64) -> Result<u8, ()> {
    return written_try_into_u8_f64_body(value);
}

/// The same check as `raw_try_into_i32_f64` behind one `#[inline(always)]` layer:
/// below O2 that layer alone can change instruction selection.
#[inline(always)]
const fn written_try_into_i32_f64_body(value: f64) -> Result<i32, ()> {
    let converted = value as i32;
    return if converted as f64 == value {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_i32_f64(value: f64) -> Result<i32, ()> {
    return written_try_into_i32_f64_body(value);
}

/// The same check as `raw_try_into_u32_f64` behind one `#[inline(always)]` layer:
/// below O2 that layer alone can change instruction selection.
#[inline(always)]
const fn written_try_into_u32_f64_body(value: f64) -> Result<u32, ()> {
    let converted = value as u32;
    return if converted as f64 == value {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_u32_f64(value: f64) -> Result<u32, ()> {
    return written_try_into_u32_f64_body(value);
}

/// The same check as `raw_try_into_i64_f64` behind one `#[inline(always)]` layer:
/// below O2 that layer alone can change instruction selection.
#[inline(always)]
const fn written_try_into_i64_f64_body(value: f64) -> Result<i64, ()> {
    let converted = value as i64;
    return if converted as f64 == value && value < 9223372036854775808.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_i64_f64(value: f64) -> Result<i64, ()> {
    return written_try_into_i64_f64_body(value);
}

/// The same check as `raw_try_into_u64_f64` behind one `#[inline(always)]` layer:
/// below O2 that layer alone can change instruction selection.
#[inline(always)]
const fn written_try_into_u64_f64_body(value: f64) -> Result<u64, ()> {
    let converted = value as u64;
    return if converted as f64 == value && value < 18446744073709551616.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_u64_f64(value: f64) -> Result<u64, ()> {
    return written_try_into_u64_f64_body(value);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_abs_f64(value: f64) -> f64 {
    return value.abs();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_abs_f64(value: MyF64) -> MyF64 {
    return value.abs();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_signum_f64(value: f64) -> f64 {
    return value.signum();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_signum_f64(value: MyF64) -> MyF64 {
    return value.signum();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_copysign_f64(
    lhs: f64,
    rhs: f64,
) -> f64 {
    return lhs.copysign(rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_copysign_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> MyF64 {
    return lhs.copysign(rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_recip_f64(value: f64) -> f64 {
    return value.recip();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_recip_f64(value: MyF64) -> MyF64 {
    return value.recip();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_min_f64(
    lhs: f64,
    rhs: f64,
) -> f64 {
    return lhs.min(rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_min_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> MyF64 {
    return lhs.min(rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_max_f64(
    lhs: f64,
    rhs: f64,
) -> f64 {
    return lhs.max(rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_max_f64(
    lhs: MyF64,
    rhs: MyF64,
) -> MyF64 {
    return lhs.max(rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_to_bits_f64(value: f64) -> u64 {
    return value.to_bits();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_to_bits_f64(value: MyF64) -> u64 {
    return value.to_bits();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_from_bits_f64(bits: u64) -> f64 {
    return f64::from_bits(bits);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_from_bits_f64(bits: u64) -> MyF64 {
    return MyF64::from_bits(bits);
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_classify_f64(value: f64) -> core::num::FpCategory {
    return value.classify();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_classify_f64(value: MyF64) -> core::num::FpCategory {
    return value.classify();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_sum_f64(values: &[f64; 4]) -> f64 {
    return values.iter().sum();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_sum_f64(values: &[MyF64; 4]) -> MyF64 {
    return values.iter().sum();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_product_f64(values: &[f64; 4]) -> f64 {
    return values.iter().product();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_product_f64(values: &[MyF64; 4]) -> MyF64 {
    return values.iter().product();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_dot_f64(
    lhs: &[f64; 4],
    rhs: &[f64; 4],
) -> f64 {
    return lhs.iter().zip(rhs).map(|(lhs, rhs)| lhs * rhs).sum();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_dot_f64(
    lhs: &[MyF64; 4],
    rhs: &[MyF64; 4],
) -> MyF64 {
    return lhs.iter().zip(rhs).map(|(lhs, rhs)| *lhs * *rhs).sum();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_floor_f64(value: f64) -> f64 {
    return value.floor();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_floor_f64(value: MyF64) -> MyF64 {
    return value.floor();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_sqrt_f64(value: f64) -> f64 {
    return value.sqrt();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_sqrt_f64(value: MyF64) -> MyF64 {
    return value.sqrt();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_mul_add_f64(
    value: f64,
    a: f64,
    b: f64,
) -> f64 {
    return value.mul_add(a, b);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_mul_add_f64(
    value: MyF64,
    a: MyF64,
    b: MyF64,
) -> MyF64 {
    return value.mul_add(a, b);
}

pub fn main() {
    let values: [f64; 12] = [
        0.0,
        -0.0,
        1.5,
        -2.25,
        1e300,
        f64::NAN,
        f64::INFINITY,
        255.0,
        -9_223_372_036_854_775_808.0,
        9_223_372_036_854_775_808.0,
        4_294_967_295.0,
        0.1,
    ];
    for lhs in values {
        for rhs in values {
            let lhs_w = MyF64::make(lhs);
            let rhs_w = MyF64::make(rhs);
            assert_eq!(
                wrapped_add_f64(lhs_w, rhs_w).raw().to_bits(),
                raw_add_f64(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_sub_f64(lhs_w, rhs_w).raw().to_bits(),
                raw_sub_f64(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_mul_f64(lhs_w, rhs_w).raw().to_bits(),
                raw_mul_f64(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_div_f64(lhs_w, rhs_w).raw().to_bits(),
                raw_div_f64(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_rem_f64(lhs_w, rhs_w).raw().to_bits(),
                raw_rem_f64(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_add_assign_f64(lhs_w, rhs_w).raw().to_bits(),
                raw_add_assign_f64(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_mul_assign_f64(lhs_w, rhs_w).raw().to_bits(),
                raw_mul_assign_f64(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_neg_f64(lhs_w).raw().to_bits(),
                raw_neg_f64(lhs).to_bits()
            );
            assert_eq!(
                wrapped_add_prim_f64(lhs_w, rhs).raw().to_bits(),
                raw_add_prim_f64(lhs, rhs).to_bits()
            );
            assert_eq!(wrapped_eq_f64(lhs_w, rhs_w), raw_eq_f64(lhs, rhs));
            assert_eq!(wrapped_ne_f64(lhs_w, rhs_w), raw_ne_f64(lhs, rhs));
            assert_eq!(wrapped_lt_f64(lhs_w, rhs_w), raw_lt_f64(lhs, rhs));
            assert_eq!(wrapped_le_f64(lhs_w, rhs_w), raw_le_f64(lhs, rhs));
            assert_eq!(wrapped_gt_f64(lhs_w, rhs_w), raw_gt_f64(lhs, rhs));
            assert_eq!(wrapped_ge_f64(lhs_w, rhs_w), raw_ge_f64(lhs, rhs));
            assert_eq!(
                wrapped_partial_cmp_f64(lhs_w, rhs_w),
                raw_partial_cmp_f64(lhs, rhs)
            );
            assert_eq!(
                wrapped_map_f64(lhs_w, rhs).raw().to_bits(),
                raw_map_f64(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_try_into_f32_f64(lhs_w).map(f32::to_bits),
                raw_try_into_f32_f64(lhs).map(f32::to_bits)
            );
            assert_eq!(
                wrapped_try_into_u8_f64(lhs_w),
                raw_try_into_u8_f64(lhs)
            );
            assert_eq!(
                wrapped_try_into_i32_f64(lhs_w),
                raw_try_into_i32_f64(lhs)
            );
            assert_eq!(
                wrapped_try_into_u32_f64(lhs_w),
                raw_try_into_u32_f64(lhs)
            );
            assert_eq!(
                wrapped_try_into_i64_f64(lhs_w),
                raw_try_into_i64_f64(lhs)
            );
            assert_eq!(
                wrapped_try_into_u64_f64(lhs_w),
                raw_try_into_u64_f64(lhs)
            );
            assert_eq!(
                written_try_into_f32_f64(lhs).map(f32::to_bits),
                raw_try_into_f32_f64(lhs).map(f32::to_bits)
            );
            assert_eq!(written_try_into_u8_f64(lhs), raw_try_into_u8_f64(lhs));
            assert_eq!(
                written_try_into_i32_f64(lhs),
                raw_try_into_i32_f64(lhs)
            );
            assert_eq!(
                written_try_into_u32_f64(lhs),
                raw_try_into_u32_f64(lhs)
            );
            assert_eq!(
                written_try_into_i64_f64(lhs),
                raw_try_into_i64_f64(lhs)
            );
            assert_eq!(
                written_try_into_u64_f64(lhs),
                raw_try_into_u64_f64(lhs)
            );
            assert_eq!(
                wrapped_abs_f64(lhs_w).raw().to_bits(),
                raw_abs_f64(lhs).to_bits()
            );
            assert_eq!(
                wrapped_signum_f64(lhs_w).raw().to_bits(),
                raw_signum_f64(lhs).to_bits()
            );
            assert_eq!(
                wrapped_copysign_f64(lhs_w, rhs_w).raw().to_bits(),
                raw_copysign_f64(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_recip_f64(lhs_w).raw().to_bits(),
                raw_recip_f64(lhs).to_bits()
            );
            assert_eq!(
                wrapped_min_f64(lhs_w, rhs_w).raw().to_bits(),
                raw_min_f64(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_max_f64(lhs_w, rhs_w).raw().to_bits(),
                raw_max_f64(lhs, rhs).to_bits()
            );
            assert_eq!(wrapped_to_bits_f64(lhs_w), raw_to_bits_f64(lhs));
            assert_eq!(
                wrapped_from_bits_f64(lhs.to_bits()).raw().to_bits(),
                raw_from_bits_f64(lhs.to_bits()).to_bits()
            );
            assert_eq!(wrapped_classify_f64(lhs_w), raw_classify_f64(lhs));
            assert_eq!(
                wrapped_sum_f64(&[
                    lhs_w,
                    rhs_w,
                    rhs_w,
                    lhs_w
                ])
                .raw()
                .to_bits(),
                raw_sum_f64(&[
                    lhs,
                    rhs,
                    rhs,
                    lhs
                ])
                .to_bits()
            );
            assert_eq!(
                wrapped_product_f64(&[
                    lhs_w,
                    rhs_w,
                    rhs_w,
                    lhs_w
                ])
                .raw()
                .to_bits(),
                raw_product_f64(&[
                    lhs,
                    rhs,
                    rhs,
                    lhs
                ])
                .to_bits()
            );
            assert_eq!(
                wrapped_dot_f64(
                    &[
                        lhs_w,
                        rhs_w,
                        rhs_w,
                        lhs_w
                    ],
                    &[
                        rhs_w,
                        lhs_w,
                        lhs_w,
                        rhs_w
                    ]
                )
                .raw()
                .to_bits(),
                raw_dot_f64(
                    &[
                        lhs,
                        rhs,
                        rhs,
                        lhs
                    ],
                    &[
                        rhs,
                        lhs,
                        lhs,
                        rhs
                    ]
                )
                .to_bits()
            );
            assert_eq!(
                wrapped_floor_f64(lhs_w).raw().to_bits(),
                raw_floor_f64(lhs).to_bits()
            );
            assert_eq!(
                wrapped_sqrt_f64(lhs_w).raw().to_bits(),
                raw_sqrt_f64(lhs).to_bits()
            );
            assert_eq!(
                wrapped_mul_add_f64(lhs_w, rhs_w, lhs_w).raw().to_bits(),
                raw_mul_add_f64(lhs, rhs, lhs).to_bits()
            );
        }
    }
}
