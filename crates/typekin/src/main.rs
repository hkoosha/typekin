use quote::ToTokens;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::sync::LazyLock;
use syn::PathSegment;
use syn::ReturnType;
use syn::Stmt;
use syn::Token;
use syn::Type;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit_mut::{VisitMut, visit_item_mod_mut};
use syn::{Attribute, Item};
use syn::{Expr, ItemImpl};
use syn::{ItemConst, parse_quote};
use syn::{ItemEnum, Path};
use syn::{ItemMod, ItemStruct};

pub(crate) trait MkErr: Spanned {
    fn fail<T>(
        &self,
        msg: impl Display,
    ) -> Result<T, syn::Error> {
        return Err(syn::Error::new(self.span(), msg));
    }
}

impl<T> MkErr for T where T: Spanned {}

const HEADER: &'static str = "// AUTO-GENERATED VIA typekin-unexpand, DO NOT MODIFY\n#![allow(clippy::needless_return)]";

static USE_REPLACEMENTS: LazyLock<
    HashMap<&'static [&'static str], &'static str>,
> = LazyLock::new(|| {
    const KV: &[(&[&str], &str)] = &[
        (&["core"], "core"),
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

    return KV.iter().map(|it| *it).collect();
});

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Replaced {
    Clone,
    CloneTrivial,
    CmpEq,
    CmpOrd,
    CmpPEq,
    CmpPOrd,
    FmtDebug,
    Hash,
    MCopy,
    MSPEq,
}

impl Replaced {
    fn is_in(
        &self,
        attributes: &Vec<Attribute>,
    ) -> bool {
        if let Some((_, it)) = self.derive(false)
            && attributes.contains(&it)
        {
            return true;
        }
        if let Some((_, it)) = self.derive(true)
            && attributes.contains(&it)
        {
            return true;
        }

        return false;
    }

    fn path(self) -> &'static [&'static [&'static str]] {
        return match self {
            Self::Clone => &[&["core", "clone", "Clone"], &["Clone"]],
            Self::CloneTrivial => &[&["core", "clone", "TrivialClone"]],
            Self::CmpEq => &[&["core", "cmp", "Eq"]],
            Self::CmpOrd => &[&["core", "cmp", "Ord"]],
            Self::CmpPEq => &[&["core", "cmp", "PartialEq"]],
            Self::CmpPOrd => &[&["core", "cmp", "PartialOrd"]],
            Self::FmtDebug => &[&["core", "fmt", "Debug"]],
            Self::Hash => &[&["core", "hash", "Hash"]],
            Self::MCopy => &[&["core", "marker", "Copy"]],
            Self::MSPEq => &[&["core", "marker", "StructuralPartialEq"]],
        };
    }

    fn derive(
        self,
        konst: bool,
    ) -> Option<(Self, Attribute)> {
        let path: Path = match self {
            Self::CloneTrivial => return None,
            Self::MSPEq => return None,

            Self::Clone => parse_quote! { ::core::clone::Clone },
            Self::CmpEq => parse_quote! { ::core::cmp::Eq },
            Self::CmpOrd => parse_quote! { ::core::cmp::Ord },
            Self::CmpPEq => parse_quote! { ::core::cmp::PartialEq },
            Self::FmtDebug => parse_quote! { ::core::fmt::Debug },
            Self::Hash => parse_quote! { ::core::hash::Hash },
            Self::MCopy => parse_quote! { ::core::marker::Copy },
            Self::CmpPOrd => parse_quote! { ::core::cmp::PartialOrd },
        };

        let attr = match konst {
            true => parse_quote! { #[derive_const(#path)] },
            false => parse_quote! { #[derive(#path)] },
        };

        return Some((self, attr));
    }

    fn check_replace(
        it: &Punctuated<PathSegment, Token![::]>,
        this: &[&str],
    ) -> bool {
        if it.len() != this.len() {
            return false;
        }

        for (i, p) in it.iter().enumerate() {
            if p.ident.to_string() != this.get(i).unwrap().to_string() {
                return false;
            }
        }

        return true;
    }

    fn replaces(
        self,
        it: &Punctuated<PathSegment, Token![::]>,
    ) -> bool {
        for this in self.path() {
            if Self::check_replace(it, this) {
                return true;
            }
        }

        return false;
    }

    fn items() -> &'static [Replaced] {
        const ITEMS: [Replaced; 10] = [
            Replaced::Clone,
            Replaced::CloneTrivial,
            Replaced::CmpEq,
            Replaced::CmpOrd,
            Replaced::CmpPEq,
            Replaced::CmpPOrd,
            Replaced::FmtDebug,
            Replaced::Hash,
            Replaced::MCopy,
            Replaced::MSPEq,
        ];

        return &ITEMS;
    }
}

#[derive(Default)]
struct Undo {
    konst: HashMap<String, HashSet<Replaced>>,
    plain: HashMap<String, HashSet<Replaced>>,
}

impl Undo {
    fn should_retain_impl(
        &mut self,
        it: &ItemImpl,
        konst: bool,
    ) -> bool {
        return if let Some(trait_) = &it.trait_
            && let Type::Path(typ) = it.self_ty.as_ref()
            && typ.path.segments.len() == 1
            && let Some(subject) = typ.path.segments.last()
            && trait_.0.segments.iter().all(|it| it.arguments.is_none())
            && let Some(replacement) = Replaced::items()
                .iter()
                .find(|r| r.replaces(&trait_.0.segments))
        {
            match konst {
                true => &mut self.konst,
                false => &mut self.plain,
            }
            .entry(subject.ident.to_string())
            .or_insert_with(HashSet::default)
            .insert(*replacement);

            false
        }
        else {
            true
        };
    }

    fn should_retain_item(
        &mut self,
        item: &Item,
        konst: bool,
    ) -> bool {
        return match item {
            Item::Impl(it) => self.should_retain_impl(it, konst),

            Item::Use(it) => !it
                .attrs
                .iter()
                .any(|it| it.path().is_ident("prelude_import")),

            _ => true,
        };
    }

    fn chk_items(
        &mut self,
        items: &mut Vec<Item>,
        konst: bool,
    ) {
        items.retain(|it| self.should_retain_item(it, konst));
        self.delete_auto_impl(items);
    }

    fn chk_statements(
        &mut self,
        items: &mut Vec<Stmt>,
        konst: bool,
    ) {
        items.retain(|it| match it {
            Stmt::Item(it) => self.should_retain_item(it, konst),
            _ => true,
        });
    }

    fn chk_attrs(
        &self,
        attrs: &mut Vec<Attribute>,
    ) {
        attrs.retain(|attr| {
            return !matches!(attr.style, syn::AttrStyle::Inner(_))
                || !attr.path().is_ident("feature")
                || !attr
                    .parse_args::<syn::Ident>()
                    .ok()
                    .map_or(false, |i| i == "prelude_import");
        });
    }

    // Until syn support const impl.
    fn delete_auto_impl(
        &mut self,
        items: &mut Vec<Item>,
    ) {
        items.retain_mut(|item| {
            return match item {
                Item::Verbatim(ts) => {
                    let reparse = Parser::parse2(
                        |input: syn::parse::ParseStream| {
                            let attrs = input.call(Attribute::parse_outer)?;

                            if input.peek(Token![const])
                                && input.peek2(Token![unsafe])
                                && input.peek3(Token![impl])
                            {
                                let _: Token![const] = input.parse()?;
                                let _: Token![unsafe] = input.parse()?;
                            }
                            else if input.peek(Token![unsafe])
                                && input.peek2(Token![const])
                                && input.peek3(Token![impl])
                            {
                                let _: Token![unsafe] = input.parse()?;
                                let _: Token![const] = input.parse()?;
                            }
                            else if input.peek(Token![const])
                                && input.peek2(Token![impl])
                            {
                                let _: Token![const] = input.parse()?;
                            }
                            else {
                                return ts.fail("ignore");
                            }

                            let mut item: ItemImpl = input.parse()?;
                            item.attrs = attrs;

                            Ok(item)
                        },
                        ts.clone(),
                    );

                    let reparse = match reparse {
                        Err(_) => return true,
                        Ok(it) => it,
                    };

                    self.should_retain_impl(&reparse, true)
                }

                Item::Mod(it) => {
                    if let Some((_, ref mut inner)) = it.content {
                        self.delete_auto_impl(inner);
                    }

                    true
                }

                _ => true,
            };
        });
    }
}

impl VisitMut for Undo {
    // Unexpand built-in rust macros.
    fn visit_expr_mut(
        &mut self,
        it: &mut Expr,
    ) {
        syn::visit_mut::visit_expr_mut(self, it);

        // Unexpand panic!().
        // DOES NOT MATCH ON RESOLVED PATH, MATCHES BY STRING!
        if let Expr::Call(call) = it
            && let Expr::Path(p) = &*call.func
            && let Some(Expr::Macro(m)) = call.args.first()
            && m.mac.path.is_ident("format_args")
            && p.path
                .segments
                .iter()
                .rev()
                .take(2)
                .rev()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .ends_with(&["panicking".to_string(), "panic_fmt".to_string()])
        {
            let inner = &m.mac.tokens;
            *it = syn::parse_quote! { panic!(#inner) };
        }

        // Unexpand panic!().
        // DOES NOT MATCH ON RESOLVED PATH, MATCHES BY STRING!
        if let Expr::Call(call) = it
            && let Expr::Path(p) = &*call.func
            && let Some(Expr::Macro(m)) = call.args.first()
            && m.mac.path.is_ident("format_args")
            && p.path
                .segments
                .iter()
                .rev()
                .take(1)
                .rev()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
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

        self.chk_attrs(&mut f.attrs);
        self.chk_items(&mut f.items, false);
    }

    fn visit_item_const_mut(
        &mut self,
        i: &mut ItemConst,
    ) {
        syn::visit_mut::visit_item_const_mut(self, i);

        if let Expr::Block(blk) = i.expr.as_mut() {
            self.chk_statements(&mut blk.block.stmts, false);
        }
    }

    fn visit_item_mod_mut(
        &mut self,
        i: &mut ItemMod,
    ) {
        visit_item_mod_mut(self, i);

        if let Some((_, inner)) = &mut i.content {
            inner.retain(|it| self.should_retain_item(it, false));
        }
    }

    fn visit_path_mut(
        &mut self,
        it: &mut Path,
    ) {
        syn::visit_mut::visit_path_mut(self, it);

        let segments = it
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>();

        let segments_ref =
            segments.iter().map(|it| it.as_str()).collect::<Vec<_>>();

        if let Some(replacement) =
            USE_REPLACEMENTS.get(&segments_ref.as_slice())
        {
            it.leading_colon = None;
            let mut r = Punctuated::new();
            std::mem::swap(&mut it.segments, &mut r);
            it.segments = Punctuated::new();
            it.segments.push(PathSegment {
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

struct Redo {
    undo: Undo,
}

impl Redo {
    fn apply(
        &mut self,
        ident: String,
        attrs: &mut Vec<Attribute>,
    ) {
        let plain = match self.undo.plain.get(&ident) {
            None => HashMap::with_capacity(0),
            Some(work) => work
                .iter()
                .filter(|it| !it.is_in(attrs))
                .flat_map(|it| it.derive(false))
                .collect::<HashMap<_, _>>(),
        };

        let konst = match self.undo.konst.get(&ident) {
            None => HashMap::with_capacity(0),
            Some(work) => work
                .iter()
                .filter(|it| !it.is_in(attrs))
                .flat_map(|it| it.derive(true))
                .collect::<HashMap<_, _>>(),
        };

        if plain.iter().any(|it| konst.contains_key(it.0))
            || konst.iter().any(|it| plain.contains_key(it.0))
        {
            panic!(
                "clash {} vs {}",
                plain
                    .keys()
                    .into_iter()
                    .map(|it| it.path()[0][0])
                    .collect::<Vec<_>>()
                    .join(", "),
                konst
                    .keys()
                    .into_iter()
                    .map(|it| it.path()[0][0])
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        let plain = plain
            .into_iter()
            .filter(|(it, _)| !it.is_in(attrs))
            .map(|(_, it)| it)
            .collect::<Vec<_>>();

        let konst = konst
            .into_iter()
            .filter(|(it, _)| !it.is_in(attrs))
            .map(|(_, it)| it)
            .collect::<Vec<_>>();

        attrs.extend(plain);
        attrs.extend(konst);
    }

    fn apply_item(
        &mut self,
        it: &mut Item,
    ) {
        match it {
            Item::Enum(it) => {
                self.apply(it.ident.to_string(), &mut it.attrs);
            }
            Item::Struct(it) => {
                self.apply(it.ident.to_string(), &mut it.attrs);
            }
            _ => {}
        }
    }

    fn apply_statements(
        &mut self,
        statements: &mut Vec<Stmt>,
    ) {
        for it in statements {
            if let Stmt::Item(it) = it {
                self.apply_item(it);
            }
        }
    }
}

impl VisitMut for Redo {
    fn visit_block_mut(
        &mut self,
        it: &mut syn::Block,
    ) {
        syn::visit_mut::visit_block_mut(self, it);

        self.apply_statements(&mut it.stmts);
    }

    fn visit_file_mut(
        &mut self,
        f: &mut syn::File,
    ) {
        syn::visit_mut::visit_file_mut(self, f);

        for it in &mut f.items {
            self.apply_item(it);
        }
    }

    fn visit_item_const_mut(
        &mut self,
        i: &mut ItemConst,
    ) {
        syn::visit_mut::visit_item_const_mut(self, i);

        if let Expr::Block(blk) = i.expr.as_mut() {
            self.apply_statements(&mut blk.block.stmts);
        }
    }

    fn visit_item_enum_mut(
        &mut self,
        it: &mut ItemEnum,
    ) {
        syn::visit_mut::visit_item_enum_mut(self, it);

        self.apply(it.ident.to_string(), &mut it.attrs);
    }

    fn visit_item_struct_mut(
        &mut self,
        it: &mut ItemStruct,
    ) {
        syn::visit_mut::visit_item_struct_mut(self, it);

        self.apply(it.ident.to_string(), &mut it.attrs);
    }
}

fn main() {
    if std::env::args().len() != 2 {
        let arg = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
        panic!("{}: {}", "bad args, expecting only an input_file", arg);
    }

    let input = std::env::args().nth(1).expect("missing arg: input_file");

    let body = std::fs::read_to_string(&input).expect("failed to read");
    let mut file = syn::parse_str(&body).expect("failed to parse input file");

    let mut undo = Undo::default();
    undo.visit_file_mut(&mut file);

    let mut redo = Redo { undo };
    redo.visit_file_mut(&mut file);

    let text = file.to_token_stream().to_token_stream().to_string();

    println!("{}\n\n{}", HEADER, text);
}
