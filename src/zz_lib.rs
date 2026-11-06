use std::cmp::Ordering;
use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::fmt::{
    Debug,
    Formatter,
};

use proc_macro2::{
    Ident,
    Span,
    TokenStream,
};
use quote::{
    ToTokens,
    quote,
};
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Attribute,
    Path,
    Token,
    Type,
    Visibility,
    braced,
    bracketed,
    parenthesized,
};

use crate::zz_domain::{
    Domain,
    Tri,
};
use crate::zz_domain_float::{
    MathProvider,
    ResolvedFloatDomain,
};
use crate::zz_domain_int::ResolvedDomain;
use crate::zz_names::{
    NameKey,
    ResolvedNames,
};
use crate::zz_runner::{
    MkErr,
    ToCollection,
};
use crate::zz_values::{
    FnName,
    N,
    Num,
};

#[derive(Clone)]
pub(crate) struct PathIdentity {
    path: Path,
    identity: String,
}

impl PathIdentity {
    pub(crate) fn new(path: Path) -> Self {
        return Self {
            identity: path.to_token_stream().to_string(),
            path,
        };
    }

    pub(crate) fn path(&self) -> &Path {
        return &self.path;
    }

    pub(crate) fn identity(&self) -> &str {
        return &self.identity;
    }
}

impl Eq for PathIdentity {}

impl PartialEq for PathIdentity {
    fn eq(
        &self,
        that: &Self,
    ) -> bool {
        return self.identity == that.identity;
    }
}

impl PartialOrd for PathIdentity {
    fn partial_cmp(
        &self,
        that: &Self,
    ) -> Option<Ordering> {
        return Some(self.cmp(that));
    }
}

