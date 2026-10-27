#[cfg(test)]
mod tests {
    use quote::{
        ToTokens,
        format_ident,
    };
    use syn::parse::Parser as _;

    use crate::integral::{
        Cfg,
        NonZeroPolicy,
        StoragePlan,
        ekran_with,
    };
    use crate::zz_lib::{
        Konst,
        Konsts,
    };
    use crate::zz_profile::{
        Arithmetic,
        FacetOwner,
        FacetSlotKey,
    };
    use crate::zz_runner::ToCollection;
    use crate::zz_value_type::{
        Int,
        N,
        Operator,
    };

    fn expansion_of(input: &str) -> String {
        let config = syn::parse_str::<Cfg>(input);
        assert!(config.is_ok(), "{input}");
        let expansion = ekran_with(
            format_ident!("Point"),
            format_ident!("u32"),
            Box::new(config.unwrap()),
            StoragePlan::primitive(N::U032),
        );
        assert!(expansion.is_ok(), "{input}");
        return expansion.unwrap().to_string();
    }

    fn profile_config(
        profile: &str,
        input: &str,
    ) -> syn::Result<Cfg> {
        let parser = |input: syn::parse::ParseStream| {
            return Cfg::parse_with_profile(input, profile);
        };

        return parser.parse_str(input);
    }

    fn parse_test_config(input: &str) -> Cfg {
        let result = syn::parse_str::<Cfg>(input);
        assert!(result.is_ok(), "{input}");
        return result.unwrap();
    }

    #[test]
    fn fixed_profile_parser_applies_delta_and_root_settings() {
        let input = "with = math_mul";
        let this = profile_config("delta", input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert_eq!(config.profile.arithmetic(), Arithmetic::PrimitiveWrapping);
        assert!(config.profile.is_circular());
        assert!(!config.profile.emits_precedes());
        assert!(config.flags.math_mul);
    }

    #[test]
    fn circular_profile_parser_accepts_direct_precedes() {
        let input = "precedes = false";
        let this = profile_config("circular", input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(config.profile.is_circular());
        assert!(!config.profile.emits_precedes());
    }

    #[test]
    fn fixed_profile_parser_rejects_profile_override() {
        let input = "profile = wrapping";
        let this = profile_config("delta", input);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "profile is selected by this macro");
    }

    #[test]
    fn delta_profile_parser_rejects_direct_precedes() {
        let input = "precedes = true";
        let this = profile_config("delta", input);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "unknown attribute");
    }

    #[test]
    fn accepts_bracketed_marker_tree() {
        let input = "
            tree = [
                Direction {
                    In,
                    Out,
                },
                Mode {
                    Active,
                },
            ],
            konst = none
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(config.tree.is_some());
    }

