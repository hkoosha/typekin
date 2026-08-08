use crate::runner;
use crate::runner::Handy;
use crate::runner::MkErr;
use crate::runner::mk_flags;
use crate::type_friendship::{FriendReq, FriendshipLevel};
use crate::value_ops::BinOp;
use crate::value_ops::UnaryOp;
use crate::value_type::N;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::ExprPath;
use syn::Path;

mk_flags! {
    #[derive(Debug, Clone)]
    pub(crate) struct IntegralFlags {
        pub impl_range: bool = false,

        pub konst: bool = true,
        pub assertions: bool = true,

        pub impl_debug: bool = true,
        pub impl_eq: bool = true,
        pub impl_fmt_binary: bool = true,
        pub impl_fmt_hex_lower: bool = true,
        pub impl_fmt_hex_upper: bool = true,
        pub impl_fmt_octal: bool = true,
        pub impl_into: bool = true,
        pub impl_ord: bool = true,
        pub impl_partial_eq: bool = true,
        pub impl_partial_ord: bool = true,
        pub impl_try_into: bool = true,

        pub impl_assign_add: bool = true,
        pub impl_assign_and: bool = true,
        pub impl_assign_div: bool = true,
        pub impl_assign_mul: bool = true,
        pub impl_assign_or: bool = true,
        pub impl_assign_rem: bool = true,
        pub impl_assign_shl: bool = true,
        pub impl_assign_shr: bool = true,
        pub impl_assign_sub: bool = true,
        pub impl_math_add: bool = true,
        pub impl_math_and: bool = true,
        pub impl_math_div: bool = true,
        pub impl_math_mul: bool = true,
        pub impl_math_not: bool = true,
        pub impl_math_or: bool = true,
        pub impl_math_rem: bool = true,
        pub impl_math_shl: bool = true,
        pub impl_math_shr: bool = true,
        pub impl_math_sub: bool = true,
        pub impl_math_xor: bool = true,

        pub impl_friend: bool = true,
        pub impl_friend_make: bool = true,
        pub impl_friend_math_bit: bool = true,
        pub impl_friend_math_ops: bool = true,
        pub impl_friend_math_rel: bool = true,
        pub impl_friend_seal: bool = true,
        pub impl_friendzone_friend_make: bool = true,
        pub impl_friendzone_friend_math_bit: bool = true,
        pub impl_friendzone_friend_math_ops: bool = true,
        pub impl_friendzone_friend_math_rel: bool = true,
        pub impl_friendzone_seal: bool = true,

        pub fn_conv_into: bool = true,
        pub fn_conv_of: bool = true,
        pub fn_conv_raw: bool = true,
        pub fn_conv_try_into_checked: bool = true,
        pub fn_conv_try_into_unchecked: bool = true,
        pub fn_make_checked: bool = true,
        pub fn_make_checked_try: bool = true,
        pub fn_make_unchecked: bool = true,
        pub fn_make_unchecked_try: bool = true,
        pub fn_math_add: bool = true,
        pub fn_math_and: bool = true,
        pub fn_math_div: bool = true,
        pub fn_math_mul: bool = true,
        pub fn_math_not: bool = true,
        pub fn_math_or: bool = true,
        pub fn_math_rem: bool = true,
        pub fn_math_shl: bool = true,
        pub fn_math_shr: bool = true,
        pub fn_math_sub: bool = true,
        pub fn_math_xor: bool = true,
        pub fn_op_cmp: bool = true,
        pub fn_op_eq: bool = true,
    }
}

pub(super) struct Generator {
    repr: N,
    el: Ident,
    ty: Ident,

    fn_get_raw_ident: Ident,
    conv_fn_name: Ident,
    friend_conv_fn_name: TokenStream,

    int: Box<IntegralFlags>,
    fn_get_raw: ExprPath,
    fn_validator: Option<Path>,
    friends: Vec<FriendReq>,

    handy: Handy,
}

