use crate::ToVec;
use crate::integral_op::NumBinArg;
use crate::integral_op::NumBinOp;
use crate::integral_op::NumUnaryPrefixOp;
use crate::integral_op::Op;
use crate::integral_types::N;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::ItemStruct;
use syn::Path;
use syn::parse_quote;
use syn::spanned::Spanned;

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

#[derive(Debug)]
pub(crate) struct Reusable {
    konst: TokenStream,
    bonst: TokenStream,
    destruct: TokenStream,
}

impl Reusable {
    fn new(with_konst: bool) -> Self {
        if(!with_konst) {
            panic!("fuck");
        }
        return match with_konst {
            true => Self {
                konst: TokenStream::new(),
                bonst: TokenStream::new(),
                destruct: TokenStream::new(),
            },
            false => Self {
                konst: quote! { const },
                bonst: quote! { [const] },
                destruct: quote! { ::core::marker::Destruct },
            },
        };
    }
}

pub(crate) struct Generator<'a> {
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
    pub(crate) fn new(
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
            tk: Reusable::new(cfg.flags.with_konst),
            ty: &item.ident,
            el,
            conv,
            cfg,
            item,
        };

        return Ok(this);
    }

    pub(crate) fn ekran(&self) -> syn::Result<TokenStream> {
        let ty = self.ty;

        let items = self.ekran_items();
        let impls = self.ekran_impls();
        let anonymous = self.ekran_anon();

        let it = quote::quote! {
            impl #ty {
                #items
            }

            #impls

            const _: () = { #anonymous };
        };
        return Ok(it);
    }

    fn ekran_anon(&self) -> TokenStream {
        let mut stream = TokenStream::new();

        if self.cfg.flags.checks_stmt {
            stream.extend(self.make_stmt_assertions());
        }

        stream.extend(self.make_derive());
        stream.extend(self.make_math());
        stream.extend(self.make_friends());
        stream.extend(self.make_friendzone());

        return stream;
    }

    fn ekran_items(&self) -> TokenStream {
        let mut stream = TokenStream::new();

        if self.cfg.flags.common_fns {
            stream.extend(self.make_fn_raw());
        }

        if self.cfg.flags.default_math_ops {
            stream.extend(self.make_fn_private_bin(Op::add()));
            stream.extend(self.make_fn_private_bin(Op::sub()));
            stream.extend(self.make_fn_private_bin(Op::mul()));
            stream.extend(self.make_fn_private_bin(Op::div()));
            stream.extend(self.make_fn_private_bin(Op::rem()));
        }
        if self.cfg.flags.default_math_bit {
            stream.extend(self.make_fn_private_bin(Op::xor()));
            stream.extend(self.make_fn_private_bin(Op::and()));
            stream.extend(self.make_fn_private_bin(Op::or()));
            stream.extend(self.make_fn_private_bin(Op::shr()));
            stream.extend(self.make_fn_private_bin(Op::shl()));
            stream.extend(self.make_fn_private_unary_prefix(Op::not()));
        }
        if self.cfg.flags.default_math_rel {
            stream.extend(self.make_fn_private_rel());
        }

        if self.cfg.flags.cast_fns {
            stream.extend(self.make_cast_fn());
        }

        if self.cfg.flags.constructor_fns {
            stream.extend(self.make_constructor());
        }

        return stream;
    }

    fn ekran_impls(&self) -> TokenStream {
        let mut stream = TokenStream::new();

        if self.cfg.flags.not_impl {
            stream.extend(self.make_impl_not());
        };

        if self.cfg.flags.cast_impl {
            stream.extend(self.make_cast_impl());
        }

        if self.cfg.flags.shr_impl {
            stream.extend(self.make_impl_shr());
        }

        if self.cfg.flags.shl_impl {
            stream.extend(self.make_impl_shl());
        }

        if self.cfg.flags.shr_assign_impl {
            stream.extend(self.make_impl_shr_assign());
        }

        if self.cfg.flags.shl_assign_impl {
            stream.extend(self.make_impl_shl_assign());
        }

        if self.cfg.flags.and_assign_impl {
            stream.extend(self.make_impl_and_assign());
        }

        if self.cfg.flags.add_assign_impl {
            stream.extend(self.make_impl_add_assign());
        }

        if self.cfg.flags.sub_assign_impl {
            stream.extend(self.make_impl_sub_assign());
        }

        if self.cfg.flags.mul_assign_impl {
            stream.extend(self.make_impl_mul_assign());
        }

        if self.cfg.flags.div_assign_impl {
            stream.extend(self.make_impl_div_assign());
        }

        if self.cfg.flags.or_assign_impl {
            stream.extend(self.make_impl_or_assign());
        }

        if self.cfg.flags.display_impl {
            stream.extend(self.make_impl_display());
        }

        if self.cfg.flags.range_impl {
            stream.extend(self.make_impl_range());
        }

        return stream;
    }
}

