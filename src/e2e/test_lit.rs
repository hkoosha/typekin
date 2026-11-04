//! Per-type literal macros (`Port` → `port!`): constants checked at compile
//! time. Rejections are covered in `test_compile_fail.rs`.

use crate::fx_lit::net::{
    Digit,
    Offset,
    Port,
    digit,
    offset,
    port,
};
use crate::fx_lit::*;
use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_builds_a_const_item() {
        let raw = HTTPS.raw();

        assert_eq!(raw, 443);
    }

    #[test]
    fn literal_builds_a_value_in_a_fn_body() {
        let port = port!(49_152);

        assert_eq!(port.raw(), 49_152);
    }

    #[test]
    fn literal_accepts_constant_expressions() {
        const SSH: u16 = 22;
        let named = port!(SSH);
        let max = port!(u16::MAX);

        assert_eq!(named.raw(), SSH);
        assert_eq!(max.raw(), u16::MAX);
    }

    #[test]
    fn literal_accepts_a_negative_signed_value() {
        let offset = offset!(-5);

        assert_eq!(offset.raw(), -5);
    }

    #[test]
    fn inline_form_builds_the_same_value() {
        let inline = port!(const 80);
        let item = port!(80);

        assert_eq!(inline.raw(), item.raw());
    }

    #[test]
    fn lit_path_needs_no_import_of_the_type() {
        let percent = net::percent!(100);

        assert_eq!(percent.raw(), 100);
    }

    #[test]
    fn unhidden_type_gets_the_same_macro() {
        let digit = digit!(9);

        assert_eq!(digit.raw(), 9);
    }

    #[test]
    fn float_literal_builds_a_float_wrapper() {
        let half = unit!(0.5);

        assert_eq!(half.raw(), 0.5);
    }

    #[test]
    fn circular_literal_builds_a_wrapper_without_ordering() {
        let sequence = sequence!(u32::MAX);

        assert_eq!(sequence.raw(), u32::MAX);
    }

    #[test]
    fn bitflag_value_literal_builds_combined_bits() {
        let both = access_value!(0b11);

        assert!(both.contains(Access::Read));
        assert!(both.contains(Access::Write));
    }

    #[test]
    fn generic_literal_takes_explicit_arguments() {
        let tagged: Tagged<Out> = tagged!([Out] 5);

        assert_eq!(tagged.raw(), 5);
    }

    #[test]
    fn generic_inline_literal_infers_arguments() {
        let tagged = inferred::<In>();

        assert_eq!(tagged.raw(), 3);
    }

    #[test]
    fn generic_inline_literal_takes_outer_parameters() {
        let tagged = explicit_inline::<Out>();

        assert_eq!(tagged.raw(), 4);
    }

    #[test]
    fn generic_literal_takes_lifetime_and_const_arguments() {
        let labeled: Labeled<'static, str, 3> = labeled!(['static, str, 3] 6);

        assert_eq!(labeled.raw(), 6);
    }

    #[test]
    fn defaulted_generic_literal_takes_no_arguments() {
        let labeled: Labeled<'static> = labeled!(8);

        assert_eq!(labeled.raw(), 8);
    }

    #[test]
    fn lit_name_overrides_the_derived_name() {
        let line = line_no!(12);

        assert_eq!(line.raw(), 12);
    }

    #[test]
    fn private_type_literal_is_path_addressable() {
        let grade = grade_by_path();

        assert_eq!(grade, 3);
    }

    #[test]
    fn same_named_public_types_get_separate_literals() {
        let net: Port = port!(80);
        let other = other_port();

        assert_eq!(net.raw(), 80);
        assert_eq!(other.raw(), 8080);
    }

    #[test]
    fn unvalidated_type_gets_the_macro_on_request() {
        let count = count!(7);

        assert_eq!(count.raw(), 7);
    }

    #[test]
    fn unvalidated_type_leaves_its_literal_name_free() {
        let plain = plain!();

        assert_eq!(plain, "caller's plain!");
    }

    #[test]
    fn without_lit_macro_leaves_the_literal_name_free() {
        let unlisted = unlisted!();

        assert_eq!(unlisted, "caller's unlisted!");
    }

    #[test]
    fn non_const_try_make_leaves_the_literal_name_free() {
        let runtime = runtime!();

        assert_eq!(runtime, "caller's runtime!");
    }
}
