use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Fields, Item, ItemStruct};

use crate::integral::{Cfg, ekran};

fn expand(source: &str) -> syn::Result<TokenStream> {
    let configuration = syn::parse_str::<Cfg>("konst = none")?;
    let item = syn::parse_str::<ItemStruct>(source)?;
    return ekran(Box::new(configuration), item);
}

fn reject_at(source: &str, marker: &str, diagnostic: &str) {
    let result = expand(source);
    assert!(result.is_err(), "{source}");
    let error = result.unwrap_err();
    let position = source.find(marker).unwrap();
    let prefix = &source[..position];
    let line = prefix.bytes().filter(|it| *it == b'\n').count() + 1;
    let column = prefix.rsplit('\n').next().unwrap().len();

    assert!(error.to_string().contains(diagnostic), "{error}");
    assert_eq!(error.span().start().line, line);
    assert_eq!(error.span().start().column, column);
}

#[test]
fn generic_shape_preserves_declaration_and_marker_order() {
    let source = "
        struct Tagged<
            'a,
            #[allow(unused)] T: ?Sized,
            const SPACE: u8 = 7,
            const ENABLED: bool = true,
            Meta = (),
        >(u32)
        where
            T: 'a,
            Meta: Clone;
    ";
    let original = syn::parse_str::<ItemStruct>(source);
    assert!(original.is_ok(), "{source}");
    let original = original.unwrap();
    let result = expand(source);
    assert!(result.is_ok(), "{source}");
    let expanded = syn::parse2::<syn::File>(result.unwrap());
    assert!(expanded.is_ok(), "{source}");
    let expanded = expanded.unwrap();
    let target = expanded.items.iter().find_map(|item| match item {
        Item::Struct(item) if item.ident == original.ident => Some(item),
        _ => None,
    }).unwrap();
    let Fields::Unnamed(fields) = &target.fields else {
        panic!("integral declaration lost its tuple shape");
    };
    let types = fields.unnamed.iter().map(|field| &field.ty).collect::<Vec<_>>();

    assert_eq!(target.generics.to_token_stream().to_string(),
        original.generics.to_token_stream().to_string());
    assert_eq!(target.generics.where_clause.to_token_stream().to_string(),
        original.generics.where_clause.to_token_stream().to_string());
    assert_eq!(types.len(), 6);
    assert_eq!(types[0].to_token_stream().to_string(), "u32");
    assert_eq!(types[1].to_token_stream().to_string(),
        quote! { ::core::marker::PhantomData<&'a ()> }.to_string());
    assert_eq!(types[2].to_token_stream().to_string(),
        quote! { ::core::marker::PhantomData<T> }.to_string());
    assert_eq!(types[5].to_token_stream().to_string(),
        quote! { ::core::marker::PhantomData<Meta> }.to_string());
    assert!(fields.unnamed.iter().all(|field|
        matches!(field.vis, syn::Visibility::Inherited)));

    let markers = expanded.items.iter().filter_map(|item| match item {
        Item::Struct(item) if item.ident != original.ident => Some(item),
        _ => None,
    }).collect::<Vec<_>>();
    assert_eq!(markers.len(), 2);
    assert_ne!(markers[0].ident, markers[1].ident);
    let first_name = &markers[0].ident;
    let second_name = &markers[1].ident;
    assert_eq!(types[3].to_token_stream().to_string(),
        quote! { ::core::marker::PhantomData<#first_name<SPACE>> }.to_string());
    assert_eq!(types[4].to_token_stream().to_string(),
        quote! { ::core::marker::PhantomData<#second_name<ENABLED>> }.to_string());
    let syn::GenericParam::Const(first) = &markers[0].generics.params[0] else {
        panic!("const marker must preserve the parameter kind");
    };
    let syn::GenericParam::Const(second) = &markers[1].generics.params[0] else {
        panic!("const marker must preserve the parameter kind");
    };
    assert_eq!(first.ty.to_token_stream().to_string(), "u8");
    assert_eq!(second.ty.to_token_stream().to_string(), "bool");
}

#[test]
fn nongeneric_shape_does_not_acquire_marker_helpers() {
    let source = "struct Plain(u32);";
    let result = expand(source);
    assert!(result.is_ok(), "{source}");
    let expanded = syn::parse2::<syn::File>(result.unwrap());
    assert!(expanded.is_ok(), "{source}");
    let expanded = expanded.unwrap();
    let structs = expanded.items.iter().filter_map(|item| match item {
        Item::Struct(item) => Some(item),
        _ => None,
    }).collect::<Vec<_>>();
    let Fields::Unnamed(fields) = &structs[0].fields else {
        panic!("integral declaration lost its tuple shape");
    };

    assert_eq!(structs.len(), 1);
    assert_eq!(fields.unnamed.len(), 1);
    assert!(structs[0].generics.params.is_empty());
}

#[test]
fn generic_public_value_field_is_rejected_at_visibility() {
    let source = "struct Tagged<T>(pub u32);";
    reject_at(source, "pub", "generic integral types require a private value field");
}

#[test]
fn generic_restricted_value_field_is_rejected_at_visibility() {
    let source = "struct Tagged<T>(pub(crate) u32);";
    reject_at(source, "pub", "generic integral types require a private value field");
}

#[test]
fn source_marker_field_is_rejected_at_the_extra_field() {
    let source = "struct Tagged<T>(u32, core::marker::PhantomData<T>);";
    reject_at(source, "core", "phantom fields are generated automatically");
}

#[test]
fn generic_backing_field_is_rejected_at_the_field_type() {
    let source = "struct Tagged<T>(T);";
    reject_at(source, "T);", "concrete primitive integer");
}

#[test]
fn named_generic_integral_is_rejected_at_the_target() {
    let source = "struct Tagged<T> { value: u32 }";
    reject_at(source, "Tagged", "expected a tuple struct");
}

#[test]
fn unit_generic_integral_is_rejected_at_the_target() {
    let source = "struct Tagged<T>;";
    reject_at(source, "Tagged", "expected a tuple struct");
}
