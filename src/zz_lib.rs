use std::collections::{
    BTreeMap,
    BTreeSet,
    HashSet,
};
use std::fmt::{
    Debug,
    Formatter,
};
use std::str::FromStr;

use proc_macro2::{
    Ident,
    Span,
    TokenStream,
};
use quote::quote;
pub(crate) use ranges::parse_domain;
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
    Lit,
    Path,
    RangeLimits,
    Token,
    Type,
    UnOp,
    Visibility,
    bracketed,
    parenthesized,
};

use crate::zz_runner::{
    MkErr,
    MySpanned,
    ToCollection,
};
use crate::zz_value_type::{
    Int,
    N,
    RangeEnding,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Konsts {
    Add,
    AddAssign,
    And,
    AndAssign,
    Assign,
    Bit,
    Comparison,
    Conversion,
    CoreInt,
    Div,
    DivAssign,
    Eq,
    Friendship,
    From,
    Into,
    Iterator,
    Map,
    Math,
    Mul,
    MulAssign,
    Not,
    Of,
    Or,
    OrAssign,
    Ord,
    PartialEq,
    PartialOrd,
    Rem,
    RemAssign,
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
    Sub,
    SubAssign,
    Text,
    TextAccess,
    TextFriendship,
    TextFrom,
    TryInto,
    TryMake,
    Value,
    Xor,
    XorAssign,
}

impl Konsts {
    pub(crate) const INTEGRAL: &[Self] = &[
        Self::Add,
        Self::AddAssign,
        Self::And,
        Self::AndAssign,
        Self::Assign,
        Self::Bit,
        Self::Comparison,
        Self::Conversion,
        Self::CoreInt,
        Self::Div,
        Self::DivAssign,
        Self::Eq,
        Self::Friendship,
        Self::Into,
        Self::Iterator,
        Self::Map,
        Self::Math,
        Self::Mul,
        Self::MulAssign,
        Self::Not,
        Self::Of,
        Self::Or,
        Self::OrAssign,
        Self::Ord,
        Self::PartialEq,
        Self::PartialOrd,
        Self::Rem,
        Self::RemAssign,
        Self::Shl,
        Self::ShlAssign,
        Self::Shr,
        Self::ShrAssign,
        Self::Sub,
        Self::SubAssign,
        Self::TryInto,
        Self::TryMake,
        Self::Value,
        Self::Xor,
        Self::XorAssign,
    ];

    pub(crate) fn is_stable(self) -> bool {
        return matches!(
            self,
            Self::CoreInt | Self::TryMake | Self::Value | Self::TextAccess
        );
    }

    pub(crate) fn parse_konst_flags(name: &str) -> Option<Konsts> {
        return Some(match name {
            "konst_add" => Konsts::Add,
            "konst_add_assign" => Konsts::AddAssign,
            "konst_and" => Konsts::And,
            "konst_and_assign" => Konsts::AndAssign,
            "konst_assign" => Konsts::Assign,
            "konst_bit" => Konsts::Bit,
            "konst_comparison" => Konsts::Comparison,
            "konst_conversion" => Konsts::Conversion,
            "konst_core_int" => Konsts::CoreInt,
            "konst_div" => Konsts::Div,
            "konst_div_assign" => Konsts::DivAssign,
            "konst_eq" => Konsts::Eq,
            "konst_friendship" => Konsts::Friendship,
            "konst_into" => Konsts::Into,
            "konst_iterator" => Konsts::Iterator,
            "konst_map" => Konsts::Map,
            "konst_math" => Konsts::Math,
            "konst_mul" => Konsts::Mul,
            "konst_mul_assign" => Konsts::MulAssign,
            "konst_not" => Konsts::Not,
            "konst_of" => Konsts::Of,
            "konst_or" => Konsts::Or,
            "konst_or_assign" => Konsts::OrAssign,
            "konst_ord" => Konsts::Ord,
            "konst_partial_eq" => Konsts::PartialEq,
            "konst_partial_ord" => Konsts::PartialOrd,
            "konst_rem" => Konsts::Rem,
            "konst_rem_assign" => Konsts::RemAssign,
            "konst_shl" => Konsts::Shl,
            "konst_shl_assign" => Konsts::ShlAssign,
            "konst_shr" => Konsts::Shr,
            "konst_shr_assign" => Konsts::ShrAssign,
            "konst_sub" => Konsts::Sub,
            "konst_sub_assign" => Konsts::SubAssign,
            "konst_try_into" => Konsts::TryInto,
            "konst_try_make" => Konsts::TryMake,
            "konst_value" => Konsts::Value,
            "konst_xor" => Konsts::Xor,
            "konst_xor_assign" => Konsts::XorAssign,
            _ => return None,
        });
    }
}

impl FromStr for Konsts {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Self::parse_konst_flags(s).ok_or(());
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Konst {
    None,
    Stable,
    Nightly,
}

impl Default for Konst {
    fn default() -> Self {
        #[cfg(feature = "nightly")]
        return Self::Nightly;

        #[cfg(not(feature = "nightly"))]
        return Self::Stable;
    }
}

impl Parse for Konst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.peek(syn::Ident) {
            return Err(syn::Error::new(
                input.span(),
                "expected `none`, `stable`, or `nightly`",
            ));
        }

        let name = input.parse::<Ident>()?;

        return match name.to_string().as_str() {
            "none" => Ok(Self::None),
            "stable" => Ok(Self::Stable),
            "nightly" => Ok(Self::Nightly),
            _ => Err(syn::Error::new(
                name.span(),
                "expected `none`, `stable`, or `nightly`",
            )),
        };
    }
}