    #[test]
    fn accepts_friend_with_single_capability() {
        let input = "
            friend = crate::convert(crate::source::Source) -> Make,
            konst = none
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn storage_plan_maps_every_non_zero_alias() {
        assert_eq!(
            StoragePlan::new(N::USIZ, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroUsize",
        );
        assert_eq!(
            StoragePlan::new(N::ISIZ, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroIsize",
        );
        assert_eq!(
            StoragePlan::new(N::U008, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroU8",
        );
        assert_eq!(
            StoragePlan::new(N::U016, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroU16",
        );
        assert_eq!(
            StoragePlan::new(N::U032, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroU32",
        );
        assert_eq!(
            StoragePlan::new(N::U064, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroU64",
        );
        assert_eq!(
            StoragePlan::new(N::U128, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroU128",
        );
        assert_eq!(
            StoragePlan::new(N::I008, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroI8",
        );
        assert_eq!(
            StoragePlan::new(N::I016, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroI16",
        );
        assert_eq!(
            StoragePlan::new(N::I032, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroI32",
        );
        assert_eq!(
            StoragePlan::new(N::I064, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroI64",
        );
        assert_eq!(
            StoragePlan::new(N::I128, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroI128",
        );
    }

    #[test]
    fn storage_plan_supports_all_combined_layouts() {
        assert_eq!(
            StoragePlan::new(N::U032, false, false)
                .field_type()
                .to_string(),
            "u32",
        );
        assert_eq!(
            StoragePlan::new(N::U032, true, false)
                .field_type()
                .to_string(),
            ":: core :: num :: NonZeroU32",
        );
        assert_eq!(
            StoragePlan::new(N::U032, false, true)
                .field_type()
                .to_string(),
            ":: core :: num :: Wrapping < u32 >",
        );
        assert_eq!(
            StoragePlan::new(N::U032, true, true)
                .field_type()
                .to_string(),
            ":: core :: num :: Wrapping < :: core :: num :: NonZeroU32 >",
        );
    }

    #[test]
    fn non_zero_storage_rejects_a_custom_unchecked_constructor() {
        let input = "
            konst = none,
            domain = 1..,
            without = fn_make_unchecked,
        ";
        let config = syn::parse_str::<Cfg>(input);
        assert!(config.is_ok(), "{input}");
        let maker = ekran_with(
            format_ident!("Point"),
            format_ident!("u32"),
            Box::new(config.unwrap()),
            StoragePlan::new(N::U032, true, false),
        );
        assert!(maker.is_err(), "{input}");
        let error = maker.err().unwrap();

        assert_eq!(
            error.to_string(),
            "nonzero storage does not support custom unchecked constructors",
        );
    }

    #[test]
    fn wrapping_storage_allows_a_custom_unchecked_constructor() {
        let input = "
            konst = none,
            profile = wrapping {},
            without = fn_make_checked,
        ";
        let config = syn::parse_str::<Cfg>(input);
        assert!(config.is_ok(), "{input}");
        let maker = ekran_with(
            format_ident!("Point"),
            format_ident!("u32"),
            Box::new(config.unwrap()),
            StoragePlan::new(N::U032, false, true),
        );

        assert!(maker.is_ok(), "{input}");
    }

    #[test]
    fn accepts_friend_with_multiple_capabilities() {
        let input = "
            konst = none,
            friend = crate::convert(crate::source::Source) -> [Make, unsafe],
            valid = is_valid
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_friend_with_custom_conversion() {
        let input = "
            konst = none,
            valid = is_valid,
            friend = Foo(crate::source::Source) -> Bit
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_friend_with_multiple_capabilities_and_range() {
        let input = "
            konst = none,
            friend = Foo(crate::source::Source) -> [Math, Bit],
            domain = 1..=4
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_implicit_friend() {
        let input = "
            konst = none,
            friend = _(Foo) -> Math
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_implicit_friend_with_multiple_capabilities() {
        let input = "
            konst = none,
            friend = _(Foo) -> [Math, Bit],
            with = [display]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_multiple_friends() {
        let input = "
            konst = none,
            friend = [convert(Source) -> Make, Foo(Other) -> [Math, Bit]],
            valid = is_valid
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_empty_friends() {
        let input = "
            konst = none,
            friend = [],
            valid = is_valid
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn rejects_multiple_single_explicit_friends() {
        let input = "
            konst = none,
            friend = convert(Source) -> Make,
            convert(Other) -> Make
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_multiple_single_implicit_friends() {
        let input = "
            konst = none,
            friend = _(Source) -> Bit,
            _(Other) -> Bit
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_missing_single_friend() {
        let input = "
            konst = none,
            friend =
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_missing_single_friend_before_validation() {
        let input = "
            konst = none,
            friend =,
            valid = is_valid
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_conversion_without_source() {
        let input = "
            konst = none,
            friend = convert() -> Make
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_conversion_without_capabilities() {
        let input = "
            konst = none,
            friend = convert(Source) ->
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_non_identifier_friend_capability() {
        let input = "
            konst = none,
            friend = convert(Source) -> [Make, 1]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_unknown_friend_capability() {
        let input = "
            konst = none,
            friend = _(Source) -> Unknown
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn unknown_root_friend_capability_reports_its_span() {
        let input = "konst = none, friend = convert(Source) -> Unknown";
        let result = syn::parse_str::<Cfg>(input);
        assert!(result.is_err(), "{input}");
        let error = result.unwrap_err();

        assert_eq!(error.to_string(), "unknown capability", "{input}");
        assert_eq!(error.span().start().line, 1, "{input}");
        assert_eq!(
            error.span().start().column,
            input.find("Unknown").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn unknown_facet_friend_capability_reports_its_span() {
        let input =
            "konst = none, facet = int { friend = convert(Source) -> Unknown }";
        let result = syn::parse_str::<Cfg>(input);
        assert!(result.is_err(), "{input}");
        let error = result.unwrap_err();

        assert_eq!(error.to_string(), "unknown capability", "{input}");
        assert_eq!(error.span().start().line, 1, "{input}");
        assert_eq!(
            error.span().start().column,
            input.find("Unknown").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn rejects_unknown_attribute_after_implicit_friend() {
        let input = "
            konst = none,
            friend = _(Source) -> [Bit],
            unknown = true
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_friends_after_implicit_friend() {
        let input = "
            konst = none,
            friend = _(Source) -> Bit,
            friend = []
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn accepts_konst_none() {
        let input = "
            konst = none
        ";
        let expected = Konst::None;
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let cfg = this.unwrap();
        assert_eq!(cfg.konst, expected);
    }

    #[test]
    fn accepts_konst_stable() {
        let input = "
            konst = stable
        ";
        let expected = Konst::Stable;
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let cfg = this.unwrap();
        assert_eq!(cfg.konst, expected);
    }

    #[test]
    fn accepts_konst_nightly() {
        let input = "
            konst = nightly
        ";
        let expected = Konst::Nightly;
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let cfg = this.unwrap();
        assert_eq!(cfg.konst, expected);
    }

    #[test]
    fn rejects_boolean_konst_true() {
        let input = "
            konst = true
        ";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("boolean konst values are no longer accepted");
        assert_eq!(
            error.to_string(),
            "expected `none`, `stable`, or `nightly`",
        );
    }

    #[test]
    fn rejects_text_only_konst_flag() {
        let result = syn::parse_str::<Cfg>("with = [konst_text_access]");
        let error = result.unwrap_err();

        assert_eq!(error.to_string(), "unknown flag");
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
    fn conflicting_profile_const_flags_report_the_second_setting() {
        let input = "profile = int { with = konst_add, without = konst_add }";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "flag specified in both `with` and `without`",
            "{input}",
        );
        assert_eq!(
            error.span().start().column,
            input.rfind("konst_add").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn const_overrides_follow_profile_and_root_precedence() {
        let input = "
            konst = none,
            profile = [
                int { with = [konst_add, konst_sub] },
                int { without = [konst_add, konst_sub] },
            ],
            with = konst_add
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        assert!(parsed.is_ok(), "{input}");
        let config = parsed.unwrap();

        assert!(config.konst_for(&[Konsts::Add]), "{input}");
        assert!(!config.konst_for(&[Konsts::Sub]), "{input}");
    }

    #[test]
    fn facet_const_flag_reports_its_identifier() {
        let input = "facet = int { with = konst_add }";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "constness is wrapper-wide and cannot be configured inside a facet",
            "{input}",
        );
        assert_eq!(
            error.span().start().column,
            input.find("konst_add").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn konst_flags_inherit_and_override_the_master_default() {
        let disabled = syn::parse_str::<Cfg>("konst = none")
            .expect("disabled konst should parse");
        assert!(!disabled.konst_for(&[
            Konsts::Add,
            Konsts::Math
        ]));

        let enabled = syn::parse_str::<Cfg>("konst = none, with = [konst_add]")
            .expect("an operation can opt into constness");
        assert!(enabled.konst_for(&[
            Konsts::Add,
            Konsts::Math
        ]));
        assert!(!enabled.konst_for(&[
            Konsts::Sub,
            Konsts::Math
        ]));

        let family = syn::parse_str::<Cfg>("konst = none, with = [konst_math]")
            .expect("a family can opt into constness");
        assert!(family.konst_for(&[
            Konsts::Add,
            Konsts::Math
        ]));

        let operation_override = syn::parse_str::<Cfg>(
            "konst = none, with = [konst_math], without = [konst_add]",
        )
        .expect("an operation can override its family");
        assert!(!operation_override.konst_for(&[
            Konsts::Add,
            Konsts::Math
        ]));
        assert!(operation_override.konst_for(&[
            Konsts::Sub,
            Konsts::Math
        ]));

        let enabled = syn::parse_str::<Cfg>("konst = nightly")
            .expect("enabled konst should parse");
        assert!(enabled.konst_for(&[
            Konsts::Add,
            Konsts::Math
        ]));

        let disabled =
            syn::parse_str::<Cfg>("konst = nightly, without = [konst_add]")
                .expect("an operation can opt out of constness");
        assert!(!disabled.konst_for(&[
            Konsts::Add,
            Konsts::Math
        ]));
        assert!(disabled.konst_for(&[
            Konsts::Sub,
            Konsts::Math
        ]));

        let map = syn::parse_str::<Cfg>("konst = none, with = [konst_map]")
            .expect("mapping can opt into constness");
        assert!(map.konst_for(&[Konsts::Map]));

        let map =
            syn::parse_str::<Cfg>("konst = nightly, without = [konst_map]")
                .expect("mapping can opt out of constness");
        assert!(!map.konst_for(&[Konsts::Map]));
    }

    #[test]
    fn default_konst_enables_crate_feature_flags() {
        let cfg = syn::parse_str::<Cfg>("")
            .expect("default konst configuration should parse");
        assert_eq!(cfg.konst, Konst::default());

        assert!(cfg.konst_for(&[Konsts::TryMake]));
        assert!(cfg.konst_for(&[
            Konsts::CoreInt,
            Konsts::Value
        ]));
        assert_eq!(
            cfg.konst_for(&[
                Konsts::Add,
                Konsts::Math
            ]),
            cfg!(feature = "nightly")
        );
        assert_eq!(
            cfg.konst_for(&[Konsts::Friendship]),
            cfg!(feature = "nightly")
        );
        assert_eq!(cfg.konst_friendship(), cfg!(feature = "nightly"));

        let cfg = syn::parse_str::<Cfg>("without = [konst_try_make]")
            .expect("explicit flags override the stable master default");
        assert!(!cfg.konst_for(&[Konsts::TryMake]));

        let cfg = syn::parse_str::<Cfg>("with = [konst_add]")
            .expect("explicit flags override the stable master default");
        assert!(cfg.konst_for(&[
            Konsts::Add,
            Konsts::Math
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
    fn defaults_layout_and_traits_and_allows_disabling_them() {
        let defaults = syn::parse_str::<Cfg>("konst = none")
            .expect("default integral configuration should parse");
        assert!(defaults.flags.clone);
        assert!(defaults.flags.copy);
        assert!(defaults.flags.debug);
        assert!(defaults.flags.hash);
        assert!(defaults.flags.repr);

        let enabled = syn::parse_str::<Cfg>(
            "konst = none, with = [clone, copy, debug, hash, repr]",
        )
        .expect("layout and traits should be configurable");
        assert!(enabled.flags.clone);
        assert!(enabled.flags.copy);
        assert!(enabled.flags.debug);
        assert!(enabled.flags.hash);
        assert!(enabled.flags.repr);

        let disabled = syn::parse_str::<Cfg>(
            "konst = none, without = [clone, copy, debug, hash, repr]",
        )
        .expect("layout and traits should be configurable");
        assert!(!disabled.flags.clone);
        assert!(!disabled.flags.copy);
        assert!(!disabled.flags.debug);
        assert!(!disabled.flags.hash);
        assert!(!disabled.flags.repr);
    }

    #[test]
    fn accepts_root_validation_attributes() {
        let config = syn::parse_str::<Cfg>(
            "konst = none, valid = is_even, domain = 1..=4",
        )
        .expect("root validation attributes should parse");

        assert_eq!(config.validation.callbacks.len(), 1);
        assert_eq!(
            config.validation.domain.as_ref().unwrap().ranges().len(),
            1,
        );
        assert!(config.validation.has_validation());
    }

    #[test]
    fn default_profile_preserves_integral_defaults() {
        let input = "konst = none, profile = int";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert_eq!(config.profile.arithmetic(), Arithmetic::Primitive);
        assert!(config.flags.math_add);
        assert!(config.flags.math_sub);
        assert!(config.flags.self_math);
        assert!(config.flags.self_bit);
        assert!(config.flags.ord);
        assert!(config.flags.partial_ord);
    }

    #[test]
    fn implicit_profile_uses_integral_behavior() {
        let input = "konst = none";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(config.profile.is_default());
        assert_eq!(config.profile.arithmetic(), Arithmetic::Primitive);
        assert!(!config.profile.is_circular());
        assert!(!config.profile.emits_precedes());
    }

    #[test]
    fn empty_profile_list_uses_integral_behavior() {
        let input = "konst = none, profile = []";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(config.profile.is_default());
        assert_eq!(config.profile.arithmetic(), Arithmetic::Primitive);
        assert!(config.flags.math_add);
        assert!(config.flags.ord);
    }

    #[test]
    fn bare_profile_preserves_following_root_flags() {
        let input = "
            konst = none,
            profile = circular,
            without = math_add
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert_eq!(config.profile.arithmetic(), Arithmetic::PrimitiveWrapping);
        assert!(config.flags.math_sub);
        assert!(!config.flags.math_add);
    }

    #[test]
    fn bare_profiles_mix_with_configured_profiles() {
        let input = "
            konst = none,
            profile = [circular, point { with = math_add }]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert_eq!(config.profile.arithmetic(), Arithmetic::PrimitiveWrapping);
        assert!(config.profile.emits_precedes());
        assert!(config.flags.math_add);
    }

    #[test]
    fn bare_profiles_require_separators() {
        let input = "profile = [circular point]";
        let this = syn::parse_str::<Cfg>(input);
        let error = this.unwrap_err();

        assert_eq!(error.span().start().column, input.find("point").unwrap(),);
    }

    #[test]
    fn bare_facet_preserves_following_root_flags() {
        let input = "
            konst = none,
            facet = [circular],
            without = math_add
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert_eq!(
            config.facets[0].profile.arithmetic(),
            Arithmetic::PrimitiveWrapping,
        );
        assert!(config.flags.math_sub);
        assert!(!config.flags.math_add);
    }

    #[test]
    fn bare_facets_keep_configured_dispatch() {
        let input = "
            konst = none,
            profile = circular {},
            facet = [
                int,
                sequential {
                    with = math_mul,
                    friend = offset_raw(Offset) -> Math,
                },
                circular,
            ],
            without = math_add
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();
        let slot = config
            .facet_slots
            .get(&FacetSlotKey {
                operator: Operator::Mul,
                rhs: "Offset".to_string(),
            })
            .unwrap();

        assert_eq!(slot.arithmetic, Arithmetic::Checked);
        assert!(!config.facet_slots.contains_key(&FacetSlotKey {
            operator: Operator::Add,
            rhs: "Offset".to_string(),
        }));
    }

    #[test]
    fn bare_facets_require_separators() {
        let input = "facet = [circular point]";
        let this = syn::parse_str::<Cfg>(input);
        let error = this.unwrap_err();

        assert_eq!(error.span().start().column, input.find("point").unwrap(),);
    }

    #[test]
    fn point_profile_restricts_integer_operations() {
        let input = "konst = none, profile = point {}";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert_eq!(config.profile.arithmetic(), Arithmetic::Primitive);
        assert!(config.flags.math_sub);
        assert!(!config.flags.math_add);
        assert!(!config.flags.assign_add);
        assert!(!config.flags.math_mul);
        assert!(!config.flags.self_friend_math_ops);
        assert!(!config.flags.self_friend_math_bit);
        assert!(!config.flags.self_friend_rel);
        assert!(!config.flags.self_bit);
        assert!(!config.flags.bit_access);
        assert!(!config.flags.core_int);
        assert!(config.flags.ord);
        assert!(config.flags.partial_ord);
    }

    #[test]
    fn point_profile_preserves_circular_policy() {
        let input = "
            konst = none,
            profile = [circular {}, point {}]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert_eq!(config.profile.arithmetic(), Arithmetic::PrimitiveWrapping);
        assert!(config.profile.is_circular());
        assert!(config.profile.emits_precedes());
        assert!(!config.flags.ord);
        assert!(!config.flags.partial_ord);
        assert!(!config.flags.math_add);
        assert!(config.flags.math_sub);
    }

    #[test]
    fn point_profile_preserves_delta_policy() {
        let input = "
            konst = none,
            profile = [delta, point]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(config.profile.is_circular());
        assert!(!config.profile.emits_precedes());
    }

    #[test]
    fn point_profile_preserves_checked_policy() {
        let input = "
            konst = none,
            profile = [sequential {}, point {}]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert_eq!(config.profile.arithmetic(), Arithmetic::Checked);
        assert!(config.flags.ord);
        assert!(config.flags.partial_ord);
        assert!(!config.flags.math_add);
    }

    #[test]
    fn point_profile_preserves_wrapping_storage() {
        let input = "
            konst = none,
            profile = [wrapping {}, point {}]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert_eq!(config.profile.arithmetic(), Arithmetic::WrapperWrapping);
        assert!(config.requires_wrapping_storage());
        assert!(config.flags.ord);
        assert!(!config.flags.math_add);
    }

    #[test]
    fn point_profile_allows_explicit_addition() {
        let input = "
            konst = none,
            profile = point { with = assign_add },
            with = math_add
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(config.flags.math_add);
        assert!(config.flags.assign_add);
    }

    #[test]
    fn point_profile_resets_earlier_addition_overrides() {
        let input = "
            konst = none,
            profile = [int { with = [math_add, assign_add] }, point {}],
            produce = Point::into_offset(_) -> Offset
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(!config.flags.math_add);
        assert!(!config.flags.assign_add);
        assert!(config.flags.math_sub);
    }

    #[test]
    fn point_profile_preserves_circular_ordering_restriction() {
        let input = "
            profile = [circular {}, point {}],
            with = ord
        ";
        let this = syn::parse_str::<Cfg>(input);
        let error = this.unwrap_err();

        assert!(error.to_string().contains("linear ordering"), "{input}");
    }

    #[test]
    fn point_profile_rejects_precedes_configuration() {
        let input = "profile = point { precedes = true }";
        let this = syn::parse_str::<Cfg>(input);
        let error = this.unwrap_err();

        assert!(error.to_string().contains("unknown attribute"), "{input}");
    }

    #[test]
    fn point_facet_inherits_root_arithmetic() {
        let input = "
            konst = none,
            profile = circular {},
            facet = [
                point {
                    with = math_add,
                    friend = offset_raw(Offset) -> Math,
                },
            ]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();
        let slot = config
            .facet_slots
            .get(&FacetSlotKey {
                operator: Operator::Add,
                rhs: "Offset".to_string(),
            })
            .unwrap();

        assert_eq!(slot.arithmetic, Arithmetic::PrimitiveWrapping);
    }

    #[test]
    fn circular_profile_resolves_restricted_defaults() {
        let input = "
            konst = none,
            profile = circular {
                precedes = false,
            }
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(config.profile.is_circular());
        assert!(!config.profile.emits_precedes());
        assert_eq!(config.profile.arithmetic(), Arithmetic::PrimitiveWrapping);
        assert!(config.flags.math_add);
        assert!(config.flags.math_sub);
        assert!(config.flags.assign_add);
        assert!(config.flags.assign_sub);
        assert!(!config.flags.math_mul);
        assert!(!config.flags.math_and);
        assert!(!config.flags.self_bit);
        assert!(!config.flags.bit_access);
        assert!(!config.flags.core_int);
        assert!(!config.flags.ord);
        assert!(!config.flags.partial_ord);
    }

    #[test]
    fn delta_profile_inherits_circular_defaults_without_precedes() {
        let input = "konst = none, profile = delta";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(config.profile.is_circular());
        assert!(!config.profile.emits_precedes());
        assert_eq!(config.profile.arithmetic(), Arithmetic::PrimitiveWrapping);
        assert!(config.flags.math_add);
        assert!(config.flags.math_sub);
        assert!(config.flags.assign_add);
        assert!(config.flags.assign_sub);
        assert!(!config.flags.math_mul);
        assert!(!config.flags.core_int);
        assert!(!config.flags.ord);
        assert!(!config.flags.partial_ord);

        let expansion = expansion_of(input);
        assert!(!expansion.contains("precedes"));
    }

    #[test]
    fn delta_profile_accepts_circular_operation_settings() {
        let input = "
            konst = none,
            profile = delta {
                with = math_mul,
            }
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(config.flags.math_mul);
        assert!(!config.profile.emits_precedes());
    }

    #[test]
    fn delta_profile_treats_precedes_as_unknown() {
        let input = "
            profile = delta {
                precedes = true,
            }
        ";
        let this = syn::parse_str::<Cfg>(input);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "unknown attribute", "{input}");
    }

    #[test]
    fn sequential_profile_resolves_checked_policy() {
        let input = "konst = none, profile = sequential {}";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(!config.profile.is_circular());
        assert_eq!(config.profile.arithmetic(), Arithmetic::Checked);
        assert!(config.flags.math_add);
        assert!(config.flags.math_sub);
        assert!(!config.flags.math_mul);
        assert!(!config.flags.math_xor);
        assert!(!config.flags.self_bit);
        assert!(config.flags.ord);
        assert!(config.flags.partial_ord);
    }

    #[test]
    fn wrapping_profile_retains_integral_defaults_and_requests_storage() {
        let input = "konst = none, profile = wrapping {}";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(!config.profile.is_circular());
        assert!(config.profile.requires_wrapping_storage());
        assert_eq!(config.profile.arithmetic(), Arithmetic::WrapperWrapping);
        assert!(config.flags.math_add);
        assert!(config.flags.math_mul);
        assert!(config.flags.math_xor);
        assert!(config.flags.self_bit);
        assert!(config.flags.core_int);
        assert!(config.flags.ord);
        assert!(config.flags.partial_ord);
    }

    #[test]
    fn wrapping_arithmetic_uses_wrapper_ops_with_a_const_fallback() {
        let runtime = Arithmetic::WrapperWrapping
            .make_expr(crate::zz_value_type::ArithmeticOp::Add)
            .to_string();
        let konst = Arithmetic::WrapperWrapping
            .make_const_expr(crate::zz_value_type::ArithmeticOp::Add)
            .to_string();

        assert!(
            runtime.contains("Wrapping (lhs) + :: core :: num :: Wrapping")
        );
        assert!(konst.contains("lhs . wrapping_add (rhs)"));
    }

    #[test]
    fn effective_profile_selects_wrapping_storage() {
        let wrapping = "
            konst = none,
            profile = [circular {}, wrapping {}]
        ";
        let circular = "
            konst = none,
            profile = [wrapping {}, circular {}]
        ";
        let wrapping = syn::parse_str::<Cfg>(wrapping).unwrap();
        let circular = syn::parse_str::<Cfg>(circular).unwrap();

        assert!(wrapping.profile.requires_wrapping_storage());
        assert!(!circular.profile.requires_wrapping_storage());
        assert_eq!(
            circular.profile.arithmetic(),
            Arithmetic::PrimitiveWrapping
        );
    }

    #[test]
    fn wrapping_facet_selects_wrapper_wide_storage_after_resolution() {
        let input = "
            konst = none,
            facet = [
                wrapping {
                    friend = offset(Offset) -> Math,
                },
                sequential {
                    friend = checked(Checked) -> Math,
                },
            ]
        ";
        let config = syn::parse_str::<Cfg>(input).unwrap();

        assert!(!config.profile.requires_wrapping_storage());
        assert!(config.facets[0].requires_wrapping_storage());
        assert!(!config.facets[1].requires_wrapping_storage());
        assert!(config.requires_wrapping_storage());

        let wrapping = config
            .facet_slots
            .get(&FacetSlotKey {
                operator: Operator::Add,
                rhs: "Offset".to_string(),
            })
            .unwrap();
        let checked = config
            .facet_slots
            .get(&FacetSlotKey {
                operator: Operator::Add,
                rhs: "Checked".to_string(),
            })
            .unwrap();

        assert_eq!(wrapping.arithmetic, Arithmetic::WrapperWrapping);
        assert_eq!(checked.arithmetic, Arithmetic::Checked);
    }

    #[test]
    fn root_wrapping_requirement_survives_non_wrapping_facets() {
        let input = "
            konst = none,
            profile = wrapping {},
            facet = [
                int {
                    friend = offset(Offset) -> Math,
                },
            ]
        ";
        let config = syn::parse_str::<Cfg>(input).unwrap();

        assert!(config.profile.requires_wrapping_storage());
        assert!(!config.facets[0].requires_wrapping_storage());
        assert!(config.requires_wrapping_storage());
    }

    #[test]
    fn root_flags_override_profile_body_regardless_of_attribute_order() {
        let root_before_profile = "
            konst = none,
            with = [math_add],
            profile = circular {
                without = [math_add],
            }
        ";
        let profile_before_root = "
            konst = none,
            profile = circular {
                without = [math_add],
            },
            with = [math_add]
        ";
        let first = syn::parse_str::<Cfg>(root_before_profile);
        let second = syn::parse_str::<Cfg>(profile_before_root);
        assert!(first.is_ok(), "{root_before_profile}");
        assert!(second.is_ok(), "{profile_before_root}");
        let first = first.unwrap();
        let second = second.unwrap();

        assert!(first.flags.math_add);
        assert!(second.flags.math_add);
        assert_eq!(first.flags.math_add, second.flags.math_add);
    }

    #[test]
    fn profile_list_applies_profiles_in_order() {
        let input = "
            konst = none,
            profile = [
                sequential {
                    without = math_add,
                    with = [math_mul, bit_access],
                },
                circular {
                    with = core_int,
                    without = math_sub,
                    precedes = false,
                },
            ]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert_eq!(config.profile.arithmetic(), Arithmetic::PrimitiveWrapping);
        assert!(config.profile.is_circular());
        assert!(!config.profile.emits_precedes());
        assert!(!config.flags.math_add);
        assert!(!config.flags.math_mul);
        assert!(!config.flags.bit_access);
        assert!(config.flags.core_int);
        assert!(!config.flags.math_sub);
    }

    #[test]
    fn later_profile_body_overrides_earlier_profile_body() {
        let input = "
            konst = none,
            profile = [
                int {
                    without = math_add,
                },
                int {
                    with = math_add,
                },
            ]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(config.flags.math_add);
    }

    #[test]
    fn parses_explicit_facets_and_normalizes_the_root_fallback() {
        let input = "
            konst = none,
            profile = sequential {
                without = math_add,
            },
            with = math_add,
            friend = root_conv(RootRhs) -> Math,
            produce = Point::into_root(_) -> RootOutput,
            facet = [
                circular {
                    friend = offset_conv(Offset) -> Math,
                    with = [math_add, math_sub],
                    produce = Self,
                },
                int {
                    friend = bits_conv(Bits) -> Bit,
                    without = math_sub,
                },
            ]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert_eq!(config.facets.len(), 2);
        assert_eq!(
            config.facets[0].profile.arithmetic(),
            Arithmetic::PrimitiveWrapping,
        );
        assert_eq!(
            config.facets[1].profile.arithmetic(),
            Arithmetic::Primitive,
        );
        assert!(
            config.facets[0]
                .ty_out
                .as_ref()
                .is_some_and(|it| it.is_self())
        );
        assert!(config.facets[1].ty_out.is_none());

        let explicit_friend = config.facets[0].friends.iter().next().unwrap();
        assert!(
            explicit_friend
                .capabilities
                .contains(&format_ident!("Math"))
        );
        assert!(
            !explicit_friend
                .capabilities
                .contains(&format_ident!("Make"))
        );
        assert!(!explicit_friend.capabilities.contains(&format_ident!("Bit")));

        let default = config.default_facet.as_ref().unwrap();
        assert_eq!(default.profile.arithmetic(), Arithmetic::Checked);
        assert!(default.flags.math_add);
        assert!(!default.ty_out.is_self());
        let root_friend = default.friends.iter().next().unwrap();
        assert!(root_friend.capabilities.contains(&format_ident!("Math")));
        assert!(root_friend.capabilities.contains(&format_ident!("Make")));
        assert!(root_friend.capabilities.contains(&format_ident!("Bit")));
    }

    #[test]
    fn facet_friend_defaults_to_unexpanded_math() {
        let input = "
            konst = none,
            facet = int {
                friend = convert(Source)
            }
        ";
        let result = syn::parse_str::<Cfg>(input);
        assert!(result.is_ok(), "{input}");
        let config = result.unwrap();
        let friend = config.facets[0].friends.iter().next().unwrap();
        let expected = [format_ident!("Math")].set();

        assert_eq!(friend.capabilities, expected, "{input}");
    }

    #[test]
    fn empty_facets_still_materialize_the_default_facet() {
        let input = "konst = none, facet = []";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(config.facets_present());
        assert!(config.facets.is_empty());
        assert!(config.default_facet.is_some());
    }

    #[test]
    fn omitted_facets_do_not_materialize_facet_configuration() {
        let input = "konst = none";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(!config.facets_present());
        assert!(config.facets.is_empty());
        assert!(config.default_facet.is_none());
    }

    #[test]
    fn single_facet_preserves_following_root_flags() {
        let input = "
            konst = none,
            facet = circular,
            without = math_add
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert_eq!(
            config.facets[0].profile.arithmetic(),
            Arithmetic::PrimitiveWrapping,
        );
        assert!(config.flags.math_sub);
        assert!(!config.flags.math_add);
    }

    #[test]
    fn single_configured_facet_preserves_dispatch() {
        let input = "
            konst = none,
            profile = circular,
            facet = sequential {
                with = math_add,
                friend = offset_raw(Offset) -> Math,
                produce = Self,
            },
            without = math_add,
            produce = Point::into_offset(_) -> Offset
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();
        let slot = config
            .facet_slots
            .get(&FacetSlotKey {
                operator: Operator::Add,
                rhs: "Offset".to_string(),
            })
            .unwrap();

        assert_eq!(slot.arithmetic, Arithmetic::Checked);
        assert!(slot.ty_out.is_self());
        assert!(!config.ty_out.is_self());
        assert!(!config.flags.math_add);
    }

    #[test]
    fn friend_generation_flag_uses_singular_name() {
        let input = "konst = none, without = friend";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        assert!(!config.flags.friend);
    }

    #[test]
    fn rejects_plural_friend_argument() {
        let input = "friends = self(u8) -> Make";
        let this = syn::parse_str::<Cfg>(input);

        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_plural_facet_argument() {
        let input = "facets = circular";
        let this = syn::parse_str::<Cfg>(input);

        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_plural_friend_flag() {
        let input = "without = friends";
        let this = syn::parse_str::<Cfg>(input);

        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_duplicate_facet_fields() {
        let input = "
            facet = [
                int {
                    produce = Self,
                    produce = Self,
                },
            ]
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(error.to_string(), "duplicated arg", "{input}");
    }

    #[test]
    fn resolves_explicit_slots_and_retains_unclaimed_defaults() {
        let input = "
            konst = none,
            profile = circular {},
            friend = offset_conv(Offset) -> Math,
            produce = Point::into_offset(_) -> RootOutput,
            facet = [
                sequential {
                    friend = offset_conv(Offset) -> Math,
                    without = math_sub,
                    produce = Self,
                },
                circular {
                    with = [self_math, math_sub],
                    without = math_add,
                    produce = Point::into_self_offset(_) -> SelfOutput,
                },
            ]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();

        let add_offset = config
            .facet_slots
            .get(&FacetSlotKey {
                operator: Operator::Add,
                rhs: "Offset".to_string(),
            })
            .unwrap();
        assert_eq!(add_offset.owner, FacetOwner::Explicit(0));
        assert_eq!(add_offset.arithmetic, Arithmetic::Checked);
        assert!(add_offset.ty_out.is_self());

        let sub_offset = config
            .facet_slots
            .get(&FacetSlotKey {
                operator: Operator::Sub,
                rhs: "Offset".to_string(),
            })
            .unwrap();
        assert_eq!(sub_offset.owner, FacetOwner::Default);
        assert_eq!(sub_offset.arithmetic, Arithmetic::PrimitiveWrapping);
        assert!(!sub_offset.ty_out.is_self());

        let add_self = config
            .facet_slots
            .get(&FacetSlotKey {
                operator: Operator::Add,
                rhs: "Self".to_string(),
            })
            .unwrap();
        assert_eq!(add_self.owner, FacetOwner::Default);

        let sub_self = config
            .facet_slots
            .get(&FacetSlotKey {
                operator: Operator::Sub,
                rhs: "Self".to_string(),
            })
            .unwrap();
        assert_eq!(sub_self.owner, FacetOwner::Explicit(1));
        assert_eq!(
            sub_self.ty_out.target.to_token_stream().to_string(),
            "SelfOutput",
        );
        assert_eq!(config.facet_slots.len(), 4);
    }

    #[test]
    fn rejects_duplicate_explicit_facet_slots() {
        let input = "
            konst = none,
            facet = [
                circular {
                    friend = first(Offset) -> Math,
                },
                sequential {
                    friend = second(Offset) -> Math,
                },
            ]
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(error.to_string(), "duplicate explicit facet slot",);
        assert_eq!(error.into_iter().count(), 2);
    }

    #[test]
    fn permits_distinct_operations_for_the_same_rhs() {
        let input = "
            konst = none,
            facet = [
                circular {
                    friend = first(Offset) -> Math,
                    without = math_sub,
                },
                circular {
                    friend = second(Offset) -> Math,
                    without = math_add,
                },
            ]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn rejects_unsupported_facet_friend_capabilities() {
        let input = "
            konst = none,
            facet = [
                int {
                    friend = make(Source) -> Make,
                },
            ]
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "facet friends only support `Math` and `Bit` capabilities",
        );
    }

    #[test]
    fn rejects_wrapper_wide_precedes_in_a_facet() {
        let input = "
            konst = none,
            facet = [
                circular {
                    precedes = false,
                    friend = convert(Source) -> Math,
                },
            ]
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "`precedes` is wrapper-wide and cannot be configured inside a facet",
        );
    }

    #[test]
    fn rejects_wrapper_wide_flags_in_a_facet() {
        let input = "
            konst = none,
            facet = [
                int {
                    friend = convert(Source) -> Math,
                    with = display,
                },
            ]
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "attribute is wrapper-wide or has no operand slot and cannot be configured inside a facet",
        );
    }

    #[test]
    fn rejects_facet_assignment_requests() {
        let input = "
            konst = none,
            facet = [
                int {
                    friend = convert(Source) -> Math,
                    with = assign_add,
                },
            ]
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "facets do not support assignment operators",
        );
    }

    #[test]
    fn rejects_contradictory_facet_flags() {
        let input = "
            konst = none,
            facet = [
                int {
                    friend = convert(Source) -> Math,
                    with = math_add,
                    without = math_add,
                },
            ]
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "flag specified in both `with` and `without`",
        );
    }

    #[test]
    fn omitted_facets_emit_global_friendship_and_operators() {
        let expansion = expansion_of(
            "
                konst = none,
                friend = convert(Offset) -> [Math, Rel]
            ",
        );

        assert!(expansion.contains("trait Math : Seal"));
        assert!(expansion.contains("trait Bit : Seal"));
        assert!(expansion.contains("impl Math for Offset"));
        assert!(
            expansion
                .contains("impl < T > :: core :: ops :: Add < T > for Point")
        );
    }

    #[test]
    fn facets_emit_dispatch_traits_slots_and_blanket_operators() {
        let expansion = expansion_of(
            "
                konst = none,
                profile = circular {},
                friend = convert(Offset) -> [Math, Rel],
                facet = [
                    circular {
                        friend = other(Other) -> Math,
                    },
                ]
            ",
        );

        assert!(expansion.contains("pub trait AddRhs"));
        assert!(expansion.contains("pub trait SubRhs"));
        assert!(!expansion.contains("trait Math : Seal"));
        assert!(!expansion.contains("trait Bit : Seal"));
        assert!(!expansion.contains("impl Math for Offset"));
        assert!(!expansion.contains("impl Bit for Offset"));
        assert!(
            expansion
                .contains("impl < T > :: core :: ops :: Add < T > for Point")
        );
        assert!(expansion.contains("T : AddRhs"));
        assert!(expansion.contains("impl AddRhs for Other"));
        assert!(expansion.contains("impl AddRhs for Offset"));
        assert!(expansion.contains("impl AddRhs for Point"));
        assert!(!expansion.contains(":: core :: ops :: Add for Point"));
        assert!(!expansion.contains(":: core :: ops :: AddAssign"));
    }

    #[test]
    fn faceted_friendship_retains_make_and_rel_capabilities() {
        let expansion = expansion_of(
            "
                konst = none,
                friend = convert(Offset) -> [Math, Rel],
                facet = []
            ",
        );

        assert!(expansion.contains("trait Make : Seal"));
        assert!(expansion.contains("trait Rel : Seal"));
        assert!(expansion.contains("impl Make for Offset"));
        assert!(expansion.contains("impl Rel for Offset"));
    }

    #[test]
    fn parses_produce_conversion_with_generic_target() {
        let input = "
            konst = none,
            produce = crate::Point::into_offset(_) -> Offset<u32>
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();
        assert_eq!(
            config.ty_out.conversion.to_token_stream().to_string(),
            "crate :: Point :: into_offset",
        );
        assert_eq!(
            config.ty_out.target.to_token_stream().to_string(),
            "Offset < u32 >",
        );
        assert!(!config.ty_out.is_self());
        assert!(!config.flags.assign_add);
        assert!(!config.flags.assign_sub);
        assert!(!config.flags.assign_and);
        assert!(!config.flags.assign_or);
    }

    #[test]
    fn defaults_output_type_to_self_converter() {
        let config = syn::parse_str::<Cfg>("konst = none")
            .expect("default configuration should parse");

        assert!(config.ty_out.is_self());
        assert_eq!(config.ty_out.target.to_token_stream().to_string(), "Self");
        assert!(config.flags.assign_add);
        assert!(config.flags.assign_sub);
        assert!(config.flags.assign_and);
        assert!(config.flags.assign_or);
    }

    #[test]
    fn self_output_type_preserves_assignment_operators() {
        let input = "
            produce = Self(_) -> Self,
            with = [assign_add]
        ";
        let config = syn::parse_str::<Cfg>(input)
            .expect("Self output type should parse");

        assert!(config.ty_out.is_self());
        assert!(config.flags.assign_add);
    }

    #[test]
    fn nested_integral_configuration_uses_profile_parser() {
        struct Nested(Cfg);

        impl syn::parse::Parse for Nested {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                let mut it = Cfg::parse_with_konst_unresolved(input)?;
                it.resolve()?;
                return Ok(Self(it));
            }
        }

        let input = "
            profile = circular {
                precedes = true,
            }
        ";
        let this = syn::parse_str::<Nested>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap().0;

        assert!(config.profile.is_circular());
        assert!(config.profile.emits_precedes());
    }

    #[test]
    fn rejects_unknown_profile() {
        let input = "profile = affine {}";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("unknown profiles should be rejected");

        assert_eq!(error.to_string(), "unknown profile");
    }

    #[test]
    fn default_profile_treats_precedes_as_unknown() {
        let input = "
            profile = int {
                precedes = true,
            }
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(error.to_string(), "unknown attribute", "{input}");
    }

    #[test]
    fn sequential_profile_treats_precedes_as_unknown() {
        let input = "
            profile = sequential {
                precedes = true,
            }
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(error.to_string(), "unknown attribute", "{input}");
    }

    #[test]
    fn wrapping_profile_treats_precedes_as_unknown() {
        let input = "
            profile = wrapping {
                precedes = true,
            }
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(error.to_string(), "unknown attribute", "{input}");
    }

    #[test]
    fn rejects_self_produce_conversion_source() {
        let input = "produce = Point::into_offset(Self) -> Offset";
        let error = syn::parse_str::<Cfg>(input).expect_err(
            "produce callbacks must take an underscore placeholder",
        );

        assert_eq!(
            error.to_string(),
            "type conversion requires a `_` argument",
        );
    }

    #[test]
    fn rejects_linear_ordering_in_circular_profile() {
        let input = "
            profile = circular {},
            with = [ord]
        ";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("circular ordering must not be enabled");

        assert_eq!(
            error.to_string(),
            "circular profiles do not support linear ordering",
        );
    }

    #[test]
    fn rejects_range_for_validated_integrals() {
        let input = "
            valid = is_valid,
            with = range
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
        let error = this.unwrap_err();

        assert_eq!(
            error.to_string(),
            "range requires an unvalidated integral type",
        );
    }

    #[test]
    fn rejects_range_for_circular_profiles() {
        let input = "
            profile = circular {},
            with = range
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
        let error = this.unwrap_err();

        assert_eq!(
            error.to_string(),
            "range is incompatible with circular profiles",
        );
    }

    #[test]
    fn rejects_assignment_override_for_produce() {
        let input = "
            produce = Point::into_offset(_) -> Offset,
            with = [assign_add]
        ";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("produce cannot retain assignment operators");

        assert_eq!(
            error.to_string(),
            "produce cannot generate assignment operators",
        );
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
        let input = "konst = none, valid = [is_even, is_not_fifty], domain = [1..=4, 8..10, ..=0]";
        let config = parse_test_config(input);
        let domain = config.validation.domain.as_ref().unwrap();
        let included =
            domain.contains_at_pointer_width(Int::NonNegative(0), 16);
        let excluded =
            domain.contains_at_pointer_width(Int::NonNegative(5), 16);

        assert_eq!(config.validation.callbacks.len(), 2, "{input}");
        assert!(included, "{input}");
        assert!(!excluded, "{input}");
    }

    #[test]
    fn primitive_integer_constant_domain_preserves_boundaries() {
        let input = "konst = none, domain = [1..=u32::MAX, u32::MAX..]";
        let config = parse_test_config(input);
        let domain = config.validation.domain.as_ref().unwrap();
        let zero = domain.contains_at_pointer_width(Int::NonNegative(0), 16);
        let maximum = domain
            .contains_at_pointer_width(Int::NonNegative(u32::MAX as u128), 16);

        assert!(!zero, "{input}");
        assert!(maximum, "{input}");
    }

    #[test]
    fn pointer_domain_excludes_16_bit_minimum() {
        let input =
            "konst = none, domain = [isize::MIN..-32_768, -isize::MAX..]";
        let config = parse_test_config(input);
        let domain = config.validation.domain.as_ref().unwrap();
        let minimum = Int::Negative(32_768);
        let contains_minimum = domain.contains_at_pointer_width(minimum, 16);

        assert!(!contains_minimum, "{input}");
    }

    #[test]
    fn pointer_domain_excludes_32_bit_minimum() {
        let input = "konst = none, domain = [isize::MIN..-2_147_483_648, -isize::MAX..]";
        let config = parse_test_config(input);
        let domain = config.validation.domain.as_ref().unwrap();
        let minimum = Int::Negative(2_147_483_648);
        let contains_minimum = domain.contains_at_pointer_width(minimum, 32);

        assert!(!contains_minimum, "{input}");
    }

    #[test]
    fn pointer_domain_includes_64_bit_minimum() {
        let input = "konst = none, domain = [isize::MIN..-2_147_483_648, -isize::MAX..]";
        let config = parse_test_config(input);
        let domain = config.validation.domain.as_ref().unwrap();
        let minimum = Int::Negative(9_223_372_036_854_775_808);
        let contains_minimum = domain.contains_at_pointer_width(minimum, 64);

        assert!(contains_minimum, "{input}");
    }

    #[test]
    fn pointer_domain_includes_32_bit_minimum() {
        let input = "konst = none, domain = [-isize::MAX.., -2_147_483_648..=-2_147_483_648]";
        let config = parse_test_config(input);
        let domain = config.validation.domain.as_ref().unwrap();
        let minimum = Int::Negative(2_147_483_648);
        let contains_minimum = domain.contains_at_pointer_width(minimum, 32);

        assert!(contains_minimum, "{input}");
    }

    #[test]
    fn singleton_domain_excludes_64_bit_minimum() {
        let input = "konst = none, domain = [-isize::MAX.., -2_147_483_648..=-2_147_483_648]";
        let config = parse_test_config(input);
        let domain = config.validation.domain.as_ref().unwrap();
        let minimum = Int::Negative(9_223_372_036_854_775_808);
        let contains_minimum = domain.contains_at_pointer_width(minimum, 64);

        assert!(!contains_minimum, "{input}");
    }

    #[test]
    fn fixed_domain_preserves_boundary_membership() {
        let input = "konst = none, domain = [10..=12, 1..3, 3..=5, 6..10]";
        let config = parse_test_config(input);
        let domain = config.validation.domain.as_ref().unwrap();
        let below = domain.contains_at_pointer_width(Int::NonNegative(0), 16);
        let lower = domain.contains_at_pointer_width(Int::NonNegative(1), 16);
        let upper = domain.contains_at_pointer_width(Int::NonNegative(12), 16);
        let above = domain.contains_at_pointer_width(Int::NonNegative(13), 16);

        assert!(!below, "{input}");
        assert!(lower, "{input}");
        assert!(upper, "{input}");
        assert!(!above, "{input}");
    }

    #[test]
    fn rejects_path_range_bound() {
        let input = "
            konst = none,
            domain = [MAX..4]
        ";
        let error =
            syn::parse_str::<Cfg>(input).expect_err("need an integer constant");
        assert_eq!(error.to_string(), "need an integer constant");
    }

    #[test]
    fn rejects_expression_range_bound() {
        let input = "
            konst = none,
            domain = [1 + 1..4]
        ";
        let error =
            syn::parse_str::<Cfg>(input).expect_err("need an integer constant");
        assert_eq!(error.to_string(), "need an integer constant");
    }

    #[test]
    fn accepts_unbracketed_root_domain_with_valid_callback() {
        let config =
            syn::parse_str::<Cfg>("konst = none, domain = 1..2, valid = foo")
                .expect(
                    "an unbracketed range with a valid callback should parse",
                );

        assert_eq!(config.validation.callbacks.len(), 1);
        assert_eq!(
            config.validation.domain.as_ref().unwrap().ranges().len(),
            1,
        );
    }

    #[test]
    fn rejects_non_range_domain_member() {
        let error = syn::parse_str::<Cfg>("konst = none, domain = [1]")
            .expect_err("domain validation needs range expressions");

        assert_eq!(error.to_string(), "invalid range definition");
    }

    #[test]
    fn rejects_plus_separated_ranges() {
        syn::parse_str::<Cfg>("konst = none, domain = [1..2 + 3..4]")
            .expect_err("multiple ranges use comma separators");
    }

    fn assert_domain_zero_membership(
        source: &str,
        expected: bool,
    ) {
        let input = format!("konst = none, domain = {source}");
        let config = parse_test_config(&input);
        let domain = config.validation.domain.as_ref().unwrap();
        assert_eq!(domain.contains_zero(), expected, "{input}");
        assert_eq!(domain.excludes_zero(), !expected, "{input}");
    }

    #[test]
    fn empty_domain_excludes_zero() {
        assert_domain_zero_membership("[]", false);
    }

    #[test]
    fn unbounded_domain_contains_zero() {
        assert_domain_zero_membership("..", true);
    }

    #[test]
    fn open_zero_upper_bound_excludes_zero() {
        assert_domain_zero_membership("..0", false);
    }

    #[test]
    fn closed_zero_upper_bound_contains_zero() {
        assert_domain_zero_membership("..=0", true);
    }

    #[test]
    fn positive_domain_excludes_zero() {
        assert_domain_zero_membership("1..", false);
    }

    #[test]
    fn negative_domain_excludes_zero() {
        assert_domain_zero_membership("-5..0", false);
    }

    #[test]
    fn split_domain_excludes_zero() {
        assert_domain_zero_membership("[-5..0, 1..]", false);
    }

    #[test]
    fn adjacent_negative_domains_exclude_zero_after_merging() {
        assert_domain_zero_membership("[-10..=-5, -4..0]", false);
    }

    #[test]
    fn domain_union_contains_zero_when_any_member_does() {
        assert_domain_zero_membership("[..=-1, 0..]", true);
    }

    #[test]
    fn unsigned_min_constant_is_zero() {
        assert_domain_zero_membership("u32::MIN..=u32::MIN", true);
    }

    #[test]
    fn signed_min_constant_is_negative() {
        assert_domain_zero_membership("i32::MIN..0", false);
    }

    #[test]
    fn pointer_domain_excluding_zero_infers_non_zero_storage() {
        let input = "konst = none, domain = [isize::MIN..0, 1..=isize::MAX]";
        let config = parse_test_config(input);
        let contains_zero =
            config.validation.domain.as_ref().unwrap().contains_zero();
        let requires_non_zero = config.requires_non_zero_storage();

        assert!(!contains_zero, "{input}");
        assert!(requires_non_zero, "{input}");
    }

    #[test]
    fn pointer_domain_including_zero_uses_ordinary_storage() {
        let input = "konst = none, domain = isize::MIN..=isize::MAX";
        let config = parse_test_config(input);
        let contains_zero =
            config.validation.domain.as_ref().unwrap().contains_zero();
        let requires_non_zero = config.requires_non_zero_storage();

        assert!(contains_zero, "{input}");
        assert!(!requires_non_zero, "{input}");
    }

    #[test]
    fn unsigned_pointer_minimum_domain_includes_zero() {
        let input = "konst = none, domain = usize::MIN..=usize::MIN";
        let config = parse_test_config(input);
        let contains_zero =
            config.validation.domain.as_ref().unwrap().contains_zero();
        let requires_non_zero = config.requires_non_zero_storage();

        assert!(contains_zero, "{input}");
        assert!(!requires_non_zero, "{input}");
    }

    #[test]
    fn resolves_non_zero_as_a_tri_state_policy() {
        let unspecified =
            syn::parse_str::<Cfg>("konst = none, valid = opaque").unwrap();
        assert_eq!(unspecified.non_zero, NonZeroPolicy::Infer);
        assert!(!unspecified.requires_non_zero_storage());

        let inferred =
            syn::parse_str::<Cfg>("konst = none, domain = [2..10, 100..123]")
                .unwrap();
        assert_eq!(inferred.non_zero, NonZeroPolicy::Infer);
        assert!(inferred.requires_non_zero_storage());

        let required =
            syn::parse_str::<Cfg>("konst = none, with = non_zero").unwrap();
        assert_eq!(required.non_zero, NonZeroPolicy::Require);
        assert!(required.requires_non_zero_storage());
        assert!(
            required
                .validation
                .emit(&quote::quote! { value })
                .to_string()
                .contains("value != 0")
        );

        let suppressed = syn::parse_str::<Cfg>(
            "konst = none, domain = 1.., without = non_zero",
        )
        .unwrap();
        assert_eq!(suppressed.non_zero, NonZeroPolicy::Suppress);
        assert!(!suppressed.requires_non_zero_storage());
        assert!(suppressed.validation.has_validation());
    }

    #[test]
    fn root_non_zero_overrides_profile_setting() {
        let config = syn::parse_str::<Cfg>(
            "
                konst = none,
                without = non_zero,
                profile = int {
                    with = non_zero,
                }
            ",
        )
        .unwrap();

        assert_eq!(config.non_zero, NonZeroPolicy::Suppress);
    }

    #[test]
    fn rejects_facet_local_non_zero() {
        let error = syn::parse_str::<Cfg>(
            "
                konst = none,
                facet = [
                    int {
                        with = non_zero,
                    },
                ]
            ",
        )
        .expect_err("facets cannot choose physical storage");

        assert_eq!(
            error.to_string(),
            "`non_zero` is wrapper-wide and cannot be configured inside a facet",
        );
    }

    #[test]
    fn defaults_missing_konst_to_crate_default() {
        let config = syn::parse_str::<Cfg>("friend = [Foo(u8) -> Bit]")
            .expect("konst should default to the crate feature");

        assert_eq!(config.konst, Konst::default());
        assert!(config.konst_for(&[Konsts::TryMake]));
        assert_eq!(
            config.konst_for(&[Konsts::Math]),
            cfg!(feature = "nightly")
        );
    }

    #[test]
    fn accepts_explicit_friend_capabilities() {
        let config = syn::parse_str::<Cfg>(
            "konst = none, friend = [convert(u8) -> [Make, Bit]]",
        )
        .expect("integral capabilities should parse");
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.capabilities.contains(&syn::parse_quote!(Make)));
        assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));
    }

    #[test]
    fn expands_math_friend_capability_without_rel() {
        let config =
            syn::parse_str::<Cfg>("konst = none, friend = [Foo(u8) -> Math]")
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
            "konst = none, friend = [convert(Source), Foo(Other)]",
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
            syn::parse_str::<Cfg>("konst = none, friend = Foo(Source) -> []")
                .expect("an explicit empty capability list should parse");
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.capabilities.is_empty());
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
    fn accepts_unsafe_friend_capability() {
        let capabilities = "unsafe";
        let input = format!(
            "
                konst = none,
                friend = [Foo(Source) -> {capabilities}]
            "
        );
        let this = syn::parse_str::<Cfg>(&input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_bracketed_unsafe_friend_capability() {
        let capabilities = "[unsafe]";
        let input = format!(
            "
                konst = none,
                friend = [Foo(Source) -> {capabilities}]
            "
        );
        let this = syn::parse_str::<Cfg>(&input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_math_and_unsafe_friend_capabilities() {
        let capabilities = "[Math, unsafe]";
        let input = format!(
            "
                konst = none,
                friend = [Foo(Source) -> {capabilities}]
            "
        );
        let this = syn::parse_str::<Cfg>(&input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_bit_rel_and_unsafe_friend_capabilities() {
        let capabilities = "[Bit, Rel, unsafe]";
        let input = format!(
            "
                konst = none,
                friend = [Foo(Source) -> {capabilities}]
            "
        );
        let this = syn::parse_str::<Cfg>(&input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn rejects_duplicate_with_display_flags() {
        let input = "
            konst = none,
            with = [display, display]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn rejects_unsupported_with_display_flag() {
        let input = "
            konst = none,
            with = [display, display_names]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");
    }

    #[test]
    fn accepts_with_from_str_flag() {
        let input = "
            konst = none,
            with = [from_str]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_without_from_str_flag() {
        let input = "
            konst = none,
            without = [from_str]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_one_generation_flag_without_brackets() {
        let input = "
            konst = none,
            with = display
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();
        assert!(config.flags.display);

        let input = "
            konst = none,
            without = display
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let config = this.unwrap();
        assert!(!config.flags.display);
    }

    #[test]
    fn accepts_with_core_int_flag() {
        let input = "
            konst = none,
            with = [core_int]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accepts_without_core_int_flag() {
        let input = "
            konst = none,
            without = [core_int]
        ";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn rejects_with_impl_core_int_attribute() {
        let input = "
            konst = none,
            with_impl_core_int = [impl_core_int]
        ";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("generation options use `with = [...]`");
        assert_eq!(error.to_string(), "unknown attribute");
    }

    #[test]
    fn accepts_qualified_friend_paths() {
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
                .path()
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .vec(),
            [
                "crate", "sources", "Source"
            ]
        );
        assert!(friend.capabilities.contains(&syn::parse_quote!(Make)));
        assert!(friend.capabilities.contains(&syn::parse_quote!(Unsafe)));
    }

    #[test]
    fn unsafe_friend_implies_only_make() {
        let input = "convert(Source) -> unsafe";
        let parsed = syn::parse_str::<crate::friendship::cfg::Friend>(input);
        assert!(parsed.is_ok(), "{input}");
        let friend = parsed.unwrap();

        assert_eq!(
            friend.capabilities.iter().map(ToString::to_string).vec(),
            [
                "Make", "Unsafe"
            ],
            "{input}"
        );
    }

    #[test]
    fn bracketed_unsafe_friend_implies_only_make() {
        let input = "convert(Source) -> [unsafe]";
        let parsed = syn::parse_str::<crate::friendship::cfg::Friend>(input);
        assert!(parsed.is_ok(), "{input}");
        let friend = parsed.unwrap();

        assert_eq!(
            friend.capabilities.iter().map(ToString::to_string).vec(),
            [
                "Make", "Unsafe"
            ],
            "{input}"
        );
    }

    #[test]
    fn accepts_friend_make_and_unsafe_capabilities() {
        let (capabilities, expected) = (
            "[Make, unsafe]",
            vec![
                "Make", "Unsafe",
            ],
        );
        let input = format!("convert(Source) -> {capabilities}");
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(&input)
            .expect(&input);
        assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
        assert!(friend.ty.path().is_ident("Source"));
        assert_eq!(
            friend.capabilities.iter().map(ToString::to_string).vec(),
            expected,
            "{input}"
        );
    }

    #[test]
    fn accepts_friend_capabilities_with_trailing_comma() {
        let (capabilities, expected) = (
            "[Make, unsafe,]",
            vec![
                "Make", "Unsafe",
            ],
        );
        let input = format!("convert(Source) -> {capabilities}");
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(&input)
            .expect(&input);
        assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
        assert!(friend.ty.path().is_ident("Source"));
        assert_eq!(
            friend.capabilities.iter().map(ToString::to_string).vec(),
            expected,
            "{input}"
        );
    }

    #[test]
    fn accepts_friend_empty_capabilities() {
        let (capabilities, expected) = ("[]", Vec::<&str>::new());
        let input = format!("convert(Source) -> {capabilities}");
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(&input)
            .expect(&input);
        assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
        assert!(friend.ty.path().is_ident("Source"));
        assert_eq!(
            friend.capabilities.iter().map(ToString::to_string).vec(),
            expected,
            "{input}"
        );
    }

    #[test]
    fn accepts_typed_implicit_friend_unsafe_capability() {
        let input = "_(Source) -> unsafe";
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(input)
            .expect(input);
        assert!(friend.conv.is_none(), "{input}");
        assert!(friend.ty.path().is_ident("Source"), "{input}");
        assert!(friend.capabilities.contains(&syn::parse_quote!(Unsafe)));
    }

    #[test]
    fn accepts_typed_implicit_friend_with_multiple_capabilities() {
        let input = "_(Source,) -> [Make, unsafe]";
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(input)
            .expect(input);
        assert!(friend.conv.is_none(), "{input}");
        assert!(friend.ty.path().is_ident("Source"), "{input}");
        assert!(friend.capabilities.contains(&syn::parse_quote!(Unsafe)));
    }

    #[test]
    fn accepts_absolute_friend_paths_with_trailing_source_comma() {
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(
            "::conversions::own(::sources::Source,) -> Make",
        )
        .expect("one source argument may have a trailing comma");

        let conversion = friend.conv.as_ref().unwrap();
        assert!(conversion.leading_colon.is_some());
        assert_eq!(conversion.segments.first().unwrap().ident, "conversions");
        assert_eq!(conversion.segments.last().unwrap().ident, "own");
        assert!(friend.ty.path().leading_colon.is_some());
        assert_eq!(friend.ty.path().segments.first().unwrap().ident, "sources");
        assert_eq!(friend.ty.path().segments.last().unwrap().ident, "Source");
    }

    #[test]
    fn rejects_obsolete_bare_source_syntax() {
        let input = "Source";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_obsolete_bare_source_with_capability() {
        let input = "Source -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_obsolete_level_argument() {
        let input = "Source(level = [Full])";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn does_not_suggest_obsolete_conversion_and_cap_arguments() {
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
    fn does_not_suggest_obsolete_implicit_cap_arguments() {
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
    fn rejects_friend_without_arrow() {
        let input = "convert(Source) Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_friend_without_capabilities() {
        let input = "convert(Source) ->";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn accepts_implicit_friend_with_default_capabilities() {
        let input = "_(Source)";
        syn::parse_str::<crate::friendship::cfg::Friend>(input).expect(input);
    }

    #[test]
    fn rejects_implicit_friend_without_capabilities() {
        let input = "_(Source) ->";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_bare_implicit_friend() {
        let input = "_";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_conversion_without_source_argument() {
        let input = "convert() -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_conversion_with_multiple_source_arguments() {
        let input = "convert(Source, Other) -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_conversion_with_underscore_source() {
        let input = "convert(_) -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_conversion_with_reference_source() {
        let input = "convert(&Source) -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_conversion_with_parenthesized_source() {
        let input = "convert((Source)) -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_implicit_conversion_without_source() {
        let input = "_() -> Make";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_string_capability() {
        let input = "convert(Source) -> \"Make\"";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_non_identifier_capability() {
        let input = "convert(Source) -> [Make, 1]";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_capability_list_without_comma() {
        let input = "convert(Source) -> [Make unsafe]";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_capability_list_with_extra_comma() {
        let input = "convert(Source) -> [Make,, unsafe]";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_nested_capability() {
        let input = "convert(Source) -> Make::Nested";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_capability_with_extra_tokens() {
        let input = "convert(Source) -> Make extra";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_comma_separated_capabilities() {
        let input = "convert(Source) -> Make, unsafe";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_capability_as_conversion_target() {
        let input = "_ -> Foo(Source)";
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }
}
