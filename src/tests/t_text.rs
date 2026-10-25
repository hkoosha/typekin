#[cfg(test)]
mod tests {
    use crate::text::Cfg;
    use crate::zz::Konsts;

    #[test]
    fn accepts_friend_with_make_capability() {
        let input = "
            friends = crate::convert(crate::source::Source) -> Make,
            konst = none
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_friend_with_make_and_unsafe_capabilities() {
        let input = "
            konst = none,
            friends = crate::convert(crate::source::Source) -> [Make, unsafe],
            valid = is_slug
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_friend_with_custom_conversion() {
        let input = "
            konst = none,
            valid = is_slug,
            friends = Foo(crate::source::Source) -> unsafe
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_self_friend_with_rel_and_unsafe_capabilities() {
        let input = "
            konst = none,
            friends = Foo(Self) -> [Rel, unsafe],
            with = [display]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_multiple_friends() {
        let input = "
            konst = none,
            friends = [convert(Source) -> [Make, unsafe], Foo(Other) -> unsafe],
            valid = is_slug
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_empty_friends() {
        let input = "
            konst = none,
            friends = [],
            valid = is_slug
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn rejects_multiple_single_explicit_friends() {
        let input = "
            friends = convert(Source) -> Make,
            convert(Other) -> Make
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_multiple_single_implicit_friends() {
        let input = "
            friends = _(Source) -> unsafe,
            _(Other) -> unsafe
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_missing_single_friend() {
        let input = "
            friends =
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_missing_single_friend_before_validation() {
        let input = "
            friends =,
            valid = is_slug
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn accepts_friend_without_capabilities_with_default_konst() {
        let input = "
            friends = convert(Source)
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn rejects_conversion_without_source() {
        let input = "
            friends = convert() -> Make
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_conversion_without_capabilities() {
        let input = "
            friends = convert(Source) ->
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_non_identifier_friend_capability() {
        let input = "
            friends = convert(Source) -> [Make, 1]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_attribute_after_implicit_friend() {
        let input = "
            friends = _(Source) -> [unsafe],
            unknown = true
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_friends_after_implicit_friend() {
        let input = "
            friends = _(Source) -> unsafe,
            friends = []
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn accepts_callback_validation_and_trusted_make_friend() {
        let cfg = syn::parse_str::<Cfg>(
            "konst = none, valid = [is_slug, is_published], friends = [Source::into_string(Source) -> [Make, unsafe]]",
        )
        .expect("text configuration should parse");

        assert_eq!(cfg.konst, crate::zz::Konst::None);
        assert_eq!(cfg.callbacks.len(), 2);
        assert!(cfg.flags.display);
    }

    #[test]
    fn accepts_literal_membership_validation() {
        let cfg = syn::parse_str::<Cfg>(
            r#"konst = none, in = ["draft", "published"]"#,
        )
        .expect("literal membership configuration should parse");
        assert_eq!(cfg.values.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn accepts_serde_and_unit_error_configuration() {
        let cfg =
            syn::parse_str::<Cfg>("konst = none, error = (), with = [serde]")
                .expect("new text options should parse");

        assert!(cfg.flags.serde);
    }

    #[test]
    fn accepts_disabling_serde() {
        let cfg = syn::parse_str::<Cfg>("konst = none, without = [serde]")
            .expect("serde generation should be independently disabled");

        assert!(!cfg.flags.serde);
    }

    #[test]
    fn accepts_one_generation_flag_without_brackets() {
        let input = "konst = none, with = display";
        let config = syn::parse_str::<Cfg>(input).expect(input);
        assert!(config.flags.display);

        let input = "konst = none, without = display";
        let config = syn::parse_str::<Cfg>(input).expect(input);
        assert!(!config.flags.display);
    }

    #[test]
    fn defaults_layout_and_traits_and_allows_disabling_them() {
        let defaults = syn::parse_str::<Cfg>("konst = none")
            .expect("default text configuration should parse");
        assert!(defaults.flags.clone);
        assert!(defaults.flags.debug);
        assert!(defaults.flags.hash);
        assert!(defaults.flags.repr);

        let enabled = syn::parse_str::<Cfg>(
            "konst = none, with = [clone, debug, hash, repr]",
        )
        .expect("layout and traits should be configurable");
        assert!(enabled.flags.clone);
        assert!(enabled.flags.debug);
        assert!(enabled.flags.hash);
        assert!(enabled.flags.repr);

        let disabled = syn::parse_str::<Cfg>(
            "konst = none, without = [clone, debug, hash, repr]",
        )
        .expect("layout and traits should be configurable");
        assert!(!disabled.flags.clone);
        assert!(!disabled.flags.debug);
        assert!(!disabled.flags.hash);
        assert!(!disabled.flags.repr);
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
    fn accepts_single_callback() {
        let cfg = syn::parse_str::<Cfg>("konst = none, valid = is_slug")
            .expect("a single text callback should parse");

        assert_eq!(cfg.callbacks.len(), 1);
    }

    #[test]
    fn rejects_duplicate_literal_membership() {
        let input = r#"in = ["draft", "draft"]"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_equivalent_duplicate_literal_membership() {
        let input = r#"in = ["é", "\u{e9}"]"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn defaults_missing_konst_to_crate_default() {
        let cfg = syn::parse_str::<Cfg>("valid = is_slug")
            .expect("konst should default to the crate feature");

        assert_eq!(cfg.konst, crate::zz::Konst::default());
        assert!(cfg.konst_for(&[Konsts::TextAccess]));
        assert_eq!(cfg.konst_for(&[Konsts::Text]), cfg!(feature = "nightly"));
    }

    #[test]
    fn accepts_integral_style_konst_modes() {
        let stable = syn::parse_str::<Cfg>("konst = stable")
            .expect("stable konst should parse");
        assert!(stable.konst_for(&[Konsts::TextAccess]));
        assert!(!stable.konst_for(&[Konsts::Text]));

        let nightly = syn::parse_str::<Cfg>("konst = nightly")
            .expect("nightly konst should parse");
        assert!(nightly.konst_for(&[Konsts::Text]));
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
    fn rejects_callbacks_and_literal_membership() {
        let input = r#"konst = none, valid = is_slug, in = ["draft"]"#;
        assert!(
            syn::parse_str::<Cfg>(input)
                .expect_err("validation modes must be exclusive")
                .to_string()
                .contains("`valid` and `in` cannot be used together"),
            "{input}",
        );
    }

    #[test]
    fn rejects_literal_membership_and_callbacks() {
        let input = r#"konst = none, in = ["draft"], valid = is_slug"#;
        assert!(
            syn::parse_str::<Cfg>(input)
                .expect_err("validation modes must be exclusive")
                .to_string()
                .contains("`valid` and `in` cannot be used together"),
            "{input}",
        );
    }

    #[test]
    fn rejects_unbracketed_literal_membership() {
        let input = r#"in = "draft""#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_expression_literal_membership() {
        let input = r#"in = [concat!("dra", "ft")]"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_string_literal_membership() {
        let input = r#"in = ["draft", 1]"#;
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
        let input = "friends = [_(Source) -> Make]";
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
        let input = "friends = [_(Source) -> [Rel]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_math_friend_capability() {
        let input = "friends = [convert(Source) -> Math]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_with_display_flags() {
        let input = "with = [display, display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_make_friends_for_source() {
        let input = "friends = [first(Source) -> [Make], \
            second(Source) -> [Make, unsafe]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsafe_friend_without_conversion() {
        let input = "konst = none, friends = [_(Source) -> unsafe]";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();
        assert_eq!(
            error.to_string(),
            "text Make friend requires `conversion(Type) -> Make`",
            "{input}"
        );
    }

    #[test]
    fn accepts_self_rel_friend() {
        let input = "konst = none, friends = [_(Self) -> Rel]";
        let config = syn::parse_str::<Cfg>(input).expect(input);
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.ty.is_ident("Self"));
        assert!(friend.conv.is_none());
        assert!(friend.capabilities.contains(&syn::parse_quote!(Rel)));
    }

    #[test]
    fn accepts_self_rel_and_unsafe_friend_with_trailing_comma() {
        let input = "
            konst = none,
            friends = [into_string(Self,) -> [Rel, unsafe]]
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        assert!(parsed.is_ok(), "{input}");
        let config = parsed.unwrap();
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.ty.is_ident("Self"));
        assert!(friend.conv.as_ref().unwrap().is_ident("into_string"));
        assert!(friend.capabilities.contains(&syn::parse_quote!(Rel)));
        assert!(friend.capabilities.contains(&syn::parse_quote!(Make)));
    }

    #[test]
    fn rejects_untyped_make_friend() {
        let input = "friends = [_ -> Make]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_legacy_trusted_true_argument() {
        let trusted = "true";
        let input = format!(
            "friends = [Source(conv = convert, cap = Make, trusted = {trusted})]"
        );
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }
}
