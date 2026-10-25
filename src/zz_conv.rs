use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::spanned::Spanned;
use syn::{
    Path,
    Token,
    Type,
    parenthesized,
};

use crate::zz_runner::MkErr;

#[derive(Clone)]
pub(crate) struct SelfConverter {
    pub(crate) conversion: Path,
    pub(crate) target: Type,
}

impl Parse for SelfConverter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let conversion = input.parse::<Path>()?;
        let content;
        let _ = parenthesized!(content in input);
        let source = content.parse::<Type>()?;

        if !content.is_empty() {
            return content
                .span()
                .fail("type conversion requires exactly one `_` argument");
        }
        else if !matches!(&source, Type::Infer(_)) {
            return source
                .span()
                .fail("type conversion requires a `_` argument");
        }

        input.parse::<Token![->]>()?;
        let target = input.parse::<Type>()?;

        return Ok(Self { conversion, target });
    }
}

#[derive(Clone)]
pub(crate) struct OperatorOutput<'a>(pub(crate) Option<&'a SelfConverter>);

impl<'a> OperatorOutput<'a> {
    pub(crate) fn of(it: Option<&'a SelfConverter>) -> Self {
        return Self(it);
    }

    pub(crate) fn ty(&self) -> TokenStream {
        return match self.0 {
            Some(SelfConverter { target, .. }) => quote! { #target },
            None => quote! { Self },
        };
    }

    pub(crate) fn convert(
        &self,
        value: TokenStream,
    ) -> TokenStream {
        return match self.0 {
            Some(SelfConverter { conversion, .. }) => quote! {
                #conversion(#value)
            },
            None => value,
        };
    }
}
