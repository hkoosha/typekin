#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use syn::ItemEnum;

    use crate::bitflag::{
        BitFlag,
        Cfg,
        ekran,
        resolve_repr,
    };
    use crate::zz_lib::Konsts;
    use crate::zz_values::N;

    fn expand(source: &str) -> syn::Result<TokenStream> {
        let configuration = syn::parse_str::<Cfg>("konst = none")?;
        let item = syn::parse_str::<ItemEnum>(source)?;
        return ekran(Box::new(configuration), item);
    }

    #[test]
    fn attrs_friends_and_konst() {
        let it = "
            friend = crate::convert(crate::source::Source) -> Bit,
            konst = stable
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn rejects_dangling_friend() {
        let it = "
            friend = convert(Source) -> Bit,
            convert(Other) -> Bit
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_dangling_friend_in_integral() {
        let it = "
            integral = [
                friend = _(Source) -> Math,
                _(Other) -> Math
            ]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_incomplete_friend() {
        let it = "
            friend =
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_incomplete_friend_in_integral() {
        let it = "
            integral = [friend =]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_incomplete_friend_combined() {
        let it = "
            friend =,
            integral = []
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_incomplete_friend_in_integral_combined() {
        let it = "
            integral = [
                friend =,
                valid = is_valid
            ]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_invalid_cap() {
        let it = "
            friend = convert(Source) -> [Bit, 1]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_unknown_root_capability_at_its_span() {
        let it = "friend = convert(Source) -> Inspector";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), "unknown capability", "{it}");
        assert_eq!(
            error.span().start().column,
            it.find("Inspector").unwrap(),
            "{it}",
        );
    }

    const ROOT_VALUE_CAPABILITY: &str = "bitflag root friends only support `Bit`; declare value capabilities in `integral = [friend = ...]`";

    fn assert_root_capability_rejected(
        it: &str,
        capability: &str,
    ) {
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
        let error = this.unwrap_err();

        assert_eq!(error.to_string(), ROOT_VALUE_CAPABILITY, "{it}");
        assert_eq!(
            error.span().start().column,
            it.find(capability).unwrap(),
            "{it}",
        );
    }

    #[test]
    fn rejects_root_make_capability_at_its_span() {
        let it = "friend = convert(Source) -> [Bit, Make]";
        assert_root_capability_rejected(it, "Make");
    }

    #[test]
    fn rejects_root_math_capability_at_its_span() {
        let it = "friend = convert(Source) -> Math";
        assert_root_capability_rejected(it, "Math");
    }

    #[test]
    fn rejects_root_cmp_capability_at_its_span() {
        let it = "friend = convert(Source) -> Cmp";
        assert_root_capability_rejected(it, "Cmp");
    }

    #[test]
    fn rejects_root_unsafe_capability_at_its_span() {
        let it = "friend = convert(Source) -> unsafe";
        assert_root_capability_rejected(it, "unsafe");
    }

    #[test]
    fn rejects_root_unsafe_capability_beside_bit_at_its_span() {
        let it = "friend = convert(Source) -> [Bit, unsafe]";
        assert_root_capability_rejected(it, "unsafe");
    }

    #[test]
    fn rejects_invalid_cap_in_integral() {
        let it = "
            integral = [friend = _(Source) -> Inspector]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_unknown_attr() {
        let it = "
            friend = _(Source) -> Bit,
            unknown = true
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_unknown_attr_in_integral_combined() {
        let it = "
            integral = [
                friend = _(Source) -> [Math, Bit],
                unknown = true
            ]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_duplicated_friends() {
        let it = "
            friend = _(Source) -> Bit,
            friend = []
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_unknown_attr_in_integral() {
        let it = "
            integral = [unknown = true]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_tree_in_nested_integral_configuration() {
        let it = "integral = [tree = Direction { In }]";
        let this = syn::parse_str::<Cfg>(it);
        let error = this.unwrap_err();

        assert_eq!(
            error.to_string(),
            "bitflag integral configuration does not support tree",
        );
    }

    #[test]
    fn rejects_produce_in_integral() {
        let it = "
            integral = [
                produce = FlagValue::into_output(_) -> Output
            ]
        ";
        let error = syn::parse_str::<Cfg>(it)
            .expect_err("bitflag values cannot have a custom produced type");

        assert_eq!(
            error.to_string(),
            "bitflag integral configuration does not support produce",
        );
    }

    #[test]
    fn rejects_non_default_profile_in_integral() {
        let it = "
            integral = [
                profile = [
                    int {},
                    circular {},
                ]
            ]
        ";
        let error = syn::parse_str::<Cfg>(it)
            .expect_err("bitflag values cannot use restricted profiles");

        assert_eq!(
            error.to_string(),
            "bitflag integral configuration only supports the default profile",
        );
    }

    #[test]
    fn rejects_facets_in_integral() {
        let input = "
            integral = [
                facet = []
            ]
        ";
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("bitflag values cannot use integral facets");

        assert_eq!(
            error.to_string(),
            "bitflag integral configuration does not support facets",
        );
    }

    #[test]
    fn rejects_non_zero_in_integral() {
        let error = syn::parse_str::<Cfg>("integral = [with = non_zero]")
            .expect_err("bitflag values require a zero identity");

        assert_eq!(
            error.to_string(),
            "bitflag integral configuration does not support `non_zero`",
        );
    }

    #[test]
    fn rejects_integral_domain_that_excludes_zero() {
        let error = syn::parse_str::<Cfg>("integral = [domain = 1..]")
            .expect_err("bitflag domains must contain zero");

        assert_eq!(
            error.to_string(),
            "bitflag integral domain must include zero",
        );
    }

    #[test]
    fn rejects_negative_pointer_domain_in_integral() {
        let input = "
            konst = none,
            integral = [
                domain = isize::MIN..-2_147_483_648 | -isize::MAX..0
            ]
        ";
        let this = syn::parse_str::<Cfg>(input);
        let error = this.unwrap_err();

        assert_eq!(
            error.to_string(),
            "bitflag integral domain must include zero",
        );
    }

    #[test]
    fn stable_konst_leaves_math_non_const() {
        let it = "konst = stable";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
        let config = this.unwrap();

        assert!(!config.int.konst_for(&[Konsts::Math]), "{it}");
    }

    #[test]
    fn accepts_konst_none() {
        let it = "
            konst = none
        ";
        let config = syn::parse_str::<Cfg>(it)
            .expect("top-level none konst should parse");
        assert_eq!(config.int.konst, crate::zz_lib::Konst::None);
    }

    #[test]
    fn repr_defaults_to_transparent_value() {
        let it = "";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
        let config = this.unwrap();

        assert!(config.bit.has(BitFlag::Repr), "{it}");
    }

    #[test]
    fn with_repr_keeps_transparent_value() {
        let it = "with = repr";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
        let config = this.unwrap();

        assert!(config.bit.has(BitFlag::Repr), "{it}");
    }

    #[test]
    fn without_repr_drops_transparent_value() {
        let it = "without = repr";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
        let config = this.unwrap();

        assert!(!config.bit.has(BitFlag::Repr), "{it}");
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
        let it = "
            with = [serde_with_name, serde_with_value]
        ";

        let error = syn::parse_str::<Cfg>(it)
            .expect_err("serde representations are mutually exclusive");

        assert_eq!(
            error.to_string(),
            "bitflag serde representation flags are mutually exclusive",
        );
    }

    #[test]
    fn rejects_unknown_serde_representation_flags() {
        let it = "
            with = [serde_with_compact]
        ";

        let this = syn::parse_str::<Cfg>(it)
            .expect_err("unknown serde representation flags must fail");
        assert_eq!(this.to_string(), "unknown flag");
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
    fn conflicting_const_flags_report_the_second_setting() {
        let input = "with = konst_value, without = konst_value";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "flag specified in both `with` and `without`",
            "{input}",
        );
        assert_eq!(
            error.span().start().column,
            input.rfind("konst_value").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn rejects_custom_error_type() {
        let it = "
            integral = [error = FlagError]
        ";

        let error = syn::parse_str::<Cfg>(it)
            .expect_err("other error types remain unsupported");

        assert_eq!(
            error.to_string(),
            "only `()` is currently supported for `error`",
        );
    }

    #[test]
    fn rejects_incompatible_konst() {
        let input = "konst = none, with = konst_value, integral = [without = konst_value]";
        let parsed = syn::parse_str::<Cfg>(input);
        let error = parsed.unwrap_err();

        assert_eq!(
            error.to_string(),
            "this value is not compatible with own konst config of bitflag",
            "{input}",
        );
        assert_eq!(
            error.span().start().column,
            input.rfind("konst_value").unwrap(),
            "{input}",
        );
    }

    #[test]
    fn rejects_text_only_konst_flag() {
        let result = syn::parse_str::<Cfg>("with = [konst_text_access]");
        let error = result.unwrap_err();

        assert_eq!(error.to_string(), "unknown flag");
    }

    #[test]
    fn default_konst_follows_crate_feature() {
        let it = "friend = [Foo(u8) -> Bit]";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
        let config = this.unwrap();

        assert_eq!(config.int.konst, crate::zz_lib::Konst::default(), "{it}");
        assert_eq!(
            config.int.konst_for(&[Konsts::Math]),
            cfg!(feature = "nightly"),
            "{it}",
        );
    }

    #[test]
    fn rejects_friendship_trait_configuration() {
        let it = "
            with = [trait_seal],
            integral = [konst = none]
        ";

        let error = syn::parse_str::<Cfg>(it).unwrap_err();

        assert_eq!(error.to_string(), "unknown flag");
    }

    #[test]
    fn accept_flag_friend_and_integral_friend() {
        let it = "
            friend = [
                convert(Source) -> Bit
            ],
            integral = [
                friend = [Foo(Other) -> Math]
            ]
        ";

        let config = syn::parse_str::<Cfg>(it);
        assert!(config.is_ok(), "{it}");
        let config = config.unwrap();

        let friend = config.friends.iter().next().unwrap();

        assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));
        assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
        assert!(friend.ty.path().is_ident("Source"));
    }

    #[test]
    fn rejects_duplicated_with_attr() {
        let it = "
            with = [display, display]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_with_unsupported_attr() {
        let it = "
            with = [display, display_names]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_without_unsupported_attr() {
        let it = "
            konst = none,
            without = [display_names]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_conflicting_with_and_without_attributes() {
        let it = "
            with = [display],
            without = [display]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_generic_enum_at_its_parameters() {
        let source = "#[repr(u8)] pub enum Flag<T> { Read = 1 }";
        let this = expand(source);
        let error = this.unwrap_err();

        assert_eq!(
            error.to_string(),
            "bitflag does not support generic enums",
            "{source}",
        );
        assert_eq!(
            error.span().start().column,
            source.find("T>").unwrap(),
            "{source}",
        );
    }

    #[test]
    fn rejects_generic_enum_at_its_where_clause() {
        let source = "#[repr(u8)] pub enum Flag where u8: Copy { Read = 1 }";
        let this = expand(source);
        let error = this.unwrap_err();

        assert_eq!(
            error.to_string(),
            "bitflag does not support generic enums",
            "{source}",
        );
        assert_eq!(
            error.span().start().column,
            source.find("where").unwrap(),
            "{source}",
        );
    }
}
