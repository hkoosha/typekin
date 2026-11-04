use crate::*;

#[typekin::integral(konst = nightly)]
pub(crate) struct My32(u32);

#[typekin::integral(konst = nightly)]
pub(crate) struct MyAuto64(u64);

#[typekin::integral(konst = none)]
pub(crate) struct PointerUnsigned(usize);

#[typekin::integral(konst = none)]
pub(crate) struct PointerSigned(isize);

#[typekin::integral(konst = none)]
pub(crate) struct SignedNarrow(i16);

#[typekin::integral(
    konst = none,
    friend = _(u64) -> Make,
)]
pub(crate) struct ExplicitPointerFriend(usize);

#[cfg(target_pointer_width = "64")]
#[typekin::integral(
    konst = none,
    domain = isize::MIN..-2_147_483_648 | -isize::MAX..,
)]
pub(crate) struct PointerDomainIncludesMinimum(isize);

#[cfg(target_pointer_width = "64")]
#[typekin::integral(
    konst = none,
    domain = -isize::MAX.. | -2_147_483_648..=-2_147_483_648,
)]
pub(crate) struct PointerDomainExcludesMinimum(isize);

#[typekin::integral(hide_in = none, konst = none, with = konst_add)]
pub(crate) struct ConstAddOnly(u8);

pub(crate) const CONST_ADD_ONLY: ConstAddOnly =
    ConstAddOnly(20) + ConstAddOnly(3);

#[typekin::integral(
    hide_in = none,
    konst = none,
    with = [konst_add_assign, konst_or_assign],
    without = [konst_add, konst_or],
)]
pub(crate) struct ConstAssignmentOverride(u8);

pub(crate) const CONST_ADD_ASSIGN_ONLY: ConstAssignmentOverride = {
    let mut lhs = ConstAssignmentOverride(20);
    let rhs = ConstAssignmentOverride(3);
    lhs += rhs;
    lhs
};

pub(crate) const CONST_OR_ASSIGN_ONLY: ConstAssignmentOverride = {
    let mut lhs = ConstAssignmentOverride(0b1000);
    let rhs = ConstAssignmentOverride(0b0011);
    lhs |= rhs;
    lhs
};

#[typekin::integral(
    konst = none,
    without = [math_add, math_or],
)]
pub(crate) struct AssignmentWithoutBinary(u8);

#[typekin::integral(hide_in = none, konst = nightly)]
pub(crate) struct ConstMap(u8);

pub(crate) const CONST_MAPPED: ConstMap =
    ConstMap(20).map(const |value| value + 3);

pub(crate) const fn accepts_final_mapping(value: u8) -> bool {
    return value == 0 || value >= 3;
}

#[typekin::integral(valid = accepts_final_mapping)]
pub(crate) struct Mapped(u8);

#[typekin::integral(konst = none)]
pub(crate) struct PlainMapped(u8);

#[typekin::integral]
pub(crate) struct FullyGenerated(u8);

#[typekin::integral(konst = none, with = range)]
pub(crate) struct RangeValue(u8);

#[typekin::integral(konst = nightly)]
pub(crate) struct FullyGeneratedConst(u8);

pub(crate) const FULLY_GENERATED_CLONE: FullyGeneratedConst =
    FullyGeneratedConst::make(23).clone();

#[typekin::integral(without = [clone, copy, repr])]
#[derive(Copy, Clone)]
pub(crate) struct ManuallyDerivedIntegral(u8);

#[typekin::integral(
    hide_in = none,
    without = fn_conv_raw,
    get_raw = Self::decoded,
)]
pub(crate) struct AccessedMap(u8);

pub(crate) const fn accessed_map_storage(value: AccessedMap) -> u8 {
    return value.0;
}

impl AccessedMap {
    const fn decoded(self) -> u8 {
        return self.0 - 100;
    }
}

#[typekin::integral(konst = nightly, with = display)]
pub(crate) struct MyI32(i32);

#[typekin::integral(
    hide_in = none,
    konst = nightly,
    with = display,
    without = fn_conv_raw,
    get_raw = Self::decoded,
)]
pub(crate) struct EncodedDisplay(i32);

impl EncodedDisplay {
    const fn decoded(self) -> i32 {
        self.0 - 100
    }
}

pub(crate) const CONST_NUMBER: My32 = match My32::try_make(23) {
    Ok(number) => number,
    Err(_) => panic!("valid number rejected"),
};

pub(crate) const fn is_even(value: u8) -> bool {
    return value % 2 == 0;
}

pub(crate) const fn is_not_fifty(value: u8) -> bool {
    return value != 50;
}

#[typekin::integral(
    valid = is_even,
    with = serde,
)]
pub(crate) struct SerdeIntegral(u8);

pub(crate) const fn accepts_ranged(_: u8) -> bool {
    return true;
}

#[typekin::integral(
    konst = nightly,
    friend = self(u8) -> [Math, Rel],
    valid = [is_even, is_not_fifty],
    domain = 2..=100,
)]
pub(crate) struct Even(u8);

#[typekin::integral(
    konst = nightly,
    friend = [self(u8) -> [Math, Rel]],
    domain = 2..=4 | 8..10,
    valid = accepts_ranged,
)]
pub(crate) struct Ranged(u8);