// Anon.
impl Generator<'_> {
    fn make_math(&self) -> TokenStream {
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
                fn eq(
                    &self,
                    rhs: &T,
                ) -> bool {
                    let that = #fconv(rhs);
                    return self.raw() == that;
                }
            }

            #konst impl<T> ::core::cmp::PartialOrd<T> for #ty
            where
                T: #bonst PartialEq<#ty>
                    + #bonst FriendMathRel
                    + #destruct
            {
                fn partial_cmp(
                    &self,
                    rhs: &T,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let that = #fconv(rhs);
                    return self.raw().partial_cmp(&that);
                }
            }

            #konst impl<T> ::core::ops::Add<T> for #ty
            where
                T: #bonst FriendMathOps + #destruct,
            {
                type Output = Self;

                fn add(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._add(that)
                }
            }

            #konst impl<T> ::core::ops::Sub<T> for #ty
            where
                T: #bonst FriendMathOps + #destruct,
            {
                type Output = Self;

                fn sub(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._sub(that)
                }
            }

            #konst impl<T> ::core::ops::Mul<T> for #ty
            where
                T: #bonst FriendMathOps + #destruct,
            {
                type Output = Self;

                fn mul(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._mul(that)
                }
            }

            #konst impl<T> ::core::ops::Div<T> for #ty
            where
                T: #bonst FriendMathOps + #destruct,
            {
                type Output = Self;

                fn div(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._div(that)
                }
            }

            #konst impl<T> ::core::ops::Rem<T> for #ty
            where
                T: #bonst FriendMathOps + #destruct,
            {
                type Output = Self;

                fn rem(
                    self,
                    rhs: T,
                ) -> Self::Output {
                    let that = #fconv(&rhs);
                    return self._rem(that)
                }
            }

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
}

// AnonFriends.
impl Generator<'_> {
    fn make_friendzone(&self) -> TokenStream {
        let el = self.el;
        let ty = self.ty;
        let raw = &self.cfg.fn_get_raw;

        let konst = &self.tk.konst;
        let bonst = &self.tk.bonst;
        let destruct = &self.tk.destruct;

        let conv = &self.conv;
        let fconv = &self.fconv;

        return quote! {
            #konst trait Seal {
                fn #conv(&self) -> #el;
            }

            #konst trait FriendMake:    #bonst Seal {}
            #konst trait FriendMathOps: #bonst Seal {}
            #konst trait FriendMathRel: #bonst Seal {}
            #konst trait FriendMathBit: #bonst Seal {}

            #konst impl Seal for #ty {
                fn #conv(&self) -> #el {
                    return #raw(*self);
                }
            }
            #konst impl FriendMake    for #ty {}
            #konst impl FriendMathOps for #ty {}
            #konst impl FriendMathRel for #ty {}
            #konst impl FriendMathBit for #ty {}

            impl #ty {
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
            }

        };
    }

    fn make_friends(&self) -> TokenStream {
        return self
            .cfg
            .friends
            .iter()
            .map(|it| self.make_friend(it))
            .reduce(|mut a, b| {
                a.extend(b);
                return a;
            })
            .unwrap_or_else(TokenStream::new);
    }

    fn make_friend(
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
                fn #conv(&self) -> #el {
                    let it: #el = #to_el;
                    return it;
                }
            }
        };
    }
}

