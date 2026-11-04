#[typekin::float]
pub struct MyF32(f32);

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_add_f32(
    lhs: f32,
    rhs: f32,
) -> f32 {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_add_f32(
    lhs: MyF32,
    rhs: MyF32,
) -> MyF32 {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_sub_f32(
    lhs: f32,
    rhs: f32,
) -> f32 {
    return lhs - rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_sub_f32(
    lhs: MyF32,
    rhs: MyF32,
) -> MyF32 {
    return lhs - rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_mul_f32(
    lhs: f32,
    rhs: f32,
) -> f32 {
    return lhs * rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_mul_f32(
    lhs: MyF32,
    rhs: MyF32,
) -> MyF32 {
    return lhs * rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_div_f32(
    lhs: f32,
    rhs: f32,
) -> f32 {
    return lhs / rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_div_f32(
    lhs: MyF32,
    rhs: MyF32,
) -> MyF32 {
    return lhs / rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_rem_f32(
    lhs: f32,
    rhs: f32,
) -> f32 {
    return lhs % rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_rem_f32(
    lhs: MyF32,
    rhs: MyF32,
) -> MyF32 {
    return lhs % rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_add_assign_f32(
    mut lhs: f32,
    rhs: f32,
) -> f32 {
    lhs += rhs;
    return lhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_add_assign_f32(
    mut lhs: MyF32,
    rhs: MyF32,
) -> MyF32 {
    lhs += rhs;
    return lhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_mul_assign_f32(
    mut lhs: f32,
    rhs: f32,
) -> f32 {
    lhs *= rhs;
    return lhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_mul_assign_f32(
    mut lhs: MyF32,
    rhs: MyF32,
) -> MyF32 {
    lhs *= rhs;
    return lhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_neg_f32(value: f32) -> f32 {
    return -value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_neg_f32(value: MyF32) -> MyF32 {
    return -value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_add_prim_f32(
    lhs: f32,
    rhs: f32,
) -> f32 {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_add_prim_f32(
    lhs: MyF32,
    rhs: f32,
) -> MyF32 {
    return lhs + rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_eq_f32(
    lhs: f32,
    rhs: f32,
) -> bool {
    return lhs == rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_eq_f32(
    lhs: MyF32,
    rhs: MyF32,
) -> bool {
    return lhs == rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_ne_f32(
    lhs: f32,
    rhs: f32,
) -> bool {
    return lhs != rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_ne_f32(
    lhs: MyF32,
    rhs: MyF32,
) -> bool {
    return lhs != rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_lt_f32(
    lhs: f32,
    rhs: f32,
) -> bool {
    return lhs < rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_lt_f32(
    lhs: MyF32,
    rhs: MyF32,
) -> bool {
    return lhs < rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_le_f32(
    lhs: f32,
    rhs: f32,
) -> bool {
    return lhs <= rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_le_f32(
    lhs: MyF32,
    rhs: MyF32,
) -> bool {
    return lhs <= rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_gt_f32(
    lhs: f32,
    rhs: f32,
) -> bool {
    return lhs > rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_gt_f32(
    lhs: MyF32,
    rhs: MyF32,
) -> bool {
    return lhs > rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_ge_f32(
    lhs: f32,
    rhs: f32,
) -> bool {
    return lhs >= rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_ge_f32(
    lhs: MyF32,
    rhs: MyF32,
) -> bool {
    return lhs >= rhs;
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_partial_cmp_f32(
    lhs: f32,
    rhs: f32,
) -> Option<core::cmp::Ordering> {
    return lhs.partial_cmp(&rhs);
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_partial_cmp_f32(
    lhs: MyF32,
    rhs: MyF32,
) -> Option<core::cmp::Ordering> {
    return lhs.partial_cmp(&rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_map_f32(
    value: f32,
    rhs: f32,
) -> f32 {
    return value * rhs;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_map_f32(
    value: MyF32,
    rhs: f32,
) -> MyF32 {
    return value.map(|raw| raw * rhs);
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_into_f64_f32(value: f32) -> f64 {
    return value as f64;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_into_f64_f32(value: MyF32) -> f64 {
    return value.into_f64();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_u8_f32(value: f32) -> Result<u8, ()> {
    let converted = value as u8;
    return if converted as f32 == value {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_u8_f32(value: MyF32) -> Result<u8, ()> {
    return value.try_into_u8();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_i32_f32(value: f32) -> Result<i32, ()> {
    let converted = value as i32;
    return if converted as f32 == value && value < 2147483648.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_i32_f32(value: MyF32) -> Result<i32, ()> {
    return value.try_into_i32();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_u32_f32(value: f32) -> Result<u32, ()> {
    let converted = value as u32;
    return if converted as f32 == value && value < 4294967296.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_u32_f32(value: MyF32) -> Result<u32, ()> {
    return value.try_into_u32();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_i64_f32(value: f32) -> Result<i64, ()> {
    let converted = value as i64;
    return if converted as f32 == value && value < 9223372036854775808.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_i64_f32(value: MyF32) -> Result<i64, ()> {
    return value.try_into_i64();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_u64_f32(value: f32) -> Result<u64, ()> {
    let converted = value as u64;
    return if converted as f32 == value && value < 18446744073709551616.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_u64_f32(value: MyF32) -> Result<u64, ()> {
    return value.try_into_u64();
}

/// The same check as `raw_try_into_u8_f32` behind one `#[inline(always)]` layer:
/// below O2 that layer alone can change instruction selection.
#[inline(always)]
const fn written_try_into_u8_f32_body(value: f32) -> Result<u8, ()> {
    let converted = value as u8;
    return if converted as f32 == value {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_u8_f32(value: f32) -> Result<u8, ()> {
    return written_try_into_u8_f32_body(value);
}

/// The same check as `raw_try_into_i32_f32` behind one `#[inline(always)]` layer:
/// below O2 that layer alone can change instruction selection.
#[inline(always)]
const fn written_try_into_i32_f32_body(value: f32) -> Result<i32, ()> {
    let converted = value as i32;
    return if converted as f32 == value && value < 2147483648.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_i32_f32(value: f32) -> Result<i32, ()> {
    return written_try_into_i32_f32_body(value);
}

/// The same check as `raw_try_into_u32_f32` behind one `#[inline(always)]` layer:
/// below O2 that layer alone can change instruction selection.
#[inline(always)]
const fn written_try_into_u32_f32_body(value: f32) -> Result<u32, ()> {
    let converted = value as u32;
    return if converted as f32 == value && value < 4294967296.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_u32_f32(value: f32) -> Result<u32, ()> {
    return written_try_into_u32_f32_body(value);
}

/// The same check as `raw_try_into_i64_f32` behind one `#[inline(always)]` layer:
/// below O2 that layer alone can change instruction selection.
#[inline(always)]
const fn written_try_into_i64_f32_body(value: f32) -> Result<i64, ()> {
    let converted = value as i64;
    return if converted as f32 == value && value < 9223372036854775808.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_i64_f32(value: f32) -> Result<i64, ()> {
    return written_try_into_i64_f32_body(value);
}

/// The same check as `raw_try_into_u64_f32` behind one `#[inline(always)]` layer:
/// below O2 that layer alone can change instruction selection.
#[inline(always)]
const fn written_try_into_u64_f32_body(value: f32) -> Result<u64, ()> {
    let converted = value as u64;
    return if converted as f32 == value && value < 18446744073709551616.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_u64_f32(value: f32) -> Result<u64, ()> {
    return written_try_into_u64_f32_body(value);
}

pub fn main() {
    let values: [f32; 12] = [
        0.0,
        -0.0,
        1.5,
        -2.25,
        1e30,
        f32::NAN,
        f32::INFINITY,
        255.0,
        -9_223_372_036_854_775_808.0,
        9_223_372_036_854_775_808.0,
        4_294_967_295.0,
        0.1,
    ];
    for lhs in values {
        for rhs in values {
            let lhs_w = MyF32::make(lhs);
            let rhs_w = MyF32::make(rhs);
            assert_eq!(
                wrapped_add_f32(lhs_w, rhs_w).raw().to_bits(),
                raw_add_f32(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_sub_f32(lhs_w, rhs_w).raw().to_bits(),
                raw_sub_f32(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_mul_f32(lhs_w, rhs_w).raw().to_bits(),
                raw_mul_f32(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_div_f32(lhs_w, rhs_w).raw().to_bits(),
                raw_div_f32(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_rem_f32(lhs_w, rhs_w).raw().to_bits(),
                raw_rem_f32(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_add_assign_f32(lhs_w, rhs_w).raw().to_bits(),
                raw_add_assign_f32(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_mul_assign_f32(lhs_w, rhs_w).raw().to_bits(),
                raw_mul_assign_f32(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_neg_f32(lhs_w).raw().to_bits(),
                raw_neg_f32(lhs).to_bits()
            );
            assert_eq!(
                wrapped_add_prim_f32(lhs_w, rhs).raw().to_bits(),
                raw_add_prim_f32(lhs, rhs).to_bits()
            );
            assert_eq!(wrapped_eq_f32(lhs_w, rhs_w), raw_eq_f32(lhs, rhs));
            assert_eq!(wrapped_ne_f32(lhs_w, rhs_w), raw_ne_f32(lhs, rhs));
            assert_eq!(wrapped_lt_f32(lhs_w, rhs_w), raw_lt_f32(lhs, rhs));
            assert_eq!(wrapped_le_f32(lhs_w, rhs_w), raw_le_f32(lhs, rhs));
            assert_eq!(wrapped_gt_f32(lhs_w, rhs_w), raw_gt_f32(lhs, rhs));
            assert_eq!(wrapped_ge_f32(lhs_w, rhs_w), raw_ge_f32(lhs, rhs));
            assert_eq!(
                wrapped_partial_cmp_f32(lhs_w, rhs_w),
                raw_partial_cmp_f32(lhs, rhs)
            );
            assert_eq!(
                wrapped_map_f32(lhs_w, rhs).raw().to_bits(),
                raw_map_f32(lhs, rhs).to_bits()
            );
            assert_eq!(
                wrapped_into_f64_f32(lhs_w).to_bits(),
                raw_into_f64_f32(lhs).to_bits()
            );
            assert_eq!(
                wrapped_try_into_u8_f32(lhs_w),
                raw_try_into_u8_f32(lhs)
            );
            assert_eq!(
                wrapped_try_into_i32_f32(lhs_w),
                raw_try_into_i32_f32(lhs)
            );
            assert_eq!(
                wrapped_try_into_u32_f32(lhs_w),
                raw_try_into_u32_f32(lhs)
            );
            assert_eq!(
                wrapped_try_into_i64_f32(lhs_w),
                raw_try_into_i64_f32(lhs)
            );
            assert_eq!(
                wrapped_try_into_u64_f32(lhs_w),
                raw_try_into_u64_f32(lhs)
            );
            assert_eq!(written_try_into_u8_f32(lhs), raw_try_into_u8_f32(lhs));
            assert_eq!(
                written_try_into_i32_f32(lhs),
                raw_try_into_i32_f32(lhs)
            );
            assert_eq!(
                written_try_into_u32_f32(lhs),
                raw_try_into_u32_f32(lhs)
            );
            assert_eq!(
                written_try_into_i64_f32(lhs),
                raw_try_into_i64_f32(lhs)
            );
            assert_eq!(
                written_try_into_u64_f32(lhs),
                raw_try_into_u64_f32(lhs)
            );
        }
    }
}
