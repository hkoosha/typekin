use crate::*;

#[typekin::integral(konst = nightly)]
struct My32(u32);

#[typekin::integral(konst = nightly)]
struct MyAuto64(u64);

#[typekin::integral(konst = none, with = konst_add)]
struct ConstAddOnly(u8);

const CONST_ADD_ONLY: ConstAddOnly = ConstAddOnly(20) + ConstAddOnly(3);

#[typekin::integral(konst = nightly)]
struct ConstMap(u8);

const CONST_MAPPED: ConstMap = ConstMap(20).map(const |value| value + 3);

const fn accepts_final_mapping(value: u8) -> bool {
    return value == 0 || value >= 3;
}

#[typekin::integral(valid = accepts_final_mapping)]
struct Mapped(u8);

#[typekin::integral(konst = none)]
struct PlainMapped(u8);

#[typekin::integral]
struct FullyGenerated(u8);

#[typekin::integral(konst = nightly)]
struct FullyGeneratedConst(u8);

const FULLY_GENERATED_CLONE: FullyGeneratedConst =
    FullyGeneratedConst::make(23).clone();

#[typekin::integral(without = [clone, copy, repr])]
#[derive(Copy, Clone)]
struct ManuallyDerivedIntegral(u8);

#[typekin::integral(
    without = fn_conv_raw,
    get_raw = Self::decoded,
)]
struct AccessedMap(u8);

impl AccessedMap {
    const fn decoded(self) -> u8 {
        return self.0 - 100;
    }
}

#[typekin::integral(konst = nightly, with = display)]
struct MyI32(i32);

#[typekin::integral(
    konst = nightly,
    with = display,
    without = fn_conv_raw,
    get_raw = Self::decoded,
)]
struct EncodedDisplay(i32);

impl EncodedDisplay {
    const fn decoded(self) -> i32 {
        self.0 - 100
    }
}

const CONST_NUMBER: My32 = match My32::try_make(23) {
    Ok(number) => number,
    Err(_) => panic!("valid number rejected"),
};

const fn is_even(value: u8) -> bool {
    return value % 2 == 0;
}

const fn is_not_fifty(value: u8) -> bool {
    return value != 50;
}

#[typekin::integral(
    valid = is_even,
    with = serde,
)]
struct SerdeIntegral(u8);

const fn accepts_ranged(_: u8) -> bool {
    return true;
}

#[typekin::integral(
    konst = nightly,
    friends = self(u8) -> [Math, Rel],
    valid = [is_even, is_not_fifty],
    domain = 2..=100,
)]
struct Even(u8);

#[typekin::integral(
    konst = nightly,
    friends = [self(u8) -> [Math, Rel]],
    domain = [2..=4, 8..10],
    valid = accepts_ranged,
)]
struct Ranged(u8);

#[typekin::integral(konst = none, with = non_zero)]
struct RequiredNonZero(u8);

#[typekin::integral(konst = none, domain = 1..)]
struct NonZeroUsize(usize);

#[typekin::integral(konst = none, domain = 1..)]
struct NonZeroIsize(isize);

#[typekin::integral(konst = none, domain = 1..)]
struct NonZeroU8(u8);

#[typekin::integral(konst = none, domain = 1..)]
struct NonZeroU16(u16);

#[typekin::integral(konst = none, domain = 1..)]
struct NonZeroU32(u32);

#[typekin::integral(konst = none, domain = 1..)]
struct NonZeroU64(u64);

#[typekin::integral(konst = none, domain = 1..)]
struct NonZeroU128(u128);

#[typekin::integral(konst = none, domain = 1..)]
struct NonZeroI8(i8);

#[typekin::integral(konst = none, domain = 1..)]
struct NonZeroI16(i16);

#[typekin::integral(konst = none, domain = 1..)]
struct NonZeroI32(i32);

#[typekin::integral(konst = none, domain = 1..)]
struct NonZeroI64(i64);

#[typekin::integral(konst = none, domain = 1..)]
struct NonZeroI128(i128);

