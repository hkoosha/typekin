use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::collections::HashSet;
use std::fmt::Display;
use std::panic::UnwindSafe;
use std::sync::Arc;
use std::sync::Mutex;
use syn::Attribute;
use syn::Token;
use syn::bracketed;
use syn::meta::ParseNestedMeta;
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseBuffer;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

pub(crate) trait Merge {
    #[must_use]
    fn merge(
        self,
        other: Self,
    ) -> Self;
}

impl Merge for TokenStream {
    fn merge(
        mut self,
        other: Self,
    ) -> Self {
        self.extend(other);
        return self;
    }
}

pub(crate) trait Merged {
    type Output;

    #[must_use]
    fn merged(self) -> Self::Output;
}

impl<I> Merged for I
where
    I: Iterator<Item: Merge + Default>,
{
    type Output = I::Item;

    fn merged(self) -> Self::Output {
        return self
            .reduce(|a, b| {
                return a.merge(b);
            })
            .unwrap_or_default();
    }
}

macro_rules! mk_flags {
    (
        #[flag_default(bool=$def_bool:literal, str=$def_str:literal)]
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                pub $f_name:ident: $f_typ:ident $(= $f_def:expr)?
            ),* $(,)?
        }
    ) => {

        $(#[$meta])*
        $vis struct $name {
            $( pub $f_name: $f_typ, )*
        }

        impl ::core::default::Default for $name {
            #[inline(always)]
            fn default() -> Self {
                return Self::new();
            }
        }

        impl $name {

            $vis fn new() -> Self {
                return Self {
                    $(
                        $f_name: mk_flags!{
                            @define,
                            [$def_bool, $def_str],
                            $f_typ,
                            [$($f_def)?]
                        }
                    ),*
                };
            }

            $vis fn parse_from(
                &mut self,
                input: syn::parse::ParseStream,
                value_if_seen: bool,
            ) -> syn::Result<()> {
                let mut duplicated = Vec::with_capacity(0);
                let mut attrs = std::collections::HashMap::with_capacity(128);

                let input = $crate::runner::unbracket(input)?;

                $crate::runner::split_comma::<syn::Ident>(&input)?
                    .map(|it| (it.to_string(), it.span()))
                    .for_each(|(name, span)| {
                        let v = attrs.entry(name).or_insert((0usize, span));
                        v.0 += 1;
                        if v.0 > 1 {
                            duplicated.push(span);
                        }
                    });

                if let Some(span) = duplicated
                    .into_iter()
                    .reduce(|a, b| a.join(b).expect("bad spans"))
                {
                    let err = "duplicated flags";
                    return $crate::runner::MkErr::fail(&span, err);
                };

                attrs.retain(|it, _| self.set_bool(it, value_if_seen).is_err());

                if let Some(span) = attrs
                    .into_values()
                    .map(|it| it.1)
                    .reduce(|a, b| a.join(b).expect("bad spans"))
                {
                    let err = "unknown flags";
                    return $crate::runner::MkErr::fail(&span, err);
                };

                return Ok(());
            }

        }

        impl $name {

            mk_flags!( @collect,
                name  = [ $name ],
                work  = [ $($f_name : $f_typ),* , ],
                bool  = [ ],
                usize = [ ],
                str   = [ ],
                any   = [ ],
            );

        }
    };

    // bool.
    (@collect,
        name  = [ $name:ident ],
        work  = [ $f:ident : bool, $($work:tt)* ],
        bool  = [ $([$b:ident])* ],
        usize = [ $([$u:ident])* ],
        str   = [ $([$s:ident])* ],
        any   = [ $([$a:ident=$t:ident])* ],
    ) => {
        mk_flags! {
            @collect,
            name  = [$name],
            work  = [ $($work)* ],
            bool  = [ $([$b])* [$f] ],
            usize = [ $([$u])* ],
            str   = [ $([$s])* ],
            any   = [ $([$a=$t])* ],
        }
    };

    // usize.
    (@collect,
        name  = [ $name:ident ],
        work  = [ $f:ident : usize, $($work:tt)* ],
        bool  = [ $([$b:ident])* ],
        usize = [ $([$u:ident])* ],
        str   = [ $([$s:ident])* ],
        any   = [ $([$a:ident=$t:ident])* ],
    ) => {
        mk_flags! {
            @collect,
            name  = [$name],
            work  = [ $($work)* ],
            bool  = [ $([$b])* ],
            usize = [ $([$u])* [$f] ],
            str   = [ $([$s])* ],
            any   = [ $([$a=$t])* ],
        }
    };

    // String.
    (@collect,
        name  = [ $name:ident ],
        work  = [ $f:ident : String, $($work:tt)* ],
        bool  = [ $([$b:ident])* ],
        usize = [ $([$u:ident])* ],
        str   = [ $([$s:ident])* ],
        any   = [ $([$a:ident=$t:ident])* ],
    ) => {
        mk_flags! {
            @collect,
            name  = [$name],
            work  = [ $($work)* ],
            bool  = [ $([$b])* ],
            usize = [ $([$u])* ],
            str   = [ $([$s])* [$f] ],
            any   = [ $([$a=$t])* ],
        }
    };

    // Any.
    (@collect,
        name  = [ $name:ident ],
        work  = [ $f:ident : $m:ident, $($work:tt)* ],
        bool  = [ $([$b:ident])* ],
        usize = [ $([$u:ident])* ],
        str   = [ $([$s:ident])* ],
        any   = [ $([$a:ident=$t:ident])* ],
    ) => {
        mk_flags! {
            @collect,
            name  = [$name],
            work  = [ $($work)* ],
            bool  = [ $([$b])* ],
            usize = [ $([$u])* ],
            str   = [ $([$s])* ],
            any   = [ $([$a=$t])* [$f=$m] ],
        }
    };

    // Fin
    (@collect,
        name  = [ $name:ident ],
        work  = [ $(,)? ],
        bool  = [ $([$b:ident])* ],
        usize = [ $([$u:ident])* ],
        str   = [ $([$s:ident])* ],
        any   = [ $([$a:ident=$t:ident])* ],
    ) => {
       mk_flags!(@field, bool,   set_bool,   get_bool,   [$([$b]),*]);
       mk_flags!(@field, usize,  set_usize,  get_usize,  [$([$u]),*]);
       mk_flags!(@field, String, set_string, get_string, [$([$s]),*]);

       // mk_flags!(@field, any, set_any, get_any, [$([$a=$t]),*]);
    };

    (
        @str,
        [$( [$field:ident ] ),* $(,)?]
    ) => {

        #[allow(unreachable_code)]
        #[allow(unused_variables)]
        #[allow(unused)]
        pub fn str_str(
            &mut self,
            field: impl ::core::convert::AsRef<str>,
            value: impl ::core::convert::AsRef<str>,
        ) -> ::core::result::Result<(), &'static str> {
            let value = value.as_ref().to_string();

            match field.as_ref() {
                $(
                    stringify!($field) => self.$field = value,
                )*

                _ => return ::core::result::Result::Err(
                    "no such field or field is not a string",
                )
            }

            return Ok(());
        }

        #[allow(unreachable_code)]
        #[allow(unused_variables)]
        #[allow(unused)]
        pub fn $getter_fn(
            &self,
            field: impl ::core::convert::AsRef<str>,
        ) -> ::core::result::Result<$ty, &'static str> {
            return match field.as_ref() {
                $(
                    stringify!($field) => Ok(self.$field.clone()),
                )*

                _ => ::core::result::Result::Err(
                    "no such field or field is not a string",
                ),
            };
        }

    };

    (
        @field,
        $ty:ident,
        $setter_fn:ident,
        $getter_fn:ident,
        [$( [$field:ident ] ),* $(,)?]
    ) => {

        #[allow(unreachable_code)]
        #[allow(unused_variables)]
        #[allow(unused)]
        pub fn $setter_fn(
            &mut self,
            field: impl ::core::convert::AsRef<str>,
            value: $ty,
        ) -> ::core::result::Result<(), &'static str> {
            match field.as_ref() {
                $(
                    stringify!($field) => self.$field = value,
                )*

                _ => return ::core::result::Result::Err(
                    ::core::concat!(
                        "no such field or field is not a ",
                        ::core::stringify!($ty)
                    )
                )
            }

            return Ok(());
        }

        #[allow(unreachable_code)]
        #[allow(unused_variables)]
        #[allow(unused)]
        pub fn $getter_fn(
            &self,
            field: impl ::core::convert::AsRef<str>,
        ) -> ::core::result::Result<$ty, &'static str> {
            return match field.as_ref() {
                $(
                    stringify!($field) => Ok(self.$field.clone()),
                )*

                _ => ::core::result::Result::Err(
                    ::core::concat!(
                        "no such field or field is not a ",
                        ::core::stringify!($ty)
                    )
                ),
            };
        }

    };

    (@defines, [
        $( [$field:ident, $ty:ident, [ $( $def:expr )? ]] ),*
    ]) => {};

    (@define, [$def_bool:expr, $def_str:expr], String,    [$default:expr]) => {
        ::core::convert::Into::<String>::into($default)
    };

    (@define, [$def_bool:expr, $def_str:expr], String,    []) => {
        ::core::convert::Into::<String>::into($def_str)
    };

    (@define, [$def_bool:expr, $def_str:expr], &str,      [$default:expr]) => {
        ::core::convert::Into::<String>::into($default)
    };

    (@define, [$def_bool:expr, $def_str:expr], &str,      []) => {
        ::core::convert::Into::<String>::into($def_str)
    };

    (@define, [$def_bool:expr, $def_str:expr], bool,      [$default:expr]) => {
        $default
    };

    (@define, [$def_bool:expr, $def_str:expr], bool,      []) => {
        ::core::convert::Into::<bool>::into($def_bool)
    };

    (@define, [$def_bool:expr, $def_str:expr], $typ:tt,   [$($ignore:tt)*]) => {
        compile_error!("unknown flag type: {}", stringify!($typ))
    };
}