#[typekin::integral(konst = none, with = non_zero)]
pub(crate) struct RequiredNonZero(u8);

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct NonZeroUsize(usize);

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct NonZeroIsize(isize);

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct NonZeroU8(u8);

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct NonZeroU16(u16);

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct NonZeroU32(u32);

#[typekin::integral(
    hide_in = none,
    konst = none,
    domain = 1..,
    profile = wrapping {},
    without = fn_conv_raw,
    get_raw = Self::value,
)]
pub(crate) struct CustomStoredRaw(u32);

impl CustomStoredRaw {
    pub(crate) const fn value(self) -> u32 {
        return self.0.0.get();
    }
}

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct NonZeroU64(u64);

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct NonZeroU128(u128);

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct NonZeroI8(i8);

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct NonZeroI16(i16);

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct NonZeroI32(i32);

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct NonZeroI64(i64);

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct NonZeroI128(i128);

#[typekin::integral(
    konst = none,
    domain = 0..=10,
    with = non_zero,
)]
pub(crate) struct DomainRequiredNonZero(u8);

#[typekin::integral(konst = none, domain = 1..)]
pub(crate) struct ExplicitlyPrivateNonZero(pub(self) u8);

#[typekin::integral(
    konst = none,
    domain = 1..,
    with = [display, serde],
)]
pub(crate) struct NonZeroSerde(u8);

#[typekin::integral(
    konst = none,
    domain = 1..=10,
    without = non_zero,
)]
pub(crate) struct PrimitivePositive(u8);

#[typekin::integral(konst = nightly, domain = ..5)]
pub(crate) struct BelowFive(u8);

#[typekin::integral(konst = nightly, domain = ..=5)]
pub(crate) struct AtMostFive(u8);

#[typekin::integral(konst = nightly, domain = -2..)]
pub(crate) struct FromNegativeTwo(i8);

pub(crate) const fn is_even_i8(value: i8) -> bool {
    return value % 2 == 0;
}

#[typekin::integral(
    konst = nightly,
    valid = is_even_i8,
    domain = ..=-6 | 4..,
)]
pub(crate) struct SplitEven(i8);

#[typekin::integral(
    konst = nightly,
    valid = is_even,
    without = fn_try_make,
)]
pub(crate) struct ParsedEven(u8);

#[typekin::integral(
    konst = nightly,
    without = core_int,
)]
pub(crate) struct CoreDisabled(u8);

#[typekin::integral(
    konst = nightly,
    without = [
        self_friend_math_ops,
        self_friend_math_bit,
    ],
)]
pub(crate) struct SelfOperations(u8);

pub(crate) const RANGED_NUMBER: Ranged = match Ranged::try_make(3) {
    Ok(number) => number,
    Err(_) => panic!("valid ranged number rejected"),
};

#[typekin::integral(
    hide_in = none,
    konst = nightly,
    friend = self(u16) -> [Math, Rel],
    get_raw = Self::value
)]
pub(crate) struct CustomRaw(u16);

#[typekin::integral(konst = nightly)]
pub(crate) struct Wide(u128);

#[typekin::integral(
    konst = nightly,
    friend = self(u8) -> [Math, unsafe],
    valid = is_even,
    domain = 2..=10,
)]
pub(crate) struct BlindTrustEven(u8);

pub(crate) const BLIND_TRUST_ODD_NUMBER: BlindTrustEven =
    BlindTrustEven::of(3u8);

#[typekin::integral(
    hide_in = none,
    konst = none,
    domain = 1..,
    profile = wrapping {},
    friend = self(u8) -> unsafe,
)]
pub(crate) struct WrappingBlindTrust(u8);

pub(crate) const fn wrapping_blind_trust_storage(
    value: WrappingBlindTrust
) -> core::num::Wrapping<core::num::NonZeroU8> {
    return value.0;
}

#[typekin::integral(
    konst = none,
    domain = 1..=100,
    with = with_blind_trust,
)]
pub(crate) struct BlindTrustOnRequest(u8);

pub(crate) fn blind_trust_on_request(raw: u8) -> BlindTrustOnRequest {
    return BlindTrustOnRequest::_blind_trust(raw);
}

/// Deliberately not `const`: `konst = none` must leave every validating path
/// non-const, so this validator compiles.
pub(crate) fn is_odd_runtime(raw: u8) -> bool {
    return raw % 2 == 1;
}

#[typekin::integral(konst = none, valid = is_odd_runtime)]
pub(crate) struct OddAtRuntime(u8);

// ==============================================================================

pub(crate) struct DefaultSource(pub(crate) Box<u8>);
pub(crate) struct CheckedSource(pub(crate) Box<u8>);
pub(crate) struct BlindTrustSource(pub(crate) Box<u8>);
pub(crate) struct MathOnlySource(pub(crate) Box<u8>);
pub(crate) struct BitOnlySource(pub(crate) Box<u8>);

pub(crate) fn default_raw(source: DefaultSource) -> u8 {
    *source.0
}

pub(crate) fn checked_raw(source: CheckedSource) -> u8 {
    *source.0
}

pub(crate) fn blind_trust_raw(source: BlindTrustSource) -> u8 {
    *source.0
}

pub(crate) fn math_only_raw(source: MathOnlySource) -> u8 {
    *source.0
}

