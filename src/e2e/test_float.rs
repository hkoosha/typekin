//! `#[typekin::float]` and `#[typekin::point]` on `f32`/`f64` as a consumer
//! sees them: arithmetic, validation, NaN handling, totality, conversions,
//! parsing, formatting, serde, friends, generics, trees, and `produce`.

use core::cmp::Ordering;
use core::marker::PhantomData;
use std::collections::BTreeSet;
use std::hash::{
    DefaultHasher,
    Hash,
    Hasher,
};

use crate::fx_float::{
    Checked,
    *,
};
use crate::{
    accepts_eq,
    accepts_hash,
    accepts_ord,
    *,
};

#[cfg(test)]
mod arithmetic {
    use super::*;

    #[test]
    fn f64_arithmetic_matches_raw() {
        let lhs = Meters::make(7.5);
        let rhs = Meters::make(2.0);

        assert_eq!((lhs + rhs).raw(), 9.5);
        assert_eq!((lhs - rhs).raw(), 5.5);
        assert_eq!((lhs * rhs).raw(), 15.0);
        assert_eq!((lhs / rhs).raw(), 3.75);
        assert_eq!((lhs % rhs).raw(), 1.5);
    }

    #[test]
    fn f32_arithmetic_matches_raw() {
        let lhs = Small::make(0.1);
        let rhs = Small::make(0.2);

        assert_eq!((lhs + rhs).raw(), 0.1_f32 + 0.2_f32);
        assert_eq!((lhs * rhs).raw(), 0.1_f32 * 0.2_f32);
    }

    #[test]
    fn assignment_operators_update_in_place() {
        let mut value = Meters::make(1.0);
        let step = Meters::make(0.5);

        value += step;
        value *= Meters::make(4.0);
        value -= step;
        value /= Meters::make(2.0);

        assert_eq!(value.raw(), 2.75);
    }

    #[test]
    fn neg_flips_the_sign() {
        let value = Meters::make(2.5);
        let zero = Meters::make(0.0);

        assert_eq!((-value).raw(), -2.5);
        assert!((-zero).raw().is_sign_negative());
    }

    #[test]
    fn primitive_rhs_uses_the_self_friend() {
        let value = Meters::make(1.5);
        let rhs = 2.0;

        assert_eq!((value + rhs).raw(), 3.5);
        assert_eq!((value * rhs).raw(), 3.0);
    }

    #[test]
    fn float_division_by_zero_is_infinite() {
        let value = Meters::make(1.0);
        let zero = Meters::make(0.0);

        assert_eq!((value / zero).raw(), f64::INFINITY);
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn validated_arithmetic_panics_outside_the_domain() {
        let lhs = Unit::try_make(0.75).unwrap();
        let rhs = Unit::try_make(0.5).unwrap();

        let _ = lhs + rhs;
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn validated_neg_panics_outside_the_domain() {
        let value = Unit::try_make(0.5).unwrap();

        let _ = -value;
    }
}

#[cfg(test)]
mod validation {
    use super::*;

    #[test]
    fn try_make_rejects_values_outside_the_domain() {
        let inside = 0.5;
        let below = -0.25;
        let above = 1.25;

        assert_eq!(Unit::try_make(inside).map(Unit::raw), Ok(0.5));
        assert_eq!(Unit::try_make(below).map(Unit::raw), Err(-0.25));
        assert_eq!(Unit::try_make(above).map(Unit::raw), Err(1.25));
    }

    #[test]
    fn bounded_domain_rejects_nan() {
        let nan = f64::NAN;
        let this = Unit::try_make(nan);

        assert!(this.is_err());
        assert!(this.unwrap_err().is_nan());
    }

    #[test]
    fn domain_and_callback_both_apply() {
        let excluded_by_callback = 1.0;
        let excluded_by_domain = 3.0;
        let accepted = 1.5;

        assert!(Checked::try_make(excluded_by_callback).is_err());
        assert!(Checked::try_make(excluded_by_domain).is_err());
        assert!(Checked::try_make(accepted).is_ok());
    }

    #[test]
    fn not_equal_domain_admits_nan() {
        let nan = f64::NAN;
        let half = 0.5;

        assert!(NotHalf::try_make(nan).is_ok());
        assert!(NotHalf::try_make(half).is_err());
    }

    #[test]
    #[should_panic(expected = "mapping operation produced an invalid value")]
    fn map_panics_on_an_invalid_result() {
        let value = Unit::try_make(0.5).unwrap();

        let _ = value.map(|it| it * 4.0);
    }
}

#[cfg(test)]
mod nan_able_comparisons {
    use super::*;

    #[test]
    fn nan_is_unordered() {
        let nan = Meters::make(f64::NAN);
        let one = Meters::make(1.0);

        assert_eq!(nan.partial_cmp(&one), None);
        assert!(!(nan < one));
        assert!(!(nan >= one));
        assert!(nan != nan);
    }

    #[test]
    fn explicit_comparisons_match_raw() {
        let lhs = Meters::make(1.0);
        let rhs = Meters::make(2.0);

        assert!(lhs < rhs);
        assert!(lhs <= rhs);
        assert!(rhs > lhs);
        assert!(rhs >= lhs);
        assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Less));
    }
}

#[cfg(test)]
mod totality_of_nan_free_types {
    use super::*;

