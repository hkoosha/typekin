#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::zz_domain::{
        Domain,
        Tri,
    };
    use crate::zz_values::N;

    fn parse_error(source: &str) -> String {
        let this = syn::parse_str::<Domain>(source);
        assert!(this.is_err(), "{source}");
        return this.err().unwrap().to_string();
    }

    fn domain(source: &str) -> Domain {
        let this = syn::parse_str::<Domain>(source);
        assert!(this.is_ok(), "{source}: {:?}", this.err());
        return this.unwrap();
    }

    fn emitted(
        source: &str,
        subject: N,
    ) -> String {
        let resolved = domain(source).resolve(subject, false);
        assert!(resolved.is_ok(), "{source}: {:?}", resolved.err());
        return resolved.unwrap().emit(&quote! { it }).to_string();
    }

    fn resolve_error(
        source: &str,
        subject: N,
    ) -> String {
        let resolved = domain(source).resolve(subject, false);
        assert!(resolved.is_err(), "{source}");
        return resolved.err().unwrap().to_string();
    }

    // -------------------------------------------------------------------------
    // Grammar

    #[test]
    fn worked_example_emits_the_written_condition() {
        let source = "(1..12 | 87..) & !{10} & !mul(3)";
        let emitted = emitted(source, N::U032);

        assert_eq!(
            emitted,
            "(((it >= (1)) && (it < (12))) || (it >= (87))) && \
            (it != (10)) && ((it % (3)) != 0)",
            "{source}"
        );
    }

    #[test]
    fn written_text_is_spaced_around_set_operators() {
        let source = "(1..12|87..)&!{10,20}&(abs!in..=3)";
        let domain = domain(source);

        assert_eq!(
            domain.text(),
            "(1..12 | 87..) & !{10, 20} & (abs !in ..=3)",
            "{source}"
        );
    }

    #[test]
    fn roster_is_a_union_of_values() {
        let source = "{1, 3, 7}";
        let emitted = emitted(source, N::U032);

        assert_eq!(
            emitted, "(it == (1)) || (it == (3)) || (it == (7))",
            "{source}"
        );
    }

    #[test]
    fn complement_of_a_roster_excludes_each_value() {
        let source = "!{1, 3}";
        let emitted = emitted(source, N::U032);

        assert_eq!(emitted, "(it != (1)) && (it != (3))", "{source}");
    }

    #[test]
    fn empty_roster_is_rejected() {
        let source = "{}";
        let error = parse_error(source);

        assert_eq!(error, "write at least one value: {1}");
    }

    #[test]
    fn range_in_a_roster_is_rejected() {
        let source = "{1..3}";
        let error = parse_error(source);

        assert_eq!(
            error,
            "{…} lists single values; write ranges outside braces: {1} | 3..5"
        );
    }

    #[test]
    fn multiples_test_divisibility() {
        let source = "mul(3)";
        let emitted = emitted(source, N::U032);

        assert_eq!(emitted, "((it % (3)) == 0)", "{source}");
    }

    #[test]
    fn multiples_need_parentheses() {
        let source = "mul";
        let error = parse_error(source);

        assert_eq!(error, "write mul(3) for the multiples of 3");
    }

    #[test]
    fn multiples_take_one_constant() {
        let source = "mul(3, 4)";
        let error = parse_error(source);

        assert_eq!(error, "mul takes one constant: mul(3)");
    }

    #[test]
    fn complement_of_a_range_needs_no_parentheses() {
        let source = "!1..5";
        let emitted = emitted(source, N::U032);

        assert_eq!(emitted, "(! ((it >= (1)) && (it < (5))))", "{source}");
    }

    #[test]
    fn not_in_excludes_a_function_s_results() {
        let source = "abs !in ..3";
        let emitted = emitted(source, N::I032);

        assert_eq!(emitted, "(! ((it) . unsigned_abs () < (3)))", "{source}");
    }

    #[test]
    fn not_in_complements_listed_results_per_value() {
        let source = "count_ones !in {0, 1}";
        let emitted = emitted(source, N::U032);

        assert_eq!(
            emitted,
            "((it) . count_ones () != (0)) && ((it) . count_ones () != (1))",
            "{source}"
        );
    }

    #[test]
    fn complement_before_an_image_suggests_not_in() {
        let source = "!abs in ..3";
        let error = parse_error(source);

        assert_eq!(
            error,
            "a leading `!` reads as complementing `abs`; write `abs !in ..3`"
        );
    }

    #[test]
    fn complement_before_a_not_in_image_suggests_in() {
        let source = "!rem_euclid(16) !in {3}";
        let error = parse_error(source);

        assert_eq!(
            error,
            "a leading `!` reads as complementing `rem_euclid(16)`; write \
            `rem_euclid(16) in {3}`"
        );
    }

    #[test]
    fn double_complement_is_the_set() {
        let source = "!!{3}";
        let emitted = emitted(source, N::U032);

        assert_eq!(emitted, "(it == (3))", "{source}");
    }

    #[test]
    fn image_after_an_operator_needs_no_parentheses() {
        let source = "{0} | abs in 5..10";
        let emitted = emitted(source, N::I032);

        assert_eq!(
            emitted,
            "(it == (0)) || (((it) . unsigned_abs () >= (5)) && \
            ((it) . unsigned_abs () < (10)))",
            "{source}"
        );
    }

    #[test]
    fn operator_after_an_image_is_ambiguous() {
        let source = "abs in ..5 | 10..";
        let error = parse_error(source);

        assert_eq!(
            error,
            "ambiguous `|` after `abs in ..5`; write `abs in (..5 | …)` to \
            extend the function's results, or `(abs in ..5) | …` to combine \
            sets"
        );
    }

    #[test]
    fn operator_after_a_not_in_image_is_ambiguous() {
        let source = "rem_euclid(16) !in {3, 5} & mul(2)";
        let error = parse_error(source);

        assert_eq!(
            error,
            "ambiguous `&` after `rem_euclid(16) !in {3, 5}`; write \
            `rem_euclid(16) !in ({3, 5} & …)` to extend the function's \
            results, or `(rem_euclid(16) !in {3, 5}) & …` to combine sets"
        );
    }

    #[test]
    fn parenthesized_values_form_one_image() {
        let source = "abs in (..5 | 10..)";
        let emitted = emitted(source, N::I032);

        assert_eq!(
            emitted,
            "((it) . unsigned_abs () < (5)) || \
            ((it) . unsigned_abs () >= (10))",
            "{source}"
        );
    }

    #[test]
    fn function_inside_image_values_is_rejected() {
        let source = "abs in (count_ones in {1})";
        let error = parse_error(source);

        assert_eq!(
            error,
            "after `in` come the function's results: a range, {…}, mul(…), \
            or (…)"
        );
    }

    #[test]
    fn double_pipe_is_rejected() {
        let source = "1..2 || 5..6";
        let error = parse_error(source);

        assert_eq!(error, "expected | or & between domain sets");
    }

    #[test]
    fn arithmetic_between_sets_is_rejected() {
        let source = "{1} - {2}";
        let error = parse_error(source);

        assert_eq!(
            error,
            "domains have no arithmetic; combine sets with |, &, and !"
        );
    }

    #[test]
    fn arithmetic_in_a_bound_is_rejected() {
        let source = "1..10 - 2";
        let error = parse_error(source);

        assert_eq!(error, "need a constant; arithmetic is not supported");
    }

    #[test]
    fn parenthesized_constant_is_rejected() {
        let source = "(1)..5";
        let error = parse_error(source);

        assert_eq!(error, "constants take no parentheses; write 1..5");
    }

    #[test]
    fn parenthesized_constant_in_a_roster_is_rejected() {
        let source = "{(5)}";
        let error = parse_error(source);

        assert_eq!(error, "constants take no parentheses; write 1..5");
    }

    #[test]
    fn bare_literal_is_not_a_set() {
        let source = "1";
        let error = parse_error(source);

        assert_eq!(error, "a value alone is not a set; write it as {…}");
    }

    #[test]
    fn bracket_list_is_ordinary_invalid_syntax() {
        let source = "[1..2, 3..4]";
        let error = parse_error(source);

        assert_eq!(error, "expected a range, {…}, mul(…), a function, or (…)");
    }

    #[test]
    fn lens_arguments_are_emitted_parenthesized() {
        let source = "rem_euclid(16) in {3}";
        let emitted = emitted(source, N::U032);

        assert_eq!(emitted, "((it) . rem_euclid ((16)) == (3))", "{source}");
    }

    // -------------------------------------------------------------------------
    // Typing and diagnostics

    #[test]
    fn mismatched_type_constant_is_rejected() {
        let source = "..u8::MAX";
        let error = resolve_error(source, N::U032);

        assert_eq!(
            error,
            "u8::MAX does not match the u32 operand; write u32::MAX or a \
            literal"
        );
    }

    #[test]
    fn lens_result_type_types_its_constants() {
        let source = "abs in ..=i32::MAX";
        let error = resolve_error(source, N::I032);

        assert_eq!(error, "abs yields u32 here; write u32::MAX or a literal");
    }

    #[test]
    fn mismatched_suffix_is_rejected() {
        let source = "..5u8";
        let error = resolve_error(source, N::U032);

        assert_eq!(error, "u8 suffix does not match the u32 operand");
    }

    #[test]
    fn float_constant_on_integer_is_rejected() {
        let source = "..5.0";
        let error = resolve_error(source, N::U032);

        assert_eq!(error, "integer domains need integer literals; write 1");
    }

    #[test]
    fn division_by_zero_is_rejected() {
        let source = "mul(0)";
        let error = resolve_error(source, N::U032);

        assert_eq!(error, "division by zero");
    }

    #[test]
    fn divisor_one_is_rejected() {
        let source = "!mul(1)";
        let error = resolve_error(source, N::U032);

        assert_eq!(error, "every integer is divisible by 1");
    }

    #[test]
    fn negative_divisor_is_rejected() {
        let source = "mul(-3)";
        let error = resolve_error(source, N::I032);

        assert_eq!(error, "negative divisors are not supported");
    }

    #[test]
    fn abs_on_unsigned_is_rejected() {
        let source = "abs in ..=3";
        let error = resolve_error(source, N::U032);

        assert_eq!(error, "abs on u32 is the identity; remove it");
    }

    #[test]
    fn function_without_values_is_rejected() {
        let source = "count_ones";
        let error = resolve_error(source, N::U032);

        assert_eq!(
            error,
            "count_ones needs `in` and a set of its results; for example \
            count_ones in ..=3"
        );
    }

    #[test]
    fn signed_only_predicate_on_unsigned_is_rejected() {
        let source = "is_negative";
        let error = resolve_error(source, N::U008);

        assert_eq!(error, "is_negative requires a signed operand");
    }

    #[test]
    fn unknown_function_lists_alternatives() {
        let source = "sqrt in ..=3";
        let error = resolve_error(source, N::I032);

        assert!(
            error
                .starts_with("unknown domain function sqrt; available for i32"),
            "{error}"
        );
    }

    #[test]
    fn empty_range_is_rejected() {
        let source = "5..1";
        let error = resolve_error(source, N::U032);

        assert!(error.starts_with("this range is empty"), "{error}");
    }

    #[test]
    fn tautology_is_rejected() {
        let source = "0..";
        let error = resolve_error(source, N::U032);

        assert_eq!(error, "domain admits every u32 value; remove the domain");
    }

    #[test]
    fn contradiction_is_rejected() {
        let source = "300..";
        let error = resolve_error(source, N::U008);

        assert_eq!(error, "domain admits no u8 value");
    }

    #[test]
    fn lens_argument_must_fit_every_width() {
        let source = "abs_diff(100_000) in ..5";
        let error = resolve_error(source, N::ISIZ);

        assert_eq!(error, "100_000 does not fit 16-bit isize");
    }

    // -------------------------------------------------------------------------
    // Folding and normalization

    #[test]
    fn out_of_range_bound_is_never_emitted() {
        let source = "-5..10";
        let emitted = emitted(source, N::U008);

        assert_eq!(emitted, "(it < (10))", "{source}");
    }

    #[test]
    fn unsigned_lower_bound_at_zero_is_dropped() {
        let source = "0..10";
        let emitted = emitted(source, N::U032);

        assert_eq!(emitted, "(it < (10))", "{source}");
    }

    #[test]
    fn overlapping_intervals_merge() {
        let source = "1..5 | 3..10";
        let emitted = emitted(source, N::U032);

        assert_eq!(emitted, "((it >= (1)) && (it < (10)))", "{source}");
    }

    #[test]
    fn adjacent_intervals_merge() {
        let source = "1..=4 | 5..10";
        let emitted = emitted(source, N::U032);

        assert_eq!(emitted, "((it >= (1)) && (it < (10)))", "{source}");
    }

    #[test]
    fn disjoint_intervals_are_sorted() {
        let source = "8..10 | 2..=4";
        let emitted = emitted(source, N::U032);

        assert_eq!(
            emitted,
            "((it >= (2)) && (it <= (4))) || ((it >= (8)) && (it < (10)))",
            "{source}"
        );
    }

    #[test]
    fn same_operand_intervals_intersect() {
        let source = "1.. & ..10 & 3..";
        let emitted = emitted(source, N::U032);

        assert_eq!(emitted, "((it >= (3)) && (it < (10)))", "{source}");
    }

    #[test]
    fn singleton_interval_emits_equality() {
        let source = "5..=5";
        let emitted = emitted(source, N::U032);

        assert_eq!(emitted, "(it == (5))", "{source}");
    }

    #[test]
    fn redundant_not_equal_is_dropped() {
        let source = "1..10 & !{20}";
        let emitted = emitted(source, N::U032);

        assert_eq!(emitted, "((it >= (1)) && (it < (10)))", "{source}");
    }

    #[test]
    fn not_equal_at_an_interval_end_makes_it_exclusive() {
        let source = "5..=10 & !{5} & !{10}";
        let emitted = emitted(source, N::U008);

        assert_eq!(emitted, "((it > (5)) && (it < (10)))", "{source}");
    }

    #[test]
    fn not_equals_at_an_interval_end_chain() {
        let source = "!{6} & 5..=10 & !{5}";
        let emitted = emitted(source, N::U008);

        assert_eq!(emitted, "((it > (6)) && (it <= (10)))", "{source}");
    }

    #[test]
    fn excluding_a_singleton_interval_leaves_nothing() {
        let source = "{5} & !{5}";
        let error = resolve_error(source, N::U008);

        assert_eq!(error, "domain admits no u8 value");
    }

    #[test]
    fn relevant_not_equals_are_never_rewritten() {
        let source = "!{10} & !{20} & !{30}";
        let emitted = emitted(source, N::U032);

        assert_eq!(
            emitted, "(it != (10)) && (it != (20)) && (it != (30))",
            "{source}"
        );
    }

    #[test]
    fn non_zero_is_folded_into_the_domain() {
        let source = "1..";
        let resolved = domain(source).resolve(N::U032, true);
        assert!(resolved.is_ok(), "{source}");
        let emitted = resolved.unwrap().emit(&quote! { it }).to_string();

        assert_eq!(emitted, "(it >= (1))", "{source}");
    }

    #[test]
    fn per_width_folding_emits_a_cfg_block() {
        let source = "..=65_535";
        let emitted = emitted(source, N::USIZ);

        assert!(
            emitted.contains("target_pointer_width = \"16\""),
            "{emitted}"
        );
        assert!(
            emitted.contains("let __typekin_domain_valid = true"),
            "{source}: {emitted}"
        );
    }

    // -------------------------------------------------------------------------
    // Zero probes

    #[test]
    fn count_zeros_of_zero_is_the_16_bit_width() {
        let source = "count_zeros in {16}";
        let resolved = domain(source).resolve(N::USIZ, false);
        assert!(resolved.is_ok(), "{source}");

        assert!(resolved.unwrap().admits_zero(16), "{source}");
    }

    #[test]
    fn untyped_probe_decides_literal_domains() {
        let excluded = domain("1..").admits_zero_untyped();
        let admitted = domain("..=0").admits_zero_untyped();

        assert_eq!(excluded, Tri::False);
        assert_eq!(admitted, Tri::True);
    }

    #[test]
    fn untyped_probe_decides_complemented_values() {
        let excluded = domain("!{0}").admits_zero_untyped();
        let admitted = domain("!mul(3) | {0}").admits_zero_untyped();

        assert_eq!(excluded, Tri::False);
        assert_eq!(admitted, Tri::True);
    }

    #[test]
    fn untyped_probe_defers_width_dependent_functions() {
        let source = "count_zeros in {8}";
        let probe = domain(source).admits_zero_untyped();

        assert_eq!(probe, Tri::Unknown, "{source}");
    }
}