pub(crate) fn bit_only_raw(source: BitOnlySource) -> u8 {
    *source.0
}

#[typekin::integral(
    friend = [
        default_raw(DefaultSource) -> Make,
        checked_raw(CheckedSource) -> [Make],
        blind_trust_raw(BlindTrustSource) -> [Math, unsafe],
        math_only_raw(MathOnlySource),
        bit_only_raw(BitOnlySource) -> [Bit, unsafe],
        self(u8) -> Rel,
    ],
    valid = is_even,
    domain = 2..=10,
)]
pub(crate) struct Number(u8);

#[typekin::integral(
    friend = [
        self(BlindTrustSelfNumber) -> unsafe,
        self(u8) -> unsafe,
    ],
    valid = is_even,
    domain = 2..=10,
)]
pub(crate) struct BlindTrustSelfNumber(u8);

// ==============================================================================

#[typekin::integral(with = display)]
pub(crate) struct NumWithDisplay(i32);

#[typekin::integral(
    hide_in = none,
    with = display,
    without = fn_conv_raw,
    get_raw = Self::decoded,
)]
pub(crate) struct EncodedWithDisplay(i32);

pub(crate) const fn encoded_with_display_storage(
    value: EncodedWithDisplay
) -> i32 {
    return value.0;
}

impl EncodedWithDisplay {
    const fn decoded(self) -> i32 {
        self.0 - 100
    }
}

// ==============================================================================

#[typekin::integral(without = [clone, copy, repr])]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub(crate) struct IntegralWithCallerProvidedRepr(u8);

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct OperatorOutput(pub(crate) u8);

pub(crate) static OPERATOR_OUTPUT_CALLS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);
pub(crate) static CHECKED_OUTPUT_CALLS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

#[typekin::integral(
    konst = none,
    produce = OutputSource::into_output(_) -> OperatorOutput,
)]
pub(crate) struct OutputSource(u8);

impl OutputSource {
    fn into_output(self) -> OperatorOutput {
        OPERATOR_OUTPUT_CALLS
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return OperatorOutput(self.raw());
    }
}

#[typekin::integral(
    konst = nightly,
    produce = ConstOutputSource::into_output(_) -> OperatorOutput,
)]
pub(crate) struct ConstOutputSource(u8);

impl ConstOutputSource {
    const fn into_output(self) -> OperatorOutput {
        return OperatorOutput(self.raw());
    }
}

pub(crate) const CONST_OPERATOR_OUTPUT: OperatorOutput =
    ConstOutputSource::make(2) + ConstOutputSource::make(3);

pub(crate) const fn is_at_most_ten(value: u8) -> bool {
    return value <= 10;
}

#[typekin::integral(
    konst = none,
    valid = is_at_most_ten,
    produce = CheckedOutputSource::into_output(_) -> OperatorOutput,
)]
pub(crate) struct CheckedOutputSource(u8);

impl CheckedOutputSource {
    fn into_output(self) -> OperatorOutput {
        CHECKED_OUTPUT_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return OperatorOutput(self.raw());
    }
}

pub(crate) static NON_ZERO_OUTPUT_CALLS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

#[typekin::integral(
    konst = none,
    with = non_zero,
    produce = NonZeroOutputSource::into_output(_) -> OperatorOutput,
)]
pub(crate) struct NonZeroOutputSource(u8);

impl NonZeroOutputSource {
    fn into_output(self) -> OperatorOutput {
        NON_ZERO_OUTPUT_CALLS
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return OperatorOutput(self.raw());
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct BoundedOutput(u8);

impl BoundedOutput {
    fn make(value: u8) -> Self {
        assert!(value <= 5);
        return Self(value);
    }
}

#[typekin::integral(
    konst = none,
    produce = TargetCheckedSource::into_output(_) -> BoundedOutput,
)]
pub(crate) struct TargetCheckedSource(u8);

impl TargetCheckedSource {
    fn into_output(self) -> BoundedOutput {
        return BoundedOutput::make(self.raw());
    }
}

#[typekin::int(
    konst = none,
    produce = Self(_) -> Self,
)]
pub(crate) struct ExplicitSelfOutput(u8);

#[typekin::circular(konst = none)]
pub(crate) struct CircularSequence(u8);

#[typekin::circular(konst = none)]
pub(crate) struct CircularSigned(i8);

#[typekin::delta(konst = none)]
pub(crate) struct DeltaSequence(u8);

#[typekin::circular(konst = none, precedes = false)]
pub(crate) struct CircularWithoutPrecedes(u8);

#[typekin::circular(
    konst = none,
    with = [math_mul, math_div, math_rem, math_shl, math_shr],
)]
pub(crate) struct CircularExtended(u8);

#[typekin::wrapping(hide_in = none, konst = none)]
pub(crate) struct WrappingSequence(u8);

pub(crate) const fn wrapping_sequence_storage(
    value: WrappingSequence
) -> core::num::Wrapping<u8> {
    return value.0;
}

#[typekin::integral(
    konst = none,
    profile = wrapping {},
    with = range,
)]
pub(crate) struct WrappingRangeValue(u8);

#[typekin::integral(
    hide_in = none,
    konst = none,
    profile = wrapping {},
    with = non_zero,
)]
pub(crate) struct WrappingNonZero(u8);

