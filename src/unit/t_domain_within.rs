//! `Within` forms and the shared subset check: forms agree with the exact
//! evaluator value by value, and the check agrees with enumeration.

/// Every 8- and 16-bit value, at every pointer width: the form admits a
/// value exactly when `contains` does, one test per leaf rule.
#[cfg(test)]
mod forms {
    use crate::zz_domain::Domain;
    use crate::zz_domain_int::ResolvedDomain;
    use crate::zz_domain_within::Form;
    use crate::zz_values::{
        Int,
        N,
        POINTER_WIDTHS,
    };

    pub(crate) fn resolved(
        source: &str,
        subject: N,
    ) -> ResolvedDomain {
        let domain = syn::parse_str::<Domain>(source);
        assert!(domain.is_ok(), "{source}");
        let resolved = domain.unwrap().resolve(subject, false);
        assert!(resolved.is_ok(), "{source}: {:?}", resolved.err());
        return resolved.unwrap();
    }

    /// Membership in a form: some piece of `value`'s residue spans it.
    pub(crate) fn contains(
        form: &Form,
        value: Int,
    ) -> bool {
        let modulus = Int::NonNegative(form.modulus);
        let residue = value.checked_rem_euclid(modulus).unwrap().magnitude();
        return form.pieces.iter().any(|it| {
            return it.residue == residue && it.lo <= value && value <= it.hi;
        });
    }

    /// Pieces sorted by `(residue, lo)`, members at both ends, disjoint and
    /// not adjacent within a residue.
    pub(crate) fn assert_invariants(
        source: &str,
        form: &Form,
    ) {
        let modulus = Int::NonNegative(form.modulus);
        let residue = |it: Int| it.checked_rem_euclid(modulus).unwrap();
        for piece in &form.pieces {
            assert!(piece.residue < form.modulus, "{source}: {form:?}");
            assert!(piece.lo <= piece.hi, "{source}: {form:?}");
            let expected = Int::NonNegative(piece.residue);
            assert_eq!(residue(piece.lo), expected, "{source}: {form:?}");
            assert_eq!(residue(piece.hi), expected, "{source}: {form:?}");
        }
        for pair in form.pieces.windows(2) {
            let (left, right) = (pair[0], pair[1]);
            assert!(
                (left.residue, left.lo) < (right.residue, right.lo),
                "{source}: {form:?}"
            );
            if left.residue == right.residue {
                let next = left.hi.checked_add(modulus).unwrap();
                assert!(next < right.lo, "{source}: {form:?}");
            }
        }
    }

    fn assert_exact(
        source: &str,
        subject: N,
    ) {
        let domain = resolved(source, subject);
        let (min, max) = subject.bounds_for_pointer_width(16);
        for (index, width) in POINTER_WIDTHS.into_iter().enumerate() {
            let form = domain.form(index);
            assert!(form.is_ok(), "{source}: {:?}", form.err());
            let form = form.unwrap();
            assert_invariants(source, &form);
            for value in min.to_i128()..=max.to_i128() {
                let it = Int::from_i128(value);
                assert_eq!(
                    contains(&form, it),
                    domain.contains(it, width),
                    "{source}: {value} at {width} bits"
                );
            }
        }
    }

    #[test]
    fn range_is_exact() {
        assert_exact("1..=100", N::U008);
        assert_exact("-50..=50", N::I008);
        assert_exact("..300 | 1000..", N::U016);
    }

    #[test]
    fn excluded_value_is_exact() {
        assert_exact("!{10}", N::U008);
        assert_exact("..=100 & !{50}", N::I016);
    }

    #[test]
    fn multiples_are_exact() {
        assert_exact("mul(3)", N::U008);
        assert_exact("!mul(4)", N::I016);
        assert_exact("mul(6) & ..=60", N::U016);
    }

    #[test]
    fn power_of_two_is_exact() {
        assert_exact("is_power_of_two", N::U016);
    }

    #[test]
    fn sign_predicates_are_exact() {
        assert_exact("is_positive", N::I008);
        assert_exact("is_negative", N::I016);
    }