/// The exact evaluator `Within` forms are checked against: every lens agrees
/// with Rust's own method on every 8- and 16-bit value at every pointer
/// width.
#[cfg(test)]
mod lens_values {
    use crate::zz_domain::{
        Domain,
        WIDTHS,
    };
    use crate::zz_domain_int::{
        Operand,
        RNode,
        lens_value,
    };
    use crate::zz_values::{
        Int,
        N,
    };

    /// The operand of `{lens} in {0}` resolved for `subject` at width
    /// `index`.
    fn operand_of(
        lens: &str,
        subject: N,
        index: usize,
    ) -> Operand {
        let source = format!("{lens} in {{0}}");
        let domain = syn::parse_str::<Domain>(&source);
        assert!(domain.is_ok(), "{source}");
        let resolved = domain.unwrap().resolve(subject, false);
        assert!(resolved.is_ok(), "{source}: {:?}", resolved.err());
        let resolved = resolved.unwrap();
        let RNode::Interval { operand, .. } = &resolved.per_width[index]
        else {
            panic!("{source} is not a single interval");
        };
        return operand.clone();
    }

    /// `lens` on every value of `subject` equals `real` at every width.
    fn assert_exact(
        lens: &str,
        subject: N,
        real: impl Fn(i128) -> i128,
    ) {
        let (min, max) = subject.bounds_for_pointer_width(64);
        for index in 0..WIDTHS {
            let operand = operand_of(lens, subject, index);
            for value in min.to_i128()..=max.to_i128() {
                let it = Int::from_i128(value);
                let expected = Int::from_i128(real(value));
                assert_eq!(
                    lens_value(&operand, it, index),
                    expected,
                    "{lens} of {value}: {subject:?} at width index {index}"
                );
            }
        }
    }

