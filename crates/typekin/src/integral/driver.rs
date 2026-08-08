use crate::integral::maker::Generator;
pub(crate) use crate::integral::maker::IntegralFlags;
use crate::runner;
use crate::runner::MkErr;
use crate::type_friendship::FriendReq;
use crate::value_type::N;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ExprPath;
use syn::Fields;
use syn::Path;
use syn::Result;
use syn::Type;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_quote;

#[derive(Default, Debug, Clone)]
pub(crate) struct IntegralCfg {
    pub(crate) int: Box<IntegralFlags>,
    fn_get_raw: Option<ExprPath>,
    pub(crate) fn_validator: Option<Path>,
    pub(crate) friends: Vec<FriendReq>,
}

impl Parse for IntegralCfg {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut this = Self::default();

        runner::parse_inner_attributes(input, |attr, span, rest| {
            match attr {
                "with" => this.int.parse_from(rest, true)?,
                "without" => this.int.parse_from(rest, false)?,

                "friends" => {
                    this.friends = runner::list(rest)?.collect();
                }

                "fn_get_raw" => {
                    this.fn_get_raw = Some(rest.parse()?);
                }

                "fn_validator" => {
                    this.fn_validator = Some(rest.parse()?);
                }

                _ => return span.fail("unknown integral arg"),
            };

            return Ok(());
        })?;

        return Ok(this);
    }
}

pub(crate) fn ekran(
    ty: Ident,
    el: Ident,
    cfg: IntegralCfg,
) -> Result<TokenStream> {
    return Generator::new(
        ty,
        el,
        cfg.int,
        cfg.fn_get_raw.unwrap_or_else(|| parse_quote! { Self::raw }),
        cfg.fn_validator,
        cfg.friends,
    )?
    .ekran();
}

pub(crate) fn integral(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(attr as IntegralCfg);
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

        let stream = ekran(ty, el, args)?;
        return Ok(quote! { #item #stream });
    });
}
