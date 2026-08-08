#![allow(dead_code)]

use crate::flag::maker::BitFlags;
use crate::flag::maker::Generator;
use crate::integral::driver::IntegralCfg;
use crate::runner;
use crate::runner::MkErr;
use crate::value_type::N;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::{Fields, Result};

#[derive(Default)]
pub(crate) struct BitflagCfg {
    bit: Box<BitFlags>,
    int_cfg: Option<IntegralCfg>,
}

impl BitflagCfg {
    fn parse_attr(
        &mut self,
        span: Span,
        input: ParseStream,
        key: &str,
    ) -> Result<()> {
        match key {
            "with" => self.bit.parse_from(input, true)?,
            "without" => self.bit.parse_from(input, false)?,

            "integral" => {
                self.int_cfg =
                    Some(IntegralCfg::parse(&runner::unbracket(&input)?)?);
            }

            _ => return span.fail("unknown bitflag arg"),
        };

        return Ok(());
    }
}

impl Parse for BitflagCfg {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut this = Self::default();

        runner::parse_inner_attributes(input, |attr, span, rest| {
            return this.parse_attr(span, rest, attr);
        })?;

        return Ok(this);
    }
}

pub(crate) fn ekran(
    vis: Visibility,
    ty: Ident,
    repr: N,
    items: Vec<Ident>,
    cfg: BitflagCfg,
) -> Result<TokenStream> {
    let int_cfg = cfg.int_cfg.unwrap_or_default();
    let bit = cfg.bit;

    let stream = Generator::new(ty, repr, int_cfg, bit, vis, items).ekran()?;
    return Ok(stream);
}

pub(crate) fn bitflag(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(attr as BitflagCfg);
    let item = syn::parse_macro_input!(item as syn::ItemEnum);

    return runner::ekran_catching(move || {
        let repr = runner::find_repr_n(&item.attrs, item.span())?;
        let ty = item.ident.clone();
        let vis = item.vis.clone();
        let items = item.variants.iter().map(|it| it.ident.clone()).collect();

        if let Some(bad) = item
            .variants
            .iter()
            .find(|it| !matches!(it.fields, Fields::Unit))
        {
            return bad.fail("only unit variants are supported");
        }

        let stream = ekran(vis, ty, repr, items, args)?;
        return Ok(quote! { #item #stream });
    });
}
