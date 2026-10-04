use crate::integral::Cfg;

#[test]
fn accepts_single_friend_with_scalar_or_list_capabilities() {
    for input in [
        "friends = crate::convert(crate::source::Source) -> Make, konst = false",
        "konst = false, friends = crate::convert(crate::source::Source) -> [Make, Trust], valid = is_valid",
        "konst = false, valid = is_valid, friends = _(crate::source::Source) -> Bit",
        "konst = false, friends = _(crate::source::Source) -> [Math, Bit], in = 1..=4",
        "konst = false, friends = _ -> Math",
        "konst = false, friends = _ -> [Math, Bit], with = [display]",
    ] {
        syn::parse_str::<Cfg>(input).expect(input);
    }
}

#[test]
fn accepts_bracketed_multiple_and_empty_friend_lists() {
    for input in [
        "konst = false, friends = [convert(Source) -> Make, _(Other) -> [Math, Bit]], valid = is_valid",
        "konst = false, friends = [], valid = is_valid",
    ] {
        syn::parse_str::<Cfg>(input).expect(input);
    }
}

#[test]
fn rejects_invalid_single_friend_boundaries() {
    for input in [
        "konst = false, friends = convert(Source) -> Make, convert(Other) -> Make",
        "konst = false, friends = _(Source) -> Bit, _(Other) -> Bit",
        "konst = false, friends =",
        "konst = false, friends =, valid = is_valid",
        "konst = false, friends = convert(Source)",
        "konst = false, friends = convert() -> Make",
        "konst = false, friends = convert(Source) ->",
        "konst = false, friends = convert(Source) -> [Make, 1]",
        "konst = false, friends = _(Source) -> Unknown",
        "konst = false, friends = _(Source) -> [Bit], unknown = true",
        "konst = false, friends = _(Source) -> Bit, friends = []",
    ] {
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }
}

#[test]
fn accepts_explicit_konst_values() {
    let konst = syn::parse_str::<Cfg>("konst = true")
        .expect("explicit true konst should parse");
    assert!(konst.konst);

    let konst = syn::parse_str::<Cfg>("konst = false")
        .expect("explicit false konst should parse");
    assert!(!konst.konst);
}

#[test]
fn accepts_root_validation_attributes() {
    let config = syn::parse_str::<Cfg>(
        "konst = false, valid = is_even, in = 1..=4",
    )
    .expect("root validation attributes should parse");

    assert_eq!(config.validation.callbacks.len(), 1);
    assert_eq!(config.validation.ranges.len(), 1);
    assert!(config.has_validation());
}

#[test]
fn accepts_root_validation_lists() {
    let config = syn::parse_str::<Cfg>(
            "konst = false, valid = [is_even, is_not_fifty], in = [1..=4 + 8..10 + ..=0]",
        )
        .expect("root validation lists should parse");

    assert_eq!(config.validation.callbacks.len(), 2);
    assert_eq!(config.validation.ranges.len(), 3);
}

#[test]
fn accepts_unbracketed_root_in_union_with_valid_callback() {
    let config = syn::parse_str::<Cfg>(
        "konst = false, in = 1..2 + 6..8, valid = foo",
    )
    .expect("an unbracketed root in union with a valid callback should parse");

    assert_eq!(config.validation.callbacks.len(), 1);
    assert_eq!(config.validation.ranges.len(), 2);
}

#[test]
fn rejects_non_in_validation() {
    let error = syn::parse_str::<Cfg>("konst = false, in = [1]")
        .expect_err("in validation needs range expressions");

    assert_eq!(error.to_string(), "invalid range definition");
}

#[test]
fn rejects_comma_separated_ranges() {
    syn::parse_str::<Cfg>("konst = false, in = [1..2, 3..4]")
        .expect_err("multiple ranges use + separators");
}

#[test]
fn rejects_legacy_validation_attributes() {
    let callback =
        syn::parse_str::<Cfg>("konst = false, callback = is_valid")
            .expect_err("valid is the validation callback key");
    assert_eq!(callback.to_string(), "unknown attribute");

    let range = syn::parse_str::<Cfg>("konst = false, range = 1..2")
        .expect_err("in is the range constraint key");
    assert_eq!(range.to_string(), "unknown attribute");

    let validator =
        syn::parse_str::<Cfg>("konst = false, validator = is_valid")
            .expect_err("valid is the validation callback key");
    assert_eq!(validator.to_string(), "unknown attribute");
}

#[test]
fn rejects_missing_konst() {
    let error = syn::parse_str::<Cfg>("friends = [_(u8) -> Bit]")
        .expect_err("konst must be explicit");
    assert_eq!(error.to_string(), "missing required `konst` argument");
}

#[test]
fn accepts_explicit_friend_capabilities() {
    let config = syn::parse_str::<Cfg>(
        "konst = false, friends = [convert(u8) -> [Make, Bit]]",
    )
    .expect("integral capabilities should parse");
    let friend = config.friends.iter().next().unwrap();

    assert!(friend.capabilities.contains(&syn::parse_quote!(Make)));
    assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));
}

