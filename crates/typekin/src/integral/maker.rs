use crate::integral::cfg::IntegralCfg;
use crate::runner;
use crate::runner::Merged;
use crate::type_friendship::FriendshipLevel;
use crate::value_type::N;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Path;
use syn::parse_quote;

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
        let it = Self {
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

        return Ok(it);
    }

    pub(crate) fn ekran(&self) -> syn::Result<TokenStream> {
        let impls = self.ekran_impls()?;
        let items = self.ekran_items();

        let ty = &self.ty;
        return Ok(quote::quote! {
            #impls

            impl #ty {
                #items
            }
        });
    }

    pub(crate) fn ekran_items(&self) -> TokenStream {
        let ty = &self.ty;
        let el = &self.el;
        let fp_unchecked = &self.fp_unchecked;
        let fp_get_raw = &self.fp_get_raw;
        let fn_get_raw = &fp_get_raw.segments.last().unwrap().ident;
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
                #konst impl ::core::ops::BitAndAssign<#ty> for #ty {
                    #[inline(always)]
                    fn bitand_assign(
                        &mut self,
                        other: #ty,
                    ) {
                        *self = self._bitand(other.0);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_add {
            let it = quote! {
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

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_sub {
            let it = quote! {
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

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_mul {
            let it = quote! {
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

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_div {
            let it = quote! {
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

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_rem {
            let it = quote! {
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

            stream.extend(it);
        }

        if self.cfg.flags.impl_assign_or {
            let it = quote! {
                #konst impl ::core::ops::BitOrAssign<#ty> for #ty {
                    #[inline(always)]
                    fn bitor_assign(
                        &mut self,
                        other: #ty,
                    ) {
                        *self = self._bitor(other.0);
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

        if self.cfg.flags.impl_friend_make {
            let it = quote! {
                #konst impl #trait_friend_make for #ty {}
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_friend_math_ops {
            let it = quote! {
                #konst impl #trait_friend_math for #ty {}
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_friend_math_bit {
            let it = quote! {
                #konst impl #trait_friend_bit for #ty {}
            };
            stream.extend(it);
        }

        if self.cfg.flags.impl_friend_math_rel {
            let it = quote! {
                #konst impl #trait_friend_rel for #ty {}
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

        if self.cfg.flags.impl_friend {
            let seal_impls = self
                .cfg
                .friends
                .iter()
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
                            },
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

    fn make_impl_range(&self) -> TokenStream {
        unimplemented!("range");
    }
}
