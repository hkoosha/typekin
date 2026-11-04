#[cfg(test)]
mod tests {
    use quote::format_ident;

    use crate::integral::{
        StoragePlan,
        ekran_with,
    };
    use crate::integral_cfg::Cfg;
    use crate::zz_values::N;

    fn expansion_of(
        input: &str,
        el: N,
    ) -> syn::Result<String> {
        let config = syn::parse_str::<Cfg>(input)?;
        let storage =
            StoragePlan::new(el, config.requires_non_zero_storage(), false);
        return ekran_with(
            format_ident!("Percent"),
            format_ident!("{}", el.rust_name()),
            Box::new(config),
            storage,
        )
        .map(|it| it.to_string());
    }

    fn proven_expansion_of(
        input: &str,
        el: N,
    ) -> String {
        let expansion = expansion_of(input, el);
        assert!(expansion.is_ok(), "{input}");
        return expansion.unwrap();
    }

    /// The body of the helper `fn {name}` up to the next item.
    fn helper_body<'a>(
        expansion: &'a str,
        name: &str,
    ) -> &'a str {
        let start = expansion
            .find(&format!("fn {name} ("))
            .unwrap_or_else(|| panic!("no fn {name}: {expansion}"));
        let rest = &expansion[start..];
        let end = rest.find("# [inline (always)]").unwrap_or(rest.len());
        return &rest[..end];
    }

    /// The impl whose header contains `header`, up to the next impl.
    fn impl_body<'a>(
        expansion: &'a str,
        header: &str,
    ) -> &'a str {
        let start = expansion
            .find(header)
            .unwrap_or_else(|| panic!("no {header}: {expansion}"));
        let rest = &expansion[start + header.len()..];
        let end = rest.find(" impl ").unwrap_or(rest.len());
        return &rest[..end];
    }

    const PROOF_DOC: &str = "\" Cannot fail validation: every result lies in \
        the domain `..=127` at every pointer width (proven by typekin at \
        expansion).\"";

    #[test]
    fn proven_shr_is_documented() {
        let input = "domain = ..=127";
        let expansion = proven_expansion_of(input, N::U008);
        let body = impl_body(&expansion, ":: core :: ops :: Shr < usize >");

        assert!(body.contains(PROOF_DOC), "{input}: {body}");
    }

    #[test]
    fn proven_shr_assign_is_documented() {
        let input = "domain = ..=127";
        let expansion = proven_expansion_of(input, N::U008);
        let body =
            impl_body(&expansion, ":: core :: ops :: ShrAssign < usize >");

        assert!(body.contains(PROOF_DOC), "{input}: {body}");
    }

    #[test]
    fn same_type_only_proof_adds_a_proven_sibling() {
        let input = "with = proven, domain = 1..=1023 | 49_152..=65_535";
        let expansion = proven_expansion_of(input, N::U016);
        let same = helper_body(&expansion, "_bitor_proven");

        assert!(same.contains("Self :: _proven (it)"), "{input}: {same}");
        assert!(
            expansion.contains("self . _bitor_proven ("),
            "{input}: {expansion}"
        );
    }

    #[test]
    fn proven_operator_is_documented() {
        let input = "domain = ..=100";
        let expansion = proven_expansion_of(input, N::U008);

        assert!(
            expansion.contains(
                "\" Cannot fail validation: every result lies in the domain \
                `..=100` at every pointer width (proven by typekin at \
                expansion).\""
            ),
            "{input}: {expansion}"
        );
    }

    #[test]
    fn proven_checked_method_skips_validation() {
        let input = "domain = ..=100";
        let expansion = proven_expansion_of(input, N::U008);
        let body = helper_body(&expansion, "checked_rem");

        assert!(body.contains("Self :: _make_proven ("), "{input}: {body}");
    }

    #[test]
    fn proven_constructor_rechecks_in_debug_builds() {
        let input = "domain = ..=100";
        let expansion = proven_expansion_of(input, N::U008);
        let body = helper_body(&expansion, "_proven");

        assert!(
            body.contains(":: core :: debug_assert !"),
            "{input}: {body}"
        );
    }

    #[test]
    fn wider_integers_skip_proofs_by_default() {
        let input = "domain = ..=100";
        let expansion = proven_expansion_of(input, N::U016);
        let body = helper_body(&expansion, "checked_rem");

        assert!(!body.contains("Self :: _make_proven ("), "{input}: {body}");
        assert!(
            !expansion.contains("Cannot fail validation"),
            "{input}: {expansion}"
        );
    }

    #[test]
    fn wider_integers_opt_into_proofs() {
        let input = "with = proven, domain = ..=100";
        let expansion = proven_expansion_of(input, N::U016);
        let body = helper_body(&expansion, "checked_rem");

        assert!(body.contains("Self :: _make_proven ("), "{input}: {body}");
    }

    #[test]
    fn eight_bit_integers_opt_out_of_proofs() {
        let input = "without = proven, domain = ..=100";
        let expansion = proven_expansion_of(input, N::U008);
        let body = helper_body(&expansion, "checked_rem");

        assert!(!body.contains("Self :: _make_proven ("), "{input}: {body}");
    }

    #[test]
    fn structural_neg_proof_ignores_the_proven_flag() {
        let input = "domain = -100..=100";
        let expansion = proven_expansion_of(input, N::I016);
        let body = helper_body(&expansion, "_neg");

        assert!(body.contains("Self :: _proven ("), "{input}: {body}");
    }

    #[test]
    fn prove_implies_proofs_on_wider_integers() {
        let input = "domain = ..=100, prove = math_rem";
        let expansion = proven_expansion_of(input, N::U016);
        let body = helper_body(&expansion, "checked_rem");

        assert!(body.contains("Self :: _make_proven ("), "{input}: {body}");
    }

    #[test]
    fn prove_rejects_disabled_proofs() {
        let input = "without = proven, domain = ..=100, prove = math_rem";
        let expansion = expansion_of(input, N::U008);
        assert!(expansion.is_err(), "{input}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "`prove` needs closure proofs; remove `without = proven`"
        );
    }

    #[test]
    fn prove_reports_a_counterexample() {
        let input = "domain = ..=100, prove = math_add";
        let expansion = expansion_of(input, N::U008);
        assert!(expansion.is_err(), "{input}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "`Percent + Percent` is not closed: 1 + 100 = 101 is outside \
            `..=100`"
        );
    }

    #[test]
    fn prove_reports_a_width_specific_counterexample() {
        let input = "domain = ..=70_000, prove = saturating_add";
        let expansion = expansion_of(input, N::USIZ);
        assert!(expansion.is_err(), "{input}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "`Percent::saturating_add` is not closed: saturating_add(1, 70000) \
            = 70001 is outside `..=70_000` at 32-bit pointer width"
        );
    }

    #[test]
    fn prove_reports_unbounded_results() {
        let input = "domain = 0..=10 | 20..=30, prove = math_and";
        let expansion = expansion_of(input, N::U008);
        assert!(expansion.is_err(), "{input}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "cannot prove `Percent & Percent`: results may reach 0..=30, \
            outside `0..=10 | 20..=30`"
        );
    }

    #[test]
    fn prove_rejects_an_opaque_domain() {
        let input = "domain = ..=100 & !{50}, prove = math_div";
        let expansion = expansion_of(input, N::U008);
        assert!(expansion.is_err(), "{input}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "`prove` needs an interval domain; `!{50}` is opaque to closure \
            proofs"
        );
    }

    #[test]
    fn prove_rejects_callbacks() {
        let input = "valid = crate::is_valid, prove = math_div";
        let expansion = expansion_of(input, N::U008);
        assert!(expansion.is_err(), "{input}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "`prove` needs an interval domain; `valid` callbacks are opaque \
            to closure proofs"
        );
    }

    #[test]
    fn prove_rejects_an_unknown_operation() {
        let input = "domain = ..=100, prove = math_foo";
        let expansion = expansion_of(input, N::U008);
        assert!(expansion.is_err(), "{input}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "unknown operation `math_foo` for `prove`"
        );
    }

    #[test]
    fn prove_rejects_a_disabled_operation() {
        let input = "domain = ..=100, without = math_div, prove = math_div";
        let expansion = expansion_of(input, N::U008);
        assert!(expansion.is_err(), "{input}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "`prove` lists `math_div`, which is not generated"
        );
    }

    #[test]
    fn prove_rejects_a_removed_method() {
        let input = "domain = ..=100, without = fn_midpoint, prove = midpoint";
        let expansion = expansion_of(input, N::U008);
        assert!(expansion.is_err(), "{input}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "`prove` lists `midpoint`, which is not generated"
        );
    }

    #[test]
    fn prove_rejects_a_right_hand_side_that_is_not_a_friend() {
        let input = "domain = ..=100, prove = math_div(Other)";
        let expansion = expansion_of(input, N::U008);
        assert!(expansion.is_err(), "{input}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "`Other` is not a primitive integer or a friend of `Percent`"
        );
    }

    #[test]
    fn prove_rejects_a_right_hand_side_on_a_method() {
        let input = "domain = ..=100, prove = midpoint(u8)";
        let expansion = expansion_of(input, N::U008);
        assert!(expansion.is_err(), "{input}");

        assert_eq!(
            expansion.unwrap_err().to_string(),
            "`midpoint` takes no right-hand side type in `prove`"
        );
    }
}
