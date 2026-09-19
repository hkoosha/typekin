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
    fn rejects_missing_konst() {
        let error = syn::parse_str::<IntegralCfg>("friends = [u8]")
            .expect_err("konst must be explicit");
        assert_eq!(error.to_string(), "missing required `konst` argument");
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
        cfg.fn_validator = Some(syn::parse_quote!(is_valid));
        let maker = crate::integral::Maker::new(
            syn::parse_quote!(Number),
            syn::parse_quote!(u32),
            Box::new(cfg),
        )
        .expect("a primitive integral representation should be valid");
        assert!(!maker.ekran_items().to_string().contains("fn make"));
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
