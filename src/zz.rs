use std::collections::HashSet;
use std::fmt::{
    Debug,
    Formatter,
};

use proc_macro2::Ident;
use syn::meta::ParseNestedMeta;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Attribute,
    ExprRange,
    Path,
    Token,
    Visibility,
    bracketed,
    parenthesized,
};

use crate::runner::MkErr;
use crate::value_type::N;

#[derive(Default, Clone)]
pub(crate) struct ValidationCfg {
    pub(crate) callbacks: Vec<Path>,
    pub(crate) ranges: Vec<ExprRange>,
}

impl Debug for ValidationCfg {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "ValidationCfg{{callback={}, ranges={}}}",
            to_debug(&self.callbacks),
            to_debug(&self.ranges),
        )
    }
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

pub(crate) use ranges::parse_ranges;

use crate::runner::{
    ToCollection,
    to_debug,
};

mod ranges {
    use std::cmp::Ordering;

    use syn::parse::ParseStream;
    use syn::{
        Expr,
        ExprRange,
        Lit,
        RangeLimits,
        UnOp,
    };

    use crate::runner::{
        MkErr,
        ToCollection,
    };
    use crate::value_type::{
        Int,
        N,
        RangeEnding,
    };
    use crate::zz::one_or_list;

    pub(crate) fn parse_ranges(
        input: ParseStream
    ) -> syn::Result<Vec<ExprRange>> {
        let mut ranges = one_or_list(input)?
            .map(|it| {
                let Expr::Range(range) = it
                else {
                    return it.fail("invalid range definition");
                };

                if !range.attrs.is_empty() {
                    return range.fail("invalid range definition");
                }

                if let Some(start) = &range.start {
                    integer_constant(start)?;
                }
                if let Some(end) = &range.end {
                    integer_constant(end)?;
                }

                return Ok(range);
            })
            .collect::<syn::Result<Vec<_>>>()?
            .into_iter()
            .map(ComparableRange::of)
            .vec();

        ranges.sort_by(|l, r| match (l.lower, r.lower) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(left), Some(right)) => left.cmp(&right),
        });

        let mut merged = Vec::<ComparableRange>::with_capacity(ranges.len());
        for mut next in ranges {
            let Some(curr) = merged.last_mut()
            else {
                merged.push(next);
                continue;
            };

            let connected = match (&curr.upper, next.lower) {
                (RangeEnding::Empty, _) => false,
                (RangeEnding::Unbounded, _) | (_, None) => true,
                (RangeEnding::Value(hi), Some(lo)) => {
                    lo <= *hi || hi.successor().is_some_and(|it| it == lo)
                }
            };
            if !connected {
                merged.push(next);
                continue;
            }

            if next.upper.cmp(&curr.upper) == Ordering::Greater {
                curr.range.end = next.range.end.take();
                curr.range.limits = next.range.limits;
                curr.upper = next.upper;
            }
        }

        let fin = merged.into_iter().map(|it| it.range).vec();

        return Ok(fin);
    }

    fn integer_constant(expr: &Expr) -> syn::Result<Int> {
        return match expr {
            Expr::Lit(expr) if expr.attrs.is_empty() => {
                let Lit::Int(literal) = &expr.lit
                else {
                    return expr.fail("range bounds must be integer constants");
                };
                Int::parse_non_negative(
                    literal.base10_digits(),
                    literal.suffix(),
                )
                .map_err(|_| expr.errorful::<()>("invalid number"))
            }

            Expr::Unary(expr)
                if expr.attrs.is_empty()
                    && matches!(&expr.op, UnOp::Neg(_)) =>
            {
                Ok(integer_constant(&expr.expr)?.negate())
            }

            Expr::Path(expr)
                if expr.attrs.is_empty() && expr.qself.is_none() =>
            {
                let Some(ty) = expr.path.segments.first()
                else {
                    return expr.fail("range bounds must be integer constants");
                };

                let Some(bound) = expr.path.segments.last()
                else {
                    return expr.fail("range bounds must be integer constants");
                };

                if expr.path.segments.len() != 2
                    || !matches!(ty.arguments, syn::PathArguments::None)
                    || !matches!(bound.arguments, syn::PathArguments::None)
                {
                    return expr.fail("range bounds must be integer constants");
                }

                let (min, max) = N::of(ty.ident.to_string())
                    .map(|it| it.bounds())
                    .ok_or_else(|| {
                        expr.errorful::<Int>(
                            "range bounds must be integer constants",
                        )
                    })?;

                match bound.ident.to_string().as_str() {
                    "MIN" => Ok(min),
                    "MAX" => Ok(max),
                    _ => expr.fail("range bounds must be integer constants"),
                }
            }

            Expr::Paren(expr) if expr.attrs.is_empty() => {
                integer_constant(&expr.expr)
            }

            _ => expr.fail("range bounds must be integer constants"),
        };
    }

    struct ComparableRange {
        range: ExprRange,
        lower: Option<Int>,
        upper: RangeEnding,
    }

    impl ComparableRange {
        fn of(it: ExprRange) -> Self {
            return Self {
                lower: it.start.as_ref().map(|it| {
                    integer_constant(it)
                        .expect("ranges were validated before merging")
                }),
                upper: match &it.end {
                    None => RangeEnding::Unbounded,
                    Some(end) => {
                        let end = integer_constant(end)
                            .expect("ranges were validated before merging");
                        match &it.limits {
                            RangeLimits::HalfOpen(_) => end
                                .predecessor()
                                .map_or(RangeEnding::Empty, RangeEnding::Value),
                            RangeLimits::Closed(_) => RangeEnding::Value(end),
                        }
                    }
                },
                range: it,
            };
        }
    }
}

pub(crate) fn parse_callbacks(input: ParseStream) -> syn::Result<Vec<Path>> {
    if input.peek(syn::token::Bracket) {
        return Ok(list(input)?.vec());
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
        .vec();

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

pub(crate) fn arg_or_list<T: Parse>(
    stream: ParseStream
) -> syn::Result<Vec<T>> {
    return if stream.peek(syn::token::Paren) {
        Ok(tuple(stream)?.vec())
    }
    else {
        let one = stream.parse::<T>()?;
        Ok(vec![one])
    };
}

pub(crate) fn tuple<T: Parse>(
    stream: ParseStream
) -> syn::Result<impl Iterator<Item = T>> {
    let content;
    let _ = parenthesized!(content in stream);
    let many =
        Punctuated::<T, Token![,]>::parse_terminated(&content)?.into_iter();
    return Ok(many);
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
            Ok(false) => {
                return stream.span().fail("unknown attribute");
            }
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
