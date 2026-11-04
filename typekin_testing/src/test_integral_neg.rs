//! Signed-integer `Neg`, unchecked `Neg`/`abs` on closed domains, and
//! integer-to-float conversions as a consumer sees them.

use crate::fx_integral_neg::Checked;
use crate::fx_integral_neg::*;
use crate::*;

#[cfg(test)]
#[cfg(feature = "test_fast")]
mod neg {
    use super::*;

    #[test]
    fn signed_neg_matches_raw() {
        let value = Signed::make(5);

        assert_eq!((-value).raw(), -5);
    }

    #[test]
    #[should_panic(expected = "operation overflowed")]
    fn sequential_neg_of_minimum_panics() {
        let minimum = Checked::make(i32::MIN);

        let _ = -minimum;
    }

    #[test]
    fn circular_neg_of_minimum_wraps() {
        let minimum = Turn::make(i8::MIN);
        let one = Turn::make(1);

        assert_eq!((-minimum).raw(), i8::MIN);
        assert_eq!((-one).raw(), -1);
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn neg_outside_the_domain_panics() {
        let value = NonNegative::try_make(5).unwrap();

        let _ = -value;
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn neg_on_an_asymmetric_domain_still_validates() {
        let value = Lopsided::try_make(-10).unwrap();

        let _ = -value;
    }

    #[test]
    fn neg_on_a_symmetric_domain_stays_valid() {
        let lowest = Symmetric::try_make(-10).unwrap();
        let highest = Symmetric::try_make(10).unwrap();

        assert_eq!((-lowest).raw(), 10);
        assert_eq!((-highest).raw(), -10);
    }

    #[test]
    fn neg_keeps_non_zero_storage() {
        let value = NotZero::try_make(-3).unwrap();

        assert_eq!((-value).raw(), 3);
        assert_eq!(size_of::<Option<NotZero>>(), size_of::<NotZero>());
    }

    #[test]
    fn abs_on_a_symmetric_domain_stays_valid() {
        let value = Symmetric::try_make(-7).unwrap();

        assert_eq!(value.abs().raw(), 7);
    }

    #[test]
    fn abs_on_a_non_negative_domain_is_the_identity() {
        let value = NonNegative::try_make(7).unwrap();

        assert_eq!(value.abs().raw(), 7);
    }
}

#[cfg(test)]
#[cfg(feature = "test_fast")]
mod integer_to_float {
    use super::*;

    #[test]
    fn lossless_float_conversion_is_exact() {
        let value = Count::make(u32::MAX);

        assert_eq!(value.into_f64(), 4_294_967_295.0);
        let converted: f64 = value.into();
        assert_eq!(converted, 4_294_967_295.0);
        assert_eq!(value.try_into_f64(), Ok(4_294_967_295.0));
    }

    #[test]
    fn narrowing_float_conversion_rejects_rounding() {
        let exact = Count::make(16_777_216);
        let rounded = Count::make(16_777_217);

        assert_eq!(exact.try_into_f32(), Ok(16_777_216.0));
        assert_eq!(rounded.try_into_f32(), Err(()));
    }

    #[test]
    fn float_conversion_rejects_a_maximum_that_rounds_up() {
        let maximum = Wide::make(u64::MAX);
        let exact = Wide::make(1 << 63);

        assert_eq!(maximum.try_into_f64(), Err(()));
        assert_eq!(exact.try_into_f64(), Ok(9_223_372_036_854_775_808.0));
    }

    #[test]
    fn f32_conversion_rejects_u128_maximum() {
        let maximum = Huge::make(u128::MAX);
        let exact = Huge::make(1 << 127);

        assert_eq!(maximum.try_into_f32(), Err(()));
        assert_eq!(exact.try_into_f32(), Ok(2.0_f32.powi(127)));
    }

    #[test]
    fn float_conversion_accepts_the_signed_minimum() {
        let minimum = Offset::make(i64::MIN);
        let maximum = Offset::make(i64::MAX);

        assert_eq!(minimum.try_into_f64(), Ok(-9_223_372_036_854_775_808.0));
        assert_eq!(maximum.try_into_f64(), Err(()));
    }

    #[test]
    fn float_try_into_trait_forwards_to_the_checked_conversion() {
        let value = Wide::make(3);

        let converted: Result<f32, ()> = value.try_into();
        assert_eq!(converted, Ok(3.0));
    }

    #[test]
    fn usize_float_conversion_follows_the_target_width() {
        let maximum = Index::make(usize::MAX);

        assert_eq!(maximum.try_into_f64().is_ok(), usize::BITS <= 53);
        assert_eq!(Index::make(7).try_into_f64(), Ok(7.0));
    }
}
