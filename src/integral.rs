use std::collections::BTreeSet;
use std::fmt::{
    Debug,
    Formatter,
};

use proc_macro2::{
    Ident,
    TokenStream,
    TokenTree,
};
use quote::{
    ToTokens,
    format_ident,
    quote,
};
use syn::{bracketed, parse::{
    Parse,
    ParseStream,
}, parse_quote, Expr, ExprRange, Fields, ItemStruct, Path, Token, Type};

use crate::{
    friendship::{
        Construction,
        Protocol,
        ProtocolFriend,
        cfg::Friend,
    },
    runner,
    runner::{
        Merged,
        MkErr,
        mk_flags,
    },
    value_type::N,
};

pub(crate) fn integral(
    attr: Box<Cfg>,
    item: ItemStruct,
) -> proc_macro::TokenStream {
    return runner::catching(move || {
        let ty = item.ident.clone();

        let el = if let Fields::Unnamed(fields) = &item.fields
            && fields.unnamed.len() == 1
            && let Type::Path(field) = &fields.unnamed[0].ty
            && let Some(id) = field.path.get_ident()
            && N::rust_names().contains(&id.to_string().as_str())
        {
            id.clone()
        }
        else {
            return item.fail(
                "expected a tuple struct with exactly one field of primitive integral type",
            );
        };

        if !runner::find_repr_transparent(&item.attrs)? {
            return item.fail("expecting #[repr(transparent, ...)]");
        }

        let it = Maker::new(ty, el, attr)?.ekran()?;

        let stream = quote::quote! {
            #[allow(dead_code)]
            #[allow(unused_qualifications)]
            #[allow(clippy::unnecessary_cast)]
            const _: () = {
                #it
            };
        };

        return Ok(quote! { #item #stream });
    });
}

mk_flags! {
    #[flag_default(bool=true, str="")]
    #[derive(Debug, Clone)]
    pub(crate) struct IntegralFlags {
        pub impl_range: bool = false,
        pub impl_as_ref: bool,

        pub auto_of_raw: bool,
        pub assertions: bool,
        pub bit_access: bool,

        pub impl_debug: bool,
        pub display: bool = false,
        pub impl_eq: bool,
        pub impl_fmt_binary: bool,
        pub impl_fmt_hex_lower: bool,
        pub impl_fmt_hex_upper: bool,
        pub impl_fmt_octal: bool,
        pub impl_from_str: bool,
        pub impl_core_int: bool,
        pub impl_into: bool,
        pub impl_ord: bool,
        pub impl_partial_eq: bool,
        pub impl_partial_ord: bool,
        pub impl_try_into: bool,

        pub impl_assign_add: bool,
        pub impl_assign_and: bool,
        pub impl_assign_div: bool,
        pub impl_assign_mul: bool,
        pub impl_assign_or: bool,
        pub impl_assign_rem: bool,
        pub impl_assign_shl: bool,
        pub impl_assign_shr: bool,
        pub impl_assign_sub: bool,
        pub impl_math_add: bool,
        pub impl_math_and: bool,
        pub impl_math_div: bool,
        pub impl_math_mul: bool,
        pub impl_math_not: bool,
        pub impl_math_or: bool,
        pub impl_math_rem: bool,
        pub impl_math_shl: bool,
        pub impl_math_shr: bool,
        pub impl_math_sub: bool,
        pub impl_math_xor: bool,

        pub impl_friends: bool,
        pub impl_self_friend_make: bool,
        pub impl_self_friend_math_bit: bool,
        pub impl_self_friend_math_ops: bool,
        pub impl_self_friend_math_rel: bool,
        pub impl_friend_seal: bool,
        pub impl_friendzone_friend_make: bool,
        pub impl_friendzone_friend_math_bit: bool,
        pub impl_friendzone_friend_math_ops: bool,
        pub impl_friendzone_friend_math_rel: bool,
        pub impl_friendzone_seal: bool,

        pub fn_conv_into: bool,
        pub fn_conv_of: bool,
        pub fn_conv_raw: bool,
        pub fn_conv_try_into_checked: bool,
        pub fn_conv_try_into_unchecked: bool,
        pub fn_make_checked: bool,
        pub fn_make_checked_try: bool,
        pub fn_make_unchecked: bool,
        pub fn_make_unchecked_try: bool,
        pub fn_math_add: bool,
        pub fn_math_and: bool,
        pub fn_math_div: bool,
        pub fn_math_mul: bool,
        pub fn_math_not: bool,
        pub fn_math_or: bool,
        pub fn_math_rem: bool,
        pub fn_math_shl: bool,
        pub fn_math_shr: bool,
        pub fn_math_sub: bool,
        pub fn_math_xor: bool,
        pub fn_op_cmp: bool,
        pub fn_op_eq: bool,

        pub fn_make: String = "make",

        pub fp_unchecked: String = "Self::_unchecked",
    }
}

#[derive(Default, Clone)]
pub(crate) struct ValidationCfg {
    pub(crate) callbacks: Vec<Path>,
    pub(crate) ranges: Vec<ExprRange>,
}

impl ValidationCfg {
    pub(crate) fn has_validation(&self) -> bool {
        return !self.callbacks.is_empty() || !self.ranges.is_empty();
    }

    pub(crate) fn parse_callbacks(
        input: ParseStream
    ) -> syn::Result<Vec<Path>> {
        if input.peek(syn::token::Bracket) {
            return Ok(runner::list::<Path>(input)?.collect());
        }

        return Ok(vec![input.parse()?]);
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

        return Self::parse_range_union(tokens);
    }

    fn parse_range_union(tokens: TokenStream) -> syn::Result<Vec<ExprRange>> {
        let mut ranges = vec![];
        let mut range = TokenStream::new();

        for token in tokens {
            if matches!(&token, TokenTree::Punct(it) if it.as_char() == '+') {
                ranges.push(Self::parse_range_tokens(range)?);
                range = TokenStream::new();
            }
            else {
                range.extend([token]);
            }
        }

        ranges.push(Self::parse_range_tokens(range)?);
        return Ok(ranges);
    }

    fn parse_range_tokens(tokens: TokenStream) -> syn::Result<ExprRange> {
        return Self::parse_range(syn::parse2(tokens)?);
    }

    fn parse_range(expr: Expr) -> syn::Result<ExprRange> {
        return match expr {
            Expr::Range(range) if range.attrs.is_empty() => Ok(range),
            expr => expr.fail("invalid range definition"),
        };
    }

    fn parse_attr(
        &mut self,
        attr: &str,
        input: ParseStream,
    ) -> syn::Result<bool> {
        match attr {
            "valid" => {
                self.callbacks = Self::parse_callbacks(input)?;
            }
            "in" => self.ranges = Self::parse_ranges(input)?,
            _ => return Ok(false),
        };

        return Ok(true);
    }
}

#[derive(Default, Clone)]
pub(crate) struct Cfg {
    pub(crate) flags: Box<IntegralFlags>,
    pub(crate) konst: bool,
    pub(crate) get_raw: Option<Path>,
    pub(crate) validation: ValidationCfg,
    pub(crate) friends: BTreeSet<Friend>,
}

impl Cfg {
    pub(crate) fn add_friend(
        &mut self,
        ty: &Ident,
        capabilities: impl IntoIterator<Item = Ident>,
        conv: Path,
    ) {
        let req = Friend::from(ty);
        let mut req = self.friends.take(&req).unwrap_or(req);
        req.capabilities.extend(capabilities);
        req.conv = Some(conv);
        assert!(self.friends.insert(req));
    }

    pub(crate) fn has_validation(&self) -> bool {
        return self.validation.has_validation();
    }

    pub(crate) fn parse_with_konst(
        input: ParseStream,
        konst: bool,
    ) -> syn::Result<Self> {
        let mut this = Self {
            konst,
            ..Default::default()
        };

        runner::parse_inner_attributes(input, |attr, rest| {
            if this.validation.parse_attr(attr, rest)? {
                return Ok(true);
            }

            match attr {
                "konst" => {
                    return attr
                        .fail("constness is specified in multiple places");
                }
                "with" => this.flags.parse_from(rest, true)?,
                "without" => this.flags.parse_from(rest, false)?,
                "friends" => {
                    let friends = runner::one_or_list::<Friend>(rest)?
                        .collect::<BTreeSet<_>>();
                    for friend in &friends {
                        for capability in &friend.capabilities {
                            if !matches!(
                                capability.to_string().as_str(),
                                "Make" | "Math" | "Bit" | "Relation" | "Trust"
                            ) {
                                return capability
                                    .fail("unknown integral capability");
                            }
                        }
                    }
                    this.friends = friends;
                }
                "get_raw" => this.get_raw = Some(rest.parse()?),
                _ => {
                    return Ok(false);
                }
            };

            return Ok(true);
        })?;

        return Ok(this);
    }
}

impl Debug for Cfg {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "IntegralCfg[int: {:?}, friends: {:?}, get_raw: {}, callbacks: {}, ranges: {}]",
            self.flags,
            self.friends,
            self.get_raw
                .as_ref()
                .map(|it| it.to_token_stream().to_string())
                .unwrap_or_default(),
            self.validation.callbacks.len(),
            self.validation.ranges.len(),
        )
    }
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();
        let mut has_konst = false;

        runner::parse_inner_attributes(input, |attr, rest| {
            if this.validation.parse_attr(attr, rest)? {
                return Ok(true);
            }

            match attr {
                "konst" => {
                    this.konst = rest.parse::<syn::LitBool>()?.value;
                    has_konst = true;
                }
                "with" => this.flags.parse_from(rest, true)?,
                "without" => this.flags.parse_from(rest, false)?,
                "friends" => {
                    let friends = runner::one_or_list::<Friend>(rest)?
                        .collect::<BTreeSet<_>>();
                    for friend in &friends {
                        for capability in &friend.capabilities {
                            if !matches!(
                                capability.to_string().as_str(),
                                "Make" | "Math" | "Bit" | "Relation" | "Trust"
                            ) {
                                return capability
                                    .fail("unknown integral capability");
                            }
                        }
                    }
                    this.friends = friends;
                }
                "get_raw" => this.get_raw = Some(rest.parse()?),
                _ => {
                    return Ok(false);
                }
            };

            return Ok(true);
        })?;

        if !has_konst {
            return input.span().fail("missing required `konst` argument");
        }

        return Ok(this);
    }
}

