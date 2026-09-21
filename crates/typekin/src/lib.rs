pub(crate) mod runner;
pub(crate) mod value_type;

pub(crate) mod bitflag;
pub(crate) mod friendship;
pub(crate) mod integral;

#[cfg(test)]
#[path = "tests/bitflag.rs"]
mod bitflag_tests;

#[cfg(test)]
#[path = "tests/integral.rs"]
mod integral_tests;

#[proc_macro_attribute]
pub fn integral(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return integral::integral(attr, item);
}

#[proc_macro_attribute]
pub fn bitflag(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return bitflag::bitflag(attr, item);
}

#[proc_macro_attribute]
pub fn friendship(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return friendship::friendship(attr, item);
}

#[proc_macro_attribute]
pub fn constructor(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return friendship::constructor(attr, item);
}
