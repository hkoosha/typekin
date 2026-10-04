extern crate core;

use syn::{
    Item,
    ItemEnum,
    ItemStruct,
    parse_macro_input,
};

pub(crate) mod bitflag;
pub(crate) mod constructor;
pub(crate) mod friendship;
pub(crate) mod integral;
pub(crate) mod runner;
pub(crate) mod text;
pub(crate) mod value_type;
pub(crate) mod zz;

#[cfg(test)]
mod tests;

#[proc_macro_attribute]
pub fn integral(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = Box::new(parse_macro_input!(attr as integral::Cfg));
    let item = parse_macro_input!(item as ItemStruct);

    return integral::integral(attr, item);
}

#[proc_macro_attribute]
pub fn bitflag(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = Box::new(parse_macro_input!(attr as bitflag::Cfg));
    let item = parse_macro_input!(item as ItemEnum);

    return bitflag::ekran(attr, item);
}

/// - `of_relation`: Path;
///
/// Optional attributes:
/// - `maker`: Ident
///   Defaults: `of`
///   Name of generated constructor fn accepting friend instances for construction of Self..
/// - `friends`: Path OR \[Path, ...]
///   Default: `[]`.
///   List of friends.
/// - `mod`: `_` OR `self` OR Ident
///   Default: `_` which expands to `const _: () { ... }`.
///   Name of module to put generated stuff in.
#[proc_macro_attribute]
pub fn text(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as text::Cfg);
    let item = parse_macro_input!(item as ItemStruct);

    return text::ekran(attr, item);
}

/// Mandatory attributes:
/// - `relation`: Type
///   Value produced by friend conversions.
///
/// Optional attributes:
/// - `friends`: single `conversion(TySource) -> Capability` OR a list of.
///   Default: `[]`.
///   Declares friends, their conversion fn and their capabilities.
/// - `scope`: `_` OR `self` OR Item.
///   Default: `_` which expands to `const _: () { ... }`.
///   Where the generated protocol and constructor are put into.
#[proc_macro_attribute]
pub fn friends(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as friendship::Cfg);
    let item = parse_macro_input!(item as Item);

    return friendship::ekran(attr, item);
}

#[proc_macro_attribute]
pub fn constructor(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as constructor::Cfg);
    let item = parse_macro_input!(item as Item);
    return constructor::ekran(attr, item);
}