    #[test]
    fn nan_free_zeros_are_equal_and_hash_alike() {
        let positive = stable(0.0);
        let negative = stable(-0.0);

        assert_eq!(positive, negative);
        assert_eq!(positive.cmp(&negative), Ordering::Equal);
        assert_eq!(hash_of(positive), hash_of(negative));
    }

    #[test]
    fn nan_free_values_sort_totally() {
        let mut values = [
            stable(3.0),
            stable(f64::NEG_INFINITY),
            stable(-1.0),
            stable(f64::INFINITY),
        ];

        values.sort();

        let raw = values.map(Stable::raw);
        assert_eq!(
            raw,
            [
                f64::NEG_INFINITY,
                -1.0,
                3.0,
                f64::INFINITY
            ]
        );
    }

    #[test]
    fn nan_free_values_work_in_ordered_sets() {
        let set = [
            1.0,
            0.5,
            1.0,
            -0.0,
            0.0,
        ]
        .into_iter()
        .map(|it| Unit::try_make(it).unwrap())
        .collect::<BTreeSet<_>>();

        let raw = set.into_iter().map(Unit::raw).collect::<Vec<_>>();
        assert_eq!(
            raw,
            [
                0.0,
                0.5,
                1.0
            ]
        );
    }

    #[test]
    fn ord_max_returns_the_second_argument_on_equal() {
        let positive = stable(0.0);
        let negative = stable(-0.0);

        let max = Ord::max(positive, negative);

        assert!(max.raw().is_sign_negative());
    }

    #[test]
    fn ord_clamp_selects_a_valid_bound() {
        let low = Unit::try_make(0.25).unwrap();
        let high = Unit::try_make(0.75).unwrap();
        let value = Unit::try_make(1.0).unwrap();

        assert_eq!(value.clamp(low, high).raw(), 0.75);
    }

    #[test]
    fn single_zero_domain_hashes_raw_bits() {
        let value = AtLeastOne::try_make(2.0).unwrap();
        let raw = 2.0_f64;

        assert_eq!(hash_of(value), hash_of(raw.to_bits()));
    }
}

#[cfg(test)]
mod conversions {
    use super::*;

    #[test]
    fn widening_conversion_is_exact() {
        let value = Small::make(0.1);

        assert_eq!(value.into_f64(), f64::from(0.1_f32));
        let widened: f64 = value.into();
        assert_eq!(widened, f64::from(0.1_f32));
    }

    #[test]
    fn narrowing_keeps_exact_values() {
        let exact = Meters::make(0.5);

        assert_eq!(exact.try_into_f32(), Ok(0.5));
    }

    #[test]
    fn narrowing_rejects_rounding() {
        let rounded = Meters::make(0.1);

        assert_eq!(rounded.try_into_f32(), Err(()));
    }

    #[test]
    fn narrowing_keeps_nan_and_infinity() {
        let nan = Meters::make(f64::NAN);
        let infinity = Meters::make(f64::INFINITY);

        assert!(nan.try_into_f32().unwrap().is_nan());
        assert_eq!(infinity.try_into_f32(), Ok(f32::INFINITY));
    }

    #[test]
    fn narrowing_rejects_overflow() {
        let large = Meters::make(1e300);

        assert_eq!(large.try_into_f32(), Err(()));
    }

    #[test]
    fn integer_conversion_accepts_the_signed_minimum() {
        let minimum = Meters::make(-9_223_372_036_854_775_808.0);

        assert_eq!(minimum.try_into_i64(), Ok(i64::MIN));
    }

    #[test]
    fn integer_conversion_rejects_two_to_the_63() {
        let above = Meters::make(9_223_372_036_854_775_808.0);

        assert_eq!(above.try_into_i64(), Err(()));
    }

    #[test]
    fn integer_conversion_rejects_fractions() {
        let fraction = Meters::make(0.5);

        assert_eq!(fraction.try_into_i64(), Err(()));
    }

    #[test]
    fn integer_conversion_rejects_nan_and_infinity() {
        let nan = Meters::make(f64::NAN);
        let infinity = Meters::make(f64::INFINITY);
        let negative_infinity = Meters::make(f64::NEG_INFINITY);

        assert_eq!(nan.try_into_i64(), Err(()));
        assert_eq!(infinity.try_into_u64(), Err(()));
        assert_eq!(negative_infinity.try_into_i8(), Err(()));
    }

    #[test]
    fn integer_conversion_accepts_negative_zero() {
        let negative_zero = Meters::make(-0.0);

        assert_eq!(negative_zero.try_into_u8(), Ok(0));
    }

    #[test]
    fn unsigned_conversion_rejects_negatives_and_its_bound() {
        let negative = Meters::make(-1.0);
        let maximum = Meters::make(255.0);
        let bound = Meters::make(256.0);

        assert_eq!(negative.try_into_u8(), Err(()));
        assert_eq!(maximum.try_into_u8(), Ok(255));
        assert_eq!(bound.try_into_u8(), Err(()));
    }

    #[test]
    fn f32_maximum_converts_to_u128() {
        let maximum = Small::make(f32::MAX);

        assert_eq!(maximum.try_into_u128(), Ok(f32::MAX as u128));
    }

