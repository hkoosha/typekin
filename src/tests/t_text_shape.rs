#[cfg(test)]
mod tests {
    use crate::text::{
        Cfg,
        TextMaker,
    };

    fn reject_at(
        source: &str,
        marker: &str,
    ) {
        let this = syn::parse_str(source);
        assert!(this.is_ok(), "{source}");
        let item = this.unwrap();
        let error =
            TextMaker::new(item, Cfg::new(crate::zz_lib::Konst::None, false))
                .err()
                .expect("unsupported text shape accepted");
        let prefix = &source[..source.find(marker).expect("diagnostic marker")];
        let line = prefix.bytes().filter(|it| *it == b'\n').count() + 1;
        let column = prefix.rsplit('\n').next().unwrap().len();
        let span = error.span().start();
        assert_eq!((span.line, span.column), (line, column), "{source}");
    }

    #[test]
    fn rejects_unit_struct() {
        let source = "
            #[repr(transparent)]
            struct Value;
        ";
        let marker = "Value";
        reject_at(source, marker);
    }

    #[test]
    fn rejects_unit_tuple_struct() {
        let source = "
            #[repr(transparent)]
            struct Value();
        ";
        let marker = "()";
        reject_at(source, marker);
    }

    #[test]
    fn rejects_multi_field_tuple_struct() {
        let source = "
            #[repr(transparent)]
            struct Value(String, u8);
        ";
        let marker = "(String";
        reject_at(source, marker);
    }

    #[test]
    fn rejects_named_field_struct() {
        let source = "
            #[repr(transparent)]
            struct Value { inner: String }
        ";
        let marker = "{";
        reject_at(source, marker);
    }

    #[test]
    fn rejects_u8_field() {
        let field = "u8";
        let source = format!(
            "
                #[repr(transparent)]
                struct Value({field});
            "
        );
        reject_at(&source, field);
    }

    #[test]
    fn rejects_static_str_field() {
        let field = "&'static str";
        let source = format!(
            "
            #[repr(transparent)]
            struct Value({field});
        "
        );
        reject_at(&source, field);
    }

    #[test]
    fn rejects_tuple_string_field() {
        let field = "(String,)";
        let source = format!(
            "
            #[repr(transparent)]
            struct Value({field});
        "
        );
        reject_at(&source, field);
    }

    #[test]
    fn rejects_string_array_field() {
        let field = "[String; 1]";
        let source = format!(
            "
            #[repr(transparent)]
            struct Value({field});
        "
        );
        reject_at(&source, field);
    }

    #[test]
    fn rejects_qualified_string_field() {
        let field = "alloc::string::String";
        let source = format!(
            "
            #[repr(transparent)]
            struct Value({field});
        "
        );
        reject_at(&source, field);
    }

    #[test]
    fn rejects_generic_string_field() {
        let field = "String<u8>";
        let source = format!(
            "
            #[repr(transparent)]
            struct Value({field});
        "
        );
        reject_at(&source, field);
    }

    #[test]
    fn rejects_type_parameter() {
        let source = "
            #[repr(transparent)]
            struct Value<T>(T);
        ";
        let marker = "T";
        reject_at(source, marker);
    }

    #[test]
    fn rejects_where_clause() {
        let source = "
            #[repr(transparent)]
            struct Value(String) where String: Clone;
        ";
        let marker = "where";
        reject_at(source, marker);
    }
}
