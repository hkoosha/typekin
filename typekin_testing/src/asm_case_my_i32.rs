#[typekin::integral]
pub struct MyI32(i32);

/// Symmetric: `Neg` and `abs` need no check.
#[typekin::integral(domain = -10..=10)]
pub struct MySymmetric(i32);

/// Non-negative: `abs` needs no check.
#[typekin::integral(domain = 0..=10)]
pub struct MyNonNegative(i32);

#[typekin::integral]
pub struct MyU64(u64);

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_neg_i32(value: i32) -> i32 {
    return -value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_neg_i32(value: MyI32) -> MyI32 {
    return -value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_neg_symmetric(value: i32) -> i32 {
    return -value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_neg_symmetric(value: MySymmetric) -> MySymmetric {
    return -value;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_abs_symmetric(value: i32) -> i32 {
    return value.abs();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_abs_symmetric(value: MySymmetric) -> MySymmetric {
    return value.abs();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_abs_non_negative(value: i32) -> i32 {
    return value.abs();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_abs_non_negative(
    value: MyNonNegative
) -> MyNonNegative {
    return value.abs();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_into_f64_i32(value: i32) -> f64 {
    return value as f64;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_into_f64_i32(value: MyI32) -> f64 {
    return value.into_f64();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_f32_i32(value: i32) -> Result<f32, ()> {
    let converted = value as f32;
    return if value == converted as i32 && converted < 2_147_483_648.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_f32_i32(value: MyI32) -> Result<f32, ()> {
    return value.try_into_f32();
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn raw_try_into_f64_u64(value: u64) -> Result<f64, ()> {
    let converted = value as f64;
    return if value == converted as u64
        && converted < 18_446_744_073_709_551_616.0
    {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn wrapped_try_into_f64_u64(value: MyU64) -> Result<f64, ()> {
    return value.try_into_f64();
}

/// The same check as `raw_try_into_f32_i32` behind one `#[inline(always)]` layer.
#[inline(always)]
const fn written_try_into_f32_i32_body(value: i32) -> Result<f32, ()> {
    let converted = value as f32;
    return if value == converted as i32 && converted < 2_147_483_648.0 {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_f32_i32(value: i32) -> Result<f32, ()> {
    return written_try_into_f32_i32_body(value);
}

/// The same check as `raw_try_into_f64_u64` behind one `#[inline(always)]` layer.
#[inline(always)]
const fn written_try_into_f64_u64_body(value: u64) -> Result<f64, ()> {
    let converted = value as f64;
    return if value == converted as u64
        && converted < 18_446_744_073_709_551_616.0
    {
        Ok(converted)
    }
    else {
        Err(())
    };
}

#[allow(improper_ctypes_definitions)]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn written_try_into_f64_u64(value: u64) -> Result<f64, ()> {
    return written_try_into_f64_u64_body(value);
}

pub fn main() {
    for value in [-10, -3, 0, 4, 10] {
        let symmetric = MySymmetric::try_make(value).unwrap();
        assert_eq!(
            wrapped_neg_symmetric(symmetric).raw(),
            raw_neg_symmetric(value)
        );
        assert_eq!(
            wrapped_abs_symmetric(symmetric).raw(),
            raw_abs_symmetric(value)
        );
    }
    for value in [0, 4, 10] {
        let non_negative = MyNonNegative::try_make(value).unwrap();
        assert_eq!(
            wrapped_abs_non_negative(non_negative).raw(),
            raw_abs_non_negative(value)
        );
    }
    for value in [
        i32::MIN + 1,
        -16_777_217,
        -1,
        0,
        16_777_216,
        16_777_217,
        i32::MAX,
    ] {
        let wrapped = MyI32::make(value);
        assert_eq!(wrapped_neg_i32(wrapped).raw(), raw_neg_i32(value));
        assert_eq!(
            wrapped_into_f64_i32(wrapped).to_bits(),
            raw_into_f64_i32(value).to_bits()
        );
        assert_eq!(
            wrapped_try_into_f32_i32(wrapped),
            raw_try_into_f32_i32(value)
        );
        assert_eq!(
            written_try_into_f32_i32(value),
            raw_try_into_f32_i32(value)
        );
    }
    for value in [0, 1 << 53, (1 << 53) + 1, 1 << 63, u64::MAX] {
        let wrapped = MyU64::make(value);
        assert_eq!(
            wrapped_try_into_f64_u64(wrapped),
            raw_try_into_f64_u64(value)
        );
        assert_eq!(
            written_try_into_f64_u64(value),
            raw_try_into_f64_u64(value)
        );
    }
}