    #[test]
    fn usize_conversion_uses_the_target_width() {
        let maximum = Meters::make(usize::MAX as f64);
        let large = Meters::make(4_294_967_295.0);

        assert_eq!(maximum.try_into_usize(), Err(()));
        assert_eq!(
            large.try_into_usize().is_ok(),
            usize::BITS >= 32,
            "{}",
            usize::BITS
        );
    }

    #[test]
    fn try_into_trait_forwards_to_the_lossy_conversion() {
        let value = Meters::make(3.0);

        let converted: Result<i32, ()> = value.try_into();
        assert_eq!(converted, Ok(3));
    }
}

#[cfg(test)]
mod parsing_formatting_serde {
    use super::*;

    #[test]
    fn from_str_accepts_float_syntax() {
        let infinity = "inf";
        let exponent = "1.5e3";
        let nan = "NaN";

        assert_eq!(
            infinity.parse::<Meters>().map(Meters::raw),
            Ok(f64::INFINITY)
        );
        assert_eq!(exponent.parse::<Meters>().map(Meters::raw), Ok(1500.0));
        assert!(nan.parse::<Meters>().unwrap().raw().is_nan());
    }

    #[test]
    fn from_str_validates() {
        let nan = "NaN";
        let outside = "2.0";
        let inside = "0.25";

        assert_eq!(nan.parse::<Unit>(), Err(()));
        assert_eq!(outside.parse::<Unit>(), Err(()));
        assert_eq!(inside.parse::<Unit>().map(Unit::raw), Ok(0.25));
    }

    #[test]
    fn display_forwards_precision() {
        let value = SerdeUnit::try_make(0.125).unwrap();

        assert_eq!(format!("{value:.2}"), "0.12");
        assert_eq!(format!("{value}"), "0.125");
    }

    #[test]
    fn debug_shows_the_raw_value() {
        let value = Meters::make(1.5);

        assert_eq!(format!("{value:?}"), "Meters(1.5)");
    }

    #[test]
    fn serde_roundtrips_and_revalidates() {
        use serde_test::{
            Token,
            assert_de_tokens_error,
            assert_tokens,
        };

        let value = SerdeUnit::try_make(0.5).unwrap();

        assert_tokens(&value, &[Token::F64(0.5)]);
        assert_de_tokens_error::<SerdeUnit>(
            &[Token::F64(2.0)],
            "invalid value",
        );
    }
}

#[cfg(test)]
mod friends_constness_profiles {
    use super::*;

    #[test]
    fn f32_rel_friend_compares_exactly() {
        let value = Friendly::make(0.5);
        let friend = 0.5_f32;

        assert!(value == friend);
        assert!(value < 1.0_f32);
    }

    #[test]
    fn integer_make_friend_converts() {
        let value = Friendly::of(7_u32);

        assert_eq!(value.raw(), 7.0);
    }

    #[test]
    fn lossy_make_friend_rounds() {
        let large = u64::MAX;
        let value = Friendly::of(large);

        assert_eq!(value.raw(), 18_446_744_073_709_551_616.0);
    }

    #[test]
    fn exact_integral_newtype_friend_constructs() {
        let steps = Steps::make(u32::MAX);

        let distance = Distance::of(steps);

        assert_eq!(distance.raw(), 4_294_967_295.0);
    }

    #[test]
    fn conversion_friend_constructs() {
        let feet = Feet(10.0);
        let value = Converted::of(feet);

        assert_eq!(value.raw(), 3.048);
    }

    #[test]
    fn stable_konst_evaluates_at_compile_time() {
        const RAW: f64 = STABLE.raw();
        const CHECKED: Result<Stable, f64> = Stable::try_make(f64::NAN);
        const CONVERTED: Result<i64, ()> = STABLE.try_into_i64();
        const EXACT: Result<f32, ()> = STABLE.try_into_f32();

        assert_eq!(RAW, 1.5);
        assert!(CHECKED.is_err());
        assert_eq!(CONVERTED, Err(()));
        assert_eq!(EXACT, Ok(1.5));
    }

    #[test]
    fn point_produces_differences() {
        let start = Instant::make(1.5);
        let end = Instant::make(4.0);

        let difference: f64 = end - start;

        assert_eq!(difference, 2.5);
    }

    #[test]
    fn point_facet_offsets_by_its_friend() {
        let start = Position::make(1.5);
        let offset = Offset(2.0);

        let moved = start + offset;
        let back = moved - offset;
        let difference: f64 = moved - start;

        assert_eq!(moved.raw(), 3.5);
        assert_eq!(back.raw(), 1.5);
        assert_eq!(difference, 2.0);
    }

    #[test]
    fn tree_leaf_implements_its_branch() {
        fn accepts<T: FloatDirection>() {}

        accepts::<FloatIn>();
        assert_eq!(TreeFloat::make(1.0).raw(), 1.0);
    }

    #[test]
    fn generic_float_keeps_its_marker() {
        struct Marker;
        let value = Tagged::<Marker>::try_make(1.0);

        assert!(value.is_ok());
        assert_eq!(value.unwrap().raw(), 1.0);
        assert!(Tagged::<Marker>::try_make(-1.0).is_err());
    }

    #[test]
    fn layout_is_transparent() {
        assert_eq!(size_of::<Meters>(), size_of::<f64>());
        assert_eq!(align_of::<Small>(), align_of::<f32>());
        assert_eq!(size_of::<Unit>(), size_of::<f64>());
    }
}