// Self
impl Generator<'_> {
    fn make_derive(&self) -> TokenStream {
        let ty = self.ty;
        let konst = &self.tk.konst;
        let fmt_str = format!("{}({})", ty, "{}");

        return quote! {
            impl ::core::marker::Copy for #ty {}

            #konst impl ::core::cmp::Eq for #ty {}

            #konst impl ::core::cmp::Ord for #ty {
                fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                    return self.partial_cmp(other).unwrap();
                }
            }

            #konst impl ::core::clone::Clone for #ty {
                fn clone(&self) -> Self {
                    Self(::core::clone::Clone::clone(&self.0))
                }
            }

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

    fn make_fn_private_bin(
        &self,
        it: NumBinOp,
    ) -> TokenStream {
        let raw = &self.cfg.fn_get_raw;
        let fn_name = format_ident!("_{}", it.op.name());
        let konst = self.tk.konst;
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

    fn make_fn_private_rel(&self) -> TokenStream {
        let raw = &self.cfg.fn_get_raw;
        let el = self.el;
        let konst = self.tk.konst;

        return quote! {
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) #konst fn _eq(
                self,
                it: #el,
            ) -> bool {
                let this = #raw(self);
                let result = this == it;
                return result;
            }

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

    fn make_fn_private_unary_prefix(
        &self,
        unary: NumUnaryPrefixOp,
    ) -> TokenStream {
        let raw = &self.cfg.fn_get_raw;
        let fn_name = format_ident!("_{}", unary.op.name());
        let konst = self.tk.konst;
        let op = unary.op.stream();

        return quote! {
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) #konst fn #fn_name(
                self,
            ) -> Self {
                let this = #raw(self);
                let result = #op this;
                return Self::_make(result);
            }
        };
    }
}

// Impl.
impl Generator<'_> {
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

    fn make_impl_range(&self) -> TokenStream {
        unimplemented!("range");
    }

    fn make_impl_display(&self) -> TokenStream {
        let ty = self.ty;
        let el = self.el;
        let raw = &self.cfg.fn_get_raw;
        let fmt = format!("{}(", ty.to_string());

        return quote! {

            impl ::core::fmt::Display for #ty {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.write_str(#fmt)?;
                    <#el as ::core::fmt::Display>::fmt(&#raw(*self), f)?;
                    f.write_str(")")
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
}

// Cast.
impl Generator<'_> {
    fn make_fn_raw(&self) -> TokenStream {
        let raw_fn_name = &self.raw_fn_name;
        let el = self.el;
        let konst = self.tk.konst;

        return quote! {

            #[must_use]
            #[inline(always)]
            pub #konst fn #raw_fn_name(self) -> #el {
                return self.0;
            }
        };
    }

    fn make_cast_fn(&self) -> TokenStream {
        let into = N::items()
            .iter()
            .map(|it| make_fn_into(self.sz, *it))
            .collect::<Vec<_>>();

        let try_into_checked = N::items()
            .iter()
            .map(|it| make_fn_try_into_checked(self.sz, *it))
            .collect::<Vec<_>>();

        let try_into_unchecked = N::items()
            .iter()
            .map(|it| make_fn_try_into_unchecked(self.sz, *it))
            .collect::<Vec<_>>();

        let mut items = TokenStream::new();
        items.extend(into);
        items.extend(try_into_unchecked);
        items.extend(try_into_checked);
        return items;
    }

    fn make_constructor_checked(
        &self,
        validator: &Path,
    ) -> TokenStream {
        let el = self.el;
        let konst = self.tk.konst;

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

            #[must_use]
            #[inline(always)]
            pub(self) #konst fn _make(it: #el) -> Self {
                if #validator(it) {
                    return Self(it);
                }
                else {
                    ::core::panic!("invalid value");
                };
            }

        };
    }

    fn make_constructor_unchecked(&self) -> TokenStream {
        let el = self.el;

        return quote! {
            #[inline(always)]
            pub const fn try_make(it: #el) -> Result<Self, #el> {
                return Ok(Self::_make(it));
            }

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

    fn make_cast_impl(&self) -> TokenStream {
        let into = N::items()
            .iter()
            .filter(|it| self.sz.fits_in(it))
            .map(|it| {
                make_impl_into(self.sz, *it, &self.item.ident, &self.tk.konst)
            })
            .collect::<Vec<_>>();

        let try_into = N::items()
            .iter()
            .filter(|it| !self.sz.fits_in(it))
            .map(|it| make_impl_try_into(*it, &self.item.ident, &self.tk.konst))
            .collect::<Vec<_>>();

        let mut items = TokenStream::new();
        items.extend(into);
        items.extend(try_into);

        return items;
    }
}
