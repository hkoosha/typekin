use crate::integral::cfg::IntegralCfg;
use crate::integral::maker::Maker;
use crate::runner;
use crate::runner::MkErr;
use crate::value_type::N;
use quote::quote;
use syn::Fields;
use syn::Type;

pub(crate) fn integral(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let cfg = Box::new(syn::parse_macro_input!(attr as IntegralCfg));
    let item = syn::parse_macro_input!(item as syn::ItemStruct);

    return runner::ekran_catching(move || {
        let ty = item.ident.clone();

        let el = if let Fields::Unnamed(fields) = &item.fields
            && fields.unnamed.len() == 1
            && let Type::Path(field) = &fields.unnamed[0].ty
            && let Some(id) = field.path.get_ident()
            && N::rust_names().contains(&id.to_string().as_str())
        {
            id.clone()
        }
        else {
            return item.fail(
                "expected a tuple struct with exactly one field of primitive integral type",
            );
        };

        if !runner::find_repr_transparent(&item.attrs)? {
            return item.fail("expecting #[repr(transparent, ...)]");
        }

        let it = Maker::new(ty, el, cfg)?.ekran()?;

        let stream = quote::quote! {
            #[allow(dead_code)]
            #[allow(unused_qualifications)]
            const _: () = {
                #it
            };
        };

        return Ok(quote! { #item #stream });
    });
}