#[cfg(test)]
mod core_float {
    use super::*;

    #[test]
    fn core_float_queries_forward_to_the_raw_value() {
        let nan = Meters::make(f64::NAN);
        let negative_zero = Meters::make(-0.0);
        let one = Meters::make(1.0);

        assert!(nan.is_nan());
        assert!(!nan.is_finite());
        assert!(negative_zero.is_sign_negative());
        assert_eq!(one.classify(), core::num::FpCategory::Normal);
        assert_eq!(one.to_bits(), 1.0_f64.to_bits());
        assert_eq!(one.to_le_bytes(), 1.0_f64.to_le_bytes());
    }

    #[test]
    fn decided_queries_answer_like_the_raw_value() {
        let value = 0.5;
        let this = Unit::try_make(value);
        assert!(this.is_ok(), "{value}");
        let half = this.unwrap();

        assert!(!half.is_nan(), "{value}");
        assert!(!half.is_infinite(), "{value}");
        assert!(half.is_finite(), "{value}");
    }

    #[test]
    fn valid_negative_zero_keeps_its_sign() {
        let value = -0.0;
        let this = Unit::try_make(value);
        assert!(this.is_ok(), "{value}");
        let negative_zero = this.unwrap();

        assert!(negative_zero.is_sign_negative(), "{value}");
        assert!(!negative_zero.is_sign_positive(), "{value}");
    }

    #[test]
    fn valid_subnormal_is_not_normal() {
        let value = f64::from_bits(1);
        let this = Unit::try_make(value);
        assert!(this.is_ok(), "{value:e}");
        let subnormal = this.unwrap();

        assert!(subnormal.is_subnormal(), "{value:e}");
        assert!(!subnormal.is_normal(), "{value:e}");
    }

    #[test]
    fn valid_infinity_is_not_finite() {
        let value = f64::INFINITY;
        let this = AtLeastOne::try_make(value);
        assert!(this.is_ok(), "{value}");
        let infinity = this.unwrap();

        assert!(infinity.is_infinite(), "{value}");
        assert!(!infinity.is_finite(), "{value}");
    }

    #[test]
    fn f32_subnormal_is_subnormal_in_f32() {
        let value = 1.0e-40_f32;
        let this = Tiny::try_make(value);
        assert!(this.is_ok(), "{value:e}");
        let tiny = this.unwrap();

        assert!(tiny.is_subnormal(), "{value:e}");
        assert!(!tiny.is_normal(), "{value:e}");
    }

    #[test]
    fn visible_field_queries_read_the_stored_value() {
        let value = f64::NAN;
        let stored = ExposedUnit(value);

        assert!(stored.is_nan(), "{value}");
        assert!(!stored.is_finite(), "{value}");
    }

    #[test]
    fn core_float_consts_describe_the_raw_type() {
        assert_eq!(Meters::MANTISSA_DIGITS, f64::MANTISSA_DIGITS);
        assert_eq!(Small::DIGITS, f32::DIGITS);
        assert_eq!(Small::MAX_EXP, f32::MAX_EXP);
    }

    #[test]
    fn core_float_operations_match_raw() {
        let value = Meters::make(-2.5);
        let sign = Meters::make(1.0);

        assert_eq!(value.abs().raw(), 2.5);
        assert_eq!(value.signum().raw(), -1.0);
        assert_eq!(value.copysign(sign).raw(), 2.5);
        assert_eq!(value.recip().raw(), -0.4);
        assert_eq!(value.midpoint(sign).raw(), -0.75);
        assert_eq!(sign.next_up().raw(), 1.0_f64.next_up());
    }

    #[test]
    #[should_panic(expected = "operation produced an invalid value")]
    fn core_float_operation_validates_its_result() {
        let half = Unit::try_make(0.5).unwrap();

        let _ = half.recip();
    }

    #[test]
    fn from_bits_constructs_a_valid_value() {
        let bits = 0.25_f64.to_bits();

        assert_eq!(Unit::from_bits(bits).raw(), 0.25);
    }

    #[test]
    #[should_panic(expected = "operation produced an invalid value")]
    fn from_bits_rejects_an_invalid_value() {
        let bits = 2.0_f64.to_bits();

        let _ = Unit::from_bits(bits);
    }

    #[test]
    fn nan_able_types_select_like_the_raw_type() {
        let nan = Meters::make(f64::NAN);
        let one = Meters::make(1.0);
        let two = Meters::make(2.0);

        assert_eq!(nan.max(one).raw(), 1.0);
        assert_eq!(two.min(one).raw(), 1.0);
        assert_eq!(Meters::make(5.0).clamp(one, two).raw(), 2.0);
    }

    #[test]
    fn total_cmp_orders_nan_and_zeros() {
        let nan = Meters::make(f64::NAN);
        let negative_zero = Meters::make(-0.0);
        let zero = Meters::make(0.0);

        assert_eq!(negative_zero.total_cmp(&zero), Ordering::Less);
        assert_eq!(zero.total_cmp(&nan), Ordering::Less);
    }
}

#[cfg(test)]
mod float_math {
    use super::*;

