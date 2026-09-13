use crate::flag::cfg::BitflagCfg;
use crate::flag::maker::Maker;
use crate::runner;
use crate::runner::MkErr;
use quote::quote;
use syn::Fields;
use syn::spanned::Spanned;

pub(crate) fn bitflag(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let cfg = Box::new(syn::parse_macro_input!(attr as BitflagCfg));
    let item = syn::parse_macro_input!(item as syn::ItemEnum);

    return runner::ekran_catching(move || {
        let repr = runner::find_repr_n(&item.attrs, item.span())?;
        let ty = item.ident.clone();
        let items = item.variants.iter().map(|it| it.ident.clone()).collect();

        if let Some(bad) = item
            .variants
            .iter()
            .find(|it| !matches!(it.fields, Fields::Unit))
        {
            return bad.fail("only unit variants are supported");
        }

        let stream = Maker::new(ty, repr, cfg, items).ekran()?;
        return Ok(quote! { #item #stream });
    });
}
