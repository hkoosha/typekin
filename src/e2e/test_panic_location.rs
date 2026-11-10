//! Where a panic in generated code is reported: at the consumer's call,
//! not inside the generated function.

use crate::fx_panic_location::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_operator_result_reports_the_caller() {
        let lhs = Percent::try_make(60).unwrap();
        let rhs = Percent::try_make(50).unwrap();

        let expected = (file!().to_owned(), line!() + 1);
        let location = panic_location(|| lhs + rhs);

        assert_eq!(location, expected, "{lhs:?} + {rhs:?}");
    }

    #[test]
    fn invalid_forward_result_reports_the_caller() {
        let lhs = Percent::try_make(60).unwrap();
        let rhs = Percent::try_make(50).unwrap();

        let expected = (file!().to_owned(), line!() + 1);
        let location = panic_location(|| lhs.saturating_add(rhs));

        assert_eq!(location, expected, "{lhs:?}.saturating_add({rhs:?})");
    }

    #[test]
    fn panicking_primitive_function_reports_the_caller() {
        let zero = Count::make(0);

        let expected = (file!().to_owned(), line!() + 1);
        let location = panic_location(|| zero.ilog2());

        assert_eq!(location, expected, "{zero:?}.ilog2()");
    }

    #[test]
    fn overflow_check_in_an_operator_reports_the_caller() {
        let lhs = Count::make(250);
        let rhs = Count::make(10);

        let expected = (file!().to_owned(), line!() + 1);
        let location = panic_location(|| lhs + rhs);

        assert_eq!(location, expected, "{lhs:?} + {rhs:?}");
    }

    #[test]
    fn invalid_friend_value_reports_the_caller() {
        let invalid = 200u16;

        let expected = (file!().to_owned(), line!() + 1);
        let location = panic_location(|| Percent::of(invalid));

        assert_eq!(location, expected, "Percent::of({invalid})");
    }
}
