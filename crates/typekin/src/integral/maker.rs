use crate::integral_op::NumBinArg;
use crate::integral_op::NumBinOp;
use crate::integral_op::NumUnaryPrefixOp;
use crate::integral_op::Op;
use crate::integral_token::ident_of_n;
use crate::integral_token::make_fn_into;
use crate::integral_token::make_fn_try_into_checked;
use crate::integral_token::make_fn_try_into_unchecked;
use crate::integral_token::make_impl_into;
use crate::integral_token::make_impl_try_into;
use crate::integral_token::n_of_ident;
use crate::integral_token::to_snake_case;
use crate::integral_types::N;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Path;
use syn::parse_quote;
use syn::spanned::Spanned;
use syn::{ExprPath, ItemStruct};

macro_rules! maybe_quote {
    ($maybe:expr, $($tt:tt)*) => {{
        if $maybe {
            quote::quote! {$($tt)*}
        }
        else {
            TokenStream::new()
        }
    }};
}

// If adding any non-boolean field, then fix the Default implementation
// else the unsafe code is UB.
#[derive(Debug)]
pub(super) struct IntegralFlags {
    pub with_const: bool,
    pub stmt_assertions: bool,

    pub impl_debug: bool,
    pub impl_clone: bool,
    pub impl_copy: bool,
    pub impl_eq: bool,
    pub impl_ord: bool,
    pub impl_partial_eq: bool,
    pub impl_partial_ord: bool,
    pub impl_into: bool,
    pub impl_try_into: bool,
    pub impl_range: bool,

    pub impl_assign_mul: bool,
    pub impl_assign_sub: bool,
    pub impl_assign_add: bool,
    pub impl_assign_div: bool,
    pub impl_assign_rem: bool,
    pub impl_assign_and: bool,
    pub impl_assign_or: bool,
    pub impl_assign_shr: bool,
    pub impl_assign_shl: bool,
    pub impl_sub: bool,
    pub impl_add: bool,
    pub impl_mul: bool,
    pub impl_div: bool,
    pub impl_rem: bool,
    pub impl_and: bool,
    pub impl_xor: bool,
    pub impl_or: bool,
    pub impl_not: bool,
    pub impl_shr: bool,
    pub impl_shl: bool,
    pub impl_friend_seal: bool,
    pub impl_friend_make: bool,
    pub impl_friend_math_ops: bool,
    pub impl_friend_math_bit: bool,
    pub impl_friend_math_rel: bool,
    pub impl_friendzone_seal: bool,
    pub impl_friendzone_friend_make: bool,
    pub impl_friendzone_friend_math_ops: bool,
    pub impl_friendzone_friend_math_bit: bool,
    pub impl_friendzone_friend_math_rel: bool,
    pub impl_friends: bool,

    pub fn_of: bool,
    pub fn_raw: bool,
    pub fn_eq: bool,
    pub fn_cmp: bool,
    pub fn_unchecked_try_make: bool,
    pub fn_unchecked_make: bool,
    pub fn_checked_try_make: bool,
    pub fn_checked_make: bool,
    pub fn_shr: bool,
    pub fn_shl: bool,
    pub fn_and: bool,
    pub fn_or: bool,
    pub fn_xor: bool,
    pub fn_not: bool,
    pub fn_add: bool,
    pub fn_sub: bool,
    pub fn_mul: bool,
    pub fn_div: bool,
    pub fn_rem: bool,
    pub fn_into: bool,
    pub fn_try_into_checked: bool,
    pub fn_try_into_unchecked: bool,
}