#[typekin::integral(
    konst = none,
    domain = 0..=10,
    with = non_zero,
)]
struct DomainRequiredNonZero(u8);

#[typekin::integral(konst = none, domain = 1..)]
struct ExplicitlyPrivateNonZero(pub(self) u8);

#[typekin::integral(
    konst = none,
    domain = 1..,
    with = [display, serde],
)]
struct NonZeroSerde(u8);

#[typekin::integral(
    konst = none,
    domain = 1..=10,
    without = non_zero,
)]
struct PrimitivePositive(u8);

#[typekin::integral(konst = nightly, domain = ..)]
struct AnyU8(u8);

#[typekin::integral(konst = nightly, domain = ..5)]
struct BelowFive(u8);

#[typekin::integral(konst = nightly, domain = ..=5)]
struct AtMostFive(u8);

#[typekin::integral(konst = nightly, domain = -2..)]
struct FromNegativeTwo(i8);

const fn is_even_i8(value: i8) -> bool {
    return value % 2 == 0;
}

#[typekin::integral(
    konst = nightly,
    valid = is_even_i8,
    domain = [..=-6, 4..],
)]
struct SplitEven(i8);

#[typekin::integral(
    konst = nightly,
    valid = is_even,
    without = fn_make_unchecked_try,
)]
struct ParsedEven(u8);

#[typekin::integral(
    konst = nightly,
    without = core_int,
)]
struct CoreDisabled(u8);

#[typekin::integral(
    konst = nightly,
    without = [
        self_friend_math_ops,
        self_friend_math_bit,
    ],
)]
struct SelfOperations(u8);

const RANGED_NUMBER: Ranged = match Ranged::try_make(3) {
    Ok(number) => number,
    Err(_) => panic!("valid ranged number rejected"),
};

#[typekin::integral(
    konst = nightly,
    friends = self(u16) -> [Math, Rel],
    get_raw = Self::value
)]
struct CustomRaw(u16);

#[typekin::integral(konst = nightly)]
struct Wide(u128);

#[typekin::integral(
    konst = nightly,
    friends = self(u8) -> [Math, unsafe],
    valid = is_even,
    domain = 2..=10,
)]
struct TrustEven(u8);

const TRUSTED_ODD_NUMBER: TrustEven = TrustEven::of(3u8);

// ==============================================================================

struct DefaultSource(Box<u8>);
struct CheckedSource(Box<u8>);
struct TrustSource(Box<u8>);
struct MathOnlySource(Box<u8>);
struct BitOnlySource(Box<u8>);

fn default_raw(source: DefaultSource) -> u8 {
    *source.0
}

fn checked_raw(source: CheckedSource) -> u8 {
    *source.0
}

fn trusted_raw(source: TrustSource) -> u8 {
    *source.0
}

fn math_only_raw(source: MathOnlySource) -> u8 {
    *source.0
}

fn bit_only_raw(source: BitOnlySource) -> u8 {
    *source.0
}

#[typekin::integral(
    friends = [
        default_raw(DefaultSource) -> Make,
        checked_raw(CheckedSource) -> [Make],
        trusted_raw(TrustSource) -> [Math, unsafe],
        math_only_raw(MathOnlySource),
        bit_only_raw(BitOnlySource) -> [Bit, unsafe],
        self(u8) -> Rel,
    ],
    valid = is_even,
    domain = 2..=10,
)]
struct Number(u8);

#[typekin::integral(
    friends = [
        self(TrustSelfNumber) -> unsafe,
        self(u8) -> unsafe,
    ],
    valid = is_even,
    domain = 2..=10,
)]
struct TrustSelfNumber(u8);

// ==============================================================================

#[typekin::integral(with = display)]
struct NumWithDisplay(i32);

#[typekin::integral(
    with = display,
    without = fn_conv_raw,
    get_raw = Self::decoded,
)]
struct EncodedWithDisplay(i32);

impl EncodedWithDisplay {
    const fn decoded(self) -> i32 {
        self.0 - 100
    }
}

// ==============================================================================