use crate::value_type::N;
pub(crate) use mk_flags;

pub(crate) fn list<T: Parse>(
    stream: ParseStream
) -> syn::Result<impl Iterator<Item = T>> {
    let content;
    let _ = bracketed!(content in stream);

    let items =
        Punctuated::<T, Token![,]>::parse_terminated(&content)?.into_iter();

    return Ok(items);
}

pub(crate) fn split_comma<T: Parse>(
    stream: ParseStream
) -> syn::Result<impl Iterator<Item = T>> {
    return Ok(
        Punctuated::<T, Token![,]>::parse_terminated(stream)?.into_iter()
    );
}

pub(crate) fn unbracket(stream: ParseStream) -> syn::Result<ParseBuffer> {
    let content;
    let _ = bracketed!(content in stream);
    return Ok(content);
}

pub(crate) fn find_repr_n(
    attrs: &Vec<Attribute>,
    span: impl Spanned,
) -> syn::Result<N> {
    let mut repr = None::<N>;

    for_repr(attrs, |it| {
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
    })?;

    return match repr {
        None => return span.fail("missing numeric repr"),
        Some(it) => Ok(it),
    };
}

fn for_repr(
    attrs: &Vec<Attribute>,
    mut exe: impl FnMut(ParseNestedMeta) -> syn::Result<()>,
) -> syn::Result<()> {
    attrs
        .into_iter()
        .filter(|it| it.path().is_ident("repr"))
        .try_for_each(|it| it.parse_nested_meta(|meta| exe(meta)))?;

    return Ok(());
}

