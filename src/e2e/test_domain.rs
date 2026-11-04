//! The domain expression language as a consumer sees it: membership,
//! inferred `NonZero` storage, and differential checks against the condition
//! as written.

use core::mem::size_of;

use crate::fx_domain::*;
use crate::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_panics;

    #[test]
    fn worked_example_membership() {
        let in_first_range = 7;
        let excluded_value = 10;
        let divisible = 9;
        let in_gap = 50;
        let in_open_range = 88;

        assert_eq!(Code::try_make(in_first_range).map(Code::raw), Ok(7));
        assert_eq!(Code::try_make(excluded_value).map(Code::raw), Err(10));
        assert_eq!(Code::try_make(divisible).map(Code::raw), Err(9));
        assert_eq!(Code::try_make(in_gap).map(Code::raw), Err(50));
        assert_eq!(Code::try_make(in_open_range).map(Code::raw), Ok(88));
    }

    #[test]
    fn worked_example_infers_non_zero_storage() {
        assert_eq!(size_of::<Option<Code>>(), size_of::<Code>());
    }

    #[test]
    fn signed_domain_with_not_divisible_infers_non_zero_storage() {
        assert_eq!(size_of::<Option<SignedCode>>(), size_of::<SignedCode>());
        assert_eq!(SignedCode::try_make(-4).map(SignedCode::raw), Ok(-4));
        assert_eq!(SignedCode::try_make(-3), Err(-3));
    }

    #[test]
    fn not_equal_zero_infers_non_zero_storage() {
        assert_eq!(size_of::<Option<NotZero>>(), size_of::<NotZero>());
        assert_eq!(NotZero::try_make(0), Err(0));
    }

    #[test]
    fn is_positive_infers_non_zero_storage() {
        assert_eq!(size_of::<Option<Positive>>(), size_of::<Positive>());
        assert_eq!(Positive::try_make(0), Err(0));
        assert_eq!(Positive::try_make(-1), Err(-1));
    }

    #[test]
    fn zero_admitting_domain_keeps_primitive_storage() {
        assert_eq!(size_of::<Option<EvenOrBand>>(), 2);
    }

    #[test]
    fn non_zero_flag_folds_into_the_domain() {
        assert_eq!(
            size_of::<Option<PositiveNonZero>>(),
            size_of::<PositiveNonZero>()
        );
        assert_eq!(PositiveNonZero::try_make(0), Err(0));
        assert_eq!(
            PositiveNonZero::try_make(1).map(PositiveNonZero::raw),
            Ok(1)
        );
    }

    #[test]
    fn per_width_domain_matches_the_target_width() {
        let maximum = 65_535;
        let above = 65_536;

        assert_eq!(
            AtMostU16::try_make(maximum).map(AtMostU16::raw),
            Ok(maximum)
        );
        assert_eq!(AtMostU16::try_make(above), Err(above));
    }

    #[test]
    fn worked_example_matches_written_condition_on_u16_range() {
        let written = |it: u32| {
            return ((1..12).contains(&it) || it >= 87)
                && it != 10
                && it % 3 != 0;
        };
        for raw in 0..=u32::from(u16::MAX) {
            assert_eq!(Code::try_make(raw).is_ok(), written(raw), "{raw}");
        }
    }

    #[test]
    fn even_or_band_matches_written_condition_exhaustively() {
        let written = |it: u8| it % 2 == 0 || (100..=200).contains(&it);
        for raw in 0..=u8::MAX {
            assert_eq!(
                EvenOrBand::try_make(raw).is_ok(),
                written(raw),
                "{raw}"
            );
        }
    }

    #[test]
    fn small_not_seven_matches_written_condition_exhaustively() {
        let written = |it: i8| it.unsigned_abs() <= 100 && it % 7 != 0;
        for raw in i8::MIN..=i8::MAX {
            let actual = SmallNotSeven::try_make(raw).is_ok();
            assert_eq!(actual, written(raw), "{raw}");
        }
    }

    #[test]
    fn bits_matches_written_condition_exhaustively() {
        let written = |it: u8| it.count_ones() == 1 || it.rem_euclid(16) == 3;
        for raw in 0..=u8::MAX {
            assert_eq!(Bits::try_make(raw).is_ok(), written(raw), "{raw}");
        }
    }

    #[test]
    fn unsorted_union_matches_written_condition_exhaustively() {
        let written = |it: u16| (8..10).contains(&it) || (2..=4).contains(&it);
        for raw in 0..=u16::MAX {
            assert_eq!(Unsorted::try_make(raw).is_ok(), written(raw), "{raw}");
        }
    }

    #[test]
    fn overlapping_union_matches_written_condition_exhaustively() {
        let written = |it: u16| (1..5).contains(&it) || (3..10).contains(&it);
        for raw in 0..=u16::MAX {
            let actual = Overlapping::try_make(raw).is_ok();
            assert_eq!(actual, written(raw), "{raw}");
        }
    }

    #[test]
    fn and_binds_tighter_than_or() {
        let below_or = 3;
        let inside_and = 15;
        let above_and = 25;
        let below_or_result = AndOverOr::try_make(below_or).map(AndOverOr::raw);
        let inside_and_result =
            AndOverOr::try_make(inside_and).map(AndOverOr::raw);
        let above_and_result =
            AndOverOr::try_make(above_and).map(AndOverOr::raw);

        assert_eq!(below_or_result, Ok(below_or), "{below_or}");
        assert_eq!(inside_and_result, Ok(inside_and), "{inside_and}");
        assert_eq!(above_and_result, Err(above_and), "{above_and}");
    }

    #[test]
    fn parentheses_group_conditions() {
        let below = 3;
        let above = 25;
        let below_result = GroupedOr::try_make(below).map(GroupedOr::raw);
        let above_result = GroupedOr::try_make(above).map(GroupedOr::raw);

        assert_eq!(below_result, Ok(below), "{below}");
        assert_eq!(above_result, Err(above), "{above}");
    }

    #[test]
    fn negated_group_inverts_membership() {
        let inside = 4;
        let outside = 7;
        let inside_result = OutsideBand::try_make(inside).map(OutsideBand::raw);
        let outside_result =
            OutsideBand::try_make(outside).map(OutsideBand::raw);

        assert_eq!(inside_result, Err(inside), "{inside}");
        assert_eq!(outside_result, Ok(outside), "{outside}");
    }

    #[test]
    fn negated_predicate_inverts_membership() {
        let power = 8;
        let other = 6;
        let power_result =
            NotPowerOfTwo::try_make(power).map(NotPowerOfTwo::raw);
        let other_result =
            NotPowerOfTwo::try_make(other).map(NotPowerOfTwo::raw);

        assert_eq!(power_result, Err(power), "{power}");
        assert_eq!(other_result, Ok(other), "{other}");
    }

    #[test]
    fn range_to_a_negative_constant_excludes_it() {
        let below = -6;
        let at = -5;
        let below_result =
            BelowMinusFive::try_make(below).map(BelowMinusFive::raw);
        let at_result = BelowMinusFive::try_make(at).map(BelowMinusFive::raw);

        assert_eq!(below_result, Ok(below), "{below}");
        assert_eq!(at_result, Err(at), "{at}");
    }

    #[test]
    fn open_range_before_or_is_a_range() {
        let open = 100;
        let one = 1;
        let gap = 2;
        let open_result = OneOrHigh::try_make(open).map(OneOrHigh::raw);
        let one_result = OneOrHigh::try_make(one).map(OneOrHigh::raw);
        let gap_result = OneOrHigh::try_make(gap).map(OneOrHigh::raw);

        assert_eq!(open_result, Ok(open), "{open}");
        assert_eq!(one_result, Ok(one), "{one}");
        assert_eq!(gap_result, Err(gap), "{gap}");
    }

    #[test]
    fn not_equal_to_a_negative_constant_excludes_it() {
        let excluded = -5;
        let other = 5;
        let excluded_result =
            NotMinusFive::try_make(excluded).map(NotMinusFive::raw);
        let other_result = NotMinusFive::try_make(other).map(NotMinusFive::raw);

        assert_eq!(excluded_result, Err(excluded), "{excluded}");
        assert_eq!(other_result, Ok(other), "{other}");
    }

    #[test]
    fn grouped_range_is_the_range() {
        let inside = 2;
        let end = 5;
        let inside_result =
            GroupedRange::try_make(inside).map(GroupedRange::raw);
        let end_result = GroupedRange::try_make(end).map(GroupedRange::raw);

        assert_eq!(inside_result, Ok(inside), "{inside}");
        assert_eq!(end_result, Err(end), "{end}");
    }

    #[test]
    fn lens_with_in_range_tests_the_function_result() {
        let negative = -3;
        let small = 1;
        let negative_result = AbsBand::try_make(negative).map(AbsBand::raw);
        let small_result = AbsBand::try_make(small).map(AbsBand::raw);

        assert_eq!(negative_result, Ok(negative), "{negative}");
        assert_eq!(small_result, Err(small), "{small}");
    }

    #[test]
    fn negative_bound_on_unsigned_is_folded() {
        let zero = 0;
        let last = 9;
        let end = 10;
        let zero_result = BelowTen::try_make(zero).map(BelowTen::raw);
        let last_result = BelowTen::try_make(last).map(BelowTen::raw);
        let end_result = BelowTen::try_make(end).map(BelowTen::raw);

        assert_eq!(zero_result, Ok(zero), "{zero}");
        assert_eq!(last_result, Ok(last), "{last}");
        assert_eq!(end_result, Err(end), "{end}");
    }

    #[test]
    fn single_set_bit_infers_non_zero_storage() {
        let zero = 0;
        let bit = 4;
        let zero_result = SingleBit::try_make(zero).map(SingleBit::raw);
        let bit_result = SingleBit::try_make(bit).map(SingleBit::raw);

        assert_eq!(size_of::<Option<SingleBit>>(), size_of::<SingleBit>());
        assert_eq!(zero_result, Err(zero), "{zero}");
        assert_eq!(bit_result, Ok(bit), "{bit}");
    }

    #[test]
    fn count_zeros_counts_at_the_target_width() {
        let zero = 0;
        let sixteen_zeros = usize::MAX >> 16;
        let zero_result = SixteenZeros::try_make(zero).map(SixteenZeros::raw);
        let sixteen_zeros_result =
            SixteenZeros::try_make(sixteen_zeros).map(SixteenZeros::raw);

        assert_eq!(zero_result, Err(zero), "{zero}");
        assert_eq!(sixteen_zeros_result, Ok(sixteen_zeros), "{sixteen_zeros}");
    }

    #[test]
    fn prove_accepts_proven_operations() {
        let hundred = ProvenPercent::try_make(100);
        let three = ProvenPercent::try_make(3);
        let seven = 7u8;
        assert!(hundred.is_ok() && three.is_ok());
        let hundred = hundred.unwrap();
        let three = three.unwrap();

        assert_eq!((hundred / three).raw(), 33, "{hundred:?} / {three:?}");
        assert_eq!((hundred / seven).raw(), 14, "{hundred:?} / {seven}");
        assert_eq!((hundred % three).raw(), 1, "{hundred:?} % {three:?}");
        assert_eq!(hundred.midpoint(three).raw(), 51, "{hundred:?}");
        assert_eq!(hundred.isqrt().raw(), 10, "{hundred:?}");
    }

    #[test]
    fn prove_accepts_a_symmetric_negation() {
        let three = Triple::try_make(3);
        assert!(three.is_ok());
        let three = three.unwrap();

        assert_eq!((-three).raw(), -3, "{three:?}");
    }

    #[test]
    fn callback_validates_an_otherwise_proven_division() {
        let four = EvenPercent::try_make(4);
        let six = EvenPercent::try_make(6);
        let two = EvenPercent::try_make(2);
        assert!(four.is_ok() && six.is_ok() && two.is_ok());
        let four = four.unwrap();
        let six = six.unwrap();
        let two = two.unwrap();

        assert_eq!((four / two).raw(), 2, "{four:?} / {two:?}");
        assert_panics(move || six / two);
    }

    #[test]
    fn primitive_bitor_validates_beside_a_same_type_proof() {
        let low = SplitPort::try_make(1);
        let high = SplitPort::try_make(49_152);
        let outside = 2_048u16;
        assert!(low.is_ok() && high.is_ok());
        let low = low.unwrap();
        let high = high.unwrap();

        assert_eq!((low | high).raw(), 49_153, "{low:?} | {high:?}");
        assert_panics(move || low | outside);
    }
}
