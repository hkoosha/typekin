#[cfg(test)]
mod tests {
    use syn::parse::{
        Parse,
        ParseStream,
    };
    use syn::{
        Ident,
        ItemStruct,
    };

    use crate::runner::ToCollection;
    use crate::value_type::N;
    use crate::{
        runner,
        zz,
    };

    struct OneOrList(Vec<Ident>);

    impl Parse for OneOrList {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            return Ok(Self(zz::one_or_list(input)?.vec()));
        }
    }

    #[test]
    fn one_or_list_preserves_single_bracketed_and_empty_forms_one_1() {
        let (input, expected) = ("One", &["One"][..]);
        let parsed = syn::parse_str::<OneOrList>(input).unwrap();
        let actual = parsed.0.iter().map(ToString::to_string).vec();

        assert_eq!(actual, expected);
    }

    #[test]
    fn one_or_list_preserves_single_bracketed_and_empty_forms_one_2() {
        let (input, expected) = (
            "[One, Two]",
            &[
                "One", "Two",
            ][..],
        );
        let parsed = syn::parse_str::<OneOrList>(input).unwrap();
        let actual = parsed.0.iter().map(ToString::to_string).vec();

        assert_eq!(actual, expected);
    }

    #[test]
    fn one_or_list_preserves_single_bracketed_and_empty_forms_case_3() {
        let (input, expected): (&str, &[&str]) = ("[]", &[]);
        let parsed = syn::parse_str::<OneOrList>(input).unwrap();
        let actual = parsed.0.iter().map(ToString::to_string).vec();

        assert_eq!(actual, expected);
    }

    #[test]
    fn numeric_repr_helper_finds_the_primitive_type() {
        let item = syn::parse_str::<ItemStruct>(
            "#[repr(C, transparent, u8)] struct Value(u8);",
        )
        .unwrap();
        let attrs = &item.attrs;
        assert_eq!(zz::find_repr_n(attrs, &item).unwrap(), N::U008);
    }

    #[test]
    fn transparent_repr_helper_recognizes_transparent_types() {
        let item = syn::parse_str::<ItemStruct>(
            "#[repr(C, transparent, u8)] struct Value(u8);",
        )
        .unwrap();
        let attrs1 = &item.attrs;
        assert!(zz::find_repr_transparent(attrs1).unwrap());
    }

    #[test]
    fn numeric_repr_helper_rejects_missing_numeric_repr() {
        let item = syn::parse_str::<ItemStruct>("#[repr(C)] struct Value(u8);")
            .unwrap();
        let attrs = &item.attrs;
        assert_eq!(
            zz::find_repr_n(attrs, &item).unwrap_err().to_string(),
            "missing numeric repr"
        );
    }

    #[test]
    fn transparent_repr_helper_rejects_nontransparent_types() {
        let item = syn::parse_str::<ItemStruct>("#[repr(C)] struct Value(u8);")
            .unwrap();
        let attrs1 = &item.attrs;
        assert!(!zz::find_repr_transparent(attrs1).unwrap());
    }

    #[test]
    fn snake_case_handles_single_capitals_and_acronym_boundaries_t_1() {
        let (input, expected) = ("T", "t");
        assert_eq!(runner::snake_case_of(input), expected);
    }

    #[test]
    fn snake_case_handles_single_capitals_and_acronym_boundaries_httpserver_2()
    {
        let (input, expected) = ("HTTPServer", "http_server");
        assert_eq!(runner::snake_case_of(input), expected);
    }

    #[test]
    fn snake_case_handles_single_capitals_and_acronym_boundaries_myurlvalue_3()
    {
        let (input, expected) = ("MyURLValue", "my_url_value");
        assert_eq!(runner::snake_case_of(input), expected);
    }

    #[test]
    fn snake_case_handles_single_capitals_and_acronym_boundaries_x509certificate_4()
     {
        let (input, expected) = ("X509Certificate", "x509_certificate");
        assert_eq!(runner::snake_case_of(input), expected);
    }
}