pub(crate) struct Maker {
    repr: N,
    el: Ident,
    ty: Ident,

    fn_conv: Ident,
    fp_get_raw: Path,
    fp_unchecked: Path,

    trait_seal: Ident,
    trait_friend_make: Ident,
    trait_friend_math: Ident,
    trait_friend_bit: Ident,
    trait_friend_rel: Ident,

    cfg: Box<Cfg>,
}

impl Maker {
    pub(crate) fn new(
        ty: Ident,
        el: Ident,
        cfg: Box<Cfg>,
    ) -> syn::Result<Self> {
        let cfg = Self::preprocess_cfg(cfg);

        let mut this = Self {
            trait_seal: format_ident!("Seal"),
            trait_friend_bit: format_ident!("Bit"),
            trait_friend_make: format_ident!("Make"),
            trait_friend_math: format_ident!("Math"),
            trait_friend_rel: format_ident!("Relation"),

            fp_unchecked: syn::parse_str(&cfg.flags.fp_unchecked)?,
            fp_get_raw: cfg
                .get_raw
                .clone()
                .unwrap_or_else(|| parse_quote! { Self::raw }),
            fn_conv: format_ident!(
                "conv_{}",
                runner::snake_case_of(&ty.to_string())
            ),

            repr: N::of(el.to_string()).ok_or_else(|| {
                syn::Error::new(el.span(), "unknown integral type")
            })?,
            el,
            ty,
            cfg,
        };

        this.fix_friendship();

        return Ok(this);
    }

    fn preprocess_cfg(mut cfg: Box<Cfg>) -> Box<Cfg> {
        if cfg.flags.auto_of_raw && cfg.has_validation() {
            cfg.flags.auto_of_raw = false;
        }

        return cfg;
    }