    #[test]
    fn abs_is_exact() {
        let lens = "abs";
        assert_exact(lens, N::I008, |it| i128::from((it as i8).unsigned_abs()));
        assert_exact(lens, N::I016, |it| {
            i128::from((it as i16).unsigned_abs())
        });
    }

    #[test]
    fn unsigned_abs_is_exact() {
        let lens = "unsigned_abs";
        assert_exact(lens, N::I008, |it| i128::from((it as i8).unsigned_abs()));
        assert_exact(lens, N::I016, |it| {
            i128::from((it as i16).unsigned_abs())
        });
    }

    #[test]
    fn abs_diff_is_exact() {
        assert_exact("abs_diff(5)", N::U008, |it| {
            i128::from((it as u8).abs_diff(5))
        });
        assert_exact("abs_diff(-7)", N::I008, |it| {
            i128::from((it as i8).abs_diff(-7))
        });
        assert_exact("abs_diff(300)", N::U016, |it| {
            i128::from((it as u16).abs_diff(300))
        });
        assert_exact("abs_diff(-300)", N::I016, |it| {
            i128::from((it as i16).abs_diff(-300))
        });
    }

    #[test]
    fn signum_is_exact() {
        let lens = "signum";
        assert_exact(lens, N::I008, |it| i128::from((it as i8).signum()));
        assert_exact(lens, N::I016, |it| i128::from((it as i16).signum()));
    }