impl Konst {
    pub(crate) fn enables(
        self,
        flags: &[Konsts],
    ) -> bool {
        return match self {
            Self::None => false,
            Self::Stable => flags.iter().any(|flag| flag.is_stable()),
            Self::Nightly => true,
        };
    }

    pub(crate) fn bonst(is_const: bool) -> Option<TokenStream> {
        return match is_const {
            true => Some(quote! { [const] }),
            false => None,
        };
    }

    pub(crate) fn konst(is_const: bool) -> Option<TokenStream> {
        return match is_const {
            true => Some(quote! { const }),
            false => None,
        };
    }

    pub(crate) fn destruct(is_const: bool) -> Option<TokenStream> {
        return match is_const {
            true => Some(quote! { + [const] ::core::marker::Destruct }),
            false => None,
        };
    }
}

#[derive(Clone)]
pub(crate) struct DomainRange {
    range: ExprRange,
    lower: Option<Int>,
    upper: RangeEnding,
}

#[derive(Clone, Default)]
pub(crate) struct IntegralDomain {
    ranges: Vec<DomainRange>,
}

impl IntegralDomain {
    #[cfg(test)]
    pub(crate) fn ranges(&self) -> impl ExactSizeIterator<Item = &ExprRange> {
        return self.ranges.iter().map(|it| &it.range);
    }

    pub(crate) fn contains_zero(&self) -> bool {
        let zero = Int::NonNegative(0);
        return self.ranges.iter().any(|it| {
            let above_lower = it.lower.is_none_or(|lower| lower <= zero);
            let below_upper = match it.upper {
                RangeEnding::Empty => false,
                RangeEnding::Value(upper) => zero <= upper,
                RangeEnding::Unbounded => true,
            };
            return above_lower && below_upper;
        });
    }

    pub(crate) fn excludes_zero(&self) -> bool {
        return !self.contains_zero();
    }