    #[test]
    fn std_math_forwards_to_inherent_methods() {
        let value = StdMath::make(2.25);
        let two = StdMath::make(2.0);
        let one = StdMath::make(1.0);

        assert_eq!(value.sqrt().raw(), 1.5);
        assert_eq!(value.floor().raw(), 2.0);
        assert_eq!(two.powi(3).raw(), 8.0);
        assert_eq!(value.mul_add(two, one).raw(), 5.5);
        assert_eq!(value.rem_euclid(two).raw(), 0.25);
    }

    #[test]
    fn std_math_sin_cos_returns_both() {
        let zero = StdMath::make(0.0);

        let (sin, cos) = zero.sin_cos();

        assert_eq!(sin.raw(), 0.0);
        assert_eq!(cos.raw(), 1.0);
    }
}

#[cfg(test)]
mod exponent_formatting {
    use super::*;

    #[test]
    fn exponent_formatting_forwards_to_the_raw_value() {
        let value = Meters::make(1500.0);

        assert_eq!(format!("{value:e}"), "1.5e3");
        assert_eq!(format!("{value:E}"), "1.5E3");
        assert_eq!(format!("{value:.2e}"), "1.50e3");
    }
}

#[cfg(test)]
mod sum_and_product {
    use super::*;

    #[test]
    fn sum_matches_the_raw_fold() {
        let values = [
            0.1,
            0.2,
            0.3,
        ];
        let wrapped = values.map(Meters::make);

        let sum: Meters = wrapped.iter().sum();

        assert_eq!(sum.raw(), values.iter().sum::<f64>());
    }

    #[test]
    fn empty_sum_is_negative_zero() {
        let empty: [Meters; 0] = [];

        let sum: Meters = empty.into_iter().sum();

        assert!(sum.raw().is_sign_negative());
        assert_eq!(sum.raw(), 0.0);
    }

    #[test]
    fn product_matches_the_raw_fold() {
        let values = [
            1.5,
            2.0,
            -4.0,
        ];
        let wrapped = values.map(Meters::make);

        let product: Meters = wrapped.into_iter().product();

        assert_eq!(product.raw(), -12.0);
    }

    #[test]
    fn empty_product_is_one() {
        let empty: [Meters; 0] = [];

        let product: Meters = empty.iter().product();

        assert_eq!(product.raw(), 1.0);
    }

    #[test]
    fn sum_validates_only_the_result() {
        let values = [
            0.75,
            0.75,
            -1.0,
        ];
        let wrapped = values.map(|it| Signed::try_make(it).unwrap());

        let sum: Signed = wrapped.into_iter().sum();

        assert_eq!(sum.raw(), 0.5);
    }

    #[test]
    #[should_panic(expected = "operation produced an invalid value")]
    fn sum_panics_on_an_invalid_result() {
        let values = [
            0.75,
            0.75,
        ];
        let wrapped = values.map(|it| Unit::try_make(it).unwrap());

        let _: Unit = wrapped.into_iter().sum();
    }

    #[test]
    #[should_panic(expected = "operation produced an invalid value")]
    fn empty_sum_panics_when_zero_is_invalid() {
        let empty: [AtLeastOne; 0] = [];

        let _: AtLeastOne = empty.into_iter().sum();
    }
}

#[cfg(test)]
mod closed_domains {
    use super::*;

    #[test]
    fn neg_on_a_symmetric_float_domain_stays_valid() {
        let lowest = Signed::try_make(-1.0).unwrap();

        assert_eq!((-lowest).raw(), 1.0);
    }

    #[test]
    #[should_panic(expected = "invalid value")]
    fn neg_on_an_asymmetric_float_domain_still_validates() {
        let lowest = Lopsided::try_make(-1.0).unwrap();

        let _ = -lowest;
    }

    #[test]
    fn abs_of_negative_zero_on_a_non_negative_domain_is_positive_zero() {
        let negative_zero = NonNegative::try_make(-0.0).unwrap();

        let absolute = negative_zero.abs();

        assert!(absolute.raw().is_sign_positive());
    }

    #[test]
    fn float_totality_defaults_off_with_a_public_field() {
        crate::assert_not_impl!(ExposedUnit: core::cmp::Ord);
    }
}

#[cfg(test)]
mod domain_forms {
    use super::*;

    #[test]
    fn half_open_range_excludes_its_upper_bound() {
        let lower = 0.0;
        let inside = 0.5;
        let upper = 1.0;

        let at_lower = HalfOpen::try_make(lower).map(HalfOpen::raw);
        let at_inside = HalfOpen::try_make(inside).map(HalfOpen::raw);
        let at_upper = HalfOpen::try_make(upper).map(HalfOpen::raw);

        assert_eq!(at_lower, Ok(lower), "{lower}");
        assert_eq!(at_inside, Ok(inside), "{inside}");
        assert_eq!(at_upper, Err(upper), "{upper}");
    }

    #[test]
    fn from_range_admits_infinity() {
        let infinity = f64::INFINITY;
        let below = 0.5;

        let at_infinity = AtLeastOne::try_make(infinity).map(AtLeastOne::raw);
        let at_below = AtLeastOne::try_make(below).map(AtLeastOne::raw);

        assert_eq!(at_infinity, Ok(infinity), "{infinity}");
        assert_eq!(at_below, Err(below), "{below}");
    }

