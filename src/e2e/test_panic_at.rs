use crate::fx_panic_at::*;
use crate::*;

fn score(raw: u8) -> Score {
    return Score::try_make(raw).unwrap();
}

#[cfg(test)]
mod operators {
    use super::*;

    #[test]
    fn invalid_results_report_their_raw_value() {
        let sum = score(60) + score(50);
        let valid = score(1) + score(2);

        assert_eq!(sum.map(Score::raw), Err(Some(110)));
        assert_eq!(valid.map(Score::raw), Ok(3));
    }

    #[test]
    fn division_by_zero_reports_no_value() {
        let quotient = score(10) / RawOperand(0);

        assert_eq!(quotient.map(Score::raw), Err(None));
    }

    #[test]
    fn nothing_reports_overflow() {
        let sum = Tally::make(u8::MAX) + Tally::make(1);
        let shifted = Tally::make(1) << 9usize;
        let in_range = Tally::make(1) << 1usize;

        assert_eq!(sum.map(Tally::raw), Err(None));
        assert_eq!(shifted.map(Tally::raw), Err(None));
        assert_eq!(in_range.map(Tally::raw), Ok(2));
    }

    #[test]
    fn overflow_reports_division_by_zero() {
        let quotient = Gauge::make(1) / Gauge::make(0);

        assert_eq!(quotient.map(Gauge::raw), Err(None));
    }

    #[test]
    fn arithmetic_leaves_division_by_zero_to_rust() {
        assert_panics(|| Meter::make(1) / Meter::make(0));
    }

    #[test]
    fn arithmetic_reports_sequential_overflow() {
        let sum = Ticket::make(u8::MAX) + Ticket::make(1);

        assert_eq!(sum.map(Ticket::raw), Err(None));
    }

    #[test]
    fn wrapping_arithmetic_wraps_and_reports_division_by_zero() {
        let sum = Phase::make(u8::MAX) + Phase::make(1);
        let quotient = Phase::make(1) / Phase::make(0);

        assert_eq!(sum.map(Phase::raw), Ok(0));
        assert_eq!(quotient.map(Phase::raw), Err(None));
    }

    #[test]
    fn nothing_reports_negating_the_minimum() {
        let minimum = -Lean::make(i8::MIN);
        let five = -Lean::make(5);

        assert_eq!(minimum.map(Lean::raw), Err(None));
        assert_eq!(five.map(Lean::raw), Ok(-5));
    }

    #[test]
    fn no_assignment_operator_is_generated() {
        assert_not_impl!(Score: core::ops::AddAssign<Score>);
        assert_not_impl!(Score: core::ops::AddAssign<RawOperand>);
        assert_not_impl!(Tally: core::ops::BitXorAssign<Tally>);
        assert_not_impl!(Tally: core::ops::ShlAssign<usize>);
        assert_not_impl!(Faceted: core::ops::AddAssign<RawOperand>);
    }
}

#[cfg(test)]
mod construction {
    use super::*;

    #[test]
    fn try_of_reports_the_rejected_raw_value() {
        let rejected = Score::try_of(RawOperand(0));
        let accepted = Score::try_of(RawOperand(5));

        assert_eq!(rejected.map(Score::raw), Err(0));
        assert_eq!(accepted.map(Score::raw), Ok(5));
    }

    #[test]
    fn panicking_constructors_are_omitted() {
        assert_no_associated_fn!(Score, of);
        assert_no_method!(Score, map);
    }

    #[test]
    fn try_map_reports_the_mapped_value() {
        let mapped = score(5).try_map(|it| it * 30);

        assert_eq!(mapped.map(Score::raw), Err(150));
    }

    #[test]
    fn within_sources_construct_through_try_of() {
        let half = Half::try_make(10).unwrap();
        let whole = Whole::try_of(half);

        assert_eq!(whole.map(Whole::raw), Ok(10));
    }
}

#[cfg(test)]
mod outputs {
    use super::*;

    #[test]
    fn produced_outputs_are_reported_in_a_result() {
        let three = Produced::try_make(3).unwrap();
        let ninety = Produced::try_make(90).unwrap();

        assert_eq!(three + three, Ok(12));
        assert_eq!(ninety + ninety, Err(Some(180)));
    }

    #[test]
    fn facets_report_failure() {
        let near_limit = Faceted::try_make(99).unwrap();
        let sum = near_limit + RawOperand(5);
        let quotient = near_limit / RawOperand(0);

        assert_eq!(sum.map(Faceted::raw), Err(Some(104)));
        assert_eq!(quotient.map(Faceted::raw), Err(None));
    }

    #[test]
    fn const_operators_report_failure() {
        assert_eq!(CONST_SCORE_SUM.map(ConstScore::raw), Ok(80));
    }
}

#[cfg(test)]
mod forwards {
    use super::*;

    #[test]
    fn checked_forwards_stay() {
        assert_eq!(score(50).checked_add(score(60)), None);
        assert_eq!(score(10).checked_add(score(5)).map(Score::raw), Some(15));
    }

    #[test]
    fn forwards_returning_validated_values_are_omitted() {
        assert_no_method!(Score, rotate_left);
        assert_no_method!(Score, wrapping_add);
        assert_no_associated_fn!(Score, from_be);
        assert_no_method!(Share, recip);
        assert_no_associated_fn!(Share, from_bits);
    }

    #[test]
    fn forwards_panicking_in_core_follow_the_level() {
        assert_no_method!(Score, ilog2);
        assert_eq!(Meter::make(8).ilog2(), 3);
    }

    #[test]
    fn non_failing_forwards_stay() {
        let half = Share::try_make(0.5).unwrap();

        assert!(half.is_finite());
        assert_eq!(score(6).count_ones(), 2);
    }
}

#[cfg(test)]
mod folds {
    use super::*;

    fn shares(raw: &[f64]) -> Vec<Share> {
        return raw.iter().map(|it| Share::try_make(*it).unwrap()).collect();
    }

    #[test]
    fn sums_collect_into_a_result() {
        let total: Result<Share, Option<f64>> = shares(&[
            0.25,
            0.5,
        ])
        .iter()
        .sum();
        let over: Result<Share, Option<f64>> = shares(&[
            0.75,
            0.5,
        ])
        .into_iter()
        .sum();

        assert_eq!(total.map(Share::raw), Ok(0.75));
        assert_eq!(over.map(Share::raw), Err(Some(1.25)));
    }

    #[test]
    fn folds_validate_every_step() {
        let product: Result<Share, Option<f64>> = shares(&[
            0.5,
            0.5,
        ])
        .into_iter()
        .product();
        let empty: Result<Share, Option<f64>> = shares(&[]).into_iter().sum();

        assert_eq!(product.map(Share::raw), Ok(0.25));
        assert_eq!(empty.map(Share::raw), Ok(0.0));
    }

    #[test]
    fn float_operators_report_invalid_values() {
        let half = Share::try_make(0.5).unwrap();
        let zero = Share::try_make(0.0).unwrap();

        assert_eq!((half + half).map(Share::raw), Ok(1.0));
        assert_eq!((half / zero).map(Share::raw), Err(Some(f64::INFINITY)));
    }
}