impl Default for IntegralFlags {
    fn default() -> Self {
        return Self {
            impl_range: false,
            // ---
            with_const: true,
            stmt_assertions: true,
            impl_debug: true,
            impl_clone: true,
            impl_copy: true,
            impl_eq: true,
            impl_ord: true,
            impl_partial_eq: true,
            impl_partial_ord: true,
            impl_into: true,
            impl_try_into: true,
            impl_assign_mul: true,
            impl_assign_sub: true,
            impl_assign_add: true,
            impl_assign_div: true,
            impl_assign_rem: true,
            impl_assign_and: true,
            impl_assign_or: true,
            impl_assign_shr: true,
            impl_assign_shl: true,
            impl_sub: true,
            impl_add: true,
            impl_mul: true,
            impl_div: true,
            impl_rem: true,
            impl_and: true,
            impl_xor: true,
            impl_or: true,
            impl_not: true,
            impl_shr: true,
            impl_shl: true,
            impl_friend_seal: true,
            impl_friend_make: true,
            impl_friend_math_ops: true,
            impl_friend_math_bit: true,
            impl_friend_math_rel: true,
            impl_friendzone_seal: true,
            impl_friendzone_friend_make: true,
            impl_friendzone_friend_math_ops: true,
            impl_friendzone_friend_math_bit: true,
            impl_friendzone_friend_math_rel: true,
            impl_friends: true,
            fn_of: true,
            fn_raw: true,
            fn_eq: true,
            fn_cmp: true,
            fn_unchecked_try_make: true,
            fn_unchecked_make: true,
            fn_checked_try_make: true,
            fn_checked_make: true,
            fn_shr: true,
            fn_shl: true,
            fn_and: true,
            fn_or: true,
            fn_xor: true,
            fn_not: true,
            fn_add: true,
            fn_sub: true,
            fn_mul: true,
            fn_div: true,
            fn_rem: true,
            fn_into: true,
            fn_try_into_checked: true,
            fn_try_into_unchecked: true,
        };
    }
}

pub(super) struct FriendReq {
    pub(super) ty: Path,
    pub(super) level: FriendshipLevel,
    pub(super) conv: Option<ExprPath>,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum FriendshipLevel {
    None,
    Make,
    Rel,
    Bit,
    Math,
    MathRel,
    MathBit,
    Full,
}

impl FriendshipLevel {
    fn has_make(self) -> bool {
        return !matches!(self, Self::None);
    }

    fn has_rel(self) -> bool {
        return matches!(self, Self::Rel | Self::MathRel | Self::Full);
    }

    fn has_math(self) -> bool {
        return matches!(
            self,
            Self::Math | Self::MathRel | Self::MathBit | Self::Full
        );
    }

    fn has_bit(self) -> bool {
        return matches!(self, Self::Bit | Self::MathBit | Self::Full);
    }
}

pub(super) struct IntegralCfg {
    pub(super) flags: IntegralFlags,
    pub(super) fn_get_raw: ExprPath,
    pub(super) fn_validator: Option<Path>,
    pub(super) friends: Vec<FriendReq>,
    pub(super) output_type: ExprPath,
}

impl Default for IntegralCfg {
    fn default() -> Self {
        return Self {
            flags: IntegralFlags::default(),
            fn_get_raw: parse_quote! { Self::raw },
            fn_validator: None,
            friends: Vec::with_capacity(0),
            output_type: parse_quote!(Self),
        };
    }
}

#[derive(Debug)]
struct Reusable {
    konst: TokenStream,
    bonst: TokenStream,
    destruct: TokenStream,
}

impl Reusable {
    fn new(with_const: bool) -> Self {
        return Self {
            bonst: maybe_quote!(with_const, [const]),
            konst: maybe_quote!(with_const, const),
            destruct: maybe_quote!(with_const, [const] ::core::marker::Destruct),
        };
    }
}

pub(super) struct Generator<'a> {
    sz: N,
    raw_fn_name: Ident,
    item: &'a ItemStruct,
    el: &'a Ident,
    ty: &'a Ident,
    conv: Ident,
    fconv: TokenStream,
    cfg: IntegralCfg,
    tk: Reusable,
}

