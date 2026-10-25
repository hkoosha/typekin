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
        let input = "
            with = [display],
            without = [display]
        ";
        let error = syn::parse_str::<FlagLists>(input).unwrap_err();

        assert_eq!(
            error.to_string(),
            "flag specified in both `with` and `without`",
        );
    }

    #[test]
    fn accepts_single_one_or_list() {
        let input = "One";
        let expected = &["One"][..];
        let this = syn::parse_str::<OneOrList>(input);
        assert!(this.is_ok(), "{input}");
        let parsed = this.unwrap();
        let actual = parsed.0.iter().map(ToString::to_string).vec();

        assert_eq!(actual, expected);
    }

    #[test]
    fn accepts_bracketed_one_or_list() {
        let input = "[One, Two]";
        let expected = &[
            "One", "Two",
        ][..];
        let this = syn::parse_str::<OneOrList>(input);
        assert!(this.is_ok(), "{input}");
        let parsed = this.unwrap();
        let actual = parsed.0.iter().map(ToString::to_string).vec();

        assert_eq!(actual, expected);
    }

    #[test]
    fn accepts_empty_one_or_list() {
        let input = "[]";
        let expected: &[&str] = &[];
        let this = syn::parse_str::<OneOrList>(input);
        assert!(this.is_ok(), "{input}");
        let parsed = this.unwrap();
        let actual = parsed.0.iter().map(ToString::to_string).vec();

        assert_eq!(actual, expected);
    }

    #[test]
    fn numeric_repr_helper_finds_the_primitive_type() {
        let input = "
            #[repr(C, transparent, u8)]
            struct Value(u8);
        ";
        let this = syn::parse_str::<ItemStruct>(input);
        assert!(this.is_ok(), "{input}");
        let item = this.unwrap();
        let attrs = &item.attrs;
        assert_eq!(zz::find_repr_n(attrs, &item).unwrap(), N::U008);
    }

    #[test]
    fn numeric_repr_helper_rejects_missing_numeric_repr() {
        let input = "
            #[repr(C)]
            struct Value(u8);
        ";
        let this = syn::parse_str::<ItemStruct>(input);
        assert!(this.is_ok(), "{input}");
        let item = this.unwrap();
        let attrs = &item.attrs;
        let error = zz::find_repr_n(attrs, &item).unwrap_err();
        assert_eq!(error.to_string(), "missing numeric repr");
    }

    #[test]
    fn snake_cases_acronym_boundary() {
        let input = "HTTPServer";
        let expected = "http_server";
        assert_eq!(zz_runner::snake_case_of(input), expected);
    }

    #[test]
    fn snake_cases_embedded_acronym() {
        let input = "MyURLValue";
        let expected = "my_url_value";
        assert_eq!(zz_runner::snake_case_of(input), expected);
    }

    #[test]
    fn snake_cases_alphanumeric_prefix() {
        let input = "X509Certificate";
        let expected = "x509_certificate";
        assert_eq!(zz_runner::snake_case_of(input), expected);
    }
}
