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
pub(crate) struct OperatorOutput {
    cfg: SelfConverter,
    ty: TokenStream,
    conversion: TokenStream,
    /// The error an operation reports failure with, when it does
    /// (`panic_at` below `anything`): `Option<Raw>`.
    failure: Option<TokenStream>,
}

impl OperatorOutput {
    pub(crate) fn of(
        configuration: &SelfConverter,
        ty: TokenStream,
        conversion: TokenStream,
        failure: Option<TokenStream>,
    ) -> Self {
        return Self {
            cfg: configuration.clone(),
            ty,
            conversion,
            failure,
        };
    }

    /// The output type: the produced type, in a `Result` when operations
    /// report failure.
    pub(crate) fn ty(&self) -> TokenStream {
        let ty = &self.ty;
        return match &self.failure {
            Some(failure) => quote! {
                ::core::result::Result<#ty, #failure>
            },
            None => ty.clone(),
        };
    }

    /// `value`, the operation's `Self` (or `Result` of one), converted to
    /// the output.
    pub(crate) fn convert(
        &self,
        value: TokenStream,
    ) -> TokenStream {
        if self.cfg.is_self() {
            return value;
        }

        let conversion = &self.conversion;
        if self.failure.is_none() {
            return quote! { #conversion(#value) };
        }
        return quote! {
            match #value {
                ::core::result::Result::Ok(it) => {
                    ::core::result::Result::Ok(#conversion(it))
                }
                ::core::result::Result::Err(it) => {
                    ::core::result::Result::Err(it)
                }
            }
        };
    }
}

impl MySpanned for OperatorOutput {
    fn my_span(&self) -> Span {
        return self.cfg.my_span();
    }
}
