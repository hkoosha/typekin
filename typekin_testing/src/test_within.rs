use crate::fx_within::*;
use crate::*;

/// Proven edges convert every source value to the same raw value.
#[cfg(test)]
#[cfg(feature = "test_fast")]
mod proven {
    use super::*;

    #[test]
    fn every_percent_converts_into_a_byte() {
        for raw in 0..=100u8 {
            let percent = Percent::try_make(raw).unwrap();

            assert_eq!(Byte::of(percent).raw(), u16::from(raw), "{raw}");
        }
    }

    #[test]
    fn every_u8_converts_into_a_byte() {
        for raw in u8::MIN..=u8::MAX {
            assert_eq!(Byte::of(raw).raw(), u16::from(raw), "{raw}");
        }
    }

    #[test]
    fn every_port_converts_into_a_positive() {
        for raw in u16::MIN..=u16::MAX {
            let Ok(port) = Port::try_make(raw)
            else {
                continue;
            };

            assert_eq!(Positive16::of(port).raw(), raw, "{raw}");
        }
    }

    #[test]
    fn every_odd_converts_into_a_wide_odd() {
        for raw in i8::MIN..=i8::MAX {
            let Ok(odd) = Odd::try_make(raw)
            else {
                continue;
            };

            assert_eq!(WideOdd::of(odd).raw(), i32::from(raw), "{raw}");
        }
    }

    #[test]
    fn every_multiple_of_six_converts_into_an_even() {
        for raw in (0..=60u32).step_by(6) {
            let sixes = Sixes::try_make(raw).unwrap();

            assert_eq!(Even::of(sixes).raw(), raw, "{raw}");
        }
    }

    #[test]
    fn every_mode_value_converts_into_its_bits() {
        for bits in 0..=0b111u8 {
            let mode = ModeValue::try_make(bits).unwrap();

            assert_eq!(ModeBits::of(mode).raw(), bits, "{bits}");
        }
    }

    #[test]
    fn every_mask_converts_into_a_lane_value() {
        for raw in 0..=15u8 {
            let mask = Mask::try_make(raw).unwrap();

            assert_eq!(LaneValue::of(mask).bits(), u16::from(raw), "{raw}");
        }
    }

    #[test]
    fn every_tagged_value_converts_into_a_tagged_byte() {
        for raw in 0..=100u16 {
            let tagged = Tagged::<In>::try_make(raw).unwrap();

            assert_eq!(TaggedByte::of(tagged).raw(), u32::from(raw), "{raw}");
        }
    }

    #[test]
    fn generic_target_carries_its_own_instantiation() {
        for raw in 0..=100u16 {
            let tagged = Tagged::<In>::try_make(raw).unwrap();

            assert_eq!(Carried::of(tagged).raw(), u32::from(raw), "{raw}");
        }
    }
}

/// `Within` changes how `Make` constructs; `Rel` and `Math` keep their
/// meaning, and operator results are still validated.
#[cfg(test)]
#[cfg(feature = "test_fast")]
mod relations {
    use super::*;

    #[test]
    fn within_friend_compares_by_raw_value() {
        let percent = Percent::try_make(40).unwrap();
        let score = Score::of(percent);

        assert!(score == percent);
        assert!(score < Percent::try_make(41).unwrap());
    }

    #[test]
    fn within_friend_math_result_is_still_validated() {
        let percent = Percent::try_make(60).unwrap();
        let score = Score::try_make(150).unwrap();

        assert_eq!((Score::of(percent) + percent).raw(), 120);
        assert_panics(move || score + percent);
    }
}

#[cfg(test)]
#[cfg(feature = "test_fast")]
mod constness {
    use super::*;

    #[test]
    fn nightly_konst_constructs_in_a_const_item() {
        const PERCENT: Percent = percent!(42);
        const BYTE: ConstByte = ConstByte::of(PERCENT);

        assert_eq!(BYTE.raw(), 42);
    }
}
