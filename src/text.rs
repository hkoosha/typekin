use std::collections::BTreeSet;
use std::fmt::{
    Debug,
    Formatter,
};

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::quote;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::spanned::Spanned;
use syn::{
    Fields,
    ItemStruct,
    LitBool,
    LitStr,
    Path,
    Type,
};

use crate::zz_flags::{
    Flag,
    FlagGrammar,
    FlagSettings,
    Flags,
};
use crate::zz_friend::{
    Capability,
    Friend,
    TEXT,
    any_friend_has,
};
use crate::zz_hide::{
    Envelope,
    HideCfg,
    HideIn,
};
use crate::zz_lib::{
    Forward,
    Key,
    Konst,
    Konsts,
    TEXT_LINTS,
    lint_scope,
    repr_transparent,
};
use crate::zz_names::{
    NameKey,
    Names,
    ResolvedNames,
};
use crate::zz_runner::{
    MkErr,
    ToCollection,
};
use crate::zz_values::FnName;
use crate::{
    marker_tree,
    zz_lib as zz,
};

/// The whole text expansion: the struct first, then its generated code.
pub(crate) fn ekran(
    attr: Cfg,
    item: ItemStruct,
) -> syn::Result<TokenStream> {
    return TextMaker::new(item, attr)?.ekran();
}

pub(crate) type TextFlags = Flags<TextFlag>;

/// A text `with`/`without` flag.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum TextFlag {
    Clone,
    Debug,
    Display,
    Hash,
    Repr,
    Serde,
    WithBlindTrust,
}

impl Flag for TextFlag {
    const ITEMS: &'static [Self] = &[
        Self::Clone,
        Self::Debug,
        Self::Display,
        Self::Hash,
        Self::Repr,
        Self::Serde,
        Self::WithBlindTrust,
    ];
    const DEFAULT_ON: &'static [Self] = &[
        Self::Clone,
        Self::Debug,
        Self::Display,
        Self::Hash,
        Self::Repr,
    ];

    fn name(self) -> &'static str {
        return match self {
            Self::Clone => "clone",
            Self::Debug => "debug",
            Self::Display => "display",
            Self::Hash => "hash",
            Self::Repr => "repr",
            Self::Serde => "serde",
            Self::WithBlindTrust => "with_blind_trust",
        };
    }

    fn index(self) -> u32 {
        return self as u32;
    }
}

// Every variant is in `ITEMS`, at its own index.
const _: () = {
    let items = <TextFlag as Flag>::ITEMS;
    assert!(items.len() == TextFlag::WithBlindTrust as usize + 1);
    let mut index = 0;
    while index < items.len() {
        assert!(items[index] as usize == index);
        index += 1;
    }
};

/// The `with`/`without` grammar of a text configuration.
const GRAMMAR: FlagGrammar = FlagGrammar {
    konsts: Konsts::TEXT,
    removals: true,
};

/// What a text value must satisfy: `valid = [..]` callbacks or a
/// `domain = {..}` of literals. `Parse` rejects both together.
#[derive(Clone)]
enum TextValidation {
    None,
    Callbacks(Vec<Path>),
    /// `domain = {}` stays valid: it admits no value.
    Values(Vec<LitStr>),
}

impl Debug for TextValidation {
    fn fmt(
        &self,
        formatter: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return match self {
            Self::None => formatter.write_str("None"),
            Self::Callbacks(callbacks) => formatter
                .debug_tuple("Callbacks")
                .field(&callbacks.len())
                .finish(),
            Self::Values(values) => formatter
                .debug_tuple("Values")
                .field(&values.len())
                .finish(),
        };
    }
}

#[derive(Clone)]
pub(crate) struct Cfg {
    pub(crate) konst: Konst,
    /// The explicit `with`/`without` settings, `without = fn_{name}`
    /// included.
    pub(crate) settings: FlagSettings<TextFlag>,
    pub(crate) std: bool,
    pub(crate) tree: Option<marker_tree::Cfg>,
    validation: TextValidation,
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) flags: TextFlags,
    pub(crate) hide_in: crate::zz_hide::HideIn,
}

impl Cfg {
    pub(crate) fn konst_for(
        &self,
        flags: &[Konsts],
    ) -> bool {
        return self
            .settings
            .konsts()
            .konst_for(flags, || self.konst.enables(flags));
    }

