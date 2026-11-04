use crate::*;

#[typekin::bitflag]
#[repr(u8)]
pub(crate) enum r#Type {
    One = 0b01,
    Two = 0b10,
}

#[typekin::bitflag(
    with = display,
    friend = self(u8) -> Bit,
    integral = [
        friend = self(u8) -> [Math, Rel],
    ],
)]
#[repr(u8)]
pub enum Thingy {
    FirstThing = 0,
    ReadThing = 0b001,
    WritersBlock = 0b010,
    Execute = 0b100,
}

#[repr(u8)]
#[typekin::bitflag(
    suffix = "Holder",
    integral = [domain = 0..=7],
)]
pub enum Thingy1 {
    FirstThing = 0,
    ReadThing = 0b001,
    WritersBlock = 0b010,
    Execute = 0b100,
}

#[repr(u8)]
#[typekin::bitflag(value_name = "Thingies2", suffix = "")]
pub enum Thingy2 {
    FirstThing = 0,
    ReadThing = 0b001,
    WritersBlock = 0b010,
    Execute = 0b100,
}

#[typekin::bitflag(with = serde)]
#[repr(u8)]
pub(crate) enum AutoSerdeFlag {
    Read = 0b0001,
    Write = 0b0010,
}

#[typekin::bitflag(with = serde_with_name)]
#[repr(u8)]
pub(crate) enum NameSerdeFlag {
    Read = 0b0001,
    Write = 0b0010,
}

#[typekin::bitflag(with = serde_with_value)]
#[repr(u8)]
pub(crate) enum ValueSerdeFlag {
    Read = 0b0001,
    Write = 0b0010,
}

#[typekin::bitflag(
    with = serde,
    integral = [domain = 0..=7],
)]
#[repr(u8)]
pub(crate) enum ValidatedSerdeFlag {
    Read = 0b0001,
    Write = 0b0010,
}

#[typekin::bitflag(with = display)]
#[repr(u8)]
pub(crate) enum CompositeFlag {
    Read = 0b0001,
    Write = 0b0010,
    ReadWrite = 0b0011,
}

#[typekin::bitflag(repr = u8)]
pub(crate) enum FullyGeneratedFlag {
    Read = 0b0001,
    Write = 0b0010,
}

#[cfg(target_pointer_width = "64")]
#[typekin::bitflag(konst = none)]
#[repr(usize)]
pub(crate) enum PointerFlag {
    One = 1,
}

#[repr(u8)]
#[typekin::bitflag(konst = nightly)]
pub(crate) enum FullyGeneratedConstFlag {
    Read = 0b0001,
}

pub(crate) const FULLY_GENERATED_FLAG_CLONE: FullyGeneratedConstFlag =
    FullyGeneratedConstFlag::Read.clone();

#[derive(Copy, Clone)]
#[repr(u8)]
#[typekin::bitflag(without = [clone, copy, repr],)]
pub(crate) enum ManuallyDerivedFlag {
    Read = 0b0001,
}

pub(crate) const THINGY_READ_VALUE: ThingyValue = ThingyValue::ReadThing;

// =============================================================================

#[repr(u8)]
#[typekin::bitflag(konst = nightly)]
pub(crate) enum ConstFlag {
    Empty = 0,
    Read = 0b001,
    Write = 0b010,
    Execute = 0b100,
}

pub(crate) const RETAINED: ConstFlagValue = ConstFlag::from_bits_retain(0b1011);
pub(crate) const TRUNCATED: ConstFlagValue =
    ConstFlag::from_bits_truncate(0b1011);
pub(crate) const SET_ALGEBRA: ConstFlagValue = ConstFlagValue::Read
    .union(ConstFlagValue::Write)
    .toggled(ConstFlagValue::Execute);
pub(crate) const FROM_ENUM: ConstFlagValue = ConstFlag::Read.into_value();
pub(crate) const ENUM_NEGATED: ConstFlagValue = !ConstFlag::Read;
pub(crate) const VALUE_NEGATED: ConstFlagValue = !ConstFlagValue::Read;

// =============================================================================

#[typekin::bitflag(with = display)]
#[repr(u32)]
pub(crate) enum DisplayWide {
    Low = 1,
    High = 0x8000_0000,
}

#[typekin::bitflag(with = display)]
#[repr(i16)]
pub(crate) enum DisplaySigned {
    Positive = 1,
    Negative = -1,
}