impl<'a> Generator<'a> {
    pub(super) fn new(
        item: &'a ItemStruct,
        el: &'a Ident,
        mut cfg: IntegralCfg,
    ) -> syn::Result<Self> {
        if cfg.output_type.path.segments.len() == 1
            && cfg.output_type.path.segments[0].ident.to_string() == "Self"
        {
            let output_type = &item.ident;
            cfg.output_type = parse_quote! { #output_type };
        }

        if cfg.fn_get_raw.path.segments.len() != 2
            || cfg.fn_get_raw.path.segments[0].ident != "Self"
        {
            return Err(syn::Error::new(
                item.span(),
                "can only generate raw fn of the form Self::FN_NAME",
            ));
        }

        let conv =
            format_ident!("conv_{}", to_snake_case(&item.ident.to_string()));

        let this = Self {
            fconv: quote! { Seal::#conv },
            sz: n_of_ident(el)?,
            raw_fn_name: cfg.fn_get_raw.path.segments[1].ident.clone(),
            tk: Reusable::new(cfg.flags.with_const),
            ty: &item.ident,
            el,
            conv,
            cfg,
            item,
        };

        return Ok(this);
    }

    pub(super) fn ekran(&self) -> syn::Result<TokenStream> {
        let ty = self.ty;
        let impl_items = self.ekran_impl_items();
        let impls = self.ekran_impls();

        let it = quote::quote! {
            #[allow(dead_code)]
            #[allow(unused_qualifications)]
            const _: () = {
                #impls

                impl #ty {
                    #impl_items
                }
            };
        };
        return Ok(it);
    }

    fn ekran_impl_items(&self) -> TokenStream {
        let mut stream = TokenStream::new();

        if self.cfg.flags.fn_of {
            stream.extend(self.make_impl_fn_of());
        }

        if self.cfg.flags.fn_raw {
            stream.extend(self.make_fn_raw());
        }

        if self.cfg.flags.fn_into {
            stream.extend(self.make_fns_into());
        }

        if self.cfg.flags.fn_try_into_unchecked {
            stream.extend(self.make_fns_try_into_unchecked());
        }

        if self.cfg.flags.fn_try_into_checked {
            stream.extend(self.make_fns_try_into_checked());
        }

        if self.cfg.flags.fn_add {
            stream.extend(self.gen_binary_op(Op::add()));
        }

        if self.cfg.flags.fn_sub {
            stream.extend(self.gen_binary_op(Op::sub()));
        }

        if self.cfg.flags.fn_mul {
            stream.extend(self.gen_binary_op(Op::mul()));
        }

        if self.cfg.flags.fn_div {
            stream.extend(self.gen_binary_op(Op::div()));
        }

        if self.cfg.flags.fn_rem {
            stream.extend(self.gen_binary_op(Op::rem()));
        }

        if self.cfg.flags.fn_xor {
            stream.extend(self.gen_binary_op(Op::xor()));
        }

        if self.cfg.flags.fn_and {
            stream.extend(self.gen_binary_op(Op::and()));
        }

        if self.cfg.flags.fn_or {
            stream.extend(self.gen_binary_op(Op::or()));
        }

        if self.cfg.flags.fn_shr {
            stream.extend(self.gen_binary_op(Op::shr()));
        }

        if self.cfg.flags.fn_shl {
            stream.extend(self.gen_binary_op(Op::shl()));
        }

        if self.cfg.flags.fn_not {
            stream.extend(self.gen_unary_op(Op::not()));
        }

        if self.cfg.flags.fn_eq {
            stream.extend(self.make_eq());
        }

        if self.cfg.flags.fn_cmp {
            stream.extend(self.make_cmp());
        }

        if self.cfg.flags.fn_unchecked_try_make
            && let Some(validator) = &self.cfg.fn_validator
        {
            stream.extend(self.make_fn_try_make_checked(validator));
        }
        else if self.cfg.flags.fn_checked_try_make
            && self.cfg.fn_validator.is_none()
        {
            stream.extend(self.make_try_make_unchecked());
        }

        if self.cfg.flags.fn_unchecked_make
            && let Some(validator) = &self.cfg.fn_validator
        {
            stream.extend(self.make_make_checked(validator));
        }
        else if self.cfg.flags.fn_checked_make
            && self.cfg.fn_validator.is_none()
        {
            stream.extend(self.make_make_unchecked());
        }

        return stream;
    }

    fn ekran_impls(&self) -> TokenStream {
        let mut stream = TokenStream::new();

        if self.cfg.flags.impl_into {
            stream.extend(self.make_impl_into());
        }

        if self.cfg.flags.impl_try_into {
            stream.extend(self.make_impl_try_into());
        }

        if self.cfg.flags.impl_shr {
            stream.extend(self.make_impl_shr());
        }

        if self.cfg.flags.impl_shl {
            stream.extend(self.make_impl_shl());
        }

        if self.cfg.flags.impl_assign_shr {
            stream.extend(self.make_impl_shr_assign());
        }

        if self.cfg.flags.impl_assign_shl {
            stream.extend(self.make_impl_shl_assign());
        }

        if self.cfg.flags.impl_assign_and {
            stream.extend(self.make_impl_and_assign());
        }

        if self.cfg.flags.impl_assign_add {
            stream.extend(self.make_impl_add_assign());
        }

        if self.cfg.flags.impl_assign_sub {
            stream.extend(self.make_impl_sub_assign());
        }

        if self.cfg.flags.impl_assign_mul {
            stream.extend(self.make_impl_mul_assign());
        }

        if self.cfg.flags.impl_assign_div {
            stream.extend(self.make_impl_div_assign());
        }

        if self.cfg.flags.impl_assign_rem {
            stream.extend(self.make_impl_rem_assign());
        }

        if self.cfg.flags.impl_assign_or {
            stream.extend(self.make_impl_or_assign());
        }

        if self.cfg.flags.impl_range {
            stream.extend(self.make_impl_range());
        }

        if self.cfg.flags.impl_debug {
            stream.extend(self.make_impl_debug());
        }

        if self.cfg.flags.impl_copy {
            stream.extend(self.make_impl_copy());
        }

        if self.cfg.flags.impl_clone {
            stream.extend(self.make_impl_clone());
        }

        if self.cfg.flags.impl_eq {
            stream.extend(self.make_impl_eq());
        }

        if self.cfg.flags.impl_ord {
            stream.extend(self.make_impl_ord());
        }

        if self.cfg.flags.stmt_assertions {
            stream.extend(self.make_stmt_assertions());
        }

        if self.cfg.flags.impl_partial_eq {
            stream.extend(self.make_impl_partial_eq());
        }

        if self.cfg.flags.impl_partial_ord {
            stream.extend(self.make_impl_partial_ord());
        }

        if self.cfg.flags.impl_add {
            stream.extend(self.make_impl_add());
        }

        if self.cfg.flags.impl_sub {
            stream.extend(self.make_impl_sub());
        }

        if self.cfg.flags.impl_mul {
            stream.extend(self.make_impl_mul());
        }

        if self.cfg.flags.impl_div {
            stream.extend(self.make_impl_div());
        }

        if self.cfg.flags.impl_rem {
            stream.extend(self.make_impl_rem());
        }

        if self.cfg.flags.impl_and {
            stream.extend(self.make_impl_and());
        }

        if self.cfg.flags.impl_or {
            stream.extend(self.make_impl_or());
        }

        if self.cfg.flags.impl_xor {
            stream.extend(self.make_impl_xor());
        }

        if self.cfg.flags.impl_not {
            stream.extend(self.make_impl_not());
        }

        if self.cfg.flags.impl_friendzone_seal {
            stream.extend(self.make_friendzone_seal());
        }

        if self.cfg.flags.impl_friendzone_friend_make {
            stream.extend(self.make_friendzone_friend_make());
        }

        if self.cfg.flags.impl_friendzone_friend_math_ops {
            stream.extend(self.make_friendzone_friend_math_ops());
        }

        if self.cfg.flags.impl_friendzone_friend_math_bit {
            stream.extend(self.make_friendzone_friend_math_bit());
        }

        if self.cfg.flags.impl_friendzone_friend_math_rel {
            stream.extend(self.make_friendzone_friend_math_rel());
        }

        if self.cfg.flags.impl_friend_seal {
            stream.extend(self.make_impl_friend_seal());
        }

        if self.cfg.flags.impl_friend_make {
            stream.extend(self.make_impl_friend_make());
        }

        if self.cfg.flags.impl_friend_math_ops {
            stream.extend(self.make_impl_friend_math_ops());
        }

        if self.cfg.flags.impl_friend_math_bit {
            stream.extend(self.make_impl_friend_math_bit());
        }

        if self.cfg.flags.impl_friend_math_rel {
            stream.extend(self.make_impl_friend_math_rel());
        }

        if self.cfg.flags.impl_friends {
            stream.extend(self.make_impl_friends());
        }

        return stream;
    }

    fn gen_binary_op(
        &self,
        it: NumBinOp,
    ) -> TokenStream {
        let raw = &self.cfg.fn_get_raw;
        let fn_name = format_ident!("_{}", it.op.name());
        let konst = &self.tk.konst;
        let el = match it.arg {
            NumBinArg::Variable => self.el,
            NumBinArg::Predefined(n) => &ident_of_n(n),
        };
        let op = it.op.stream();

        return quote! {
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) #konst fn #fn_name(
                self,
                it: #el,
            ) -> Self {
                let this = #raw(self);
                let result = this #op it;
                return Self::_make(result);
            }
        };
    }

    fn gen_unary_op(
        &self,
        unary: NumUnaryPrefixOp,
    ) -> TokenStream {
        let raw = &self.cfg.fn_get_raw;
        let fn_name = format_ident!("_{}", unary.op.name());
        let op = unary.op.stream();

        return quote! {
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn #fn_name(
                self,
            ) -> Self {
                let this = #raw(self);
                let result = #op this;
                return Self::_make(result);
            }
        };
    }

    fn gen_friend(
        &self,
        cty: &FriendReq,
    ) -> TokenStream {
        let el = self.el;
        let konst = &self.tk.konst;
        let conv = &self.conv;
        let friend = &cty.ty;
        let level = cty.level;

        let to_el = match &cty.conv {
            None => quote! { *self },
            Some(it) => match it.path.is_ident("self") {
                true => quote! { *self },
                false => quote! { #it(self) },
            },
        };

        let mut friendships = TokenStream::new();

        if level.has_math() {
            friendships.extend(quote! {
                #konst impl FriendMathOps for #friend { }
            });
        }
        if level.has_bit() {
            friendships.extend(quote! {
                #konst impl FriendMathBit for #friend { }
            });
        }
        if level.has_rel() {
            friendships.extend(quote! {
                #konst impl FriendMathRel for #friend { }
            });
        }
        if level.has_make() {
            friendships.extend(quote! {
                #konst impl FriendMake for #friend { }
            });
        }

        return quote! {
            #friendships

            #konst impl Seal for #friend {
                #[inline(always)]
                fn #conv(&self) -> #el {
                    let it: #el = #to_el;
                    return it;
                }
            }
        };
    }
}

