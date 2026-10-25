use proc_macro2::{
    Span,
    TokenStream,
};
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

use crate::zz_runner::{
    MkErr,
    MySpanned,
};

#[derive(Clone)]
pub(crate) struct SelfConverter {
    pub(crate) conversion: Path,
    pub(crate) target: Type,
}

impl MySpanned for SelfConverter {
    fn my_span(&self) -> Span {
        return self.conversion.span();
    }
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
pub(crate) struct OperatorOutput<'a> {
    configuration: &'a SelfConverter,
    ty: TokenStream,
    conversion: TokenStream,
}

impl<'a> OperatorOutput<'a> {
    pub(crate) fn of(
        configuration: &'a SelfConverter,
        ty: TokenStream,
        conversion: TokenStream,
    ) -> Self {
        return Self {
            configuration,
            ty,
            conversion,
        };
    }

    pub(crate) fn ty(&self) -> TokenStream {
        return self.ty.clone();
    }

    pub(crate) fn convert(
        &self,
        value: TokenStream,
    ) -> TokenStream {
        if self.configuration.is_self() {
            return value;
        }

        let conversion = &self.conversion;
        return quote! { #conversion(#value) };
    }
}

impl MySpanned for OperatorOutput<'_> {
    fn my_span(&self) -> Span {
        return self.configuration.my_span();
    }
}
