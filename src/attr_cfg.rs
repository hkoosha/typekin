use proc_macro2::{
    TokenStream,
    TokenTree,
};
use syn::parse::ParseStream;
use syn::{
    Expr,
    ExprRange,
    Path,
    Token,
    bracketed,
};

use crate::runner;
use crate::runner::MkErr;

#[derive(Default, Clone)]
pub(crate) struct ValidationCfg {
    pub(crate) callbacks: Vec<Path>,
    pub(crate) ranges: Vec<ExprRange>,
}

impl ValidationCfg {
    pub(crate) fn has_validation(&self) -> bool {
        return !self.callbacks.is_empty() || !self.ranges.is_empty();
    }

    pub(crate) fn parse_attr(
        &mut self,
        attr: &str,
        input: ParseStream,
    ) -> syn::Result<bool> {
        match attr {
            "valid" => self.callbacks = parse_callbacks(input)?,
            "in" => self.ranges = parse_ranges(input)?,
            _ => return Ok(false),
        };

        return Ok(true);
    }
}

fn parse_ranges(input: ParseStream) -> syn::Result<Vec<ExprRange>> {
    let tokens = if input.peek(syn::token::Bracket) {
        let content;
        let _ = bracketed!(content in input);
        content.parse()?
    }
    else {
        let mut tokens = TokenStream::new();

        while !input.is_empty() && !input.peek(Token![,]) {
            let token: TokenTree = input.parse()?;
            tokens.extend([token]);
        }

        tokens
    };

    fn no_attr_range(it: Expr) -> syn::Result<ExprRange> {
        return match it {
            Expr::Range(range) if range.attrs.is_empty() => Ok(range),
            expr => expr.fail("invalid range definition"),
        };
    }

    let mut ranges = vec![];
    let mut range = TokenStream::new();

    for token in tokens {
        if matches!(&token, TokenTree::Punct(it) if it.as_char() == '+') {
            ranges.push(no_attr_range(syn::parse2(range)?)?);
            range = TokenStream::new();
        }
        else {
            range.extend([token]);
        }
    }

    // WTF?
    ranges.push(no_attr_range(syn::parse2(range)?)?);
    return Ok(ranges);
}

pub(crate) fn parse_callbacks(input: ParseStream) -> syn::Result<Vec<Path>> {
    if input.peek(syn::token::Bracket) {
        return Ok(runner::list::<Path>(input)?.collect());
    }

    return Ok(vec![input.parse()?]);
}