pub(crate) fn find_repr_transparent(
    attrs: &Vec<Attribute>
) -> syn::Result<bool> {
    let mut found = false;

    for_repr(attrs, |it| {
        if it.path.is_ident("transparent") {
            found = true;
        }
        return Ok(());
    })?;

    return Ok(found);
}

pub(crate) fn parse_inner_attributes(
    input: ParseStream,
    mut on_attr: impl FnMut(&str, Span, ParseStream) -> syn::Result<()>,
) -> syn::Result<()> {
    let mut seen = HashSet::with_capacity(5);

    let span = input.span();

    while !input.is_empty() {
        let attr = {
            let key: Ident = input.parse()?;
            let it = key.to_string();
            if seen.contains(&it) {
                return key.fail("duplicated arg");
            }
            it
        };

        input.parse::<Token![=]>()?;

        on_attr(&attr, span, &input)?;

        seen.insert(attr);

        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }
    }

    return Ok(());
}

pub(crate) fn parse_optional_attributes(
    input: ParseStream,
    on_attr: impl FnMut(&str, Span, ParseStream) -> syn::Result<()>,
) -> syn::Result<()> {
    if !input.peek(syn::token::Paren) {
        return Ok(());
    }

    let content;
    let _ = parenthesized!(content in input);

    return parse_inner_attributes(&content, on_attr);
}

pub(crate) fn snake_case_of(it: &str) -> String {
    // AI generated, whatever.
    let mut result = String::with_capacity(it.len());
    let chars = it.chars().collect::<Vec<_>>();

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

pub(crate) trait MkErr: Spanned {
    fn fail<T>(
        &self,
        msg: impl Display,
    ) -> Result<T, syn::Error> {
        return Err(syn::Error::new(self.span(), msg));
    }
}

impl<T> MkErr for T where T: Spanned {}

fn backtraced<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R, String> {
    let err = Arc::new(Mutex::new(None));

    let err0 = err.clone();
    std::panic::set_hook(Box::new(move |it| {
        let text =
            format!("\n{}\n{}\n", it, std::backtrace::Backtrace::capture());
        err0.lock().unwrap().replace(text);
    }));

    let result = std::panic::catch_unwind(f);

    let _ = std::panic::take_hook();

    return result.map_err(|_| err.lock().unwrap().take().unwrap());
}

pub(crate) fn ekran_catching(
    f: impl FnOnce() -> syn::Result<TokenStream> + UnwindSafe
) -> proc_macro::TokenStream {
    return backtraced(|| {
        f().unwrap_or_else(|it| it.to_compile_error().into())
    })
    .map_err(|mut it| {
        let remove = "disabled backtrace\n";
        if it.ends_with(remove) {
            it.truncate(it.len() - remove.len());
        }
        return it;
    })
    .map_err(|it| quote::quote! { compile_error!(#it); })
    .unwrap_or_else(|it| it.into())
    .into();
}

pub(crate) fn konst(is_const: bool) -> Option<TokenStream> {
    return match is_const {
        true => Some(quote! { const }),
        false => None,
    };
}

pub(crate) fn konst_bonst_and_destruct(
    is_const: bool
) -> (
    Option<TokenStream>,
    Option<TokenStream>,
    Option<TokenStream>,
) {
    return (
        konst(is_const),
        match is_const {
            true => Some(quote! { [const] }),
            false => None,
        },
        match is_const {
            true => Some(quote! { + [const] ::core::marker::Destruct }),
            false => None,
        },
    );
}