pub(crate) const fn wrapping_non_zero_storage(
    value: WrappingNonZero
) -> core::num::Wrapping<core::num::NonZeroU8> {
    return value.0;
}

#[typekin::integral(
    konst = nightly,
    profile = wrapping {},
)]
pub(crate) struct ConstWrappingSequence(u8);

pub(crate) const CONST_WRAPPING_RESULT: ConstWrappingSequence =
    ConstWrappingSequence::make(u8::MAX) + ConstWrappingSequence::make(1);

#[typekin::sequential(konst = none)]
pub(crate) struct SequentialCounter(u8);

#[typekin::integral(
    konst = none,
    profile = sequential {
        with = [math_mul, math_div, math_rem, math_shl, math_shr],
    },
)]
pub(crate) struct SequentialExtended(u8);

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
pub(crate) struct LayeredProfiles(u8);

#[typekin::point(
    konst = none,
    produce = Self::into_difference(_) -> i16,
)]
pub(crate) struct LinearPoint(i16);

impl LinearPoint {
    fn into_difference(self) -> i16 {
        return self.raw();
    }
}

#[typekin::integral(
    konst = none,
    profile = [sequential, point],
    produce = Self::into_difference(_) -> u8,
)]
pub(crate) struct CheckedPoint(u8);

impl CheckedPoint {
    fn into_difference(self) -> u8 {
        return self.raw();
    }
}

#[typekin::integral(
    hide_in = none,
    konst = none,
    profile = [wrapping, point],
    produce = Self::into_difference(_) -> u8,
)]
pub(crate) struct WrappingPoint(u8);

pub(crate) const fn wrapping_point_storage(
    value: WrappingPoint
) -> core::num::Wrapping<u8> {
    return value.0;
}

impl WrappingPoint {
    fn into_difference(self) -> u8 {
        return self.raw();
    }
}

#[typekin::integral(
    konst = none,
    profile = [circular, point],
    produce = Self::into_difference(_) -> u8,
    facet = point {
        with = math_add,
        friend = facet_offset_raw(FacetOffset) -> Math,
        produce = Self,
    },
)]
pub struct SurfaceFacetPoint(u8);

impl SurfaceFacetPoint {
    fn into_difference(self) -> u8 {
        return self.raw();
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct FacetResult(pub(crate) u8);

#[derive(Copy, Clone)]
pub struct FacetOffset(pub(crate) u8);

#[derive(Copy, Clone)]
pub struct FacetChecked(pub(crate) u8);

#[derive(Copy, Clone)]
pub struct FacetBits(pub(crate) u8);

#[derive(Copy, Clone)]
pub struct FacetWrapping(pub(crate) u8);

pub(crate) fn facet_offset_raw(value: FacetOffset) -> u8 {
    return value.0;
}

pub(crate) fn facet_checked_raw(value: FacetChecked) -> u8 {
    return value.0;
}

pub(crate) fn facet_bits_raw(value: FacetBits) -> u8 {
    return value.0;
}

pub(crate) fn facet_wrapping_raw(value: FacetWrapping) -> u8 {
    return value.0;
}

#[typekin::integral(
    hide_in = none,
    konst = none,
    profile = circular {},
    produce = FacetPoint::into_result(_) -> FacetResult,
    facet = [
        circular {
            friend = facet_offset_raw(FacetOffset) -> Math,
            produce = Self,
        },
        sequential {
            friend = facet_checked_raw(FacetChecked) -> Math,
            without = math_sub,
            produce = Self,
        },
        int {
            friend = facet_bits_raw(FacetBits) -> Bit,
            with = math_xor,
            produce = Self,
        },
        wrapping {
            friend = facet_wrapping_raw(FacetWrapping) -> Math,
            produce = Self,
        },
    ],
)]
pub struct FacetPoint(u8);

pub(crate) const fn facet_point_storage(
    value: FacetPoint
) -> core::num::Wrapping<u8> {
    return value.0;
}

impl FacetPoint {
    fn into_result(self) -> FacetResult {
        return FacetResult(self.raw());
    }
}

#[derive(Copy, Clone)]
pub(crate) struct FacetConstOffset(u8);

pub(crate) const fn facet_const_offset_raw(value: FacetConstOffset) -> u8 {
    return value.0;
}

#[typekin::integral(
    hide_in = none,
    konst = nightly,
    facet = [
        wrapping {
            friend = facet_const_offset_raw(FacetConstOffset) -> Math,
            without = math_sub,
        },
    ],
)]
pub(crate) struct FacetConstPoint(u8);

pub(crate) const fn facet_const_point_storage(
    value: FacetConstPoint
) -> core::num::Wrapping<u8> {
    return value.0;
}

pub(crate) const FACET_CONST_RESULT: FacetConstPoint =
    FacetConstPoint::make(2) + FacetConstOffset(3);

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct FacetValidatedResult(pub(crate) u8);

pub(crate) static FACET_OUTPUTS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

pub(crate) const fn facet_valid(value: u8) -> bool {
    return value != 5;
}

#[typekin::integral(
    hide_in = none,
    konst = none,
    valid = facet_valid,
    with = non_zero,
    produce = FacetValidated::into_result(_) -> FacetValidatedResult,
    facet = [
        wrapping {
            with = self_math,
        },
    ],
)]
pub(crate) struct FacetValidated(u8);

