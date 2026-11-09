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
    Int,
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

/// A family of names that logic compares, matches, or looks up: an enum
/// whose text lives in one row table. `of` is the parse boundary.
pub(crate) trait Named: Copy + 'static {
    /// Every variant with its text, in declaration order:
    /// `TABLE[i].0 as usize == i`, which each enum asserts at compile time.
    const TABLE: &'static [(Self, &'static str)];

    /// `self as usize`: the position in `TABLE`.
    fn index(self) -> usize;

    fn name(self) -> &'static str {
        return Self::TABLE[self.index()].1;
    }

    fn of(name: &str) -> Option<Self> {
        return Self::TABLE
            .iter()
            .find(|(_, text)| *text == name)
            .map(|(it, _)| *it);
    }

    /// Every variant, in `TABLE` order.
    fn items() -> impl Iterator<Item = Self> {
        return Self::TABLE.iter().map(|(it, _)| *it);
    }
}

/// "expected `a`, `b`, or `c`": the names of `T`, in `TABLE` order.
pub(crate) fn expected_choice<T: Named>() -> String {
    let names = T::TABLE
        .iter()
        .map(|(_, name)| format!("`{name}`"))
        .collect::<Vec<_>>();
    return match names.as_slice() {
        [] => String::from("expected nothing"),
        [only] => format!("expected {only}"),
        [
            first,
            second,
        ] => format!("expected {first} or {second}"),
        [
            rest @ ..,
            last,
        ] => {
            format!("expected {}, or {last}", rest.join(", "))
        }
    };
}