    #[test]
    fn abs_is_exact() {
        assert_exact("abs in 3..=10", N::I008);
        assert_exact("abs !in {5}", N::I016);
        assert_exact("abs in mul(3)", N::I016);
        assert_exact("unsigned_abs in ..=100", N::I016);
    }

    #[test]
    fn abs_diff_is_exact() {
        assert_exact("abs_diff(100) in ..=20", N::U008);
        assert_exact("abs_diff(-5) in mul(4)", N::I016);
        assert_exact("abs_diff(300) !in {7}", N::U016);
        assert_exact("abs_diff(-100) in 50..", N::I008);
    }

    #[test]
    fn signum_is_exact() {
        assert_exact("signum in {-1, 1}", N::I008);
        assert_exact("signum in mul(2)", N::I016);
    }

    #[test]
    fn div_euclid_is_exact() {
        assert_exact("div_euclid(10) in 2..=5", N::U008);
        assert_exact("div_euclid(7) in mul(3)", N::I016);
        assert_exact("div_euclid(100) !in {-3}", N::I016);
    }

    #[test]
    fn rem_euclid_is_exact() {
        assert_exact("rem_euclid(16) in {3, 5}", N::U008);
        assert_exact("rem_euclid(7) in mul(2)", N::I016);
    }

    #[test]
    fn isqrt_is_exact() {
        assert_exact("isqrt in 3..=9", N::U016);
        assert_exact("isqrt !in {7}", N::U008);
    }

    #[test]
    fn leading_zeros_is_exact() {
        assert_exact("leading_zeros in 2..=4", N::U016);
        assert_exact("leading_zeros in mul(2)", N::I008);
        assert_exact("leading_zeros in {0}", N::I016);
    }

    #[test]
    fn leading_ones_is_exact() {
        assert_exact("leading_ones in {1, 3}", N::I016);
        assert_exact("leading_ones in 2..", N::U008);
    }

    #[test]
    fn trailing_zeros_is_exact() {
        assert_exact("trailing_zeros in 3..", N::U016);
        assert_exact("trailing_zeros in {0, 2}", N::I008);
        assert_exact("trailing_zeros in mul(2)", N::I008);
    }

    #[test]
    fn trailing_ones_is_exact() {
        assert_exact("trailing_ones in 2..=3", N::U008);
        assert_exact("trailing_ones in {8}", N::I008);
        assert_exact("trailing_ones !in {0}", N::I016);
    }

    #[test]
    fn mixed_domain_is_exact() {
        assert_exact("(1..12 | 87..) & !{10} & !mul(3)", N::U008);
        assert_exact("(1..12 | 87..) & !{10} & !mul(3)", N::U016);
    }

    #[test]
    fn lens_with_multiples_is_exact() {
        assert_exact("(abs in ..=100) & !mul(7)", N::I016);
    }

    #[test]
    fn lens_union_is_exact() {
        assert_exact(
            "(trailing_zeros in 3..) | (leading_ones in {2})",
            N::U016,
        );
    }

    #[test]
    fn complemented_group_is_exact() {
        assert_exact("!(3..6 | mul(5)) & ..=200", N::U008);
    }

    #[test]
    fn non_zero_requirement_is_exact() {
        let source = "..=100";
        let domain = syn::parse_str::<Domain>(source).unwrap();
        let resolved = domain.resolve(N::U008, true);
        assert!(resolved.is_ok(), "{source}");
        let resolved = resolved.unwrap();
        let form = resolved.form(0);
        assert!(form.is_ok(), "{source}: {:?}", form.err());

        let form = form.unwrap();
        assert!(!contains(&form, Int::NonNegative(0)), "{source}: {form:?}");
        assert!(contains(&form, Int::NonNegative(1)), "{source}: {form:?}");
        assert!(contains(&form, Int::NonNegative(100)), "{source}: {form:?}");
        assert!(
            !contains(&form, Int::NonNegative(101)),
            "{source}: {form:?}"
        );
    }

