#[cfg(test)]
mod test {
    use crate::text::TextCfg;

    #[test]
    fn accepts_callbacks_literals_and_trusted_make_friend() {
        let cfg = syn::parse_str::<TextCfg>(
            "konst = false, valid = [is_slug, is_published], in = [\"draft\", \"published\"], friends = [Source(conv = Source::into_string, cap = [Make], trusted = true)], with = [display]",
        )
        .expect("text configuration should parse");

        assert!(!cfg.konst);
        assert_eq!(cfg.callbacks.len(), 2);
        assert_eq!(cfg.values.len(), 2);
        assert!(cfg.display);
        assert!(cfg.friends.iter().next().unwrap().trusted);
    }

    #[test]
    fn accepts_single_callback_and_defaults_to_non_const() {
        let cfg = syn::parse_str::<TextCfg>("valid = is_slug")
            .expect("a single text callback should parse");

        assert_eq!(cfg.callbacks.len(), 1);
        assert!(!cfg.konst);
    }

    #[test]
    fn rejects_empty_or_duplicate_literal_membership() {
        let empty = syn::parse_str::<TextCfg>("in = []")
            .expect_err("text membership cannot be empty");
        assert_eq!(
            empty.to_string(),
            "text `in` requires at least one string literal",
        );

        let duplicate =
            syn::parse_str::<TextCfg>("in = [\"draft\", \"draft\"]")
                .expect_err("text membership cannot contain duplicates");
        assert_eq!(duplicate.to_string(), "duplicated text literal");
    }

    #[test]
    fn rejects_invalid_friend_configuration() {
        let trusted = syn::parse_str::<TextCfg>(
            "friends = [Self(cap = [Rel], trusted = true)]",
        )
        .expect_err("trusted friends must make text");
        assert_eq!(trusted.to_string(), "text `trusted` requires `Make`");

        let missing_conversion =
            syn::parse_str::<TextCfg>("friends = [Source(cap = [Make])]")
                .expect_err("Make friends must provide an owning conversion");
        assert_eq!(
            missing_conversion.to_string(),
            "text Make friend requires `conv = ...`",
        );
    }
}