#[typekin::integral(without = [clone, copy, repr])]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct IntegralWithCallerProvidedRepr(u8);

#[derive(Debug, Eq, PartialEq)]
struct OperatorOutput(u8);

static OPERATOR_OUTPUT_CALLS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);
static CHECKED_OUTPUT_CALLS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

#[typekin::integral(
    konst = none,
    out_type = OutputSource::into_output(_) -> OperatorOutput,
)]
struct OutputSource(u8);

impl OutputSource {
    fn into_output(self) -> OperatorOutput {
        OPERATOR_OUTPUT_CALLS
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return OperatorOutput(self.raw());
    }
}

#[typekin::integral(
    konst = nightly,
    out_type = ConstOutputSource::into_output(_) -> OperatorOutput,
)]
struct ConstOutputSource(u8);

impl ConstOutputSource {
    const fn into_output(self) -> OperatorOutput {
        return OperatorOutput(self.raw());
    }
}

const CONST_OPERATOR_OUTPUT: OperatorOutput =
    ConstOutputSource::make(2) + ConstOutputSource::make(3);

const fn is_at_most_ten(value: u8) -> bool {
    return value <= 10;
}

#[typekin::integral(
    konst = none,
    valid = is_at_most_ten,
    out_type = CheckedOutputSource::into_output(_) -> OperatorOutput,
)]
struct CheckedOutputSource(u8);

impl CheckedOutputSource {
    fn into_output(self) -> OperatorOutput {
        CHECKED_OUTPUT_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return OperatorOutput(self.raw());
    }
}

static NON_ZERO_OUTPUT_CALLS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

#[typekin::integral(
    konst = none,
    with = non_zero,
    out_type = NonZeroOutputSource::into_output(_) -> OperatorOutput,
)]
struct NonZeroOutputSource(u8);

impl NonZeroOutputSource {
    fn into_output(self) -> OperatorOutput {
        NON_ZERO_OUTPUT_CALLS
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return OperatorOutput(self.raw());
    }
}

#[derive(Debug, Eq, PartialEq)]
struct BoundedOutput(u8);

impl BoundedOutput {
    fn make(value: u8) -> Self {
        assert!(value <= 5);
        return Self(value);
    }
}

#[typekin::integral(
    konst = none,
    out_type = TargetCheckedSource::into_output(_) -> BoundedOutput,
)]
struct TargetCheckedSource(u8);

impl TargetCheckedSource {
    fn into_output(self) -> BoundedOutput {
        return BoundedOutput::make(self.raw());
    }
}

#[typekin::integral(
    konst = none,
    out_type = Self(_) -> Self,
)]
struct ExplicitSelfOutput(u8);

#[typekin::integral(
    konst = none,
    profile = circular {},
)]
struct CircularSequence(u8);

#[typekin::integral(
    konst = none,
    profile = circular {},
)]
struct CircularSigned(i8);

#[typekin::integral(
    konst = none,
    profile = circular {
        with = [math_mul, math_div, math_rem, math_shl, math_shr],
    },
)]
struct CircularExtended(u8);

#[typekin::integral(
    konst = none,
    profile = sequential {},
)]
struct SequentialCounter(u8);

#[typekin::integral(
    konst = none,
    profile = sequential {
        with = [math_mul, math_div, math_rem, math_shl, math_shr],
    },
)]
struct SequentialExtended(u8);

#[typekin::integral(
    konst = none,
    profile = [
        sequential {
            with = math_mul,
        },
        circular {
            with = math_mul,
            precedes = false,
        },
    ],
)]
struct LayeredProfiles(u8);

#[derive(Debug, Eq, PartialEq)]
pub struct FacetResult(u8);

#[derive(Copy, Clone)]
pub struct FacetOffset(u8);

#[derive(Copy, Clone)]
pub struct FacetChecked(u8);

#[derive(Copy, Clone)]
pub struct FacetBits(u8);

fn facet_offset_raw(value: FacetOffset) -> u8 {
    return value.0;
}

fn facet_checked_raw(value: FacetChecked) -> u8 {
    return value.0;
}

