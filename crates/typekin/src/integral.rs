use crate::{
    runner::{
        self,
        Merged,
        MkErr,
        mk_flags,
    },
    type_friendship::{
        FriendReq,
        FriendshipLevel,
    },
    value_type::N,
};

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    ToTokens,
    format_ident,
    quote,
};
use syn::{
    Fields,
    Path,
    Type,
    parse::{
        Parse,
        ParseStream,
    },
    parse_quote,
};

use std::fmt::{
    Debug,
    Formatter,
};

pub(crate) fn integral(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let cfg = Box::new(syn::parse_macro_input!(attr as IntegralCfg));
    let item = syn::parse_macro_input!(item as syn::ItemStruct);

    return runner::ekran_catching(move || {
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

        let it = Maker::new(ty, el, cfg)?.ekran()?;

        let stream = quote::quote! {
            #[allow(dead_code)]
            #[allow(unused_qualifications)]
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

        pub auto_of_raw: bool,
        pub assertions: bool,
        pub bit_access: bool,

        pub impl_debug: bool,
        pub impl_eq: bool,
        pub impl_fmt_binary: bool,
        pub impl_fmt_hex_lower: bool,
        pub impl_fmt_hex_upper: bool,
        pub impl_fmt_octal: bool,
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

        pub trait_seal: String = "Seal",
        pub trait_friend_make: String = "FriendMake",
        pub trait_friend_math: String = "FriendMath",
        pub trait_friend_bit: String = "FriendBit",
        pub trait_friend_rel: String = "FriendRel",

        pub fn_make: String = "make",

        pub fn_prefix_seal: String = "into_",
        pub fp_unchecked: String = "Self::_unchecked",
    }
}

#[derive(Default, Clone)]
pub(crate) struct IntegralCfg {
    pub(crate) flags: Box<IntegralFlags>,
    pub(crate) konst: bool,
    pub(crate) fn_get_raw: Option<Path>,
    pub(crate) fn_validator: Option<Path>,
    pub(crate) friends: Vec<FriendReq>,
}

impl Debug for IntegralCfg {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "IntegralCfg[int: {:?}, friends: {:?}, fn_get_raw: {}, fn_validator: {}",
            self.flags,
            self.friends,
            self.fn_get_raw
                .as_ref()
                .map(|it| it.to_token_stream().to_string())
                .unwrap_or_default(),
            self.fn_validator
                .as_ref()
                .map(|it| it.to_token_stream().to_string())
                .unwrap_or_default(),
        )
    }
}

impl Parse for IntegralCfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();
        let mut has_konst = false;

        runner::parse_inner_attributes(input, |attr, rest| {
            match attr {
                "konst" => {
                    this.konst = rest.parse::<syn::LitBool>()?.value;
                    has_konst = true;
                }
                "with" => this.flags.parse_from(rest, true)?,
                "without" => this.flags.parse_from(rest, false)?,
                "friends" => this.friends = runner::list(rest)?.collect(),
                "fn_get_raw" => this.fn_get_raw = Some(rest.parse()?),
                "fn_validator" => this.fn_validator = Some(rest.parse()?),
                it if it.starts_with("with_") => {
                    this.flags.parse_from(rest, true)?
                }
                _ => {
                    return Ok(false);
                }
            };

            return Ok(true);
        })?;

        if !has_konst {
            return Err(syn::Error::new(
                input.span(),
                "missing required `konst` argument",
            ));
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

    cfg: Box<IntegralCfg>,
}