    #[test]
    fn pointer_sized_domain_at_16_bits_is_exact() {
        let source = "..=1000 | 60_000..";
        let domain = resolved(source, N::USIZ);
        let form = domain.form(0);
        assert!(form.is_ok(), "{source}: {:?}", form.err());
        let form = form.unwrap();

        for value in 0..=u16::MAX {
            let it = Int::NonNegative(u128::from(value));
            assert_eq!(
                contains(&form, it),
                domain.contains(it, 16),
                "{source}: {value}"
            );
        }
    }
}

/// Types too wide to enumerate: the form agrees with `contains` at every
/// segment start, one value either side, one modulus either side, the
/// type's ends, and zero.
#[cfg(test)]
mod wide_forms {
    use super::forms::{
        assert_invariants,
        contains,
        resolved,
    };
    use crate::zz_values::{
        Int,
        N,
        POINTER_WIDTHS,
    };

    fn assert_exact_at_boundaries(
        source: &str,
        subject: N,
    ) {
        let domain = resolved(source, subject);
        for (index, width) in POINTER_WIDTHS.into_iter().enumerate() {
            let (min, max) = subject.bounds_for_pointer_width(width);
            let segments = domain.segments(index);
            assert!(segments.is_ok(), "{source}: {:?}", segments.err());
            let (starts, modulus) = segments.unwrap();
            let form = domain.form(index);
            assert!(form.is_ok(), "{source}: {:?}", form.err());
            let form = form.unwrap();
            assert_invariants(source, &form);

            let modulus = Int::NonNegative(modulus);
            let one = Int::NonNegative(1);
            let mut points = vec![
                min,
                max,
                Int::NonNegative(0),
            ];
            for start in starts {
                points.extend(
                    [
                        start.checked_sub(one),
                        Some(start),
                        start.checked_add(one),
                        start.checked_sub(modulus),
                        start.checked_add(modulus),
                    ]
                    .into_iter()
                    .flatten(),
                );
            }
            for point in
                points.into_iter().filter(|it| min <= *it && *it <= max)
            {
                assert_eq!(
                    contains(&form, point),
                    domain.contains(point, width),
                    "{source}: {point} at {width} bits"
                );
            }
        }
    }

    #[test]
    fn u64_range_is_exact_at_boundaries() {
        assert_exact_at_boundaries(
            "1..=1_000 | 4_000_000_000.. & !{18_446_744_073_709_551_000}",
            N::U064,
        );
    }

    #[test]
    fn i64_lenses_are_exact_at_boundaries() {
        assert_exact_at_boundaries(
            "(abs in 10..=1_000_000) & !mul(7) | (leading_zeros in {3})",
            N::I064,
        );
    }

    #[test]
    fn u128_lenses_are_exact_at_boundaries() {
        assert_exact_at_boundaries(
            "(trailing_zeros in 4..) & (isqrt in 1_000..) | {u128::MAX}",
            N::U128,
        );
    }

    #[test]
    fn i128_lenses_are_exact_at_boundaries() {
        assert_exact_at_boundaries(
            "(abs_diff(-5) in ..=100) | (div_euclid(1_000) in 7..) | \
            (leading_ones in {4})",
            N::I128,
        );
    }

    #[test]
    fn usize_range_is_exact_at_boundaries() {
        assert_exact_at_boundaries("..=1_000 | 4_294_967_296..", N::USIZ);
    }
}

/// Domains without a form, each with its reason.
#[cfg(test)]
mod opaque {
    use super::forms::resolved;
    use crate::integral_cfg::Cfg;
    use crate::zz_domain_within::Opaque;
    use crate::zz_values::{
        FnName,
        N,
    };

    fn reason(
        source: &str,
        subject: N,
    ) -> Opaque {
        let form = resolved(source, subject).form(2);
        assert!(form.is_err(), "{source}: {form:?}");
        return form.unwrap_err();
    }

    #[test]
    fn count_ones_is_opaque() {
        let source = "count_ones in {3}";
        let reason = reason(source, N::U016);

        assert_eq!(reason, Opaque::Lens(FnName::CountOnes), "{source}");
        assert_eq!(
            reason.describe("Wide"),
            "`Wide`'s domain uses `count_ones`, which is opaque to subdomain \
            proofs"
        );
    }

