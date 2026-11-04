//! Per-type literal macros: `#[typekin::integral(domain = 1..)] struct
//! Port(u16);` also generates `port!`. `port!(443)` builds the wrapper in a
//! block-level `const` item, so an invalid value is a compile error (reported
//! by `cargo check`) and a valid one leaves no runtime check or panic path.
//! `port!(const 443)` uses an inline `const` block instead, which also works
//! where the type mentions generic parameters of the enclosing item.
//!
//! The macro is named after the snake-cased type. Below `pub` it is a
//! `macro_rules!` made path-addressable by a `use`; a `pub` type needs
//! `#[macro_export]`, under a mangled name re-exported beside the type. A
//! macro body names items unhygienically, so the expansion names the type as
//! written (`Port`, resolved at the call site) unless `lit_path` gives a
//! crate-relative path. A hidden guard method rejects a different type of the
//! same name at the call site.

use std::collections::BTreeSet;

use proc_macro2::{
    Ident,
    Span,
    TokenStream,
};
use quote::{
    ToTokens,
    format_ident,
    quote,
};
use syn::ext::IdentExt;
use syn::{
    GenericParam,
    Generics,
    Path,
    Visibility,
};

use crate::zz_names::{
    NameKey,
    Names,
};
use crate::zz_runner::{
    MkErr,
    snake_case_of,
};

/// Rust keywords (strict and reserved, edition 2024): not valid macro names.
const KEYWORDS: [&str; 51] = [
    "as",
    "async",
    "await",
    "break",
    "const",
    "continue",
    "crate",
    "dyn",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "gen",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "try",
];

/// Macros usable without an import: a generated macro with one of these
/// names would silently replace it in the defining module.
const PRELUDE_MACROS: [&str; 39] = [
    "assert",
    "assert_eq",
    "assert_ne",
    "cfg",
    "column",
    "compile_error",
    "concat",
    "dbg",
    "debug_assert",
    "debug_assert_eq",
    "debug_assert_ne",
    "env",
    "eprint",
    "eprintln",
    "file",
    "format",
    "format_args",
    "include",
    "include_bytes",
    "include_str",
    "line",
    "matches",
    "module_path",
    "option_env",
    "panic",
    "print",
    "println",
    "stringify",
    "thread_local",
    "todo",
    "unimplemented",
    "unreachable",
    "vec",
    "write",
    "writeln",
    "const_format_args",
    "concat_idents",
    "log_syntax",
    "trace_macros",
];

/// One annotated type's literal macro.
pub(crate) struct LitMacro {
    ty: Ident,
    name: Ident,
    /// The `#[macro_export]` name of a `pub` type's macro.
    exported: Option<Ident>,
    guard: Ident,
    /// The type as the expansion names it: `Port` or `$crate::net::Port`.
    path: TokenStream,
    generic: bool,
    /// Every type and const parameter has a default, so the plain item form
    /// can name the type without arguments.
    defaults: bool,
}

/// The literal macro's name: `lit_name`, else the snake-cased type name with
/// the type's span.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct LitMacroName;

impl NameKey for LitMacroName {
    type Owner = Ident;

    fn default_name(
        self,
        owner: &Ident,
    ) -> String {
        return snake_case_of(&owner.unraw().to_string());
    }

    fn default_span(
        self,
        owner: &Ident,
    ) -> Span {
        return owner.span();
    }
}

impl LitMacro {
    /// `name` and `path` are the `lit_name` and `lit_path` settings.
    pub(crate) fn new(
        ty: &Ident,
        generics: &Generics,
        visibility: &Visibility,
        name: Option<&Ident>,
        path: Option<&Path>,
    ) -> syn::Result<Self> {
        let type_name = ty.unraw().to_string();
        let mut names = Names::default();
        match name {
            Some(name) => names.choose(LitMacroName, name.clone()),
            None => {
                let derived = LitMacroName.default_name(ty);
                if KEYWORDS.contains(&derived.as_str()) {
                    return ty.fail(format!(
                        "`{type_name}` would generate `{derived}!`, which is \
                        a keyword; set `lit_name` or add \
                        `without = lit_macro`"
                    ));
                }
                if PRELUDE_MACROS.contains(&derived.as_str()) {
                    return ty.fail(format!(
                        "`{type_name}` would generate `{derived}!`, shadowing \
                        `core::{derived}!`; set `lit_name` or add \
                        `without = lit_macro`"
                    ));
                }
            }
        }
        let name = names.resolve(ty.clone(), BTreeSet::new()).get(LitMacroName);

        let hash = hash_of(ty);
        let exported = matches!(visibility, Visibility::Public(_))
            .then(|| format_ident!("__typekin_{}_{}", name, hash));
        let guard = format_ident!("__typekin_lit_{}", hash);
        let path = match path {
            Some(path) => crate_path(path)?,
            None => ty.to_token_stream(),
        };
        let defaults = generics.params.iter().all(|it| match it {
            GenericParam::Lifetime(_) => true,
            GenericParam::Type(it) => it.default.is_some(),
            GenericParam::Const(it) => it.default.is_some(),
        });

        return Ok(Self {
            ty: ty.clone(),
            name,
            exported,
            guard,
            path,
            generic: !generics.params.is_empty(),
            defaults,
        });
    }

    /// The name callers use: `port` for `port!`.
    pub(crate) fn name(&self) -> &Ident {
        return &self.name;
    }