// Pub.
impl Generator<'_> {
    fn make_impl_add(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;
        let destruct = &self.tk.destruct;
        let fconv = &self.fconv;

        return quote! {
            #konst impl<T> ::core::ops::Add<T> for #ty
            where
                T: #bonst FriendMathOps + #destruct,
            {
                type Output = Self;

                #[inline(always)]
                fn add(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._add(that)
                }
            }
        };
    }

    fn make_impl_sub(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;
        let destruct = &self.tk.destruct;
        let fconv = &self.fconv;

        return quote! {
            #konst impl<T> ::core::ops::Sub<T> for #ty
            where
                T: #bonst FriendMathOps + #destruct,
            {
                type Output = Self;

                #[inline(always)]
                fn sub(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._sub(that)
                }
            }
        };
    }

    fn make_impl_mul(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;
        let destruct = &self.tk.destruct;
        let fconv = &self.fconv;

        return quote! {
            #konst impl<T> ::core::ops::Mul<T> for #ty
            where
                T: #bonst FriendMathOps + #destruct,
            {
                type Output = Self;

                #[inline(always)]
                fn mul(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._mul(that)
                }
            }
        };
    }

    fn make_impl_div(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;
        let destruct = &self.tk.destruct;
        let fconv = &self.fconv;

        return quote! {
            #konst impl<T> ::core::ops::Div<T> for #ty
            where
                T: #bonst FriendMathOps + #destruct,
            {
                type Output = Self;

                #[inline(always)]
                fn div(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._div(that)
                }
            }

        };
    }

    fn make_impl_rem(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;
        let destruct = &self.tk.destruct;
        let fconv = &self.fconv;

        return quote! {
            #konst impl<T> ::core::ops::Rem<T> for #ty
            where
                T: #bonst FriendMathOps + #destruct,
            {
                type Output = Self;

                #[inline(always)]
                fn rem(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._rem(that)
                }
            }

        };
    }

    fn make_impl_and(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;
        let destruct = &self.tk.destruct;
        let fconv = &self.fconv;

        return quote! {
            #konst impl<T> ::core::ops::BitAnd<T> for #ty
            where
                T: #bonst FriendMathBit + #destruct,
            {
                type Output = Self;

                #[inline(always)]
                fn bitand(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._and(that)
                }
            }

        };
    }

    fn make_impl_or(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;
        let destruct = &self.tk.destruct;
        let fconv = &self.fconv;

        return quote! {
            #konst impl<T> ::core::ops::BitOr<T> for #ty
            where
                T: #bonst FriendMathBit + #destruct,
            {
                type Output = Self;

                #[inline(always)]
                fn bitor(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._or(that)
                }
            }

        };
    }

    fn make_impl_xor(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;
        let destruct = &self.tk.destruct;
        let fconv = &self.fconv;

        return quote! {
            #konst impl<T> ::core::ops::BitXor<T> for #ty
            where
                T: #bonst FriendMathBit + #destruct,
            {
                type Output = Self;

                #[inline(always)]
                fn bitxor(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._xor(that)
                }
            }
        };
    }

    fn make_impl_not(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let out = &self.cfg.output_type;

        return quote! {

            #konst impl ::core::ops::Not for #ty {
                type Output = #out;

                #[inline(always)]
                fn not(self) -> Self::Output {
                    return self._not();
                }
            }
        };
    }

    fn make_impl_debug(&self) -> TokenStream {
        let ty = self.ty;
        let fmt_str = format!("{}({})", ty, "{}");

        return quote! {
            impl ::core::fmt::Debug for #ty {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    ::core::write!(f, #fmt_str, self.0)
                }
            }
        };
    }

    fn make_impl_copy(&self) -> TokenStream {
        let ty = self.ty;

        return quote! {
            impl ::core::marker::Copy for #ty {}
        };
    }

    fn make_impl_clone(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl ::core::clone::Clone for #ty {
                #[inline(always)]
                fn clone(&self) -> Self {
                    Self(::core::clone::Clone::clone(&self.0))
                }
            }
        };
    }

    fn make_impl_eq(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl ::core::cmp::Eq for #ty {}
        };
    }

    fn make_impl_ord(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl ::core::cmp::Ord for #ty {
                #[inline(always)]
                fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                    return self.partial_cmp(other).unwrap();
                }
            }
        };
    }

    fn make_impl_into(&self) -> TokenStream {
        let items = N::items()
            .iter()
            .filter(|it| self.sz.fits_in(it))
            .map(|it| {
                make_impl_into(self.sz, *it, &self.item.ident, &self.tk.konst)
            })
            .collect::<Vec<_>>();

        let mut merged = TokenStream::new();
        merged.extend(items);

        return merged;
    }

    fn make_impl_try_into(&self) -> TokenStream {
        let items = N::items()
            .iter()
            .filter(|it| !self.sz.fits_in(it))
            .map(|it| make_impl_try_into(*it, &self.item.ident, &self.tk.konst))
            .collect::<Vec<_>>();

        let mut merged = TokenStream::new();
        merged.extend(items);

        return merged;
    }
}