impl Generator {
    pub(super) fn new(
        ty: Ident,
        el: Ident,
        int: Box<IntegralFlags>,
        fn_get_raw: ExprPath,
        fn_validator: Option<Path>,
        friends: Vec<FriendReq>,
    ) -> syn::Result<Self> {
        if fn_get_raw.path.segments.len() != 2
            || fn_get_raw.path.segments[0].ident != "Self"
        {
            return ty.fail(
                "currently can only generate raw fn of the form Self::FN_NAME",
            );
        }

        let conv =
            format_ident!("conv_{}", runner::snake_case_of(&ty.to_string()));

        let fn_get_raw_ident = fn_get_raw.path.segments[1].ident.clone();
        let friend_conv_fn_name = quote! { Seal::#conv };
        let repr = N::of(el.to_string()).ok_or_else(|| {
            syn::Error::new(el.span(), "unknown integral type")
        })?;

        let this = Self {
            handy: Handy::of(int.konst),
            friend_conv_fn_name,
            repr,
            fn_get_raw_ident,
            ty,
            el,
            conv_fn_name: conv,
            fn_get_raw,
            fn_validator,
            friends,
            int,
        };

        return Ok(this);
    }

    pub(super) fn ekran(&self) -> syn::Result<TokenStream> {
        let ty = &self.ty;
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

        if self.int.fn_conv_of {
            stream.extend(self.make_impl_fn_of());
        }

        if self.int.fn_conv_raw {
            stream.extend(self.make_fn_raw());
        }

        if self.int.fn_conv_into {
            stream.extend(self.make_fns_into());
        }

        if self.int.fn_conv_try_into_unchecked {
            stream.extend(self.make_fns_try_into_unchecked());
        }

        if self.int.fn_conv_try_into_checked {
            stream.extend(self.make_fns_try_into_checked());
        }

        if self.int.fn_math_add {
            stream.extend(self.gen_binary_op(BinOp::Add));
        }

        if self.int.fn_math_sub {
            stream.extend(self.gen_binary_op(BinOp::Sub));
        }

        if self.int.fn_math_mul {
            stream.extend(self.gen_binary_op(BinOp::Mul));
        }

        if self.int.fn_math_div {
            stream.extend(self.gen_binary_op(BinOp::Div));
        }

        if self.int.fn_math_rem {
            stream.extend(self.gen_binary_op(BinOp::Rem));
        }

        if self.int.fn_math_xor {
            stream.extend(self.gen_binary_op(BinOp::Xor));
        }

        if self.int.fn_math_and {
            stream.extend(self.gen_binary_op(BinOp::And));
        }

        if self.int.fn_math_or {
            stream.extend(self.gen_binary_op(BinOp::Or_));
        }

        if self.int.fn_math_shr {
            stream.extend(self.gen_binary_op(BinOp::Shr));
        }

        if self.int.fn_math_shl {
            stream.extend(self.gen_binary_op(BinOp::Shl));
        }

        if self.int.fn_math_not {
            stream.extend(self.gen_unary_op(UnaryOp::Not));
        }

        if self.int.fn_op_eq {
            stream.extend(self.make_eq());
        }

        if self.int.fn_op_cmp {
            stream.extend(self.make_cmp());
        }

        if self.int.fn_make_unchecked_try
            && let Some(validator) = &self.fn_validator
        {
            stream.extend(self.make_fn_try_make_checked(validator));
        }
        else if self.int.fn_make_checked_try && self.fn_validator.is_none() {
            stream.extend(self.make_try_make_unchecked());
        }

        if self.int.fn_make_unchecked
            && let Some(validator) = &self.fn_validator
        {
            stream.extend(self.make_make_checked(validator));
        }
        else if self.int.fn_make_checked && self.fn_validator.is_none() {
            stream.extend(self.make_make_unchecked());
        }

        return stream;
    }

    fn ekran_impls(&self) -> TokenStream {
        let mut stream = TokenStream::new();

        if self.int.impl_into {
            stream.extend(self.make_impl_into());
        }

        if self.int.impl_try_into {
            stream.extend(self.make_impl_try_into());
        }

        if self.int.impl_math_shr {
            stream.extend(self.make_impl_shr());
        }

        if self.int.impl_math_shl {
            stream.extend(self.make_impl_shl());
        }

        if self.int.impl_assign_shr {
            stream.extend(self.make_impl_shr_assign());
        }

        if self.int.impl_assign_shl {
            stream.extend(self.make_impl_shl_assign());
        }

        if self.int.impl_assign_and {
            stream.extend(self.make_impl_and_assign());
        }

        if self.int.impl_assign_add {
            stream.extend(self.make_impl_add_assign());
        }

        if self.int.impl_assign_sub {
            stream.extend(self.make_impl_sub_assign());
        }

        if self.int.impl_assign_mul {
            stream.extend(self.make_impl_mul_assign());
        }

        if self.int.impl_assign_div {
            stream.extend(self.make_impl_div_assign());
        }

        if self.int.impl_assign_rem {
            stream.extend(self.make_impl_rem_assign());
        }

        if self.int.impl_assign_or {
            stream.extend(self.make_impl_or_assign());
        }

        if self.int.impl_range {
            stream.extend(self.make_impl_range());
        }

        if self.int.impl_debug {
            stream.extend(self.make_impl_fmt_debug());
        }

        if self.int.impl_eq {
            stream.extend(self.make_impl_eq());
        }

        if self.int.impl_ord {
            stream.extend(self.make_impl_ord());
        }

        if self.int.assertions {
            stream.extend(self.make_assertions());
        }

        if self.int.impl_partial_eq {
            stream.extend(self.make_impl_partial_eq());
        }

        if self.int.impl_partial_ord {
            stream.extend(self.make_impl_partial_ord());
        }

        if self.int.impl_math_add {
            stream.extend(self.make_impl_add());
        }

        if self.int.impl_math_sub {
            stream.extend(self.make_impl_sub());
        }

        if self.int.impl_math_mul {
            stream.extend(self.make_impl_mul());
        }

        if self.int.impl_math_div {
            stream.extend(self.make_impl_div());
        }

        if self.int.impl_math_rem {
            stream.extend(self.make_impl_rem());
        }

        if self.int.impl_math_and {
            stream.extend(self.make_impl_and());
        }

        if self.int.impl_math_or {
            stream.extend(self.make_impl_or());
        }

        if self.int.impl_math_xor {
            stream.extend(self.make_impl_xor());
        }

        if self.int.impl_math_not {
            stream.extend(self.make_impl_not());
        }

        if self.int.impl_friendzone_seal {
            stream.extend(self.make_friendzone_seal());
        }

        if self.int.impl_friendzone_friend_make {
            stream.extend(self.make_friendzone_friend_make());
        }

        if self.int.impl_friendzone_friend_math_ops {
            stream.extend(self.make_friendzone_friend_math_ops());
        }

        if self.int.impl_friendzone_friend_math_bit {
            stream.extend(self.make_friendzone_friend_math_bit());
        }

        if self.int.impl_friendzone_friend_math_rel {
            stream.extend(self.make_friendzone_friend_math_rel());
        }

        if self.int.impl_friend_seal {
            stream.extend(self.make_impl_friend_seal());
        }

        if self.int.impl_friend_make {
            stream.extend(self.make_impl_friend_make());
        }

        if self.int.impl_friend_math_ops {
            stream.extend(self.make_impl_friend_math_ops());
        }

        if self.int.impl_friend_math_bit {
            stream.extend(self.make_impl_friend_math_bit());
        }

        if self.int.impl_friend_math_rel {
            stream.extend(self.make_impl_friend_math_rel());
        }

        if self.int.impl_friend {
            stream.extend(self.make_impl_friends());
        }

        if self.int.impl_fmt_binary {
            stream.extend(self.make_impl_fmt_binary());
        }

        if self.int.impl_fmt_octal {
            stream.extend(self.make_impl_fmt_octal());
        }

        if self.int.impl_fmt_hex_lower {
            stream.extend(self.make_impl_fmt_hex_lower());
        }

        if self.int.impl_fmt_hex_upper {
            stream.extend(self.make_impl_fmt_hex_upper());
        }

        return stream;
    }
}

// Gen.
impl Generator {
    fn gen_binary_op(
        &self,
        it: BinOp,
    ) -> TokenStream {
        let raw = &self.fn_get_raw;
        let fn_name = format_ident!("_{}", it.name());
        let konst = self.handy.konst();
        let el = match it.predefined_arg_size() {
            Some(n) => &runner::ident_of(n),
            None => &self.el,
        };
        let op = it.stream();

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
                return Self::_unchecked(result);
            }
        };
    }

    fn gen_unary_op(
        &self,
        unary: UnaryOp,
    ) -> TokenStream {
        let raw = &self.fn_get_raw;
        let fn_name = format_ident!("_{}", unary.name());
        let op = unary.stream();

        return quote! {
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self) const fn #fn_name(
                self,
            ) -> Self {
                let this = #raw(self);
                let result = #op this;
                return Self::_unchecked(result);
            }
        };
    }

    fn gen_friend(
        &self,
        cty: &FriendReq,
    ) -> TokenStream {
        let el = &self.el;
        let konst = self.handy.konst();
        let conv = &self.conv_fn_name;
        let friend = &cty.ty;
        let level = &cty.level;

        let to_el = match &cty.conv {
            None => quote! { *self },
            Some(it) => match it.path.is_ident("self") {
                true => quote! { *self },
                false => quote! { #it(self) },
            },
        };

        let mut friendships = TokenStream::new();

        if level.contains(&FriendshipLevel::Math) {
            friendships.extend(quote! {
                #konst impl FriendMathOps for #friend { }
            });
        }
        if level.contains(&FriendshipLevel::Bit) {
            friendships.extend(quote! {
                #konst impl FriendMathBit for #friend { }
            });
        }
        if level.contains(&FriendshipLevel::Rel) {
            friendships.extend(quote! {
                #konst impl FriendMathRel for #friend { }
            });
        }
        if level.contains(&FriendshipLevel::Make) {
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
impl Generator {
    fn make_impl_add(&self) -> TokenStream {
        let ty = &self.ty;
        let (konst, bonst, destruct) = self.handy.konst_bonst_destruct();
        let fconv = &self.friend_conv_fn_name;

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
        let ty = &self.ty;
        let (konst, bonst, destruct) = self.handy.konst_bonst_destruct();
        let fconv = &self.friend_conv_fn_name;

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
        let ty = &self.ty;
        let (konst, bonst, destruct) = self.handy.konst_bonst_destruct();
        let fconv = &self.friend_conv_fn_name;

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
        let ty = &self.ty;
        let (konst, bonst, destruct) = self.handy.konst_bonst_destruct();
        let fconv = &self.friend_conv_fn_name;

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
        let ty = &self.ty;
        let (konst, bonst, destruct) = self.handy.konst_bonst_destruct();
        let fconv = &self.friend_conv_fn_name;

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
        let ty = &self.ty;
        let (konst, bonst, destruct) = self.handy.konst_bonst_destruct();
        let fconv = &self.friend_conv_fn_name;

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
        let ty = &self.ty;
        let (konst, bonst, destruct) = self.handy.konst_bonst_destruct();
        let fconv = &self.friend_conv_fn_name;

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
        let ty = &self.ty;
        let (konst, bonst, destruct) = self.handy.konst_bonst_destruct();
        let fconv = &self.friend_conv_fn_name;

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
        let ty = &self.ty;
        let konst = self.handy.konst();

        return quote! {

            #konst impl ::core::ops::Not for #ty {
                type Output = Self;

                #[inline(always)]
                fn not(self) -> Self::Output {
                    return self._not();
                }
            }
        };
    }

    fn make_impl_fmt_debug(&self) -> TokenStream {
        let ty = &self.ty;
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

    fn make_impl_fmt_binary(&self) -> TokenStream {
        let ty = &self.ty;
        let raw = &self.fn_get_raw;

        return quote! {

            impl ::core::fmt::Binary for #ty {
                fn fmt(
                    &self,
                    f: &mut core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    let raw = #raw(*self);
                    return ::core::fmt::Binary::fmt(&raw, f);
                }
            }

        };
    }

    fn make_impl_fmt_octal(&self) -> TokenStream {
        let ty = &self.ty;
        let raw = &self.fn_get_raw;

        return quote! {

            impl ::core::fmt::Octal for #ty {
                fn fmt(
                    &self,
                    f: &mut core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    let raw = #raw(*self);
                    return ::core::fmt::Octal::fmt(&raw, f);
                }
            }

        };
    }

    fn make_impl_fmt_hex_lower(&self) -> TokenStream {
        let ty = &self.ty;
        let raw = &self.fn_get_raw;

        return quote! {

            impl ::core::fmt::LowerHex for #ty {
                fn fmt(
                    &self,
                    f: &mut core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    let raw = #raw(*self);
                    return ::core::fmt::LowerHex::fmt(&raw, f);
                }
            }

        };
    }

    fn make_impl_fmt_hex_upper(&self) -> TokenStream {
        let ty = &self.ty;
        let raw = &self.fn_get_raw;

        return quote! {

            impl ::core::fmt::UpperHex for #ty {
                fn fmt(
                    &self,
                    f: &mut core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    let raw = #raw(*self);
                    return ::core::fmt::UpperHex::fmt(&raw, f);
                }
            }

        };
    }

    fn make_impl_eq(&self) -> TokenStream {
        let ty = &self.ty;
        let konst = self.handy.konst();

        return quote! {
            #konst impl ::core::cmp::Eq for #ty {}
        };
    }

    fn make_impl_ord(&self) -> TokenStream {
        let ty = &self.ty;
        let konst = self.handy.konst();

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
            .filter(|it| self.repr.fits_in(it))
            .map(|it| {
                stuff::impl_into_of(
                    self.repr,
                    *it,
                    &self.ty,
                    self.handy.konst(),
                )
            })
            .collect::<Vec<_>>();

        let mut merged = TokenStream::new();
        merged.extend(items);

        return merged;
    }

    fn make_impl_try_into(&self) -> TokenStream {
        let items = N::items()
            .iter()
            .filter(|it| !self.repr.fits_in(it))
            .map(|it| {
                stuff::impl_try_into_of(*it, &self.ty, self.handy.konst())
            })
            .collect::<Vec<_>>();

        let mut merged = TokenStream::new();
        merged.extend(items);

        return merged;
    }
}

