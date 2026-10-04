#[cfg(test)]
mod tests {
    use syn::{
        Ident,
        ItemStruct,
        parse::{
            Parse,
            ParseStream,
        },
    };

    use crate::{
        runner,
        value_type::N,
        zz,
    };

    struct OneOrList(Vec<Ident>);

    impl Parse for OneOrList {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            return Ok(Self(zz::one_or_list(input)?.collect()));
        }
    }

    #[test]
    fn one_or_list_preserves_single_bracketed_and_empty_forms() {
        for (input, expected) in [
            ("One", &["One"][..]),
            (
                "[One, Two]",
                &[
                    "One", "Two",
                ][..],
            ),
            ("[]", &[][..]),
        ] {
            let parsed = syn::parse_str::<OneOrList>(input).unwrap();
            let actual =
                parsed.0.iter().map(ToString::to_string).collect::<Vec<_>>();

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn repr_helpers_distinguish_numeric_and_transparent_representations() {
        let item = syn::parse_str::<ItemStruct>(
            "#[repr(C, transparent, u8)] struct Value(u8);",
        )
        .unwrap();
        let attrs = &item.attrs;
        assert_eq!(zz::find_repr_n(attrs, &item).unwrap(), N::U008);
        let attrs1 = &item.attrs;
        assert!(zz::find_repr_transparent(attrs1).unwrap());

        let item = syn::parse_str::<ItemStruct>("#[repr(C)] struct Value(u8);")
            .unwrap();
        let attrs = &item.attrs;
        assert_eq!(
            zz::find_repr_n(attrs, &item).unwrap_err().to_string(),
            "missing numeric repr"
        );
        let attrs1 = &item.attrs;
        assert!(!zz::find_repr_transparent(attrs1).unwrap());
    }

    #[test]
    fn snake_case_handles_single_capitals_and_acronym_boundaries() {
        for (input, expected) in [
            ("T", "t"),
            ("HTTPServer", "http_server"),
            ("MyURLValue", "my_url_value"),
            ("X509Certificate", "x509_certificate"),
        ] {
            assert_eq!(runner::snake_case_of(input), expected);
        }
    }
}
