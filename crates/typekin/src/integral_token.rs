use crate::integral_types::N;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Fields, ItemStruct, Type};

pub(crate) fn find_unit_struct_inner_type(
    item: &ItemStruct
) -> syn::Result<&Ident> {
    const MSG: &'static str = "expected a tuple struct with exactly one field of primitive integral type";

    let fields = if let Fields::Unnamed(it) = &item.fields {
        if it.unnamed.len() != 1 {
            return err(it, MSG);
        }
        it
    }
    else {
        return err(item, MSG);
    };

    let field = if let Type::Path(it) = &fields.unnamed[0].ty {
        it
    }
    else {
        return err(&fields.unnamed[0].ty, MSG);
    };

    let id = if let Some(it) = field.path.get_ident() {
        it
    }
    else {
        return err(field, MSG);
    };
    if !N::rust_names().contains(&id.to_string().as_str()) {
        return err(id, MSG);
    }

    return Ok(id);
}

pub(crate) fn err<T>(
    t: impl Spanned,
    msg: &'static str,
) -> syn::Result<T> {
    return Err(syn::Error::new(t.span(), msg));
}

pub(crate) fn to_snake_case(s: &str) -> String {
    // AI generated, whatever.
    let mut result = String::with_capacity(s.len());
    let chars = s.chars().collect::<Vec<_>>();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !chars[i - 1].is_uppercase() {
                result.push('_');
            }
            else if i > 0 && chars[i - 1].is_uppercase() {
                if let Some(&next) = chars.get(i + 1) {
                    if next.is_lowercase() {
                        result.push('_');
                    }
                }
            }

            result.push(c.to_lowercase().next().unwrap());
        }
        else {
            result.push(c);
        }
    }

    result
}

pub(crate) fn n_of_ident(it: &Ident) -> syn::Result<N> {
    return N::of(it.to_string())
        .ok_or_else(|| syn::Error::new(it.span(), "unknown integral type"));
}

pub(crate) fn ident_of_n(n: N) -> Ident {
    return Ident::new(n.rust_name(), Span::call_site());
}

pub(crate) fn make_fn_try_into_checked(
    this: N,
    to: N,
) -> TokenStream {
    if this.can_safe_cast_to(to) {
        return TokenStream::new();
    }

    let source = ident_of_n(this);
    let target = ident_of_n(to);
    let name = format_ident!("try_into_{}", to.rust_name());

    return quote! {

        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn #name(self) -> ::core::result::Result<#target, ()> {
            let r = self.raw();
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

pub(crate) fn make_fn_into(
    this: N,
    to: N,
) -> TokenStream {
    if !this.can_safe_cast_to(to) {
        return TokenStream::new();
    }

    let target = ident_of_n(to);
    let name = format_ident!("into_{}", to.rust_name());

    return quote! {

        #[must_use]
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn #name(self) -> #target {
            return self.raw() as #target;
        }

    };
}

pub(crate) fn make_fn_try_into_unchecked(
    this: N,
    to: N,
) -> TokenStream {
    if !this.can_safe_cast_to(to) {
        return TokenStream::new();
    }

    let target = ident_of_n(to);
    let name = format_ident!("try_into_{}", to.rust_name());

    return quote! {
        #[inline(always)]
        #[allow(clippy::unnecessary_cast)]
        pub const fn #name(self) -> ::core::result::Result<#target, ()> {
            return ::core::result::Result::Ok(
                self.raw() as #target
            );
        }
    };
}

pub(crate) fn make_impl_try_into(
    to: N,
    whom: &Ident,
    konst: &TokenStream,
) -> TokenStream {
    let target = ident_of_n(to);
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

pub(crate) fn make_impl_into(
    this: N,
    to: N,
    whom: &Ident,
    konst: &TokenStream,
) -> TokenStream {
    if !this.can_safe_cast_to(to) {
        return TokenStream::new();
    }

    let target = ident_of_n(to);
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