    /// `::std::#path`, or `::alloc::#path` without `std`.
    fn alloc_path(
        &self,
        path: TokenStream,
    ) -> TokenStream {
        return match self.std {
            true => quote! { ::std::#path },
            false => quote! { ::alloc::#path },
        };
    }

    fn parse_values(input: ParseStream) -> syn::Result<Vec<LitStr>> {
        let values = zz::set::<LitStr>(input)?.vec();

        let mut seen = BTreeSet::new();
        for value in &values {
            if !seen.insert(value.value()) {
                return value.fail("duplicated text literal");
            }
        }

        return Ok(values);
    }

    fn parse_friends(
        &mut self,
        input: ParseStream,
    ) -> syn::Result<()> {
        let mut friends = BTreeSet::new();

        for friend in zz::one_or_list::<Friend>(input)? {
            TEXT.check(&friend)?;
            let has_make = friend.has(Capability::Make);

            if has_make && friend.conv.is_none() {
                return input.span().fail(
                    "text Make friend requires `conversion(Type) -> Make`",
                );
            }
            let span = friend.ty.path().span();
            if !friends.insert(friend) {
                return span.fail("duplicated text friend");
            }
        }

        self.friends = friends;
        return Ok(());
    }

    /// Fails at `rest` when a `valid` or `domain` is already set.
    fn check_no_validation(
        &self,
        rest: ParseStream,
    ) -> syn::Result<()> {
        if !matches!(self.validation, TextValidation::None) {
            rest.span()
                .fail("`valid` and `domain` cannot be used together")?;
        }
        return Ok(());
    }

    fn has_validation(&self) -> bool {
        return !matches!(self.validation, TextValidation::None);
    }

    pub(crate) fn new(
        konst: Konst,
        std: bool,
    ) -> Self {
        return Self {
            konst,
            settings: FlagSettings::default(),
            std,
            tree: None,
            validation: TextValidation::None,
            friends: BTreeSet::new(),
            flags: Default::default(),
            hide_in: Default::default(),
        };
    }
}

impl Debug for Cfg {
    fn fmt(
        &self,
        formatter: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return formatter
            .debug_struct("TextCfg")
            .field("konst", &self.konst)
            .field("std", &self.std)
            .field("validation", &self.validation)
            .field("tree", &self.tree.is_some())
            .field("friends", &self.friends)
            .field("flags", &self.flags)
            .finish();
    }
}

impl HideCfg for Cfg {
    fn hide_in(&self) -> &HideIn {
        return &self.hide_in;
    }
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::new(Konst::default(), true);

        zz::parse_inner(input, |attr, rest| {
            match attr {
                Key::Std => this.std = rest.parse::<LitBool>()?.value,
                Key::Konst => {
                    this.konst = rest.parse()?;
                }
                Key::Error => zz::parse_unit_error(rest)?,
                Key::Valid => {
                    this.check_no_validation(rest)?;
                    // `valid = []` validates nothing, as no `valid`.
                    let callbacks = zz::parse_callbacks(rest)?;
                    if !callbacks.is_empty() {
                        this.validation = TextValidation::Callbacks(callbacks);
                    }
                }
                Key::Domain => {
                    this.check_no_validation(rest)?;
                    this.validation =
                        TextValidation::Values(Self::parse_values(rest)?);
                }
                Key::Friend => this.parse_friends(rest)?,
                Key::Tree => {
                    this.tree = Some(marker_tree::Cfg::parse_attribute(rest)?);
                }
                Key::HideIn => this.hide_in = rest.parse()?,
                Key::With => this.settings.parse(rest, true, &GRAMMAR)?,
                Key::Without => this.settings.parse(rest, false, &GRAMMAR)?,
                _ => return Ok(false),
            }

            return Ok(true);
        })?;
        this.settings.apply(&mut this.flags);

        return Ok(this);
    }
}

/// A name text generates.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum TextName {
    /// `TextMake`: the friend protocol's trait.
    MakeTrait,
    /// A forward, keyed as `without = fn_*` is.
    Fn(FnName),
}

impl NameKey for TextName {
    type Owner = Ident;

    fn default_name(
        self,
        _owner: &Ident,
    ) -> String {
        return match self {
            Self::MakeTrait => "TextMake".to_string(),
            Self::Fn(function) => function.name().to_string(),
        };
    }
}

impl From<FnName> for TextName {
    fn from(function: FnName) -> Self {
        return Self::Fn(function);
    }
}

pub(crate) struct TextMaker {
    item: ItemStruct,
    ty: Ident,
    /// `::std::string::String`, or `::alloc::string::String` without `std`.
    ty_string: TokenStream,
    cfg: Cfg,
    names: ResolvedNames<TextName>,
}