    #[test]
    fn to_range_admits_negative_infinity() {
        let infinity = f64::NEG_INFINITY;
        let bound = 1.5;

        let at_infinity =
            BelowOneAndAHalf::try_make(infinity).map(BelowOneAndAHalf::raw);
        let at_bound =
            BelowOneAndAHalf::try_make(bound).map(BelowOneAndAHalf::raw);

        assert_eq!(at_infinity, Ok(infinity), "{infinity}");
        assert_eq!(at_bound, Err(bound), "{bound}");
    }

    #[test]
    fn to_inclusive_range_admits_its_bound() {
        let bound = 1.5;
        let above = 2.0;

        let at_bound =
            UpToOneAndAHalf::try_make(bound).map(UpToOneAndAHalf::raw);
        let at_above =
            UpToOneAndAHalf::try_make(above).map(UpToOneAndAHalf::raw);

        assert_eq!(at_bound, Ok(bound), "{bound}");
        assert_eq!(at_above, Err(above), "{above}");
    }

    #[test]
    fn union_admits_either_range() {
        let first = 0.5;
        let second = 10.0;
        let between = 5.0;

        let in_first = SplitRange::try_make(first).map(SplitRange::raw);
        let in_second = SplitRange::try_make(second).map(SplitRange::raw);
        let in_between = SplitRange::try_make(between).map(SplitRange::raw);

        assert_eq!(in_first, Ok(first), "{first}");
        assert_eq!(in_second, Ok(second), "{second}");
        assert_eq!(in_between, Err(between), "{between}");
    }

    #[test]
    fn negative_bound_is_typed() {
        let bound = -1.0;
        let below = -1.25;

        let at_bound = Signed::try_make(bound).map(Signed::raw);
        let at_below = Signed::try_make(below).map(Signed::raw);

        assert_eq!(at_bound, Ok(bound), "{bound}");
        assert_eq!(at_below, Err(below), "{below}");
    }

    #[test]
    fn parenthesized_groups_follow_precedence() {
        let low = 0.25;
        let excluded = 0.5;
        let high = 11.0;

        let at_low = Grouped::try_make(low).map(Grouped::raw);
        let at_excluded = Grouped::try_make(excluded).map(Grouped::raw);
        let at_high = Grouped::try_make(high).map(Grouped::raw);

        assert_eq!(at_low, Ok(low), "{low}");
        assert_eq!(at_excluded, Err(excluded), "{excluded}");
        assert_eq!(at_high, Ok(high), "{high}");
    }

    #[test]
    fn type_bound_constant_bounds_the_domain() {
        let bound = f64::MAX;
        let below = f64::MAX.next_down();

        let at_bound = BelowMax::try_make(bound).map(BelowMax::raw);
        let at_below = BelowMax::try_make(below).map(BelowMax::raw);

        assert_eq!(at_bound, Err(bound), "{bound:e}");
        assert_eq!(at_below, Ok(below), "{below:e}");
    }

    #[test]
    fn infinity_bound_admits_infinity() {
        let infinity = f64::INFINITY;
        let below = -1.0;

        let at_infinity =
            UpToInfinity::try_make(infinity).map(UpToInfinity::raw);
        let at_below = UpToInfinity::try_make(below).map(UpToInfinity::raw);

        assert_eq!(at_infinity, Ok(infinity), "{infinity}");
        assert_eq!(at_below, Err(below), "{below}");
    }

    #[test]
    fn std_consts_path_bounds_the_domain() {
        let bound = core::f64::consts::TAU;
        let below = 6.0;

        let at_bound = Angle::try_make(bound).map(Angle::raw);
        let at_below = Angle::try_make(below).map(Angle::raw);

        assert_eq!(at_bound, Err(bound), "{bound}");
        assert_eq!(at_below, Ok(below), "{below}");
    }

    #[test]
    fn is_finite_rejects_infinity_and_nan() {
        let large = 1.0e300;
        let infinity = f64::INFINITY;
        let nan = f64::NAN;

        let at_large = Finite::try_make(large).map(Finite::raw);
        let at_infinity = Finite::try_make(infinity).map(Finite::raw);
        let at_nan = Finite::try_make(nan);

        assert_eq!(at_large, Ok(large), "{large:e}");
        assert_eq!(at_infinity, Err(infinity), "{infinity}");
        assert!(at_nan.is_err(), "{nan}");
    }

    #[test]
    fn f32_constant_is_typed_as_f32() {
        let bound = 0.1_f32;
        let above = 0.1_f32.next_up();

        let at_bound = TenthOrLess::try_make(bound).map(TenthOrLess::raw);
        let at_above = TenthOrLess::try_make(above).map(TenthOrLess::raw);

        assert_eq!(at_bound, Ok(bound), "{bound}");
        assert_eq!(at_above, Err(above), "{above}");
    }

    #[test]
    fn exact_divisor_admits_its_multiples() {
        let multiple = 0.75;
        let other = 0.8;

        let at_multiple = Quarters::try_make(multiple).map(Quarters::raw);
        let at_other = Quarters::try_make(other).map(Quarters::raw);

        assert_eq!(at_multiple, Ok(multiple), "{multiple}");
        assert_eq!(at_other, Err(other), "{other}");
    }