// Anon.
impl Generator {
    fn make_impl_partial_eq(&self) -> TokenStream {
        let ty = &self.ty;
        let (konst, bonst, destruct) = self.handy.konst_bonst_destruct();
        let fconv = &self.friend_conv_fn_name;
        let raw = &self.fn_get_raw;

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
                    return #raw(*self) == that;
                }
            }
        };
    }

    fn make_impl_partial_ord(&self) -> TokenStream {
        let ty = &self.ty;
        let (konst, bonst, destruct) = self.handy.konst_bonst_destruct();
        let fconv = &self.friend_conv_fn_name;
        let raw = &self.fn_get_raw;

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
                    return #raw(*self).partial_cmp(&that);
                }
            }
        };
    }

    fn make_friendzone_seal(&self) -> TokenStream {
        let el = &self.el;
        let konst = self.handy.konst();
        let conv = &self.conv_fn_name;

        return quote! {
            #konst trait Seal {
                #[must_use]
                fn #conv(&self) -> #el;
            }
        };
    }

    fn make_friendzone_friend_make(&self) -> TokenStream {
        let (konst, bonst) = self.handy.konst_bonst();

        return quote! {
            #konst trait FriendMake:    #bonst Seal {}
        };
    }

    fn make_friendzone_friend_math_ops(&self) -> TokenStream {
        let (konst, bonst) = self.handy.konst_bonst();

        return quote! {
            #konst trait FriendMathOps: #bonst Seal {}
        };
    }

    fn make_friendzone_friend_math_rel(&self) -> TokenStream {
        let (konst, bonst) = self.handy.konst_bonst();

        return quote! {
            #konst trait FriendMathRel: #bonst Seal {}
        };
    }

    fn make_friendzone_friend_math_bit(&self) -> TokenStream {
        let (konst, bonst) = self.handy.konst_bonst();

        return quote! {
            #konst trait FriendMathBit: #bonst Seal {}
        };
    }

    fn make_impl_friend_seal(&self) -> TokenStream {
        let el = &self.el;
        let ty = &self.ty;
        let konst = self.handy.konst();
        let conv = &self.conv_fn_name;
        let raw = &self.fn_get_raw;

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
        let ty = &self.ty;
        let konst = self.handy.konst();

        return quote! {
            #konst impl FriendMake for #ty {}
        };
    }

    fn make_impl_friend_math_ops(&self) -> TokenStream {
        let ty = &self.ty;
        let konst = self.handy.konst();

        return quote! {
            #konst impl FriendMathOps for #ty {}
        };
    }

    fn make_impl_friend_math_rel(&self) -> TokenStream {
        let ty = &self.ty;
        let konst = self.handy.konst();

        return quote! {
            #konst impl FriendMathRel for #ty {}
        };
    }

    fn make_impl_friend_math_bit(&self) -> TokenStream {
        let ty = &self.ty;
        let konst = self.handy.konst();

        return quote! {
            #konst impl FriendMathBit for #ty {}
        };
    }

    fn make_impl_fn_of(&self) -> TokenStream {
        let ty = &self.ty;
        let (konst, bonst, destruct) = self.handy.konst_bonst_destruct();
        let fconv = &self.friend_conv_fn_name;

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
                return Self::_unchecked(this);
            }
        };
    }

    fn make_impl_friends(&self) -> TokenStream {
        return self
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
        let raw = &self.fn_get_raw;
        let el = &self.el;

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
        let raw = &self.fn_get_raw;
        let el = &self.el;
        let konst = self.handy.konst();

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
        let ty = &self.ty;
        let konst = self.handy.konst();

        return quote::quote! {

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
    }

    fn make_impl_shl(&self) -> TokenStream {
        let ty = &self.ty;
        let konst = self.handy.konst();

        return quote::quote! {

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
    }

    fn make_impl_shr_assign(&self) -> TokenStream {
        let ty = &self.ty;
        let konst = self.handy.konst();

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
        let ty = &self.ty;
        let konst = self.handy.konst();

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
        let ty = &self.ty;
        let konst = self.handy.konst();

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
        let ty = &self.ty;
        let konst = self.handy.konst();

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
        let ty = &self.ty;
        let konst = self.handy.konst();

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
        let ty = &self.ty;
        let konst = self.handy.konst();

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
        let ty = &self.ty;
        let konst = self.handy.konst();

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
        let ty = &self.ty;
        let konst = self.handy.konst();

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
        let ty = &self.ty;
        let konst = self.handy.konst();

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
        let raw_fn_name = &self.fn_get_raw_ident;
        let el = &self.el;

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
            .map(|it| stuff::fn_into_of(self.repr, *it, &self.fn_get_raw))
            .collect::<Vec<_>>();

        let mut merged = TokenStream::new();
        merged.extend(items);

        return merged;
    }

    fn make_fns_try_into_checked(&self) -> TokenStream {
        let items = N::items()
            .iter()
            .map(|it| {
                stuff::fn_try_into_checked_of(self.repr, *it, &self.fn_get_raw)
            })
            .collect::<Vec<_>>();

        let mut merged = TokenStream::new();
        merged.extend(items);

        return merged;
    }

    fn make_fns_try_into_unchecked(&self) -> TokenStream {
        let items = N::items()
            .iter()
            .map(|it| {
                stuff::fn_try_into_unchecked_of(
                    self.repr,
                    *it,
                    &self.fn_get_raw,
                )
            })
            .collect::<Vec<_>>();

        let mut merged = TokenStream::new();
        merged.extend(items);
        return merged;
    }

    fn make_fn_try_make_checked(
        &self,
        validator: &Path,
    ) -> TokenStream {
        let el = &self.el;
        let konst = self.handy.konst();

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
        let el = &self.el;

        return quote! {
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
    }

    fn make_try_make_unchecked(&self) -> TokenStream {
        let el = &self.el;

        return quote! {
            #[inline(always)]
            pub const fn try_make(it: #el) -> Result<Self, #el> {
                return Ok(Self::_unchecked(it));
            }
        };
    }

    fn make_make_unchecked(&self) -> TokenStream {
        let el = &self.el;

        return quote! {
            #[must_use]
            #[inline(always)]
            pub(self) const fn _unchecked(it: #el) -> Self {
                return Self(it);
            }
        };
    }

    fn make_assertions(&self) -> TokenStream {
        let ty = &self.ty;
        let el = &self.el;

        return quote! {
            if !(size_of::<#ty>() == size_of::<#el>()) {
                panic!("invalid memory layout: #ty(#el) != #el");
            };
        };
    }
}

mod stuff {
    use super::*;

    pub(super) fn fn_try_into_checked_of(
        this: N,
        to: N,
        raw: &ExprPath,
    ) -> TokenStream {
        if this.can_safe_cast_to(to) {
            return TokenStream::new();
        }

        let source = runner::ident_of(this);
        let target = runner::ident_of(to);
        let name = format_ident!("try_into_{}", to.rust_name());

        return quote! {

            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn #name(self) -> ::core::result::Result<#target, ()> {
                let r = #raw(self);
                let t = r as #target;
                let s = t as #source;

                return if s == r {
                    ::core::result::Result::Ok(t)
                }
                else {
                    ::core::result::Result::Err(())
                }
            }

        };
    }

    pub(super) fn fn_into_of(
        this: N,
        to: N,
        raw: &ExprPath,
    ) -> TokenStream {
        if !this.can_safe_cast_to(to) {
            return TokenStream::new();
        }

        let target = runner::ident_of(to);
        let name = format_ident!("into_{}", to.rust_name());

        return quote! {

            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn #name(self) -> #target {
                return #raw(self) as #target;
            }

        };
    }

    pub(super) fn fn_try_into_unchecked_of(
        this: N,
        to: N,
        raw: &ExprPath,
    ) -> TokenStream {
        if !this.can_safe_cast_to(to) {
            return TokenStream::new();
        }

        let target = runner::ident_of(to);
        let name = format_ident!("try_into_{}", to.rust_name());

        return quote! {
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn #name(self) -> ::core::result::Result<#target, ()> {
                return ::core::result::Result::Ok(
                    #raw(self) as #target
                );
            }
        };
    }

    pub(super) fn impl_try_into_of(
        to: N,
        whom: &Ident,
        konst: &TokenStream,
    ) -> TokenStream {
        let target = runner::ident_of(to);
        let conv = format_ident!("try_into_{}", to.rust_name());

        return quote! {
           #konst impl ::core::convert::TryInto<#target> for #whom {
               type Error = ();

               #[inline(always)]
               fn try_into(self) -> ::core::result::Result<#target, Self::Error> {
                    return #whom::#conv(self);
               }

           }
        };
    }

    pub(super) fn impl_into_of(
        this: N,
        to: N,
        whom: &Ident,
        konst: &TokenStream,
    ) -> TokenStream {
        if !this.can_safe_cast_to(to) {
            return TokenStream::new();
        }

        let target = runner::ident_of(to);
        let conv = format_ident!("into_{}", to.rust_name());

        return quote! {
           #konst impl ::core::convert::Into<#target> for #whom {

               #[inline(always)]
               fn into(self) -> #target {
                    return #whom::#conv(self);
               }

           }
        };
    }
}
