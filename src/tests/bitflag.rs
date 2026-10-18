#[cfg(test)]
mod tests {
    use crate::bitflag::{
        Cfg,
        resolve_repr,
    };
    use crate::zz_value_type::N;

    #[test]
    fn accepts_single_root_and_nested_friends_friends_crate_convert_crate_source_source_in_1()
     {
        let input = "friends = crate::convert(crate::source::Source) -> Inspector, konst = none";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_root_and_nested_friends_konst_false_friends_crate_convert_crate_sour_2()
     {
        let input = "konst = none, friends = crate::convert(crate::source::Source) -> [Bit, unsafe], integral = []";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_root_and_nested_friends_konst_false_integral_friends_foo_crate_sourc_3()
     {
        let input = "konst = none, integral = [], friends = Foo(crate::source::Source) -> Bit";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_root_and_nested_friends_konst_false_integral_friends_crate_convert_c_4()
     {
        let input = "konst = none, integral = [friends = crate::convert(crate::source::Source) -> Make, valid = is_valid]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_root_and_nested_friends_konst_false_integral_valid_is_valid_friends_case_5()
     {
        let input = "konst = none, integral = [valid = is_valid, friends = Foo(crate::source::Source) -> [Math, Bit]], suffix = \"Value\"";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_friends_at_each_scope() {
        for input in [
            "konst = none, friends = [], integral = [friends = [], valid = is_valid]",
        ] {
            syn::parse_str::<Cfg>(input).expect(input);
        }
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_friends_convert_source_bit_convert_other_bit_1()
     {
        let attributes =
            "friends = convert(Source) -> Bit, convert(Other) -> Bit";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_integral_friends_source_math_other_math_2()
     {
        let attributes =
            "integral = [friends = _(Source) -> Math, _(Other) -> Math]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_friends_3() {
        let attributes = "friends =";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_integral_friends_4()
     {
        let attributes = "integral = [friends =]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_friends_integral_5()
     {
        let attributes = "friends =, integral = []";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_integral_friends_valid_is_valid_6()
     {
        let attributes = "integral = [friends =, valid = is_valid]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_friends_convert_source_bit_1_7()
     {
        let attributes = "friends = convert(Source) -> [Bit, 1]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_integral_friends_source_inspector_8()
     {
        let attributes = "integral = [friends = _(Source) -> Inspector]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_friends_source_bit_unknown_true_9()
     {
        let attributes = "friends = _(Source) -> Bit, unknown = true";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_integral_friends_source_math_bit_unknown_tru_10()
     {
        let attributes =
            "integral = [friends = _(Source) -> [Math, Bit], unknown = true]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_friends_source_bit_friends_11()
     {
        let attributes = "friends = _(Source) -> Bit, friends = []";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friends_at_root_and_nested_scope_integral_unknown_true_12()
     {
        let attributes = "integral = [unknown = true]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn accepts_top_level_konst_values() {
        let config = syn::parse_str::<Cfg>("konst = nightly")
            .expect("top-level nightly konst should parse");
        assert_eq!(config.int.konst, crate::zz::Konst::Nightly);

        let config = syn::parse_str::<Cfg>("konst = stable")
            .expect("top-level stable konst should parse");
        assert_eq!(config.int.konst, crate::zz::Konst::Stable);
        assert!(config.int.konst_for(&["value"]));
        assert!(!config.int.konst_for(&["math"]));

        let config = syn::parse_str::<Cfg>("konst = none")
            .expect("top-level none konst should parse");
        assert_eq!(config.int.konst, crate::zz::Konst::None);
    }

    #[test]
    fn accepts_serde_and_each_representation_mode() {
        for mode in [
            "serde_with_auto",
            "serde_with_name",
            "serde_with_value",
        ] {
            let input = format!("konst = none, with = [{mode}]");
            let config = syn::parse_str::<Cfg>(&input).expect(&input);
            assert!(config.bit.serde);
        }
    }

    #[test]
    fn defaults_serde_representation_to_auto() {
        let config = syn::parse_str::<Cfg>("konst = none, with = [serde]")
            .expect("serde should not require an explicit representation");

        assert!(config.bit.serde);
        assert!(!config.bit.serde_with_name);
        assert!(!config.bit.serde_with_value);
    }

    #[test]
    fn defaults_layout_and_traits_and_allows_disabling_them() {
        let defaults = syn::parse_str::<Cfg>("konst = none")
            .expect("default bitflag configuration should parse");
        assert!(defaults.bit.clone);
        assert!(defaults.bit.copy);
        assert!(defaults.bit.hash);
        assert!(defaults.bit.debug);
        assert!(defaults.bit.repr);

        let enabled = syn::parse_str::<Cfg>(
            "konst = none, with = [clone, copy, hash, debug, repr]",
        )
        .expect("layout and traits should be configurable");
        assert!(enabled.bit.clone);
        assert!(enabled.bit.copy);
        assert!(enabled.bit.hash);
        assert!(enabled.bit.debug);
        assert!(enabled.bit.repr);

        let disabled = syn::parse_str::<Cfg>(
            "konst = none, without = [clone, copy, hash, debug, repr]",
        )
        .expect("layout and traits should be configurable");
        assert!(!disabled.bit.clone);
        assert!(!disabled.bit.copy);
        assert!(!disabled.bit.hash);
        assert!(!disabled.bit.debug);
        assert!(!disabled.bit.repr);
    }

    #[test]
    fn rejects_matching_repr_on_both_bitflag_and_enum() {
        let error = resolve_repr(
            Some(N::U008),
            Some(N::U008),
            proc_macro2::Span::call_site(),
        )
        .expect_err("matching repr values must not be specified twice");

        assert_eq!(
            error.to_string(),
            "repr cannot be specified on both bitflag and the enum",
        );
    }

    #[test]
    fn rejects_multiple_serde_representation_modes() {
        let error = syn::parse_str::<Cfg>(
            "konst = none, with = [serde_with_name, serde_with_value]",
        )
        .expect_err("serde representations are mutually exclusive");

        assert_eq!(
            error.to_string(),
            "bitflag serde representation flags are mutually exclusive",
        );
    }

    #[test]
    fn rejects_unknown_or_duplicate_serde_representation_flags() {
        let unknown =
            syn::parse_str::<Cfg>("konst = none, with = [serde_with_compact]")
                .expect_err("unknown serde representation flags must fail");
        assert_eq!(unknown.to_string(), "unknown flag");

        let duplicate = syn::parse_str::<Cfg>(
            "konst = none, with = [serde_with_auto, serde_with_auto]",
        )
        .expect_err("duplicate serde representation flags must fail");
        assert_eq!(duplicate.to_string(), "duplicated flags");
    }

    #[test]
    fn accepts_nested_unit_error_and_rejects_other_types() {
        syn::parse_str::<Cfg>("konst = none, integral = [error = ()]")
            .expect("unit errors should be accepted in nested integral config");

        let error = syn::parse_str::<Cfg>(
            "konst = none, integral = [error = FlagError]",
        )
        .expect_err("other error types remain unsupported");
        assert_eq!(
            error.to_string(),
            "only `()` is currently supported for `error`",
        );
    }

    #[test]
    fn bitflag_rejects_boolean_konst_values() {
        let error = syn::parse_str::<Cfg>("konst = true")
            .expect_err("boolean konst values are no longer accepted");
        assert_eq!(
            error.to_string(),
            "expected `none`, `stable`, or `nightly`",
        );
    }

    #[test]
    fn bitflag_reconciles_forwarded_integral_konst_flags() {
        let config = syn::parse_str::<Cfg>("integral = [with = [konst_value]]")
            .expect("matching forwarded konst flag should parse");
        assert!(config.int.konst_for(&["value"]));

        let error =
            syn::parse_str::<Cfg>("integral = [without = [konst_value]]")
                .expect_err(
                    "incompatible forwarded konst flag must be rejected",
                );
        assert_eq!(
            error.to_string(),
            "this value is not compatible with bitflags' own konst config",
        );
    }

    #[test]
    fn defaults_missing_konst_to_crate_default() {
        let config = syn::parse_str::<Cfg>("friends = [Foo(u8) -> Bit]")
            .expect("konst should default to the crate feature");

        assert_eq!(config.int.konst, crate::zz::Konst::default());
        assert!(config.int.konst_for(&["value"]));
        assert_eq!(config.int.konst_for(&["math"]), cfg!(feature = "nightly"),);
    }

    #[test]
    fn rejects_friendship_trait_configuration() {
        let error = match syn::parse_str::<Cfg>(
            "with = [trait_seal], integral = [konst = none]",
        ) {
            Ok(_) => panic!("bitflag must delegate friendship trait names"),
            Err(error) => error,
        };

        assert_eq!(error.to_string(), "unknown flag");
    }

    #[test]
    fn accepts_friend_capabilities() {
        let config = syn::parse_str::<Cfg>(
            "konst = none, friends = [Foo(u8) -> [Bit]], integral = []",
        )
        .expect("bitflag capabilities should parse");
        let friend = config.friends.iter().next().unwrap();
        assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));
    }

    #[test]
    fn rejects_legacy_friend_levels() {
        assert!(syn::parse_str::<Cfg>(
            "konst = none, friends = [u8(conv = self, level = [Bit])], integral = []",
        )
            .is_err());
    }

    #[test]
    fn accepts_trusted_root_and_nested_capability_without_make_friends_foo_source_trust_1()
     {
        let attributes = "friends = [Foo(Source) -> unsafe]";
        let input = format!("konst = none, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_root_and_nested_capability_without_make_friends_foo_source_trust_2()
     {
        let attributes = "friends = [Foo(Source) -> [unsafe]]";
        let input = format!("konst = none, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_root_and_nested_capability_without_make_friends_foo_source_bit_trust_3()
     {
        let attributes = "friends = [Foo(Source) -> [Bit, unsafe]]";
        let input = format!("konst = none, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_root_and_nested_capability_without_make_integral_friends_foo_source_trust_4()
     {
        let attributes = "integral = [friends = [Foo(Source) -> unsafe]]";
        let input = format!("konst = none, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_root_and_nested_capability_without_make_integral_friends_foo_source_trust_5()
     {
        let attributes = "integral = [friends = [Foo(Source) -> [unsafe]]]";
        let input = format!("konst = none, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_root_and_nested_capability_without_make_integral_friends_foo_source_math_bit_trust_6()
     {
        let attributes =
            "integral = [friends = [Foo(Source) -> [Math, Bit, unsafe]]]";
        let input = format!("konst = none, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_root_marker_capabilities_without_narrowing_integral_vocabulary()
    {
        let config = syn::parse_str::<Cfg>(
            "konst = none, friends = [convert(Source) -> Inspector], integral = [friends = [Foo(Other) -> Math]]",
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
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_legacy_trusted_argument_at_root_and_nested_scope_true_1_format_integral_friends_sour_2()
     {
        let trusted = "true";
        let attributes = format!(
            "integral = [friends = [Source(cap = Make, trusted = {trusted})]]"
        );
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_legacy_trusted_argument_at_root_and_nested_scope_false_2_format_friends_source_cap_ma_1()
     {
        let trusted = "false";
        let attributes =
            format!("friends = [Source(cap = Make, trusted = {trusted})]");
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_legacy_trusted_argument_at_root_and_nested_scope_false_2_format_integral_friends_sour_2()
     {
        let trusted = "false";
        let attributes = format!(
            "integral = [friends = [Source(cap = Make, trusted = {trusted})]]"
        );
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_with_display_display_1()
     {
        let input = "konst = none, with = [display, display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_without_display_display_2()
     {
        let input = "konst = none, without = [display, display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_with_display_with_display_3()
     {
        let input = "konst = none, with = [display], with = [display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_without_display_without_display_4()
     {
        let input = "konst = none, without = [display], without = [display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_integral_with_display_display_5()
     {
        let input = "konst = none, integral = [with = [display, display]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_integral_without_display_display_6()
     {
        let input = "konst = none, integral = [without = [display, display]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_integral_with_display_with_displ_7()
     {
        let input =
            "konst = none, integral = [with = [display], with = [display]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_root_and_nested_display_flags_konst_false_integral_without_display_without_8()
     {
        let input = "konst = none, integral = [without = [display], without = [display]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_root_and_nested_display_flags_konst_false_with_display_display_names_1()
     {
        let input = "konst = none, with = [display, display_names]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_root_and_nested_display_flags_konst_false_without_display_names_2()
     {
        let input = "konst = none, without = [display_names]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_root_and_nested_display_flags_konst_false_integral_with_display_display_na_3()
     {
        let input =
            "konst = none, integral = [with = [display, display_names]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_root_and_nested_display_flags_konst_false_integral_without_display_names_4()
     {
        let input = "konst = none, integral = [without = [display_names]]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_with_display_without_display_1()
     {
        let attributes = "with = [display], without = [display]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_without_display_with_display_2()
     {
        let attributes = "without = [display], with = [display]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_with_display_make_value_without_make_value_i_3()
     {
        let attributes =
            "with = [display, make_value], without = [make_value, impl_value]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_without_make_value_impl_value_with_display_m_4()
     {
        let attributes =
            "without = [make_value, impl_value], with = [display, make_value]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_integral_with_display_without_display_5()
     {
        let attributes = "integral = [with = [display], without = [display]]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_integral_without_display_with_display_6()
     {
        let attributes = "integral = [without = [display], with = [display]]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_integral_with_display_impl_math_add_without_case_7()
     {
        let attributes = "integral = [with = [display, impl_math_add], without = [impl_math_add, fn_conv_raw]]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_conflicting_flags_within_root_or_nested_scope_integral_without_impl_math_add_fn_conv_raw_w_8()
     {
        let attributes = "integral = [without = [impl_math_add, fn_conv_raw], with = [display, impl_math_add]]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn opposite_display_settings_in_distinct_scopes_are_legal_with_display_integral_without_display_1()
     {
        let attributes = "with = [display], integral = [without = [display]]";
        let input = format!("konst = none, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn opposite_display_settings_in_distinct_scopes_are_legal_integral_without_display_with_display_2()
     {
        let attributes = "integral = [without = [display]], with = [display]";
        let input = format!("konst = none, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn opposite_display_settings_in_distinct_scopes_are_legal_without_display_integral_with_display_3()
     {
        let attributes = "without = [display], integral = [with = [display]]";
        let input = format!("konst = none, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn opposite_display_settings_in_distinct_scopes_are_legal_integral_with_display_without_display_4()
     {
        let attributes = "integral = [with = [display]], without = [display]";
        let input = format!("konst = none, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn rejects_unsupported_nested_single_capabilities_integral_friends_foo_source_relation_1()
     {
        let attributes = "integral = [friends = [Foo(Source) -> Relation]]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_nested_single_capabilities_integral_friends_foo_source_numeric_2()
     {
        let attributes = "integral = [friends = [Foo(Source) -> Numeric]]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_nested_single_capabilities_integral_friends_foo_source_inspector_3()
     {
        let attributes = "integral = [friends = [Foo(Source) -> Inspector]]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_nested_single_capabilities_integral_friends_source_relation_4()
     {
        let attributes = "integral = [friends = [_(Source) -> Relation]]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_nested_single_capabilities_integral_friends_source_numeric_5()
     {
        let attributes = "integral = [friends = [_(Source) -> Numeric]]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_nested_single_capabilities_integral_friends_source_inspector_6()
     {
        let attributes = "integral = [friends = [_(Source) -> Inspector]]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_root_capability_value_syntax_friends_foo_source_bit_1() {
        let attributes = "friends = [Foo(Source) -> \"Bit\"]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_root_capability_value_syntax_friends_foo_source_bit_1_2()
    {
        let attributes = "friends = [Foo(Source) -> [Bit, 1]]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_root_capability_value_syntax_friends_foo_source_bit_3() {
        let attributes = "friends = [Foo(Source) -> {Bit}]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_root_capability_value_syntax_friends_source_bit_4() {
        let attributes = "friends = [_(Source) -> \"Bit\"]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_root_capability_value_syntax_friends_source_bit_1_5() {
        let attributes = "friends = [_(Source) -> [Bit, 1]]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_root_capability_value_syntax_friends_source_bit_6() {
        let attributes = "friends = [_(Source) -> {Bit}]";
        let input = format!("konst = none, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }
}
