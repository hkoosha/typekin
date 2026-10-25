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
    parse_quote,
};

use crate::zz_runner::MkErr;

#[derive(Clone)]
pub(crate) struct SelfConverter {
    pub(crate) conversion: Path,
    pub(crate) target: Type,
}

impl Default for SelfConverter {
    fn default() -> Self {
        return Self {
            conversion: parse_quote! { Self },
            target: parse_quote! { Self },
        };
    }
}

impl Parse for SelfConverter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let conversion = input.parse::<Path>()?;
        if conversion.is_ident("Self") && !input.peek(syn::token::Paren) {
            return Ok(Self::default());
        }

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

impl SelfConverter {
    pub(crate) fn is_self(&self) -> bool {
        return self.conversion.is_ident("Self");
    }
}

#[derive(Clone)]
pub(crate) struct OperatorOutput<'a>(pub(crate) &'a SelfConverter);

impl<'a> OperatorOutput<'a> {
    pub(crate) fn of(it: &'a SelfConverter) -> Self {
        return Self(it);
    }

    pub(crate) fn ty(&self) -> TokenStream {
        if self.0.is_self() {
            return quote! { Self };
        }

        let target = &self.0.target;
        return quote! { #target };
    }

    pub(crate) fn convert(
        &self,
        value: TokenStream,
    ) -> TokenStream {
        if self.0.is_self() {
            return value;
        }

        let conversion = &self.0.conversion;
        return quote! { #conversion(#value) };
    }
}