impl Maker {
    pub(crate) fn new(
        ty: Ident,
        el: Ident,
        cfg: Box<IntegralCfg>,
    ) -> syn::Result<Self> {
        let cfg = Self::preprocess_cfg(cfg);

        let mut this = Self {
            trait_seal: format_ident!("{}", cfg.flags.trait_seal),
            trait_friend_bit: format_ident!("{}", cfg.flags.trait_friend_bit),
            trait_friend_make: format_ident!("{}", cfg.flags.trait_friend_make),
            trait_friend_math: format_ident!("{}", cfg.flags.trait_friend_math),
            trait_friend_rel: format_ident!("{}", cfg.flags.trait_friend_rel),

            fp_unchecked: syn::parse_str(&cfg.flags.fp_unchecked)?,
            fp_get_raw: cfg
                .fn_get_raw
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

    fn preprocess_cfg(mut cfg: Box<IntegralCfg>) -> Box<IntegralCfg> {
        cfg.friends.sort();

        if cfg.flags.auto_of_raw && cfg.fn_validator.is_some() {
            cfg.flags.auto_of_raw = false;
        }

        return cfg;
    }

    fn fix_friendship(&mut self) {
        let xty = &self.ty;
        let xel = &self.el;

        self.cfg.friends.sort();

        for (ty, to_el, is_always_valid) in [
            (xty, Some(parse_quote! { #xty::raw }), true),
            (
                xel,
                Some(parse_quote! { self }),
                self.cfg.fn_validator.is_none(),
            ),
        ] {
            if let Some(already) =
                self.cfg.friends.iter_mut().find(|it| it.ty.is_ident(ty))
            {
                if is_always_valid && self.cfg.flags.impl_self_friend_make {
                    already.level.insert(FriendshipLevel::Make);
                }

                if self.cfg.flags.impl_self_friend_math_ops {
                    already.level.insert(FriendshipLevel::Math);
                }
                if self.cfg.flags.impl_self_friend_math_rel {
                    already.level.insert(FriendshipLevel::Rel);
                }
                if self.cfg.flags.impl_self_friend_math_bit {
                    already.level.insert(FriendshipLevel::Bit);
                }

                if already.conv.is_none() && to_el.is_some() {
                    already.conv = to_el;
                }
            }
            else {
                let mut levels = vec![];

                if is_always_valid && self.cfg.flags.impl_self_friend_make {
                    levels.push(FriendshipLevel::Make);
                }

                if self.cfg.flags.impl_self_friend_math_ops {
                    levels.push(FriendshipLevel::Math)
                }
                if self.cfg.flags.impl_self_friend_math_rel {
                    levels.push(FriendshipLevel::Rel)
                }
                if self.cfg.flags.impl_self_friend_math_bit {
                    levels.push(FriendshipLevel::Bit)
                }

                let self_friendship = FriendReq::new(
                    Path::from(ty.clone()),
                    levels.into_iter().collect(),
                    to_el,
                );

                self.cfg.friends.push(self_friendship);
            }
        }
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
        let fn_get_raw = &fp_get_raw.segments.last().unwrap().ident;
        let fn_make = format_ident!("{}", self.cfg.flags.fn_make);
        let fp_friend_conv: Path = {
            let trait_seal = &self.trait_seal;
            let fn_conv = &self.fn_conv;
            parse_quote! { #trait_seal::#fn_conv }
        };
        let (konst, bonst, destruct) =
            runner::konst_bonst_and_destruct(self.cfg.konst);

        let mut stream = TokenStream::new();

        if self.cfg.flags.fn_conv_of {
            let what = &self.trait_friend_make;
            let it = quote! {
                #[inline(always)]
                #[allow(private_bounds)]
                pub #konst fn of<T>(
                    it: T,
                ) -> #ty
                where
                    T: #bonst #what #destruct,
                {
                    let this = #fp_friend_conv(&it);
                    return #fp_unchecked(this);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.auto_of_raw {
            let it = quote! {
                #[inline(always)]
                pub #konst fn #fn_make(it: #el) -> #ty {
                    return #ty::of(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_conv_raw {
            let it = quote! {
                #[must_use]
                #[inline(always)]
                pub const fn #fn_get_raw(self) -> #el {
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

        if self.cfg.flags.fn_make_unchecked_try
            && let Some(validator) = &self.cfg.fn_validator
        {
            let it = quote! {
                #[inline(always)]
                pub #konst fn try_make(it: #el) -> Result<Self, #el> {
                    return if #validator(it) {
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
            && self.cfg.fn_validator.is_none()
        {
            let it = quote! {
                #[inline(always)]
                pub const fn try_make(it: #el) -> Result<Self, #el> {
                    return Ok(#fp_unchecked(it));
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_make_unchecked
            && let Some(validator) = &self.cfg.fn_validator
        {
            let it = quote! {
                #[must_use]
                #[inline(always)]
                pub(self) const fn _unchecked(it: #el) -> Self {
                    if #validator(it) {
                        return Self(it);
                    }
                    else {
                        ::core::panic!("invalid value");
                    };
                }
            };
            stream.extend(it);
        }
        else if self.cfg.flags.fn_make_checked
            && self.cfg.fn_validator.is_none()
        {
            let it = quote! {
                #[must_use]
                #[inline(always)]
                pub(self) const fn _unchecked(it: #el) -> Self {
                    return Self(it);
                }
            };
            stream.extend(it);
        }

        return stream;
    }

    pub(crate) fn ekran_impls(&self) -> syn::Result<TokenStream> {
        let ty = &self.ty;
        let el = &self.el;

        let trait_friend_make = &self.trait_friend_make;
        let trait_friend_bit = &self.trait_friend_bit;
        let trait_friend_math = &self.trait_friend_math;
        let trait_friend_rel = &self.trait_friend_rel;
        let trait_seal = &self.trait_seal;

        let fn_conv = &self.fn_conv;
        let fp_friend_conv: Path = parse_quote! { #trait_seal::#fn_conv };
        let fp_get_raw = &self.fp_get_raw;

        let (konst, bonst, destruct) =
            runner::konst_bonst_and_destruct(self.cfg.konst);

        let mut stream = TokenStream::new();

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
                where T: #bonst #trait_friend_bit #destruct
                {
                    #[inline(always)]
                    fn bitand_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(&rhs);
                        *self = self._bitand(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_add {
            let it = quote! {
                #konst impl<T> ::core::ops::AddAssign<T> for #ty
                where T: #bonst #trait_friend_math #destruct
                {
                    #[inline(always)]
                    fn add_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(&rhs);
                        *self = self._add(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_sub {
            let it = quote! {
                #konst impl<T> ::core::ops::SubAssign<T> for #ty
                where T: #bonst #trait_friend_math #destruct
                {
                    #[inline(always)]
                    fn sub_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(&rhs);
                        *self = self._sub(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_mul {
            let it = quote! {
                #konst impl<T> ::core::ops::MulAssign<T> for #ty
                where T: #bonst #trait_friend_math #destruct
                {
                    #[inline(always)]
                    fn mul_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(&rhs);
                        *self = self._mul(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_div {
            let it = quote! {
                #konst impl<T> ::core::ops::DivAssign<T> for #ty
                where T: #bonst #trait_friend_math #destruct
                {
                    #[inline(always)]
                    fn div_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(&rhs);
                        *self = self._div(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_rem {
            let it = quote! {
                #konst impl<T> ::core::ops::RemAssign<T> for #ty
                where T: #bonst #trait_friend_math #destruct
                {
                    #[inline(always)]
                    fn rem_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(&rhs);
                        *self = self._rem(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_or {
            let it = quote! {
                #konst impl<T> ::core::ops::BitOrAssign<T> for #ty
                where T: #bonst #trait_friend_bit #destruct
                {
                    #[inline(always)]
                    fn bitor_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(&rhs);
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
                    panic!("invalid memory layout: #ty(#el) != #el");
                };
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_add {
            let it = quote! {
                #konst impl<T> ::core::ops::Add<T> for #ty
                where T: #bonst #trait_friend_math #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn add(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(&rhs);
                        return self._add(it);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_sub {
            let it = quote! {
                #konst impl<T> ::core::ops::Sub<T> for #ty
                where T: #bonst #trait_friend_math #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn sub(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(&rhs);
                        return self._sub(it);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_mul {
            let it = quote! {
                #konst impl<T> ::core::ops::Mul<T> for #ty
                where T: #bonst #trait_friend_math #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn mul(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(&rhs);
                        return self._mul(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_math_div {
            let it = quote! {
                #konst impl<T> ::core::ops::Div<T> for #ty
                where T: #bonst #trait_friend_math #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn div(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(&rhs);
                        return self._div(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_math_rem {
            let it = quote! {
                #konst impl<T> ::core::ops::Rem<T> for #ty
                where T: #bonst #trait_friend_math #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn rem(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(&rhs);
                        return self._rem(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_math_and {
            let it = quote! {
                #konst impl<T> ::core::ops::BitAnd<T> for #ty
                where T: #bonst #trait_friend_bit #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn bitand(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(&rhs);
                        return self._bitand(it);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_or {
            let it = quote! {
                #konst impl<T> ::core::ops::BitOr<T> for #ty
                where T: #bonst #trait_friend_bit #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn bitor(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(&rhs);
                        return self._bitor(it);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_math_xor {
            let it = quote! {
                #konst impl<T> ::core::ops::BitXor<T> for #ty
                where T: #bonst #trait_friend_bit #destruct
                {
                    type Output = Self;

                    #[inline(always)]
                    fn bitxor(
                        self,
                        rhs: T,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(&rhs);
                        return self._bitxor(it);
                    }
                }

                #konst impl<T> ::core::ops::BitXorAssign<T> for #ty
                where T: #bonst #trait_friend_bit #destruct
                {
                    #[inline(always)]
                    fn bitxor_assign(
                        &mut self,
                        rhs: T,
                    ) {
                        let it = #fp_friend_conv(&rhs);
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

        if self.cfg.flags.impl_friend_seal {
            let it = quote! {
                #konst impl #trait_seal for #ty {
                    #[inline(always)]
                    fn #fn_conv(&self) -> #el {
                        return #fp_get_raw(*self);
                    }
                }
            };
            stream.extend(it);
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

        if self.cfg.flags.impl_friends {
            let seal_impls = self
                .cfg
                .friends
                .iter()
                .filter(|it| !it.ty.is_ident(ty))
                .map(|it| {
                    let whom = &it.ty;
                    let body = match &it.conv {
                        Some(conv) if conv.is_ident("self") => quote! {
                            let it: #el = *self;
                            return it;
                        },
                        Some(conv) => quote! {
                            return #conv(*self);
                        },
                        None => quote! {
                            let it: #el = (*self).into();
                            return it;
                        },
                    };

                    quote! {
                        #konst impl #trait_seal for #whom {
                            #[inline(always)]
                            fn #fn_conv(&self) -> #el {
                                #body
                            }
                        }
                    }
                })
                .merged();
            stream.extend(seal_impls);

            let marker_impls = self
                .cfg
                .friends
                .iter()
                .flat_map(|it| {
                    it.level
                        .iter()
                        .flat_map(|level| level.normalize().into_iter())
                        .map(|level| match level {
                            FriendshipLevel::Make => trait_friend_make,
                            FriendshipLevel::Rel => trait_friend_rel,
                            FriendshipLevel::Bit => trait_friend_bit,
                            FriendshipLevel::Math => trait_friend_math,
                            FriendshipLevel::Full => unreachable!(),
                            FriendshipLevel::None => unreachable!(),
                            FriendshipLevel::XCustom(custom) => {
                                unimplemented!(
                                    "custom friendship level not implemented: {}",
                                    custom,
                                )
                            }
                        })
                        .map(|level| (&it.ty, level))
                })
                .map(|(whom, what)| {
                    quote! {
                        #konst impl #what for #whom {}
                    }
                })
                .merged();
            stream.extend(marker_impls);
        }

        if self.cfg.flags.impl_partial_eq {
            let it = quote! {
                #konst impl<T> ::core::cmp::PartialEq<T> for #ty
                where
                    T: #bonst #trait_friend_rel #destruct,
                {
                    #[inline(always)]
                    fn eq(
                        &self,
                        rhs: &T,
                    ) -> bool {
                        let lhs = #fp_get_raw(*self);
                        let rhs = #fp_friend_conv(rhs);
                        return lhs == rhs;
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_partial_ord {
            let it = quote! {
                #konst impl<T> ::core::cmp::PartialOrd<T> for #ty
                where
                    T: #bonst #trait_friend_rel #destruct,
                {
                    #[inline(always)]
                    fn partial_cmp(
                        &self,
                        rhs: &T,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        let lhs = #fp_get_raw(*self);
                        let rhs = #fp_friend_conv(rhs);
                        return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_friendzone_seal {
            let it = quote! {
                #konst trait #trait_seal {
                    fn #fn_conv(&self) -> #el;
                }

                #konst impl<T> #trait_seal for &T
                where
                    T: #bonst #trait_seal,
                {
                    #[inline(always)]
                    fn #fn_conv(&self) -> #el {
                        return #trait_seal::#fn_conv(&**self);
                    }
                }

                #konst impl<T> #trait_seal for &mut T
                where
                    T: #bonst #trait_seal,
                {
                    #[inline(always)]
                    fn #fn_conv(&self) -> #el {
                        return #trait_seal::#fn_conv(&**self);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_friendzone_friend_make {
            let it = quote! {
                #konst trait #trait_friend_make : #bonst #trait_seal {}

                #konst impl<T> #trait_friend_make for &T
                where
                    T: #bonst #trait_friend_make,
                {}

                #konst impl<T> #trait_friend_make for &mut T
                where
                    T: #bonst #trait_friend_make,
                {}
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_friendzone_friend_math_ops {
            let it = quote! {
                #konst trait #trait_friend_math : #bonst #trait_seal {}

                #konst impl<T> #trait_friend_math for &T
                where
                    T: #bonst #trait_friend_math,
                {}

                #konst impl<T> #trait_friend_math for &mut T
                where
                    T: #bonst #trait_friend_math,
                {}
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_friendzone_friend_math_bit {
            let it = quote! {
                #konst trait #trait_friend_bit : #bonst #trait_seal {}

                #konst impl<T> #trait_friend_bit for &T
                where
                    T: #bonst #trait_friend_bit,
                {}

                #konst impl<T> #trait_friend_bit for &mut T
                where
                    T: #bonst #trait_friend_bit,
                {}
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_friendzone_friend_math_rel {
            let it = quote! {
                #konst trait #trait_friend_rel : #bonst #trait_seal {}

                #konst impl<T> #trait_friend_rel for &T
                where
                    T: #bonst #trait_friend_rel,
                {}

                #konst impl<T> #trait_friend_rel for &mut T
                where
                    T: #bonst #trait_friend_rel,
                {}
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
