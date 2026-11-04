#[cfg(test)]
mod tests {
    use crate::text::Cfg;
    use crate::zz_lib::Konsts;

    #[test]
    fn rejects_multiple_single_explicit_friends() {
        let input = "
            friend = convert(Source) -> Make,
            convert(Other) -> Make
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_multiple_single_implicit_friends() {
        let input = "
            friend = _(Source) -> unsafe,
            _(Other) -> unsafe
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_missing_single_friend() {
        let input = "
            friend =
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_missing_single_friend_before_validation() {
        let input = "
            friend =,
            valid = is_slug
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_conversion_without_source() {
        let input = "
            friend = convert() -> Make
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_conversion_without_capabilities() {
        let input = "
            friend = convert(Source) ->
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_non_identifier_friend_capability() {
        let input = "
            friend = convert(Source) -> [Make, 1]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_attribute_after_implicit_friend() {
        let input = "
            friend = _(Source) -> [unsafe],
            unknown = true
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_friends_after_implicit_friend() {
        let input = "
            friend = _(Source) -> unsafe,
            friend = []
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_error_configuration() {
        let error =
            syn::parse_str::<Cfg>("konst = none, error = (), error = ()")
                .expect_err("error configuration may occur only once");

        assert_eq!(error.to_string(), "duplicated arg");
    }

    #[test]
    fn rejects_non_unit_error_configuration() {
        let error = syn::parse_str::<Cfg>("konst = none, error = TextError")
            .expect_err("only unit validation errors are reserved");

        assert_eq!(
            error.to_string(),
            "only `()` is currently supported for `error`",
        );
    }

    #[test]
    fn rejects_duplicate_literal_membership() {
        let input = r#"domain = {"draft", "draft"}"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_equivalent_duplicate_literal_membership() {
        let input = r#"domain = {"é", "\u{e9}"}"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn defaults_owned_text_konst_to_nightly_feature() {
        let input = "valid = is_slug";
        let parsed = syn::parse_str::<Cfg>(input);
        assert!(parsed.is_ok(), "{input}");
        let config = parsed.unwrap();

        assert_eq!(config.konst, crate::zz_lib::Konst::default(), "{input}");
        assert_eq!(
            config.konst_for(&[Konsts::Text]),
            cfg!(feature = "nightly"),
            "{input}",
        );
    }

    #[test]
    fn stable_konst_keeps_owned_text_operations_non_const() {
        let input = "konst = stable";
        let parsed = syn::parse_str::<Cfg>(input);
        assert!(parsed.is_ok(), "{input}");
        let config = parsed.unwrap();

        assert!(!config.konst_for(&[Konsts::Text]), "{input}");
    }

    #[test]
    fn rejects_boolean_konst_modes() {
        let error = syn::parse_str::<Cfg>("konst = false")
            .expect_err("boolean konst values are no longer accepted");
        assert_eq!(
            error.to_string(),
            "expected `none`, `stable`, or `nightly`",
        );
    }

    #[test]
    fn rejects_integral_only_const_flag() {
        let result = syn::parse_str::<Cfg>("with = konst_add");
        let error = result.unwrap_err();

        assert_eq!(error.to_string(), "unknown flag");
    }

    #[test]
    fn specific_const_disables_take_precedence_over_text_families() {
        let input = "
            konst = none,
            with = [konst_text, konst_from],
            without = [konst_text_access, konst_text_from]
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        assert!(parsed.is_ok(), "{input}");
        let config = parsed.unwrap();

        assert!(
            !config.konst_for(&[
                Konsts::TextAccess,
                Konsts::Text
            ]),
            "{input}",
        );
        assert!(
            !config.konst_for(&[
                Konsts::TextFrom,
                Konsts::From,
                Konsts::Text,
            ]),
            "{input}",
        );
        assert!(
            config.konst_for(&[
                Konsts::From,
                Konsts::Text
            ]),
            "{input}"
        );
    }

    #[test]
    fn conflicting_const_flags_report_the_second_setting() {
        let input = "without = konst_text_access, with = konst_text_access";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "flag specified in both `with` and `without`",
            "{input}",
        );
        assert_eq!(
            error.span().start().column,
            input.rfind("konst_text_access").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn unknown_flag_precedes_a_later_duplicate() {
        let input = "with = [not_a_flag, display, display]";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(error.to_string(), "unknown flag", "{input}");
        assert_eq!(
            error.span().start().column,
            input.find("not_a_flag").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn function_removal_rejects_with() {
        let input = "with = fn_len";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "`fn_len` only removes the generated `len`; write `without = \
            fn_len`",
            "{input}",
        );
    }

    #[test]
    fn removing_an_ungenerated_function_reports_its_flag() {
        let input = "without = [fn_len, fn_floor]";
        let source = "struct Name(String);";
        let config = syn::parse_str::<Cfg>(input);
        assert!(config.is_ok(), "{input}");
        let item = syn::parse_str::<syn::ItemStruct>(source);
        assert!(item.is_ok(), "{source}");
        let expansion = crate::text::ekran(config.unwrap(), item.unwrap());
        let error = expansion.unwrap_err();

        assert_eq!(
            error.to_string(),
            "`fn_floor` removes nothing: no generated function `floor`",
            "{input}",
        );
        assert_eq!(
            error.span().start().column,
            input.find("fn_floor").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn rejects_callbacks_and_literal_membership() {
        let input = r#"konst = none, valid = is_slug, domain = {"draft"}"#;
        assert!(
            syn::parse_str::<Cfg>(input)
                .expect_err("validation modes must be exclusive")
                .to_string()
                .contains("`valid` and `domain` cannot be used together"),
            "{input}",
        );
    }

    #[test]
    fn rejects_literal_membership_and_callbacks() {
        let input = r#"konst = none, domain = {"draft"}, valid = is_slug"#;
        assert!(
            syn::parse_str::<Cfg>(input)
                .expect_err("validation modes must be exclusive")
                .to_string()
                .contains("`valid` and `domain` cannot be used together"),
            "{input}",
        );
    }

    #[test]
    fn rejects_unbraced_literal_membership() {
        let input = r#"domain = "draft""#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_expression_literal_membership() {
        let input = r#"domain = {concat!("dra", "ft")}"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_string_literal_membership() {
        let input = r#"domain = {"draft", 1}"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_string_validation_callback() {
        let input = r#"valid = "is_slug""#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_path_validation_callback_in_list() {
        let input = r#"valid = [is_slug, "is_not_reserved"]"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_closure_validation_callback() {
        let input = "valid = |value| true";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_implicit_make_friend() {
        let input = "friend = [_(Source) -> Make]";
        syn::parse_str::<Cfg>(input)
            .expect_err("Make friends must provide an owning conversion");
    }

    #[test]
    fn rejects_unknown_attribute() {
        let input = "unknown = true";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_deref_mut_flag() {
        let input = "with = [deref_mut]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_implicit_rel_friend_capability() {
        let input = "friend = [_(Source) -> [Rel]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_math_friend_capability() {
        let input = "friend = [convert(Source) -> Math]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn duplicate_flags_report_the_repeated_identifier() {
        let input = "with = [display, display, not_a_flag]";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(error.to_string(), "duplicated flags", "{input}");
        assert_eq!(
            error.span().start().column,
            input.rfind("display").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn rejects_duplicate_make_friends_for_source() {
        let input = "friend = [first(Source) -> [Make], second(Source) -> [Make, unsafe]]";
        let parsed = syn::parse_str::<Cfg>(input);
        assert!(parsed.is_err(), "{input}");
        let error = parsed.unwrap_err();

        assert_eq!(error.to_string(), "duplicated text friend", "{input}");
        assert_eq!(
            error.span().start().column,
            input.rfind("Source").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn rejects_unsafe_friend_without_conversion() {
        let input = "konst = none, friend = [_(Source) -> unsafe]";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();
        assert_eq!(
            error.to_string(),
            "text Make friend requires `conversion(Type) -> Make`",
            "{input}"
        );
    }

    #[test]
    fn rejects_untyped_make_friend() {
        let input = "friend = [_ -> Make]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }
}