    fn range_condition(
        range: &ExprRange,
        value: &TokenStream,
    ) -> TokenStream {
        let lower = range.start.as_ref().map(|start| {
            return quote! { #value >= (#start) };
        });
        let upper = range.end.as_ref().map(|end| {
            return match &range.limits {
                syn::RangeLimits::HalfOpen(_) => {
                    quote! { #value < (#end) }
                }
                syn::RangeLimits::Closed(_) => {
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

    fn validation_condition(
        &self,
        value: &TokenStream,
    ) -> TokenStream {
        let callbacks = self
            .cfg
            .validation
            .callbacks
            .iter()
            .map(|callback| quote! { #callback(#value) })
            .reduce(|left, right| quote! { (#left) && (#right) });
        let ranges = self
            .cfg
            .validation
            .ranges
            .iter()
            .map(|range| Self::range_condition(range, value))
            .reduce(|left, right| quote! { (#left) || (#right) });

        return match (callbacks, ranges) {
            (Some(callbacks), Some(ranges)) => {
                quote! { (#callbacks) && (#ranges) }
            }
            (Some(callbacks), None) => quote! { #callbacks },
            (None, Some(ranges)) => quote! { #ranges },
            (None, None) => quote! { true },
        };
    }

    fn fix_friendship(&mut self) {
        let target = &self.ty;
        let relation = &self.el;
        let validated = self.cfg.has_validation();
        let has_make = self.cfg.flags.impl_self_friend_make;

        for (needle, conversion, is_validated) in [
            (
                Friend::from(target),
                Some(parse_quote! { #target::raw }),
                false,
            ),
            (
                Friend::from(relation),
                Some(parse_quote! { self }),
                validated,
            ),
        ] {
            let mut friend = self.cfg.friends.take(&needle).unwrap_or(needle);

            if friend.conv.is_none() {
                friend.conv = conversion;
            }

            if !is_validated && has_make {
                friend.capabilities.insert(format_ident!("Make"));
            }

            if self.cfg.flags.impl_self_friend_math_ops {
                friend.capabilities.insert(format_ident!("Math"));
            }

            if self.cfg.flags.impl_self_friend_math_rel {
                friend.capabilities.insert(format_ident!("Relation"));
            }

            if self.cfg.flags.impl_self_friend_math_bit {
                friend.capabilities.insert(format_ident!("Bit"));
            }

            assert!(self.cfg.friends.insert(friend));
        }

        if !validated && has_make {
            N::items()
                .iter()
                .filter(|it| {
                    **it != self.repr && it.can_safe_cast_to(self.repr)
                })
                .map(|it| format_ident!("{}", it.rust_name()))
                .map(|ty| Friend::from(&ty))
                .for_each(|friend| {
                    let mut friend =
                        self.cfg.friends.take(&friend).unwrap_or(friend);
                    friend.capabilities.insert(format_ident!("Make"));
                    assert!(self.cfg.friends.insert(friend));
                });
        }
    }

    fn ekran_friendship(&self) -> TokenStream {
        let target = self.ty.clone();
        let relation_ident = &self.el;
        let relation: Type = parse_quote! { #relation_ident };
        let seal = self.trait_seal.clone();
        let conversion = self.fn_conv.clone();
        let fp_get_raw = &self.fp_get_raw;
        let target_conversion = self.cfg.flags.impl_friend_seal.then(|| {
            quote! {
                return #fp_get_raw(self);
            }
        });
        let capabilities = [
            (
                self.cfg.flags.impl_friendzone_friend_make,
                self.trait_friend_make.clone(),
            ),
            (
                self.cfg.flags.impl_friendzone_friend_math_ops,
                self.trait_friend_math.clone(),
            ),
            (
                self.cfg.flags.impl_friendzone_friend_math_bit,
                self.trait_friend_bit.clone(),
            ),
            (
                self.cfg.flags.impl_friendzone_friend_math_rel,
                self.trait_friend_rel.clone(),
            ),
            (true, format_ident!("Trust")),
        ]
        .into_iter()
        .filter_map(|(enabled, capability)| enabled.then_some(capability))
        .collect();
        let friends = match self.cfg.flags.impl_friends {
            false => vec![],
            true => self
                .cfg
                .friends
                .iter()
                .filter_map(|friend| {
                    let ty = friend.ty.as_ref()?;
                    let is_raw = ty.get_ident().is_some_and(|ident| {
                        N::of(ident.to_string()).is_some()
                    });
                    let conversion = if ty.is_ident(&self.ty) {
                        None
                    }
                    else if is_raw {
                        let relation = &self.el;
                        Some(quote! {
                            return self as #relation;
                        })
                    }
                    else {
                        match &friend.conv {
                            Some(conv) if conv.is_ident("self") => {
                                let relation = &self.el;
                                Some(quote! {
                                    let relation: #relation = self;
                                    return relation;
                                })
                            }
                            Some(conv) => Some(quote! {
                                return #conv(self);
                            }),
                            None => {
                                let relation = &self.el;
                                Some(quote! {
                                    let relation: #relation = self.into();
                                    return relation;
                                })
                            }
                        }
                    };

                    Some(ProtocolFriend {
                        ty: ty.clone(),
                        capabilities: friend.capabilities.clone(),
                        conversion,
                    })
                })
                .collect(),
        };
        let mut checked = self.fp_unchecked.clone();
        if checked.leading_colon.is_none()
            && let Some(segment) = checked.segments.first_mut()
            && segment.ident == "Self"
        {
            segment.ident = target.clone();
        }

        return crate::friendship::emit_protocol(Protocol {
            target,
            relation,
            seal,
            conversion,
            capabilities,
            friends,
            emit_seal: self.cfg.flags.impl_friendzone_seal,
            target_conversion,
            construction: Some(Construction {
                make: self.trait_friend_make.clone(),
                trusted: format_ident!("Trust"),
                checked: quote! { #checked },
            }),
            konst: self.cfg.konst,
        });
    }

    pub(crate) fn make_impl_range(&self) -> TokenStream {
        unimplemented!("range");
    }

    pub(crate) fn ekran(&self) -> syn::Result<TokenStream> {
        let impls = self.ekran_impls()?;
        let items = self.ekran_items();
        let bit_access = self.ekran_bit_access_items();

        let ty = &self.ty;
        return Ok(quote::quote! {
            #impls

            impl #ty {
                #items
            }

            #[allow(clippy::unnecessary_cast)]
            impl #ty {
                #bit_access
            }
        });
    }

    pub(crate) fn ekran_items(&self) -> TokenStream {
        let ty = &self.ty;
        let el = &self.el;
        let fp_unchecked = &self.fp_unchecked;
        let fp_get_raw = &self.fp_get_raw;

        let mut stream = TokenStream::new();

        if self.cfg.flags.fn_conv_of {
            let konst = runner::konst(self.cfg.konst);
            let bonst = runner::bonst(self.cfg.konst);
            let destruct = runner::destruct(self.cfg.konst);
            let what = &self.trait_friend_make;

            let fn_of = format_ident!("of");

            let it = quote! {
                #[inline(always)]
                #[allow(private_bounds)]
                pub #konst fn #fn_of<T>(
                    it: T,
                ) -> #ty
                where
                    T: #bonst #what #destruct,
                {
                    return <T as #what>::make(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.auto_of_raw {
            let konst = runner::konst(self.cfg.konst);
            let fn_make = format_ident!("{}", self.cfg.flags.fn_make);
            let it = quote! {
                #[inline(always)]
                pub #konst fn #fn_make(it: #el) -> #ty {
                    return #ty::of(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_conv_raw {
            let get_raw = &fp_get_raw.segments.last().unwrap().ident;

            let it = quote! {
                #[must_use]
                #[inline(always)]
                pub const fn #get_raw(self) -> #el {
                    return self.0;
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_conv_into {
            let it = N::items()
                .iter()
                .filter(|it| self.repr.can_safe_cast_to(**it))
                .map(|it| {
                    let target = format_ident!("{}", it.rust_name());
                    let conv = format_ident!("into_{}", it.rust_name());
                    return quote! {
                        #[must_use]
                        #[inline(always)]
                        #[allow(clippy::unnecessary_cast)]
                        pub const fn #conv(self) -> #target {
                            let it = #fp_get_raw(self);
                            return it as #target;
                        }
                    };
                })
                .merged();
            stream.extend(it);
        }

        if self.cfg.flags.fn_conv_try_into_unchecked {
            let it = N::items()
                .iter()
                .filter(|it| self.repr.can_safe_cast_to(**it))
                .map(|it| {
                    let target = format_ident!("{}", it.rust_name());
                    let conv = format_ident!("try_into_{}", it.rust_name());
                    return quote! {
                        #[inline(always)]
                        #[allow(clippy::unnecessary_cast)]
                        pub const fn #conv(self) -> ::core::result::Result<#target, ()> {
                            return ::core::result::Result::Ok(
                                #fp_get_raw(self) as #target
                            );
                        }
                    };
                })
                .merged();
            stream.extend(it);
        }

        if self.cfg.flags.fn_conv_try_into_checked {
            let source = format_ident!("{}", self.repr.rust_name());
            let it = N::items()
                .iter()
                .filter(|it| !self.repr.can_safe_cast_to(**it))
                .map(|it| {
                    let target = format_ident!("{}", it.rust_name());
                    let name = format_ident!("try_into_{}", it.rust_name());
                    let sign_check = match (self.repr.is_signed(), it.is_signed()) {
                        (false, true) => quote! { t >= 0 },
                        (true, false) => quote! { r >= 0 },
                        _ => quote! { true },
                    };
                    return quote! {
                    #[inline(always)]
                    #[allow(clippy::unnecessary_cast)]
                    pub const fn #name(self) -> ::core::result::Result<#target, ()> {
                        let r = #fp_get_raw(self);
                        let t = r as #target;
                        let s = t as #source;

                        return if s == r && #sign_check {
                            ::core::result::Result::Ok(t)
                        }
                        else {
                            ::core::result::Result::Err(())
                        };
                    }
                };
                })
                .merged();
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_add {
            let it = quote! {
                pub(self) const fn _add(
                    &self,
                    rhs: #el
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs + rhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_sub {
            let it = quote! {
                pub(self) const fn _sub(
                    &self,
                    rhs: #el
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs - rhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_mul {
            let it = quote! {
                pub(self) const fn _mul(
                    &self,
                    rhs: #el
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs * rhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_div {
            let it = quote! {
                pub(self) const fn _div(
                    &self,
                    rhs: #el
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs / rhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_rem {
            let it = quote! {
                pub(self) const fn _rem(
                    &self,
                    rhs: #el
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs % rhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_xor {
            let it = quote! {
                pub(self) const fn _bitxor(
                    &self,
                    rhs: #el
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs ^ rhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_and {
            let it = quote! {
                pub(self) const fn _bitand(
                    &self,
                    rhs: #el
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs & rhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_or {
            let it = quote! {
                pub(self) const fn _bitor(
                    &self,
                    rhs: #el
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs | rhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_shr {
            let it = quote! {
                pub(self) const fn _shr(
                    &self,
                    count: usize,
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs >> count;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_shl {
            let it = quote! {
                pub(self) const fn _shl(
                    &self,
                    count: usize,
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs << count;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_not {
            let it = quote! {
                pub(self) const fn _not(
                    &self,
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = !lhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_op_eq {
            let it = quote! {
                #[must_use]
                #[inline(always)]
                #[doc(hidden)]
                pub(self) const fn _eq(
                    self,
                    rhs: #el,
                ) -> bool {
                    let lhs = #fp_get_raw(self);
                    return lhs == rhs;
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_op_cmp {
            let it = quote! {
                #[must_use]
                #[inline(always)]
                #[doc(hidden)]
                pub(self) fn _cmp(
                    self,
                    rhs: #el,
                ) -> ::core::cmp::Ordering {
                    let lhs = #fp_get_raw(self);
                    return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs)
                        .unwrap();
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_make_unchecked_try && self.cfg.has_validation() {
            let konst = runner::konst(self.cfg.konst);
            let value = quote! { it };
            let validation = self.validation_condition(&value);
            let it = quote! {
                #[inline(always)]
                pub #konst fn try_make(it: #el) -> Result<Self, #el> {
                    return if #validation {
                        return ::core::result::Result::Ok(Self(it));
                    }
                    else {
                        ::core::result::Result::Err(it)
                    };
                }
            };
            stream.extend(it);
        }
        else if self.cfg.flags.fn_make_checked_try
            && !self.cfg.has_validation()
        {
            let it = quote! {
                #[inline(always)]
                pub const fn try_make(it: #el) -> Result<Self, #el> {
                    return Ok(#fp_unchecked(it));
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_make_unchecked && self.cfg.has_validation() {
            let value = quote! { it };
            let validation = self.validation_condition(&value);
            let it = quote! {
                #[must_use]
                #[inline(always)]
                pub(self) const fn _unchecked(it: #el) -> Self {
                    if #validation {
                        return Self(it);
                    }
                    else {
                        ::core::panic!("invalid value");
                    };
                }
            };
            stream.extend(it);
        }
        else if self.cfg.flags.fn_make_checked && !self.cfg.has_validation() {
            let it = quote! {
                #[must_use]
                #[inline(always)]
                pub(self) const fn _unchecked(it: #el) -> Self {
                    return Self(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_core_int {
            let konst = runner::konst(self.cfg.konst);
            let value = quote! { value };
            let validation = self.validation_condition(&value);
            let it = quote! {
                #[inline(always)]
                #[track_caller]
                pub(self) #konst fn _core_int(value: #el) -> Self {
                    if #validation {
                        return Self(value);
                    }
                    else {
                        ::core::panic!("integral operation produced an invalid value");
                    }
                }

                #[inline(always)]
                pub(self) #konst fn _core_int_checked(
                    value: #el,
                ) -> ::core::option::Option<Self> {
                    return if #validation {
                        ::core::option::Option::Some(Self(value))
                    }
                    else {
                        ::core::option::Option::None
                    };
                }
            };
            stream.extend(it);
        }

        return stream;
    }

    pub(crate) fn ekran_impls(&self) -> syn::Result<TokenStream> {
        let ty = &self.ty;
        let el = &self.el;
        let value = quote! { value };
        let validation = self.validation_condition(&value);

        let trait_friend_bit = &self.trait_friend_bit;
        let trait_friend_math = &self.trait_friend_math;
        let trait_seal = &self.trait_seal;

        let fn_conv = &self.fn_conv;
        let fp_friend_conv = quote! { #trait_seal::#fn_conv };
        let fp_get_raw = &self.fp_get_raw;

        let konst = runner::konst(self.cfg.konst);
        let destruct = runner::destruct(self.cfg.konst);

        let cond_seal = {
            let bonst = runner::bonst(self.cfg.konst);
            quote! { #bonst #trait_seal }
        };

        let mut stream = TokenStream::new();
        stream.extend(self.ekran_friendship());

        if self.cfg.flags.impl_as_ref {
            let it = quote! {
                impl ::core::convert::AsRef<#el> for #ty {
                    #[inline(always)]
                    fn as_ref(&self) -> &#el {
                        return &self.0;
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_from_str {
            stream.extend(quote! {
                impl ::core::str::FromStr for #ty {
                    type Err = ();

                    #[inline(always)]
                    fn from_str(
                        source: &str,
                    ) -> ::core::result::Result<Self, Self::Err> {
                        let value =
                            match <#el as ::core::str::FromStr>::from_str(source) {
                                ::core::result::Result::Ok(value) => value,
                                ::core::result::Result::Err(_) => {
                                    return ::core::result::Result::Err(());
                                }
                            };

                        return if #validation {
                            ::core::result::Result::Ok(Self(value))
                        }
                        else {
                            ::core::result::Result::Err(())
                        };
                    }
                }
            });
        }

        if self.cfg.flags.impl_core_int {
            let it = quote! {
                impl #ty {
                    /// The number of bits in the wrapped primitive integer.
                    pub const BITS: u32 = #el::BITS;

                    #[inline(always)]
                    pub const fn count_ones(self) -> u32 {
                        return #fp_get_raw(self).count_ones();
                    }

                    #[inline(always)]
                    pub const fn count_zeros(self) -> u32 {
                        return #fp_get_raw(self).count_zeros();
                    }

                    #[inline(always)]
                    pub const fn leading_zeros(self) -> u32 {
                        return #fp_get_raw(self).leading_zeros();
                    }

                    #[inline(always)]
                    pub const fn trailing_zeros(self) -> u32 {
                        return #fp_get_raw(self).trailing_zeros();
                    }

                    #[inline(always)]
                    pub const fn leading_ones(self) -> u32 {
                        return #fp_get_raw(self).leading_ones();
                    }

                    #[inline(always)]
                    pub const fn trailing_ones(self) -> u32 {
                        return #fp_get_raw(self).trailing_ones();
                    }

                    #[inline(always)]
                    pub const fn highest_one(self) -> ::core::option::Option<u32> {
                        return #fp_get_raw(self).highest_one();
                    }

                    #[inline(always)]
                    pub const fn lowest_one(self) -> ::core::option::Option<u32> {
                        return #fp_get_raw(self).lowest_one();
                    }

                    #[inline(always)]
                    pub const fn ilog(self, base: Self) -> u32 {
                        return #fp_get_raw(self).ilog(#fp_get_raw(base));
                    }

                    #[inline(always)]
                    pub const fn ilog2(self) -> u32 {
                        return #fp_get_raw(self).ilog2();
                    }

                    #[inline(always)]
                    pub const fn ilog10(self) -> u32 {
                        return #fp_get_raw(self).ilog10();
                    }

                    #[inline(always)]
                    pub const fn checked_ilog(
                        self,
                        base: Self,
                    ) -> ::core::option::Option<u32> {
                        return #fp_get_raw(self).checked_ilog(#fp_get_raw(base));
                    }

                    #[inline(always)]
                    pub const fn checked_ilog2(self) -> ::core::option::Option<u32> {
                        return #fp_get_raw(self).checked_ilog2();
                    }

                    #[inline(always)]
                    pub const fn checked_ilog10(self) -> ::core::option::Option<u32> {
                        return #fp_get_raw(self).checked_ilog10();
                    }

                    #[inline(always)]
                    pub const fn to_be_bytes(
                        self,
                    ) -> [u8; ::core::mem::size_of::<#el>()] {
                        return #fp_get_raw(self).to_be_bytes();
                    }

                    #[inline(always)]
                    pub const fn to_le_bytes(
                        self,
                    ) -> [u8; ::core::mem::size_of::<#el>()] {
                        return #fp_get_raw(self).to_le_bytes();
                    }

                    #[inline(always)]
                    pub const fn to_ne_bytes(
                        self,
                    ) -> [u8; ::core::mem::size_of::<#el>()] {
                        return #fp_get_raw(self).to_ne_bytes();
                    }
                }
            };
            stream.extend(it);

            let konst = runner::konst(self.cfg.konst);
            let it = quote! {
                impl #ty {
                    #[inline(always)]
                    pub #konst fn rotate_left(self, n: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).rotate_left(n));
                    }

                    #[inline(always)]
                    pub #konst fn rotate_right(self, n: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).rotate_right(n));
                    }

                    #[inline(always)]
                    pub #konst fn swap_bytes(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).swap_bytes());
                    }

                    #[inline(always)]
                    pub #konst fn reverse_bits(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).reverse_bits());
                    }

                    #[inline(always)]
                    pub #konst fn isolate_highest_one(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).isolate_highest_one());
                    }

                    #[inline(always)]
                    pub #konst fn isolate_lowest_one(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).isolate_lowest_one());
                    }

                    #[inline(always)]
                    pub #konst fn to_be(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).to_be());
                    }

                    #[inline(always)]
                    pub #konst fn to_le(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).to_le());
                    }

                    #[inline(always)]
                    pub #konst fn from_be(value: Self) -> Self {
                        return Self::_core_int(#el::from_be(#fp_get_raw(value)));
                    }

                    #[inline(always)]
                    pub #konst fn from_le(value: Self) -> Self {
                        return Self::_core_int(#el::from_le(#fp_get_raw(value)));
                    }

                    #[inline(always)]
                    pub #konst fn from_ne_bytes(
                        bytes: [u8; ::core::mem::size_of::<#el>()],
                    ) -> Self {
                        return Self::_core_int(#el::from_ne_bytes(bytes));
                    }

                    #[inline(always)]
                    pub #konst fn from_be_bytes(
                        bytes: [u8; ::core::mem::size_of::<#el>()],
                    ) -> Self {
                        return Self::_core_int(#el::from_be_bytes(bytes));
                    }

                    #[inline(always)]
                    pub #konst fn from_le_bytes(
                        bytes: [u8; ::core::mem::size_of::<#el>()],
                    ) -> Self {
                        return Self::_core_int(#el::from_le_bytes(bytes));
                    }

                    #[inline(always)]
                    pub #konst fn checked_add(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_add(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_sub(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_sub(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_neg(
                        self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_neg() {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_mul(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_mul(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_div(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_div(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_rem(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_rem(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_div_euclid(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_div_euclid(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_rem_euclid(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_rem_euclid(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_shl(
                        self,
                        rhs: u32,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_shl(rhs) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_shr(
                        self,
                        rhs: u32,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_shr(rhs) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_pow(
                        self,
                        exp: u32,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_pow(exp) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                }
            };
            stream.extend(it);

            let it = quote! {
                impl #ty {
                    #[inline(always)]
                    pub #konst fn midpoint(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).midpoint(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn unbounded_shl(self, rhs: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).unbounded_shl(rhs));
                    }

                    #[inline(always)]
                    pub #konst fn unbounded_shr(self, rhs: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).unbounded_shr(rhs));
                    }

                    #[inline(always)]
                    pub #konst fn saturating_add(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).saturating_add(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn saturating_sub(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).saturating_sub(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn saturating_mul(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).saturating_mul(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn saturating_div(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).saturating_div(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn saturating_pow(self, exp: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).saturating_pow(exp));
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_add(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_add(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_sub(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_sub(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_neg(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).wrapping_neg());
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_mul(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_mul(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_div(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_div(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_rem(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_rem(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_div_euclid(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_div_euclid(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_rem_euclid(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_shl(self, rhs: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).wrapping_shl(rhs));
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_shr(self, rhs: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).wrapping_shr(rhs));
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_pow(self, exp: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).wrapping_pow(exp));
                    }

                    #[inline(always)]
                    pub #konst fn pow(self, exp: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).pow(exp));
                    }

                    #[inline(always)]
                    pub #konst fn isqrt(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).isqrt());
                    }

                    #[inline(always)]
                    pub #konst fn div_euclid(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).div_euclid(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn rem_euclid(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).rem_euclid(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_add(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_add(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_sub(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_sub(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_neg(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).strict_neg());
                    }

                    #[inline(always)]
                    pub #konst fn strict_mul(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_mul(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_div(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_div(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_rem(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_rem(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_div_euclid(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_div_euclid(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_rem_euclid(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_rem_euclid(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_shl(self, rhs: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).strict_shl(rhs));
                    }

                    #[inline(always)]
                    pub #konst fn strict_shr(self, rhs: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).strict_shr(rhs));
                    }

                    #[inline(always)]
                    pub #konst fn strict_pow(self, exp: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).strict_pow(exp));
                    }
                }
            };
            stream.extend(it);

            let it = quote! {
                impl #ty {
                    #[inline(always)]
                    pub #konst fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_add(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_sub(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_neg(self) -> (Self, bool) {
                        let (value, overflowed) = #fp_get_raw(self).overflowing_neg();
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_mul(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_div(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_rem(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_div_euclid(
                        self,
                        rhs: Self,
                    ) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_div_euclid(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_rem_euclid(
                        self,
                        rhs: Self,
                    ) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_rem_euclid(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_shl(rhs);
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_shr(rhs);
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_pow(
                        self,
                        exp: u32,
                    ) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_pow(exp);
                        return (Self::_core_int(value), overflowed);
                    }
                }
            };
            stream.extend(it);

            if self.repr.is_signed() {
                let unsigned =
                    format_ident!("{}", self.repr.unsigned_rust_name());
                let it = quote! {
                    impl #ty {
                        #[inline(always)]
                        pub #konst fn cast_unsigned(self) -> #unsigned {
                            return #fp_get_raw(self).cast_unsigned();
                        }

                        #[inline(always)]
                        pub #konst fn abs(self) -> Self {
                            return Self::_core_int(#fp_get_raw(self).abs());
                        }

                        #[inline(always)]
                        pub #konst fn checked_abs(
                            self,
                        ) -> ::core::option::Option<Self> {
                            return match #fp_get_raw(self).checked_abs() {
                                ::core::option::Option::Some(value) => {
                                    Self::_core_int_checked(value)
                                }
                                ::core::option::Option::None => {
                                    ::core::option::Option::None
                                }
                            };
                        }

                        #[inline(always)]
                        pub #konst fn checked_isqrt(
                            self,
                        ) -> ::core::option::Option<Self> {
                            return match #fp_get_raw(self).checked_isqrt() {
                                ::core::option::Option::Some(value) => {
                                    Self::_core_int_checked(value)
                                }
                                ::core::option::Option::None => {
                                    ::core::option::Option::None
                                }
                            };
                        }

                        #[inline(always)]
                        pub #konst fn strict_abs(self) -> Self {
                            return Self::_core_int(#fp_get_raw(self).strict_abs());
                        }

                        #[inline(always)]
                        pub #konst fn saturating_neg(self) -> Self {
                            return Self::_core_int(#fp_get_raw(self).saturating_neg());
                        }

                        #[inline(always)]
                        pub #konst fn saturating_abs(self) -> Self {
                            return Self::_core_int(#fp_get_raw(self).saturating_abs());
                        }

                        #[inline(always)]
                        pub #konst fn wrapping_abs(self) -> Self {
                            return Self::_core_int(#fp_get_raw(self).wrapping_abs());
                        }

                        #[inline(always)]
                        pub #konst fn overflowing_abs(self) -> (Self, bool) {
                            let (value, overflowed) =
                                #fp_get_raw(self).overflowing_abs();
                            return (Self::_core_int(value), overflowed);
                        }

                        #[inline(always)]
                        pub #konst fn unsigned_abs(self) -> #unsigned {
                            return #fp_get_raw(self).unsigned_abs();
                        }

                        #[inline(always)]
                        pub #konst fn abs_diff(self, rhs: Self) -> #unsigned {
                            return #fp_get_raw(self).abs_diff(#fp_get_raw(rhs));
                        }

                        #[inline(always)]
                        pub #konst fn signum(self) -> Self {
                            return Self::_core_int(#fp_get_raw(self).signum());
                        }

                        #[inline(always)]
                        pub #konst fn is_positive(self) -> bool {
                            return #fp_get_raw(self).is_positive();
                        }

                        #[inline(always)]
                        pub #konst fn is_negative(self) -> bool {
                            return #fp_get_raw(self).is_negative();
                        }
                    }
                };
                stream.extend(it);
            }
            else {
                let signed = format_ident!(
                    "{}",
                    match self.repr {
                        N::USIZ => "isize",
                        N::U008 => "i8",
                        N::U016 => "i16",
                        N::U032 => "i32",
                        N::U064 => "i64",
                        N::U128 => "i128",
                        _ => unreachable!("unsigned representation"),
                    }
                );
                let it = quote! {
                    impl #ty {
                        #[inline(always)]
                        pub #konst fn cast_signed(self) -> #signed {
                            return #fp_get_raw(self).cast_signed();
                        }

                        #[inline(always)]
                        pub #konst fn bit_width(self) -> u32 {
                            return #fp_get_raw(self).bit_width();
                        }

                        #[inline(always)]
                        pub #konst fn funnel_shl(
                            self,
                            right: Self,
                            n: u32,
                        ) -> Self {
                            return Self::_core_int(
                                #fp_get_raw(self).funnel_shl(
                                    #fp_get_raw(right),
                                    n,
                                )
                            );
                        }

                        #[inline(always)]
                        pub #konst fn funnel_shr(
                            self,
                            right: Self,
                            n: u32,
                        ) -> Self {
                            return Self::_core_int(
                                #fp_get_raw(self).funnel_shr(
                                    #fp_get_raw(right),
                                    n,
                                )
                            );
                        }

                        #[inline(always)]
                        pub #konst fn abs_diff(self, rhs: Self) -> #el {
                            return #fp_get_raw(self).abs_diff(#fp_get_raw(rhs));
                        }

                        #[inline(always)]
                        pub #konst fn is_multiple_of(self, rhs: Self) -> bool {
                            return #fp_get_raw(self).is_multiple_of(#fp_get_raw(rhs));
                        }

                        #[inline(always)]
                        pub #konst fn is_power_of_two(self) -> bool {
                            return #fp_get_raw(self).is_power_of_two();
                        }

                        #[inline(always)]
                        pub #konst fn next_power_of_two(self) -> Self {
                            return Self::_core_int(
                                #fp_get_raw(self).next_power_of_two()
                            );
                        }

                        #[inline(always)]
                        pub #konst fn checked_next_power_of_two(
                            self,
                        ) -> ::core::option::Option<Self> {
                            return match #fp_get_raw(self).checked_next_power_of_two() {
                                ::core::option::Option::Some(value) => {
                                    Self::_core_int_checked(value)
                                }
                                ::core::option::Option::None => {
                                    ::core::option::Option::None
                                }
                            };
                        }

                        #[inline(always)]
                        pub #konst fn div_ceil(self, rhs: Self) -> Self {
                            return Self::_core_int(
                                #fp_get_raw(self).div_ceil(#fp_get_raw(rhs))
                            );
                        }

                        #[inline(always)]
                        pub #konst fn next_multiple_of(self, rhs: Self) -> Self {
                            return Self::_core_int(
                                #fp_get_raw(self).next_multiple_of(#fp_get_raw(rhs))
                            );
                        }

                        #[inline(always)]
                        pub #konst fn checked_next_multiple_of(
                            self,
                            rhs: Self,
                        ) -> ::core::option::Option<Self> {
                            return match #fp_get_raw(self).checked_next_multiple_of(
                                #fp_get_raw(rhs),
                            ) {
                                ::core::option::Option::Some(value) => {
                                    Self::_core_int_checked(value)
                                }
                                ::core::option::Option::None => {
                                    ::core::option::Option::None
                                }
                            };
                        }

                        #[inline(always)]
                        pub fn carrying_add(
                            self,
                            rhs: Self,
                            carry: bool,
                        ) -> (Self, bool) {
                            let (value, carry) = #fp_get_raw(self).carrying_add(
                                #fp_get_raw(rhs),
                                carry,
                            );
                            return (Self::_core_int(value), carry);
                        }

                        #[inline(always)]
                        pub fn borrowing_sub(
                            self,
                            rhs: Self,
                            borrow: bool,
                        ) -> (Self, bool) {
                            let (value, borrow) = #fp_get_raw(self).borrowing_sub(
                                #fp_get_raw(rhs),
                                borrow,
                            );
                            return (Self::_core_int(value), borrow);
                        }

                        #[inline(always)]
                        pub fn carrying_mul(
                            self,
                            rhs: Self,
                            carry: Self,
                        ) -> (Self, Self) {
                            let (low, high) = #fp_get_raw(self).carrying_mul(
                                #fp_get_raw(rhs),
                                #fp_get_raw(carry),
                            );
                            return (
                                Self::_core_int(low),
                                Self::_core_int(high),
                            );
                        }

                        #[inline(always)]
                        pub fn carrying_mul_add(
                            self,
                            rhs: Self,
                            carry: Self,
                            add: Self,
                        ) -> (Self, Self) {
                            let (low, high) =
                                #fp_get_raw(self).carrying_mul_add(
                                    #fp_get_raw(rhs),
                                    #fp_get_raw(carry),
                                    #fp_get_raw(add),
                                );
                            return (
                                Self::_core_int(low),
                                Self::_core_int(high),
                            );
                        }
                    }
                };
                stream.extend(it);
            }
        }

        if self.cfg.flags.impl_into {
            let it = N::items()
                .iter()
                .filter(|it| self.repr.can_safe_cast_to(**it))
                .map(|it| {
                    let target = format_ident!("{}", it.rust_name());
                    let conv = format_ident!("into_{}", it.rust_name());
                    return quote! {
                       #konst impl ::core::convert::Into<#target> for #ty {
                           #[inline(always)]
                           fn into(self) -> #target {
                                return #ty::#conv(self);
                           }
                       }
                    };
                })
                .merged();
            stream.extend(it);
        }

        if self.cfg.flags.impl_try_into {
            let it = N::items()
                .iter()
                .filter(|it| !self.repr.can_safe_cast_to(**it))
                .map(|it| {
                    let target = format_ident!("{}", it.rust_name());
                    let conv = format_ident!("try_into_{}", it.rust_name());
                    return quote! {
                       #konst impl ::core::convert::TryInto<#target> for #ty {
                           type Error = ();

                           #[inline(always)]
                           fn try_into(self) -> ::core::result::Result<#target, Self::Error> {
                                return #ty::#conv(self);
                           }

                       }
                    };
                })
                .merged();
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_shr {
            let it = quote! {
                #konst impl ::core::ops::Shr<usize> for #ty {
                    type Output = Self;

                    #[inline(always)]
                    fn shr(
                        self,
                        rhs: usize,
                    ) -> Self::Output {
                        return self._shr(rhs);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_shl {
            let it = quote::quote! {
                #konst impl ::core::ops::Shl<usize> for #ty {
                    type Output = Self;

                    #[inline(always)]
                    fn shl(
                        self,
                        rhs: usize,
                    ) -> Self::Output {
                        return self._shl(rhs);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_shr {
            let it = quote! {
                #konst impl ::core::ops::ShrAssign<usize> for #ty {
                    #[inline(always)]
                    fn shr_assign(
                        &mut self,
                        other: usize,
                    ) {
                        *self = self._shr(other);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_shl {
            let it = quote! {
                #konst impl ::core::ops::ShlAssign<usize> for #ty {
                    #[inline(always)]
                    fn shl_assign(
                        &mut self,
                        other: usize,
                    ) {
                        *self = self._shl(other);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_and {
            let it = quote! {
                #konst impl<T> ::core::ops::BitAndAssign<T> for #ty
                where T: #trait_friend_bit + #cond_seal #destruct
                {
                    #[inline(always)]
                    fn bitand_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self._bitand(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_add {
            let it = quote! {
                #konst impl<T> ::core::ops::AddAssign<T> for #ty
                where T: #trait_friend_math + #cond_seal #destruct
                {
                    #[inline(always)]
                    fn add_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self._add(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_sub {
            let it = quote! {
                #konst impl<T> ::core::ops::SubAssign<T> for #ty
                where T: #trait_friend_math + #cond_seal #destruct
                {
                    #[inline(always)]
                    fn sub_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self._sub(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_mul {
            let it = quote! {
                #konst impl<T> ::core::ops::MulAssign<T> for #ty
                where T: #trait_friend_math + #cond_seal #destruct
                {
                    #[inline(always)]
                    fn mul_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self._mul(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_div {
            let it = quote! {
                #konst impl<T> ::core::ops::DivAssign<T> for #ty
                where T: #trait_friend_math + #cond_seal #destruct
                {
                    #[inline(always)]
                    fn div_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self._div(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_rem {
            let it = quote! {
                #konst impl<T> ::core::ops::RemAssign<T> for #ty
                where T: #trait_friend_math + #cond_seal #destruct
                {
                    #[inline(always)]
                    fn rem_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self._rem(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_or {
            let it = quote! {
                #konst impl<T> ::core::ops::BitOrAssign<T> for #ty
                where T: #trait_friend_bit + #cond_seal #destruct
                {
                    #[inline(always)]
                    fn bitor_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self._bitor(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_range {
            stream.extend(self.make_impl_range());
        }

        if self.cfg.flags.impl_debug {
            let fmt_str = format!("{}({})", ty, "{}");
            let it = quote! {
                impl ::core::fmt::Debug for #ty {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        ::core::write!(f, #fmt_str, self.0)
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_eq {
            // TODO why can't we do derive?
            let it = quote! {
                #konst impl ::core::cmp::Eq for #ty {}
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_ord {
            let it = quote! {
                #konst impl ::core::cmp::Ord for #ty {
                    #[inline(always)]
                    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                        return self.partial_cmp(other).unwrap();
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.assertions {
            let it = quote! {
                if !(::core::mem::size_of::<#ty>() == ::core::mem::size_of::<#el>()) {
                    panic!("invalid memory layout, mismatching sizes: #ty(#el) != #el");
                };
                if !(::core::mem::align_of::<#ty>() == ::core::mem::align_of::<#el>()) {
                    panic!("invalid memory layout, mismatching alignment: #ty(#el) != #el");
                };
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_add {
            let it = quote! {
                #konst impl<T> ::core::ops::Add<T> for #ty
                where T: #trait_friend_math + #cond_seal #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn add(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return self._add(it);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_sub {
            let it = quote! {
                #konst impl<T> ::core::ops::Sub<T> for #ty
                where T: #trait_friend_math + #cond_seal #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn sub(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return self._sub(it);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_mul {
            let it = quote! {
                #konst impl<T> ::core::ops::Mul<T> for #ty
                where T: #trait_friend_math + #cond_seal #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn mul(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return self._mul(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_math_div {
            let it = quote! {
                #konst impl<T> ::core::ops::Div<T> for #ty
                where T: #trait_friend_math + #cond_seal #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn div(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return self._div(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_math_rem {
            let it = quote! {
                #konst impl<T> ::core::ops::Rem<T> for #ty
                where T: #trait_friend_math + #cond_seal #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn rem(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return self._rem(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_math_and {
            let it = quote! {
                #konst impl<T> ::core::ops::BitAnd<T> for #ty
                where T: #trait_friend_bit + #cond_seal #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn bitand(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return self._bitand(it);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_or {
            let it = quote! {
                #konst impl<T> ::core::ops::BitOr<T> for #ty
                where T: #trait_friend_bit + #cond_seal #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn bitor(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return self._bitor(it);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_xor {
            let it = quote! {
                #konst impl<T> ::core::ops::BitXor<T> for #ty
                where T: #trait_friend_bit + #cond_seal #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn bitxor(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return self._bitxor(it);
                    }
                }

                #konst impl<T> ::core::ops::BitXorAssign<T> for #ty
                where T: #trait_friend_bit + #cond_seal #destruct
                {
                    #[inline(always)]
                    fn bitxor_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self._bitxor(it);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_not {
            let it = quote! {
                #konst impl ::core::ops::Not for #ty {
                    type Output = Self;

                    #[inline(always)]
                    fn not(self) -> Self::Output {
                        return self._not();
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.display {
            stream.extend(quote! {
                impl ::core::fmt::Display for #ty {
                    #[inline(always)]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let raw = #fp_get_raw(*self);
                        return ::core::fmt::Display::fmt(&raw, f);
                    }
                }
            });
        }

        if self.cfg.flags.impl_fmt_binary {
            let it = quote! {
                impl ::core::fmt::Binary for #ty {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let raw = #fp_get_raw(*self);
                        return ::core::fmt::Binary::fmt(&raw, f);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_fmt_octal {
            let it = quote! {
                impl ::core::fmt::Octal for #ty {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let raw = #fp_get_raw(*self);
                        return ::core::fmt::Octal::fmt(&raw, f);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_fmt_hex_lower {
            let it = quote! {
                impl ::core::fmt::LowerHex for #ty {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let raw = #fp_get_raw(*self);
                        return ::core::fmt::LowerHex::fmt(&raw, f);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_fmt_hex_upper {
            let it = quote! {
                impl ::core::fmt::UpperHex for #ty {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let raw = #fp_get_raw(*self);
                        return ::core::fmt::UpperHex::fmt(&raw, f);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_partial_eq {
            let it = quote! {
                #konst impl ::core::cmp::PartialEq for #ty {
                    #[inline(always)]
                    fn eq(
                        &self,
                        rhs: &Self,
                    ) -> bool {
                        let lhs = #fp_get_raw(*self);
                        let rhs = #fp_get_raw(*rhs);
                        return lhs == rhs;
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_partial_ord {
            let it = quote! {
                #konst impl ::core::cmp::PartialOrd for #ty {
                    #[inline(always)]
                    fn partial_cmp(
                        &self,
                        rhs: &Self,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        let lhs = #fp_get_raw(*self);
                        let rhs = #fp_get_raw(*rhs);
                        return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
                    }
                }
            };
            stream.extend(it);
        }

        return Ok(stream);
    }

    pub(crate) fn ekran_bit_access_items(&self) -> TokenStream {
        let fp_get_raw = &self.fp_get_raw;
        let el = &self.el;

        if !self.cfg.flags.bit_access || self.repr.is_signed() {
            return TokenStream::new();
        }

        let bytes = (0..self.repr.bytes())
            .into_iter()
            .map(|i| {
                let name = format_ident!("byte{}", i);
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn #name(self) -> u8 {
                        return ((#fp_get_raw(self) >> (8 * #i)) & (0xFF as #el)) as u8;
                    }
                };
            })
            .merged();

        let words = (0..self.repr.words())
            .into_iter()
            .map(|i| {
                let name = format_ident!("word{}", i);
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn #name(self) -> u16 {
                        return ((#fp_get_raw(self) >> (16 * #i)) & (0xFFFF as #el)) as u16;
                    }
                };
            })
            .merged();

        let dwords = (0..self.repr.dwords())
            .into_iter()
            .map(|i| {
                let name = format_ident!("dword{}", i);
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn #name(self) -> u32 {
                        return ((#fp_get_raw(self) >> (32 * #i)) & (0xFFFFFFFF as #el)) as u32;
                    }
                };
            })
            .merged();

        let qwords = (0..self.repr.qwords())
            .into_iter()
            .map(|i| {
                let name = format_ident!("qword{}", i);
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn #name(self) -> u64 {
                        return ((#fp_get_raw(self) >> (64 * #i)) & (0xFFFFFFFFFFFFFFFF as #el)) as u64;
                    }
                };
            })
            .merged();

        let halved = match self.repr {
            N::U016 => {
                quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn lo8(self) -> u8 {
                        return self.byte0();
                    }

                    #[must_use]
                    #[inline(always)]
                    pub const fn hi8(self) -> u8 {
                        return self.byte1();
                    }
                }
            }

            N::U032 => {
                quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn lo16(self) -> u16 {
                        return self.word0();
                    }

                    #[must_use]
                    #[inline(always)]
                    pub const fn hi16(self) -> u16 {
                        return self.word1();
                    }
                }
            }

            N::U064 => {
                quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn lo32(self) -> u32 {
                        return self.dword0();
                    }

                    #[must_use]
                    #[inline(always)]
                    pub const fn hi32(self) -> u32 {
                        return self.dword1();
                    }
                }
            }

            N::U128 => {
                quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn lo64(self) -> u64 {
                        return self.qword0();
                    }

                    #[must_use]
                    #[inline(always)]
                    pub const fn hi64(self) -> u64 {
                        return self.qword1();
                    }
                }
            }

            // N::USIZ => {}
            _ => TokenStream::new(),
        };

        return quote! {
            #halved

            #bytes

            #words

            #dwords

            #qwords
        };
    }
}
