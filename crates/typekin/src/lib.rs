extern crate core;

pub(crate) mod runner;
pub(crate) mod type_friendship;
pub(crate) mod value_type;

pub(crate) mod flag;
pub(crate) mod integral;

#[proc_macro_attribute]
pub fn integral(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return integral::driver::integral(attr, item);
}

#[proc_macro_attribute]
pub fn bitflag(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    return flag::driver::bitflag(attr, item);
}