fn facet_bits_raw(value: FacetBits) -> u8 {
    return value.0;
}

#[typekin::integral(
    konst = none,
    profile = circular {},
    out_type = FacetPoint::into_result(_) -> FacetResult,
    facets = [
        circular {
            friends = facet_offset_raw(FacetOffset) -> Math,
            out_type = Self,
        },
        sequential {
            friends = facet_checked_raw(FacetChecked) -> Math,
            without = math_sub,
            out_type = Self,
        },
        int {
            friends = facet_bits_raw(FacetBits) -> Bit,
            with = math_xor,
            out_type = Self,
        },
    ],
)]
pub struct FacetPoint(u8);

impl FacetPoint {
    fn into_result(self) -> FacetResult {
        return FacetResult(self.raw());
    }
}

#[derive(Copy, Clone)]
struct FacetConstOffset(u8);

const fn facet_const_offset_raw(value: FacetConstOffset) -> u8 {
    return value.0;
}

#[typekin::integral(
    konst = nightly,
    facets = [
        int {
            friends = facet_const_offset_raw(FacetConstOffset) -> Math,
            without = math_sub,
        },
    ],
)]
struct FacetConstPoint(u8);

const FACET_CONST_RESULT: FacetConstPoint =
    FacetConstPoint::make(2) + FacetConstOffset(3);

#[derive(Debug, Eq, PartialEq)]
struct FacetValidatedResult(u8);

static FACET_OUTPUTS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

const fn facet_valid(value: u8) -> bool {
    return value != 5;
}

#[typekin::integral(
    konst = none,
    valid = facet_valid,
    out_type = FacetValidated::into_result(_) -> FacetValidatedResult,
    facets = [],
)]
struct FacetValidated(u8);

impl FacetValidated {
    fn into_result(self) -> FacetValidatedResult {
        FACET_OUTPUTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return FacetValidatedResult(self.raw());
    }
}

// ==============================================================================

#[cfg(test)]
#[cfg(feature = "fast_tests")]
mod tests {
    use super::*;
    use core::cmp::Ordering;
    use serde_test::{
        Configure,
        Token,
        assert_de_tokens_error,
        assert_tokens,
    };

