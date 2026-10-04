use proc_macro2::{
    Ident,
    TokenStream,
    TokenTree,
};
use std::collections::HashSet;
use syn::meta::ParseNestedMeta;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Attribute,
    Expr,
    ExprRange,
    Path,
    Token,
    Visibility,
    bracketed,
};

use crate::{
    runner::MkErr,
    value_type::N,
};

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
        return Ok(list(input)?.collect());
    }

    return Ok(vec![input.parse()?]);
}

pub(crate) fn get_concrete_type(
    item: &mut syn::Item
) -> syn::Result<(&Ident, &Visibility, &mut Vec<Attribute>)> {
    const MSG: &str = "friendship only supports struct, enum, and union targets that have no generics";

    let span = item.span();

    let (target, visibility, attrs, generics) = match item {
        syn::Item::Struct(item) => {
            (&item.ident, &item.vis, &mut item.attrs, &item.generics)
        }
        syn::Item::Enum(item) => {
            (&item.ident, &item.vis, &mut item.attrs, &item.generics)
        }
        syn::Item::Union(item) => {
            (&item.ident, &item.vis, &mut item.attrs, &item.generics)
        }
        _ => return span.fail(MSG),
    };

    if !generics.params.is_empty() || generics.where_clause.is_some() {
        return span.fail(MSG);
    }

    return Ok((target, visibility, attrs));
}

pub(crate) fn pop_attr(
    attrs: &mut Vec<Attribute>,
    name: &str,
) -> syn::Result<Option<Attribute>> {
    let indexes = attrs
        .iter()
        .enumerate()
        .filter_map(|(index, attr)| {
            attr.path()
                .segments
                .last()
                .filter(|segment| segment.ident == name)
                .map(|_| index)
        })
        .collect::<Vec<_>>();

    let Some(index) = indexes.first().copied()
    else {
        return Ok(None);
    };

    if let Some(duplicate) = indexes.get(1) {
        return attrs[*duplicate].span().fail("duplicated attribute");
    }

    return Ok(Some(attrs.remove(index)));
}

pub(crate) fn list<T: Parse>(
    stream: ParseStream
) -> syn::Result<impl Iterator<Item = T>> {
    let content;
    let _ = bracketed!(content in stream);

    let items =
        Punctuated::<T, Token![,]>::parse_terminated(&content)?.into_iter();

    return Ok(items);
}

pub(crate) fn one_or_list<T: Parse>(
    stream: ParseStream
) -> syn::Result<impl Iterator<Item = T>> {
    let (one, many) = if stream.peek(syn::token::Bracket) {
        (None, Some(list(stream)?))
    }
    else {
        (Some(stream.parse::<T>()?), None)
    };

    return Ok(one.into_iter().chain(many.into_iter().flatten()));
}

pub(crate) fn find_repr_n(
    attrs: &[Attribute],
    span: impl Spanned,
) -> syn::Result<N> {
    let mut repr = None::<N>;

    for_repr(attrs, |it| {
        for n in N::items() {
            if it.path.is_ident(n.rust_name()) {
                if repr.is_some() {
                    return span.fail("multiple repr matched");
                }
                else {
                    repr = Some(*n);
                }
            }
        }
        return Ok(());
    })?;

    return match repr {
        None => return span.fail("missing numeric repr"),
        Some(it) => Ok(it),
    };
}

fn for_repr(
    attrs: &[Attribute],
    mut exe: impl FnMut(ParseNestedMeta) -> syn::Result<()>,
) -> syn::Result<()> {
    attrs
        .iter()
        .filter(|it| it.path().is_ident("repr"))
        .try_for_each(|it| it.parse_nested_meta(&mut exe))?;

    return Ok(());
}

pub(crate) fn find_repr_transparent(attrs: &[Attribute]) -> syn::Result<bool> {
    let mut found = false;

    for_repr(attrs, |it| {
        if it.path.is_ident("transparent") {
            found = true;
        }
        return Ok(());
    })?;

    return Ok(found);
}

// TODO cleanup the mess...
pub(crate) fn parse_inner(
    stream: ParseStream,
    mut on_attr: impl FnMut(&str, ParseStream) -> syn::Result<bool>,
) -> syn::Result<()> {
    let mut seen = HashSet::with_capacity(5);

    while !stream.is_empty() {
        let attr = {
            let here = stream.span();

            let it = if stream.peek(Token![_]) {
                let _: Token![_] = stream.parse()?;
                "_".to_string()
            }
            else if stream.peek(Token![mod]) {
                let _: Token![mod] = stream.parse()?;
                "mod".to_string()
            }
            else if stream.peek(Token![in]) {
                let _: Token![in] = stream.parse()?;
                "in".to_string()
            }
            else {
                let key: Ident = stream.parse()?;
                key.to_string()
            };

            if seen.contains(&it) {
                return here.fail("duplicated arg");
            }

            it
        };

        stream.parse::<Token![=]>()?;

        match on_attr(&attr, stream) {
            Ok(true) => {}
            Ok(false) => return stream.span().fail("unknown attribute"),
            Err(err) => {
                return Err(err);
            }
        }

        seen.insert(attr);

        if !stream.is_empty() {
            stream.parse::<Token![,]>()?;
        }
    }

    return Ok(());
}