#[test]
fn accepts_shared_friend_without_conversion() {
    let config = syn::parse_str::<Cfg>(
        "konst = false, friends = [_(u8) -> Bit]",
    )
    .expect("integral may use the shared Into conversion default");
    let friend = config.friends.iter().next().unwrap();

    assert!(friend.conv.is_none());
    assert!(friend.ty.as_ref().unwrap().is_ident("u8"));

    assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));
}

#[test]
fn rejects_legacy_friend_levels_and_custom_trait_names() {
    syn::parse_str::<Cfg>(
        "konst = false, friends = [u8(level = [Full])]",
    )
    .expect_err("levels were replaced by capabilities");

    let trait_name =
        syn::parse_str::<Cfg>("konst = false, trait_seal = OtherSeal")
            .expect_err("integral seal name is fixed");
    assert_eq!(trait_name.to_string(), "unknown attribute");

    let of_name =
        syn::parse_str::<Cfg>("konst = false, fn_of = custom_of")
            .expect_err("integral of name is fixed");
    assert_eq!(of_name.to_string(), "unknown attribute");
}

#[test]
fn accepts_trusted_capability_without_make() {
    for capabilities in [
        "Trust",
        "[Trust]",
        "[Math, Trust]",
        "[Bit, Relation, Trust]",
    ] {
        let input =
            format!("konst = false, friends = [_(Source) -> {capabilities}]");
        syn::parse_str::<Cfg>(&input).expect(&input);
    }
}

#[test]
fn rejects_legacy_trusted_argument() {
    for trusted in ["true", "false"] {
        let input = format!(
            "konst = false, friends = [Source(cap = Make, trusted = {trusted})]"
        );
        assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
    }
}

#[test]
fn rejects_duplicate_display_flags_and_generation_arguments() {
    for input in [
        "konst = false, with = [display, display]",
        "konst = false, without = [display, display]",
        "konst = false, with = [display], with = [display]",
        "konst = false, without = [display], without = [display]",
    ] {
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }
}

#[test]
fn rejects_unsupported_display_flags() {
    for input in [
        "konst = false, with = [display, display_names]",
        "konst = false, without = [display_names]",
    ] {
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }
}

#[test]
fn rejects_conflicting_generation_flags_in_either_order_and_lists() {
    for input in [
        "konst = false, with = [display], without = [display]",
        "konst = false, without = [display], with = [display]",
        "konst = false, with = [display, impl_math_add], without = [impl_math_add, fn_conv_raw]",
        "konst = false, without = [impl_math_add, fn_conv_raw], with = [display, impl_math_add]",
    ] {
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }
}

#[test]
fn accepts_from_str_generation_flag() {
    for input in [
        "konst = false, with = [impl_from_str]",
        "konst = false, without = [impl_from_str]",
    ] {
        syn::parse_str::<Cfg>(input).expect(input);
    }
}

#[test]
fn accepts_core_integral_generation_flag() {
    for input in [
        "konst = false, with = [impl_core_int]",
        "konst = false, without = [impl_core_int]",
    ] {
        syn::parse_str::<Cfg>(input).expect(input);
    }
}

#[test]
fn rejects_undocumented_with_aliases() {
    for input in [
        "konst = false, with_impl_core_int = [impl_core_int]",
        "konst = false, with_display = [display]",
    ] {
        let error = syn::parse_str::<Cfg>(input)
            .expect_err("generation options use `with = [...]`");
        assert_eq!(error.to_string(), "unknown attribute");
    }
}

#[test]
fn rejects_unsupported_single_capabilities() {
    for input in [
        "konst = false, friends = [_(Source) -> Rel]",
        "konst = false, friends = [_(Source) -> Inspector]",
    ] {
        assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
    }
}

#[test]
fn shared_friend_parser_accepts_qualified_conversion_and_source_paths() {
    let friend = syn::parse_str::<crate::friendship::cfg::Friend>(
        "crate::conversions::own(crate::sources::Source) -> [Make, Trust]",
    )
    .expect("qualified conversion and source paths should parse");

    let conversion = friend.conv.as_ref().unwrap();
    let source = friend.ty.as_ref().unwrap();
    assert_eq!(
        conversion
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>(),
        ["crate", "conversions", "own"]
    );
    assert_eq!(
        source
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>(),
        ["crate", "sources", "Source"]
    );
    assert!(friend.capabilities.contains(&syn::parse_quote!(Make)));
    assert!(friend.capabilities.contains(&syn::parse_quote!(Trust)));
}

