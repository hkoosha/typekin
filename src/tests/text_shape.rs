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
        let item = syn::parse_str(source).expect("valid Rust struct syntax");
        let error = match TextMaker::new(item, Cfg::new(false, false)) {
            Err(error) => error,
            Ok(_) => panic!("unsupported text shape accepted: {source}"),
        };
        let prefix = &source[..source.find(marker).expect("diagnostic marker")];
        let line = prefix.bytes().filter(|it| *it == b'\n').count() + 1;
        let column = prefix.rsplit('\n').next().unwrap().len();
        let span = error.span().start();
        assert_eq!((span.line, span.column), (line, column), "{source}");
    }

    #[test]
    fn missing_transparency_points_to_the_struct_name_struct_value_string_1() {
        let source = "struct Value(String);";
        reject_at(source, "Value");
    }

    #[test]
    fn missing_transparency_points_to_the_struct_name_repr_c_nstruct_value_string_2()
     {
        let source = "#[repr(C)]\nstruct Value(String);";
        reject_at(source, "Value");
    }

    #[test]
    fn unsupported_field_shapes_point_to_the_fields_repr_transparent_nstruct_value_1()
     {
        let (source, marker) = ("#[repr(transparent)]\nstruct Value;", "Value");
        reject_at(source, marker);
    }

    #[test]
    fn unsupported_field_shapes_point_to_the_fields_repr_transparent_nstruct_value_2()
     {
        let (source, marker) = ("#[repr(transparent)]\nstruct Value();", "()");
        reject_at(source, marker);
    }

    #[test]
    fn unsupported_field_shapes_point_to_the_fields_repr_transparent_nstruct_value_string_3()
     {
        let (source, marker) =
            ("#[repr(transparent)]\nstruct Value(String, u8);", "(String");
        reject_at(source, marker);
    }

    #[test]
    fn unsupported_field_shapes_point_to_the_fields_repr_transparent_nstruct_value_inner_string_4()
     {
        let (source, marker) =
            ("#[repr(transparent)]\nstruct Value { inner: String }", "{");
        reject_at(source, marker);
    }

    #[test]
    fn unsupported_field_types_point_to_the_type_u8_1() {
        let field = "u8";
        let source = format!("#[repr(transparent)]\nstruct Value({field});");
        reject_at(&source, field);
    }

    #[test]
    fn unsupported_field_types_point_to_the_type_vec_u8_2() {
        let field = "Vec<u8>";
        let source = format!("#[repr(transparent)]\nstruct Value({field});");
        reject_at(&source, field);
    }

    #[test]
    fn unsupported_field_types_point_to_the_type_static_str_3() {
        let field = "&'static str";
        let source = format!("#[repr(transparent)]\nstruct Value({field});");
        reject_at(&source, field);
    }

    #[test]
    fn unsupported_field_types_point_to_the_type_option_string_4() {
        let field = "Option<String>";
        let source = format!("#[repr(transparent)]\nstruct Value({field});");
        reject_at(&source, field);
    }

    #[test]
    fn unsupported_field_types_point_to_the_type_string_5() {
        let field = "(String,)";
        let source = format!("#[repr(transparent)]\nstruct Value({field});");
        reject_at(&source, field);
    }

    #[test]
    fn unsupported_field_types_point_to_the_type_string_1_6() {
        let field = "[String; 1]";
        let source = format!("#[repr(transparent)]\nstruct Value({field});");
        reject_at(&source, field);
    }

    #[test]
    fn unsupported_field_types_point_to_the_type_alloc_string_string_7() {
        let field = "alloc::string::String";
        let source = format!("#[repr(transparent)]\nstruct Value({field});");
        reject_at(&source, field);
    }

    #[test]
    fn unsupported_field_types_point_to_the_type_string_u8_8() {
        let field = "String<u8>";
        let source = format!("#[repr(transparent)]\nstruct Value({field});");
        reject_at(&source, field);
    }

    #[test]
    fn unsupported_generics_point_to_parameters_or_where_clause_repr_transparent_nstruct_value_t_t_1()
     {
        let (source, marker) =
            ("#[repr(transparent)]\nstruct Value<T>(T);", "T");
        reject_at(source, marker);
    }

    #[test]
    fn unsupported_generics_point_to_parameters_or_where_clause_repr_transparent_nstruct_value_a_a_str_2()
     {
        let (source, marker) =
            ("#[repr(transparent)]\nstruct Value<'a>(&'a str);", "'a");
        reject_at(source, marker);
    }

    #[test]
    fn unsupported_generics_point_to_parameters_or_where_clause_repr_transparent_nstruct_value_const_n_usize_str_3()
     {
        let (source, marker) = (
            "#[repr(transparent)]\nstruct Value<const N: usize>(String);",
            "const N",
        );
        reject_at(source, marker);
    }

    #[test]
    fn unsupported_generics_point_to_parameters_or_where_clause_repr_transparent_nstruct_value_string_where_stri_4()
     {
        let (source, marker) = (
            "#[repr(transparent)]\nstruct Value(String) where String: Clone;",
            "where",
        );
        reject_at(source, marker);
    }
}