    #[test]
    fn count_ones_is_exact() {
        let lens = "count_ones";
        assert_exact(lens, N::U008, |it| i128::from((it as u8).count_ones()));
        assert_exact(lens, N::I008, |it| i128::from((it as i8).count_ones()));
        assert_exact(lens, N::U016, |it| i128::from((it as u16).count_ones()));
        assert_exact(lens, N::I016, |it| i128::from((it as i16).count_ones()));
    }

    #[test]
    fn count_zeros_is_exact() {
        let lens = "count_zeros";
        assert_exact(lens, N::U008, |it| i128::from((it as u8).count_zeros()));
        assert_exact(lens, N::I008, |it| i128::from((it as i8).count_zeros()));
        assert_exact(lens, N::U016, |it| i128::from((it as u16).count_zeros()));
        assert_exact(lens, N::I016, |it| i128::from((it as i16).count_zeros()));
    }

    #[test]
    fn leading_zeros_is_exact() {
        let lens = "leading_zeros";
        assert_exact(lens, N::U008, |it| {
            i128::from((it as u8).leading_zeros())
        });
        assert_exact(lens, N::I008, |it| {
            i128::from((it as i8).leading_zeros())
        });
        assert_exact(lens, N::U016, |it| {
            i128::from((it as u16).leading_zeros())
        });
        assert_exact(lens, N::I016, |it| {
            i128::from((it as i16).leading_zeros())
        });
    }

