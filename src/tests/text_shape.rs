use crate::text::{
    Cfg,
    TextMaker,
};

fn reject_at(
    source: &str,
    marker: &str,
) {
    let item = syn::parse_str(source).expect("valid Rust struct syntax");
    let error = match TextMaker::new(item, Cfg::default()) {
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
fn missing_transparency_points_to_the_struct_name() {
    for source in ["struct Value(String);", "#[repr(C)]\nstruct Value(String);"]
    {
        reject_at(source, "Value");
    }
}

#[test]
fn unsupported_field_shapes_point_to_the_fields() {
    for (source, marker) in [
        ("#[repr(transparent)]\nstruct Value;", "Value"),
        ("#[repr(transparent)]\nstruct Value();", "()"),
        ("#[repr(transparent)]\nstruct Value(String, u8);", "(String"),
        ("#[repr(transparent)]\nstruct Value { inner: String }", "{"),
    ] {
        reject_at(source, marker);
    }
}

#[test]
fn unsupported_field_types_point_to_the_type() {
    for field in [
        "u8",
        "Vec<u8>",
        "&'static str",
        "Option<String>",
        "(String,)",
        "[String; 1]",
        "alloc::string::String",
        "String<u8>",
    ] {
        let source = format!("#[repr(transparent)]\nstruct Value({field});");
        reject_at(&source, field);
    }
}

#[test]
fn unsupported_generics_point_to_parameters_or_where_clause() {
    for (source, marker) in [
        ("#[repr(transparent)]\nstruct Value<T>(T);", "T"),
        ("#[repr(transparent)]\nstruct Value<'a>(&'a str);", "'a"),
        (
            "#[repr(transparent)]\nstruct Value<const N: usize>(String);",
            "const N",
        ),
        (
            "#[repr(transparent)]\nstruct Value(String) where String: Clone;",
            "where",
        ),
    ] {
        reject_at(source, marker);
    }
}