impl TextMaker {
    pub(crate) fn new(
        item: ItemStruct,
        cfg: Cfg,
    ) -> syn::Result<Self> {
        if !item.generics.params.is_empty() {
            return item
                .generics
                .params
                .fail("text does not support generic structs");
        }
        if let Some(clause) = &item.generics.where_clause {
            return clause.fail("text does not support generic structs");
        }
        let shape_error =
            "expected a tuple struct with exactly one String field";
        let field = match &item.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                &fields.unnamed[0]
            }
            Fields::Unit => return item.ident.fail(shape_error),
            fields => return fields.fail(shape_error),
        };
        if !matches!(
            &field.ty,
            Type::Path(path) if path.qself.is_none() && path.path.is_ident("String")
        ) {
            return field.ty.fail(shape_error);
        }

        return Ok(Self {
            ty: item.ident.clone(),
            names: Names::default()
                .resolve(item.ident.clone(), BTreeSet::new()),
            ty_string: cfg.alloc_path(quote! { string::String }),
            item,
            cfg,
        });
    }

    fn validation_condition(&self) -> TokenStream {
        return match &self.cfg.validation {
            TextValidation::None => quote! { true },
            TextValidation::Callbacks(callbacks) => {
                zz::callback_condition(callbacks, &quote! { value.as_str() })
                    .unwrap_or_else(|| quote! { true })
            }
            TextValidation::Values(values) => values
                .iter()
                .map(|value| quote! { value.as_str() == #value })
                .reduce(|left, right| quote! { (#left) || (#right) })
                .unwrap_or_else(|| quote! { false }),
        };
    }

    /// `is_valid`, `try_make`, `try_from_str`, `map` and `into_inner`.
    fn make_core(&self) -> TokenStream {
        let ty = &self.ty;
        let ty_string = &self.ty_string;
        let konst = Konst::konst(self.cfg.konst_for(&[Konsts::Text]));
        let validation = self.validation_condition();
        let try_make = zz::validated_result(
            &quote! { Self::is_valid(&value) },
            quote! { Self(value) },
            quote! { () },
        );

        return quote! {
            impl #ty {
                #[inline(always)]
                #konst fn is_valid(value: &#ty_string) -> bool {
                    #validation
                }

                #[inline(always)]
                pub #konst fn try_make(
                    value: #ty_string,
                ) -> ::core::result::Result<Self, ()> {
                    #try_make
                }

                #[inline(always)]
                pub fn try_from_str(
                    value: &str,
                ) -> ::core::result::Result<Self, ()> {
                    return Self::try_make(#ty_string::from(value));
                }

                #[inline(always)]
                pub fn map<F>(
                    self,
                    function: F,
                ) -> ::core::result::Result<Self, ()>
                where
                    F: ::core::ops::FnOnce(&mut #ty_string),
                {
                    let mut value = self.0;
                    function(&mut value);
                    return Self::try_make(value);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn into_inner(self) -> #ty_string {
                    return self.0;
                }
            }
        };
    }

    /// `impl #ty { forwards }`, without the `without = fn_*` removals.
    fn make_forwards(&self) -> syn::Result<TokenStream> {
        let ty = &self.ty;
        let mut forwards = self.edit_forwards();
        forwards.extend(self.read_forwards());
        let forwards = self.cfg.settings.removals().keep(forwards)?;

        return Ok(quote! {
            impl #ty {
                #forwards
            }
        });
    }

    /// The forwards that edit the `String` through `map`.
    fn edit_forwards(&self) -> Vec<Forward> {
        let mutate = |name: FnName,
                      parameters: TokenStream,
                      call: TokenStream| {
            return Forward::new(name, &self.names, |name| {
                return quote! {
                    #[inline(always)]
                    pub fn #name(self #parameters) -> ::core::result::Result<Self, ()> {
                        return self.map(|value| value.#call);
                    }
                };
            });
        };

        return vec![
            mutate(
                FnName::TryPush,
                quote! { , character: char },
                quote! { push(character) },
            ),
            mutate(
                FnName::TryPushStr,
                quote! { , text: &str },
                quote! { push_str(text) },
            ),
            mutate(
                FnName::TryInsert,
                quote! { , index: usize, character: char },
                quote! { insert(index, character) },
            ),
            mutate(
                FnName::TryInsertStr,
                quote! { , index: usize, text: &str },
                quote! { insert_str(index, text) },
            ),
            Forward::new(FnName::TryReplaceRange, &self.names, |name| {
                return quote! {
                    #[inline(always)]
                    pub fn #name<R>(self, range: R, text: &str) -> ::core::result::Result<Self, ()>
                    where
                        R: ::core::ops::RangeBounds<usize>,
                    {
                        return self.map(|value| value.replace_range(range, text));
                    }
                };
            }),
            mutate(
                FnName::TryTruncate,
                quote! { , length: usize },
                quote! { truncate(length) },
            ),
            mutate(FnName::TryClear, quote! {}, quote! { clear() }),
            Forward::new(FnName::TryRetain, &self.names, |name| {
                return quote! {
                    #[inline(always)]
                    pub fn #name<F>(self, mut predicate: F) -> ::core::result::Result<Self, ()>
                    where
                        F: ::core::ops::FnMut(char) -> bool,
                    {
                        return self.map(|value| value.retain(|character| predicate(character)));
                    }
                };
            }),
        ];
    }

    /// The forwards that take the `String` apart or read it.
    fn read_forwards(&self) -> Vec<Forward> {
        let ty_vec = self.cfg.alloc_path(quote! { vec::Vec });
        let konst = Konst::konst(self.cfg.konst_for(&[Konsts::Text]));
        let access_konst = Konst::konst(self.cfg.konst_for(&[
            Konsts::TextAccess,
            Konsts::Text,
        ]));
        // The `String` method is `name`.
        let access = |name: FnName, output: TokenStream| {
            return Forward::new(name, &self.names, |name| {
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub #access_konst fn #name(&self) -> #output {
                        return self.0.#name();
                    }
                };
            });
        };

        return vec![
            Forward::new(FnName::TryRemove, &self.names, |name| {
                return quote! {
                    #[inline(always)]
                    pub fn #name(self, index: usize) -> ::core::result::Result<(Self, char), ()> {
                        let mut value = self.0;
                        let removed = value.remove(index);
                        return match Self::try_make(value) {
                            ::core::result::Result::Ok(value) => ::core::result::Result::Ok((value, removed)),
                            ::core::result::Result::Err(()) => ::core::result::Result::Err(()),
                        };
                    }
                };
            }),
            Forward::new(FnName::TryPop, &self.names, |name| {
                return quote! {
                    #[inline(always)]
                    pub fn #name(self) -> ::core::result::Result<(Self, ::core::option::Option<char>), ()> {
                        let mut value = self.0;
                        let popped = value.pop();
                        return match Self::try_make(value) {
                            ::core::result::Result::Ok(value) => ::core::result::Result::Ok((value, popped)),
                            ::core::result::Result::Err(()) => ::core::result::Result::Err(()),
                        };
                    }
                };
            }),
            Forward::new(FnName::IntoBytes, &self.names, |name| {
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub #konst fn #name(self) -> #ty_vec<u8> {
                        return self.0.#name();
                    }
                };
            }),
            access(FnName::AsStr, quote! { &str }),
            access(FnName::AsBytes, quote! { &[u8] }),
            access(FnName::Len, quote! { usize }),
            access(FnName::IsEmpty, quote! { bool }),
            access(FnName::Capacity, quote! { usize }),
        ];
    }

    /// `Add<&str>`, `AsRef<str>`, `AsRef<[u8]>`, `Borrow<str>` and `Deref`.
    fn make_str_access(&self) -> TokenStream {
        let ty = &self.ty;

        return quote! {
            impl ::core::ops::Add<&str> for #ty {
                type Output = Self;

                #[inline(always)]
                fn add(self, text: &str) -> Self {
                    return match self.map(|value| value.push_str(text)) {
                        ::core::result::Result::Ok(value) => value,
                        ::core::result::Result::Err(()) => ::core::panic!("invalid value from text addition"),
                    };
                }
            }

            impl ::core::convert::AsRef<str> for #ty {
                #[inline(always)]
                fn as_ref(&self) -> &str {
                    return self.0.as_str();
                }
            }

            impl ::core::convert::AsRef<[u8]> for #ty {
                #[inline(always)]
                fn as_ref(&self) -> &[u8] {
                    return self.0.as_bytes();
                }
            }

            impl ::core::borrow::Borrow<str> for #ty {
                #[inline(always)]
                fn borrow(&self) -> &str {
                    return self.0.as_str();
                }
            }

            impl ::core::ops::Deref for #ty {
                type Target = str;

                #[inline(always)]
                fn deref(&self) -> &Self::Target {
                    return self.0.as_str();
                }
            }
        };
    }

    fn make_debug(&self) -> TokenStream {
        if !self.cfg.flags.has(TextFlag::Debug) {
            return TokenStream::new();
        }
        let ty = &self.ty;
        let name = ty.to_string();

        return quote! {
            impl ::core::fmt::Debug for #ty {
                #[inline(always)]
                fn fmt(
                    &self,
                    formatter: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    return formatter
                        .debug_tuple(#name)
                        .field(&self.0.as_str())
                        .finish();
                }
            }
        };
    }

    fn make_hash(&self) -> TokenStream {
        if !self.cfg.flags.has(TextFlag::Hash) {
            return TokenStream::new();
        }
        let ty = &self.ty;

        return quote! {
            impl ::core::hash::Hash for #ty {
                #[inline(always)]
                fn hash<H>(
                    &self,
                    state: &mut H,
                )
                where
                    H: ::core::hash::Hasher,
                {
                    return ::core::hash::Hash::hash(
                        self.0.as_str(),
                        state,
                    );
                }
            }
        };
    }

    /// `PartialEq`, `Eq`, `PartialOrd` and `Ord`, by `as_str`.
    fn make_comparison(&self) -> TokenStream {
        let ty = &self.ty;

        return quote! {
            impl ::core::cmp::PartialEq for #ty {
                #[inline(always)]
                fn eq(
                    &self,
                    that: &Self,
                ) -> bool {
                    return self.0.as_str() == that.0.as_str();
                }
            }

            impl ::core::cmp::Eq for #ty {}

            impl ::core::cmp::PartialOrd for #ty {
                #[inline(always)]
                fn partial_cmp(
                    &self,
                    that: &Self,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    return ::core::option::Option::Some(self.cmp(that));
                }
            }

            impl ::core::cmp::Ord for #ty {
                #[inline(always)]
                fn cmp(
                    &self,
                    that: &Self,
                ) -> ::core::cmp::Ordering {
                    return self.0.as_str().cmp(that.0.as_str());
                }
            }
        };
    }

    /// `From<#ty> for String`, then, without validation, the infallible
    /// `From<String>` and `From<&str>`.
    fn make_conversions(&self) -> TokenStream {
        let ty = &self.ty;
        let ty_string = &self.ty_string;
        let into_string = quote! {
            impl ::core::convert::From<#ty> for #ty_string {
                #[inline(always)]
                fn from(value: #ty) -> Self {
                    return value.into_inner();
                }
            }
        };
        if self.cfg.has_validation() {
            return into_string;
        }
        let konst = Konst::konst(self.cfg.konst_for(&[
            Konsts::TextFrom,
            Konsts::From,
            Konsts::Text,
        ]));

        return quote! {
            #into_string

            #konst impl ::core::convert::From<#ty_string> for #ty {
                #[inline(always)]
                fn from(value: #ty_string) -> Self {
                    return Self(value);
                }
            }

            impl ::core::convert::From<&str> for #ty {
                #[inline(always)]
                fn from(value: &str) -> Self {
                    return Self(#ty_string::from(value));
                }
            }
        };
    }

    fn make_display(&self) -> TokenStream {
        if !self.cfg.flags.has(TextFlag::Display) {
            return TokenStream::new();
        }
        let ty = &self.ty;

        return quote! {
            impl ::core::fmt::Display for #ty {
                #[inline(always)]
                fn fmt(
                    &self,
                    formatter: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    return ::core::fmt::Display::fmt(self.0.as_str(), formatter);
                }
            }
        };
    }

    fn is_friendship_konst(&self) -> bool {
        return self.cfg.konst_for(&[
            Konsts::TextFriendship,
            Konsts::Friendship,
            Konsts::Text,
        ]);
    }

    /// The `TextMake` trait, its impls for the `Make` friends, and `of`.
    fn make_friendship(&self) -> TokenStream {
        let makers = self
            .cfg
            .friends
            .iter()
            .filter(|friend| friend.has(Capability::Make))
            .collect::<Vec<_>>();
        if makers.is_empty() {
            return TokenStream::new();
        }
        let ty = &self.ty;
        let is_konst = self.is_friendship_konst();
        let konst = Konst::konst(is_konst);
        let seal = self.names.get(TextName::MakeTrait);
        let bound = Konst::bound(is_konst, quote! { #seal });
        let impls = makers
            .into_iter()
            .map(|friend| self.make_friend_impl(friend));

        return quote! {
            #konst trait #seal {
                fn make_text(self) -> #ty;
            }

            #(#impls)*

            impl #ty {
                #[allow(private_bounds)]
                #[inline(always)]
                pub #konst fn of<T>(value: T) -> Self
                where
                    T: #bound,
                {
                    return #seal::make_text(value);
                }
            }
        };
    }

    /// `impl TextMake for #friend`.
    fn make_friend_impl(
        &self,
        friend: &Friend,
    ) -> TokenStream {
        let ty = &self.ty;
        let konst = Konst::konst(self.is_friendship_konst());
        let seal = self.names.get(TextName::MakeTrait);
        let friend_ty = friend.ty.path();
        let friend_ty = match friend_ty.is_ident("Self") {
            true => quote! { #ty },
            false => quote! { #friend_ty },
        };
        let make = self.make_friend_body(friend);

        return quote! {
            #konst impl #seal for #friend_ty {
                #[inline(always)]
                fn make_text(self) -> #ty {
                    #make
                }
            }
        };
    }

    /// `make_text`: blind trust for an `Unsafe` friend, else validation
    /// that panics on an invalid value.
    fn make_friend_body(
        &self,
        friend: &Friend,
    ) -> TokenStream {
        let ty = &self.ty;
        let conv = friend.conv.as_ref().expect("text Make conversion missing");
        if friend.has(Capability::Unsafe) {
            return quote! {
                return #ty::_blind_trust(#conv(self));
            };
        }

        return quote! {
            return match #ty::try_make(#conv(self)) {
                ::core::result::Result::Ok(value) => value,
                ::core::result::Result::Err(()) => {
                    ::core::panic!("invalid value from untrusted text friend")
                }
            };
        };
    }

    fn make_blind_trust(&self) -> TokenStream {
        let ty = &self.ty;
        let has_friend = any_friend_has(&self.cfg.friends, Capability::Unsafe);
        if !self.cfg.flags.has(TextFlag::WithBlindTrust) && !has_friend {
            return TokenStream::new();
        }

        let ty_string = &self.ty_string;
        let visibility = crate::zz_hide::blind_trust_visibility(
            &self.cfg.hide_in,
            self.cfg.flags.has(TextFlag::WithBlindTrust),
        );
        return quote! {
            impl #ty {
                /// Builds the wrapper **without validation**; the caller
                /// vouches for `raw`.
                #[doc(hidden)]
                #[must_use]
                #[inline(always)]
                #visibility const fn _blind_trust(raw: #ty_string) -> Self {
                    return Self(raw);
                }
            }
        };
    }

    fn make_serde(&self) -> TokenStream {
        if !self.cfg.flags.has(TextFlag::Serde) {
            return TokenStream::new();
        }
        let ty = &self.ty;
        let ty_string = &self.ty_string;

        return quote! {
            impl ::serde::Serialize for #ty {
                #[inline(always)]
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    return <str as ::serde::Serialize>::serialize(
                        self.0.as_str(),
                        serializer,
                    );
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #ty {
                #[inline(always)]
                fn deserialize<D>(
                    deserializer: D,
                ) -> ::core::result::Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    let value =
                        <#ty_string as ::serde::Deserialize<'de>>::
                            deserialize(deserializer)?;

                    return Self::try_make(value).map_err(|_| {
                        <D::Error as ::serde::de::Error>::custom(
                            "invalid value",
                        )
                    });
                }
            }
        };
    }

    fn ekran(self) -> syn::Result<TokenStream> {
        let mut item = self.item.clone();
        let mut envelope = Envelope::new(&self.cfg.hide_in, &item, &item.vis);
        envelope.prepare(&mut item, self.cfg.tree.as_ref())?;
        let generated = [
            self.make_core(),
            self.make_forwards()?,
            self.make_str_access(),
            self.make_debug(),
            self.make_hash(),
            self.make_comparison(),
            self.make_conversions(),
            self.make_display(),
            self.make_friendship(),
            self.make_blind_trust(),
            self.make_serde(),
        ]
        .into_iter()
        .collect::<TokenStream>();

        let repr = repr_transparent(self.cfg.flags.has(TextFlag::Repr));
        let derive_clone = self
            .cfg
            .flags
            .has(TextFlag::Clone)
            .then(|| quote! { #[derive(Clone)] });

        return Ok(envelope.finish(
            quote! { #repr #derive_clone },
            item,
            lint_scope(TEXT_LINTS, generated),
        ));
    }
}