    #[test]
    fn trailing_zeros_is_exact() {
        let lens = "trailing_zeros";
        assert_exact(lens, N::U008, |it| {
            i128::from((it as u8).trailing_zeros())
        });
        assert_exact(lens, N::I008, |it| {
            i128::from((it as i8).trailing_zeros())
        });
        assert_exact(lens, N::U016, |it| {
            i128::from((it as u16).trailing_zeros())
        });
        assert_exact(lens, N::I016, |it| {
            i128::from((it as i16).trailing_zeros())
        });
    }

    #[test]
    fn leading_ones_is_exact() {
        let lens = "leading_ones";
        assert_exact(lens, N::U008, |it| i128::from((it as u8).leading_ones()));
        assert_exact(lens, N::I008, |it| i128::from((it as i8).leading_ones()));
        assert_exact(lens, N::U016, |it| {
            i128::from((it as u16).leading_ones())
        });
        assert_exact(lens, N::I016, |it| {
            i128::from((it as i16).leading_ones())
        });
    }

    #[test]
    fn trailing_ones_is_exact() {
        let lens = "trailing_ones";
        assert_exact(lens, N::U008, |it| {
            i128::from((it as u8).trailing_ones())
        });
        assert_exact(lens, N::I008, |it| {
            i128::from((it as i8).trailing_ones())
        });
        assert_exact(lens, N::U016, |it| {
            i128::from((it as u16).trailing_ones())
        });
        assert_exact(lens, N::I016, |it| {
            i128::from((it as i16).trailing_ones())
        });
    }

