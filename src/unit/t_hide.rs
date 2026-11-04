#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::ItemStruct;

    use crate::integral::ekran;
    use crate::integral_cfg::Cfg;
    use crate::zz_hide::inner_visibility;

    fn expand(
        config: &str,
        source: &str,
    ) -> syn::Result<String> {
        let config = syn::parse_str::<Cfg>(config)?;
        let item = syn::parse_str::<ItemStruct>(source)?;
        return ekran(Box::new(config), item).map(|it| it.to_string());
    }

    #[test]
    fn hide_in_rejects_an_empty_string() {
        let it = r#"hide_in = """#;
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");

        assert_eq!(
            this.unwrap_err().to_string(),
            "hide_in takes a module name or none"
        );
    }

    #[test]
    fn hide_in_rejects_a_quoted_name() {
        let it = r#"hide_in = "inner""#;
        let this = syn::parse_str::<Cfg>(it);
        assert!(this.is_err(), "{it}");

        assert_eq!(
            this.unwrap_err().to_string(),
            "hide_in takes a module name or none"
        );
    }

    #[test]
    fn hide_in_rejects_duplicates() {
        let it = "hide_in = a, hide_in = b";
        let this = syn::parse_str::<Cfg>(it);

        assert!(this.is_err(), "{it}");
    }

    #[test]
    fn hide_in_is_rejected_inside_bitflag_integral() {
        let it = "integral = [hide_in = none]";
        let this = syn::parse_str::<crate::bitflag::Cfg>(it);
        assert!(this.is_err(), "{it}");

        assert_eq!(
            this.unwrap_err().to_string(),
            "hide_in belongs on the bitflag root, not in integral = [..]"
        );
    }

    #[test]
    fn private_item_becomes_pub_super_inside_the_module() {
        let declared = syn::Visibility::Inherited;
        let inner = inner_visibility(&declared);

        assert_eq!(quote! { #inner }.to_string(), "pub (super)");
    }

    #[test]
    fn pub_super_item_reaches_the_grandparent_inside_the_module() {
        let declared: syn::Visibility = syn::parse_quote! { pub(super) };
        let inner = inner_visibility(&declared);

        assert_eq!(quote! { #inner }.to_string(), "pub (in super :: super)");
    }

    #[test]
    fn crate_and_pub_items_keep_their_visibility() {
        let krate: syn::Visibility = syn::parse_quote! { pub(crate) };
        let public: syn::Visibility = syn::parse_quote! { pub };
        let inner_krate = inner_visibility(&krate);
        let inner_public = inner_visibility(&public);

        assert_eq!(quote! { #inner_krate }.to_string(), "pub (crate)");
        assert_eq!(quote! { #inner_public }.to_string(), "pub");
    }

    #[test]
    fn get_raw_is_rejected_when_hidden() {
        let expansion = expand(
            "konst = none, without = fn_conv_raw, get_raw = Self::value",
            "struct P(u8);",
        );
        assert!(expansion.is_err(), "{expansion:?}");

        assert!(
            expansion
                .unwrap_err()
                .to_string()
                .starts_with("get_raw needs access to the value field"),
        );
    }

    #[test]
    fn custom_or_panic_is_rejected_when_hidden() {
        let expansion =
            expand("konst = none, without = fn_or_panic", "struct P(u8);");
        assert!(expansion.is_err(), "{expansion:?}");

        assert!(
            expansion
                .unwrap_err()
                .to_string()
                .starts_with("a custom or_panic constructor needs access"),
        );
    }

    #[test]
    fn super_attribute_string_is_rejected_when_hidden() {
        let expansion = expand(
            "konst = none",
            r#"#[serde(with = "super::codec")] struct P(u8);"#,
        );
        assert!(expansion.is_err(), "{expansion:?}");

        assert!(
            expansion
                .unwrap_err()
                .to_string()
                .contains("would resolve inside hide_in's module"),
        );
    }

    #[test]
    fn doc_strings_mentioning_super_are_allowed() {
        let expansion =
            expand("konst = none", r#"#[doc = "super::codec"] struct P(u8);"#);

        assert!(expansion.is_ok(), "{expansion:?}");
    }
}