    #[test]
    fn isqrt_multiples_are_opaque() {
        let source = "isqrt in mul(2)";
        let reason = reason(source, N::U016);

        assert_eq!(reason, Opaque::IsqrtMultiples, "{source}");
        assert_eq!(
            reason.describe("Root"),
            "`Root`'s domain uses `isqrt` with `mul`, which is opaque to \
            subdomain proofs"
        );
    }

    #[test]
    fn callback_is_opaque() {
        let it = "valid = is_even";
        let cfg = syn::parse_str::<Cfg>(it);
        assert!(cfg.is_ok(), "{it}");
        let form = cfg.unwrap().validation.form(N::U016, 0);
        assert!(form.is_err(), "{it}: {form:?}");
        let reason = form.unwrap_err();

        assert_eq!(reason, Opaque::Callbacks("is_even".to_string()), "{it}");
        assert_eq!(
            reason.describe("Even"),
            "`Even`'s `valid = is_even` is opaque to subdomain proofs"
        );
    }

    #[test]
    fn modulus_above_the_cap_is_opaque() {
        let source = "mul(32) & mul(33)";
        let reason = reason(source, N::U032);

        assert_eq!(reason, Opaque::Modulus(1056), "{source}");
        assert_eq!(
            reason.describe("Odd"),
            "`Odd`'s domain repeats with period 1056, above the subdomain \
            proof limit of 1024"
        );
    }

    #[test]
    fn evaluations_above_the_cap_are_opaque() {
        let source = "(leading_zeros in {3}) & mul(1000)";
        let reason = reason(source, N::U064);

        assert!(
            matches!(reason, Opaque::Evaluations(it) if it > 65_536),
            "{source}: {reason:?}"
        );
    }

    #[test]
    fn pieces_above_the_cap_are_opaque() {
        let source = "rem_euclid(1000) in ..500";
        let reason = reason(source, N::U032);

        assert_eq!(reason, Opaque::Pieces(500), "{source}");
        assert_eq!(
            reason.describe("Half"),
            "`Half`'s domain has 500 pieces, above the subdomain proof limit \
            of 256"
        );
    }
}

/// The shared check against enumeration: proven exactly when every source
/// member lies in the target, and a refutation names the least member that
/// does not.
#[cfg(test)]
mod subset {
    use super::forms::{
        contains,
        resolved,
    };
    use crate::zz_domain_within::{
        Form,
        subset,
    };
    use crate::zz_domain_within_const::__TypekinWithin;
    use crate::zz_values::{
        Int,
        N,
    };

    fn form(
        source: Option<&str>,
        subject: N,
    ) -> Form {
        let Some(source) = source
        else {
            return Form::full(subject, 0);
        };
        let form = resolved(source, subject).form(0);
        assert!(form.is_ok(), "{source}: {:?}", form.err());
        return form.unwrap();
    }

    /// The verdict as text: `proven`, `range -5`, `domain 7`, `budget 9`.
    fn text(verdict: &__TypekinWithin) -> String {
        let value = |negative: bool, magnitude: u128| {
            return Int::from_sign_magnitude(negative, magnitude).to_string();
        };
        return match verdict {
            __TypekinWithin::Proven => "proven".to_string(),
            __TypekinWithin::Range(negative, magnitude) => {
                format!("range {}", value(*negative, *magnitude))
            }
            __TypekinWithin::Domain(negative, magnitude) => {
                format!("domain {}", value(*negative, *magnitude))
            }
            __TypekinWithin::Budget(steps) => format!("budget {steps}"),
        };
    }