// =============================================================================

#[repr(u8)]
#[typekin::bitflag(
    with = display,
    integral = [without = display],
)]
pub(crate) enum RootBeforeNested {
    Read = 1,
    Write = 2,
}

#[repr(u8)]
#[typekin::bitflag(
    integral = [without = display],
    with = display,
)]
pub(crate) enum NestedBeforeRoot {
    Read = 1,
    Write = 2,
}

#[repr(u8)]
#[typekin::bitflag(integral = [with = display])]
pub(crate) enum NestedOnly {
    Read = 1,
    Write = 2,
}

#[repr(u8)]
#[typekin::bitflag(
    without = display,
    integral = [with = display],
)]
pub(crate) enum DisabledRootBeforeNested {
    Read = 1,
    Write = 2,
}

#[repr(u8)]
#[typekin::bitflag(
    integral = [with = display],
    without = display,
)]
pub(crate) enum NestedBeforeDisabledRoot {
    Read = 1,
    Write = 2,
}

// =============================================================================

pub(crate) struct DefaultBits(pub(crate) u8);
pub(crate) struct CheckedBits(pub(crate) u8);
pub(crate) struct BlindTrustBits(pub(crate) u8);
pub(crate) struct BitOnlyBits(pub(crate) u8);
pub(crate) struct MathOnlyBits(pub(crate) u8);

pub(crate) const fn default_bits(source: DefaultBits) -> u8 {
    source.0
}

pub(crate) const fn checked_bits(source: CheckedBits) -> u8 {
    source.0
}

pub(crate) const fn blind_trust_bits(source: BlindTrustBits) -> u8 {
    source.0
}

pub(crate) const fn bit_only_bits(source: BitOnlyBits) -> u8 {
    source.0
}

pub(crate) const fn math_only_bits(source: MathOnlyBits) -> u8 {
    source.0
}

pub(crate) const fn is_even(bits: u8) -> bool {
    bits % 2 == 0
}

#[repr(u8)]
#[typekin::bitflag(
    friend = [
        blind_trust_bits(BlindTrustBits) -> Bit,
        bit_only_bits(BitOnlyBits) -> Bit,
    ],
    integral = [
        friend = [
            default_bits(DefaultBits) -> Make,
            checked_bits(CheckedBits) -> [Make],
            blind_trust_bits(BlindTrustBits) -> [Bit, unsafe],
            bit_only_bits(BitOnlyBits) -> [Bit, unsafe],
            math_only_bits(MathOnlyBits) -> [Math, unsafe],
        ],
        valid = is_even,
        domain = 0..=6,
    ],
)]
pub(crate) enum Flags {
    Empty = 0,
    Read = 2,
    Write = 4,
}

// =============================================================================

#[repr(u8)]
#[typekin::bitflag(konst = stable)]
pub(crate) enum StableFlag {
    Read = 0b01,
    Write = 0b10,
}

pub(crate) const STABLE_RETAINED: StableFlagValue =
    StableFlag::from_bits_retain(0b111);
pub(crate) const STABLE_UNION: StableFlagValue =
    StableFlagValue::Read.union(StableFlagValue::Write);

pub(crate) const DEFAULT_RETAINED: Thingies2 =
    Thingy2::from_bits_retain(0b1011);
pub(crate) const DEFAULT_UNION: Thingies2 =
    Thingies2::ReadThing.union(Thingies2::Execute);

#[repr(u8)]
#[typekin::bitflag(integral = [with = [konst_value]])]
pub(crate) enum ForwardedKonstFlag {
    Read = 0b01,
    Write = 0b10,
}

pub(crate) const FORWARDED_RETAINED: ForwardedKonstFlagValue =
    ForwardedKonstFlag::from_bits_retain(0b111);
pub(crate) const FORWARDED_UNION: ForwardedKonstFlagValue =
    ForwardedKonstFlagValue::Read.union(ForwardedKonstFlagValue::Write);

#[repr(u8)]
#[typekin::bitflag(
    konst = none,
    with = konst_value,
    integral = [with = konst_value],
)]
pub(crate) enum OverriddenKonstFlag {
    Read = 0b01,
    Write = 0b10,
}

pub(crate) const OVERRIDDEN_RETAINED: OverriddenKonstFlagValue =
    OverriddenKonstFlag::from_bits_retain(0b111);
