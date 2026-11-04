#[cfg(test)]
mod tests {
    use syn::spanned::Spanned;
    use syn::{
        Attribute,
        ItemStruct,
    };
    use zz_lib as zz;

    use crate::zz_runner::MkErr;
    use crate::zz_values::N;
    use crate::{
        zz_lib,
        zz_runner,
    };

    fn find_repr_n(
        attrs: &[Attribute],
        span: impl Spanned + quote::ToTokens,
    ) -> syn::Result<N> {
        return match zz::find_repr_n(attrs, &span)? {
            None => return span.fail("missing numeric repr"),
            Some(it) => Ok(it),
        };
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
        assert_eq!(find_repr_n(attrs, &item).unwrap(), N::U008);
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
        let error = find_repr_n(attrs, &item).unwrap_err();
        assert_eq!(error.to_string(), "missing numeric repr");
    }

    #[test]
    fn catcher_converts_syn_errors_to_compile_errors() {
        let this = zz_runner::catching(|| {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "runner error",
            ));
        });
        let output = this.to_string();

        assert!(output.contains("compile_error"));
        assert!(output.contains("runner error"));
    }

    #[test]
    fn catcher_follows_internal_panic_configuration() {
        let this = std::panic::catch_unwind(|| {
            return zz_runner::catching(|| {
                panic!("runner panic");
            });
        });

        if zz_runner::typekin_catch_panics() {
            assert!(this.is_ok());
            let output = this.unwrap().to_string();
            assert!(output.contains("compile_error"));
            assert!(output.contains("runner panic"));
        }
        else {
            assert!(this.is_err());
        }
    }
}
