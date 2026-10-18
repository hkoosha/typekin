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

    use crate::zz_runner::{
        ToCollection,
        mk_flags,
    };
    use crate::zz_value_type::N;
    use crate::{
        zz,
        zz_runner,
    };

    struct OneOrList(Vec<Ident>);

    mk_flags! {
        #[flag_default(bool=true, str="")]
        struct TestFlags {
            pub display: bool,
        }
    }

    impl Parse for OneOrList {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            return Ok(Self(zz::one_or_list(input)?.vec()));
        }
    }

    #[derive(Debug)]
    struct FlagLists;

    impl Parse for FlagLists {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let mut flags = TestFlags::new();

            let with = input.parse::<Ident>()?;
            if with != "with" {
                return Err(syn::Error::new(with.span(), "expected `with`"));
            }
            input.parse::<syn::Token![=]>()?;
            flags.parse_from(input, true)?;

            input.parse::<syn::Token![,]>()?;

            let without = input.parse::<Ident>()?;
            if without != "without" {
                return Err(syn::Error::new(
                    without.span(),
                    "expected `without`",
                ));
            }
            input.parse::<syn::Token![=]>()?;
            flags.parse_from(input, false)?;

            return Ok(Self);
        }
    }

    #[test]
    fn rejects_flags_enabled_and_disabled_in_the_shared_flag_parser() {
        let error = syn::parse_str::<FlagLists>(
            "with = [display], without = [display]",
        )
        .expect_err("a generation flag cannot be both enabled and disabled");

        assert_eq!(
            error.to_string(),
            "flag specified in both `with` and `without`",
        );
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
    fn snake_case_handles_single_capitals_and_acronym_boundaries_t_1() {
        let (input, expected) = ("T", "t");
        assert_eq!(zz_runner::snake_case_of(input), expected);
    }

    #[test]
    fn snake_case_handles_single_capitals_and_acronym_boundaries_httpserver_2()
    {
        let (input, expected) = ("HTTPServer", "http_server");
        assert_eq!(zz_runner::snake_case_of(input), expected);
    }

    #[test]
    fn snake_case_handles_single_capitals_and_acronym_boundaries_myurlvalue_3()
    {
        let (input, expected) = ("MyURLValue", "my_url_value");
        assert_eq!(zz_runner::snake_case_of(input), expected);
    }

    #[test]
    fn snake_case_handles_single_capitals_and_acronym_boundaries_x509certificate_4()
     {
        let (input, expected) = ("X509Certificate", "x509_certificate");
        assert_eq!(zz_runner::snake_case_of(input), expected);
    }
}