    #[test]
    fn facets_dispatch_by_rhs_with_independent_policies_and_outputs() {
        let wrapped = FacetPoint::make(u8::MAX) + FacetOffset(1);
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
    fn facets_preserve_constness_and_validate_before_output_conversion() {
        assert_eq!(FACET_CONST_RESULT.raw(), 5);

        let lhs = FacetValidated::try_make(2).unwrap();
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
    fn trusted_const_construction_bypasses_callback_and_range_validation() {
        assert_eq!(TRUSTED_ODD_NUMBER.raw(), 3);
        assert_eq!(TrustEven::of(12u8).raw(), 12);
        assert_eq!(TrustEven::try_make(3), Err(3));
        assert_eq!(TrustEven::try_make(12), Err(12));
    }

    #[test]
    fn trusted_construction_cannot_bypass_non_zero_storage() {
        assert_panics(|| TrustEven::of(0u8));
    }

    #[test]
    fn trusted_const_friend_operations_still_validate() {
        assert_panics(|| {
            let _ = TrustEven::try_make(4).unwrap() + 1u8;
        });
        assert_panics(|| {
            let _ = TrustEven::try_make(4).unwrap() | 1u8;
        });
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

        assert_eq!(mapped.0, 42);
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
        let raw: &u32 = value.as_ref();
        assert_eq!(*raw, 7);
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
        assert_eq!(AnyU8::try_make(0).map(AnyU8::raw), Ok(0));
        assert_eq!(AnyU8::try_make(u8::MAX).map(AnyU8::raw), Ok(u8::MAX));

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
    fn converts_into_usize() {
        let it = My32::make(23);
        let value: usize = it.into_usize();
        assert_eq!(value, 23);
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
        assert_eq!(MyI32::of(-1).try_into_u32(), Err(()));
    }

    #[test]
    fn implements_into_usize() {
        let it = My32::make(23);
        let value: usize = it.into();
        assert_eq!(value, 23);
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
    fn implements_into_i128() {
        let it = My32::make(23);
        let value: i128 = it.into();
        assert_eq!(value, 23);
    }

    #[test]
    fn implements_try_into_isize() {
        let it = My32::make(23);
        let value: Result<isize, ()> = it.try_into();
        assert_eq!(value, Ok(23));
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

    fn bitwise_ands_friend_value() {
        let it = My32::make(0b10111);
        assert_eq!((it & 0b10111).raw(), 0b10111);
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
        assert_eq!(value.0, 123);
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
        for raw in [3, 12] {
            let untrusted_target = Number::of(TrustSource(Box::new(raw)));
            assert_panics(|| Number::of(untrusted_target));

            let trusted_target = TrustSelfNumber::of(raw);
            assert_eq!(TrustSelfNumber::of(trusted_target).raw(), raw);
            assert_eq!(TrustSelfNumber::try_make(raw), Err(raw));
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
    fn trusted_owned_friend_bypasses_odd_value_construction() {
        let raw = 3;
        let source = TrustSource(Box::new(raw));
        assert_eq!(Number::of(source).raw(), raw);
        assert_eq!(Number::try_make(raw), Err(raw));

        assert_panics(|| {
            let _ = Number::try_make(4).unwrap() + TrustSource(Box::new(1));
        });
        assert_panics(|| {
            let _ = Number::try_make(4).unwrap() | TrustSource(Box::new(1));
        });
        assert_panics(|| {
            let _ = Number::try_make(6).unwrap() + Number::try_make(6).unwrap();
        });
    }

    #[test]
    fn trusted_owned_friend_bypasses_out_of_range_construction() {
        let raw = 12;
        let source = TrustSource(Box::new(raw));
        assert_eq!(Number::of(source).raw(), raw);
        assert_eq!(Number::try_make(raw), Err(raw));

        assert_panics(|| {
            let _ = Number::try_make(4).unwrap() + TrustSource(Box::new(1));
        });
        assert_panics(|| {
            let _ = Number::try_make(4).unwrap() | TrustSource(Box::new(1));
        });
        assert_panics(|| {
            let _ = Number::try_make(6).unwrap() + Number::try_make(6).unwrap();
        });
    }

    #[test]
    fn trusted_assignment_rejects_odd_rhs_without_mutating() {
        let rhs = 1;
        let mut number = Number::try_make(4).unwrap();
        assert_panics(move || number += TrustSource(Box::new(rhs)));
        assert_eq!(number.raw(), 4);

        assert_panics(move || number |= TrustSource(Box::new(rhs)));
        assert_eq!(number.raw(), 4);
    }

    #[test]
    fn trusted_assignment_rejects_out_of_range_rhs_without_mutating() {
        let rhs = 8;
        let mut number = Number::try_make(4).unwrap();
        assert_panics(move || number += TrustSource(Box::new(rhs)));
        assert_eq!(number.raw(), 4);

        assert_panics(move || number |= TrustSource(Box::new(rhs)));
        assert_eq!(number.raw(), 4);
    }

    #[test]
    fn trusted_operation_only_friends_consume_sources_and_validate_results() {
        let math_source = MathOnlySource(Box::new(2));
        assert_eq!((Number::try_make(4).unwrap() + math_source).raw(), 6);
        let bit_source = BitOnlySource(Box::new(2));
        assert_eq!((Number::try_make(4).unwrap() | bit_source).raw(), 6);

        for rhs in [1, 8] {
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
        assert_eq!(value.0, 123);
        assert_eq!(format!("{}", value), "23");
        assert_eq!(format!("{:+06}", value), "+00023");
    }

    #[test]
    fn traits_and_layout_can_be_supplied_by_the_caller() {
        accepts_copy::<IntegralWithCallerProvidedRepr>();
        assert_eq!(IntegralWithCallerProvidedRepr::make(1).raw(), 1);
    }
}