impl Ord for PathIdentity {
    fn cmp(
        &self,
        that: &Self,
    ) -> Ordering {
        return self.identity.cmp(&that.identity);
    }
}

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
    CoreFloat,
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
    Neg,
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
        Self::CoreFloat,
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
        Self::Neg,
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

    pub(crate) const TEXT: &[Self] = &[
        Self::Friendship,
        Self::From,
        Self::Text,
        Self::TextAccess,
        Self::TextFriendship,
        Self::TextFrom,
    ];

    pub(crate) fn is_stable(self) -> bool {
        return matches!(
            self,
            Self::CoreInt
                | Self::CoreFloat
                | Self::TryMake
                | Self::Value
                | Self::TextAccess
        );
    }

    /// `name`'s `konst_*` flag, if `family` (`INTEGRAL`, `TEXT`) has it.
    pub(crate) fn of_in(
        name: &str,
        family: &[Self],
    ) -> Option<Self> {
        return Self::parse_konst_flags(name).filter(|it| family.contains(it));
    }

    fn parse_konst_flags(name: &str) -> Option<Konsts> {
        return Some(match name {
            "konst_add" => Konsts::Add,
            "konst_add_assign" => Konsts::AddAssign,
            "konst_and" => Konsts::And,
            "konst_and_assign" => Konsts::AndAssign,
            "konst_assign" => Konsts::Assign,
            "konst_bit" => Konsts::Bit,
            "konst_comparison" => Konsts::Comparison,
            "konst_conversion" => Konsts::Conversion,
            "konst_core_float" => Konsts::CoreFloat,
            "konst_core_int" => Konsts::CoreInt,
            "konst_div" => Konsts::Div,
            "konst_div_assign" => Konsts::DivAssign,
            "konst_eq" => Konsts::Eq,
            "konst_friendship" => Konsts::Friendship,
            "konst_from" => Konsts::From,
            "konst_into" => Konsts::Into,
            "konst_iterator" => Konsts::Iterator,
            "konst_map" => Konsts::Map,
            "konst_math" => Konsts::Math,
            "konst_mul" => Konsts::Mul,
            "konst_mul_assign" => Konsts::MulAssign,
            "konst_neg" => Konsts::Neg,
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
            "konst_text" => Konsts::Text,
            "konst_text_access" => Konsts::TextAccess,
            "konst_text_friendship" => Konsts::TextFriendship,
            "konst_text_from" => Konsts::TextFrom,
            "konst_try_into" => Konsts::TryInto,
            "konst_try_make" => Konsts::TryMake,
            "konst_value" => Konsts::Value,
            "konst_xor" => Konsts::Xor,
            "konst_xor_assign" => Konsts::XorAssign,
            _ => return None,
        });
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

    /// `[const] #bound + [const] Destruct` when `is_const`, else `#bound`.
    pub(crate) fn bound(
        is_const: bool,
        bound: TokenStream,
    ) -> TokenStream {
        let bonst = Self::bonst(is_const);
        let destruct = Self::destruct(is_const);
        return quote! { #bonst #bound #destruct };
    }

    /// `#[derive_const(traits)]` under `konst = nightly`, else
    /// `#[derive(traits)]`.
    pub(crate) fn derive(
        self,
        traits: TokenStream,
    ) -> TokenStream {
        return match self {
            Self::Nightly => quote! { #[derive_const(#traits)] },
            Self::None | Self::Stable => quote! { #[derive(#traits)] },
        };
    }
}

/// `#[repr(transparent)]` when `enabled`.
pub(crate) fn repr_transparent(enabled: bool) -> Option<TokenStream> {
    return enabled.then(|| quote! { #[repr(transparent)] });
}

/// A lint a family allows on the `const _` scope of its generated code.
#[derive(Clone, Copy)]
pub(crate) enum ScopeLint {
    DeadCode,
    UnusedQualifications,
    UnnecessaryCast,
    /// Domain checks compare bounds explicitly: `RangeInclusive::contains`
    /// is not `const`.
    ManualRangeContains,
}

impl ScopeLint {
    fn path(self) -> TokenStream {
        return match self {
            Self::DeadCode => quote! { dead_code },
            Self::UnusedQualifications => quote! { unused_qualifications },
            Self::UnnecessaryCast => quote! { clippy::unnecessary_cast },
            Self::ManualRangeContains => {
                quote! { clippy::manual_range_contains }
            }
        };
    }
}

/// The scope lints of the numeric families (integral, float, bitflag).
pub(crate) const NUMERIC_LINTS: &[ScopeLint] = &[
    ScopeLint::DeadCode,
    ScopeLint::UnusedQualifications,
    ScopeLint::UnnecessaryCast,
    ScopeLint::ManualRangeContains,
];

/// The scope lints of text.
pub(crate) const TEXT_LINTS: &[ScopeLint] = &[
    ScopeLint::DeadCode,
    ScopeLint::UnusedQualifications,
];

/// `const _: () = { items };` with one `#[allow(..)]` per lint, in order.
pub(crate) fn lint_scope(
    lints: &[ScopeLint],
    items: TokenStream,
) -> TokenStream {
    let lints = lints.iter().map(|it| it.path());
    return quote! {
        #(#[allow(#lints)])*
        const _: () = {
            #items
        };
    };
}

/// `return if #condition { Ok(#ok) } else { Err(#err) };`, fully qualified.
pub(crate) fn validated_result(
    condition: &TokenStream,
    ok: TokenStream,
    err: TokenStream,
) -> TokenStream {
    return quote! {
        return if #condition {
            ::core::result::Result::Ok(#ok)
        }
        else {
            ::core::result::Result::Err(#err)
        };
    };
}

pub(crate) fn callback_condition(
    callbacks: &[Path],
    value: &TokenStream,
) -> Option<TokenStream> {
    return callbacks
        .iter()
        .map(|callback| quote! { #callback(#value) })
        .reduce(|left, right| quote! { (#left) && (#right) });
}

/// A domain resolved for its value type.
#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Resolved {
    Int(ResolvedDomain),
    Float(ResolvedFloatDomain),
}

impl Resolved {
    /// The domain's membership condition on `value`.
    fn emit(
        &self,
        value: &TokenStream,
    ) -> TokenStream {
        return match self {
            Self::Int(it) => it.emit(value),
            Self::Float(it) => it.emit(value),
        };
    }

    /// The domain is symmetric, so negation stays inside it.
    fn neg_closed(&self) -> bool {
        return match self {
            Self::Int(it) => it.neg_closed(),
            Self::Float(it) => it.neg_closed(),
        };
    }

    /// `abs` of a member is a member.
    fn abs_closed(&self) -> bool {
        return match self {
            Self::Int(it) => it.abs_closed(),
            Self::Float(it) => it.abs_closed(),
        };
    }
}

#[derive(Default, Clone)]
pub(crate) struct ValidationCfg {
    pub(crate) callbacks: Vec<Path>,
    pub(crate) domain: Option<Domain>,
    resolved: Option<Resolved>,
    non_zero: bool,
}

impl ValidationCfg {
    /// Domain first, then callbacks in declaration order (D12): callbacks only
    /// see values the domain admits.
    pub(crate) fn emit(
        &self,
        value: &TokenStream,
    ) -> TokenStream {
        let domain = self.resolved.as_ref().map(|it| it.emit(value));
        assert!(
            domain.is_some() || (self.domain.is_none() && !self.non_zero),
            "validation is emitted after the domain is resolved"
        );
        let callbacks = callback_condition(&self.callbacks, value);

        return domain
            .into_iter()
            .chain(callbacks)
            .reduce(|left, right| quote! { (#left) && (#right) })
            .unwrap_or_else(|| quote! { true });
    }

    /// Types the domain (and `non_zero`) for the value type. Called where the
    /// type is first known; repeated calls re-resolve.
    pub(crate) fn resolve(
        &mut self,
        subject: Num,
        math: MathProvider,
    ) -> syn::Result<()> {
        self.resolved = match (subject, &self.domain, self.non_zero) {
            (Num::Int(subject), Some(domain), non_zero) => {
                Some(Resolved::Int(domain.resolve(subject, non_zero)?))
            }
            (Num::Int(subject), None, true) => {
                Some(Resolved::Int(Domain::non_zero_only(subject)))
            }
            (Num::Float(subject), Some(domain), _) => {
                Some(Resolved::Float(domain.resolve_float(subject, math)?))
            }
            (_, None, _) => None,
        };
        return Ok(());
    }

    /// The resolved integer domain, if any.
    pub(crate) fn resolved(&self) -> Option<&ResolvedDomain> {
        return match self.resolved.as_ref() {
            Some(Resolved::Int(it)) => Some(it),
            _ => None,
        };
    }

    /// The resolved float domain, if any.
    pub(crate) fn float_domain(&self) -> Option<&ResolvedFloatDomain> {
        return match self.resolved.as_ref() {
            Some(Resolved::Float(it)) => Some(it),
            _ => None,
        };
    }

    /// Some NaN passes the domain (exact before the float width is known).
    /// No domain admits every value, NaN included.
    pub(crate) fn admits_nan(&self) -> bool {
        return self.domain.as_ref().is_none_or(Domain::admits_nan_untyped);
    }

    /// Negating a valid value always gives a valid value, so `Neg` needs no
    /// check. Callbacks are opaque: any callback rules this out.
    pub(crate) fn neg_closed(&self) -> bool {
        return self.callbacks.is_empty()
            && self.resolved.as_ref().is_some_and(Resolved::neg_closed);
    }

    /// `abs` of a valid value is always valid, so `abs` needs no check.
    pub(crate) fn abs_closed(&self) -> bool {
        return self.callbacks.is_empty()
            && self.resolved.as_ref().is_some_and(Resolved::abs_closed);
    }

    /// The domain as written (with `with = non_zero` folded in), for
    /// generated documentation.
    pub(crate) fn domain_text(&self) -> String {
        let text = self.domain.as_ref().map(|it| it.text().to_string());
        return match (text, self.non_zero) {
            (Some(text), false) => text,
            (Some(text), true) => format!("({text}) & !{{0}}"),
            (None, true) => "!{0}".to_string(),
            (None, false) => String::new(),
        };
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
            .field("domain", &self.domain.is_some())
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

    /// Zero is rejected at every pointer width: exact once resolved; before
    /// that, only when the untyped probe is definite.
    pub(crate) fn domain_excludes_zero(&self) -> bool {
        match &self.resolved {
            Some(Resolved::Int(it)) => return it.excludes_zero(),
            Some(Resolved::Float(_)) => return false,
            None => {}
        }
        return self
            .domain
            .as_ref()
            .is_some_and(|it| it.admits_zero_untyped() == Tri::False);
    }

    pub(crate) fn require_non_zero(
        &mut self,
        required: bool,
    ) {
        self.non_zero = required;
    }

    pub(crate) fn parse_attr(
        &mut self,
        attr: Key,
        input: ParseStream,
    ) -> syn::Result<bool> {
        match attr {
            Key::Valid => self.callbacks = parse_callbacks(input)?,
            Key::Domain => self.domain = Some(input.parse()?),
            _ => return Ok(false),
        };

        return Ok(true);
    }
}

pub(crate) fn parse_callbacks(input: ParseStream) -> syn::Result<Vec<Path>> {
    return Ok(one_or_list::<Path>(input)?.collect());
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

/// `{a, b}`: a braced, comma-separated set.
pub(crate) fn set<T: Parse>(
    stream: ParseStream
) -> syn::Result<impl Iterator<Item = T>> {
    let content;
    let _ = braced!(content in stream);

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

/// A generated function forwarding to a `core` function (text: a `String`
/// method). `without = fn_{name}` removes it, `name` keying both its removal
/// and its generated name.
pub(crate) struct Forward {
    name: FnName,
    tokens: TokenStream,
}

impl Forward {
    /// `emit` receives the function's identifier from `names`.
    pub(crate) fn new<K: NameKey + From<FnName>>(
        name: FnName,
        names: &ResolvedNames<K>,
        emit: impl FnOnce(&Ident) -> TokenStream,
    ) -> Self {
        let tokens = emit(&names.get(K::from(name)));
        return Self { name, tokens };
    }
}

/// `without = fn_{name}`: the generated functions to remove, with the span
/// of each flag.
#[derive(Clone, Default)]
pub(crate) struct Removals(BTreeMap<FnName, Span>);

impl Removals {
    /// Records the removal of `function`, written at `span`.
    pub(crate) fn insert(
        &mut self,
        function: FnName,
        span: Span,
    ) {
        self.0.insert(function, span);
    }

    pub(crate) fn extend(
        &mut self,
        that: &Self,
    ) {
        self.0
            .extend(that.0.iter().map(|(name, span)| (*name, *span)));
    }

    pub(crate) fn removes(
        &self,
        name: FnName,
    ) -> bool {
        return self.0.contains_key(&name);
    }

    pub(crate) fn first_span(&self) -> Option<Span> {
        return self.0.values().next().copied();
    }

    /// The forwards not removed. A removal naming none of `forwards` is an
    /// error: the configuration does not generate that function.
    pub(crate) fn keep(
        &self,
        forwards: Vec<Forward>,
    ) -> syn::Result<TokenStream> {
        let mut removed = BTreeSet::new();
        let mut kept = TokenStream::new();
        for forward in forwards {
            if self.removes(forward.name) {
                removed.insert(forward.name);
                continue;
            }
            kept.extend(forward.tokens);
        }

        if let Some((name, span)) =
            self.0.iter().find(|(name, _)| !removed.contains(*name))
        {
            let name = name.name();
            return span.fail(format!(
                "`fn_{name}` removes nothing: no generated function `{name}`"
            ));
        }

        return Ok(kept);
    }
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

/// A configuration argument key, `key = ...`, as read by [`parse_inner`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Key {
    Conversion,
    Domain,
    Error,
    ExtraCaps,
    Facet,
    Friend,
    GetRaw,
    HideIn,
    Integral,
    Konst,
    LitName,
    LitPath,
    Maker,
    Math,
    Mod,
    PanicAt,
    Precedes,
    Produce,
    Profile,
    Prove,
    Relation,
    Relationship,
    Repr,
    Scope,
    Seal,
    Std,
    Suffix,
    Tree,
    Valid,
    Value,
    ValueName,
    ValuesName,
    With,
    Without,
}

impl Key {
    const ITEMS: [Self; 34] = [
        Self::Conversion,
        Self::Domain,
        Self::Error,
        Self::ExtraCaps,
        Self::Facet,
        Self::Friend,
        Self::GetRaw,
        Self::HideIn,
        Self::Integral,
        Self::Konst,
        Self::LitName,
        Self::LitPath,
        Self::Maker,
        Self::Math,
        Self::Mod,
        Self::PanicAt,
        Self::Precedes,
        Self::Produce,
        Self::Profile,
        Self::Prove,
        Self::Relation,
        Self::Relationship,
        Self::Repr,
        Self::Scope,
        Self::Seal,
        Self::Std,
        Self::Suffix,
        Self::Tree,
        Self::Valid,
        Self::Value,
        Self::ValueName,
        Self::ValuesName,
        Self::With,
        Self::Without,
    ];

    pub(crate) fn name(self) -> &'static str {
        return match self {
            Self::Conversion => "conversion",
            Self::Domain => "domain",
            Self::Error => "error",
            Self::ExtraCaps => "extra_caps",
            Self::Facet => "facet",
            Self::Friend => "friend",
            Self::GetRaw => "get_raw",
            Self::HideIn => "hide_in",
            Self::Integral => "integral",
            Self::Konst => "konst",
            Self::LitName => "lit_name",
            Self::LitPath => "lit_path",
            Self::Maker => "maker",
            Self::Math => "math",
            Self::Mod => "mod",
            Self::PanicAt => "panic_at",
            Self::Precedes => "precedes",
            Self::Produce => "produce",
            Self::Profile => "profile",
            Self::Prove => "prove",
            Self::Relation => "relation",
            Self::Relationship => "relationship",
            Self::Repr => "repr",
            Self::Scope => "scope",
            Self::Seal => "seal",
            Self::Std => "std",
            Self::Suffix => "suffix",
            Self::Tree => "tree",
            Self::Valid => "valid",
            Self::Value => "value",
            Self::ValueName => "value_name",
            Self::ValuesName => "values_name",
            Self::With => "with",
            Self::Without => "without",
        };
    }

    pub(crate) fn of(name: &str) -> Option<Self> {
        return Self::ITEMS.into_iter().find(|it| it.name() == name);
    }
}

/// Parses `key = value, ...`. A name outside [`Key`], or a key `on_attr`
/// rejects, is an unknown attribute at the value span; a repeated key is a
/// duplicated arg at the key span.
///
/// `on_attr` parses the value after `=` and returns `Ok(true)`, or returns
/// `Ok(false)` to reject the key; it must reject before consuming any input,
/// so the error lands on the value span. `FacetCfg` records `precedes_span`
/// before delegating, and `Profile::parse_attr` returns `false` for
/// `precedes` on a non-circular profile without consuming.
pub(crate) fn parse_inner(
    stream: ParseStream,
    mut on_attr: impl FnMut(Key, ParseStream) -> syn::Result<bool>,
) -> syn::Result<()> {
    let mut seen = BTreeSet::new();

    while !stream.is_empty() {
        let here = stream.span();
        let key = parse_key(stream)?;
        if key.is_some_and(|key| !seen.insert(key)) {
            return here.fail("duplicated arg");
        }

        stream.parse::<Token![=]>()?;

        if !key.map_or(Ok(false), |key| on_attr(key, stream))? {
            return stream.span().fail("unknown attribute");
        }

        if !stream.is_empty() {
            stream.parse::<Token![,]>()?;
        }
    }
    return Ok(());
}

/// [`Key::of`] an identifier or `mod`; `None` for `_`, `in` and other names.
/// `mod` is a keyword `Ident` rejects; other keywords stay "expected
/// identifier".
fn parse_key(stream: ParseStream) -> syn::Result<Option<Key>> {
    if stream.peek(Token![_]) {
        let _: Token![_] = stream.parse()?;
        return Ok(None);
    }
    if stream.peek(Token![mod]) {
        let _: Token![mod] = stream.parse()?;
        return Ok(Some(Key::Mod));
    }
    if stream.peek(Token![in]) {
        let _: Token![in] = stream.parse()?;
        return Ok(None);
    }
    let key: Ident = stream.parse()?;
    return Ok(Key::of(&key.to_string()));
}

pub(crate) fn parse_unit_error(input: ParseStream) -> syn::Result<()> {
    let error: Type = input.parse()?;

    if matches!(&error, Type::Tuple(tuple) if tuple.elems.is_empty()) {
        return Ok(());
    }

    return error.fail("only `()` is currently supported for `error`");
}