pub(crate) const fn facet_validated_storage(
    value: FacetValidated
) -> core::num::Wrapping<core::num::NonZeroU8> {
    return value.0;
}

impl FacetValidated {
    fn into_result(self) -> FacetValidatedResult {
        FACET_OUTPUTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        return FacetValidatedResult(self.raw());
    }
}

#[typekin::integral(konst = none, valid = is_even)]
pub(crate) struct ValidatedWithoutMake(u8);

pub(crate) struct MakeSource(pub(crate) u8);

pub(crate) fn make_source_raw(source: MakeSource) -> u8 {
    return source.0;
}

#[typekin::integral(
    konst = none,
    friend = make_source_raw(MakeSource) -> Make,
)]
pub(crate) struct MakeFriendNumber(u8);

pub(crate) struct BitSource(pub(crate) u8);

pub(crate) fn bit_source_raw(source: BitSource) -> u8 {
    return source.0;
}

#[typekin::integral(
    konst = none,
    friend = bit_source_raw(BitSource) -> Bit,
)]
pub(crate) struct BitFriendNumber(u8);

#[typekin::integral(
    konst = none,
    hide_in = none,
    without = fn_conv_raw,
    get_raw = Self::value,
)]
pub(crate) struct WithoutRaw(u8);

impl WithoutRaw {
    pub(crate) const fn value(self) -> u8 {
        return self.0;
    }
}

#[typekin::integral(konst = none, without = self_math)]
pub(crate) struct WithoutSelfMath(u8);

#[typekin::integral(konst = none, without = self_bit)]
pub(crate) struct WithoutSelfBit(u8);

pub(crate) struct XorOutput;

#[typekin::integral(
    konst = none,
    produce = Self::into_output(_) -> XorOutput,
)]
pub(crate) struct OutputWithoutXorAssign(u8);

impl OutputWithoutXorAssign {
    fn into_output(self) -> XorOutput {
        return XorOutput;
    }
}

#[typekin::integral(konst = none, profile = point {})]
pub(crate) struct PointOnly(u8);

#[typekin::integral(konst = none, without = from_str)]
pub(crate) struct WithoutFromStr(u8);

#[typekin::integral(konst = none)]
pub(crate) struct DefaultCoreMethods(u8);

#[typekin::integral(
    konst = stable,
    domain = isize::MIN..0 | 1..=isize::MAX,
)]
pub(crate) struct PointerNonZeroSigned(isize);

#[forbid(unsafe_code)]
pub(crate) mod forbid_unsafe {
    #[typekin::integral(domain = 1..)]
    pub struct Positive(u32);
}

// ==============================================================================

#[typekin::delta(konst = none, with = math_mul)]
pub(crate) struct DeltaExtended(u8);

#[typekin::integral(konst = none, profile = delta { with = math_mul })]
pub(crate) struct DeltaBodyExtended(u8);

#[typekin::integral(konst = none, profile = int)]
pub(crate) struct IntProfile(u8);

#[typekin::integral(konst = none, profile = [])]
pub(crate) struct EmptyProfileList(u8);

#[typekin::integral(
    konst = none,
    profile = circular,
    without = math_add,
)]
pub(crate) struct BareCircularWithoutAdd(u8);

#[typekin::integral(
    konst = none,
    profile = [circular, point { with = math_add }],
)]
pub(crate) struct BareMixedProfiles(u8);

#[typekin::integral(
    konst = none,
    facet = [circular],
    without = math_add,
)]
pub(crate) struct BareFacetWithoutAdd(u8);

#[typekin::integral(
    konst = none,
    facet = circular,
    without = math_add,
)]
pub(crate) struct SingleFacetWithoutAdd(u8);

#[typekin::integral(
    konst = none,
    profile = circular {},
    facet = [
        int,
        sequential {
            with = math_mul,
            friend = facet_offset_raw(FacetOffset) -> Math,
        },
        circular,
    ],
    without = math_add,
)]
pub struct BareFacetDispatch(u8);

#[typekin::integral(konst = none, profile = [circular {}, point {}])]
pub(crate) struct CircularPoint(u8);

#[typekin::integral(konst = none, profile = [delta, point])]
pub(crate) struct DeltaPoint(u8);

#[typekin::integral(
    konst = none,
    profile = point { with = assign_add },
    with = math_add,
)]
pub(crate) struct PointWithAddition(u8);

#[typekin::integral(
    konst = none,
    profile = [int { with = [math_add, assign_add] }, point {}],
)]
pub(crate) struct PointResetsAddition(u8);

#[typekin::integral(
    hide_in = none,
    konst = none,
    profile = [circular {}, wrapping {}],
)]
pub(crate) struct CircularThenWrapping(u8);

pub(crate) const fn circular_then_wrapping_storage(
    value: CircularThenWrapping
) -> core::num::Wrapping<u8> {
    return value.0;
}

#[typekin::integral(
    hide_in = none,
    konst = none,
    profile = [wrapping {}, circular {}],
)]
pub(crate) struct WrappingThenCircular(u8);

pub(crate) const fn wrapping_then_circular_storage(
    value: WrappingThenCircular
) -> u8 {
    return value.0;
}

