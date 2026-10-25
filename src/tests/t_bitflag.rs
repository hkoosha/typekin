#[cfg(test)]
mod tests {
    use crate::bitflag::{
        Cfg,
        resolve_repr,
    };
    use crate::zz::Konsts;
    use crate::zz_value_type::N;

    #[test]
    fn attrs_friends_and_konst() {
        let it = "
            friends = crate::convert(crate::source::Source) -> Inspector,
            konst = stable
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn attrs_konst_and_friends_and_integral() {
        let it = "
            konst = stable,
            friends = crate::convert(crate::source::Source) -> [Bit, unsafe],
            integral = []
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn attrs_konst_and_integral_and_friends() {
        let it = "
            konst = stable,
            integral = [],
            friends = Foo(crate::source::Source) -> Bit
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn attrs_konst_and_integral_and_friends_and_valid() {
        let it = "
            konst = stable,
            integral = [
                friends = crate::convert(crate::source::Source) -> Make,
                valid = is_valid,
            ]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn attrs_integral_and_suffix() {
        let it = "
            integral = [
                valid = is_valid,
                friends = Foo(crate::source::Source) -> [Math, Bit]
            ],
            suffix = \"Value\"
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn attrs_friend_and_integral() {
        let it = "
            friends = [],
            integral = [
                friends = [],
                valid = is_valid
            ]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn rejects_dangling_friend() {
        let it = "
            friends = convert(Source) -> Bit,
            convert(Other) -> Bit
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_dangling_friend_in_integral() {
        let it = "
            integral = [
                friends = _(Source) -> Math,
                _(Other) -> Math
            ]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_incomplete_friend() {
        let it = "
            friends =
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_incomplete_friend_in_integral() {
        let it = "
            integral = [friends =]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_incomplete_friend_combined() {
        let it = "
            friends =,
            integral = []
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_incomplete_friend_in_integral_combined() {
        let it = "
            integral = [
                friends =,
                valid = is_valid
            ]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_invalid_cap() {
        let it = "
            friends = convert(Source) -> [Bit, 1]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_invalid_cap_in_integral() {
        let it = "
            integral = [friends = _(Source) -> Inspector]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_unknown_attr() {
        let it = "
            friends = _(Source) -> Bit,
            unknown = true
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_unknown_attr_in_integral_combined() {
        let it = "
            integral = [
                friends = _(Source) -> [Math, Bit],
                unknown = true
            ]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn rejects_duplicated_friends() {
        let it = "
            friends = _(Source) -> Bit,
            friends = []
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
    fn rejects_output_type_in_integral() {
        let it = "
            integral = [
                out_type = FlagValue::into_output(_) -> Output
            ]
        ";
        let error = syn::parse_str::<Cfg>(it)
            .expect_err("bitflag values cannot have a custom output type");

        assert_eq!(
            error.to_string(),
            "bitflag integral configuration does not support out_type",
        );
    }

    #[test]
    fn rejects_non_default_profile_in_integral() {
        let it = "
            integral = [
                profile = [
                    default {},
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
    fn accepts_konst_nightly() {
        let it = "
            konst = nightly
        ";
        let config = syn::parse_str::<Cfg>(it)
            .expect("top-level nightly konst should parse");
        assert_eq!(config.int.konst, crate::zz::Konst::Nightly);
    }

    #[test]
    fn accepts_konst_stable() {
        let it = "
            konst = stable
        ";
        let config = syn::parse_str::<Cfg>(it)
            .expect("top-level stable konst should parse");
        assert_eq!(config.int.konst, crate::zz::Konst::Stable);
        assert!(config.int.konst_for(&[Konsts::Value]));
        assert!(!config.int.konst_for(&[Konsts::Math]));
    }

    #[test]
    fn accepts_konst_none() {
        let it = "
            konst = none
        ";
        let config = syn::parse_str::<Cfg>(it)
            .expect("top-level none konst should parse");
        assert_eq!(config.int.konst, crate::zz::Konst::None);
    }

    #[test]
    fn accepts_serde_auto() {
        let it = "
            with = serde_with_auto
        ";
        let config = syn::parse_str::<Cfg>(it).unwrap();
        assert!(config.bit.serde);
    }

    #[test]
    fn accepts_serde_name() {
        let it = "
            with = serde_with_name
        ";
        let config = syn::parse_str::<Cfg>(it).unwrap();
        assert!(config.bit.serde);
    }

    #[test]
    fn accepts_serde_value() {
        let it = "
            with = serde_with_value
        ";
        let config = syn::parse_str::<Cfg>(it).unwrap();
        assert!(config.bit.serde);
    }

    #[test]
    fn serde_defaults_to_auto() {
        let it = "
            with = serde
        ";
        let config = syn::parse_str::<Cfg>(it).unwrap();
        assert!(config.bit.serde);
        assert!(!config.bit.serde_with_name);
        assert!(!config.bit.serde_with_value);
    }

    #[test]
    fn bitflag_derives_default() {
        let it = "
        ";
        let this = syn::parse_str::<Cfg>(it).unwrap();
        assert!(this.bit.clone);
        assert!(this.bit.copy);
        assert!(this.bit.hash);
        assert!(this.bit.debug);
        assert!(this.bit.repr);
    }

    #[test]
    fn bitflag_derives_enabled() {
        let it = "
            with = [clone, copy, hash, debug, repr]
        ";
        let this = syn::parse_str::<Cfg>(it).unwrap();
        assert!(this.bit.clone);
        assert!(this.bit.copy);
        assert!(this.bit.hash);
        assert!(this.bit.debug);
        assert!(this.bit.repr);
    }

    #[test]
    fn bitflag_derives_disabled() {
        let it = "
            without = [clone, copy, hash, debug, repr]
        ";
        let this = syn::parse_str::<Cfg>(it).unwrap();
        assert!(!this.bit.clone);
        assert!(!this.bit.copy);
        assert!(!this.bit.hash);
        assert!(!this.bit.debug);
        assert!(!this.bit.repr);
    }

    #[test]
    fn with_display() {
        let it = "
            with = display
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
        let config = this.unwrap();
        assert!(config.bit.display);
    }

    #[test]
    fn without_display() {
        let it = "
            without = display
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
        let config = this.unwrap();
        assert!(!config.bit.display);
    }

    #[test]
    fn integral_with_display() {
        let input = "integral = [with = display]";
        let config = syn::parse_str::<Cfg>(input).expect(input);
        assert!(config.int.flags.display);
    }

    #[test]
    fn integral_without_display() {
        let it = "
            integral = [without = display]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
        let config = this.unwrap();
        assert!(!config.int.flags.display);
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
    fn accepts_nested_unit_error() {
        let it = "
            integral = [error = ()]
        ";
        syn::parse_str::<Cfg>(it).unwrap();
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
    fn accepts_forwarding_konst_to_integral() {
        let it = "
            integral = [with = [konst_value]]
        ";

        let config = syn::parse_str::<Cfg>(it)
            .expect("matching forwarded konst flag should parse");

        assert!(config.int.konst_for(&[Konsts::Value]));
    }

    #[test]
    fn rejects_incompatible_konst() {
        let it = "
            integral = [without = [konst_value]]
        ";

        let error = syn::parse_str::<Cfg>(it)
            .expect_err("incompatible forwarded konst flag must be rejected");

        assert_eq!(
            error.to_string(),
            "this value is not compatible with bitflags' own konst config",
        );
    }

    #[test]
    fn rejects_text_only_konst_flag() {
        let result = syn::parse_str::<Cfg>("with = [konst_text_access]");
        let error = result.unwrap_err();

        assert_eq!(error.to_string(), "unknown flag");
    }

    #[test]
    fn defaults_missing_konst_to_crate_default() {
        let it = "
            friends = [Foo(u8) -> Bit]
        ";

        let config = syn::parse_str::<Cfg>(it)
            .expect("konst should default to the crate feature");

        assert_eq!(config.int.konst, crate::zz::Konst::default());
        assert!(config.int.konst_for(&[Konsts::Value]));
        assert_eq!(
            config.int.konst_for(&[Konsts::Math]),
            cfg!(feature = "nightly"),
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
    fn accepts_friend_capabilities() {
        let it = "
            konst = none,
            friends = [Foo(u8) -> [Bit]],
            integral = []
        ";

        let config = syn::parse_str::<Cfg>(it)
            .expect("bitflag capabilities should parse");

        let friend = config.friends.iter().next().unwrap();

        assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));
    }

    #[test]
    fn rejects_legacy_friend_levels() {
        let it = "
            friends = [u8(conv = self, level = [Bit])],
            integral = []
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn accepts_unsafe_friend() {
        let it = "
            friends = [Foo(Source) -> unsafe]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_unsafe_and_bit_friend() {
        let it = "
            friends = [Foo(Source) -> [Bit, unsafe]]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_unsafe_integral_friends() {
        let it = "
            integral = [friends = [Foo(Source) -> unsafe]]
        ";
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_ok(), "{it}");
    }

    #[test]
    fn accepts_unsafe_integral_friends_combined() {
        let it = "
            integral = [friends = [Foo(Source) -> [Math, Bit, unsafe]]]
        ";
        let input = format!("konst = none, {it}");
        let this = syn::parse_str::<Cfg>(&input);
        assert!(this.is_ok(), "{input}");
    }

    #[test]
    fn accept_flag_friend_and_integral_friend() {
        let it = "
            friends = [
                convert(Source) -> Inspector
            ],
            integral = [
                friends = [Foo(Other) -> Math]
            ]
        ";

        let config = syn::parse_str::<Cfg>(it)
            .expect("bitflag root capabilities may name arbitrary markers");

        let friend = config.friends.iter().next().unwrap();

        assert!(friend.capabilities.contains(&syn::parse_quote!(Inspector)));
        assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
        assert!(friend.ty.is_ident("Source"));
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
}
