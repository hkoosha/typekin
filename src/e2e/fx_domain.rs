use crate::*;

#[typekin::integral(domain = (1..12 | 87..) & !{10} & !mul(3))]
pub struct Code(u32);

#[typekin::integral(domain = (-5..12 | 87..) & !{10} & !mul(3))]
pub struct SignedCode(i64);

#[typekin::integral(domain = !{0})]
pub struct NotZero(u16);

#[typekin::integral(domain = is_positive)]
pub struct Positive(i32);

#[typekin::integral(domain = mul(2) | 100..=200)]
pub struct EvenOrBand(u8);

#[typekin::integral(domain = (abs in ..=100) & !mul(7))]
pub struct SmallNotSeven(i8);

#[typekin::integral(domain = (count_ones in {1}) | (rem_euclid(16) in {3}))]
pub struct Bits(u8);

#[typekin::integral(domain = 8..10 | 2..=4)]
pub struct Unsorted(u16);

#[typekin::integral(domain = 1..5 | 3..10)]
pub struct Overlapping(u16);

#[typekin::integral(domain = ..=65_535)]
pub struct AtMostU16(usize);

#[typekin::integral(domain = 1.., with = non_zero)]
pub struct PositiveNonZero(u8);

#[typekin::integral(domain = ..5 | 10.. & ..20)]
pub struct AndOverOr(u32);

#[typekin::integral(domain = (..5 | 10..) & ..20)]
pub struct GroupedOr(u32);

#[typekin::integral(domain = !(3..6))]
pub struct OutsideBand(u32);

#[typekin::integral(domain = !is_power_of_two)]
pub struct NotPowerOfTwo(u32);

#[typekin::integral(domain = ..-5)]
pub struct BelowMinusFive(i32);

#[typekin::integral(domain = 87..|{1})]
pub struct OneOrHigh(u32);

#[typekin::integral(domain = !{-5})]
pub struct NotMinusFive(i32);

#[typekin::integral(domain = (1..5))]
pub struct GroupedRange(u32);

#[typekin::integral(domain = abs in 2..5)]
pub struct AbsBand(i32);

#[typekin::integral(domain = -5..10)]
pub struct BelowTen(u8);

#[typekin::integral(domain = count_ones in {1})]
pub struct SingleBit(u16);

#[typekin::integral(domain = count_zeros in {16})]
pub struct SixteenZeros(usize);

#[typekin::integral(domain = 1..=1023 | 49_152..=65_535)]
pub struct SplitPort(u16);

pub(crate) const fn is_even(value: u8) -> bool {
    return value % 2 == 0;
}

#[typekin::integral(domain = ..=100, valid = is_even)]
pub struct EvenPercent(u8);

#[typekin::integral(
    domain = ..=100,
    friend = self(u8) -> Math,
    prove = [math_div, math_div(u8), math_rem, midpoint, isqrt],
)]
pub struct ProvenPercent(u8);

#[typekin::integral(domain = mul(3), prove = math_neg)]
pub struct Triple(i32);