/// One name of `T`. Any other name is "expected `a`, `b`, or `c`" at the
/// name; a token that is no identifier is syn's "expected identifier".
pub(crate) fn parse_choice<T: Named>(input: ParseStream) -> syn::Result<T> {
    let name = input.parse::<Ident>()?;
    return match T::of(&name.to_string()) {
        Some(it) => Ok(it),
        None => name.span().fail(expected_choice::<T>()),
    };
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

/// The families whose `with`/`without` take `konst_*` flags.
#[derive(Clone, Copy)]
pub(crate) enum KonstFamily {
    Integral,
    Text,
}

impl KonstFamily {
    /// The family's `konst_*` flags, in `Konsts` order.
    pub(crate) fn konsts(self) -> impl Iterator<Item = Konsts> {
        return Konsts::ROWS
            .iter()
            .filter(move |row| row.is_in(self))
            .map(|row| row.konst);
    }
}

/// One `konst_*` flag: its name after `konst_`, whether its items can be
/// `const` on stable Rust, and the families that take it.
struct KonstRow {
    konst: Konsts,
    name: &'static str,
    stable: bool,
    integral: bool,
    text: bool,
}

impl KonstRow {
    /// A nightly-only flag no family takes yet.
    const fn of(
        konst: Konsts,
        name: &'static str,
    ) -> Self {
        return Self {
            konst,
            name,
            stable: false,
            integral: false,
            text: false,
        };
    }

    const fn stable(mut self) -> Self {
        self.stable = true;
        return self;
    }

    const fn integral(mut self) -> Self {
        self.integral = true;
        return self;
    }

    const fn text(mut self) -> Self {
        self.text = true;
        return self;
    }

    fn is_in(
        &self,
        family: KonstFamily,
    ) -> bool {
        return match family {
            KonstFamily::Integral => self.integral,
            KonstFamily::Text => self.text,
        };
    }
}

impl Konsts {
    /// The flag prefix: `with = konst_{name}`.
    const PREFIX: &str = "konst_";

    /// Every variant's row, in declaration order:
    /// `ROWS[i].konst as usize == i`.
    const ROWS: &[KonstRow] = &[
        KonstRow::of(Self::Add, "add").integral(),
        KonstRow::of(Self::AddAssign, "add_assign").integral(),
        KonstRow::of(Self::And, "and").integral(),
        KonstRow::of(Self::AndAssign, "and_assign").integral(),
        KonstRow::of(Self::Assign, "assign").integral(),
        KonstRow::of(Self::Bit, "bit").integral(),
        KonstRow::of(Self::Comparison, "comparison").integral(),
        KonstRow::of(Self::Conversion, "conversion").integral(),
        KonstRow::of(Self::CoreFloat, "core_float")
            .integral()
            .stable(),
        KonstRow::of(Self::CoreInt, "core_int").integral().stable(),
        KonstRow::of(Self::Div, "div").integral(),
        KonstRow::of(Self::DivAssign, "div_assign").integral(),
        KonstRow::of(Self::Eq, "eq").integral(),
        KonstRow::of(Self::Friendship, "friendship")
            .integral()
            .text(),
        KonstRow::of(Self::From, "from").text(),
        KonstRow::of(Self::Into, "into").integral(),
        KonstRow::of(Self::Iterator, "iterator").integral(),
        KonstRow::of(Self::Map, "map").integral(),
        KonstRow::of(Self::Math, "math").integral(),
        KonstRow::of(Self::Mul, "mul").integral(),
        KonstRow::of(Self::MulAssign, "mul_assign").integral(),
        KonstRow::of(Self::Neg, "neg").integral(),
        KonstRow::of(Self::Not, "not").integral(),
        KonstRow::of(Self::Of, "of").integral(),
        KonstRow::of(Self::Or, "or").integral(),
        KonstRow::of(Self::OrAssign, "or_assign").integral(),
        KonstRow::of(Self::Ord, "ord").integral(),
        KonstRow::of(Self::PartialEq, "partial_eq").integral(),
        KonstRow::of(Self::PartialOrd, "partial_ord").integral(),
        KonstRow::of(Self::Rem, "rem").integral(),
        KonstRow::of(Self::RemAssign, "rem_assign").integral(),
        KonstRow::of(Self::Shl, "shl").integral(),
        KonstRow::of(Self::ShlAssign, "shl_assign").integral(),
        KonstRow::of(Self::Shr, "shr").integral(),
        KonstRow::of(Self::ShrAssign, "shr_assign").integral(),
        KonstRow::of(Self::Sub, "sub").integral(),
        KonstRow::of(Self::SubAssign, "sub_assign").integral(),
        KonstRow::of(Self::Text, "text").text(),
        KonstRow::of(Self::TextAccess, "text_access")
            .text()
            .stable(),
        KonstRow::of(Self::TextFriendship, "text_friendship").text(),
        KonstRow::of(Self::TextFrom, "text_from").text(),
        KonstRow::of(Self::TryInto, "try_into").integral(),
        KonstRow::of(Self::TryMake, "try_make").integral().stable(),
        KonstRow::of(Self::Value, "value").integral().stable(),
        KonstRow::of(Self::Xor, "xor").integral(),
        KonstRow::of(Self::XorAssign, "xor_assign").integral(),
    ];

    fn row(self) -> &'static KonstRow {
        return &Self::ROWS[self as usize];
    }

    pub(crate) fn is_stable(self) -> bool {
        return self.row().stable;
    }

    /// The flag `konst_{name}`, of any family.
    fn of(name: &str) -> Option<Self> {
        let name = name.strip_prefix(Self::PREFIX)?;
        return Self::ROWS
            .iter()
            .find(|row| row.name == name)
            .map(|row| row.konst);
    }

    /// `name`'s `konst_*` flag, if `family` takes it.
    pub(crate) fn of_in(
        name: &str,
        family: KonstFamily,
    ) -> Option<Self> {
        return Self::of(name).filter(|it| it.row().is_in(family));
    }
}

// Every row of `ROWS` is at its variant's index.
const _: () = {
    let rows = Konsts::ROWS;
    let mut index = 0;
    while index < rows.len() {
        assert!(rows[index].konst as usize == index);
        index += 1;
    }
};

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

impl Named for Konst {
    const TABLE: &'static [(Self, &'static str)] = &[
        (Self::None, "none"),
        (Self::Stable, "stable"),
        (Self::Nightly, "nightly"),
    ];

    fn index(self) -> usize {
        return self as usize;
    }
}

// Every row of `TABLE` is at its variant's index.
const _: () = {
    let table = Konst::TABLE;
    let mut index = 0;
    while index < table.len() {
        assert!(table[index].0 as usize == index);
        index += 1;
    }
};

impl Parse for Konst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.peek(syn::Ident) {
            return input.span().fail(expected_choice::<Self>());
        }

        return parse_choice(input);
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