// Anon.
impl Generator<'_> {
    fn make_impl_partial_eq(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;
        let destruct = &self.tk.destruct;
        let fconv = &self.fconv;

        return quote! {
            #konst impl<T> ::core::cmp::PartialEq<T> for #ty
            where
                T: #bonst FriendMathRel + #destruct,
            {
                #[inline(always)]
                fn eq(
                    &self,
                    rhs: &T,
                ) -> bool {
                    let that = #fconv(rhs);
                    return self.raw() == that;
                }
            }
        };
    }

    fn make_impl_partial_ord(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;
        let destruct = &self.tk.destruct;
        let fconv = &self.fconv;

        return quote! {
            #konst impl<T> ::core::cmp::PartialOrd<T> for #ty
            where
                T: #bonst PartialEq<#ty>
                    + #bonst FriendMathRel
                    + #destruct
            {
                #[inline(always)]
                fn partial_cmp(
                    &self,
                    rhs: &T,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let that = #fconv(rhs);
                    return self.raw().partial_cmp(&that);
                }
            }
        };
    }

    fn make_friendzone_seal(&self) -> TokenStream {
        let el = self.el;
        let konst = &self.tk.konst;
        let conv = &self.conv;

        return quote! {
            #konst trait Seal {
                #[must_use]
                fn #conv(&self) -> #el;
            }
        };
    }

    fn make_friendzone_friend_make(&self) -> TokenStream {
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;

        return quote! {
            #konst trait FriendMake:    #bonst Seal {}
        };
    }

    fn make_friendzone_friend_math_ops(&self) -> TokenStream {
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;

        return quote! {
            #konst trait FriendMathOps: #bonst Seal {}
        };
    }

    fn make_friendzone_friend_math_rel(&self) -> TokenStream {
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;

        return quote! {
            #konst trait FriendMathRel: #bonst Seal {}
        };
    }

    fn make_friendzone_friend_math_bit(&self) -> TokenStream {
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;

        return quote! {
            #konst trait FriendMathBit: #bonst Seal {}
        };
    }

    fn make_impl_friend_seal(&self) -> TokenStream {
        let el = self.el;
        let ty = self.ty;
        let konst = &self.tk.konst;
        let conv = &self.conv;
        let raw = &self.cfg.fn_get_raw;

        return quote! {
            #konst impl Seal for #ty {
                #[inline(always)]
                fn #conv(&self) -> #el {
                    return #raw(*self);
                }
            }
        };
    }

    fn make_impl_friend_make(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl FriendMake    for #ty {}
        };
    }

    fn make_impl_friend_math_ops(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl FriendMathOps for #ty {}
        };
    }

    fn make_impl_friend_math_rel(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl FriendMathRel for #ty {}
        };
    }

    fn make_impl_friend_math_bit(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl FriendMathBit for #ty {}
        };
    }

    fn make_impl_fn_of(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;
        let destruct = &self.tk.destruct;
        let fconv = &self.fconv;

        return quote! {
            #[inline(always)]
            #[allow(private_bounds)]
            pub #konst fn of<T, B>(
                it: B,
            ) -> #ty
            where
                T: #bonst FriendMake,
                B: #bonst ::core::borrow::Borrow<T> + #destruct,
            {
                let this = #fconv(it.borrow());
                return Self::_make(this);
            }
        };
    }

    fn make_impl_friends(&self) -> TokenStream {
        return self
            .cfg
            .friends
            .iter()
            .map(|it| self.gen_friend(it))
            .reduce(|mut a, b| {
                a.extend(b);
                return a;
            })
            .unwrap_or_else(TokenStream::new);
    }

    fn make_eq(&self) -> TokenStream {
        let raw = &self.cfg.fn_get_raw;
        let el = self.el;

        return quote! {
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn _eq(
                self,
                it: #el,
            ) -> bool {
                let this = #raw(self);
                let result = this == it;
                return result;
            }
        };
    }

    fn make_cmp(&self) -> TokenStream {
        let raw = &self.cfg.fn_get_raw;
        let el = self.el;
        let konst = &self.tk.konst;

        return quote! {
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) #konst fn _cmp(
                self,
                it: #el,
            ) -> ::core::cmp::Ordering {
                let this = #raw(self);
                let result = ::core::cmp::PartialOrd::partial_cmp(&this, &it);
                return result.unwrap();
            }
        };
    }

    fn make_impl_shr(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let out = &self.cfg.output_type;

        return quote::quote! {

            #konst impl ::core::ops::Shr<usize> for #ty {
                type Output = #out;

                #[inline(always)]
                fn shr(
                    self,
                    rhs: usize,
                ) -> Self::Output {
                    return self._shr(rhs);
                }
            }

        };
    }

    fn make_impl_shl(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let out = &self.cfg.output_type;

        return quote::quote! {

            #konst impl ::core::ops::Shl<usize> for #ty {
                type Output = #out;

                #[inline(always)]
                fn shl(
                    self,
                    rhs: usize,
                ) -> Self::Output {
                    return self._shl(rhs);
                }
            }

        };
    }

    fn make_impl_shr_assign(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
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
    }

    fn make_impl_shl_assign(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
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
    }

    fn make_impl_and_assign(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl ::core::ops::BitAndAssign<#ty> for #ty {
                #[inline(always)]
                fn bitand_assign(
                    &mut self,
                    other: #ty,
                ) {
                    *self = self._and(other.0);
                }
            }
        };
    }

    fn make_impl_or_assign(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl ::core::ops::BitOrAssign<#ty> for #ty {
                #[inline(always)]
                fn bitor_assign(
                    &mut self,
                    other: #ty,
                ) {
                    *self = self._or(other.0);
                }
            }
        };
    }

    fn make_impl_add_assign(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl ::core::ops::AddAssign<#ty> for #ty {
                #[inline(always)]
                fn add_assign(
                    &mut self,
                    other: #ty,
                ) {
                    *self = self._add(other.0);
                }
            }
        };
    }

    fn make_impl_sub_assign(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl ::core::ops::SubAssign<#ty> for #ty {
                #[inline(always)]
                fn sub_assign(
                    &mut self,
                    other: #ty,
                ) {
                    *self = self._sub(other.0);
                }
            }
        };
    }

    fn make_impl_mul_assign(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl ::core::ops::MulAssign<#ty> for #ty {
                #[inline(always)]
                fn mul_assign(
                    &mut self,
                    other: #ty,
                ) {
                    *self = self._mul(other.0);
                }
            }
        };
    }

    fn make_impl_div_assign(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl ::core::ops::DivAssign<#ty> for #ty {
                #[inline(always)]
                fn div_assign(
                    &mut self,
                    other: #ty,
                ) {
                    *self = self._div(other.0);
                }
            }
        };
    }

    fn make_impl_rem_assign(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;

        return quote! {
            #konst impl ::core::ops::RemAssign<#ty> for #ty {
                #[inline(always)]
                fn rem_assign(
                    &mut self,
                    other: #ty,
                ) {
                    *self = self._rem(other.0);
                }
            }
        };
    }

    fn make_impl_range(&self) -> TokenStream {
        unimplemented!("range");
    }

    // =========================================================================

    fn make_fn_raw(&self) -> TokenStream {
        let raw_fn_name = &self.raw_fn_name;
        let el = self.el;

        return quote! {
            #[must_use]
            #[inline(always)]
            pub const fn #raw_fn_name(self) -> #el {
                return self.0;
            }
        };
    }

    fn make_fns_into(&self) -> TokenStream {
        let items = N::items()
            .iter()
            .map(|it| make_fn_into(self.sz, *it))
            .collect::<Vec<_>>();

        let mut merged = TokenStream::new();
        merged.extend(items);

        return merged;
    }

    fn make_fns_try_into_checked(&self) -> TokenStream {
        let items = N::items()
            .iter()
            .map(|it| make_fn_try_into_checked(self.sz, *it))
            .collect::<Vec<_>>();

        let mut merged = TokenStream::new();
        merged.extend(items);

        return merged;
    }

    fn make_fns_try_into_unchecked(&self) -> TokenStream {
        let items = N::items()
            .iter()
            .map(|it| make_fn_try_into_unchecked(self.sz, *it))
            .collect::<Vec<_>>();

        let mut merged = TokenStream::new();
        merged.extend(items);
        return merged;
    }

    fn make_fn_try_make_checked(
        &self,
        validator: &Path,
    ) -> TokenStream {
        let el = self.el;
        let konst = &self.tk.konst;

        return quote! {
            #[inline(always)]
            pub #konst fn try_make(it: #el) -> Result<Self, #el> {
                return if #validator(it) {
                    return Self(it);
                }
                else {
                    ::core::result::Result::Err(it)
                };
            }
        };
    }

    fn make_make_checked(
        &self,
        validator: &Path,
    ) -> TokenStream {
        let el = self.el;

        return quote! {
            #[must_use]
            #[inline(always)]
            pub(self) const fn _make(it: #el) -> Self {
                if #validator(it) {
                    return Self(it);
                }
                else {
                    ::core::panic!("invalid value");
                };
            }

        };
    }

    fn make_try_make_unchecked(&self) -> TokenStream {
        let el = self.el;

        return quote! {
            #[inline(always)]
            pub const fn try_make(it: #el) -> Result<Self, #el> {
                return Ok(Self::_make(it));
            }
        };
    }

    fn make_make_unchecked(&self) -> TokenStream {
        let el = self.el;

        return quote! {
            #[must_use]
            #[inline(always)]
            pub(self) const fn _make(it: #el) -> Self {
                return Self(it);
            }
        };
    }

    fn make_stmt_assertions(&self) -> TokenStream {
        let ty = self.ty;
        let el = self.el;

        return quote! {
            if !(size_of::<#ty>() == size_of::<#el>()) {
                panic!("invalid memory layout: #ty(#el) != #el");
            };
        };
    }
}