    #[test]
    fn rem_euclid_is_exact() {
        assert_exact("rem_euclid(7)", N::U008, |it| {
            i128::from((it as u8).rem_euclid(7))
        });
        assert_exact("rem_euclid(7)", N::I008, |it| {
            i128::from((it as i8).rem_euclid(7))
        });
        assert_exact("rem_euclid(1000)", N::U016, |it| {
            i128::from((it as u16).rem_euclid(1000))
        });
        assert_exact("rem_euclid(1000)", N::I016, |it| {
            i128::from((it as i16).rem_euclid(1000))
        });
    }

    #[test]
    fn div_euclid_is_exact() {
        assert_exact("div_euclid(3)", N::U008, |it| {
            i128::from((it as u8).div_euclid(3))
        });
        assert_exact("div_euclid(3)", N::I008, |it| {
            i128::from((it as i8).div_euclid(3))
        });
        assert_exact("div_euclid(300)", N::U016, |it| {
            i128::from((it as u16).div_euclid(300))
        });
        assert_exact("div_euclid(300)", N::I016, |it| {
            i128::from((it as i16).div_euclid(300))
        });
    }

    #[test]
    fn isqrt_is_exact() {
        let lens = "isqrt";
        assert_exact(lens, N::U008, |it| i128::from((it as u8).isqrt()));
        assert_exact(lens, N::U016, |it| i128::from((it as u16).isqrt()));
    }
}
