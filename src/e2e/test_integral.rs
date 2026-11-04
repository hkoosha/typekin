use core::cmp::Ordering;

use serde_test::{
    Configure,
    Token,
    assert_de_tokens_error,
    assert_tokens,
};

use crate::fx_integral::*;
use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwise_ands_friend_value() {
        let value = 0b10111;
        let it = My32::make(value);
        assert_eq!((it & value).raw(), value);
    }

    #[test]
    fn empty_friend_capabilities_do_not_call_conversion() {
        struct Source;

        impl Source {
            #[deny(dead_code)]
            fn convert(self) -> u32 {
                panic!("an empty friend must not be converted");
            }
        }

        #[typekin::integral(hide_in = none, konst = none, friend = Source::convert(Source) -> [])]
        struct Value(u32);

        let raw = 7u32;
        let value = Value::make(raw);
        assert_eq!(value.raw(), raw);
    }

    #[test]
    fn empty_const_generic_friend_uses_its_conversion_in_scope() {
        struct Source<T>(core::marker::PhantomData<T>);

        #[typekin::integral(hide_in = none, konst = nightly, friend = Self::convert(Source<T>) -> [])]
        struct Value<T: Copy>(u32);

        impl<T: Copy> Value<T> {
            #[deny(dead_code)]
            const fn convert(_: Source<T>) -> u32 {
                panic!("an empty friend must not be converted");
            }
        }

        #[derive(Clone, Copy)]
        struct CopyMarker;
        let raw = 7u32;
        let value = Value::<CopyMarker>::make(raw);
        assert_eq!(value.raw(), raw);
    }

    #[test]
    fn empty_facet_friend_capabilities_do_not_call_conversion() {
        struct Source;

        #[deny(dead_code)]
        fn convert(_: Source) -> u32 {
            panic!("an empty facet friend must not be converted");
        }

        #[typekin::integral(
            hide_in = none,
            konst = none,
            with = konst_friendship,
            facet = int { friend = convert(Source) -> [] },
        )]
        struct Value(u32);

        let raw = 7u32;
        let value = Value::make(raw);
        assert_eq!(value.raw(), raw);
    }

    #[test]
    fn empty_const_facet_friend_does_not_call_conversion() {
        struct Source;

        #[deny(dead_code)]
        const fn convert(_: Source) -> u32 {
            panic!("an empty facet friend must not be converted");
        }

        #[typekin::integral(
            hide_in = none,
            konst = none,
            with = konst_add,
            facet = int { friend = convert(Source) -> [] },
        )]
        struct Value(u32);

        let raw = 7u32;
        let value = Value::make(raw);
        assert_eq!(value.raw(), raw);
    }

    #[test]
    fn empty_generic_friend_does_not_shadow_its_callback() {
        struct Source<T>(core::marker::PhantomData<T>);

        #[deny(dead_code)]
        fn __typekin_check_empty_friends<T>(_: Source<T>) -> u32 {
            panic!("an empty friend must not be converted");
        }

        #[typekin::integral(
            hide_in = none,
            konst = none,
            friend = __typekin_check_empty_friends(Source<T>) -> [],
        )]
        struct Value<T>(u32);

        let raw = 7u32;
        let value = Value::<()>::make(raw);
        assert_eq!(value.raw(), raw);
    }

    #[test]
    fn empty_self_friend_capabilities_reference_the_declared_conversion() {
        #[typekin::integral(konst = none, friend = Self::convert(Self) -> [])]
        struct Value(u32);

        impl Value {
            #[deny(dead_code)]
            fn convert(_: Self) -> u32 {
                panic!("an empty friend must not be converted");
            }
        }

        let raw = 7u32;
        let value = Value::make(raw);
        assert_eq!(value.raw(), raw);
    }

    #[test]
    fn empty_implicit_friend_does_not_require_a_conversion() {
        struct Source;

        #[typekin::integral(konst = none, friend = _(Source) -> [])]
        struct Value(u32);

        let raw = 7u32;
        let value = Value::make(raw);
        assert_eq!(value.raw(), raw);
    }

    #[test]
    fn facets_dispatch_by_rhs_with_independent_policies_and_outputs() {
        let storage: core::num::Wrapping<u8> =
            facet_point_storage(FacetPoint::make(1));
        assert_eq!(storage.0, 1);

        let wrapped = FacetPoint::make(u8::MAX) + FacetOffset(1);
        assert_eq!(wrapped.raw(), 0);

        let wrapped = FacetPoint::make(u8::MAX) + FacetWrapping(1);
        assert_eq!(wrapped.raw(), 0);

        let same_type = FacetPoint::make(2) + FacetPoint::make(3);
        assert_eq!(same_type, FacetResult(5));

        let bits = FacetPoint::make(0b1100) ^ FacetBits(0b1010);
        assert_eq!(bits.raw(), 0b0110);

        assert_panics(|| {
            let _ = FacetPoint::make(u8::MAX) + FacetChecked(1);
        });
    }

    #[test]
    fn range_flag_iterates_forward_and_backward() {
        let range = RangeValue::make(2)..RangeValue::make(5);
        assert_eq!(range.size_hint(), (3, Some(3)));
        let forward = range.map(RangeValue::raw).collect::<Vec<_>>();
        assert_eq!(
            forward,
            [
                2,
                3,
                4
            ]
        );

        let backward = (RangeValue::make(2)..=RangeValue::make(4))
            .rev()
            .map(RangeValue::raw)
            .collect::<Vec<_>>();
        assert_eq!(
            backward,
            [
                4,
                3,
                2
            ]
        );
    }

    #[test]
    fn range_flag_supports_wrapping_storage() {
        let values = (WrappingRangeValue::make(253)
            ..WrappingRangeValue::make(u8::MAX))
            .map(WrappingRangeValue::raw)
            .collect::<Vec<_>>();

        assert_eq!(
            values,
            [
                253,
                254
            ]
        );
    }

    #[test]
    fn facets_preserve_constness_and_validate_before_output_conversion() {
        assert_eq!(FACET_CONST_RESULT.raw(), 5);
        let storage: core::num::Wrapping<u8> =
            facet_const_point_storage(FACET_CONST_RESULT);
        assert_eq!(storage.0, 5);

        let lhs = FacetValidated::try_make(2).unwrap();
        let storage: core::num::Wrapping<core::num::NonZeroU8> =
            facet_validated_storage(lhs);
        assert_eq!(storage.0.get(), 2);
        let rhs = FacetValidated::try_make(3).unwrap();
        FACET_OUTPUTS.store(0, std::sync::atomic::Ordering::Relaxed);

        assert_panics(|| {
            let _ = lhs + rhs;
        });
        assert_eq!(FACET_OUTPUTS.load(std::sync::atomic::Ordering::Relaxed), 0,);

        let lhs = FacetValidated::try_make(2).unwrap();
        let rhs = FacetValidated::try_make(2).unwrap();
        let result = lhs + rhs;

        assert_eq!(result, FacetValidatedResult(4));
        assert_eq!(FACET_OUTPUTS.load(std::sync::atomic::Ordering::Relaxed), 1,);
    }

    #[test]
    fn output_type_maps_every_non_assignment_operator_once() {
        OPERATOR_OUTPUT_CALLS.store(0, std::sync::atomic::Ordering::Relaxed);

        assert_eq!(
            OutputSource::make(2) + OutputSource::make(3),
            OperatorOutput(5),
        );
        assert_eq!(OutputSource::make(5) + 2u8, OperatorOutput(7));
        assert_eq!(
            OutputSource::make(9) - OutputSource::make(2),
            OperatorOutput(7),
        );
        assert_eq!(
            OutputSource::make(3) * OutputSource::make(4),
            OperatorOutput(12),
        );
        assert_eq!(
            OutputSource::make(9) / OutputSource::make(2),
            OperatorOutput(4),
        );
        assert_eq!(
            OutputSource::make(9) % OutputSource::make(4),
            OperatorOutput(1),
        );
        assert_eq!(
            OutputSource::make(0b1100) & OutputSource::make(0b1010),
            OperatorOutput(0b1000),
        );
        assert_eq!(
            OutputSource::make(0b1100) | OutputSource::make(0b1010),
            OperatorOutput(0b1110),
        );
        assert_eq!(
            OutputSource::make(0b1100) ^ OutputSource::make(0b1010),
            OperatorOutput(0b0110),
        );
        assert_eq!(!OutputSource::make(0b0011), OperatorOutput(0b1111_1100));
        assert_eq!(OutputSource::make(3) << 2, OperatorOutput(12));
        assert_eq!(OutputSource::make(12) >> 2, OperatorOutput(3));
        assert_eq!(
            OPERATOR_OUTPUT_CALLS.load(std::sync::atomic::Ordering::Relaxed),
            12,
        );

        let core_integer_result =
            OutputSource::make(5).wrapping_add(OutputSource::make(1));
        assert_eq!(core_integer_result.raw(), 6);
    }

    #[test]
    fn output_type_uses_const_callback_in_const_operator() {
        assert_eq!(CONST_OPERATOR_OUTPUT, OperatorOutput(5));
    }

    #[test]
    fn output_type_callback_runs_after_source_validation() {
        CHECKED_OUTPUT_CALLS.store(0, std::sync::atomic::Ordering::Relaxed);

        assert_panics(|| {
            let _ = CheckedOutputSource::try_make(8).unwrap()
                + CheckedOutputSource::try_make(4).unwrap();
        });
        assert_eq!(
            CHECKED_OUTPUT_CALLS.load(std::sync::atomic::Ordering::Relaxed),
            0,
        );
    }

    #[test]
    fn non_zero_reconstruction_precedes_output_conversion() {
        NON_ZERO_OUTPUT_CALLS.store(0, std::sync::atomic::Ordering::Relaxed);

        let lhs = NonZeroOutputSource::try_make(2).unwrap();
        let rhs = NonZeroOutputSource::try_make(2).unwrap();
        assert_panics(|| {
            let _ = lhs - rhs;
        });
        assert_eq!(
            NON_ZERO_OUTPUT_CALLS.load(std::sync::atomic::Ordering::Relaxed),
            0,
        );

        let lhs = NonZeroOutputSource::try_make(2).unwrap();
        let rhs = NonZeroOutputSource::try_make(3).unwrap();
        let result = lhs + rhs;
        assert_eq!(result, OperatorOutput(5));
        assert_eq!(
            NON_ZERO_OUTPUT_CALLS.load(std::sync::atomic::Ordering::Relaxed),
            1,
        );
    }

    #[test]
    fn output_type_preserves_target_validation_failure() {
        assert_panics(|| {
            let _ = TargetCheckedSource::make(4) + TargetCheckedSource::make(3);
        });
    }

    #[test]
    fn self_output_type_keeps_identity_operations_and_assignments() {
        let sum: ExplicitSelfOutput =
            ExplicitSelfOutput::make(2) + ExplicitSelfOutput::make(3);
        assert_eq!(sum.raw(), 5);

        let mut assigned = ExplicitSelfOutput::make(2);
        assigned += ExplicitSelfOutput::make(3);
        assert_eq!(assigned.raw(), 5);
    }

    #[test]
    fn linear_point_differences_use_the_explicit_output_type() {
        let source = LinearPoint::make(12);
        let origin = LinearPoint::make(5);
        let forward: i16 = source - origin;
        let backward: i16 = origin - source;

        assert_eq!(forward, 7);
        assert_eq!(backward, -7);
        assert!(origin < source);
    }

    #[test]
    fn checked_point_subtraction_rejects_underflow() {
        let source = CheckedPoint::make(0);
        let origin = CheckedPoint::make(1);

        assert_panics(|| {
            let _ = source - origin;
        });
    }

    #[test]
    fn wrapping_point_differences_keep_wrapping_storage() {
        let source = WrappingPoint::make(0);
        let origin = WrappingPoint::make(1);
        let _storage: core::num::Wrapping<u8> = wrapping_point_storage(source);
        let difference: u8 = source - origin;

        assert_eq!(difference, u8::MAX);
    }

    #[test]
    fn point_facet_preserves_root_wrapping_policy() {
        let maximum = SurfaceFacetPoint::make(u8::MAX);
        let offset = FacetOffset(1);
        let wrapped = maximum + offset;
        let restored = wrapped - offset;

        assert_eq!(wrapped.raw(), 0);
        assert_eq!(restored.raw(), maximum.raw());
    }

    #[test]
    fn circular_profile_uses_wrapping_arithmetic_and_half_range_precedence() {
        assert_eq!(
            (CircularSequence::make(u8::MAX) + CircularSequence::make(1)).raw(),
            0,
        );
        assert_eq!(
            (CircularSequence::make(0) - CircularSequence::make(1)).raw(),
            u8::MAX,
        );

        let mut assigned = CircularSequence::make(u8::MAX);
        assigned += CircularSequence::make(1);
        assert_eq!(assigned.raw(), 0);

        assert!(CircularSequence::make(0).precedes(CircularSequence::make(1)));
        assert!(
            CircularSequence::make(u8::MAX).precedes(CircularSequence::make(0))
        );
        assert!(!CircularSequence::make(0).precedes(CircularSequence::make(0)));
        assert!(
            !CircularSequence::make(0).precedes(CircularSequence::make(128))
        );
        assert!(
            !CircularSequence::make(128).precedes(CircularSequence::make(0))
        );

        assert!(CircularSigned::make(-1).precedes(CircularSigned::make(0)));
        assert!(!CircularSigned::make(0).precedes(CircularSigned::make(-128)));
        assert!(!CircularSigned::make(-128).precedes(CircularSigned::make(0)));
    }

    #[test]
    fn delta_profile_uses_circular_arithmetic_without_serial_precedence() {
        let maximum = DeltaSequence::make(u8::MAX);
        let one = DeltaSequence::make(1);
        let zero = maximum + one;
        let restored = zero - one;

        assert_eq!(zero.raw(), 0);
        assert_eq!(restored.raw(), u8::MAX);
    }

    #[test]
    fn circular_shorthand_can_disable_precedes() {
        let maximum = CircularWithoutPrecedes::make(u8::MAX);
        let one = CircularWithoutPrecedes::make(1);
        let zero = maximum + one;

        assert_eq!(zero.raw(), 0);
    }

    #[test]
    fn circular_profile_applies_wrapping_policy_to_reenabled_operations() {
        assert_eq!(
            (CircularExtended::make(200) * CircularExtended::make(2)).raw(),
            144,
        );
        assert_eq!(
            (CircularExtended::make(10) / CircularExtended::make(3)).raw(),
            3,
        );
        assert_eq!(
            (CircularExtended::make(10) % CircularExtended::make(3)).raw(),
            1,
        );
        assert_eq!((CircularExtended::make(128) << 1).raw(), 0);
        assert_eq!((CircularExtended::make(16) >> 2).raw(), 4);
    }

    #[test]
    fn wrapping_profile_uses_physical_wrapping_storage_and_linear_ordering() {
        let maximum = WrappingSequence::make(u8::MAX);
        let storage: core::num::Wrapping<u8> =
            wrapping_sequence_storage(maximum);
        assert_eq!(storage.0, u8::MAX);

        let wrapped = maximum + WrappingSequence::make(1);
        assert_eq!(wrapped.raw(), 0);
        assert!(WrappingSequence::make(1) < WrappingSequence::make(2));

        let mut assigned = WrappingSequence::make(u8::MAX);
        assigned += WrappingSequence::make(1);
        assert_eq!(assigned.raw(), 0);

        assert_eq!(
            (WrappingSequence::make(200) * WrappingSequence::make(2)).raw(),
            144,
        );
        assert_eq!((WrappingSequence::make(128) << 1).raw(), 0);
    }

    #[test]
    fn wrapping_profile_supports_combined_non_zero_storage() {
        let value = WrappingNonZero::try_make(2).unwrap();
        let storage: core::num::Wrapping<core::num::NonZeroU8> =
            wrapping_non_zero_storage(value);
        assert_eq!(storage.0.get(), 2);
        assert_eq!(
            core::mem::size_of::<Option<WrappingNonZero>>(),
            core::mem::size_of::<u8>(),
        );

        let lhs = WrappingNonZero::try_make(2).unwrap();
        let rhs = WrappingNonZero::try_make(3).unwrap();
        assert_eq!((lhs + rhs).raw(), 5);

        let lhs = WrappingNonZero::try_make(2).unwrap();
        let rhs = WrappingNonZero::try_make(2).unwrap();
        assert_panics(|| {
            let _ = lhs - rhs;
        });
    }

    #[test]
    fn wrapping_profile_retains_const_arithmetic() {
        assert_eq!(CONST_WRAPPING_RESULT.raw(), 0);
    }

    #[test]
    fn sequential_profile_panics_on_overflow() {
        assert_panics(|| {
            let _ =
                SequentialCounter::make(u8::MAX) + SequentialCounter::make(1);
        });
        assert_panics(|| {
            let _ = SequentialCounter::make(0) - SequentialCounter::make(1);
        });

        let mut assigned = SequentialCounter::make(1);
        assert_panics(move || assigned -= SequentialCounter::make(2));
        assert_eq!(assigned.raw(), 1);
    }

    #[test]
    fn sequential_profile_applies_checked_policy_to_reenabled_operations() {
        assert_eq!(
            (SequentialExtended::make(10) / SequentialExtended::make(3)).raw(),
            3,
        );
        assert_eq!(
            (SequentialExtended::make(10) % SequentialExtended::make(3)).raw(),
            1,
        );
        assert_eq!((SequentialExtended::make(16) >> 2).raw(), 4);

        assert_panics(|| {
            let _ = SequentialExtended::make(200) * SequentialExtended::make(2);
        });
        assert_panics(|| {
            let _ = SequentialExtended::make(128) << 8;
        });
    }

    #[test]
    fn profile_list_uses_the_last_arithmetic_policy() {
        let lhs = LayeredProfiles::make(200);
        let rhs = LayeredProfiles::make(2);
        let product = lhs * rhs;

        assert_eq!(product.raw(), 144);
    }

    #[test]
    fn blind_trust_const_construction_bypasses_callback_and_range_validation() {
        assert_eq!(BLIND_TRUST_ODD_NUMBER.raw(), 3);
        assert_eq!(BlindTrustEven::of(12u8).raw(), 12);
        assert_eq!(BlindTrustEven::try_make(3), Err(3));
        assert_eq!(BlindTrustEven::try_make(12), Err(12));
    }

    #[test]
    fn blind_trust_construction_cannot_bypass_non_zero_storage() {
        assert_panics(|| BlindTrustEven::of(0u8));
    }

    #[test]
    fn blind_trust_construction_preserves_combined_storage_validity() {
        assert_panics(|| WrappingBlindTrust::of(0u8));

        let value = WrappingBlindTrust::of(7u8);
        let storage: core::num::Wrapping<core::num::NonZeroU8> =
            wrapping_blind_trust_storage(value);
        assert_eq!(storage.0.get(), 7);
    }

    #[test]
    fn blind_trust_const_friend_operations_still_validate() {
        assert_panics(|| {
            let _ = BlindTrustEven::try_make(4).unwrap() + 1u8;
        });
        assert_panics(|| {
            let _ = BlindTrustEven::try_make(4).unwrap() | 1u8;
        });
    }

    #[test]
    fn with_blind_trust_constructs_without_validation() {
        let raw = 200;
        let value = blind_trust_on_request(raw);

        assert_eq!(value.raw(), raw, "{raw}");
        assert_eq!(BlindTrustOnRequest::try_make(raw), Err(raw), "{raw}");
    }

    #[test]
    fn with_blind_trust_cannot_store_zero_in_non_zero_storage() {
        let raw = 0;

        assert_panics(move || blind_trust_on_request(raw));
    }

    #[test]
    fn non_const_validator_works_without_konst() {
        let odd = OddAtRuntime::try_make(3);
        let even = OddAtRuntime::try_make(4);
        assert!(odd.is_ok(), "3");

        assert_eq!(even, Err(4));
        assert_panics(move || odd.unwrap() + 1u8);
        assert_eq!((odd.unwrap() + 2u8).raw(), 5);
    }

    #[test]
    fn bit_accessors_split_u16() {
        let it = CustomRaw::make(0xa1b2);
        assert_eq!(it.lo8(), 0xb2);
        assert_eq!(it.hi8(), 0xa1);
    }

    #[test]
    fn integral_layout_and_copy_traits_can_be_supplied_manually() {
        accepts_copy::<ManuallyDerivedIntegral>();
        assert_eq!(ManuallyDerivedIntegral::make(7).raw(), 7);
    }

    #[test]
    fn bit_accessors_split_u32() {
        let it = My32::make(0xa1_b2_c3_d4);
        assert_eq!(it.lo16(), 0xc3d4);
        assert_eq!(it.hi16(), 0xa1b2);
        assert_eq!(it.byte0(), 0xd4);
        assert_eq!(it.byte1(), 0xc3);
        assert_eq!(it.byte2(), 0xb2);
        assert_eq!(it.byte3(), 0xa1);
    }

    #[test]
    fn bit_accessors_split_u64() {
        let it = MyAuto64::make(0x01_02_03_04_05_06_07_08);
        assert_eq!(it.lo32(), 0x05_06_07_08);
        assert_eq!(it.hi32(), 0x01_02_03_04);
        assert_eq!(it.word0(), 0x0708);
        assert_eq!(it.word3(), 0x0102);
        assert_eq!(it.byte0(), 0x08);
        assert_eq!(it.byte7(), 0x01);
    }

    #[test]
    fn bit_accessors_split_u128() {
        let it = Wide::make(0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f_10);
        assert_eq!(it.lo64(), 0x09_0a_0b_0c_0d_0e_0f_10);
        assert_eq!(it.hi64(), 0x01_02_03_04_05_06_07_08);
        assert_eq!(it.dword0(), 0x0d_0e_0f_10);
        assert_eq!(it.dword3(), 0x01_02_03_04);
        assert_eq!(it.word0(), 0x0f10);
        assert_eq!(it.word7(), 0x0102);
        assert_eq!(it.byte0(), 0x10);
        assert_eq!(it.byte15(), 0x01);
    }

    #[test]
    fn full_integral_generates_layout_and_common_traits() {
        accepts_hash::<FullyGenerated>();
        accepts_copy::<FullyGenerated>();
        assert_eq!(size_of::<FullyGenerated>(), 1);
        assert_eq!(align_of::<FullyGenerated>(), 1);
        assert_eq!(FULLY_GENERATED_CLONE.raw(), 23);
    }

    #[test]
    fn selectively_enabled_add_is_const() {
        assert_eq!(CONST_ADD_ONLY.raw(), 23);
    }

    #[test]
    fn add_assignment_const_override_takes_precedence() {
        let result = CONST_ADD_ASSIGN_ONLY;
        assert_eq!(result.raw(), 23);
    }

    #[test]
    fn or_assignment_const_override_takes_precedence() {
        let result = CONST_OR_ASSIGN_ONLY;
        assert_eq!(result.raw(), 0b1011);
    }

    #[test]
    fn makes_from_automatic_raw_friend() {
        let it = MyAuto64::make(23);
        assert_eq!(it.raw(), 23);
    }

    #[test]
    fn makes_from_safely_widening_raw_friends() {
        assert_eq!(My32::of(23u8).raw(), 23);
        assert_eq!(My32::of(23u16).raw(), 23);
    }

    #[test]
    fn makes_from_owned_friend() {
        let value = 23u32;
        let it = My32::of(value);

        assert_eq!(it.raw(), value);
    }

    #[test]
    fn exposes_underlying_reference_with_as_ref() {
        let it = My32::make(23);
        let raw: &u32 = it.as_ref();

        assert_eq!(*raw, 23);
    }

    #[test]
    fn exposes_configured_raw_value() {
        assert_eq!(CustomRaw::make(23).value(), 23);
    }

    #[test]
    fn try_make_accepts_unvalidated_value() {
        let it = My32::make(23);
        assert_eq!(My32::try_make(23), Ok(it));
    }

    #[test]
    fn map_validates_only_the_final_value_and_calls_once() {
        let calls = core::cell::Cell::new(0);
        let mapped = Mapped::try_make(0)
            .unwrap()
            .try_map(|value| {
                calls.set(calls.get() + 1);
                return value + 1 + 2;
            })
            .unwrap();

        assert_eq!(mapped.raw(), 3);
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn map_returns_rejected_final_value_and_panics_in_infallible_form() {
        assert_eq!(Mapped::try_make(3).unwrap().try_map(|_| 1), Err(1));
        assert_panics(|| {
            let _ = Mapped::try_make(3).unwrap().map(|_| 1);
        });
    }

    #[test]
    fn map_uses_the_configured_raw_accessor() {
        let mapped = AccessedMap::make(123).map(|value| {
            assert_eq!(value, 23);
            return 42;
        });

        assert_eq!(accessed_map_storage(mapped), 42);
    }

    #[test]
    fn map_preserves_closure_panic_and_primitive_overflow_behavior() {
        assert_panics(|| {
            let _ = PlainMapped::make(1).map(|_| panic!("closure panic"));
        });
        assert_panics(|| {
            let _ = PlainMapped::make(u8::MAX).map(|value| value + 1);
        });
    }

    #[test]
    fn map_works_for_unvalidated_and_const_integrals() {
        assert_eq!(PlainMapped::make(2).map(|value| value + 4).raw(), 6);
        assert_eq!(CONST_MAPPED.raw(), 23);
    }

    #[test]
    fn integral_serde_serializes_raw_values_and_revalidates() {
        assert_tokens(
            &SerdeIntegral::try_make(4).unwrap().readable(),
            &[Token::U8(4)],
        );
        assert_tokens(
            &SerdeIntegral::try_make(4).unwrap().compact(),
            &[Token::U8(4)],
        );
        assert_de_tokens_error::<SerdeIntegral>(
            &[Token::U8(3)],
            "invalid value",
        );
    }

    #[test]
    fn parses_integrals_from_plain_numbers() {
        assert_eq!("23".parse::<My32>().map(My32::raw), Ok(23));
        assert_eq!("-23".parse::<MyI32>().map(MyI32::raw), Ok(-23));
        assert_eq!("24".parse::<Even>().map(Even::raw), Ok(24));
        assert_eq!("-6".parse::<SplitEven>().map(SplitEven::raw), Ok(-6));
    }

    #[test]
    fn rejects_empty_my32_input_with_negative_odd_split_value() {
        let source = "";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "23";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "-5";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_empty_my32_input_with_zero_split_value() {
        let source = "";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "23";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "0";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_empty_my32_input_with_large_even_value() {
        let source = "";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "50";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "-5";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_empty_my32_input_with_large_even_and_zero_split_value() {
        let source = "";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "50";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "0";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_empty_my32_input_with_out_of_range_even_value() {
        let source = "";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "102";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "-5";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_empty_my32_input_with_out_of_range_even_and_zero_split_value() {
        let source = "";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "102";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "0";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_negative_my32_input_with_negative_odd_split_value() {
        let source = "-1";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "23";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "-5";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_negative_my32_input_with_zero_split_value() {
        let source = "-1";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "23";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "0";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_negative_my32_input_with_large_even_value() {
        let source = "-1";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "50";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "-5";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_negative_my32_input_with_large_even_and_zero_split_value() {
        let source = "-1";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "50";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "0";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_negative_my32_input_with_out_of_range_even_value() {
        let source = "-1";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "102";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "-5";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_negative_my32_input_with_out_of_range_even_and_zero_split_value()
    {
        let source = "-1";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "102";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "0";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_overflowing_my32_input_with_negative_odd_split_value() {
        let source = "4_294_967_296";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "23";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "-5";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_overflowing_my32_input_with_zero_split_value() {
        let source = "4_294_967_296";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "23";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "0";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_overflowing_my32_input_with_large_even_value() {
        let source = "4_294_967_296";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "50";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "-5";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_overflowing_my32_input_with_large_even_and_zero_split_value() {
        let source = "4_294_967_296";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "50";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "0";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_overflowing_my32_input_with_out_of_range_even_value() {
        let source = "4_294_967_296";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "102";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "-5";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_overflowing_my32_input_with_out_of_range_even_and_zero_split_value()
     {
        let source = "4_294_967_296";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "102";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "0";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_non_numeric_my32_input_with_negative_odd_split_value() {
        let source = "not a number";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "23";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "-5";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_non_numeric_my32_input_with_zero_split_value() {
        let source = "not a number";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "23";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "0";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_non_numeric_my32_input_with_large_even_value() {
        let source = "not a number";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "50";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "-5";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_non_numeric_my32_input_with_large_even_and_zero_split_value() {
        let source = "not a number";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "50";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "0";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_non_numeric_my32_input_with_out_of_range_even_value() {
        let source = "not a number";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "102";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "-5";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn rejects_non_numeric_my32_input_with_out_of_range_even_and_zero_split_value()
     {
        let source = "not a number";
        assert_eq!(source.parse::<My32>(), Err(()), "{source:?}");

        let source = "102";
        assert_eq!(source.parse::<Even>(), Err(()), "{source:?}");

        let source = "0";
        assert_eq!(source.parse::<SplitEven>(), Err(()), "{source:?}");
    }

    #[test]
    fn parsing_remains_checked_when_try_make_is_disabled() {
        assert_eq!("24".parse::<ParsedEven>().map(ParsedEven::raw), Ok(24));
        assert_eq!("23".parse::<ParsedEven>(), Err(()));
    }

    #[test]
    fn forwards_common_stable_integral_methods() {
        let number = My32::make(0x12_34_56_78);
        assert_eq!(My32::BITS, u32::BITS);
        assert_eq!(number.count_ones(), 13);
        assert_eq!(number.count_zeros(), 19);
        assert_eq!(number.leading_zeros(), 3);
        assert_eq!(number.trailing_zeros(), 3);
        assert_eq!(number.leading_ones(), 0);
        assert_eq!(number.trailing_ones(), 0);
        assert_eq!(number.highest_one(), Some(28));
        assert_eq!(number.lowest_one(), Some(3));
        assert_eq!(number.ilog2(), 28);
        assert_eq!(number.ilog10(), 8);
        assert_eq!(number.checked_ilog2(), Some(28));
        assert_eq!(number.to_be_bytes(), 0x12_34_56_78u32.to_be_bytes());
        assert_eq!(number.rotate_left(4).raw(), 0x23_45_67_81);
        assert_eq!(number.rotate_right(4).raw(), 0x81_23_45_67);
        assert_eq!(number.swap_bytes().raw(), 0x78_56_34_12);
        assert_eq!(number.reverse_bits().raw(), 0x1e_6a_2c_48);
        assert_eq!(My32::from_be_bytes(number.to_be_bytes()), number);
        assert_eq!(
            number.checked_add(My32::make(1)).map(My32::raw),
            Some(0x12_34_56_79)
        );
        assert_eq!(number.checked_shl(32), None);
        assert_eq!(number.unbounded_shl(32).raw(), 0);
        assert_eq!(number.wrapping_shr(32), number);
        assert_eq!(number.midpoint(My32::make(0)).raw(), 0x09_1a_2b_3c);
        assert_eq!(My32::make(12).checked_pow(3).map(My32::raw), Some(1728));
        assert_eq!(My32::make(12).saturating_mul(My32::make(3)).raw(), 36);
        assert_eq!(My32::make(12).overflowing_mul(My32::make(3)).0.raw(), 36);
        assert_eq!(MyI32::make(-7).div_euclid(MyI32::make(3)).raw(), -3);
        assert_eq!(MyI32::make(-7).rem_euclid(MyI32::make(3)).raw(), 2);
        assert_eq!(My32::make(144).isqrt().raw(), 12);
    }

    #[test]
    fn checked_add_returns_none_for_an_invalid_result() {
        let upper_bound = Even::try_make(100).unwrap();
        let two = Even::try_make(2).unwrap();
        assert_eq!(upper_bound.checked_add(two), None);
    }

    #[test]
    fn saturating_add_panics_for_an_invalid_result() {
        let upper_bound = Even::try_make(100).unwrap();
        let two = Even::try_make(2).unwrap();
        assert_panics(|| upper_bound.saturating_add(two));
    }

    #[test]
    fn from_ne_bytes_panics_for_an_invalid_result() {
        assert_panics(|| Even::from_ne_bytes([1]));
    }

    #[test]
    fn midpoint_panics_for_an_invalid_result() {
        let two = Even::try_make(2).unwrap();
        let hundred = Even::try_make(100).unwrap();
        assert_panics(|| two.midpoint(hundred));
    }

    #[test]
    fn overflowing_add_panics_for_an_invalid_result() {
        let two = Even::try_make(2).unwrap();
        let hundred = Even::try_make(100).unwrap();
        assert_panics(|| hundred.overflowing_add(two));
    }

    #[test]
    fn carrying_add_panics_for_an_invalid_result() {
        let two = Even::try_make(2).unwrap();
        let hundred = Even::try_make(100).unwrap();
        assert_panics(|| hundred.carrying_add(two, false));
    }

    #[test]
    fn signed_signum_panics_for_an_invalid_result() {
        let negative = SplitEven::try_make(-6).unwrap();
        assert_panics(|| negative.signum());
    }

    #[test]
    fn forwards_signed_stable_integral_methods() {
        let negative = MyI32::make(-12);
        assert_eq!(negative.abs().raw(), 12);
        assert_eq!(negative.checked_abs().map(MyI32::raw), Some(12));
        assert_eq!(negative.strict_abs().raw(), 12);
        assert_eq!(negative.saturating_neg().raw(), 12);
        assert_eq!(negative.saturating_abs().raw(), 12);
        assert_eq!(negative.wrapping_abs().raw(), 12);
        assert_eq!(negative.overflowing_abs().0.raw(), 12);
        assert_eq!(negative.unsigned_abs(), 12u32);
        assert_eq!(negative.abs_diff(MyI32::make(3)), 15u32);
        assert_eq!(negative.cast_unsigned(), (-12i32).cast_unsigned());
        assert_eq!(negative.signum().raw(), -1);
        assert!(negative.is_negative());
        assert!(!negative.is_positive());
        assert_eq!(MyI32::make(144).checked_isqrt().map(MyI32::raw), Some(12));
        assert_eq!(negative.checked_isqrt(), None);
    }

    #[test]
    fn forwards_unsigned_stable_integral_methods() {
        assert_eq!(My32::make(15).bit_width(), 4);
        assert_eq!(My32::make(12).abs_diff(My32::make(3)), 9u32);
        assert_eq!(My32::make(u32::MAX).cast_signed(), -1i32);
        assert_eq!(
            My32::make(0x12_34_56_78)
                .funnel_shl(My32::make(0x9a_bc_de_f0), 4)
                .raw(),
            0x23_45_67_89
        );
        assert_eq!(
            My32::make(0x12_34_56_78)
                .funnel_shr(My32::make(0x9a_bc_de_f0), 4)
                .raw(),
            0x89_ab_cd_ef
        );
        assert!(My32::make(12).is_multiple_of(My32::make(3)));
        assert!(!My32::make(12).is_multiple_of(My32::make(5)));
        assert!(My32::make(16).is_power_of_two());
        assert_eq!(My32::make(15).next_power_of_two().raw(), 16);
        assert_eq!(
            My32::make(15).checked_next_power_of_two().map(My32::raw),
            Some(16)
        );
        assert_eq!(My32::make(10).div_ceil(My32::make(3)).raw(), 4);
        assert_eq!(My32::make(10).next_multiple_of(My32::make(6)).raw(), 12);
        assert_eq!(
            My32::make(10)
                .checked_next_multiple_of(My32::make(6))
                .map(My32::raw),
            Some(12)
        );
        assert_eq!(My32::make(1).carrying_add(My32::make(2), false).0.raw(), 3);
        assert_eq!(
            My32::make(5).borrowing_sub(My32::make(2), false).0.raw(),
            3
        );
        assert_eq!(
            My32::make(3)
                .carrying_mul(My32::make(4), My32::make(5))
                .0
                .raw(),
            17
        );
        assert_eq!(
            My32::make(3)
                .carrying_mul_add(My32::make(4), My32::make(5), My32::make(7))
                .0
                .raw(),
            24
        );
    }

    #[test]
    fn try_make_is_const() {
        assert_eq!(CONST_NUMBER.raw(), 23);
    }

    #[test]
    fn try_make_accepts_validated_value() {
        assert_eq!(Even::try_make(24).map(Even::raw), Ok(24));
    }

    #[test]
    fn try_make_rejects_invalidated_value() {
        assert_eq!(Even::try_make(23), Err(23));
    }

    #[test]
    fn try_make_rejects_second_callback() {
        assert_eq!(Even::try_make(50), Err(50));
    }

    #[test]
    fn try_make_rejects_even_value_outside_configured_ranges() {
        assert_eq!(Even::try_make(102), Err(102));
    }

    #[test]
    fn range_validation_accepts_each_configured_interval() {
        assert_eq!(Ranged::try_make(2).map(Ranged::raw), Ok(2));
        assert_eq!(Ranged::try_make(4).map(Ranged::raw), Ok(4));
        assert_eq!(Ranged::try_make(8).map(Ranged::raw), Ok(8));
        assert_eq!(Ranged::try_make(9).map(Ranged::raw), Ok(9));
        assert_eq!(Ranged::try_make(1), Err(1));
        assert_eq!(Ranged::try_make(5), Err(5));
        assert_eq!(Ranged::try_make(10), Err(10));
    }

    #[test]
    fn non_zero_policy_adds_and_preserves_semantic_validation() {
        assert_eq!(RequiredNonZero::try_make(0), Err(0));
        assert_eq!(
            RequiredNonZero::try_make(1).map(RequiredNonZero::raw),
            Ok(1),
        );

        assert_eq!(DomainRequiredNonZero::try_make(0), Err(0));
        assert_eq!(
            DomainRequiredNonZero::try_make(10).map(DomainRequiredNonZero::raw),
            Ok(10),
        );
        assert_eq!(DomainRequiredNonZero::try_make(11), Err(11));

        assert_eq!(PrimitivePositive::try_make(0), Err(0));
        assert_eq!(
            PrimitivePositive::try_make(1).map(PrimitivePositive::raw),
            Ok(1),
        );
    }

    #[test]
    fn non_zero_storage_preserves_every_primitive_layout_and_niche() {
        assert_eq!(
            core::mem::size_of::<NonZeroUsize>(),
            core::mem::size_of::<usize>(),
        );
        assert_eq!(
            core::mem::size_of::<Option<NonZeroUsize>>(),
            core::mem::size_of::<usize>(),
        );
        assert_eq!(
            core::mem::size_of::<Option<NonZeroIsize>>(),
            core::mem::size_of::<isize>(),
        );
        assert_eq!(
            core::mem::size_of::<Option<NonZeroU8>>(),
            core::mem::size_of::<u8>(),
        );
        assert_eq!(
            core::mem::size_of::<Option<NonZeroU16>>(),
            core::mem::size_of::<u16>(),
        );
        assert_eq!(
            core::mem::size_of::<Option<NonZeroU32>>(),
            core::mem::size_of::<u32>(),
        );
        assert_eq!(
            core::mem::size_of::<Option<NonZeroU64>>(),
            core::mem::size_of::<u64>(),
        );
        assert_eq!(
            core::mem::size_of::<Option<NonZeroU128>>(),
            core::mem::size_of::<u128>(),
        );
        assert_eq!(
            core::mem::size_of::<Option<NonZeroI8>>(),
            core::mem::size_of::<i8>(),
        );
        assert_eq!(
            core::mem::size_of::<Option<NonZeroI16>>(),
            core::mem::size_of::<i16>(),
        );
        assert_eq!(
            core::mem::size_of::<Option<NonZeroI32>>(),
            core::mem::size_of::<i32>(),
        );
        assert_eq!(
            core::mem::size_of::<Option<NonZeroI64>>(),
            core::mem::size_of::<i64>(),
        );
        assert_eq!(
            core::mem::size_of::<Option<NonZeroI128>>(),
            core::mem::size_of::<i128>(),
        );
    }

    #[test]
    fn non_zero_storage_keeps_primitive_raw_apis() {
        let unsigned = NonZeroU32::try_make(7).unwrap();
        let signed = NonZeroI32::try_make(9).unwrap();

        assert_eq!(unsigned.raw(), 7u32);
        assert_eq!(signed.raw(), 9i32);
        assert_eq!(NonZeroU32::try_make(0), Err(0));
        assert_eq!(NonZeroI32::try_make(0), Err(0));
    }

    #[test]
    fn non_zero_storage_preserves_maps_and_parsing() {
        let value = NonZeroU32::try_make(2).unwrap();
        let mapped = value.try_map(|raw| raw + 3);
        assert_eq!(mapped.map(NonZeroU32::raw), Ok(5));

        let value = NonZeroU32::try_make(2).unwrap();
        assert_eq!(value.try_map(|_| 0), Err(0));
        assert_eq!("7".parse::<NonZeroU32>().map(NonZeroU32::raw), Ok(7));
        assert_eq!("0".parse::<NonZeroU32>(), Err(()));
    }

    #[test]
    fn non_zero_storage_preserves_raw_views_and_formatting() {
        use core::hash::{
            Hash,
            Hasher,
        };

        let value = NonZeroU32::try_make(7).unwrap();
        assert_eq!(value.into_u64(), 7u64);
        assert_eq!(format!("{value:?}"), "NonZeroU32(7)");
        assert_eq!(format!("{value:b}"), "111");

        let mut wrapped_hasher =
            std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut wrapped_hasher);
        let mut raw_hasher = std::collections::hash_map::DefaultHasher::new();
        7u32.hash(&mut raw_hasher);
        assert_eq!(wrapped_hasher.finish(), raw_hasher.finish());
    }

    #[test]
    fn non_zero_storage_has_no_as_ref() {
        assert_not_impl!(NonZeroU32: AsRef<u32>);
    }

    #[test]
    fn rewritten_storage_preserves_a_custom_primitive_raw_accessor() {
        let value = CustomStoredRaw::try_make(7).unwrap();
        assert_eq!(value.value(), 7);

        let mapped = value.map(|raw| raw + 2);
        assert_eq!(mapped.value(), 9);

        let sum = mapped + CustomStoredRaw::try_make(3).unwrap();
        assert_eq!(sum.value(), 12);
    }

    #[test]
    fn non_zero_storage_preserves_serde_wire_values() {
        assert_tokens(
            &NonZeroSerde::try_make(7).unwrap().readable(),
            &[Token::U8(7)],
        );
        assert_tokens(
            &NonZeroSerde::try_make(7).unwrap().compact(),
            &[Token::U8(7)],
        );
        assert_de_tokens_error::<NonZeroSerde>(
            &[Token::U8(0)],
            "invalid value",
        );
        assert_eq!(format!("{}", NonZeroSerde::try_make(7).unwrap()), "7");
    }

    #[test]
    fn non_zero_storage_reconstructs_operations_and_integer_methods() {
        let lhs = NonZeroU32::try_make(2).unwrap();
        let rhs = NonZeroU32::try_make(3).unwrap();
        assert_eq!((lhs + rhs).raw(), 5);

        let mut assigned = NonZeroU32::try_make(2).unwrap();
        assigned += NonZeroU32::try_make(3).unwrap();
        assert_eq!(assigned.raw(), 5);

        let lhs = NonZeroU32::try_make(2).unwrap();
        let rhs = NonZeroU32::try_make(2).unwrap();
        assert_panics(|| {
            let _ = lhs - rhs;
        });

        let lhs = NonZeroU32::try_make(2).unwrap();
        let rhs = NonZeroU32::try_make(2).unwrap();
        assert_eq!(lhs.checked_sub(rhs), None);
    }

    #[test]
    fn explicit_non_zero_storage_has_a_niche_without_a_domain() {
        assert_eq!(
            core::mem::size_of::<Option<RequiredNonZero>>(),
            core::mem::size_of::<u8>(),
        );
    }

    #[test]
    fn explicit_private_visibility_allows_storage_rewriting() {
        assert_eq!(
            ExplicitlyPrivateNonZero::try_make(7)
                .map(ExplicitlyPrivateNonZero::raw),
            Ok(7),
        );
        assert_eq!(
            core::mem::size_of::<Option<ExplicitlyPrivateNonZero>>(),
            core::mem::size_of::<u8>(),
        );
    }

    #[test]
    fn suppressing_non_zero_keeps_primitive_storage() {
        assert_eq!(
            core::mem::size_of::<Option<PrimitivePositive>>(),
            core::mem::size_of::<Option<u8>>(),
        );
        assert_eq!(PrimitivePositive::try_make(0), Err(0));
    }

    #[test]
    fn range_validation_lowers_open_and_closed_endpoints() {
        assert_eq!(BelowFive::try_make(0).map(BelowFive::raw), Ok(0));
        assert_eq!(BelowFive::try_make(4).map(BelowFive::raw), Ok(4));
        assert_eq!(BelowFive::try_make(5), Err(5));

        assert_eq!(AtMostFive::try_make(0).map(AtMostFive::raw), Ok(0));
        assert_eq!(AtMostFive::try_make(5).map(AtMostFive::raw), Ok(5));
        assert_eq!(AtMostFive::try_make(6), Err(6));

        assert_eq!(
            FromNegativeTwo::try_make(-2).map(FromNegativeTwo::raw),
            Ok(-2)
        );
        assert_eq!(
            FromNegativeTwo::try_make(i8::MAX).map(FromNegativeTwo::raw),
            Ok(i8::MAX)
        );
        assert_eq!(FromNegativeTwo::try_make(-3), Err(-3));
    }

    #[test]
    fn accepts_negative_lower_bound_and_rejects_negative_odd() {
        let value = -8;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = -7;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_negative_lower_bound_and_rejects_negative_out_of_range() {
        let value = -8;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = -5;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_negative_lower_bound_and_rejects_zero() {
        let value = -8;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = 0;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_negative_lower_bound_and_rejects_positive_odd() {
        let value = -8;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = 5;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_negative_inner_bound_and_rejects_negative_odd() {
        let value = -6;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = -7;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_negative_inner_bound_and_rejects_negative_out_of_range() {
        let value = -6;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = -5;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_negative_inner_bound_and_rejects_zero() {
        let value = -6;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = 0;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_negative_inner_bound_and_rejects_positive_odd() {
        let value = -6;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = 5;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_positive_inner_bound_and_rejects_negative_odd() {
        let value = 4;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = -7;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_positive_inner_bound_and_rejects_negative_out_of_range() {
        let value = 4;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = -5;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_positive_inner_bound_and_rejects_zero() {
        let value = 4;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = 0;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_positive_inner_bound_and_rejects_positive_odd() {
        let value = 4;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = 5;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_positive_upper_bound_and_rejects_negative_odd() {
        let value = 6;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = -7;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_positive_upper_bound_and_rejects_negative_out_of_range() {
        let value = 6;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = -5;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_positive_upper_bound_and_rejects_zero() {
        let value = 6;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = 0;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn accepts_positive_upper_bound_and_rejects_positive_odd() {
        let value = 6;
        assert_eq!(SplitEven::try_make(value).map(SplitEven::raw), Ok(value));

        let value = 5;
        assert_eq!(SplitEven::try_make(value), Err(value));
    }

    #[test]
    fn range_validation_is_const() {
        assert_eq!(RANGED_NUMBER.raw(), 3);
    }

    #[test]
    fn range_validation_checks_friended_construction() {
        assert_panics(|| Ranged::of(5u8));
    }

    #[test]
    fn range_validation_checks_math_results() {
        let value = Ranged::try_make(4).unwrap();
        assert_panics(|| value + 1u8);
    }

    #[test]
    fn self_operations_do_not_require_friendship() {
        let one = SelfOperations::make(1);
        let two = SelfOperations::make(2);

        assert_eq!((one + two).raw(), 3);
        assert_eq!((one | two).raw(), 3);

        let mut value = one;
        value += two;
        assert_eq!(value.raw(), 3);
        value &= SelfOperations::make(1);
        assert_eq!(value.raw(), 1);
    }

    #[test]
    fn negative_isize_to_u128_is_rejected() {
        let raw = -1isize;
        let value = PointerSigned::make(raw);
        let result = value.try_into_u128();
        let via_trait: Result<u128, ()> = value.try_into();

        assert_eq!(result, Err(()), "{raw}");
        assert_eq!(via_trait, Err(()), "{raw}");
    }

    #[test]
    fn negative_isize_to_u64_is_rejected() {
        let raw = -1isize;
        let value = PointerSigned::make(raw);
        let result = value.try_into_u64();
        let via_trait: Result<u64, ()> = value.try_into();

        assert_eq!(result, Err(()), "{raw}");
        assert_eq!(via_trait, Err(()), "{raw}");
    }

    #[test]
    fn pointer_bounds_validate_non_zero_domain() {
        let zero =
            PointerNonZeroSigned::try_make(0).map(PointerNonZeroSigned::raw);
        let minimum = PointerNonZeroSigned::try_make(isize::MIN)
            .map(PointerNonZeroSigned::raw);

        assert_eq!(zero, Err(0));
        assert_eq!(minimum, Ok(isize::MIN));
    }

    #[test]
    fn negative_i16_to_u16_is_rejected() {
        let raw = -1i16;
        let value = SignedNarrow::make(raw);
        let result = value.try_into_u16();
        let via_trait: Result<u16, ()> = value.try_into();

        assert_eq!(result, Err(()), "{raw}");
        assert_eq!(via_trait, Err(()), "{raw}");
    }

    #[test]
    fn converts_into_u32() {
        let it = My32::make(23);
        let value: u32 = it.into_u32();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_u64() {
        let it = My32::make(23);
        let value: u64 = it.into_u64();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_u128() {
        let it = My32::make(23);
        let value: u128 = it.into_u128();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_i64() {
        let it = My32::make(23);
        let value: i64 = it.into_i64();
        assert_eq!(value, 23);
    }

    #[test]
    fn converts_into_i128() {
        let it = My32::make(23);
        let value: i128 = it.into_i128();
        assert_eq!(value, 23);
    }

    #[test]
    fn safely_tries_into_usize() {
        let it = My32::make(23);
        assert_eq!(it.try_into_usize(), Ok(23));
    }

    #[test]
    fn safely_tries_into_u32() {
        let it = My32::make(23);
        assert_eq!(it.try_into_u32(), Ok(23));
    }

    #[test]
    fn safely_tries_into_u64() {
        let it = My32::make(23);
        assert_eq!(it.try_into_u64(), Ok(23));
    }

    #[test]
    fn safely_tries_into_u128() {
        let it = My32::make(23);
        assert_eq!(it.try_into_u128(), Ok(23));
    }

    #[test]
    fn safely_tries_into_i64() {
        let it = My32::make(23);
        assert_eq!(it.try_into_i64(), Ok(23));
    }

    #[test]
    fn safely_tries_into_i128() {
        let it = My32::make(23);
        assert_eq!(it.try_into_i128(), Ok(23));
    }

    #[test]
    fn checked_conversion_to_isize_accepts_representable_value() {
        let it = My32::make(23);
        assert_eq!(it.try_into_isize(), Ok(23));
    }

    #[test]
    fn checked_conversion_to_u8_rejects_overflow() {
        let it = My32::make(256);
        assert_eq!(it.try_into_u8(), Err(()));
    }

    #[test]
    fn checked_conversion_to_u16_rejects_overflow() {
        let it = My32::make(65_536);
        assert_eq!(it.try_into_u16(), Err(()));
    }

    #[test]
    fn checked_conversion_to_u16_accepts_maximum() {
        let raw = u32::from(u16::MAX);
        let value = My32::make(raw);
        let result = value.try_into_u16();

        assert_eq!(result, Ok(u16::MAX), "{raw}");
    }

    #[test]
    fn checked_conversion_to_i8_rejects_overflow() {
        let it = My32::make(128);
        assert_eq!(it.try_into_i8(), Err(()));
    }

    #[test]
    fn checked_conversion_to_i16_rejects_overflow() {
        let it = My32::make(32_768);
        assert_eq!(it.try_into_i16(), Err(()));
    }

    #[test]
    fn checked_conversion_to_i32_rejects_overflow() {
        let it = My32::make(2_147_483_648);
        assert_eq!(it.try_into_i32(), Err(()));
    }

    #[test]
    fn checked_conversion_from_signed_to_unsigned_rejects_negative() {
        let value = MyI32::make(-1);
        let result = value.try_into_u32();
        let via_trait: Result<u32, ()> = value.try_into();

        assert_eq!(result, Err(()));
        assert_eq!(via_trait, Err(()));
    }

    #[test]
    fn implements_into_u32() {
        let it = My32::make(23);
        let value: u32 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_u64() {
        let it = My32::make(23);
        let value: u64 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_u128() {
        let it = My32::make(23);
        let value: u128 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_i64() {
        let it = My32::make(23);
        let value: i64 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn widening_into_i64_preserves_u32_maximum() {
        let raw = u32::MAX;
        let value = My32::make(raw);
        let result: i64 = value.into();

        assert_eq!(result, i64::from(raw), "{raw}");
    }

    #[test]
    fn implements_into_i128() {
        let it = My32::make(23);
        let value: i128 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_try_into_u8() {
        let it = My32::make(256);
        let value: Result<u8, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn implements_try_into_u16() {
        let it = My32::make(65_536);
        let value: Result<u16, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn implements_try_into_i8() {
        let it = My32::make(128);
        let value: Result<i8, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn implements_try_into_i16() {
        let it = My32::make(32_768);
        let value: Result<i16, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn implements_try_into_i32() {
        let it = My32::make(2_147_483_648);
        let value: Result<i32, ()> = it.try_into();
        assert_eq!(value, Err(()));
    }

    #[test]
    fn bitwise_ors_friend_value() {
        let it = My32::make(0b10001);
        assert_eq!((it | 0b00110).raw(), 0b10111);
    }

    #[test]
    fn bitwise_xors_friend_value() {
        let it = My32::make(0b10111);
        assert_eq!((it ^ 0b01000).raw(), 0b11111);
    }

    #[test]
    fn shifts_left() {
        let it = My32::make(23);
        assert_eq!((it << 1).raw(), 46);
    }

    #[test]
    fn shifts_right() {
        let it = My32::make(46);
        assert_eq!((it >> 1).raw(), 23);
    }

    #[test]
    fn bitwise_nots() {
        let it = My32::make(0);
        assert_eq!((!it).raw(), u32::MAX);
    }

    #[test]
    fn adds_assign() {
        let it = My32::make(20);
        let mut value = it;
        let it = My32::make(3);
        value += it;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn add_assignment_does_not_require_binary_add() {
        let lhs = AssignmentWithoutBinary::make(4);
        let rhs = 2u8;
        let mut result = lhs;
        result += rhs;

        assert_eq!(result.raw(), 6);
    }

    #[test]
    fn subtracts_assign() {
        let it = My32::make(26);
        let mut value = it;
        let it = My32::make(3);
        value -= it;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn multiplies_assign() {
        let it = My32::make(23);
        let mut value = it;
        let it = My32::make(3);
        value *= it;
        assert_eq!(value.raw(), 69);
    }

    #[test]
    fn divides_assign() {
        let it = My32::make(69);
        let mut value = it;
        let it = My32::make(3);
        value /= it;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn takes_remainder_assign() {
        let it = My32::make(70);
        let mut value = it;
        let it = My32::make(47);
        value %= it;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn bitwise_ands_assign() {
        let it = My32::make(0b10111);
        let mut value = it;
        let it = My32::make(0b10111);
        value &= it;
        assert_eq!(value.raw(), 0b10111);
    }

    #[test]
    fn bitwise_ors_assign() {
        let it = My32::make(0b10001);
        let mut value = it;
        let it = My32::make(0b00110);
        value |= it;
        assert_eq!(value.raw(), 0b10111);
    }

    #[test]
    fn or_assignment_does_not_require_binary_or() {
        let lhs = AssignmentWithoutBinary::make(0b1000);
        let rhs = 0b0011u8;
        let mut result = lhs;
        result |= rhs;

        assert_eq!(result.raw(), 0b1011);
    }

    #[test]
    fn shifts_left_assign() {
        let it = My32::make(23);
        let mut value = it;
        value <<= 1;
        assert_eq!(value.raw(), 46);
    }

    #[test]
    fn shifts_right_assign() {
        let it = My32::make(46);
        let mut value = it;
        value >>= 1;
        assert_eq!(value.raw(), 23);
    }

    #[test]
    fn implements_eq() {
        accepts_eq::<My32>();
    }

    #[test]
    fn implements_ord() {
        let it = My32::make(24);
        let other = it;
        let it = My32::make(23);
        assert_eq!(it.cmp(&other), Ordering::Less);
    }

    #[test]
    fn const_mode_display_preserves_signed_decimal_formatting() {
        let negative = MyI32::make(-23);
        assert_eq!(format!("{}", negative), "-23");
        assert_eq!(format!("{:+06}", negative), "-00023");
        assert_eq!(format!("{:+06}", MyI32::make(23)), "+00023");
        assert_eq!(format!("{:_<7}", negative), "-23____");
        assert_eq!(format!("{:*^7}", negative), "**-23**");
        assert_eq!(format!("{:>6}", negative), "   -23");
    }

    #[test]
    fn const_mode_display_uses_configured_raw_accessor() {
        let value = EncodedWithDisplay::make(123);
        assert_eq!(encoded_with_display_storage(value), 123);
        assert_eq!(format!("{}", value), "23");
        assert_eq!(format!("{:+06}", value), "+00023");
    }

    #[test]
    fn formats_debug() {
        let it = My32::make(23);
        assert_eq!(format!("{:?}", it), "My32(23)");
    }

    #[test]
    fn formats_binary() {
        let it = My32::make(23);
        assert_eq!(format!("{:b}", it), "10111");
    }

    #[test]
    fn formats_octal() {
        let it = My32::make(23);
        assert_eq!(format!("{:o}", it), "27");
    }

    #[test]
    fn formats_lower_hex() {
        let it = My32::make(23);
        assert_eq!(format!("{:x}", it), "17");
    }

    #[test]
    fn formats_upper_hex() {
        let it = My32::make(23);
        assert_eq!(format!("{:X}", it), "17");
    }

    #[test]
    fn preserves_transparent_layout() {
        assert_eq!(size_of::<My32>(), size_of::<u32>());
    }

    #[test]
    fn same_type_construction_requires_explicit_target_trust_to_bypass_validation()
     {
        for raw in [
            3,
            12,
        ] {
            let untrusted_target = Number::of(BlindTrustSource(Box::new(raw)));
            assert_panics(|| Number::of(untrusted_target));

            let blind_trust_target = BlindTrustSelfNumber::of(raw);
            assert_eq!(BlindTrustSelfNumber::of(blind_trust_target).raw(), raw);
            assert_eq!(BlindTrustSelfNumber::try_make(raw), Err(raw));
        }
    }

    #[test]
    fn untrusted_owned_friends_reject_odd_values() {
        let raw = 3;
        assert_panics(|| Number::of(DefaultSource(Box::new(raw))));
        assert_panics(|| Number::of(CheckedSource(Box::new(raw))));

        assert_eq!(Number::of(DefaultSource(Box::new(4))).raw(), 4);
        assert_eq!(Number::of(CheckedSource(Box::new(4))).raw(), 4);
    }

    #[test]
    fn untrusted_owned_friends_reject_out_of_range_values() {
        let raw = 12;
        assert_panics(|| Number::of(DefaultSource(Box::new(raw))));
        assert_panics(|| Number::of(CheckedSource(Box::new(raw))));

        assert_eq!(Number::of(DefaultSource(Box::new(4))).raw(), 4);
        assert_eq!(Number::of(CheckedSource(Box::new(4))).raw(), 4);
    }

    #[test]
    fn blind_trust_owned_friend_bypasses_odd_value_construction() {
        let raw = 3;
        let source = BlindTrustSource(Box::new(raw));
        assert_eq!(Number::of(source).raw(), raw);
        assert_eq!(Number::try_make(raw), Err(raw));

        assert_panics(|| {
            let _ =
                Number::try_make(4).unwrap() + BlindTrustSource(Box::new(1));
        });
        assert_panics(|| {
            let _ =
                Number::try_make(4).unwrap() | BlindTrustSource(Box::new(1));
        });
        assert_panics(|| {
            let _ = Number::try_make(6).unwrap() + Number::try_make(6).unwrap();
        });
    }

    #[test]
    fn blind_trust_owned_friend_bypasses_out_of_range_construction() {
        let raw = 12;
        let source = BlindTrustSource(Box::new(raw));
        assert_eq!(Number::of(source).raw(), raw);
        assert_eq!(Number::try_make(raw), Err(raw));

        assert_panics(|| {
            let _ =
                Number::try_make(4).unwrap() + BlindTrustSource(Box::new(1));
        });
        assert_panics(|| {
            let _ =
                Number::try_make(4).unwrap() | BlindTrustSource(Box::new(1));
        });
        assert_panics(|| {
            let _ = Number::try_make(6).unwrap() + Number::try_make(6).unwrap();
        });
    }

    #[test]
    fn blind_trust_assignment_rejects_odd_rhs_without_mutating() {
        let rhs = 1;
        let mut number = Number::try_make(4).unwrap();
        assert_panics(move || number += BlindTrustSource(Box::new(rhs)));
        assert_eq!(number.raw(), 4);

        assert_panics(move || number |= BlindTrustSource(Box::new(rhs)));
        assert_eq!(number.raw(), 4);
    }

    #[test]
    fn blind_trust_assignment_rejects_out_of_range_rhs_without_mutating() {
        let rhs = 8;
        let mut number = Number::try_make(4).unwrap();
        assert_panics(move || number += BlindTrustSource(Box::new(rhs)));
        assert_eq!(number.raw(), 4);

        assert_panics(move || number |= BlindTrustSource(Box::new(rhs)));
        assert_eq!(number.raw(), 4);
    }

    #[test]
    fn blind_trust_op_friends_consume_sources_and_validate_results() {
        let math_source = MathOnlySource(Box::new(2));
        assert_eq!((Number::try_make(4).unwrap() + math_source).raw(), 6);
        let bit_source = BitOnlySource(Box::new(2));
        assert_eq!((Number::try_make(4).unwrap() | bit_source).raw(), 6);

        for rhs in [
            1,
            8,
        ] {
            assert_panics(|| {
                let _ = Number::try_make(4).unwrap()
                    + MathOnlySource(Box::new(rhs));
            });
            assert_panics(|| {
                let _ =
                    Number::try_make(4).unwrap() | BitOnlySource(Box::new(rhs));
            });

            let mut number = Number::try_make(4).unwrap();
            assert_panics(move || number += MathOnlySource(Box::new(rhs)));
            assert_eq!(number.raw(), 4);
            assert_panics(move || number |= BitOnlySource(Box::new(rhs)));
            assert_eq!(number.raw(), 4);
        }
        assert_eq!(Number::try_make(3), Err(3));
        assert_eq!(Number::try_make(12), Err(12));
    }

    #[test]
    fn math_friend_enables_construction_math_and_bit_operations() {
        let construction_source = MathOnlySource(Box::new(2));
        assert_eq!(Number::of(construction_source).raw(), 2);

        let math_source = MathOnlySource(Box::new(2));
        assert_eq!((Number::try_make(4).unwrap() + math_source).raw(), 6);

        let bit_source = MathOnlySource(Box::new(2));
        assert_eq!((Number::try_make(4).unwrap() | bit_source).raw(), 6);
    }

    #[test]
    fn rel_friend_enables_cross_type_equality_and_ordering() {
        let value = Number::try_make(4).unwrap();

        assert_eq!(value, 4u8);
        assert_ne!(value, 2u8);
        assert!(value < 6u8);
        assert!(value <= 4u8);
        assert!(value > 2u8);
        assert!(value >= 4u8);
    }

    #[test]
    fn display_is_signed_decimal_and_preserves_formatter_flags() {
        let negative = NumWithDisplay::make(-23);
        assert_eq!(format!("{}", negative), "-23");
        assert_eq!(
            format!("{}", NumWithDisplay::make(i32::MIN)),
            "-2147483648"
        );
        assert_eq!(format!("{:+}", NumWithDisplay::make(23)), "+23");
        assert_eq!(format!("{:+06}", NumWithDisplay::make(23)), "+00023");
        assert_eq!(format!("{:06}", negative), "-00023");
        assert_eq!(format!("{:_<7}", negative), "-23____");
        assert_eq!(format!("{:*^7}", negative), "**-23**");
        assert_eq!(format!("{:>6}", negative), "   -23");
    }

    #[test]
    fn display_uses_configured_raw_accessor() {
        let value = EncodedWithDisplay::make(123);
        assert_eq!(encoded_with_display_storage(value), 123);
        assert_eq!(format!("{}", value), "23");
        assert_eq!(format!("{:+06}", value), "+00023");
    }

    #[test]
    fn traits_and_layout_can_be_supplied_by_the_caller() {
        accepts_copy::<IntegralWithCallerProvidedRepr>();
        assert_eq!(IntegralWithCallerProvidedRepr::make(1).raw(), 1);
    }

    #[test]
    fn validated_integral_has_no_make() {
        assert_no_associated_fn!(ValidatedWithoutMake, make);
    }

    #[test]
    fn make_does_not_grant_math() {
        let source = MakeSource(2);
        let number = MakeFriendNumber::of(source);

        assert_eq!(number.raw(), 2);
        assert_not_impl!(MakeFriendNumber: core::ops::Add<MakeSource>);
    }

    #[test]
    fn bit_does_not_grant_math() {
        let source = BitSource(2);
        let combined = BitFriendNumber::make(1) | source;

        assert_eq!(combined.raw(), 3);
        assert_not_impl!(BitFriendNumber: core::ops::Add<BitSource>);
    }

    #[test]
    fn without_removes_raw() {
        let value = WithoutRaw::make(3);

        assert_eq!(value.value(), 3);
        assert_no_method!(WithoutRaw, raw);
    }

    #[test]
    fn without_removes_self_math() {
        assert_not_impl!(WithoutSelfMath: core::ops::Add<WithoutSelfMath>);
    }

    #[test]
    fn without_removes_self_bit() {
        assert_not_impl!(WithoutSelfBit: core::ops::BitOr<WithoutSelfBit>);
    }

    #[test]
    fn output_type_removes_bitxor_assignment() {
        assert_not_impl!(
            OutputWithoutXorAssign: core::ops::BitXorAssign<OutputWithoutXorAssign>
        );
    }

    #[test]
    fn point_profile_has_no_point_addition() {
        assert_not_impl!(PointOnly: core::ops::Add<PointOnly>);
    }

    #[test]
    fn point_profile_has_no_point_addition_assignment() {
        assert_not_impl!(PointOnly: core::ops::AddAssign<PointOnly>);
    }

    #[test]
    fn point_profile_has_no_primitive_subtraction() {
        assert_not_impl!(PointOnly: core::ops::Sub<u8>);
    }

    #[test]
    fn point_profile_has_no_core_integer_helpers() {
        assert_no_method!(PointOnly, count_ones);
    }

    #[test]
    fn circular_profile_without_precedes_has_no_precedes_method() {
        assert_no_method!(CircularWithoutPrecedes, precedes);
    }

    #[test]
    fn circular_profile_has_no_linear_ordering() {
        assert_not_impl!(CircularSequence: core::cmp::Ord);
    }

    #[test]
    fn circular_profile_has_no_primitive_arithmetic() {
        assert_not_impl!(CircularSequence: core::ops::Add<u8>);
    }

    #[test]
    fn circular_profile_has_no_core_integer_helpers() {
        assert_no_method!(CircularSequence, count_ones);
    }

    #[test]
    fn without_removes_from_str() {
        assert_not_impl!(WithoutFromStr: core::str::FromStr);
    }

    #[test]
    fn without_removes_common_integral_methods() {
        assert_no_method!(CoreDisabled, count_ones);
    }

    #[test]
    fn core_integral_methods_exclude_unsafe_operations() {
        assert_no_method!(DefaultCoreMethods, unchecked_add);
    }

    #[test]
    fn forbid_unsafe_code_accepts_inferred_non_zero_storage() {
        use forbid_unsafe::Positive;

        let zero = Positive::try_make(0).map(Positive::raw);
        let one = Positive::try_make(1).map(Positive::raw);
        let option_size = core::mem::size_of::<Option<Positive>>();

        assert_eq!(zero, Err(0));
        assert_eq!(one, Ok(1));
        assert_eq!(option_size, core::mem::size_of::<u32>());
    }
}

#[cfg(test)]
#[cfg(target_pointer_width = "64")]
mod pointer_width_64 {
    use super::*;

    #[test]
    fn usize_exposes_upper_bytes_on_64_bit_targets() {
        let raw = 0x0123_4567_89ab_cdefusize;
        let value = PointerUnsigned::make(raw);
        let upper_byte = value.byte4();
        let highest_byte = value.byte7();

        assert_eq!(upper_byte, 0x67, "{raw}");
        assert_eq!(highest_byte, 0x01, "{raw}");
    }

    #[test]
    fn u64_to_usize_is_infallible_on_64_bit_targets() {
        let raw = u64::MAX;
        let value = MyAuto64::make(raw);
        let converted = value.into_usize();
        let via_trait: usize = value.into();
        let checked = value.try_into_usize();

        assert_eq!(converted, usize::MAX, "{raw}");
        assert_eq!(via_trait, usize::MAX, "{raw}");
        assert_eq!(checked, Ok(usize::MAX), "{raw}");
    }

    #[test]
    fn u64_is_an_automatic_usize_make_friend_on_64_bit_targets() {
        let raw = u64::MAX;
        let value = PointerUnsigned::of(raw);
        let result = value.raw();

        assert_eq!(result, usize::MAX, "{raw}");
    }

    #[test]
    fn usize_to_u64_is_infallible() {
        let raw = usize::MAX;
        let value = PointerUnsigned::make(raw);
        let result: u64 = value.into();

        assert_eq!(result, u64::MAX, "{raw}");
    }

    #[test]
    fn isize_to_i64_is_infallible() {
        let raw = isize::MIN;
        let value = PointerSigned::make(raw);
        let result: i64 = value.into();

        assert_eq!(result, i64::MIN, "{raw}");
    }

    #[test]
    fn isize_domain_includes_native_minimum_when_first_range_is_nonempty() {
        let raw = isize::MIN;
        let result = PointerDomainIncludesMinimum::try_make(raw)
            .map(PointerDomainIncludesMinimum::raw);

        assert_eq!(result, Ok(raw), "{raw}");
    }

    #[test]
    fn isize_domain_excludes_native_minimum_when_singleton_is_above_it() {
        let raw = isize::MIN;
        let result = PointerDomainExcludesMinimum::try_make(raw);

        assert_eq!(result, Err(raw), "{raw}");
    }

    #[test]
    fn implements_into_isize_when_u32_fits() {
        let raw = u32::MAX;
        let it = My32::make(raw);
        let value: isize = it.into();
        assert_eq!(value, i64::from(raw) as isize);
    }

    #[test]
    fn blanket_try_into_isize_is_infallible_when_u32_fits() {
        let raw = u32::MAX;
        let value = My32::make(raw);
        let result: Result<isize, core::convert::Infallible> = value.try_into();

        assert_eq!(result, Ok(i64::from(raw) as isize));
    }
}

#[cfg(test)]
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
mod pointer_width_32_or_64 {
    use super::*;

    #[test]
    fn converts_into_usize() {
        let it = My32::make(23);
        let value: usize = it.into_usize();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_into_usize() {
        let it = My32::make(23);
        let value: usize = it.into();
        assert_eq!(value, 23);
    }
}

#[cfg(test)]
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
mod pointer_width_16_or_32 {
    use super::*;

    #[test]
    fn explicitly_declared_u64_make_friend_is_available() {
        let raw = 23u64;
        let value = ExplicitPointerFriend::of(raw);
        let result = value.raw();

        assert_eq!(result, 23, "{raw}");
    }

    #[test]
    fn implements_try_into_isize() {
        let it = My32::make(23);
        let value: Result<isize, ()> = it.try_into();
        assert_eq!(value, Ok(23));
    }
}

#[cfg(test)]
mod profiles {
    use super::*;

    #[test]
    fn delta_shorthand_applies_root_settings() {
        let lhs = DeltaExtended::make(200);
        let rhs = DeltaExtended::make(2);
        let product = lhs * rhs;

        assert_eq!(product.raw(), 144);
        assert_no_method!(DeltaExtended, precedes);
    }

    #[test]
    fn delta_profile_body_enables_wrapping_multiplication() {
        let lhs = DeltaBodyExtended::make(200);
        let rhs = DeltaBodyExtended::make(2);
        let product = lhs * rhs;

        assert_eq!(product.raw(), 144);
        assert_no_method!(DeltaBodyExtended, precedes);
    }

    #[test]
    fn int_profile_keeps_integral_defaults() {
        let lhs = IntProfile::make(5);
        let rhs = IntProfile::make(3);

        assert_eq!((lhs + rhs).raw(), 8);
        assert_eq!((lhs - rhs).raw(), 2);
        assert_eq!((lhs | rhs).raw(), 7);
        assert_eq!(lhs.cmp(&rhs), Ordering::Greater);
        assert_no_method!(IntProfile, precedes);
    }

    #[test]
    fn default_profile_is_linear_without_precedes() {
        let lhs = FullyGenerated::make(1);
        let rhs = FullyGenerated::make(2);

        assert_eq!(lhs.cmp(&rhs), Ordering::Less);
        assert_no_method!(FullyGenerated, precedes);
    }

    #[test]
    fn empty_profile_list_keeps_integral_defaults() {
        let lhs = EmptyProfileList::make(5);
        let rhs = EmptyProfileList::make(3);

        assert_eq!((lhs + rhs).raw(), 8);
        assert_eq!(lhs.cmp(&rhs), Ordering::Greater);
        assert_no_method!(EmptyProfileList, precedes);
    }

    #[test]
    fn bare_profile_preserves_following_root_flags() {
        let zero = BareCircularWithoutAdd::make(0);
        let one = BareCircularWithoutAdd::make(1);

        assert_eq!((zero - one).raw(), u8::MAX);
        assert_not_impl!(
            BareCircularWithoutAdd: core::ops::Add<BareCircularWithoutAdd>
        );
    }

    #[test]
    fn bare_profiles_mix_with_configured_profiles() {
        let maximum = BareMixedProfiles::make(u8::MAX);
        let one = BareMixedProfiles::make(1);
        let sum = maximum + one;

        assert_eq!(sum.raw(), 0);
        assert!(maximum.precedes(one));
    }

    #[test]
    fn bare_facet_preserves_following_root_flags() {
        let three = BareFacetWithoutAdd::make(3);
        let one = BareFacetWithoutAdd::make(1);

        assert_eq!((three - one).raw(), 2);
        assert_not_impl!(
            BareFacetWithoutAdd: core::ops::Add<BareFacetWithoutAdd>
        );
    }

    #[test]
    fn single_facet_preserves_following_root_flags() {
        let three = SingleFacetWithoutAdd::make(3);
        let one = SingleFacetWithoutAdd::make(1);

        assert_eq!((three - one).raw(), 2);
        assert_not_impl!(
            SingleFacetWithoutAdd: core::ops::Add<SingleFacetWithoutAdd>
        );
    }

    #[test]
    fn bare_facets_keep_configured_dispatch() {
        let small = BareFacetDispatch::make(10);
        let offset = FacetOffset(3);
        let product = small * offset;
        let large = BareFacetDispatch::make(200);

        assert_eq!(product.raw(), 30);
        assert_panics(move || large * FacetOffset(2));
        assert_not_impl!(BareFacetDispatch: core::ops::Add<FacetOffset>);
    }

    #[test]
    fn point_profile_keeps_subtraction_and_linear_ordering() {
        let lhs = PointOnly::make(5);
        let rhs = PointOnly::make(3);
        let difference = lhs - rhs;

        assert_eq!(difference.raw(), 2);
        assert_eq!(lhs.cmp(&rhs), Ordering::Greater);
    }

    #[test]
    fn point_profile_has_no_multiplication() {
        assert_not_impl!(PointOnly: core::ops::Mul<PointOnly>);
    }

    #[test]
    fn point_profile_has_no_self_bitwise_operations() {
        assert_not_impl!(PointOnly: core::ops::BitOr<PointOnly>);
    }

    #[test]
    fn point_profile_has_no_primitive_comparison() {
        assert_not_impl!(PointOnly: core::cmp::PartialEq<u8>);
    }

    #[test]
    fn point_profile_preserves_circular_policy() {
        let zero = CircularPoint::make(0);
        let one = CircularPoint::make(1);
        let difference = zero - one;

        assert_eq!(difference.raw(), u8::MAX);
        assert!(zero.precedes(one));
        assert_not_impl!(CircularPoint: core::cmp::Ord);
        assert_not_impl!(CircularPoint: core::ops::Add<CircularPoint>);
    }

    #[test]
    fn point_profile_preserves_delta_policy() {
        let zero = DeltaPoint::make(0);
        let one = DeltaPoint::make(1);
        let difference = zero - one;

        assert_eq!(difference.raw(), u8::MAX);
        assert_no_method!(DeltaPoint, precedes);
    }

    #[test]
    fn checked_point_keeps_linear_ordering_without_addition() {
        let lhs = CheckedPoint::make(5);
        let rhs = CheckedPoint::make(3);

        assert_eq!(lhs.cmp(&rhs), Ordering::Greater);
        assert_not_impl!(CheckedPoint: core::ops::Add<CheckedPoint>);
    }

    #[test]
    fn wrapping_point_keeps_linear_ordering_without_addition() {
        let lhs = WrappingPoint::make(5);
        let rhs = WrappingPoint::make(3);

        assert_eq!(lhs.cmp(&rhs), Ordering::Greater);
        assert_not_impl!(WrappingPoint: core::ops::Add<WrappingPoint>);
    }

    #[test]
    fn point_profile_allows_explicit_addition() {
        let lhs = PointWithAddition::make(2);
        let rhs = PointWithAddition::make(3);
        let sum = lhs + rhs;
        let mut assigned = PointWithAddition::make(2);
        assigned += rhs;

        assert_eq!(sum.raw(), 5);
        assert_eq!(assigned.raw(), 5);
    }

    #[test]
    fn point_profile_resets_earlier_addition_overrides() {
        let lhs = PointResetsAddition::make(5);
        let rhs = PointResetsAddition::make(3);
        let difference = lhs - rhs;

        assert_eq!(difference.raw(), 2);
        assert_not_impl!(
            PointResetsAddition: core::ops::Add<PointResetsAddition>
        );
        assert_not_impl!(
            PointResetsAddition: core::ops::AddAssign<PointResetsAddition>
        );
    }

    #[test]
    fn circular_profile_wraps_subtraction_assignment() {
        let mut assigned = CircularWithoutPrecedes::make(0);
        assigned -= CircularWithoutPrecedes::make(1);

        assert_eq!(assigned.raw(), u8::MAX);
    }

    #[test]
    fn circular_profile_has_no_multiplication() {
        assert_not_impl!(
            CircularWithoutPrecedes: core::ops::Mul<CircularWithoutPrecedes>
        );
    }

    #[test]
    fn circular_profile_has_no_bitwise_operations() {
        assert_not_impl!(
            CircularWithoutPrecedes: core::ops::BitAnd<CircularWithoutPrecedes>
        );
    }

    #[test]
    fn delta_profile_wraps_assignments() {
        let mut added = DeltaSequence::make(u8::MAX);
        added += DeltaSequence::make(1);
        let mut subtracted = DeltaSequence::make(0);
        subtracted -= DeltaSequence::make(1);

        assert_eq!(added.raw(), 0);
        assert_eq!(subtracted.raw(), u8::MAX);
    }

    #[test]
    fn delta_profile_has_no_precedes_method() {
        assert_no_method!(DeltaSequence, precedes);
    }

    #[test]
    fn delta_profile_has_no_multiplication() {
        assert_not_impl!(DeltaSequence: core::ops::Mul<DeltaSequence>);
    }

    #[test]
    fn delta_profile_has_no_linear_ordering() {
        assert_not_impl!(DeltaSequence: core::cmp::Ord);
    }

    #[test]
    fn delta_profile_has_no_core_integer_helpers() {
        assert_no_method!(DeltaSequence, count_ones);
    }

    #[test]
    fn sequential_profile_keeps_linear_ordering() {
        let lhs = SequentialCounter::make(1);
        let rhs = SequentialCounter::make(2);

        assert_eq!(lhs.cmp(&rhs), Ordering::Less);
        assert_no_method!(SequentialCounter, precedes);
    }

    #[test]
    fn sequential_profile_has_no_multiplication() {
        assert_not_impl!(SequentialCounter: core::ops::Mul<SequentialCounter>);
    }

    #[test]
    fn sequential_profile_has_no_self_bitwise_operations() {
        assert_not_impl!(
            SequentialCounter: core::ops::BitXor<SequentialCounter>
        );
    }

    #[test]
    fn wrapping_profile_keeps_bitwise_and_core_integer_methods() {
        let lhs = WrappingSequence::make(0b1100);
        let rhs = WrappingSequence::make(0b1010);
        let bits = lhs ^ rhs;

        assert_eq!(bits.raw(), 0b0110);
        assert_eq!(lhs.count_ones(), 2);
    }

    #[test]
    fn last_wrapping_profile_selects_wrapping_storage() {
        let maximum = CircularThenWrapping::make(u8::MAX);
        let storage: core::num::Wrapping<u8> =
            circular_then_wrapping_storage(maximum);
        let sum = maximum + CircularThenWrapping::make(1);

        assert_eq!(storage.0, u8::MAX);
        assert_eq!(sum.raw(), 0);
    }

    #[test]
    fn later_circular_profile_restores_primitive_storage() {
        let maximum = WrappingThenCircular::make(u8::MAX);
        let storage: u8 = wrapping_then_circular_storage(maximum);
        let sum = maximum + WrappingThenCircular::make(1);

        assert_eq!(storage, u8::MAX);
        assert_eq!(sum.raw(), 0);
    }

    #[test]
    fn root_wrapping_storage_survives_non_wrapping_facets() {
        let value = WrappingWithIntFacet::make(7);
        let storage: core::num::Wrapping<u8> =
            wrapping_with_int_facet_storage(value);
        let sum = value + FacetOffset(3);

        assert_eq!(storage.0, 7);
        assert_eq!(sum.raw(), 10);
    }

    #[test]
    fn root_flags_override_profile_body_regardless_of_attribute_order() {
        let root_first =
            RootAddBeforeProfile::make(u8::MAX) + RootAddBeforeProfile::make(1);
        let profile_first =
            ProfileBeforeRootAdd::make(u8::MAX) + ProfileBeforeRootAdd::make(1);

        assert_eq!(root_first.raw(), 0);
        assert_eq!(profile_first.raw(), 0);
    }

    #[test]
    fn profile_list_applies_profiles_in_order() {
        let value = OrderedProfiles::make(0b1011);

        assert_eq!(value.count_ones(), 3);
        assert_not_impl!(OrderedProfiles: core::ops::Add<OrderedProfiles>);
        assert_not_impl!(OrderedProfiles: core::ops::Sub<OrderedProfiles>);
        assert_not_impl!(OrderedProfiles: core::ops::Mul<OrderedProfiles>);
        assert_no_method!(OrderedProfiles, precedes);
        assert_no_method!(OrderedProfiles, lo8);
    }

    #[test]
    fn later_profile_body_overrides_earlier_profile_body() {
        let lhs = LaterProfileAdds::make(2);
        let rhs = LaterProfileAdds::make(3);
        let sum = lhs + rhs;

        assert_eq!(sum.raw(), 5);
    }
}

#[cfg(test)]
mod facets {
    use super::*;

    #[test]
    fn explicit_facet_wraps_and_keeps_self_output() {
        let maximum = ExplicitFacets::make(u8::MAX);
        let sum: ExplicitFacets = maximum + FacetOffset(1);
        let difference: ExplicitFacets = sum - FacetOffset(1);

        assert_eq!(sum.raw(), 0);
        assert_eq!(difference.raw(), u8::MAX);
    }

    #[test]
    fn facet_without_output_inherits_the_root_output() {
        let value = ExplicitFacets::make(0b1100);
        let bits: RootOutput = value | FacetBits(0b0011);

        assert_eq!(bits, RootOutput(0b1111));
        assert_not_impl!(ExplicitFacets: core::ops::Sub<FacetBits>);
    }

    #[test]
    fn root_fallback_uses_root_policy_output_and_construction() {
        let constructed = ExplicitFacets::of(RootRhs(7));
        let sum: RootOutput = constructed + RootRhs(3);
        let same_type: RootOutput = constructed + constructed;
        let maximum = ExplicitFacets::make(u8::MAX);

        assert_eq!(constructed.raw(), 7);
        assert_eq!(sum, RootOutput(10));
        assert_eq!(same_type, RootOutput(14));
        assert_panics(move || maximum + RootRhs(1));
    }

    #[test]
    fn facet_friend_defaults_to_math_without_bit() {
        let value = DefaultFacetFriend::make(2);
        let sum = value + FacetChecked(3);

        assert_eq!(sum.raw(), 5);
        assert_not_impl!(DefaultFacetFriend: core::ops::BitOr<FacetChecked>);
    }

    #[test]
    fn single_configured_facet_preserves_dispatch() {
        let maximum = SingleConfiguredFacet::make(u8::MAX);
        let sum: SingleConfiguredFacet =
            SingleConfiguredFacet::make(2) + FacetOffset(3);
        let zero = SingleConfiguredFacet::make(0);
        let one = SingleConfiguredFacet::make(1);
        let difference: FacetResult = zero - one;

        assert_eq!(sum.raw(), 5);
        assert_panics(move || maximum + FacetOffset(1));
        assert_eq!(difference, FacetResult(u8::MAX));
        assert_not_impl!(
            SingleConfiguredFacet: core::ops::Add<SingleConfiguredFacet>
        );
    }

    #[test]
    fn explicit_facet_claims_its_slot() {
        let maximum = SlotResolution::make(u8::MAX);
        let sum: SlotResolution = SlotResolution::make(2) + FacetOffset(3);

        assert_eq!(sum.raw(), 5);
        assert_panics(move || maximum + FacetOffset(1));
    }

    #[test]
    fn unclaimed_rhs_slot_keeps_the_root_facet() {
        let zero = SlotResolution::make(0);
        let difference: FacetResult = zero - FacetOffset(1);

        assert_eq!(difference, FacetResult(u8::MAX));
    }

    #[test]
    fn unclaimed_self_slot_keeps_the_root_facet() {
        let maximum = SlotResolution::make(u8::MAX);
        let one = SlotResolution::make(1);
        let sum: FacetResult = maximum + one;

        assert_eq!(sum, FacetResult(0));
    }

    #[test]
    fn self_facet_claims_the_self_slot() {
        let zero = SlotResolution::make(0);
        let one = SlotResolution::make(1);
        let difference: SlotSelfOutput = zero - one;

        assert_eq!(difference, SlotSelfOutput(u8::MAX));
    }

    #[test]
    fn distinct_facets_serve_different_operations_for_one_rhs() {
        let value = DistinctFacetOperations::make(10);
        let sum = value + FacetOffset(1);
        let difference = value - FacetOffset(1);

        assert_eq!(sum.raw(), 11);
        assert_eq!(difference.raw(), 8);
    }

    #[test]
    fn facets_dispatch_root_and_facet_friends_without_assignments() {
        let maximum = FacetDispatchOperators::make(u8::MAX);
        let by_facet = maximum + FacetWrapping(1);
        let by_root = maximum + FacetOffset(1);
        let by_self = maximum + FacetDispatchOperators::make(1);

        assert_eq!(by_facet.raw(), 0);
        assert_eq!(by_root.raw(), 0);
        assert_eq!(by_self.raw(), 0);
        assert_not_impl!(
            FacetDispatchOperators: core::ops::AddAssign<FacetDispatchOperators>
        );
    }

    #[test]
    fn faceted_friendship_retains_make_and_rel_capabilities() {
        let offset = FacetOffset(4);
        let value = FacetedFriendship::of(offset);

        assert_eq!(value.raw(), 4);
        assert!(value == offset);
        assert!(value < FacetOffset(5));
    }
}

#[cfg(test)]
mod output_types {
    use super::*;

    #[test]
    fn produce_accepts_a_qualified_callback_and_generic_target() {
        let lhs = GenericProduce::make(2);
        let rhs = GenericProduce::make(3);
        let sum: GenericOffset<u32> = lhs + rhs;

        assert_eq!(sum, GenericOffset(5));
    }

    #[test]
    fn produce_removes_add_assignment() {
        assert_not_impl!(GenericProduce: core::ops::AddAssign<GenericProduce>);
    }

    #[test]
    fn produce_removes_sub_assignment() {
        assert_not_impl!(GenericProduce: core::ops::SubAssign<GenericProduce>);
    }

    #[test]
    fn produce_removes_and_assignment() {
        assert_not_impl!(
            GenericProduce: core::ops::BitAndAssign<GenericProduce>
        );
    }

    #[test]
    fn produce_removes_or_assignment() {
        assert_not_impl!(
            GenericProduce: core::ops::BitOrAssign<GenericProduce>
        );
    }

    #[test]
    fn with_const_map_fn_makes_map_const_without_konst() {
        assert_eq!(CONST_MAP_FN.raw(), 23);
    }

    #[test]
    fn without_removes_map() {
        assert_no_method!(WithoutSerdeAndMap, map);
    }

    #[test]
    fn without_removes_serde() {
        assert_not_impl!(WithoutSerdeAndMap: serde::Serialize);
    }

    #[test]
    fn without_removes_display() {
        assert_not_impl!(WithoutDisplay: core::fmt::Display);
    }

    #[test]
    fn empty_friend_list_keeps_automatic_raw_friends() {
        let raw = 4u8;
        let value = EmptyFriendList::of(raw);
        let sum = value + 1u8;

        assert_eq!(value.raw(), raw, "{raw}");
        assert_eq!(sum.raw(), 5, "{raw}");
    }

    #[test]
    fn bracketed_tree_generates_every_root() {
        fn accepts_mode<T: IntegralMode>() {}
        fn accepts_flow<T: IntegralFlow>() {}

        accepts_mode::<IntegralActive>();
        accepts_flow::<IntegralForward>();
        accepts_flow::<IntegralBackward>();
        assert_not_impl!(IntegralActive: IntegralFlow);
        assert_eq!(size_of::<IntegralActive>(), 0);
    }

    #[test]
    fn wrapping_storage_uses_a_custom_or_panic_constructor() {
        let accepted = CustomOrPanicWrapping::make(7);
        let limit = 101;

        assert_eq!(accepted.raw(), 7);
        assert_panics(move || CustomOrPanicWrapping::make(limit));
    }

    #[test]
    fn blind_trust_is_absent_without_flag_or_unsafe_friend() {
        assert_no_associated_fn!(NonZeroU8, _blind_trust);
    }
}

#[cfg(test)]
mod constness {
    use super::*;

    #[test]
    fn konst_math_family_makes_addition_const() {
        assert_eq!(KONST_MATH_SUM.raw(), 23);
    }

    #[test]
    fn konst_family_stays_const_for_operations_not_overridden() {
        assert_eq!(KONST_MATH_DIFFERENCE.raw(), 17);
    }

    #[test]
    fn nightly_konst_stays_const_for_operations_not_overridden() {
        assert_eq!(KONST_NIGHTLY_DIFFERENCE.raw(), 17);
    }

    #[test]
    fn konst_map_makes_map_const_without_konst() {
        assert_eq!(KONST_MAPPED.raw(), 23);
    }

    #[test]
    fn root_const_flag_overrides_profile_const_flags() {
        assert_eq!(PROFILE_KONST_SUM.raw(), 23);
    }

    #[test]
    fn default_konst_makes_validation_and_core_integer_methods_const() {
        assert_eq!(DEFAULT_KONST_ONES, 3);
    }

    #[test]
    fn explicit_const_flag_extends_the_default_konst() {
        assert_eq!(DEFAULT_KONST_SUM.raw(), 23);
    }

    #[test]
    fn stable_konst_makes_validation_const() {
        assert_eq!(KONST_STABLE.raw(), 7);
    }

    #[test]
    fn unvalidated_make_is_const_without_konst() {
        assert_eq!(NO_KONST_MADE.raw(), 7);
    }
}

#[cfg(test)]
mod friends {
    use super::*;

    #[test]
    fn raw_identifier_type_builds_its_friend_protocol() {
        let raw = 7;
        let value = r#Type::of(raw);

        assert_eq!(value.raw(), raw, "{raw}");
        assert_eq!(r#Type::make(raw), value, "{raw}");
    }

    #[test]
    fn no_primitive_make_friend_generates_no_make() {
        assert_no_associated_fn!(NoPrimitiveMake, make);
    }

    #[test]
    fn qualified_friend_paths_construct_with_blind_trust() {
        let odd = 3;
        let value = QualifiedFriend::of(QualifiedSource(odd));

        assert_eq!(value.raw(), odd, "{odd}");
        assert_not_impl!(QualifiedFriend: core::ops::Add<QualifiedSource>);
    }

    #[test]
    fn unsafe_friend_constructs_with_blind_trust() {
        let odd = 3;
        let value = UnsafeOnly::of(UnsafeSource(odd));

        assert_eq!(value.raw(), odd, "{odd}");
    }

    #[test]
    fn unsafe_friend_grants_no_math() {
        assert_not_impl!(UnsafeOnly: core::ops::Add<UnsafeSource>);
    }

    #[test]
    fn unsafe_friend_grants_no_bit() {
        assert_not_impl!(UnsafeOnly: core::ops::BitOr<UnsafeSource>);
    }

    #[test]
    fn bracketed_unsafe_friend_constructs_with_blind_trust() {
        let odd = 3;
        let value = BracketedUnsafeOnly::of(UnsafeSource(odd));

        assert_eq!(value.raw(), odd, "{odd}");
        assert_not_impl!(BracketedUnsafeOnly: core::ops::Add<UnsafeSource>);
    }

    #[test]
    fn capability_list_accepts_a_trailing_comma() {
        let odd = 3;
        let value = TrailingCommaCapabilities::of(UnsafeSource(odd));

        assert_eq!(value.raw(), odd, "{odd}");
    }

    #[test]
    fn bit_rel_and_unsafe_friend_combines_capabilities() {
        let odd = 3;
        let value = BitRelUnsafe::of(RelSource(odd));
        let even = BitRelUnsafe::try_make(4).unwrap();

        assert_eq!(value.raw(), odd, "{odd}");
        assert_eq!((even | RelSource(2)).raw(), 6);
        assert_panics(move || even | RelSource(1));
        assert!(even == RelSource(4));
        assert_not_impl!(BitRelUnsafe: core::ops::Add<RelSource>);
    }

    #[test]
    fn implicit_unsafe_friend_constructs_with_blind_trust() {
        let odd = 3;
        let value = ImplicitUnsafe::of(ImplicitSource(odd));

        assert_eq!(value.raw(), odd, "{odd}");
        assert_not_impl!(ImplicitUnsafe: core::ops::Add<ImplicitSource>);
    }

    #[test]
    fn implicit_friend_source_accepts_a_trailing_comma() {
        let odd = 3;
        let value = ImplicitTrailingComma::of(ImplicitSource(odd));

        assert_eq!(value.raw(), odd, "{odd}");
    }

    #[test]
    fn implicit_friend_defaults_to_math() {
        let value = ImplicitDefault::of(ImplicitSource(4));
        let sum = value + ImplicitSource(2);
        let bits = value | ImplicitSource(1);

        assert_eq!(value.raw(), 4);
        assert_eq!(sum.raw(), 6);
        assert_eq!(bits.raw(), 5);
    }

    #[test]
    fn implicit_math_friend_enables_construction_math_and_bits() {
        let value = ImplicitMath::of(ImplicitSource(4));
        let sum = value + ImplicitSource(2);
        let bits = value | ImplicitSource(1);

        assert_eq!(value.raw(), 4);
        assert_eq!(sum.raw(), 6);
        assert_eq!(bits.raw(), 5);
    }

    #[test]
    fn absolute_friend_paths_accept_a_trailing_source_comma() {
        let source = core::num::NonZeroU8::new(9).unwrap();
        let value = AbsoluteFriendPaths::of(source);

        assert_eq!(value.raw(), 9, "{source}");
        assert_not_impl!(
            AbsoluteFriendPaths: core::ops::Add<core::num::NonZeroU8>
        );
    }

    #[test]
    fn make_and_bit_friend_grants_no_math() {
        let value = MakeBitFriend::of(CapabilitySource(4));
        let bits = value | CapabilitySource(1);

        assert_eq!(value.raw(), 4);
        assert_eq!(bits.raw(), 5);
        assert_not_impl!(MakeBitFriend: core::ops::Add<CapabilitySource>);
    }

    #[test]
    fn math_friend_grants_make_and_bit_without_rel() {
        let value = ExplicitMathFriend::of(CapabilitySource(4));
        let sum = value + CapabilitySource(2);
        let bits = value | CapabilitySource(1);

        assert_eq!(value.raw(), 4);
        assert_eq!(sum.raw(), 6);
        assert_eq!(bits.raw(), 5);
        assert_not_impl!(
            ExplicitMathFriend: core::cmp::PartialEq<CapabilitySource>
        );
    }

    #[test]
    fn omitted_friend_capabilities_grant_no_rel() {
        assert_not_impl!(Number: core::cmp::PartialEq<MathOnlySource>);
    }
}

#[cfg(test)]
mod domain_membership {
    use super::*;

    #[test]
    fn validation_lists_combine_callbacks_and_domain_alternatives() {
        let zero = ValidationLists::try_make(0).map(ValidationLists::raw);
        let negative = ValidationLists::try_make(-8).map(ValidationLists::raw);
        let gap = ValidationLists::try_make(6).map(ValidationLists::raw);
        let odd = ValidationLists::try_make(3).map(ValidationLists::raw);
        let rejected = ValidationLists::try_make(-50).map(ValidationLists::raw);

        assert_eq!(zero, Ok(0));
        assert_eq!(negative, Ok(-8));
        assert_eq!(gap, Err(6));
        assert_eq!(odd, Err(3));
        assert_eq!(rejected, Err(-50));
    }

    #[test]
    fn primitive_constant_domain_preserves_boundaries() {
        let zero = ConstantBounds::try_make(0).map(ConstantBounds::raw);
        let maximum =
            ConstantBounds::try_make(u32::MAX).map(ConstantBounds::raw);

        assert_eq!(zero, Err(0));
        assert_eq!(maximum, Ok(u32::MAX));
    }

    #[test]
    fn unordered_domain_preserves_boundary_membership() {
        let below = UnorderedDomain::try_make(0).map(UnorderedDomain::raw);
        let lower = UnorderedDomain::try_make(1).map(UnorderedDomain::raw);
        let joined = UnorderedDomain::try_make(6).map(UnorderedDomain::raw);
        let upper = UnorderedDomain::try_make(12).map(UnorderedDomain::raw);
        let above = UnorderedDomain::try_make(13).map(UnorderedDomain::raw);

        assert_eq!(below, Err(0));
        assert_eq!(lower, Ok(1));
        assert_eq!(joined, Ok(6));
        assert_eq!(upper, Ok(12));
        assert_eq!(above, Err(13));
    }

    #[test]
    fn open_zero_upper_bound_excludes_zero() {
        let zero = BelowZero::try_make(0).map(BelowZero::raw);
        let below = BelowZero::try_make(-1).map(BelowZero::raw);

        assert_eq!(zero, Err(0));
        assert_eq!(below, Ok(-1));
        assert!(has_niche::<BelowZero>());
    }

    #[test]
    fn closed_zero_upper_bound_contains_zero() {
        let zero = AtMostZero::try_make(0).map(AtMostZero::raw);
        let above = AtMostZero::try_make(1).map(AtMostZero::raw);

        assert_eq!(zero, Ok(0));
        assert_eq!(above, Err(1));
        assert!(!has_niche::<AtMostZero>());
    }

    #[test]
    fn negative_domain_excludes_zero() {
        let zero = NegativeOpen::try_make(0).map(NegativeOpen::raw);
        let lowest = NegativeOpen::try_make(-5).map(NegativeOpen::raw);

        assert_eq!(zero, Err(0));
        assert_eq!(lowest, Ok(-5));
        assert!(has_niche::<NegativeOpen>());
    }

    #[test]
    fn split_domain_excludes_zero() {
        let zero = SplitAroundZero::try_make(0).map(SplitAroundZero::raw);
        let below = SplitAroundZero::try_make(-1).map(SplitAroundZero::raw);
        let above = SplitAroundZero::try_make(1).map(SplitAroundZero::raw);

        assert_eq!(zero, Err(0));
        assert_eq!(below, Ok(-1));
        assert_eq!(above, Ok(1));
        assert!(has_niche::<SplitAroundZero>());
    }

    #[test]
    fn adjacent_negative_domains_exclude_zero() {
        let zero = AdjacentNegative::try_make(0).map(AdjacentNegative::raw);
        let joined = AdjacentNegative::try_make(-5).map(AdjacentNegative::raw);
        let highest = AdjacentNegative::try_make(-1).map(AdjacentNegative::raw);

        assert_eq!(zero, Err(0));
        assert_eq!(joined, Ok(-5));
        assert_eq!(highest, Ok(-1));
        assert!(has_niche::<AdjacentNegative>());
    }

    #[test]
    fn domain_union_contains_zero_when_any_member_does() {
        let zero = UnionWithZero::try_make(0).map(UnionWithZero::raw);
        let above = UnionWithZero::try_make(6).map(UnionWithZero::raw);

        assert_eq!(zero, Ok(0));
        assert_eq!(above, Err(6));
        assert!(!has_niche::<UnionWithZero>());
    }

    #[test]
    fn signed_minimum_constant_is_negative() {
        let zero = FromSignedMinimum::try_make(0).map(FromSignedMinimum::raw);
        let minimum =
            FromSignedMinimum::try_make(i32::MIN).map(FromSignedMinimum::raw);

        assert_eq!(zero, Err(0));
        assert_eq!(minimum, Ok(i32::MIN));
        assert!(has_niche::<FromSignedMinimum>());
    }

    #[test]
    fn unsigned_minimum_constant_is_zero() {
        let zero =
            UnsignedMinimumOnly::try_make(0).map(UnsignedMinimumOnly::raw);
        let one =
            UnsignedMinimumOnly::try_make(1).map(UnsignedMinimumOnly::raw);

        assert_eq!(zero, Ok(0));
        assert_eq!(one, Err(1));
        assert!(!has_niche::<UnsignedMinimumOnly>());
    }

    #[test]
    fn pointer_domain_excluding_zero_selects_non_zero_storage() {
        assert!(has_niche::<PointerNonZeroSigned>());
    }

    #[test]
    fn pointer_domain_including_zero_keeps_primitive_storage() {
        let zero = PointerAroundZero::try_make(0).map(PointerAroundZero::raw);

        assert_eq!(zero, Ok(0));
        assert!(!has_niche::<PointerAroundZero>());
    }

    #[test]
    fn unsigned_pointer_minimum_domain_includes_zero() {
        let zero = PointerMinimumOnly::try_make(0).map(PointerMinimumOnly::raw);
        let one = PointerMinimumOnly::try_make(1).map(PointerMinimumOnly::raw);

        assert_eq!(zero, Ok(0));
        assert_eq!(one, Err(1));
        assert!(!has_niche::<PointerMinimumOnly>());
    }

    #[test]
    fn callback_validation_keeps_primitive_storage() {
        let zero = CallbackOnly::try_make(0).map(CallbackOnly::raw);

        assert_eq!(zero, Ok(0));
        assert!(!has_niche::<CallbackOnly>());
    }

    #[test]
    fn gapped_positive_domain_selects_non_zero_storage() {
        let gap = GappedPositive::try_make(50).map(GappedPositive::raw);
        let upper = GappedPositive::try_make(122).map(GappedPositive::raw);

        assert_eq!(gap, Err(50));
        assert_eq!(upper, Ok(122));
        assert!(has_niche::<GappedPositive>());
    }

    #[test]
    fn root_non_zero_overrides_profile_setting() {
        let zero = RootSuppressesProfileNonZero::make(0);

        assert_eq!(zero.raw(), 0);
        assert!(!has_niche::<RootSuppressesProfileNonZero>());
    }
}

#[cfg(test)]
mod lit_macro_names {
    use super::*;

    #[test]
    fn literal_macro_name_splits_a_leading_acronym() {
        let value = http_server!(80);

        assert_eq!(value.raw(), 80);
    }

    #[test]
    fn literal_macro_name_splits_an_embedded_acronym() {
        let value = my_url_value!(80);

        assert_eq!(value.raw(), 80);
    }

    #[test]
    fn literal_macro_name_keeps_an_alphanumeric_prefix() {
        let value = x509_certificate!(80);

        assert_eq!(value.raw(), 80);
    }
}

#[cfg(test)]
mod function_removals {
    use super::*;

    #[test]
    fn without_fn_removes_only_the_named_core_int_methods() {
        let raw = -6;
        let value = WithoutSomeIntFns::make(raw);

        crate::assert_no_method!(WithoutSomeIntFns, count_ones);
        crate::assert_no_method!(WithoutSomeIntFns, checked_add);
        crate::assert_no_method!(WithoutSomeIntFns, abs);
        crate::assert_no_associated_fn!(WithoutSomeIntFns, from_be_bytes);
        assert_eq!(value.count_zeros(), raw.count_zeros(), "{raw}");
        assert_eq!(value.checked_sub(value).map(|it| it.raw()), Some(0));
        assert_eq!(value.unsigned_abs(), raw.unsigned_abs(), "{raw}");
        let bytes = value.to_le_bytes();
        assert_eq!(WithoutSomeIntFns::from_le_bytes(bytes).raw(), raw);
    }

    #[test]
    fn profile_body_removes_a_core_int_method() {
        let raw = 6;
        let value = ProfileWithoutCountOnes::make(raw);

        crate::assert_no_method!(ProfileWithoutCountOnes, count_ones);
        assert_eq!(value.count_zeros(), raw.count_zeros(), "{raw}");
    }
}
