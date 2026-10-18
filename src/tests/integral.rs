#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use crate::integral::{
        Cfg,
        Konst,
    };
    use crate::runner::ToCollection;

    #[test]
    fn accepts_single_friend_with_scalar_or_list_capabilities_friends_crate_convert_crate_source_source_ma_1()
     {
        let input = "friends = crate::convert(crate::source::Source) -> Make, konst = none";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_friend_with_scalar_or_list_capabilities_konst_none_friends_crate_convert_crate_sourc_2()
     {
        let input = "konst = none, friends = crate::convert(crate::source::Source) -> [Make, unsafe], valid = is_valid";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_friend_with_scalar_or_list_capabilities_konst_none_valid_is_valid_friends_foo_crate_case_3()
     {
        let input = "konst = none, valid = is_valid, friends = Foo(crate::source::Source) -> Bit";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_friend_with_scalar_or_list_capabilities_konst_none_friends_foo_crate_source_source_m_4()
     {
        let input = "konst = none, friends = Foo(crate::source::Source) -> [Math, Bit], in = 1..=4";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_friend_with_scalar_or_list_capabilities_konst_none_friends_self_foo_math_5()
     {
        let input = "konst = none, friends = Self(Foo) -> Math";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_single_friend_with_scalar_or_list_capabilities_konst_none_friends_self_foo_math_bit_with_di_6()
     {
        let input = "konst = none, friends = Self(Foo) -> [Math, Bit], with = [display]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_friend_lists_konst_none_friends_convert_source_make_foo_o_1()
     {
        let input = "konst = none, friends = [convert(Source) -> Make, Foo(Other) -> [Math, Bit]], valid = is_valid";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_friend_lists_konst_none_friends_valid_is_valid_2()
     {
        let input = "konst = none, friends = [], valid = is_valid";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_konst_none_friends_convert_source_make_conve_1()
     {
        let input = "konst = none, friends = convert(Source) -> Make, convert(Other) -> Make";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_konst_none_friends_source_bit_other_bit_2()
     {
        let input = "konst = none, friends = _(Source) -> Bit, _(Other) -> Bit";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_konst_none_friends_3() {
        let input = "konst = none, friends =";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_konst_none_friends_valid_is_valid_4()
     {
        let input = "konst = none, friends =, valid = is_valid";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_konst_none_friends_convert_make_5()
     {
        let input = "konst = none, friends = convert() -> Make";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_konst_none_friends_convert_source_6()
     {
        let input = "konst = none, friends = convert(Source) ->";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_konst_none_friends_convert_source_make_1_7()
     {
        let input = "konst = none, friends = convert(Source) -> [Make, 1]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_konst_none_friends_source_unknown_8()
     {
        let input = "konst = none, friends = _(Source) -> Unknown";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_konst_none_friends_source_bit_unknown_true_9()
     {
        let input =
            "konst = none, friends = _(Source) -> [Bit], unknown = true";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_konst_none_friends_source_bit_friends_10()
     {
        let input = "konst = none, friends = _(Source) -> Bit, friends = []";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn accepts_explicit_konst_modes_konst_none_1() {
        let (input, expected) = ("konst = none", Konst::None);
        let cfg = syn::parse_str::<Cfg>(input).expect(input);
        assert_eq!(cfg.konst, expected);
    }

    #[test]
    fn accepts_explicit_konst_modes_konst_stable_2() {
        let (input, expected) = ("konst = stable", Konst::Stable);
        let cfg = syn::parse_str::<Cfg>(input).expect(input);
        assert_eq!(cfg.konst, expected);
    }

    #[test]
    fn accepts_explicit_konst_modes_konst_nightly_3() {
        let (input, expected) = ("konst = nightly", Konst::Nightly);
        let cfg = syn::parse_str::<Cfg>(input).expect(input);
        assert_eq!(cfg.konst, expected);
    }

    #[test]
    fn rejects_boolean_konst_modes_konst_true_1() {
        let input = "konst = true";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("boolean konst values are no longer accepted");
        assert_eq!(
            error.to_string(),
            "expected `none`, `stable`, or `nightly`",
        );
    }

    #[test]
    fn rejects_boolean_konst_modes_konst_false_2() {
        let input = "konst = false";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("boolean konst values are no longer accepted");
        assert_eq!(
            error.to_string(),
            "expected `none`, `stable`, or `nightly`",
        );
    }

    #[test]
    fn konst_flags_inherit_and_override_the_master_default() {
        let disabled = syn::parse_str::<Cfg>("konst = none")
            .expect("disabled konst should parse");
        assert!(!disabled.konst_for(&[
            "add", "math"
        ]));

        let enabled = syn::parse_str::<Cfg>("konst = none, with = [konst_add]")
            .expect("an operation can opt into constness");
        assert!(enabled.konst_for(&[
            "add", "math"
        ]));
        assert!(!enabled.konst_for(&[
            "sub", "math"
        ]));

        let family = syn::parse_str::<Cfg>("konst = none, with = [konst_math]")
            .expect("a family can opt into constness");
        assert!(family.konst_for(&[
            "add", "math"
        ]));

        let operation_override = syn::parse_str::<Cfg>(
            "konst = none, with = [konst_math], without = [konst_add]",
        )
        .expect("an operation can override its family");
        assert!(!operation_override.konst_for(&[
            "add", "math"
        ]));
        assert!(operation_override.konst_for(&[
            "sub", "math"
        ]));

        let enabled = syn::parse_str::<Cfg>("konst = nightly")
            .expect("enabled konst should parse");
        assert!(enabled.konst_for(&[
            "add", "math"
        ]));

        let disabled =
            syn::parse_str::<Cfg>("konst = nightly, without = [konst_add]")
                .expect("an operation can opt out of constness");
        assert!(!disabled.konst_for(&[
            "add", "math"
        ]));
        assert!(disabled.konst_for(&[
            "sub", "math"
        ]));
    }

    #[test]
    fn default_konst_enables_only_stable_flags() {
        let cfg = syn::parse_str::<Cfg>("")
            .expect("stable konst should be the default");
        assert_eq!(cfg.konst, Konst::Stable);

        assert!(cfg.konst_for(&["try_make"]));
        assert!(cfg.konst_for(&[
            "core_int", "value"
        ]));
        assert!(!cfg.konst_for(&[
            "add", "math"
        ]));
        assert!(!cfg.konst_for(&["friendship"]));
        assert!(!cfg.konst_friendship());

        let cfg = syn::parse_str::<Cfg>("without = [konst_try_make]")
            .expect("explicit flags override the stable master default");
        assert!(!cfg.konst_for(&["try_make"]));

        let cfg = syn::parse_str::<Cfg>("with = [konst_add]")
            .expect("explicit flags override the stable master default");
        assert!(cfg.konst_for(&[
            "add", "math"
        ]));
        assert!(cfg.konst_friendship());
    }

    #[test]
    fn defaults_self_operators_and_allows_disabling_them() {
        let defaults = syn::parse_str::<Cfg>("konst = none")
            .expect("default integral configuration should parse");
        assert!(defaults.flags.self_math);
        assert!(defaults.flags.self_bit);

        let disabled = syn::parse_str::<Cfg>(
            "konst = none, without = [self_math, self_bit]",
        )
        .expect("self operators should be configurable");
        assert!(!disabled.flags.self_math);
        assert!(!disabled.flags.self_bit);
    }

    #[test]
    fn accepts_root_validation_attributes() {
        let config =
            syn::parse_str::<Cfg>("konst = none, valid = is_even, in = 1..=4")
                .expect("root validation attributes should parse");

        assert_eq!(config.validation.callbacks.len(), 1);
        assert_eq!(config.validation.ranges.len(), 1);
        assert!(config.has_validation());
    }

    #[test]
    fn accepts_new_generation_flags_and_unit_error() {
        let config = syn::parse_str::<Cfg>(
            "konst = none, error = (), with = [serde, with_const_map_fn]",
        )
        .expect("new integral options should parse");

        assert!(config.flags.serde);
        assert!(config.flags.with_const_map_fn);
        assert!(config.flags.map);
    }

    #[test]
    fn accepts_disabling_serde_and_integral_mapping() {
        let config =
            syn::parse_str::<Cfg>("konst = none, without = [serde, map]")
                .expect("generation flags should be independently disabled");

        assert!(!config.flags.serde);
        assert!(!config.flags.map);
    }

    #[test]
    fn rejects_duplicate_error_configuration() {
        let error =
            syn::parse_str::<Cfg>("konst = none, error = (), error = ()")
                .expect_err("error configuration may occur only once");

        assert_eq!(error.to_string(), "duplicated arg");
    }

    #[test]
    fn rejects_non_unit_error_type() {
        let error = syn::parse_str::<Cfg>("konst = none, error = MyError")
            .expect_err("only unit validation errors are reserved");

        assert_eq!(
            error.to_string(),
            "only `()` is currently supported for `error`",
        );
    }

    #[test]
    fn accepts_root_validation_lists() {
        let config = syn::parse_str::<Cfg>(
            "konst = none, valid = [is_even, is_not_fifty], in = [1..=4, 8..10, ..=0]",
        )
            .expect("root validation lists should parse");

        assert_eq!(config.validation.callbacks.len(), 2);
        assert_eq!(config.validation.ranges.len(), 2);
    }

    #[test]
    fn merges_overlapping_and_adjacent_ranges() {
        let config = syn::parse_str::<Cfg>(
            "konst = none, in = [10..=12, 1..3, 3..=5, 6..10]",
        )
        .expect("constant ranges should parse");

        assert_eq!(config.validation.ranges.len(), 1);
        let range = &config.validation.ranges[0];
        assert_eq!(
            range.start.as_ref().unwrap().to_token_stream().to_string(),
            "1",
        );
        assert_eq!(
            range.end.as_ref().unwrap().to_token_stream().to_string(),
            "12",
        );
        assert!(matches!(range.limits, syn::RangeLimits::Closed(_)));
    }

    #[test]
    fn accepts_primitive_integer_constants_in_ranges() {
        let config = syn::parse_str::<Cfg>(
            "konst = none, in = [1..=u32::MAX, u32::MAX..]",
        )
        .expect("primitive integer constants should parse");
        assert_eq!(config.validation.ranges.len(), 1);
    }

    #[test]
    fn rejects_non_constant_range_bounds_konst_none_in_max_4_1() {
        let input = "konst = none, in = [MAX..4]";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("range bounds must be integer constants");
        assert_eq!(error.to_string(), "range bounds must be integer constants");
    }

    #[test]
    fn rejects_non_constant_range_bounds_konst_none_in_foo_2() {
        let input = "konst = none, in = [foo..]";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("range bounds must be integer constants");
        assert_eq!(error.to_string(), "range bounds must be integer constants");
    }

    #[test]
    fn rejects_non_constant_range_bounds_konst_none_in_1_1_4_3() {
        let input = "konst = none, in = [1 + 1..4]";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("range bounds must be integer constants");
        assert_eq!(error.to_string(), "range bounds must be integer constants");
    }

    #[test]
    fn accepts_unbracketed_root_in_union_with_valid_callback() {
        let config =
            syn::parse_str::<Cfg>("konst = none, in = 1..2, valid = foo")
                .expect(
                    "an unbracketed range with a valid callback should parse",
                );

        assert_eq!(config.validation.callbacks.len(), 1);
        assert_eq!(config.validation.ranges.len(), 1);
    }

    #[test]
    fn rejects_non_in_validation() {
        let error = syn::parse_str::<Cfg>("konst = none, in = [1]")
            .expect_err("in validation needs range expressions");

        assert_eq!(error.to_string(), "invalid range definition");
    }

    #[test]
    fn rejects_plus_separated_ranges() {
        syn::parse_str::<Cfg>("konst = none, in = [1..2 + 3..4]")
            .expect_err("multiple ranges use comma separators");
    }

    #[test]
    fn rejects_legacy_validation_attributes() {
        let callback =
            syn::parse_str::<Cfg>("konst = none, callback = is_valid")
                .expect_err("valid is the validation callback key");
        assert_eq!(callback.to_string(), "unknown attribute");

        let range = syn::parse_str::<Cfg>("konst = none, range = 1..2")
            .expect_err("in is the range constraint key");
        assert_eq!(range.to_string(), "unknown attribute");

        let validator =
            syn::parse_str::<Cfg>("konst = none, validator = is_valid")
                .expect_err("valid is the validation callback key");
        assert_eq!(validator.to_string(), "unknown attribute");
    }

    #[test]
    fn defaults_missing_konst_to_stable() {
        let config = syn::parse_str::<Cfg>("friends = [Foo(u8) -> Bit]")
            .expect("konst should default to stable");

        assert_eq!(config.konst, Konst::Stable);
        assert!(config.konst_for(&["try_make"]));
        assert!(!config.konst_for(&["math"]));
    }

    #[test]
    fn accepts_explicit_friend_capabilities() {
        let config = syn::parse_str::<Cfg>(
            "konst = none, friends = [convert(u8) -> [Make, Bit]]",
        )
        .expect("integral capabilities should parse");
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.capabilities.contains(&syn::parse_quote!(Make)));
        assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));
    }

    #[test]
    fn expands_math_friend_capability_without_rel() {
        let config =
            syn::parse_str::<Cfg>("konst = none, friends = [Foo(u8) -> Math]")
                .expect(
                    "Math should expand to integral operation capabilities",
                );
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.capabilities.contains(&syn::parse_quote!(Make)));
        assert!(friend.capabilities.contains(&syn::parse_quote!(Math)));
        assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));
        assert!(!friend.capabilities.contains(&syn::parse_quote!(Rel)));
    }

    #[test]
    fn defaults_omitted_friend_capabilities_to_math() {
        let config = syn::parse_str::<Cfg>(
            "konst = none, friends = [convert(Source), Foo(Other)]",
        )
        .expect("integral friends may omit their capabilities");

        for friend in &config.friends {
            assert!(friend.capabilities.contains(&syn::parse_quote!(Make)));
            assert!(friend.capabilities.contains(&syn::parse_quote!(Math)));
            assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));
            assert!(!friend.capabilities.contains(&syn::parse_quote!(Rel)));
        }
    }

    #[test]
    fn preserves_explicit_empty_friend_capabilities() {
        let config =
            syn::parse_str::<Cfg>("konst = none, friends = Foo(Source) -> []")
                .expect("an explicit empty capability list should parse");
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.capabilities.is_empty());
    }

    #[test]
    fn rejects_legacy_friend_levels() {
        syn::parse_str::<Cfg>("konst = none, friends = [u8(level = [Full])]")
            .expect_err("levels were replaced by capabilities");
    }

    #[test]
    fn rejects_custom_friendship_trait_names() {
        let trait_name =
            syn::parse_str::<Cfg>("konst = none, trait_seal = OtherSeal")
                .expect_err("integral seal name is fixed");
        assert_eq!(trait_name.to_string(), "unknown attribute");

        let of_name = syn::parse_str::<Cfg>("konst = none, fn_of = custom_of")
            .expect_err("integral of name is fixed");
        assert_eq!(of_name.to_string(), "unknown attribute");
    }

    #[test]
    fn accepts_trusted_capability_without_make_trust_1() {
        let capabilities = "unsafe";
        let input =
            format!("konst = none, friends = [Foo(Source) -> {capabilities}]");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_capability_without_make_trust_2() {
        let capabilities = "[unsafe]";
        let input =
            format!("konst = none, friends = [Foo(Source) -> {capabilities}]");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_capability_without_make_math_trust_3() {
        let capabilities = "[Math, unsafe]";
        let input =
            format!("konst = none, friends = [Foo(Source) -> {capabilities}]");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn accepts_trusted_capability_without_make_bit_rel_trust_4() {
        let capabilities = "[Bit, Rel, unsafe]";
        let input =
            format!("konst = none, friends = [Foo(Source) -> {capabilities}]");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }

    #[test]
    fn rejects_legacy_trusted_argument_true_1() {
        let trusted = "true";
        let input = format!(
            "konst = none, friends = [Source(cap = Make, trusted = {trusted})]"
        );
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_legacy_trusted_argument_false_2() {
        let trusted = "false";
        let input = format!(
            "konst = none, friends = [Source(cap = Make, trusted = {trusted})]"
        );
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_display_flags_and_generation_arguments_konst_none_with_display_display_1()
     {
        let input = "konst = none, with = [display, display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_display_flags_and_generation_arguments_konst_none_without_display_display_2()
     {
        let input = "konst = none, without = [display, display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_display_flags_and_generation_arguments_konst_none_with_display_with_display_3()
     {
        let input = "konst = none, with = [display], with = [display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_display_flags_and_generation_arguments_konst_none_without_display_without_display_4()
     {
        let input = "konst = none, without = [display], without = [display]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_display_flags_konst_none_with_display_display_names_1()
     {
        let input = "konst = none, with = [display, display_names]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_display_flags_konst_none_without_display_names_2() {
        let input = "konst = none, without = [display_names]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn accepts_from_str_generation_flag_konst_none_with_from_str_1() {
        let input = "konst = none, with = [from_str]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_from_str_generation_flag_konst_none_without_from_str_2() {
        let input = "konst = none, without = [from_str]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_core_integral_generation_flag_konst_none_with_core_int_1() {
        let input = "konst = none, with = [core_int]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn accepts_core_integral_generation_flag_konst_none_without_core_int_2() {
        let input = "konst = none, without = [core_int]";
        syn::parse_str::<Cfg>(input).expect(input);
    }

    #[test]
    fn rejects_undocumented_with_aliases_konst_none_with_impl_core_int_impl_core_int_1()
     {
        let input = "konst = none, with_impl_core_int = [impl_core_int]";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("generation options use `with = [...]`");
        assert_eq!(error.to_string(), "unknown attribute");
    }

    #[test]
    fn rejects_undocumented_with_aliases_konst_none_with_display_display_2() {
        let input = "konst = none, with_display = [display]";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("generation options use `with = [...]`");
        assert_eq!(error.to_string(), "unknown attribute");
    }

    #[test]
    fn rejects_unsupported_single_capabilities_konst_none_friends_source_relation_1()
     {
        let input = "konst = none, friends = [_(Source) -> Relation]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_single_capabilities_konst_none_friends_source_numeric_2()
     {
        let input = "konst = none, friends = [_(Source) -> Numeric]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_single_capabilities_konst_none_friends_source_inspector_3()
     {
        let input = "konst = none, friends = [_(Source) -> Inspector]";
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }

    #[test]
    fn shared_friend_parser_accepts_qualified_conversion_and_source_paths() {
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(
            "crate::conversions::own(crate::sources::Source) -> [Make, unsafe]",
        )
        .expect("qualified conversion and source paths should parse");

        let conversion = friend.conv.as_ref().unwrap();
        assert_eq!(
            conversion
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .vec(),
            [
                "crate",
                "conversions",
                "own"
            ]
        );
        assert_eq!(
            friend
                .ty
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .vec(),
            [
                "crate", "sources", "Source"
            ]
        );
        assert!(friend.capabilities.contains(&syn::parse_quote!(Make)));
        assert!(
            friend
                .capabilities
                .contains(&syn::parse_quote!(UnsafeTrust))
        );
    }

    #[test]
    fn shared_friend_parser_accepts_scalar_list_and_empty_capabilities_trust_1()
    {
        let (capabilities, expected) = ("unsafe", vec!["UnsafeTrust"]);
        let input = format!("convert(Source) -> {capabilities}");
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(&input)
            .expect(&input);
        assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
        assert!(friend.ty.is_ident("Source"));
        assert_eq!(
            friend.capabilities.iter().map(ToString::to_string).vec(),
            expected,
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_accepts_scalar_list_and_empty_capabilities_trust_2()
    {
        let (capabilities, expected) = ("[unsafe]", vec!["UnsafeTrust"]);
        let input = format!("convert(Source) -> {capabilities}");
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(&input)
            .expect(&input);
        assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
        assert!(friend.ty.is_ident("Source"));
        assert_eq!(
            friend.capabilities.iter().map(ToString::to_string).vec(),
            expected,
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_accepts_scalar_list_and_empty_capabilities_make_3()
    {
        let (capabilities, expected) = (
            "[Make, unsafe]",
            vec![
                "Make",
                "UnsafeTrust",
            ],
        );
        let input = format!("convert(Source) -> {capabilities}");
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(&input)
            .expect(&input);
        assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
        assert!(friend.ty.is_ident("Source"));
        assert_eq!(
            friend.capabilities.iter().map(ToString::to_string).vec(),
            expected,
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_accepts_scalar_list_and_empty_capabilities_make_4()
    {
        let (capabilities, expected) = (
            "[Make, unsafe,]",
            vec![
                "Make",
                "UnsafeTrust",
            ],
        );
        let input = format!("convert(Source) -> {capabilities}");
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(&input)
            .expect(&input);
        assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
        assert!(friend.ty.is_ident("Source"));
        assert_eq!(
            friend.capabilities.iter().map(ToString::to_string).vec(),
            expected,
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_accepts_scalar_list_and_empty_capabilities_case_5()
    {
        let (capabilities, expected) = ("[]", Vec::<&str>::new());
        let input = format!("convert(Source) -> {capabilities}");
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(&input)
            .expect(&input);
        assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
        assert!(friend.ty.is_ident("Source"));
        assert_eq!(
            friend.capabilities.iter().map(ToString::to_string).vec(),
            expected,
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_accepts_typed_implicit_conversion_self_source_trust_1()
     {
        let input = "Self(Source) -> unsafe";
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(input)
            .expect(input);
        assert!(friend.conv.is_none(), "{input}");
        assert!(friend.ty.is_ident("Source"), "{input}");
        assert!(
            friend
                .capabilities
                .contains(&syn::parse_quote!(UnsafeTrust))
        );
    }

    #[test]
    fn shared_friend_parser_accepts_typed_implicit_conversion_self_source_make_trust_2()
     {
        let input = "Self(Source,) -> [Make, unsafe]";
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(input)
            .expect(input);
        assert!(friend.conv.is_none(), "{input}");
        assert!(friend.ty.is_ident("Source"), "{input}");
        assert!(
            friend
                .capabilities
                .contains(&syn::parse_quote!(UnsafeTrust))
        );
    }

    #[test]
    fn shared_friend_parser_accepts_absolute_paths_and_trailing_source_comma() {
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(
            "::conversions::own(::sources::Source,) -> Make",
        )
        .expect("one source argument may have a trailing comma");

        let conversion = friend.conv.as_ref().unwrap();
        assert!(conversion.leading_colon.is_some());
        assert_eq!(conversion.segments.first().unwrap().ident, "conversions");
        assert_eq!(conversion.segments.last().unwrap().ident, "own");
        assert!(friend.ty.leading_colon.is_some());
        assert_eq!(friend.ty.segments.first().unwrap().ident, "sources");
        assert_eq!(friend.ty.segments.last().unwrap().ident, "Source");
    }

    #[test]
    fn shared_friend_parser_rejects_obsolete_relationship_syntax_source_1() {
        let input = "Source";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_obsolete_relationship_syntax_source_make_2()
    {
        let input = "Source -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_obsolete_relationship_syntax_source_conv_convert_cap_make_3()
     {
        let input = "Source(conv = convert, cap = Make)";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_obsolete_relationship_syntax_source_cap_make_trust_4()
     {
        let input = "Source(cap = [Make, unsafe])";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_obsolete_relationship_syntax_cap_make_5() {
        let input = "_(cap = Make)";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_obsolete_relationship_syntax_source_level_full_6()
     {
        let input = "Source(level = [Full])";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_obsolete_relationship_syntax_source_conv_convert_cap_make_trusted_true_7()
     {
        let input = "Source(conv = convert, cap = Make, trusted = true)";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_obsolete_relationship_syntax_source_conv_convert_cap_make_trusted_false_8()
     {
        let input = "Source(conv = convert, cap = Make, trusted = false)";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_does_not_suggest_obsolete_migrations_source_conv_convert_cap_make_1()
     {
        let input = "Source(conv = convert, cap = Make)";
        let error = syn::parse_str::<crate::friendship::cfg::Friend>(input)
            .expect_err("obsolete friend syntax must fail");
        assert!(
            !error.to_string().contains("were removed"),
            "{input}: {error}"
        );
        assert!(!error.to_string().contains("use `_ ->"), "{input}: {error}");
    }

    #[test]
    fn shared_friend_parser_does_not_suggest_obsolete_migrations_cap_make_2() {
        let input = "_(cap = Make)";
        let error = syn::parse_str::<crate::friendship::cfg::Friend>(input)
            .expect_err("obsolete friend syntax must fail");
        assert!(
            !error.to_string().contains("were removed"),
            "{input}: {error}"
        );
        assert!(!error.to_string().contains("use `_ ->"), "{input}: {error}");
    }

    #[test]
    fn shared_friend_parser_requires_arrow_and_capabilities_convert_source_make_1()
     {
        let input = "convert(Source) Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_arrow_and_capabilities_convert_source_2() {
        let input = "convert(Source) ->";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_arrow_and_capabilities_source_3() {
        let input = "_(Source)";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_arrow_and_capabilities_source_4() {
        let input = "_(Source) ->";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_arrow_and_capabilities_case_5() {
        let input = "_";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_arrow_and_capabilities_make_6() {
        let input = "_ Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_arrow_and_capabilities_case_7() {
        let input = "_ ->";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_arrow_and_capabilities_foo_source_8() {
        let input = "Foo(Source) ->";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_arrow_and_capabilities_foo_9() {
        let input = "Foo";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_arrow_and_capabilities_foo_10() {
        let input = "Foo ->";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_exactly_one_source_path_convert_make_1() {
        let input = "convert() -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_exactly_one_source_path_convert_source_other_make_2()
     {
        let input = "convert(Source, Other) -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_exactly_one_source_path_convert_make_3() {
        let input = "convert(_) -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_exactly_one_source_path_convert_source_make_4()
     {
        let input = "convert(&Source) -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_exactly_one_source_path_convert_source_make_5()
     {
        let input = "convert((Source)) -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_exactly_one_source_path_make_6() {
        let input = "_() -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_exactly_one_source_path_source_other_make_7()
     {
        let input = "_(Source, Other) -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_requires_exactly_one_source_path_make_8() {
        let input = "_(_) -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens_convert_source_make_1()
     {
        let input = "convert(Source) -> \"Make\"";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens_convert_source_make_1_2()
     {
        let input = "convert(Source) -> [Make, 1]";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens_convert_source_make_3()
     {
        let input = "convert(Source) -> {Make}";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens_convert_source_make_4()
     {
        let input = "convert(Source) -> (Make)";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens_convert_source_make_trust_5()
     {
        let input = "convert(Source) -> [Make unsafe]";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens_convert_source_make_trust_6()
     {
        let input = "convert(Source) -> [Make,, unsafe]";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens_convert_source_make_nested_7()
     {
        let input = "convert(Source) -> Make::Nested";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens_convert_source_make_nested_8()
     {
        let input = "convert(Source) -> [Make::Nested]";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens_convert_source_make_extra_9()
     {
        let input = "convert(Source) -> Make extra";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens_convert_source_make_extra_10()
     {
        let input = "convert(Source) -> [Make] extra";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens_convert_source_make_trust_11()
     {
        let input = "convert(Source) -> Make, unsafe";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens_foo_source_12()
     {
        let input = "_ -> Foo(Source)";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }
}