    fn validation_expression(
        &self,
        value: &TokenStream,
    ) -> TokenStream {
        return self
            .ranges
            .iter()
            .map(|it| Self::range_condition(&it.range, value))
            .reduce(|left, right| quote! { (#left) || (#right) })
            .unwrap_or_else(|| quote! { false });
    }

    fn range_condition(
        it: &ExprRange,
        value: &TokenStream,
    ) -> TokenStream {
        let lower = it.start.as_ref().map(|start| {
            return quote! { #value >= (#start) };
        });
        let upper = it.end.as_ref().map(|end| {
            return match &it.limits {
                RangeLimits::HalfOpen(_) => {
                    quote! { #value < (#end) }
                }
                RangeLimits::Closed(_) => {
                    quote! { #value <= (#end) }
                }
            };
        });

        return match (lower, upper) {
            (None, None) => quote! { true },
            (Some(lower), None) => quote! { #lower },
            (None, Some(upper)) => quote! { #upper },
            (Some(lower), Some(upper)) => {
                quote! { (#lower) && (#upper) }
            }
        };
    }
}

#[derive(Default, Clone)]
pub(crate) struct ValidationCfg {
    pub(crate) callbacks: Vec<Path>,
    pub(crate) domain: Option<IntegralDomain>,
    non_zero: bool,
}

impl ValidationCfg {
    pub(crate) fn emit(
        &self,
        value: &TokenStream,
    ) -> TokenStream {
        let callbacks = self
            .callbacks
            .iter()
            .map(|callback| quote! { #callback(#value) })
            .reduce(|left, right| quote! { (#left) && (#right) });

        let domain = self
            .domain
            .as_ref()
            .map(|domain| domain.validation_expression(value));
        let non_zero = self.non_zero.then(|| quote! { #value != 0 });

        return callbacks
            .into_iter()
            .chain(domain)
            .chain(non_zero)
            .reduce(|left, right| quote! { (#left) && (#right) })
            .unwrap_or_else(|| quote! { true });
    }
}

impl Debug for ValidationCfg {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return f
            .debug_struct("ValidationCfg")
            .field("callbacks_count", &self.callbacks.len())
            .field(
                "ranges_count",
                &self.domain.as_ref().map_or(0, |it| it.ranges.len()),
            )
            .field("non_zero", &self.non_zero)
            .finish();
    }
}

impl ValidationCfg {
    pub(crate) fn has_validation(&self) -> bool {
        return !self.callbacks.is_empty()
            || self.domain.is_some()
            || self.non_zero;
    }

    pub(crate) fn domain_excludes_zero(&self) -> bool {
        return self
            .domain
            .as_ref()
            .is_some_and(IntegralDomain::excludes_zero);
    }

    pub(crate) fn require_non_zero(
        &mut self,
        required: bool,
    ) {
        self.non_zero = required;
    }

    pub(crate) fn parse_attr(
        &mut self,
        attr: &str,
        input: ParseStream,
    ) -> syn::Result<bool> {
        match attr {
            "valid" => self.callbacks = parse_callbacks(input)?,
            "domain" => self.domain = Some(parse_domain(input)?),
            _ => return Ok(false),
        };

        return Ok(true);
    }
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
                parse_int_const(it)
                    .expect("ranges were validated before merging")
            }),
            upper: match &it.end {
                None => RangeEnding::Unbounded,
                Some(end) => {
                    let end = parse_int_const(end)
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

mod ranges {
    use std::cmp::Ordering;

    use syn::Expr;
    use syn::parse::ParseStream;

    use crate::zz_lib::{
        ComparableRange,
        DomainRange,
        IntegralDomain,
        one_or_list,
        parse_int_const,
    };
    use crate::zz_runner::{
        MkErr,
        ToCollection,
    };
    use crate::zz_value_type::RangeEnding;

    pub(crate) fn parse_domain(
        input: ParseStream
    ) -> syn::Result<IntegralDomain> {
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
                    parse_int_const(start)?;
                }
                if let Some(end) = &range.end {
                    parse_int_const(end)?;
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

        return Ok(IntegralDomain {
            ranges: merged
                .into_iter()
                .map(|it| DomainRange {
                    range: it.range,
                    lower: it.lower,
                    upper: it.upper,
                })
                .vec(),
        });
    }
}

pub(crate) fn parse_callbacks(input: ParseStream) -> syn::Result<Vec<Path>> {
    if input.peek(syn::token::Bracket) {
        return Ok(list(input)?.vec());
    }

    return Ok(vec![input.parse()?]);
}

fn parse_int_const(expr: &Expr) -> syn::Result<Int> {
    return match expr {
        Expr::Lit(expr) if expr.attrs.is_empty() => {
            let Lit::Int(literal) = &expr.lit
            else {
                return expr.fail("need an integer constant");
            };

            Int::parse_non_negative(literal.base10_digits(), literal.suffix())
                .map_err(|_| expr.errorful("invalid number"))
        }

        Expr::Unary(expr)
            if expr.attrs.is_empty() && matches!(&expr.op, UnOp::Neg(_)) =>
        {
            Ok(parse_int_const(&expr.expr)?.negate())
        }

        Expr::Path(expr) if expr.attrs.is_empty() && expr.qself.is_none() => {
            let Some(ty) = expr.path.segments.first()
            else {
                return expr.fail("need an integer constant");
            };

            let Some(bound) = expr.path.segments.last()
            else {
                return expr.fail("need an integer constant");
            };

            if expr.path.segments.len() != 2
                || !matches!(ty.arguments, syn::PathArguments::None)
                || !matches!(bound.arguments, syn::PathArguments::None)
            {
                return expr.fail("need an integer constant");
            }

            let (min, max) = N::of(ty.ident.to_string())
                .map(|it| it.bounds())
                .ok_or_else(|| expr.errorful("need an integer constant"))?;

            match bound.ident.to_string().as_str() {
                "MIN" => Ok(min),
                "MAX" => Ok(max),
                _ => expr.fail("need an integer constant"),
            }
        }

        Expr::Paren(expr) if expr.attrs.is_empty() => {
            parse_int_const(&expr.expr)
        }

        _ => expr.fail("need an integer constant"),
    };
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
        return attrs[*duplicate].fail("duplicated attribute");
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

#[derive(Clone, Copy)]
pub(crate) enum DuplicateFlagPolicy {
    WithinList,
    AnyList,
}

pub(crate) struct ExplicitFlag {
    pub(crate) name: String,
    pub(crate) value: bool,
    pub(crate) span: Span,
}

impl MySpanned for ExplicitFlag {
    fn my_span(&self) -> Span {
        return self.span;
    }
}

pub(crate) fn parse_explicit_flags(
    input: ParseStream,
    value: bool,
    seen: &mut BTreeMap<String, bool>,
    duplicates: DuplicateFlagPolicy,
    mut on_flag: impl FnMut(&ExplicitFlag) -> syn::Result<()>,
) -> syn::Result<()> {
    let attrs = one_or_list::<Ident>(input)?;
    let mut list_seen = BTreeSet::new();

    for attr in attrs {
        let flag = ExplicitFlag {
            name: attr.to_string(),
            value,
            span: attr.span(),
        };

        if matches!(duplicates, DuplicateFlagPolicy::WithinList)
            && !list_seen.insert(flag.name.clone())
        {
            return flag.fail("duplicated flags");
        }

        if let Some(previous) = seen.get(&flag.name) {
            if *previous != value {
                return flag
                    .fail("flag specified in both `with` and `without`");
            }
            if matches!(duplicates, DuplicateFlagPolicy::AnyList) {
                return flag.fail("duplicated flags");
            }
        }

        on_flag(&flag)?;
        seen.insert(flag.name, flag.value);
    }

    return Ok(());
}

pub(crate) fn find_repr_n(
    attrs: &[Attribute],
    span: &impl Spanned,
) -> syn::Result<Option<N>> {
    let mut repr = None::<N>;

    attrs
        .iter()
        .filter(|it| it.path().is_ident("repr"))
        .try_for_each(|it| {
            it.parse_nested_meta(|it| {
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
            })
        })?;

    return Ok(repr);
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

pub(crate) fn parse_unit_error(input: ParseStream) -> syn::Result<()> {
    let error: Type = input.parse()?;

    if matches!(&error, Type::Tuple(tuple) if tuple.elems.is_empty()) {
        return Ok(());
    }

    return error.fail("only `()` is currently supported for `error`");
}

pub(crate) fn resolve_konst(
    flags: &[Konsts],
    value: impl FnMut(&Konsts) -> Option<bool>,
    fallback: impl FnOnce() -> bool,
) -> bool {
    return flags.iter().find_map(value).unwrap_or_else(fallback);
}
