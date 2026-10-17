#[cfg(test)]
mod tests {
    use crate::text::Cfg;

    #[test]
    fn accepts_single_friend_without_consuming_neighboring_attributes_friends_crate_convert_crate_source_source_ma_1()
     {
        let input = "friends = crate::convert(crate::source::Source) -> Make, konst = none";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_friend_without_consuming_neighboring_attributes_konst_false_friends_crate_convert_crate_sour_2()
     {
        let input = "konst = none, friends = crate::convert(crate::source::Source) -> [Make, unsafe], valid = is_slug";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_friend_without_consuming_neighboring_attributes_konst_false_valid_is_slug_friends_foo_crate_case_3()
     {
        let input = "konst = none, valid = is_slug, friends = Foo(crate::source::Source) -> unsafe";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_friend_without_consuming_neighboring_attributes_konst_false_friends_foo_self_rel_trust_with_case_4()
     {
        let input = "konst = none, friends = Foo(Self) -> [Rel, unsafe], with = [display]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_friend_lists_konst_false_friends_convert_source_make_trus_1()
     {
        let input = "konst = none, friends = [convert(Source) -> [Make, unsafe], Foo(Other) -> unsafe], valid = is_slug";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_friend_lists_konst_false_friends_valid_is_slug_2()
     {
        let input = "konst = none, friends = [], valid = is_slug";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_convert_source_make_convert_other_ma_1()
     {
        let input = "friends = convert(Source) -> Make, convert(Other) -> Make";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_source_trust_other_trust_2()
     {
        let input = "friends = _(Source) -> unsafe, _(Other) -> unsafe";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_3()
     {
        let input = "friends =";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_valid_is_slug_4()
     {
        let input = "friends =, valid = is_slug";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_convert_source_5()
     {
        let input = "friends = convert(Source)";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_convert_make_6()
     {
        let input = "friends = convert() -> Make";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_convert_source_7()
     {
        let input = "friends = convert(Source) ->";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_convert_source_make_1_8()
     {
        let input = "friends = convert(Source) -> [Make, 1]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_source_make_9()
     {
        let input = "friends = _(Source) -> Make";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_source_make_trust_10()
     {
        let input = "friends = _(Source) -> [Make, unsafe]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_trust_11()
     {
        let input = "friends = _ -> unsafe";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_rel_trust_12()
     {
        let input = "friends = _ -> [Rel, unsafe]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_convert_source_math_13()
     {
        let input = "friends = convert(Source) -> Math";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_source_trust_unknown_true_14()
     {
        let input = "friends = _(Source) -> [unsafe], unknown = true";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities_friends_source_trust_friends_15()
     {
        let input = "friends = _(Source) -> unsafe, friends = []";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn accepts_callback_validation_and_trusted_make_friend() {
        let cfg = syn::parse_str::<Cfg>(
            "konst = none, valid = [is_slug, is_published], friends = [Source::into_string(Source) -> [Make, unsafe]]",
        )
        .expect("text configuration should parse");

        assert_eq!(cfg.konst, crate::integral::Konst::None);
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
    fn accepts_single_callback() {
        let cfg = syn::parse_str::<Cfg>("konst = none, valid = is_slug")
            .expect("a single text callback should parse");

        assert_eq!(cfg.callbacks.len(), 1);
    }

    #[test]
    fn rejects_duplicate_literal_membership_in_draft_draft_1() {
        let input = r#"in = ["draft", "draft"]"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_literal_membership_in_u_e9_2() {
        let input = r#"in = ["é", "\u{e9}"]"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_literal_membership_in_draft_draft_3() {
        let input = r##"in = ["draft", r#"draft"#]"##;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn requires_konst() {
        assert!(
            syn::parse_str::<Cfg>("valid = is_slug")
                .expect_err("konst must be explicit")
                .to_string()
                .contains("missing required `konst` argument")
        );
    }

    #[test]
    fn accepts_integral_style_konst_modes() {
        let stable = syn::parse_str::<Cfg>("konst = stable")
            .expect("stable konst should parse");
        assert!(stable.konst_for(&["text_access"]));
        assert!(!stable.konst_for(&["text"]));

        let nightly = syn::parse_str::<Cfg>("konst = nightly")
            .expect("nightly konst should parse");
        assert!(nightly.konst_for(&["text"]));
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
    fn rejects_combined_validation_modes_konst_false_valid_is_slug_in_draft_1()
    {
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
    fn rejects_combined_validation_modes_konst_false_in_draft_valid_is_slug_2()
    {
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
    fn rejects_non_literal_membership_in_draft_1() {
        let input = r#"in = "draft""#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_literal_membership_in_draft_2() {
        let input = r#"in = [DRAFT]"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_literal_membership_in_concat_dra_ft_3() {
        let input = r#"in = [concat!("dra", "ft")]"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_literal_membership_in_draft_1_4() {
        let input = r#"in = ["draft", 1]"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_literal_membership_in_b_draft_5() {
        let input = r#"in = [b"draft"]"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_literal_membership_in_1_3_8_10_6() {
        let input = r#"in = 1..=3 + 8..10"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_path_validation_callbacks_valid_is_slug_1() {
        let input = r#"valid = "is_slug""#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_path_validation_callbacks_valid_is_slug_is_not_reserved_2() {
        let input = r#"valid = [is_slug, "is_not_reserved"]"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_non_path_validation_callbacks_valid_value_true_3() {
        let input = "valid = |value| true";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_friend_configuration_friends_source_make_1() {
        let input = "friends = [_(Source) -> [Make]]";
        syn::parse_str::<Cfg>(input)
            .expect_err("Make friends must provide an owning conversion");
    }

    #[test]
    fn rejects_invalid_friend_configuration_friends_source_make_2() {
        let input = "friends = [_(Source) -> Make]";
        syn::parse_str::<Cfg>(input)
            .expect_err("Make friends must provide an owning conversion");
    }

    #[test]
    fn rejects_invalid_friend_configuration_friends_source_make_trust_3() {
        let input = "friends = [_(Source) -> [Make, unsafe]]";
        syn::parse_str::<Cfg>(input)
            .expect_err("Make friends must provide an owning conversion");
    }

    #[test]
    fn rejects_unknown_text_attributes_flags_and_capabilities_unknown_true_1() {
        let input = "unknown = true";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_text_attributes_flags_and_capabilities_get_raw_self_raw_2()
     {
        let input = "get_raw = Self::raw";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_text_attributes_flags_and_capabilities_with_deref_mut_3()
    {
        let input = "with = [deref_mut]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_text_attributes_flags_and_capabilities_without_impl_math_add_4()
     {
        let input = "without = [impl_math_add]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_text_attributes_flags_and_capabilities_friends_convert_source_math_5()
     {
        let input = "friends = [convert(Source) -> [Math]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_text_attributes_flags_and_capabilities_friends_convert_source_bit_6()
     {
        let input = "friends = [convert(Source) -> [Bit]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_text_attributes_flags_and_capabilities_friends_convert_source_relation_7()
     {
        let input = "friends = [convert(Source) -> [Relation]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_text_attributes_flags_and_capabilities_friends_source_rel_8()
     {
        let input = "friends = [_(Source) -> [Rel]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_text_attributes_flags_and_capabilities_friends_convert_source_math_9()
     {
        let input = "friends = [convert(Source) -> Math]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_text_attributes_flags_and_capabilities_friends_convert_source_bit_10()
     {
        let input = "friends = [convert(Source) -> Bit]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_text_attributes_flags_and_capabilities_friends_convert_source_relation_11()
     {
        let input = "friends = [convert(Source) -> Relation]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_configuration_keys_konst_false_konst_false_1() {
        let input = "konst = none, konst = none";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_configuration_keys_with_display_with_display_2() {
        let input = "with = [display], with = [display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_configuration_keys_without_display_without_display_3()
    {
        let input = "without = [display], without = [display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_configuration_keys_valid_is_slug_valid_is_slug_4() {
        let input = "valid = is_slug, valid = is_slug";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_configuration_keys_in_draft_in_draft_5() {
        let input = r#"in = ["draft"], in = ["draft"]"#;
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_configuration_keys_friends_friends_6() {
        let input = "friends = [], friends = []";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_generation_flags_and_friend_types_with_display_display_1()
     {
        let input = "with = [display, display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_generation_flags_and_friend_types_without_display_display_2()
     {
        let input = "without = [display, display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_generation_flags_and_friend_types_friends_self_rel_self_rel_3()
     {
        let input = "friends = [_(Self) -> [Rel], _(Self) -> [Rel]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_generation_flags_and_friend_types_friends_first_source_make_second_source_make_tru_4()
     {
        let input = "friends = [first(Source) -> [Make], \
            second(Source) -> [Make, unsafe]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn accepts_trusted_capability_without_make_or_conversion_konst_false_friends_self_self_rel_trust_1()
     {
        let input = "konst = none, friends = [Self(Self) -> [Rel, unsafe]]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_trusted_capability_without_make_or_conversion_konst_false_friends_self_self_trust_2()
     {
        let input = "konst = none, friends = [Self(Self) -> unsafe]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_trusted_capability_without_make_or_conversion_konst_false_friends_self_self_trust_3()
     {
        let input = "konst = none, friends = [Self(Self) -> [unsafe]]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_trusted_capability_without_make_or_conversion_konst_false_friends_self_source_trust_4()
     {
        let input = "konst = none, friends = [Self(Source) -> unsafe]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_trusted_capability_without_make_or_conversion_konst_false_friends_self_source_trust_5()
     {
        let input = "konst = none, friends = [Self(Source) -> [unsafe]]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_self_relation_with_implicit_conversion_konst_false_friends_self_self_rel_1()
     {
        let input = "konst = none, friends = [Self(Self) -> Rel]";
        let config = syn::parse_str::<Cfg>(input).expect(input);
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.ty.is_ident("Self"));
        assert!(friend.conv.is_none());
        assert!(friend.capabilities.contains(&syn::parse_quote!(Rel)));
    }

    #[test]
    fn accepts_self_relation_with_implicit_conversion_konst_false_friends_self_self_rel_2()
     {
        let input = "konst = none, friends = [Self(Self) -> [Rel]]";
        let config = syn::parse_str::<Cfg>(input).expect(input);
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.ty.is_ident("Self"));
        assert!(friend.conv.is_none());
        assert!(friend.capabilities.contains(&syn::parse_quote!(Rel)));
    }

    #[test]
    fn accepts_self_relation_with_implicit_conversion_konst_false_friends_self_self_rel_trust_3()
     {
        let input = "konst = none, friends = [Self(Self,) -> [Rel, unsafe]]";
        let config = syn::parse_str::<Cfg>(input).expect(input);
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.ty.is_ident("Self"));
        assert!(friend.conv.is_none());
        assert!(friend.capabilities.contains(&syn::parse_quote!(Rel)));
    }

    #[test]
    fn rejects_untyped_friend_declarations_friends_make_1() {
        let input = "friends = [_ -> Make]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_untyped_friend_declarations_friends_rel_2() {
        let input = "friends = [_ -> Rel]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_untyped_friend_declarations_friends_trust_3() {
        let input = "friends = [_ -> unsafe]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_untyped_friend_declarations_friends_rel_trust_4() {
        let input = "friends = [_ -> [Rel, unsafe]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_untyped_friend_declarations_friends_5() {
        let input = "friends = [_ -> []]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_legacy_trusted_argument_true_1() {
        let trusted = "true";
        let input = format!(
            "friends = [Source(conv = convert, cap = Make, trusted = {trusted})]"
        );
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_legacy_trusted_argument_false_2() {
        let trusted = "false";
        let input = format!(
            "friends = [Source(conv = convert, cap = Make, trusted = {trusted})]"
        );
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_generation_flags_in_either_order_with_display_without_display_1()
     {
        let input = "with = [display], without = [display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_generation_flags_in_either_order_without_display_with_display_2()
     {
        let input = "without = [display], with = [display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }
}