    /// The verdict as text, checked against every member of an 8- or 16-bit
    /// source: the least member outside the raw range, else the least one
    /// outside the target.
    fn verdict(
        (source, source_type): (Option<&str>, N),
        (target, target_type): (Option<&str>, N),
    ) -> String {
        let source_form = form(source, source_type);
        let target_form = form(target, target_type);
        let range = target_type.bounds_for_pointer_width(16);
        let (min, max) = source_type.bounds_for_pointer_width(16);
        let fits = |it: Int| range.0 <= it && it <= range.1;
        let members = (min.to_i128()..=max.to_i128())
            .map(Int::from_i128)
            .filter(|it| contains(&source_form, *it));
        let outside = members.clone().find(|it| !fits(*it));
        let uncovered = members.clone().find(|it| !contains(&target_form, *it));
        let verdict = text(&subset(&source_form, &target_form, range));
        let expected = match (outside, uncovered) {
            (Some(it), _) => format!("range {it}"),
            (None, Some(it)) => format!("domain {it}"),
            (None, None) => "proven".to_string(),
        };
        assert_eq!(verdict, expected, "{source:?} within {target:?}");
        return verdict;
    }

    #[test]
    fn narrow_range_is_within_a_wider_type() {
        let verdict =
            verdict((Some("..=100"), N::U008), (Some("..=255"), N::U016));

        assert_eq!(verdict, "proven");
    }

    #[test]
    fn union_is_within_its_hull() {
        let verdict = verdict(
            (Some("1..=1023 | 49_152.."), N::U016),
            (Some("1.."), N::U016),
        );

        assert_eq!(verdict, "proven");
    }

    #[test]
    fn zero_refutes_a_positive_target() {
        let verdict =
            verdict((Some("..=255"), N::U016), (Some("1.."), N::U016));

        assert_eq!(verdict, "domain 0");
    }

    #[test]
    fn least_even_outside_the_sixes_refutes() {
        let verdict = verdict(
            (Some("mul(2)"), N::U016),
            (Some("mul(6) & ..=60"), N::U016),
        );

        assert_eq!(verdict, "domain 2");
    }

    #[test]
    fn multiples_of_six_are_within_the_evens() {
        let verdict = verdict(
            (Some("mul(6) & ..=60"), N::U016),
            (Some("mul(2)"), N::U016),
        );

        assert_eq!(verdict, "proven");
    }

    #[test]
    fn least_value_above_the_raw_range_refutes() {
        let verdict = verdict((Some("mul(3)"), N::U016), (None, N::U008));

        assert_eq!(verdict, "range 258");
    }

    #[test]
    fn least_value_below_the_raw_range_refutes() {
        let verdict =
            verdict((Some("-200..=-5 | 7.."), N::I016), (None, N::I008));

        assert_eq!(verdict, "range -200");
    }

    #[test]
    fn signed_source_refutes_at_its_least_uncovered_value() {
        let verdict = verdict(
            (Some("abs in ..=100"), N::I008),
            (Some("-128..=100 & !{-50} & !{20}"), N::I016),
        );

        assert_eq!(verdict, "domain -50");
    }

    #[test]
    fn residue_classes_combine_across_moduli() {
        let verdict = verdict(
            (Some("rem_euclid(12) in {2, 10}"), N::U016),
            (
                Some("mul(2) & !mul(4) | mul(4) & !mul(8) | mul(8)"),
                N::U016,
            ),
        );

        assert_eq!(verdict, "proven");
    }

    #[test]
    fn lens_source_is_checked_value_by_value() {
        let verdict = verdict(
            (Some("leading_zeros in {2}"), N::U016),
            (Some("8_192..=16_000"), N::U016),
        );

        assert_eq!(verdict, "domain 16001");
    }

    #[test]
    fn single_value_target_refutes_the_least_member() {
        let verdict =
            verdict((Some("100..=200"), N::U008), (Some("{7}"), N::U016));

        assert_eq!(verdict, "domain 100");
    }

    #[test]
    fn walk_above_the_budget_is_refused() {
        let source = form(Some("mul(997)"), N::U064);
        let target = form(Some("rem_euclid(1009) in ..100"), N::U064);
        let range = N::U064.bounds_for_pointer_width(16);
        let verdict = text(&subset(&source, &target, range));

        assert_eq!(verdict, "budget 101909");
    }
}