    /// An inherent `impl` with the guard method the expansion calls.
    pub(crate) fn guard(
        &self,
        impl_generics: impl ToTokens,
        ty: impl ToTokens,
        where_clause: impl ToTokens,
    ) -> TokenStream {
        let guard = &self.guard;
        return quote! {
            impl #impl_generics #ty #where_clause {
                #[doc(hidden)]
                #[inline(always)]
                pub const fn #guard(&self) {}
            }
        };
    }

    /// The macro and the `use` that makes it path-addressable in its module.
    /// `hidden`: emitted inside the `hide_in` module, whose re-export of
    /// [`Self::name`] the caller adds.
    pub(crate) fn definition(
        &self,
        visibility: &Visibility,
        hidden: bool,
    ) -> TokenStream {
        let name = &self.name;
        let arms = self.arms();
        let doc = self.doc();

        return match &self.exported {
            Some(exported) => quote! {
                #[doc(hidden)]
                #[macro_export]
                macro_rules! #exported {
                    #arms
                }

                #doc
                #[doc(inline)]
                #[allow(unused_imports)]
                pub use #exported as #name;
            },
            None => {
                let visibility = match hidden {
                    // The re-export beside the type narrows it to the
                    // declared visibility.
                    true => quote! { pub(crate) },
                    false => visibility.to_token_stream(),
                };
                quote! {
                    #doc
                    #[allow(unused_macros)]
                    macro_rules! #name {
                        #arms
                    }

                    #[allow(unused_imports)]
                    #visibility use #name;
                }
            }
        };
    }

    fn doc(&self) -> TokenStream {
        let name = &self.name;
        let ty = &self.ty;
        let summary = format!(
            " Builds a [`{ty}`] from a constant checked at compile time: an \
            invalid value is a compile error."
        );
        let usage = match self.generic {
            false => format!(
                " `{name}!(value)` uses a `const` item, checked by \
                `cargo check`; `{name}!(const value)` uses an inline `const` \
                block."
            ),
            true => format!(
                " `{name}!([Args] value)` uses a `const` item, checked by \
                `cargo check`; `{name}!(const [Args] value)` uses an inline \
                `const` block, and `{name}!(const value)` infers the \
                arguments."
            ),
        };
        return quote! {
            #[doc = #summary]
            #[doc = ""]
            #[doc = #usage]
        };
    }

    fn arms(&self) -> TokenStream {
        let ty = &self.path;
        let item = |ty_tokens: TokenStream, call: TokenStream| {
            let checked = self.checked(call);
            return quote! {
                {
                    const VALUE: #ty_tokens = #checked;
                    VALUE
                }
            };
        };
        let inline = |call: TokenStream| {
            let checked = self.checked(call);
            return quote! { const { #checked } };
        };

        let plain = match (self.generic, self.defaults) {
            (false, _) | (true, true) => {
                let it = item(quote! { #ty }, quote! { #ty::try_make($value) });
                quote! { ($value:expr) => { #it }; }
            }
            (true, false) => {
                let name = &self.name;
                let message = format!(
                    "`{name}!` needs generic arguments: `{name}!([Args] \
                    value)`, or `{name}!(const value)` to infer them"
                );
                quote! {
                    ($value:expr) => {
                        ::core::compile_error!(#message)
                    };
                }
            }
        };
        let inline_plain = inline(quote! { #ty::try_make($value) });
        if !self.generic {
            return quote! {
                (const $value:expr) => { #inline_plain };
                #plain
            };
        }

        let explicit_ty = quote! { #ty<$($args)*> };
        let explicit_call = quote! { #ty::<$($args)*>::try_make($value) };
        let inline_explicit = inline(explicit_call.clone());
        let item_explicit = item(explicit_ty, explicit_call);
        return quote! {
            (const [$($args:tt)*] $value:expr) => { #inline_explicit };
            (const $value:expr) => { #inline_plain };
            ([$($args:tt)*] $value:expr) => { #item_explicit };
            #plain
        };
    }

    /// `call` evaluated, with the guard on success and a panic naming the
    /// value and the type otherwise.
    fn checked(
        &self,
        call: TokenStream,
    ) -> TokenStream {
        let guard = &self.guard;
        let message = format!(" is outside the domain of {}", self.ty);
        return quote! {
            match #call {
                ::core::result::Result::Ok(value) => {
                    value.#guard();
                    value
                }
                ::core::result::Result::Err(_) => ::core::panic!(
                    "{}",
                    ::core::concat!(::core::stringify!($value), #message),
                ),
            }
        };
    }
}

/// `crate::a::Type` as `$crate::a::Type`.
fn crate_path(path: &Path) -> syn::Result<TokenStream> {
    let mut segments = path.segments.iter();
    let starts_at_crate = path.leading_colon.is_none()
        && segments.next().is_some_and(|it| it.ident == "crate");
    if !starts_at_crate || path.segments.len() < 2 {
        return path.fail(
            "lit_path must be a crate-relative path such as `crate::net::Port`",
        );
    }
    if let Some(it) = path.segments.iter().find(|it| !it.arguments.is_none()) {
        return it.fail("lit_path cannot have generic arguments");
    }

    let rest = segments.map(|it| &it.ident);
    return Ok(quote! { $crate #(:: #rest)* });
}

/// FNV-1a over the type name and its source location: deterministic, and
/// distinct for same-named types in different modules of one crate, whose
/// `#[macro_export]` names share the crate root.
fn hash_of(ty: &Ident) -> String {
    let span = ty.span();
    let start = span.start();
    let key = format!("{}:{}:{}:{}", ty, span.file(), start.line, start.column);
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in key.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    return format!("{hash:016x}");
}