#[test]
fn shared_friend_parser_accepts_scalar_list_and_empty_capabilities() {
    for (capabilities, expected) in [
        ("Trust", vec!["Trust"]),
        ("[Trust]", vec!["Trust"]),
        ("[Make, Trust]", vec!["Make", "Trust"]),
        ("[Make, Trust,]", vec!["Make", "Trust"]),
        ("[]", vec![]),
    ] {
        let input = format!("convert(Source) -> {capabilities}");
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(&input)
            .expect(&input);
        assert!(friend.conv.as_ref().unwrap().is_ident("convert"));
        assert!(friend.ty.as_ref().unwrap().is_ident("Source"));
        assert_eq!(
            friend
                .capabilities
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            expected,
            "{input}"
        );
    }
}

#[test]
fn shared_friend_parser_accepts_typed_implicit_conversion() {
    for input in ["_(Source) -> Trust", "_(Source,) -> [Make, Trust]"] {
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(input)
            .expect(input);
        assert!(friend.conv.is_none(), "{input}");
        assert!(friend.ty.as_ref().unwrap().is_ident("Source"), "{input}");
        assert!(friend.capabilities.contains(&syn::parse_quote!(Trust)));
    }
}

#[test]
fn shared_friend_parser_accepts_capability_only_declarations() {
    for (input, expected) in [
        ("_ -> Foo", vec!["Foo"]),
        ("_ -> [Foo, Bar]", vec!["Bar", "Foo"]),
        ("_ -> []", vec![]),
    ] {
        let friend = syn::parse_str::<crate::friendship::cfg::Friend>(input)
            .expect(input);
        assert!(friend.conv.is_none(), "{input}");
        assert!(friend.ty.is_none(), "{input}");
        assert_eq!(
            friend
                .capabilities
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            expected,
            "{input}"
        );
    }
}

#[test]
fn shared_friend_parser_accepts_absolute_paths_and_trailing_source_comma() {
    let friend = syn::parse_str::<crate::friendship::cfg::Friend>(
        "::conversions::own(::sources::Source,) -> Make",
    )
    .expect("one source argument may have a trailing comma");

    let conversion = friend.conv.as_ref().unwrap();
    let source = friend.ty.as_ref().unwrap();
    assert!(conversion.leading_colon.is_some());
    assert_eq!(conversion.segments.first().unwrap().ident, "conversions");
    assert_eq!(conversion.segments.last().unwrap().ident, "own");
    assert!(source.leading_colon.is_some());
    assert_eq!(source.segments.first().unwrap().ident, "sources");
    assert_eq!(source.segments.last().unwrap().ident, "Source");
}

#[test]
fn shared_friend_parser_rejects_obsolete_relationship_syntax() {
    for input in [
        "Source",
        "Source -> Make",
        "Source(conv = convert, cap = Make)",
        "Source(cap = [Make, Trust])",
        "_(cap = Make)",
        "Source(level = [Full])",
        "Source(conv = convert, cap = Make, trusted = true)",
        "Source(conv = convert, cap = Make, trusted = false)",
    ] {
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }
}

#[test]
fn shared_friend_parser_does_not_suggest_obsolete_migrations() {
    for input in ["Source(conv = convert, cap = Make)", "_(cap = Make)"] {
        let error = syn::parse_str::<crate::friendship::cfg::Friend>(input)
            .expect_err("obsolete friend syntax must fail");
        assert!(
            !error.to_string().contains("were removed"),
            "{input}: {error}"
        );
        assert!(!error.to_string().contains("use `_ ->"), "{input}: {error}");
    }
}

#[test]
fn shared_friend_parser_requires_arrow_and_capabilities() {
    for input in [
        "convert(Source)",
        "convert(Source) Make",
        "convert(Source) ->",
        "_(Source)",
        "_(Source) ->",
        "_",
        "_ Make",
        "_ ->",
    ] {
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }
}

#[test]
fn shared_friend_parser_requires_exactly_one_source_path() {
    for input in [
        "convert() -> Make",
        "convert(Source, Other) -> Make",
        "convert(_) -> Make",
        "convert(&Source) -> Make",
        "convert((Source)) -> Make",
        "_() -> Make",
        "_(Source, Other) -> Make",
        "_(_) -> Make",
    ] {
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }
}

#[test]
fn shared_friend_parser_rejects_malformed_capabilities_and_extra_tokens() {
    for input in [
        "convert(Source) -> \"Make\"",
        "convert(Source) -> [Make, 1]",
        "convert(Source) -> {Make}",
        "convert(Source) -> (Make)",
        "convert(Source) -> [Make Trust]",
        "convert(Source) -> [Make,, Trust]",
        "convert(Source) -> Make::Nested",
        "convert(Source) -> [Make::Nested]",
        "convert(Source) -> Make extra",
        "convert(Source) -> [Make] extra",
        "convert(Source) -> Make, Trust",
        "_ -> Foo(Source)",
    ] {
        assert!(
            syn::parse_str::<crate::friendship::cfg::Friend>(input).is_err(),
            "{input}"
        );
    }
}