#[typekin::integral(
    hide_in = none,
    konst = none,
    profile = wrapping {},
    facet = [
        int {
            friend = facet_offset_raw(FacetOffset) -> Math,
        },
    ],
)]
pub struct WrappingWithIntFacet(u8);

pub(crate) const fn wrapping_with_int_facet_storage(
    value: WrappingWithIntFacet
) -> core::num::Wrapping<u8> {
    return value.0;
}

#[typekin::integral(
    konst = none,
    with = [math_add],
    profile = circular {
        without = [math_add],
    },
)]
pub(crate) struct RootAddBeforeProfile(u8);

#[typekin::integral(
    konst = none,
    profile = circular {
        without = [math_add],
    },
    with = [math_add],
)]
pub(crate) struct ProfileBeforeRootAdd(u8);

#[typekin::integral(
    konst = none,
    profile = [
        sequential {
            without = math_add,
            with = [math_mul, bit_access],
        },
        circular {
            with = core_int,
            without = math_sub,
            precedes = false,
        },
    ],
)]
pub(crate) struct OrderedProfiles(u16);

#[typekin::integral(
    konst = none,
    profile = [
        int {
            without = math_add,
        },
        int {
            with = math_add,
        },
    ],
)]
pub(crate) struct LaterProfileAdds(u8);

// ==============================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct RootOutput(pub(crate) u8);

pub struct RootRhs(pub(crate) u8);

pub(crate) fn root_rhs_raw(value: RootRhs) -> u8 {
    return value.0;
}

#[typekin::integral(
    konst = none,
    profile = sequential {
        without = math_add,
    },
    with = math_add,
    friend = root_rhs_raw(RootRhs) -> Math,
    produce = Self::into_root(_) -> RootOutput,
    facet = [
        circular {
            friend = facet_offset_raw(FacetOffset) -> Math,
            with = [math_add, math_sub],
            produce = Self,
        },
        int {
            friend = facet_bits_raw(FacetBits) -> Bit,
            with = math_or,
            without = math_sub,
        },
    ],
)]
pub struct ExplicitFacets(u8);

impl ExplicitFacets {
    fn into_root(self) -> RootOutput {
        return RootOutput(self.raw());
    }
}

#[typekin::integral(
    konst = none,
    facet = int {
        friend = facet_checked_raw(FacetChecked)
    },
)]
pub struct DefaultFacetFriend(u8);

#[typekin::integral(
    konst = none,
    profile = circular,
    facet = sequential {
        with = math_add,
        friend = facet_offset_raw(FacetOffset) -> Math,
        produce = Self,
    },
    without = math_add,
    produce = Self::into_result(_) -> FacetResult,
)]
pub struct SingleConfiguredFacet(u8);

impl SingleConfiguredFacet {
    fn into_result(self) -> FacetResult {
        return FacetResult(self.raw());
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SlotSelfOutput(pub(crate) u8);

#[typekin::integral(
    konst = none,
    profile = circular {},
    friend = facet_offset_raw(FacetOffset) -> Math,
    produce = Self::into_result(_) -> FacetResult,
    facet = [
        sequential {
            friend = facet_offset_raw(FacetOffset) -> Math,
            without = math_sub,
            produce = Self,
        },
        circular {
            with = [self_math, math_sub],
            without = math_add,
            produce = Self::into_self_output(_) -> SlotSelfOutput,
        },
    ],
)]
pub struct SlotResolution(u8);

impl SlotResolution {
    fn into_result(self) -> FacetResult {
        return FacetResult(self.raw());
    }

    fn into_self_output(self) -> SlotSelfOutput {
        return SlotSelfOutput(self.raw());
    }
}

pub(crate) fn doubled_facet_offset_raw(value: FacetOffset) -> u8 {
    return value.0 * 2;
}

#[typekin::integral(
    konst = none,
    facet = [
        circular {
            friend = facet_offset_raw(FacetOffset) -> Math,
            without = math_sub,
        },
        circular {
            friend = doubled_facet_offset_raw(FacetOffset) -> Math,
            without = math_add,
        },
    ],
)]
pub struct DistinctFacetOperations(u8);

#[typekin::integral(
    konst = none,
    profile = circular {},
    friend = facet_offset_raw(FacetOffset) -> [Math, Rel],
    facet = [
        circular {
            friend = facet_wrapping_raw(FacetWrapping) -> Math,
        },
    ],
)]
pub struct FacetDispatchOperators(u8);

#[typekin::integral(
    konst = none,
    friend = facet_offset_raw(FacetOffset) -> [Math, Rel],
    facet = [],
)]
pub struct FacetedFriendship(u8);

// ==============================================================================

#[derive(Debug, Eq, PartialEq)]
pub struct GenericOffset<T>(pub(crate) T);

#[typekin::integral(
    konst = none,
    produce = crate::fx_integral::GenericProduce::into_offset(_)
        -> GenericOffset<u32>,
)]
pub struct GenericProduce(u8);

impl GenericProduce {
    fn into_offset(self) -> GenericOffset<u32> {
        return GenericOffset(self.raw().into());
    }
}

#[typekin::integral(
    hide_in = none,
    konst = none,
    error = (),
    with = [serde, with_const_map_fn],
)]
pub(crate) struct ConstMapFn(u8);