pub(crate) const OVERRIDDEN_UNION: OverriddenKonstFlagValue =
    OverriddenKonstFlagValue::Read.union(OverriddenKonstFlagValue::Write);

// =============================================================================

#[repr(u8)]
#[typekin::bitflag(with = [clone, copy, hash, debug, repr])]
pub(crate) enum DerivedFlag {
    Read = 1,
}

#[derive(Copy, Clone)]
#[repr(u8)]
#[typekin::bitflag(without = [clone, copy, hash, debug, repr])]
pub(crate) enum UnderivedFlag {
    Read = 1,
}

#[repr(u8)]
#[typekin::bitflag(without = display)]
pub(crate) enum UndisplayedFlag {
    Read = 1,
}

#[repr(u8)]
#[typekin::bitflag(integral = [without = display])]
pub(crate) enum UndisplayedValueFlag {
    Read = 1,
}

#[repr(u8)]
#[typekin::bitflag(integral = [error = (), domain = 0..=3])]
pub(crate) enum UnitErrorFlag {
    Read = 1,
    Write = 2,
}

// =============================================================================

pub(crate) struct ValueUnsafeBits(pub(crate) u8);
pub(crate) struct MathBitBits(pub(crate) u8);

pub(crate) const fn value_unsafe_bits(source: ValueUnsafeBits) -> u8 {
    return source.0;
}

pub(crate) const fn math_bit_bits(source: MathBitBits) -> u8 {
    return source.0;
}

#[repr(u8)]
#[typekin::bitflag(
    integral = [
        friend = value_unsafe_bits(ValueUnsafeBits) -> unsafe,
        domain = 0..=3,
    ],
)]
pub(crate) enum ValueUnsafeFlag {
    Read = 1,
    Write = 2,
}

#[repr(u8)]
#[typekin::bitflag(
    friend = [],
    integral = [friend = []],
)]
pub(crate) enum FriendlessFlag {
    Read = 1,
    Write = 2,
}

#[repr(u8)]
#[typekin::bitflag(
    integral = [
        valid = is_even,
        friend = math_bit_bits(MathBitBits) -> [Math, Bit],
    ],
    suffix = "Value",
)]
pub(crate) enum MathBitFlag {
    Empty = 0,
    Read = 2,
    Write = 4,
}

#[repr(u8)]
#[typekin::bitflag(integral = [without = fn_count_ones])]
pub(crate) enum NoCountOnesFlag {
    Read = 1,
    Write = 2,
}

#[typekin::bitflag(with = display)]
#[repr(u32)]
pub(crate) enum Wide {
    Low = 1,
    High = 0x8000_0000,
}

#[typekin::bitflag(with = display)]
#[repr(i16)]
pub(crate) enum Signed {
    Positive = 1,
    Negative = -1,
}

pub(crate) fn assert_display(
    value: impl core::fmt::Display,
    expected: &str,
) {
    assert_eq!(format!("{value}"), expected);
    assert_eq!(format!("{value:+014}"), expected);
    assert_eq!(format!("{value:*^15.2}"), expected);
}

// =============================================================================

#[repr(i8)]
#[typekin::bitflag]
pub(crate) enum SignedWithoutNested {
    Read = 1,
    Write = 2,
}

#[repr(i8)]
#[typekin::bitflag(integral = [])]
pub(crate) enum SignedEmptyNested {
    Read = 1,
    Write = 2,
}

#[repr(u8)]
#[typekin::bitflag]
pub(crate) enum UnsignedWithoutNested {
    Read = 1,
    Write = 2,
}

#[repr(u8)]
#[typekin::bitflag(integral = [])]
pub(crate) enum UnsignedEmptyNested {
    Read = 1,
    Write = 2,
}

// =============================================================================

#[repr(u8)]
#[typekin::bitflag(
    hide_in = none,
    integral = [with = with_blind_trust, domain = 0..=3],
)]
pub(crate) enum UnhiddenBlindTrustFlag {
    Read = 1,
    Write = 2,
}

/// Builds an `UnhiddenBlindTrustFlagValue` without validation, through the
/// `_blind_trust` constructor reachable only in this module.
pub(crate) const fn unhidden_blind_trust(
    raw: u8
) -> UnhiddenBlindTrustFlagValue {
    return UnhiddenBlindTrustFlagValue::_blind_trust(raw);
}