    #[test]
    fn std_provider_function_bounds_the_domain() {
        let inside = 4.0;
        let outside = 4.5;

        let at_inside = RootAtMostTwo::try_make(inside).map(RootAtMostTwo::raw);
        let at_outside =
            RootAtMostTwo::try_make(outside).map(RootAtMostTwo::raw);

        assert_eq!(at_inside, Ok(inside), "{inside}");
        assert_eq!(at_outside, Err(outside), "{outside}");
    }
}

#[cfg(test)]
mod nan_freeness {
    use super::*;

    #[test]
    fn bounded_domain_is_totally_ordered() {
        accepts_eq::<Unit>();
        accepts_ord::<Unit>();
        accepts_hash::<Unit>();
    }

    #[test]
    fn unbounded_union_is_nan_free() {
        let nan = f64::NAN;

        let this = OutsideUnit::try_make(nan);

        assert!(this.is_err(), "{nan}");
        accepts_ord::<OutsideUnit>();
    }

    #[test]
    fn is_finite_domain_is_totally_ordered() {
        accepts_eq::<Finite>();
        accepts_ord::<Finite>();
        accepts_hash::<Finite>();
    }

    #[test]
    fn not_equal_domain_has_no_totality() {
        crate::assert_not_impl!(NotHalf: core::cmp::Eq);
    }

    #[test]
    fn negated_range_admits_nan() {
        let nan = f64::NAN;
        let below = 4.0;

        let at_nan = NotBelowFive::try_make(nan);
        let at_below = NotBelowFive::try_make(below).map(NotBelowFive::raw);

        assert!(at_nan.is_ok(), "{nan}");
        assert_eq!(at_below, Err(below), "{below}");
        crate::assert_not_impl!(NotBelowFive: core::cmp::Ord);
    }

    #[test]
    fn callbacks_alone_admit_nan() {
        let nan = f64::NAN;
        let rejected = 2.0;

        let at_nan = CallbackOnly::try_make(nan);
        let at_rejected =
            CallbackOnly::try_make(rejected).map(CallbackOnly::raw);

        assert!(at_nan.is_ok(), "{nan}");
        assert_eq!(at_rejected, Err(rejected), "{rejected}");
        crate::assert_not_impl!(CallbackOnly: core::cmp::Ord);
    }

    #[test]
    fn totality_can_be_disabled_on_a_nan_free_type() {
        accepts_eq::<Unordered>();
        crate::assert_not_impl!(Unordered: core::cmp::Ord);
    }

    #[test]
    fn unvalidated_type_has_no_totality() {
        crate::assert_not_impl!(Meters: core::cmp::Eq);
        crate::assert_not_impl!(Meters: core::cmp::Ord);
        crate::assert_not_impl!(Meters: core::hash::Hash);
    }
}

#[cfg(test)]
mod flags_profiles_math_friends {
    use super::*;

    #[test]
    fn default_math_has_no_float_math() {
        crate::assert_no_method!(Meters, sqrt);
    }

    #[test]
    fn core_math_has_no_float_math() {
        crate::assert_no_method!(CoreMath, sqrt);
    }

    #[test]
    fn float_math_can_be_disabled() {
        crate::assert_no_method!(StdWithoutFloatMath, sqrt);
    }

    #[test]
    fn without_fn_removes_only_the_named_float_methods() {
        let raw = 2.5;
        let value = WithoutSomeFloatFns::make(raw);

        crate::assert_no_method!(WithoutSomeFloatFns, round_ties_even);
        crate::assert_no_method!(WithoutSomeFloatFns, is_nan);
        crate::assert_no_method!(WithoutSomeFloatFns, sin_cos);
        crate::assert_no_associated_fn!(WithoutSomeFloatFns, from_bits);
        assert_eq!(value.round().raw(), raw.round(), "{raw}");
        assert!(value.is_finite(), "{raw}");
        assert_eq!(value.sin().raw(), raw.sin(), "{raw}");
        assert_eq!(value.to_bits(), raw.to_bits(), "{raw}");
    }

    #[test]
    fn removed_float_method_can_be_supplied_by_the_caller() {
        let zero = ZeroSignum::make(0.0);
        let negative = ZeroSignum::make(-3.0);

        assert_eq!(zero.signum().raw(), 0.0, "{zero:?}");
        assert_eq!(negative.signum().raw(), -1.0, "{negative:?}");
    }

    #[test]
    fn default_has_no_integer_operations() {
        crate::assert_no_method!(Meters, byte0);
        crate::assert_no_method!(Meters, count_ones);
        crate::assert_not_impl!(Meters: core::fmt::Binary);
        crate::assert_not_impl!(Meters: core::fmt::LowerHex);
        crate::assert_not_impl!(Meters: core::iter::Step);
        crate::assert_not_impl!(Meters: core::ops::BitAnd<f64>);
        crate::assert_not_impl!(Meters: core::ops::Shl<u32>);
        crate::assert_not_impl!(Meters: core::ops::BitOrAssign<f64>);
        crate::assert_not_impl!(Meters: core::ops::BitOr<Meters>);
    }

    #[test]
    fn disabling_bit_access_is_accepted() {
        crate::assert_no_method!(WithoutBitAccess, byte0);
    }

    #[test]
    fn math_friend_constructs_and_computes() {
        let boost = Boost(1.5);
        let value = Boosted::make(1.0);

        let constructed = Boosted::of(boost);
        let sum = value + boost;

        assert_eq!(constructed.raw(), 1.5);
        assert_eq!(sum.raw(), 2.5);
        crate::assert_not_impl!(Boosted: core::ops::BitOr<Boost>);
    }