pub(crate) const CONST_MAP_FN: ConstMapFn =
    ConstMapFn(20).map(const |value| value + 3);

#[typekin::integral(konst = none, without = [serde, map])]
pub(crate) struct WithoutSerdeAndMap(u8);

#[typekin::integral(konst = none, without = display)]
pub(crate) struct WithoutDisplay(i32);

#[typekin::integral(konst = none, friend = [])]
pub(crate) struct EmptyFriendList(u8);

#[typekin::integral(
    konst = none,
    tree = [
        IntegralMode {
            IntegralActive,
        },
        IntegralFlow {
            IntegralForward,
            IntegralBackward,
        },
    ],
)]
pub(crate) struct BracketedTree(u8);

#[typekin::integral(
    hide_in = none,
    konst = none,
    profile = wrapping {},
    without = fn_or_panic,
)]
pub(crate) struct CustomOrPanicWrapping(u32);

impl CustomOrPanicWrapping {
    const fn _or_panic(it: u32) -> Self {
        if it > 100 {
            panic!("custom or_panic limit");
        }
        return Self(core::num::Wrapping(it));
    }
}

// ==============================================================================

#[typekin::integral(hide_in = none, konst = none, with = konst_math)]
pub(crate) struct KonstMathFamily(u8);

pub(crate) const KONST_MATH_SUM: KonstMathFamily =
    KonstMathFamily(20) + KonstMathFamily(3);

#[typekin::integral(
    hide_in = none,
    konst = none,
    with = konst_math,
    without = konst_add,
)]
pub(crate) struct KonstMathWithoutAdd(u8);

pub(crate) const KONST_MATH_DIFFERENCE: KonstMathWithoutAdd =
    KonstMathWithoutAdd(20) - KonstMathWithoutAdd(3);

#[typekin::integral(konst = nightly, without = konst_add)]
pub(crate) struct KonstNightlyWithoutAdd(u8);

pub(crate) const KONST_NIGHTLY_DIFFERENCE: KonstNightlyWithoutAdd =
    KonstNightlyWithoutAdd::make(20) - KonstNightlyWithoutAdd::make(3);

#[typekin::integral(hide_in = none, konst = none, with = konst_map)]
pub(crate) struct KonstMapOnly(u8);

pub(crate) const KONST_MAPPED: KonstMapOnly =
    KonstMapOnly(20).map(const |value| value + 3);

#[typekin::integral(
    hide_in = none,
    konst = none,
    profile = [
        int { with = [konst_add, konst_sub] },
        int { without = [konst_add, konst_sub] },
    ],
    with = konst_add,
)]
pub(crate) struct ProfileKonstOverride(u8);

pub(crate) const PROFILE_KONST_SUM: ProfileKonstOverride =
    ProfileKonstOverride(20) + ProfileKonstOverride(3);

#[typekin::integral(domain = 1..=100)]
pub(crate) struct DefaultKonst(u8);

pub(crate) const DEFAULT_KONST_ONES: u32 = match DefaultKonst::try_make(7) {
    Ok(value) => value.count_ones(),
    Err(_) => panic!("valid value rejected"),
};

#[typekin::integral(hide_in = none, with = konst_add)]
pub(crate) struct DefaultKonstWithAdd(u8);

pub(crate) const DEFAULT_KONST_SUM: DefaultKonstWithAdd =
    DefaultKonstWithAdd(20) + DefaultKonstWithAdd(3);

#[typekin::integral(konst = stable, domain = 1..)]
pub(crate) struct KonstStable(u8);

pub(crate) const KONST_STABLE: KonstStable = match KonstStable::try_make(7) {
    Ok(value) => value,
    Err(_) => panic!("valid value rejected"),
};

// ==============================================================================

pub struct QualifiedSource(pub(crate) u8);

pub fn qualified_raw(source: QualifiedSource) -> u8 {
    return source.0;
}

#[typekin::integral(
    konst = none,
    friend = crate::fx_integral::qualified_raw(
        crate::fx_integral::QualifiedSource
    ) -> [Make, unsafe],
    valid = is_even,
)]
pub(crate) struct QualifiedFriend(u8);

pub(crate) struct UnsafeSource(pub(crate) u8);

pub(crate) fn unsafe_raw(source: UnsafeSource) -> u8 {
    return source.0;
}

#[typekin::integral(
    konst = none,
    friend = unsafe_raw(UnsafeSource) -> unsafe,
    valid = is_even,
)]
pub(crate) struct UnsafeOnly(u8);

#[typekin::integral(
    konst = none,
    friend = unsafe_raw(UnsafeSource) -> [unsafe],
    valid = is_even,
)]
pub(crate) struct BracketedUnsafeOnly(u8);

#[typekin::integral(
    konst = none,
    friend = unsafe_raw(UnsafeSource) -> [Make, unsafe,],
    valid = is_even,
)]
pub(crate) struct TrailingCommaCapabilities(u8);

#[derive(Copy, Clone)]
pub(crate) struct RelSource(pub(crate) u8);

pub(crate) fn rel_raw(source: RelSource) -> u8 {
    return source.0;
}

#[typekin::integral(
    konst = none,
    friend = rel_raw(RelSource) -> [Bit, Rel, unsafe],
    valid = is_even,
)]
pub(crate) struct BitRelUnsafe(u8);