/// `expr`'s `Ok` value; its `Err` panics with `message`, the arguments of
/// `panic!`.
pub(crate) fn unwrap_or_panic(
    expr: TokenStream,
    message: impl ToTokens,
) -> TokenStream {
    return quote! {
        match #expr {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err(_) => {
                ::core::panic!(#message)
            }
        }
    };
}

/// When a constructor evaluates its validation.
#[derive(Clone, Copy)]
pub(crate) enum Check {
    Validate,
    /// In a `debug_assert!` with this message: a proof shows the value
    /// valid, so release builds construct unchecked.
    DebugOnly(&'static str),
}

/// What a constructor returns for an invalid value; that also fixes how it
/// returns the valid one (`Ok`, `Some`, or the bare value).
pub(crate) enum OnInvalid {
    Err(TokenStream),
    None,
    /// A panic with this message.
    Panic(&'static str),
}

/// The validation of one raw value, an ordered list of conjuncts: the
/// domain's condition, then each callback's call. As tokens it is their
/// short-circuit conjunction (`true` without conjuncts), the callbacks
/// grouped as one operand beside the domain.
pub(crate) struct Validation {
    domain: Option<TokenStream>,
    callbacks: Vec<TokenStream>,
}

impl Validation {
    pub(crate) fn new(
        domain: Option<TokenStream>,
        callbacks: &[Path],
        value: &TokenStream,
    ) -> Self {
        let callbacks = callbacks.iter().map(|it| quote! { #it(#value) });
        return Self {
            domain,
            callbacks: callbacks.collect(),
        };
    }

    /// The statements ending a constructor's body: check the raw value,
    /// return `constructed` (built from it), else fail as `on_invalid`
    /// says. `Check::DebugOnly` only asserts, and only when there is
    /// something to check.
    pub(crate) fn construct(
        &self,
        check: Check,
        on_invalid: OnInvalid,
        constructed: TokenStream,
    ) -> TokenStream {
        let (valid, invalid) = match on_invalid {
            OnInvalid::Err(error) => (
                quote! { ::core::result::Result::Ok(#constructed) },
                quote! { ::core::result::Result::Err(#error) },
            ),
            OnInvalid::None => (
                quote! { ::core::option::Option::Some(#constructed) },
                quote! { ::core::option::Option::None },
            ),
            OnInvalid::Panic(message) => {
                (constructed, quote! { ::core::panic!(#message) })
            }
        };
        return match check {
            Check::Validate => quote! {
                return if #self {
                    #valid
                }
                else {
                    #invalid
                };
            },
            Check::DebugOnly(message) => {
                let checked =
                    self.domain.is_some() || !self.callbacks.is_empty();
                let assertion = checked.then(|| {
                    return quote! {
                        ::core::debug_assert!(#self, #message,);
                    };
                });
                quote! {
                    #assertion
                    return #valid;
                }
            }
        };
    }
}

impl ToTokens for Validation {
    fn to_tokens(
        &self,
        tokens: &mut TokenStream,
    ) {
        let both = |left, right| quote! { (#left) && (#right) };
        let callbacks = self.callbacks.iter().cloned().reduce(both);
        let condition = self.domain.iter().cloned().chain(callbacks);
        tokens
            .extend(condition.reduce(both).unwrap_or_else(|| quote! { true }));
    }
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
    /// The validation of `value`: the domain first, then callbacks in
    /// declaration order (D12), so callbacks only see values the domain
    /// admits.
    pub(crate) fn emit(
        &self,
        value: &TokenStream,
    ) -> Validation {
        let domain = self.resolved.as_ref().map(|it| it.emit(value));
        assert!(
            domain.is_some() || (self.domain.is_none() && !self.non_zero),
            "validation is emitted after the domain is resolved"
        );
        return Validation::new(domain, &self.callbacks, value);
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
            Some(Resolved::Int(it)) => {
                return it.excludes(Int::NonNegative(0));
            }
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

/// Fails with `message` at the generic parameters, else at the `where`
/// clause, of an item that may have neither.
pub(crate) fn reject_generics(
    generics: &syn::Generics,
    message: &str,
) -> syn::Result<()> {
    if !generics.params.is_empty() {
        return generics.params.fail(message);
    }
    if let Some(clause) = &generics.where_clause {
        return clause.fail(message);
    }
    return Ok(());
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

/// The form of a generated function beside the method it forwards to.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Form {
    /// `{name}`: the method's own name.
    Regular,
    /// `try_{name}`: the method's effect, validated into a `Result`.
    Try,
}

impl Form {
    /// What the form puts before the method's name in the generated
    /// function's name, and so in `without = fn_{prefix}{name}`.
    pub(crate) fn prefix(self) -> &'static str {
        return match self {
            Self::Regular => "",
            Self::Try => "try_",
        };
    }
}

/// A generated function forwarding to a `core` function (text: a `String`
/// method). `without = fn_{name}` removes it, `form` and `name` keying both
/// its removal and its generated name.
pub(crate) struct Forward {
    form: Form,
    name: FnName,
    tokens: TokenStream,
}

impl Forward {
    /// The regular form of `name`. `emit` receives the function's
    /// identifier from `names`.
    pub(crate) fn new<K: NameKey + From<FnName>>(
        name: FnName,
        names: &ResolvedNames<K>,
        emit: impl FnOnce(&Ident) -> TokenStream,
    ) -> Self {
        let tokens = emit(&names.get(K::from(name)));
        return Self {
            form: Form::Regular,
            name,
            tokens,
        };
    }

    /// The `try_` form of `name`. `emit` receives the function's
    /// identifier (`try_{name}`) from `names`.
    pub(crate) fn new_try<K: NameKey + From<(Form, FnName)>>(
        name: FnName,
        names: &ResolvedNames<K>,
        emit: impl FnOnce(&Ident) -> TokenStream,
    ) -> Self {
        let form = Form::Try;
        let tokens = emit(&names.get(K::from((form, name))));
        return Self { form, name, tokens };
    }
}

/// `without = fn_{name}`: the generated functions to remove, each keyed by
/// its form and the method it forwards to, with the span of each flag.
#[derive(Clone, Default)]
pub(crate) struct Removals(BTreeMap<(Form, FnName), Span>);

impl Removals {
    /// Records the removal of the `form` of `function`, written at `span`.
    pub(crate) fn insert(
        &mut self,
        form: Form,
        function: FnName,
        span: Span,
    ) {
        self.0.insert((form, function), span);
    }

    pub(crate) fn extend(
        &mut self,
        that: &Self,
    ) {
        self.0
            .extend(that.0.iter().map(|(key, span)| (*key, *span)));
    }

    /// Whether the regular form of `name` is removed.
    pub(crate) fn removes(
        &self,
        name: FnName,
    ) -> bool {
        return self.removes_form(Form::Regular, name);
    }

    pub(crate) fn removes_form(
        &self,
        form: Form,
        name: FnName,
    ) -> bool {
        return self.0.contains_key(&(form, name));
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
            let key = (forward.form, forward.name);
            if self.0.contains_key(&key) {
                removed.insert(key);
                continue;
            }
            kept.extend(forward.tokens);
        }

        if let Some(((form, name), span)) =
            self.0.iter().find(|(key, _)| !removed.contains(*key))
        {
            let prefix = form.prefix();
            let name = name.name();
            return span.fail(format!(
                "`fn_{prefix}{name}` removes nothing: no generated function `{prefix}{name}`"
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
                    if it.path.is_ident(n.name()) {
                        if repr.is_some() {
                            return span.fail("multiple repr matched");
                        }
                        else {
                            repr = Some(n);
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

impl Named for Key {
    const TABLE: &'static [(Self, &'static str)] = &[
        (Self::Conversion, "conversion"),
        (Self::Domain, "domain"),
        (Self::Error, "error"),
        (Self::ExtraCaps, "extra_caps"),
        (Self::Facet, "facet"),
        (Self::Friend, "friend"),
        (Self::GetRaw, "get_raw"),
        (Self::HideIn, "hide_in"),
        (Self::Integral, "integral"),
        (Self::Konst, "konst"),
        (Self::LitName, "lit_name"),
        (Self::LitPath, "lit_path"),
        (Self::Maker, "maker"),
        (Self::Math, "math"),
        (Self::Mod, "mod"),
        (Self::PanicAt, "panic_at"),
        (Self::Precedes, "precedes"),
        (Self::Produce, "produce"),
        (Self::Profile, "profile"),
        (Self::Prove, "prove"),
        (Self::Relation, "relation"),
        (Self::Relationship, "relationship"),
        (Self::Repr, "repr"),
        (Self::Scope, "scope"),
        (Self::Seal, "seal"),
        (Self::Std, "std"),
        (Self::Suffix, "suffix"),
        (Self::Tree, "tree"),
        (Self::Valid, "valid"),
        (Self::Value, "value"),
        (Self::ValueName, "value_name"),
        (Self::ValuesName, "values_name"),
        (Self::With, "with"),
        (Self::Without, "without"),
    ];

    fn index(self) -> usize {
        return self as usize;
    }
}

// Every row of `TABLE` is at its variant's index.
const _: () = {
    let table = Key::TABLE;
    let mut index = 0;
    while index < table.len() {
        assert!(table[index].0 as usize == index);
        index += 1;
    }
};

/// Where a `key = value` list is parsed, which decides the keys
/// [`rejection`] rules out there.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Site {
    /// A macro's own arguments or a profile body: no rule rejects a key, and
    /// the family reports a key it does not take as an unknown attribute.
    Root,
    /// The arguments of a fixed-profile macro (`circular`, `point`, …).
    FixedProfile,
    /// Bitflag's `integral = [..]`.
    Nested,
    /// A facet body.
    Facet,
    /// The arguments of `packed`.
    Packed,
}

/// A key a site rejects, with its message.
const REJECTIONS: &[(Site, Key, &str)] = &[
    (
        Site::FixedProfile,
        Key::Profile,
        "profile is selected by this macro",
    ),
    (
        Site::Nested,
        Key::Konst,
        "constness is specified in multiple places",
    ),
    (
        Site::Nested,
        Key::PanicAt,
        "panic_at belongs on integral types, not in integral = [..]",
    ),
    (
        Site::Nested,
        Key::Tree,
        "bitflag integral configuration does not support tree",
    ),
    (
        Site::Nested,
        Key::HideIn,
        "hide_in belongs on the bitflag root, not in integral = [..]",
    ),
    (
        Site::Facet,
        Key::Precedes,
        "`precedes` is wrapper-wide and cannot be configured inside a facet",
    ),
];

/// Why `site` rejects `key`, when a rule says it does. [`parse_inner`] asks
/// before the family sees the key and reports the answer at the key.
pub(crate) fn rejection(
    site: Site,
    key: Key,
) -> Option<&'static str> {
    return REJECTIONS
        .iter()
        .find(|(at, rejected, _)| *at == site && *rejected == key)
        .map(|(_, _, message)| *message);
}

impl Site {
    /// What a key the family does not take is called here: `packed` names
    /// it, the other sites report an unknown attribute.
    fn unsupported(
        self,
        key: Key,
    ) -> Option<String> {
        return match self {
            Self::Packed => {
                Some(format!("`packed` does not support `{}`", key.name()))
            }
            Self::Root | Self::FixedProfile | Self::Nested | Self::Facet => {
                None
            }
        };
    }
}

/// Parses `key = value, ...` at `site`. A repeated key is a duplicated arg
/// and a key [`rejection`] rules out at `site` fails with the rule's
/// message, both at the key span. A key `on_attr` rejects fails at the key
/// where the site names such keys ([`Site::unsupported`]); elsewhere it is,
/// like a name outside [`Key`], an unknown attribute at the value span.
///
/// `on_attr` parses the value after `=` and returns `Ok(true)`, or returns
/// `Ok(false)` to reject the key; it must reject before consuming any input,
/// so the error lands on the value span. `Profile::parse_attr` returns
/// `false` for `precedes` on a non-circular profile without consuming.
pub(crate) fn parse_inner(
    stream: ParseStream,
    site: Site,
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

        if let Some(message) = key.and_then(|key| rejection(site, key)) {
            return here.fail(message);
        }
        if !key.map_or(Ok(false), |key| on_attr(key, stream))? {
            return match key.and_then(|key| site.unsupported(key)) {
                Some(message) => here.fail(message),
                None => stream.span().fail("unknown attribute"),
            };
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