    #[test]
    fn int_profile_keeps_float_operations() {
        let lhs = IntProfile::make(1.5);
        let rhs = IntProfile::make(2.0);

        let sum = lhs + rhs;
        let negated = -lhs;

        assert_eq!(sum.raw(), 3.5);
        assert_eq!(negated.raw(), -1.5);
        assert!(!lhs.is_nan());
    }

    #[test]
    fn point_profile_subtracts_points_only() {
        let start = PointProfile::make(1.5);
        let end = PointProfile::make(4.0);

        let difference = end - start;

        assert_eq!(difference.raw(), 2.5);
        crate::assert_not_impl!(PointProfile: core::ops::Add<PointProfile>);
        crate::assert_not_impl!(PointProfile: core::ops::Neg);
        crate::assert_not_impl!(PointProfile: core::iter::Sum);
        crate::assert_no_method!(PointProfile, is_nan);
    }

    #[test]
    fn point_entry_point_has_no_product() {
        crate::assert_not_impl!(Instant: core::iter::Product);
    }

    #[test]
    fn friends_module_keeps_a_float_relation() {
        let level = 2.5;
        let reading = Reading(level);

        let gauge = Gauge::of(reading);

        assert_eq!(gauge.level(), level, "{level}");
    }
}

#[cfg(test)]
mod bounded_float_to_integer_conversions {
    use super::*;

    #[test]
    fn f32_conversion_rejects_the_u32_bound() {
        let bound = Small::make(4_294_967_296.0);
        let below = Small::make(4_294_967_040.0);

        let at_bound = bound.try_into_u32();
        let at_below = below.try_into_u32();

        assert_eq!(at_bound, Err(()), "{bound:?}");
        assert_eq!(at_below, Ok(4_294_967_040), "{below:?}");
    }

    #[test]
    fn f32_usize_conversion_uses_the_target_width() {
        let bound = Small::make(18_446_744_073_709_551_616.0);
        let below = Small::make(18_446_742_974_197_923_840.0);

        let at_bound = bound.try_into_usize();
        let at_below = below.try_into_usize();

        assert_eq!(at_bound, Err(()), "{bound:?}");
        assert_eq!(at_below.is_ok(), usize::BITS == 64, "{}", usize::BITS);
    }

    #[test]
    fn f64_has_no_infallible_f32_conversion() {
        crate::assert_no_method!(Meters, into_f32);
    }

    #[test]
    fn floats_have_no_infallible_integer_conversion() {
        crate::assert_no_method!(Small, into_i128);
    }
}

#[cfg(test)]
mod integer_to_float_conversions {
    use super::*;

    #[test]
    fn u16_converts_into_f32() {
        let value = Short::make(u16::MAX);

        let converted: f32 = value.into();

        assert_eq!(value.into_f32(), 65_535.0);
        assert_eq!(converted, 65_535.0);
    }

    #[test]
    fn i16_converts_into_f32() {
        let value = SignedShort::make(i16::MIN);

        let converted: f32 = value.into();

        assert_eq!(value.into_f32(), -32_768.0);
        assert_eq!(converted, -32_768.0);
    }

    #[test]
    fn i32_converts_into_f64() {
        let value = SignedCount::make(i32::MIN);

        let converted: f64 = value.into();

        assert_eq!(value.into_f64(), -2_147_483_648.0);
        assert_eq!(converted, -2_147_483_648.0);
    }

    #[test]
    fn u32_has_no_infallible_f32_conversion() {
        crate::assert_no_method!(Steps, into_f32);
        crate::assert_not_impl!(Steps: Into<f32>);
    }

    #[test]
    fn u32_f32_conversion_rejects_the_rounded_maximum() {
        let maximum = Steps::make(u32::MAX);

        let converted = maximum.try_into_f32();

        assert_eq!(converted, Err(()), "{maximum:?}");
    }

    #[test]
    fn u64_has_no_infallible_f64_conversion() {
        crate::assert_no_method!(Large, into_f64);
        crate::assert_not_impl!(Large: Into<f64>);
    }

    #[test]
    fn i64_has_no_infallible_f64_conversion() {
        crate::assert_no_method!(SignedLarge, into_f64);
        crate::assert_not_impl!(SignedLarge: Into<f64>);
    }

    #[test]
    fn usize_has_no_infallible_f64_conversion() {
        crate::assert_no_method!(Pointer, into_f64);
        crate::assert_not_impl!(Pointer: Into<f64>);
    }
}

#[cfg(test)]
mod float_shared_flags_on_integers {
    use super::*;

    #[test]
    fn unsigned_integral_has_no_neg() {
        crate::assert_not_impl!(Steps: core::ops::Neg);
    }

    #[test]
    fn point_signed_integral_has_no_neg() {
        crate::assert_not_impl!(Tick: core::ops::Neg);
    }

    #[test]
    fn integral_exp_formatting_is_opt_in() {
        let value = Scientific::make(1500);

        assert_eq!(format!("{value:e}"), "1.5e3", "{value:?}");
        crate::assert_not_impl!(Scientific: core::fmt::UpperExp);
        crate::assert_not_impl!(Steps: core::fmt::LowerExp);
    }
}
