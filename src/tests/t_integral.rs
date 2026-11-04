#[cfg(test)]
mod tests {
    use quote::format_ident;
    use syn::parse::Parser as _;

    use crate::integral::{
        StoragePlan,
        ekran,
        ekran_with,
    };
    use crate::integral_cfg::Cfg;
    use crate::zz_lib::{
        Konst,
        Konsts,
    };
    use crate::zz_profile_integral::{
        Arithmetic,
        ProfileKind,
    };
    use crate::zz_values::{
        Int,
        N,
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
        profile: ProfileKind,
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

    fn item_error(
        input: &str,
        source: &str,
    ) -> String {
        let config = parse_test_config(input);
        let item = syn::parse_str::<syn::ItemStruct>(source);
        assert!(item.is_ok(), "{source}");
        let expansion = ekran(Box::new(config), item.unwrap());
        assert!(expansion.is_err(), "{input}: {source}");
        return expansion.unwrap_err().to_string();
    }

    #[test]
    fn fixed_profile_parser_rejects_profile_override() {
        let input = "profile = wrapping";
        let this = profile_config(ProfileKind::Delta, input);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "profile is selected by this macro");
    }

    #[test]
    fn delta_profile_parser_rejects_direct_precedes() {
        let input = "precedes = true";
        let this = profile_config(ProfileKind::Delta, input);
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "unknown attribute");
    }

    #[test]
    fn non_zero_storage_rejects_a_custom_or_panic_constructor() {
        let input = "
            konst = none,
            domain = 1..,
            without = fn_or_panic,
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
            "nonzero storage does not support a custom or_panic constructor",
        );
    }

    #[test]
    fn rewritten_storage_requires_a_private_field() {
        let input = "domain = 1..";
        let source = "struct Code(pub u8);";
        let error = item_error(input, source);

        assert_eq!(
            error,
            "integral storage can only rewrite a private tuple field",
        );
    }

    #[test]
    fn wrapping_storage_requires_a_private_field() {
        let input = "profile = wrapping {}";
        let source = "struct Code(pub u8);";
        let error = item_error(input, source);

        assert_eq!(
            error,
            "integral storage can only rewrite a private tuple field",
        );
    }

    #[test]
    fn float_field_points_to_the_float_macro() {
        let input = "";
        let source = "struct Meters(f64);";
        let error = item_error(input, source);

        assert_eq!(
            error,
            "integral value field must be a concrete primitive integer; \
            use #[typekin::float] for f32/f64",
        );
    }

    #[test]
    fn validating_helpers_are_not_const_without_konst() {
        let input = "konst = none, domain = 2..";
        let expansion = expansion_of(input);

        assert!(expansion.contains("fn _or_panic"), "{input}: {expansion}");
        assert!(
            !expansion.contains("const fn _or_panic"),
            "{input}: {expansion}"
        );
        assert!(!expansion.contains("const fn _add"), "{input}: {expansion}");
    }

    #[test]
    fn validating_helpers_are_const_under_nightly_konst() {
        let input = "konst = nightly, domain = 2..";
        let expansion = expansion_of(input);

        assert!(
            expansion.contains("const fn _or_panic"),
            "{input}: {expansion}"
        );
        assert!(expansion.contains("const fn _add"), "{input}: {expansion}");
    }

    #[test]
    fn unvalidated_helpers_stay_const_without_konst() {
        let input = "konst = none";
        let expansion = expansion_of(input);

        assert!(
            expansion.contains("const fn _or_panic"),
            "{input}: {expansion}"
        );
        assert!(expansion.contains("const fn _add"), "{input}: {expansion}");
    }

    #[test]
    fn unsafe_friend_stays_unsafe_beside_a_generic_named_unsafe() {
        let input = "friend = source_bits(Source) -> unsafe";
        let source = "struct Point<Unsafe>(u32);";
        let config = parse_test_config(input);
        let item = syn::parse_str::<syn::ItemStruct>(source);
        assert!(item.is_ok(), "{source}");
        let expansion = ekran(Box::new(config), item.unwrap());
        assert!(expansion.is_ok(), "{input}: {source}");
        let expansion = expansion.unwrap().to_string();

        assert!(
            expansion.contains("unsafe trait __TypekinUnsafe"),
            "{input}: {source}: {expansion}"
        );
        assert!(
            expansion.contains("unsafe impl < Unsafe > __TypekinUnsafe"),
            "{input}: {source}: {expansion}"
        );
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
    fn function_removal_rejects_with() {
        let input = "with = fn_count_ones";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "`fn_count_ones` only removes the generated `count_ones`; write \
            `without = fn_count_ones`",
            "{input}",
        );
        assert_eq!(
            error.span().start().column,
            input.find("fn_count_ones").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn removing_an_ungenerated_function_reports_its_flag() {
        let input = "konst = none, without = [fn_count_ones, fn_floor]";
        let source = "struct Count(u32);";
        let config = parse_test_config(input);
        let item = syn::parse_str::<syn::ItemStruct>(source);
        assert!(item.is_ok(), "{source}");
        let expansion = ekran(Box::new(config), item.unwrap());
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
    fn function_removal_rejects_an_unknown_function() {
        let input = "without = fn_not_a_function";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(error.to_string(), "unknown flag", "{input}");
        assert_eq!(
            error.span().start().column,
            input.find("fn_not_a_function").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn facet_rejects_function_removals() {
        let input = "
            konst = none,
            facet = [
                int {
                    friend = convert(Source) -> Math,
                    without = fn_abs,
                },
            ]
        ";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "function removals are wrapper-wide and cannot be configured \
            inside a facet",
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
    fn later_profile_const_flags_disable_earlier_ones() {
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
    fn konst_flags_leave_unselected_operations_non_const() {
        let input = "konst = none";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let disabled = this.unwrap();
        assert!(
            !disabled.konst_for(&[
                Konsts::Add,
                Konsts::Math
            ]),
            "{input}"
        );

        let input = "konst = none, with = [konst_add]";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let enabled = this.unwrap();
        assert!(
            !enabled.konst_for(&[
                Konsts::Sub,
                Konsts::Math
            ]),
            "{input}"
        );

        let input = "konst = none, with = [konst_math], without = [konst_add]";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let operation_override = this.unwrap();
        assert!(
            !operation_override.konst_for(&[
                Konsts::Add,
                Konsts::Math
            ]),
            "{input}"
        );

        let input = "konst = nightly, without = [konst_add]";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let disabled = this.unwrap();
        assert!(
            !disabled.konst_for(&[
                Konsts::Add,
                Konsts::Math
            ]),
            "{input}"
        );

        let input = "konst = nightly, without = [konst_map]";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let map = this.unwrap();
        assert!(!map.konst_for(&[Konsts::Map]), "{input}");
    }

    #[test]
    fn default_konst_follows_the_crate_feature() {
        let input = "";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let cfg = this.unwrap();
        assert_eq!(cfg.konst, Konst::default());
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

        let input = "without = [konst_try_make]";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let cfg = this.unwrap();
        assert!(!cfg.konst_for(&[Konsts::TryMake]), "{input}");

        let input = "with = [konst_add]";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_ok(), "{input}");
        let cfg = this.unwrap();
        assert!(cfg.konst_friendship(), "{input}");
    }

    #[test]
    fn bare_profiles_require_separators() {
        let input = "profile = [circular point]";
        let this = syn::parse_str::<Cfg>(input);
        let error = this.unwrap_err();

        assert_eq!(error.span().start().column, input.find("point").unwrap(),);
    }

    #[test]
    fn bare_facet_resolves_its_profile() {
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
    }

    #[test]
    fn bare_facets_require_separators() {
        let input = "facet = [circular point]";
        let this = syn::parse_str::<Cfg>(input);
        let error = this.unwrap_err();

        assert_eq!(error.span().start().column, input.find("point").unwrap(),);
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
    fn wrapping_arithmetic_uses_wrapper_ops_with_a_const_fallback() {
        let runtime = Arithmetic::WrapperWrapping
            .make_expr(crate::zz_values::ArithmeticOp::Add)
            .to_string();
        let konst = Arithmetic::WrapperWrapping
            .make_const_expr(crate::zz_values::ArithmeticOp::Add)
            .to_string();

        assert!(
            runtime.contains("Wrapping (lhs) + :: core :: num :: Wrapping")
        );
        assert!(konst.contains("lhs . wrapping_add (rhs)"));
    }

    #[test]
    fn facet_friend_capabilities_do_not_include_make() {
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
        let make = format_ident!("Make");

        assert!(!friend.capabilities.contains(&make), "{input}");
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
    fn single_facet_resolves_its_profile() {
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
    }

    #[test]
    fn rejects_friend_flag() {
        let input = "konst = none, without = friend";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");

        assert_eq!(this.unwrap_err().to_string(), "unknown flag", "{input}");
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

    fn resolved_config(
        input: &str,
        subject: N,
    ) -> Cfg {
        let mut config = parse_test_config(input);
        let resolved = config.validation.resolve(
            subject.into(),
            crate::zz_domain_float::MathProvider::Core,
        );
        assert!(resolved.is_ok(), "{input}: {:?}", resolved.err());
        return config;
    }

    fn contains_at(
        config: &Cfg,
        value: Int,
        width: u32,
    ) -> bool {
        let resolved = config.validation.resolved();
        assert!(resolved.is_some(), "domain was resolved");
        return resolved.unwrap().contains(value, width);
    }

    fn resolve_error(
        input: &str,
        subject: N,
    ) -> String {
        let mut config = parse_test_config(input);
        let resolved = config.validation.resolve(
            subject.into(),
            crate::zz_domain_float::MathProvider::Core,
        );
        assert!(resolved.is_err(), "{input}");
        return resolved.unwrap_err().to_string();
    }

    #[test]
    fn pointer_domain_excludes_16_bit_minimum() {
        let input =
            "konst = none, domain = isize::MIN..-32_768 | -isize::MAX..";
        let config = resolved_config(input, N::ISIZ);
        let minimum = Int::Negative(32_768);

        assert!(!contains_at(&config, minimum, 16), "{input}");
    }

    #[test]
    fn pointer_domain_excludes_32_bit_minimum() {
        let input = "konst = none, \
            domain = isize::MIN..-2_147_483_648 | -isize::MAX..";
        let config = resolved_config(input, N::ISIZ);
        let minimum = Int::Negative(2_147_483_648);

        assert!(!contains_at(&config, minimum, 32), "{input}");
    }

    #[test]
    fn pointer_domain_includes_32_bit_minimum() {
        let input = "konst = none, \
            domain = -isize::MAX.. | -2_147_483_648..=-2_147_483_648";
        let config = resolved_config(input, N::ISIZ);
        let minimum = Int::Negative(2_147_483_648);

        assert!(contains_at(&config, minimum, 32), "{input}");
    }

    #[test]
    fn rejects_bare_bound_constant() {
        let input = "konst = none, domain = MAX..4";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");

        assert_eq!(
            this.unwrap_err().to_string(),
            "need a constant; write u32::MAX with the operand's type"
        );
    }

    #[test]
    fn rejects_arithmetic_range_bound() {
        let input = "konst = none, domain = 1 + 1..4";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");

        assert_eq!(
            this.unwrap_err().to_string(),
            "need a constant; arithmetic is not supported"
        );
    }

    #[test]
    fn rejects_bare_literal_domain() {
        let input = "konst = none, domain = 1";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");

        assert_eq!(
            this.unwrap_err().to_string(),
            "a value alone is not a set; write it as {…}"
        );
    }

    #[test]
    fn rejects_plus_between_ranges() {
        let input = "konst = none, domain = 1..2 + 3..4";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");

        assert_eq!(
            this.unwrap_err().to_string(),
            "need a constant; arithmetic is not supported"
        );
    }

    #[test]
    fn rejects_range_list_in_roster() {
        let input = "konst = none, domain = {1..2, 3..4}";
        let this = syn::parse_str::<Cfg>(input);
        assert!(this.is_err(), "{input}");

        assert_eq!(
            this.unwrap_err().to_string(),
            "{…} lists single values; write ranges outside braces: {1} | 3..5"
        );
    }

    #[test]
    fn contradictory_domain_is_rejected() {
        let input = "konst = none, domain = ..0";
        let error = resolve_error(input, N::U032);

        assert_eq!(error, "domain admits no u32 value");
    }

    #[test]
    fn tautological_domain_is_rejected() {
        let input = "konst = none, domain = ..";
        let error = resolve_error(input, N::I032);

        assert_eq!(error, "domain admits every i32 value; remove the domain");
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
        assert_eq!(
            config.konst_for(&[Konsts::Math]),
            cfg!(feature = "nightly")
        );
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
    fn rejects_obsolete_bare_source_syntax() {
        let input = "Source";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_obsolete_bare_source_with_capability() {
        let input = "Source -> Make";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_obsolete_level_argument() {
        let input = "Source(level = [Full])";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn does_not_suggest_obsolete_conversion_and_cap_arguments() {
        let input = "Source(conv = convert, cap = Make)";
        let error = syn::parse_str::<crate::zz_friend::Friend>(input)
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
        let error = syn::parse_str::<crate::zz_friend::Friend>(input)
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
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_friend_without_capabilities() {
        let input = "convert(Source) ->";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_implicit_friend_without_capabilities() {
        let input = "_(Source) ->";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_bare_implicit_friend() {
        let input = "_";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_conversion_without_source_argument() {
        let input = "convert() -> Make";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_conversion_with_multiple_source_arguments() {
        let input = "convert(Source, Other) -> Make";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_conversion_with_underscore_source() {
        let input = "convert(_) -> Make";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_conversion_with_reference_source() {
        let input = "convert(&Source) -> Make";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_conversion_with_parenthesized_source() {
        let input = "convert((Source)) -> Make";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_implicit_conversion_without_source() {
        let input = "_() -> Make";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_string_capability() {
        let input = "convert(Source) -> \"Make\"";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_non_identifier_capability() {
        let input = "convert(Source) -> [Make, 1]";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_capability_list_without_comma() {
        let input = "convert(Source) -> [Make unsafe]";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_capability_list_with_extra_comma() {
        let input = "convert(Source) -> [Make,, unsafe]";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_nested_capability() {
        let input = "convert(Source) -> Make::Nested";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_capability_with_extra_tokens() {
        let input = "convert(Source) -> Make extra";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_comma_separated_capabilities() {
        let input = "convert(Source) -> Make, unsafe";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }

    #[test]
    fn rejects_capability_as_conversion_target() {
        let input = "_ -> Foo(Source)";
        assert!(
            syn::parse_str::<crate::zz_friend::Friend>(input).is_err(),
            "{input}"
        );
    }
}
