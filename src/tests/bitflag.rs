#[cfg(test)]
mod tests {
    use crate::bitflag::Cfg;

    #[test]
    fn accepts_single_root_and_nested_friends_friends_crate_convert_crate_source_source_in_1()
     {
        let input = "friends = crate::convert(crate::source::Source) -> Inspector, konst = false";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_root_and_nested_friends_konst_false_friends_crate_convert_crate_sour_2()
     {
        let input = "konst = false, friends = crate::convert(crate::source::Source) -> [Bit, unsafe], integral = []";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_root_and_nested_friends_konst_false_integral_friends_foo_crate_sourc_3()
     {
        let input = "konst = false, integral = [], friends = Foo(crate::source::Source) -> Bit";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_root_and_nested_friends_konst_false_integral_friends_crate_convert_c_4()
     {
        let input = "konst = false, integral = [friends = crate::convert(crate::source::Source) -> Make, valid = is_valid]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_root_and_nested_friends_konst_false_integral_valid_is_valid_friends_case_5()
     {
        let input = "konst = false, integral = [valid = is_valid, friends = Foo(crate::source::Source) -> [Math, Bit]], suffix = \"Value\"";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_friends_at_each_scope() {
        for input in [
            "konst = false, friends = [], integral = [friends = [], valid = is_valid]",
        ] {
            syn::parse_str::<Cfg>(input).expect(input);
        }
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_friends_convert_source_bit_convert_other_bit_1()
     {
        let attributes =
            "friends = convert(Source) -> Bit, convert(Other) -> Bit";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_integral_friends_source_math_other_math_2()
     {
        let attributes =
            "integral = [friends = _(Source) -> Math, _(Other) -> Math]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_friends_3() {
        let attributes = "friends =";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_integral_friends_4()
     {
        let attributes = "integral = [friends =]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_friends_integral_5()
     {
        let attributes = "friends =, integral = []";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_integral_friends_valid_is_valid_6()
     {
        let attributes = "integral = [friends =, valid = is_valid]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_friends_convert_source_bit_1_7()
     {
        let attributes = "friends = convert(Source) -> [Bit, 1]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_integral_friends_source_inspector_8()
     {
        let attributes = "integral = [friends = _(Source) -> Inspector]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_friends_source_bit_unknown_true_9()
     {
        let attributes = "friends = _(Source) -> Bit, unknown = true";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_integral_friends_source_math_bit_unknown_tru_10()
     {
        let attributes =
            "integral = [friends = _(Source) -> [Math, Bit], unknown = true]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_friends_source_bit_friends_11()
     {
        let attributes = "friends = _(Source) -> Bit, friends = []";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_integral_unknown_true_12()
     {
        let attributes = "integral = [unknown = true]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn accepts_top_level_konst_values() {
        let config = syn::parse_str::<Cfg>("konst = true")
            .expect("top-level true konst should parse");
        assert_eq!(config.int.konst, crate::integral::Konst::Nightly);

        let config = syn::parse_str::<Cfg>("konst = false")
            .expect("top-level false konst should parse");
        assert_eq!(config.int.konst, crate::integral::Konst::Nightly);
    }

    #[test]
    fn rejects_missing_konst() {
        let error = match syn::parse_str::<Cfg>("friends = [Foo(u8) -> Bit]") {
            Ok(_) => panic!("konst must be explicit"),
            Err(error) => error,
        };
        assert_eq!(error.to_string(), "missing required `konst` argument",);
    }

    #[test]
    fn rejects_friendship_trait_configuration() {
        let error = match syn::parse_str::<Cfg>(
            "with = [trait_seal], integral = [konst = false]",
        ) {
            Ok(_) => panic!("bitflag must delegate friendship trait names"),
            Err(error) => error,
        };

        assert_eq!(error.to_string(), "unknown flag");
    }

    #[test]
    fn accepts_friend_capabilities() {
        let config = syn::parse_str::<Cfg>(
            "konst = false, friends = [Foo(u8) -> [Bit]], integral = []",
        )
        .expect("bitflag capabilities should parse");
        let friend = config.friends.iter().next().unwrap();
        assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));
    }

    #[test]
    fn rejects_legacy_friend_levels() {
        assert!(syn::parse_str::<Cfg>(
            "konst = false, friends = [u8(conv = self, level = [Bit])], integral = []",
        )
            .is_err());
    }

    #[test]
    fn accepts_trusted_root_and_nested_capability_without_make_friends_foo_source_trust_1()
     {
        let attributes = "friends = [Foo(Source) -> unsafe]";
        let input = format!("konst = false, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_root_and_nested_capability_without_make_friends_foo_source_trust_2()
     {
        let attributes = "friends = [Foo(Source) -> [unsafe]]";
        let input = format!("konst = false, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_root_and_nested_capability_without_make_friends_foo_source_bit_trust_3()
     {
        let attributes = "friends = [Foo(Source) -> [Bit, unsafe]]";
        let input = format!("konst = false, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_root_and_nested_capability_without_make_integral_friends_foo_source_trust_4()
     {
        let attributes = "integral = [friends = [Foo(Source) -> unsafe]]";
        let input = format!("konst = false, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_root_and_nested_capability_without_make_integral_friends_foo_source_trust_5()
     {
        let attributes = "integral = [friends = [Foo(Source) -> [unsafe]]]";
        let input = format!("konst = false, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_root_and_nested_capability_without_make_integral_friends_foo_source_math_bit_trust_6()
     {
        let attributes =
            "integral = [friends = [Foo(Source) -> [Math, Bit, unsafe]]]";
        let input = format!("konst = false, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_root_marker_capabilities_without_narrowing_integral_vocabulary()
    {
        let config = syn::parse_str::<Cfg>(
            "konst = false, friends = [convert(Source) -> Inspector], integral = [friends = [Foo(Other) -> Math]]",
        )
            .expect("bitflag root capabilities may name arbitrary markers");
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.capabilities.contains(&syn::parse_quote!(Inspector)));
        assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
        assert!(friend.ty.is_ident("Source"));
    }

    #[test]
    fn rejects_legacy_trusted_argument_at_root_and_nested_scope_true_1_format_friends_source_cap_ma_1()
     {
        let trusted = "true";
        let attributes =
            format!("friends = [Source(cap = Make, trusted = {trusted})]");
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_legacy_trusted_argument_at_root_and_nested_scope_true_1_format_integral_friends_sour_2()
     {
        let trusted = "true";
        let attributes = format!(
            "integral = [friends = [Source(cap = Make, trusted = {trusted})]]"
        );
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_legacy_trusted_argument_at_root_and_nested_scope_false_2_format_friends_source_cap_ma_1()
     {
        let trusted = "false";
        let attributes =
            format!("friends = [Source(cap = Make, trusted = {trusted})]");
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_legacy_trusted_argument_at_root_and_nested_scope_false_2_format_integral_friends_sour_2()
     {
        let trusted = "false";
        let attributes = format!(
            "integral = [friends = [Source(cap = Make, trusted = {trusted})]]"
        );
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_with_display_display_1()
     {
        let input = "konst = false, with = [display, display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_without_display_display_2()
     {
        let input = "konst = false, without = [display, display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_with_display_with_display_3()
     {
        let input = "konst = false, with = [display], with = [display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_without_display_without_display_4()
     {
        let input = "konst = false, without = [display], without = [display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_integral_with_display_display_5()
     {
        let input = "konst = false, integral = [with = [display, display]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_integral_without_display_display_6()
     {
        let input = "konst = false, integral = [without = [display, display]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_integral_with_display_with_displ_7()
     {
        let input =
            "konst = false, integral = [with = [display], with = [display]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_integral_without_display_without_8()
     {
        let input = "konst = false, integral = [without = [display], without = [display]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_root_and_nested_display_flags_konst_false_with_display_display_names_1()
     {
        let input = "konst = false, with = [display, display_names]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_root_and_nested_display_flags_konst_false_without_display_names_2()
     {
        let input = "konst = false, without = [display_names]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_root_and_nested_display_flags_konst_false_integral_with_display_display_na_3()
     {
        let input =
            "konst = false, integral = [with = [display, display_names]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_root_and_nested_display_flags_konst_false_integral_without_display_names_4()
     {
        let input = "konst = false, integral = [without = [display_names]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_with_display_without_display_1()
     {
        let attributes = "with = [display], without = [display]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_without_display_with_display_2()
     {
        let attributes = "without = [display], with = [display]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_with_display_make_value_without_make_value_i_3()
     {
        let attributes =
            "with = [display, make_value], without = [make_value, impl_value]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_without_make_value_impl_value_with_display_m_4()
     {
        let attributes =
            "without = [make_value, impl_value], with = [display, make_value]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_integral_with_display_without_display_5()
     {
        let attributes = "integral = [with = [display], without = [display]]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_integral_without_display_with_display_6()
     {
        let attributes = "integral = [without = [display], with = [display]]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_integral_with_display_impl_math_add_without_case_7()
     {
        let attributes = "integral = [with = [display, impl_math_add], without = [impl_math_add, fn_conv_raw]]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_integral_without_impl_math_add_fn_conv_raw_w_8()
     {
        let attributes = "integral = [without = [impl_math_add, fn_conv_raw], with = [display, impl_math_add]]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn opposite_display_settings_in_distinct_scopes_are_legal_with_display_integral_without_display_1()
     {
        let attributes = "with = [display], integral = [without = [display]]";
        let input = format!("konst = false, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn opposite_display_settings_in_distinct_scopes_are_legal_integral_without_display_with_display_2()
     {
        let attributes = "integral = [without = [display]], with = [display]";
        let input = format!("konst = false, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn opposite_display_settings_in_distinct_scopes_are_legal_without_display_integral_with_display_3()
     {
        let attributes = "without = [display], integral = [with = [display]]";
        let input = format!("konst = false, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn opposite_display_settings_in_distinct_scopes_are_legal_integral_with_display_without_display_4()
     {
        let attributes = "integral = [with = [display]], without = [display]";
        let input = format!("konst = false, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn rejects_unsupported_nested_single_capabilities_integral_friends_foo_source_relation_1()
     {
        let attributes = "integral = [friends = [Foo(Source) -> Relation]]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_nested_single_capabilities_integral_friends_foo_source_numeric_2()
     {
        let attributes = "integral = [friends = [Foo(Source) -> Numeric]]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_nested_single_capabilities_integral_friends_foo_source_inspector_3()
     {
        let attributes = "integral = [friends = [Foo(Source) -> Inspector]]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_nested_single_capabilities_integral_friends_source_relation_4()
     {
        let attributes = "integral = [friends = [_(Source) -> Relation]]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_nested_single_capabilities_integral_friends_source_numeric_5()
     {
        let attributes = "integral = [friends = [_(Source) -> Numeric]]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_nested_single_capabilities_integral_friends_source_inspector_6()
     {
        let attributes = "integral = [friends = [_(Source) -> Inspector]]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_root_capability_value_syntax_friends_foo_source_bit_1() {
        let attributes = "friends = [Foo(Source) -> \"Bit\"]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_root_capability_value_syntax_friends_foo_source_bit_1_2()
    {
        let attributes = "friends = [Foo(Source) -> [Bit, 1]]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_root_capability_value_syntax_friends_foo_source_bit_3() {
        let attributes = "friends = [Foo(Source) -> {Bit}]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_root_capability_value_syntax_friends_source_bit_4() {
        let attributes = "friends = [_(Source) -> \"Bit\"]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_root_capability_value_syntax_friends_source_bit_1_5() {
        let attributes = "friends = [_(Source) -> [Bit, 1]]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_root_capability_value_syntax_friends_source_bit_6() {
        let attributes = "friends = [_(Source) -> {Bit}]";
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }
}
