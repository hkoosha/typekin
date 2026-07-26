use crate::integral::driver;
use crate::integral::driver::IntegralCfgParser;
use proc_macro2::TokenStream;
use std::panic::UnwindSafe;
use std::sync::Arc;
use std::sync::Mutex;

// mod flag;
mod integral;
pub(crate) mod integral_op;
pub(crate) mod integral_token;
pub(crate) mod integral_types;

fn backtraced<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R, String> {
    let err = Arc::new(Mutex::new(None));

    let err0 = err.clone();
    std::panic::set_hook(Box::new(move |it| {
        let text =
            format!("\n{}\n{}\n", it, std::backtrace::Backtrace::capture());
        err0.lock().unwrap().replace(text);
    }));

    let result = std::panic::catch_unwind(f);

    let _ = std::panic::take_hook();

    return result.map_err(|_| err.lock().unwrap().take().unwrap());
}

fn ekran_catching(
    f: impl FnOnce() -> syn::Result<TokenStream> + UnwindSafe
) -> proc_macro::TokenStream {
    return backtraced(|| {
        f().unwrap_or_else(|it| it.to_compile_error().into())
    })
    .map_err(|mut it| {
        let remove = "disabled backtrace\n";
        if it.ends_with(remove) {
            it.truncate(it.len() - remove.len());
        }
        return it;
    })
    .map_err(|it| quote::quote! { compile_error!(#it); })
    .unwrap_or_else(|it| it.into())
    .into();
}

#[proc_macro_attribute]
pub fn integral(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(attr as IntegralCfgParser);
    let item = syn::parse_macro_input!(item as syn::ItemStruct);

    return ekran_catching(move || driver::ekran(args, item));
}
