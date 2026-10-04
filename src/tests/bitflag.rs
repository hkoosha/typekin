use crate::bitflag::Cfg;

#[test]
fn accepts_single_root_and_nested_friends() {
    for input in [
        "friends = crate::convert(crate::source::Source) -> Inspector, konst = false",
        "konst = false, friends = crate::convert(crate::source::Source) -> [Bit, Trust], integral = []",
        "konst = false, integral = [], friends = _(crate::source::Source) -> Bit",
        "konst = false, friends = _ -> [Bit, Inspector], with = [display]",
        "konst = false, integral = [friends = crate::convert(crate::source::Source) -> Make, valid = is_valid]",
        "konst = false, integral = [valid = is_valid, friends = _(crate::source::Source) -> [Math, Bit]], suffix = \"Value\"",
        "konst = false, integral = [friends = _ -> Math]",
    ] {
        syn::parse_str::<Cfg>(input).expect(input);
    }
}

#[test]
fn accepts_bracketed_multiple_and_empty_friends_at_each_scope() {
    for input in [
        "konst = false, friends = [convert(Source) -> Bit, _(Other) -> [Bit, Trust]], integral = [friends = [convert(Source) -> Make, _(Other) -> Math]]",
        "konst = false, friends = [], integral = [friends = [], valid = is_valid]",
    ] {
        syn::parse_str::<Cfg>(input).expect(input);
    }
}

#[test]
fn rejects_invalid_single_friends_at_root_and_nested_scope() {
    for attributes in [
        "friends = convert(Source) -> Bit, convert(Other) -> Bit",
        "integral = [friends = _(Source) -> Math, _(Other) -> Math]",
        "friends =",
        "integral = [friends =]",
        "friends =, integral = []",
        "integral = [friends =, valid = is_valid]",
        "friends = convert(Source)",
        "integral = [friends = convert(Source)]",
        "friends = convert(Source) -> [Bit, 1]",
        "integral = [friends = _(Source) -> Inspector]",
        "friends = _(Source) -> Bit, unknown = true",
        "integral = [friends = _(Source) -> [Math, Bit], unknown = true]",
        "friends = _(Source) -> Bit, friends = []",
    ] {
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }
}

#[test]
fn accepts_top_level_konst_values() {
    let config = syn::parse_str::<Cfg>("konst = true")
        .expect("top-level true konst should parse");
    assert!(config.int.konst);

    let config = syn::parse_str::<Cfg>("konst = false")
        .expect("top-level false konst should parse");
    assert!(!config.int.konst);
}

#[test]
fn rejects_missing_konst() {
    let error = match syn::parse_str::<Cfg>("friends = [_(u8) -> Bit]") {
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
fn accepts_friend_capabilities_and_rejects_levels() {
    let config = syn::parse_str::<Cfg>(
        "konst = false, friends = [_(u8) -> [Bit]], integral = []",
    )
    .expect("bitflag capabilities should parse");
    let friend = config.friends.iter().next().unwrap();
    assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));

    assert!(syn::parse_str::<Cfg>(
            "konst = false, friends = [u8(conv = self, level = [Bit])], integral = []",
        )
        .is_err());
}

#[test]
fn accepts_trusted_root_and_nested_capability_without_make() {
    for attributes in [
        "friends = [_(Source) -> Trust]",
        "friends = [_(Source) -> [Trust]]",
        "friends = [_(Source) -> [Bit, Trust]]",
        "integral = [friends = [_(Source) -> Trust]]",
        "integral = [friends = [_(Source) -> [Trust]]]",
        "integral = [friends = [_(Source) -> [Math, Bit, Trust]]]",
    ] {
        let input = format!("konst = false, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }
}

#[test]
fn accepts_root_marker_capabilities_without_narrowing_integral_vocabulary() {
    let config = syn::parse_str::<Cfg>(
            "konst = false, friends = [convert(Source) -> Inspector], integral = [friends = [_(Other) -> Math]]",
        )
        .expect("bitflag root capabilities may name arbitrary markers");
    let friend = config.friends.iter().next().unwrap();

    assert!(friend.capabilities.contains(&syn::parse_quote!(Inspector)));
    assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
    assert!(friend.ty.as_ref().unwrap().is_ident("Source"));
}

#[test]
fn rejects_legacy_trusted_argument_at_root_and_nested_scope() {
    for trusted in ["true", "false"] {
        for attributes in [
            format!("friends = [Source(cap = Make, trusted = {trusted})]"),
            format!(
                "integral = [friends = [Source(cap = Make, trusted = {trusted})]]"
            ),
        ] {
            let input = format!("konst = false, {attributes}");
            assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
        }
    }
}

#[test]
fn rejects_duplicate_root_and_nested_display_flags() {
    for input in [
        "konst = false, with = [display, display]",
        "konst = false, without = [display, display]",
        "konst = false, with = [display], with = [display]",
        "konst = false, without = [display], without = [display]",
        "konst = false, integral = [with = [display, display]]",
        "konst = false, integral = [without = [display, display]]",
        "konst = false, integral = [with = [display], with = [display]]",
        "konst = false, integral = [without = [display], without = [display]]",
    ] {
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }
}

#[test]
fn rejects_unsupported_root_and_nested_display_flags() {
    for input in [
        "konst = false, with = [display, display_names]",
        "konst = false, without = [display_names]",
        "konst = false, integral = [with = [display, display_names]]",
        "konst = false, integral = [without = [display_names]]",
    ] {
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }
}

#[test]
fn rejects_conflicting_flags_within_root_or_nested_scope() {
    for attributes in [
        "with = [display], without = [display]",
        "without = [display], with = [display]",
        "with = [display, make_value], without = [make_value, impl_value]",
        "without = [make_value, impl_value], with = [display, make_value]",
        "integral = [with = [display], without = [display]]",
        "integral = [without = [display], with = [display]]",
        "integral = [with = [display, impl_math_add], without = [impl_math_add, fn_conv_raw]]",
        "integral = [without = [impl_math_add, fn_conv_raw], with = [display, impl_math_add]]",
    ] {
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }
}

#[test]
fn opposite_display_settings_in_distinct_scopes_are_legal() {
    for attributes in [
        "with = [display], integral = [without = [display]]",
        "integral = [without = [display]], with = [display]",
        "without = [display], integral = [with = [display]]",
        "integral = [with = [display]], without = [display]",
    ] {
        let input = format!("konst = false, {attributes}");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }
}

#[test]
fn rejects_unsupported_nested_single_capabilities() {
    for attributes in [
        "integral = [friends = [_(Source) -> Rel]]",
        "integral = [friends = [_(Source) -> Inspector]]",
    ] {
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }
}

#[test]
fn rejects_invalid_root_capability_value_syntax() {
    for attributes in [
        "friends = [_(Source) -> \"Bit\"]",
        "friends = [_(Source) -> [Bit, 1]]",
        "friends = [_(Source) -> {Bit}]",
    ] {
        let input = format!("konst = false, {attributes}");
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }
}
