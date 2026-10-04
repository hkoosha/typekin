#[cfg(test)]
mod tests {
    use crate::text::Cfg;

    #[test]
    fn accepts_single_friend_without_consuming_neighboring_attributes() {
        for input in [
            "friends = crate::convert(crate::source::Source) -> Make, konst = false",
            "konst = false, friends = crate::convert(crate::source::Source) -> [Make, Trust], valid = is_slug",
            "valid = is_slug, friends = _(crate::source::Source) -> Trust",
            "friends = _(Self) -> [Rel, Trust], with = [display]",
        ] {
            syn::parse_str::<Cfg>(input).expect(input);
        }
    }

    #[test]
    fn accepts_bracketed_multiple_and_empty_friend_lists() {
        for input in [
            "friends = [convert(Source) -> [Make, Trust], _(Other) -> Trust], valid = is_slug",
            "friends = [], valid = is_slug",
        ] {
            syn::parse_str::<Cfg>(input).expect(input);
        }
    }

    #[test]
    fn rejects_invalid_single_friend_boundaries_and_family_capabilities() {
        for input in [
            "friends = convert(Source) -> Make, convert(Other) -> Make",
            "friends = _(Source) -> Trust, _(Other) -> Trust",
            "friends =",
            "friends =, valid = is_slug",
            "friends = convert(Source)",
            "friends = convert() -> Make",
            "friends = convert(Source) ->",
            "friends = convert(Source) -> [Make, 1]",
            "friends = _(Source) -> Make",
            "friends = _(Source) -> [Make, Trust]",
            "friends = _ -> Trust",
            "friends = _ -> [Rel, Trust]",
            "friends = convert(Source) -> Math",
            "friends = _(Source) -> [Trust], unknown = true",
            "friends = _(Source) -> Trust, friends = []",
        ] {
            assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
        }
    }

    #[test]
    fn accepts_callbacks_literals_and_trusted_make_friend() {
        let cfg = syn::parse_str::<Cfg>(
            "konst = false, valid = [is_slug, is_published], in = [\"draft\", \"published\"], friends = [Source::into_string(Source) -> [Make, Trust]], with = [display]",
        )
        .expect("text configuration should parse");

        assert!(!cfg.konst);
        assert_eq!(cfg.callbacks.len(), 2);
        assert_eq!(cfg.values.as_ref().unwrap().len(), 2);
        assert!(cfg.flags.display);
    }

    #[test]
    fn accepts_single_callback() {
        let cfg = syn::parse_str::<Cfg>("valid = is_slug")
            .expect("a single text callback should parse");

        assert_eq!(cfg.callbacks.len(), 1);
    }

    #[test]
    fn rejects_duplicate_literal_membership() {
        for input in [
            r#"in = ["draft", "draft"]"#,
            r#"in = ["é", "\u{e9}"]"#,
            r##"in = ["draft", r#"draft"#]"##,
        ] {
            assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
        }
    }

    #[test]
    fn rejects_non_literal_membership() {
        for input in [
            r#"in = "draft""#,
            r#"in = [DRAFT]"#,
            r#"in = [concat!("dra", "ft")]"#,
            r#"in = ["draft", 1]"#,
            r#"in = [b"draft"]"#,
            r#"in = 1..=3 + 8..10"#,
        ] {
            assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
        }
    }

    #[test]
    fn rejects_non_path_validation_callbacks() {
        for input in [
            r#"valid = "is_slug""#,
            r#"valid = [is_slug, "is_not_reserved"]"#,
            "valid = |value| true",
        ] {
            assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
        }
    }

    #[test]
    fn rejects_invalid_friend_configuration() {
        for input in [
            "friends = [_(Source) -> [Make]]",
            "friends = [_(Source) -> Make]",
            "friends = [_(Source) -> [Make, Trust]]",
        ] {
            syn::parse_str::<Cfg>(input)
                .expect_err("Make friends must provide an owning conversion");
        }
    }

    #[test]
    fn rejects_unknown_text_attributes_flags_and_capabilities() {
        for input in [
            "unknown = true",
            "get_raw = Self::raw",
            "with = [deref_mut]",
            "without = [impl_math_add]",
            "friends = [convert(Source) -> [Math]]",
            "friends = [convert(Source) -> [Bit]]",
            "friends = [convert(Source) -> [Relation]]",
            "friends = [_(Source) -> [Rel]]",
            "friends = [convert(Source) -> Math]",
            "friends = [convert(Source) -> Bit]",
            "friends = [convert(Source) -> Relation]",
        ] {
            assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
        }
    }

    #[test]
    fn rejects_duplicate_configuration_keys() {
        for input in [
            "konst = false, konst = false",
            "with = [display], with = [display]",
            "without = [display], without = [display]",
            "valid = is_slug, valid = is_slug",
            r#"in = ["draft"], in = ["draft"]"#,
            "friends = [], friends = []",
        ] {
            assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
        }
    }

    #[test]
    fn rejects_duplicate_generation_flags_and_friend_types() {
        for input in [
            "with = [display, display]",
            "without = [display, display]",
            "friends = [_(Self) -> [Rel], _(Self) -> [Rel]]",
            "friends = [first(Source) -> [Make], \
            second(Source) -> [Make, Trust]]",
        ] {
            assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
        }
    }

    #[test]
    fn accepts_trusted_capability_without_make_or_conversion() {
        for input in [
            "friends = [_(Self) -> [Rel, Trust]]",
            "friends = [_(Self) -> Trust]",
            "friends = [_(Self) -> [Trust]]",
            "friends = [_(Source) -> Trust]",
            "friends = [_(Source) -> [Trust]]",
        ] {
            syn::parse_str::<Cfg>(input).expect(input);
        }
    }

    #[test]
    fn accepts_self_relation_with_implicit_conversion() {
        for input in [
            "friends = [_(Self) -> Rel]",
            "friends = [_(Self) -> [Rel]]",
            "friends = [_(Self,) -> [Rel, Trust]]",
        ] {
            let config = syn::parse_str::<Cfg>(input).expect(input);
            let friend = config.friends.iter().next().unwrap();

            assert!(friend.ty.as_ref().unwrap().is_ident("Self"));
            assert!(friend.conv.is_none());
            assert!(friend.capabilities.contains(&syn::parse_quote!(Rel)));
        }
    }

    #[test]
    fn rejects_untyped_friend_declarations() {
        for input in [
            "friends = [_ -> Make]",
            "friends = [_ -> Rel]",
            "friends = [_ -> Trust]",
            "friends = [_ -> [Rel, Trust]]",
            "friends = [_ -> []]",
        ] {
            assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
        }
    }

    #[test]
    fn rejects_legacy_trusted_argument() {
        for trusted in ["true", "false"] {
            let input = format!(
                "friends = [Source(conv = convert, cap = Make, trusted = {trusted})]"
            );
            assert!(syn::parse_str::<Cfg>(&input).is_err(), "{input}");
        }
    }

    #[test]
    fn rejects_conflicting_generation_flags_in_either_order() {
        for input in [
            "with = [display], without = [display]",
            "without = [display], with = [display]",
        ] {
            assert!(syn::parse_str::<Cfg>(input).is_err(), "{input}");
        }
    }
}