pub(crate) struct ImplicitSource(pub(crate) u8);

impl From<ImplicitSource> for u8 {
    fn from(source: ImplicitSource) -> Self {
        return source.0;
    }
}

#[typekin::integral(
    konst = none,
    friend = _(ImplicitSource) -> unsafe,
    valid = is_even,
)]
pub(crate) struct ImplicitUnsafe(u8);

#[typekin::integral(
    konst = none,
    friend = _(ImplicitSource,) -> [Make, unsafe],
    valid = is_even,
)]
pub(crate) struct ImplicitTrailingComma(u8);

#[typekin::integral(konst = none, friend = _(ImplicitSource))]
pub(crate) struct ImplicitDefault(u8);

#[typekin::integral(konst = none, friend = _(ImplicitSource) -> Math)]
pub(crate) struct ImplicitMath(u8);

#[typekin::integral(
    konst = none,
    friend = ::core::num::NonZeroU8::get(::core::num::NonZeroU8,) -> Make,
)]
pub(crate) struct AbsoluteFriendPaths(u8);

pub(crate) struct CapabilitySource(pub(crate) u8);

pub(crate) fn capability_raw(source: CapabilitySource) -> u8 {
    return source.0;
}

#[typekin::integral(
    konst = none,
    friend = [capability_raw(CapabilitySource) -> [Make, Bit]],
)]
pub(crate) struct MakeBitFriend(u8);

#[typekin::integral(
    konst = none,
    friend = [capability_raw(CapabilitySource) -> Math],
)]
pub(crate) struct ExplicitMathFriend(u8);

// ==============================================================================

pub(crate) const fn is_even_i32(value: i32) -> bool {
    return value % 2 == 0;
}

pub(crate) const fn is_not_minus_fifty(value: i32) -> bool {
    return value != -50;
}

#[typekin::integral(
    konst = none,
    valid = [is_even_i32, is_not_minus_fifty],
    domain = 1..=4 | 8..10 | ..=0,
)]
pub(crate) struct ValidationLists(i32);

#[typekin::integral(konst = none, domain = 1..=u32::MAX | u32::MAX..)]
pub(crate) struct ConstantBounds(u32);

#[typekin::integral(
    konst = none,
    domain = 10..=12 | 1..3 | 3..=5 | 6..10,
)]
pub(crate) struct UnorderedDomain(u32);

#[typekin::integral(konst = none, domain = ..0)]
pub(crate) struct BelowZero(i32);

#[typekin::integral(konst = none, domain = ..=0)]
pub(crate) struct AtMostZero(i32);

#[typekin::integral(konst = none, domain = -5..0)]
pub(crate) struct NegativeOpen(i32);

#[typekin::integral(konst = none, domain = -5..0 | 1..)]
pub(crate) struct SplitAroundZero(i32);

#[typekin::integral(konst = none, domain = -10..=-5 | -4..0)]
pub(crate) struct AdjacentNegative(i32);

#[typekin::integral(konst = none, domain = ..=-1 | 0..=5)]
pub(crate) struct UnionWithZero(i32);

#[typekin::integral(konst = none, domain = i32::MIN..0)]
pub(crate) struct FromSignedMinimum(i32);

#[typekin::integral(konst = none, domain = u32::MIN..=u32::MIN)]
pub(crate) struct UnsignedMinimumOnly(u32);

#[typekin::integral(konst = none, domain = -5..=5)]
pub(crate) struct PointerAroundZero(isize);

#[typekin::integral(konst = none, domain = usize::MIN..=usize::MIN)]
pub(crate) struct PointerMinimumOnly(usize);

pub(crate) const fn opaque(_: u32) -> bool {
    return true;
}

#[typekin::integral(konst = none, valid = opaque)]
pub(crate) struct CallbackOnly(u32);

#[typekin::integral(konst = none, domain = 2..10 | 100..123)]
pub(crate) struct GappedPositive(u32);

#[typekin::integral(
    konst = none,
    without = non_zero,
    profile = int {
        with = non_zero,
    },
)]
pub(crate) struct RootSuppressesProfileNonZero(u8);

// ==============================================================================

#[typekin::integral(domain = 1..)]
pub(crate) struct HTTPServer(u16);

#[typekin::integral(domain = 1..)]
pub(crate) struct MyURLValue(u16);

#[typekin::integral(domain = 1..)]
pub(crate) struct X509Certificate(u16);

// ==============================================================================

#[typekin::integral(
    konst = none,
    without = [fn_count_ones, fn_checked_add, fn_abs, fn_from_be_bytes],
)]
pub(crate) struct WithoutSomeIntFns(i32);

#[typekin::integral(konst = none, profile = int { without = fn_count_ones })]
pub(crate) struct ProfileWithoutCountOnes(u8);

#[typekin::integral(konst = none)]
pub(crate) struct r#Type(u8);

#[typekin::integral(
    konst = none,
    friend = [],
    without = [
        self_friend_make,
        self_friend_math_ops,
        self_friend_math_bit,
        self_friend_rel,
    ],
)]
pub(crate) struct NoPrimitiveMake(u16);

// ==============================================================================

pub(crate) fn has_niche<T>() -> bool {
    return size_of::<Option<T>>() == size_of::<T>();
}
