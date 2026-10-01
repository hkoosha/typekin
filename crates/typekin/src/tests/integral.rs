#[cfg(test)]
mod test {
    use crate::integral::IntegralCfg;

    #[test]
    fn accepts_explicit_konst_values() {
        let konst = syn::parse_str::<IntegralCfg>("konst = true")
            .expect("explicit true konst should parse");
        assert!(konst.konst);

        let konst = syn::parse_str::<IntegralCfg>("konst = false")
            .expect("explicit false konst should parse");
        assert!(!konst.konst);
    }

    #[test]
    fn accepts_root_validation_attributes() {
        let config = syn::parse_str::<IntegralCfg>(
            "konst = false, valid = is_even, in = 1..=4",
        )
        .expect("root validation attributes should parse");

        assert_eq!(config.validation.callbacks.len(), 1);
        assert_eq!(config.validation.ranges.len(), 1);
        assert!(config.has_validation());
    }

    #[test]
    fn accepts_root_validation_lists() {
        let config = syn::parse_str::<IntegralCfg>(
            "konst = false, valid = [is_even, is_not_fifty], in = [1..=4 + 8..10 + ..=0]",
        )
        .expect("root validation lists should parse");

        assert_eq!(config.validation.callbacks.len(), 2);
        assert_eq!(config.validation.ranges.len(), 3);
    }

    #[test]
    fn accepts_unbracketed_root_in_union_with_valid_callback() {
        let config = syn::parse_str::<IntegralCfg>(
            "konst = false, in = 1..2 + 6..8, valid = foo",
        )
        .expect(
            "an unbracketed root in union with a valid callback should parse",
        );

        assert_eq!(config.validation.callbacks.len(), 1);
        assert_eq!(config.validation.ranges.len(), 2);
    }

    #[test]
    fn rejects_non_in_validation() {
        let error = syn::parse_str::<IntegralCfg>("konst = false, in = [1]")
            .expect_err("in validation needs range expressions");

        assert_eq!(error.to_string(), "invalid range definition");
    }

    #[test]
    fn rejects_comma_separated_ranges() {
        syn::parse_str::<IntegralCfg>("konst = false, in = [1..2, 3..4]")
            .expect_err("multiple ranges use + separators");
    }

    #[test]
    fn rejects_legacy_validation_attributes() {
        let callback =
            syn::parse_str::<IntegralCfg>("konst = false, callback = is_valid")
                .expect_err("valid is the validation callback key");
        assert_eq!(callback.to_string(), "unknown attribute");

        let range =
            syn::parse_str::<IntegralCfg>("konst = false, range = 1..2")
                .expect_err("in is the range constraint key");
        assert_eq!(range.to_string(), "unknown attribute");

        let validator = syn::parse_str::<IntegralCfg>(
            "konst = false, validator = is_valid",
        )
        .expect_err("valid is the validation callback key");
        assert_eq!(validator.to_string(), "unknown attribute");
    }

    #[test]
    fn rejects_missing_konst() {
        let error = syn::parse_str::<IntegralCfg>("friends = [u8]")
            .expect_err("konst must be explicit");
        assert_eq!(error.to_string(), "missing required `konst` argument");
    }

    #[test]
    fn accepts_explicit_friend_capabilities() {
        let config = syn::parse_str::<IntegralCfg>(
            "konst = false, friends = [u8(conv = self, cap = [Make, Bit])]",
        )
        .expect("integral capabilities should parse");
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.capabilities.contains(&syn::parse_quote!(Make)));
        assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));
    }

    #[test]
    fn accepts_shared_friend_without_conversion() {
        let config = syn::parse_str::<IntegralCfg>(
            "konst = false, friends = [u8(cap = [Bit])]",
        )
        .expect("integral may use the shared Into conversion default");
        let friend = config.friends.iter().next().unwrap();

        assert!(friend.conv.is_none());
        assert!(friend.capabilities.contains(&syn::parse_quote!(Bit)));
    }

    #[test]
    fn rejects_legacy_friend_levels_and_custom_trait_names() {
        let level = syn::parse_str::<IntegralCfg>(
            "konst = false, friends = [u8(level = [Full])]",
        )
        .expect_err("levels were replaced by capabilities");
        assert_eq!(level.to_string(), "unknown attribute");

        let trait_name = syn::parse_str::<IntegralCfg>(
            "konst = false, trait_seal = OtherSeal",
        )
        .expect_err("integral seal name is fixed");
        assert_eq!(trait_name.to_string(), "unknown attribute");

        let of_name =
            syn::parse_str::<IntegralCfg>("konst = false, fn_of = custom_of")
                .expect_err("integral of name is fixed");
        assert_eq!(of_name.to_string(), "unknown attribute");
    }

    #[test]
    fn generates_make_only_for_unvalidated_raw_inputs() {
        let maker = crate::integral::Maker::new(
            syn::parse_quote!(Number),
            syn::parse_quote!(u32),
            Box::default(),
        )
        .expect("a primitive integral representation should be valid");
        let generated = maker.ekran_items().to_string();
        assert!(generated.contains(
            "pub fn make (it : u32) -> Number { return Number :: of (it) ; }"
        ));

        let mut cfg = IntegralCfg::default();
        cfg.flags.auto_of_raw = false;
        let maker = crate::integral::Maker::new(
            syn::parse_quote!(Number),
            syn::parse_quote!(u32),
            Box::new(cfg),
        )
        .expect("a primitive integral representation should be valid");
        assert!(!maker.ekran_items().to_string().contains("fn make"));

        let mut cfg = IntegralCfg::default();
        cfg.validation.callbacks = vec![syn::parse_quote!(is_valid)];
        let maker = crate::integral::Maker::new(
            syn::parse_quote!(Number),
            syn::parse_quote!(u32),
            Box::new(cfg),
        )
        .expect("a primitive integral representation should be valid");
        let items = maker.ekran_items().to_string();
        assert!(!items.contains("fn make"));
        assert!(items.contains("pub fn of < T >"));

        let impls = maker
            .ekran_impls()
            .expect("validated integral generation should succeed")
            .to_string();
        assert!(!impls.contains("impl Make for u32"));
        assert!(!impls.contains("impl Make for u16"));
    }

    #[test]
    fn generates_make_with_configured_name() {
        let mut cfg = IntegralCfg::default();
        cfg.flags.fn_make = "from_raw".into();
        let maker = crate::integral::Maker::new(
            syn::parse_quote!(Number),
            syn::parse_quote!(u32),
            Box::new(cfg),
        )
        .expect("a primitive integral representation should be valid");
        assert!(maker.ekran_items().to_string().contains(
            "pub fn from_raw (it : u32) -> Number { return Number :: of (it) ; }"
        ));
    }
}
