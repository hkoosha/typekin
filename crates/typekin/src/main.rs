extern crate core;

use quote::ToTokens;
use std::collections::HashMap;
use std::fmt::Display;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;
use syn::{ReturnType, Type};

const HEADER: &'static str = "// AUTO-GENERATED VIA typekin-unexpand, DO NOT MODIFY\n#![allow(clippy::needless_return)]";

const REPLACEMENTS: &[(&[&str], &str)] = &[
    (&["core", "option", "Option"], "Option"),
    (&["core", "option", "Option", "None"], "None"),
    (&["core", "option", "Option", "Some"], "Some"),
    (&["core", "result", "Result"], "Result"),
    (&["core", "result", "Result", "Ok"], "Ok"),
    (&["core", "result", "Result", "Err"], "Err"),
    (&["core", "convert", "Into"], "Into"),
    (&["core", "convert", "TryInto"], "TryInto"),
    (&["core", "clone", "Clone"], "Clone"),
    (&["std", "string", "String"], "String"),
];

trait ToVec<I> {
    fn collect_vec(self) -> Vec<I>;
}

impl<T: Iterator> ToVec<T::Item> for T {
    fn collect_vec(self) -> Vec<T::Item> {
        return self.collect();
    }
}

struct Unexpand {
    path_replacements: HashMap<&'static [&'static str], &'static str>,
}

impl Unexpand {
    fn new() -> Self {
        return Self {
            path_replacements: REPLACEMENTS.iter().map(|it| *it).collect(),
        };
    }

    fn ekran(text: &str) -> syn::Result<String> {
        let mut ast: syn::File = syn::parse_str(text)?;

        Self::new().visit_file_mut(&mut ast);

        let fixed = ast.to_token_stream().to_string();

        // FIXME
        let hack = fixed
            .replace(":: core :: convert :: Into", "Into")
            .replace(":: core", "core");

        return Ok(hack);
    }
}

impl VisitMut for Unexpand {
    // Drop useless blocks.
    fn visit_block_mut(
        &mut self,
        it: &mut syn::Block,
    ) {
        syn::visit_mut::visit_block_mut(self, it);

        if it.stmts.len() == 1 {
            if let syn::Stmt::Expr(syn::Expr::Block(inner), _) =
                it.stmts[0].clone()
            {
                it.stmts = inner.block.stmts;
            }
        }
    }

    // Unexpand built-in rust macros.
    fn visit_expr_mut(
        &mut self,
        it: &mut syn::Expr,
    ) {
        syn::visit_mut::visit_expr_mut(self, it);

        // Unexpand panic!().
        // DOES NOT MATCH ON RESOLVED PATH, MATCHES BY STRING!
        if let syn::Expr::Call(call) = it
            && let syn::Expr::Path(p) = &*call.func
            && let Some(syn::Expr::Macro(m)) = call.args.first()
            && m.mac.path.is_ident("format_args")
            && p.path
                .segments
                .iter()
                .rev()
                .take(2)
                .rev()
                .map(|s| s.ident.to_string())
                .collect_vec()
                .ends_with(&["panicking".to_string(), "panic_fmt".to_string()])
        {
            let inner = &m.mac.tokens;
            *it = syn::parse_quote! { panic!(#inner) };
        }

        // Unexpand panic!().
        // DOES NOT MATCH ON RESOLVED PATH, MATCHES BY STRING!
        if let syn::Expr::Call(call) = it
            && let syn::Expr::Path(p) = &*call.func
            && let Some(syn::Expr::Macro(m)) = call.args.first()
            && m.mac.path.is_ident("format_args")
            && p.path
                .segments
                .iter()
                .rev()
                .take(1)
                .rev()
                .map(|s| s.ident.to_string())
                .collect_vec()
                .ends_with(&["_print".to_string()])
        {
            let inner = &m.mac.tokens;
            *it = syn::parse_quote! { print!(#inner) };
        }
    }

    // Drop automatically_derived and prelude_import items.
    fn visit_file_mut(
        &mut self,
        f: &mut syn::File,
    ) {
        syn::visit_mut::visit_file_mut(self, f);

        f.items.retain(|item| {
            return !match item {
                syn::Item::Use(it) => it
                    .attrs
                    .iter()
                    .any(|it| it.path().is_ident("prelude_import")),

                syn::Item::Impl(it) => it
                    .attrs
                    .iter()
                    .any(|it| it.path().is_ident("automatically_derived")),

                _ => false,
            };
        });

        f.attrs.retain(|attr| {
            let is_inner = matches!(attr.style, syn::AttrStyle::Inner(_));
            if !is_inner {
                return true;
            }
            let is_feature_prelude = attr.path().is_ident("feature")
                && attr
                    .parse_args::<syn::Ident>()
                    .ok()
                    .map_or(false, |i| i == "prelude_import");
            !is_feature_prelude
        });
    }

    // Replace full path expr with import.
    fn visit_path_mut(
        &mut self,
        it: &mut syn::Path,
    ) {
        syn::visit_mut::visit_path_mut(self, it);

        let segments = it
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect_vec();

        let segments_ref = segments.iter().map(|it| it.as_str()).collect_vec();

        if let Some(replacement) =
            self.path_replacements.get(&segments_ref.as_slice())
        {
            it.leading_colon = None;
            let mut r = Punctuated::new();
            std::mem::swap(&mut it.segments, &mut r);
            it.segments = Punctuated::new();
            it.segments.push(syn::PathSegment {
                ident: syn::Ident::new(
                    replacement,
                    proc_macro2::Span::call_site(),
                ),
                arguments: r.last().unwrap().arguments.clone(),
            });
        }
    }

    fn visit_return_type_mut(
        &mut self,
        it: &mut ReturnType,
    ) {
        syn::visit_mut::visit_return_type_mut(self, it);

        if let ReturnType::Type(_, ty) = it {
            if let Type::Path(path) = ty.as_mut() {
                self.visit_path_mut(&mut path.path);
            }
        }
    }
}

fn die(
    msg: &str,
    arg: impl Display,
) -> ! {
    eprintln!("{}: {}", msg, arg);
    std::process::exit(1);
}

fn main() {
    if std::env::args().len() > 2 {
        die(
            "bad args, expecting only an input_file",
            std::env::args().collect_vec().join(" "),
        );
    }

    let input = match std::env::args().nth(1) {
        None => die("missing arg", "input_file"),
        Some(it) => it,
    };

    let body = match std::fs::read_to_string(&input) {
        Err(it) => die("failed to read input file", it),
        Ok(it) => it,
    };

    let mut text = match Unexpand::ekran(&body) {
        Err(it) => die("failed to process input", it),
        Ok(it) => it,
    };

    // WTF?
    const HACKS: &[(&[&str], &str)] = REPLACEMENTS;
    for (path, with) in HACKS {
        let it = path.join(" :: ");
        text = text.replace(&it, with);
    }

    println!("{}\n\n{}", HEADER, text);
}
