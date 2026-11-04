//! `hide_in`: moves a generated wrapper and everything typekin emits for it
//! into a private module and re-exports the public surface at the call site,
//! so the tuple constructor and the value field are unreachable even from the
//! defining module.

use std::collections::BTreeSet;

use proc_macro2::{
    Group,
    Ident,
    TokenStream,
    TokenTree,
};
use quote::{
    ToTokens,
    quote,
};
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::{
    Attribute,
    Expr,
    Fields,
    Lit,
    Meta,
    Visibility,
    parse_quote,
};

use crate::zz_lit::LitMacro;
use crate::zz_names::{
    NameKey,
    Names,
};
use crate::zz_runner::{
    MkErr,
    snake_case_of,
};

/// The parsed `hide_in` option.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) enum HideIn {
    /// Not configured: hide in `__` + the snake-cased type name.
    #[default]
    Default,
    /// `hide_in = none`: no hiding.
    Off,
    /// `hide_in = name`.
    Named(Ident),
}

impl Parse for HideIn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.peek(syn::Ident) {
            return input.span().fail("hide_in takes a module name or none");
        }

        let name = input.parse::<Ident>()?;
        return match name == "none" {
            true => Ok(Self::Off),
            false => Ok(Self::Named(name)),
        };
    }
}

/// The hidden module's name: `hide_in = name`, else `__` + the snake-cased
/// type name.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct ModuleName;

impl NameKey for ModuleName {
    type Owner = Ident;

    fn default_name(
        self,
        owner: &Ident,
    ) -> String {
        return format!("__{}", snake_case_of(&owner.to_string()));
    }
}

impl HideIn {
    /// The module to hide `ty` in, or `None` when hiding is off.
    pub(crate) fn module_for(
        &self,
        ty: &Ident,
    ) -> Option<Ident> {
        let mut names = Names::default();
        match self {
            Self::Off => return None,
            Self::Default => {}
            Self::Named(name) => names.choose(ModuleName, name.clone()),
        }
        return Some(
            names.resolve(ty.clone(), BTreeSet::new()).get(ModuleName),
        );
    }
}

/// A family configuration that carries `hide_in`.
pub(crate) trait HideCfg {
    fn hide_in(&self) -> &HideIn;
}

/// Re-anchors paths for code that moves one module deeper: `self::x` becomes
/// `super::x` and `super::x` becomes `super::super::x`. Other paths resolve
/// through the module's `use super::*`. String literals are left alone.
pub(crate) fn rewrite_child_paths(tokens: TokenStream) -> TokenStream {
    let tokens = tokens.into_iter().collect::<Vec<_>>();
    let mut result = Vec::with_capacity(tokens.len());

    for (index, token) in tokens.iter().enumerate() {
        match token {
            TokenTree::Group(group) => {
                let mut rewritten = Group::new(
                    group.delimiter(),
                    rewrite_child_paths(group.stream()),
                );
                rewritten.set_span(group.span());
                result.push(TokenTree::Group(rewritten));
            }
            TokenTree::Ident(ident)
                if starts_path(&tokens, index)
                    && (*ident == "self" || *ident == "super") =>
            {
                let span = ident.span();
                if *ident == "super" {
                    result.push(TokenTree::Ident(Ident::new("super", span)));
                    result.extend(quote::quote_spanned! { span=> :: });
                }
                result.push(TokenTree::Ident(Ident::new("super", span)));
            }
            other => result.push(other.clone()),
        }
    }

    return result.into_iter().collect();
}

/// `tokens[index]` is followed by `::` and not preceded by `::`.
fn starts_path(
    tokens: &[TokenTree],
    index: usize,
) -> bool {
    let is_colon = |at: Option<&TokenTree>| {
        return matches!(at, Some(TokenTree::Punct(punct)) if punct.as_char() == ':');
    };
    let followed =
        is_colon(tokens.get(index + 1)) && is_colon(tokens.get(index + 2));
    let preceded = index >= 2
        && is_colon(tokens.get(index - 1))
        && is_colon(tokens.get(index - 2));
    return followed && !preceded;
}

/// `pub(super)` on a field moved one module deeper becomes
/// `pub(in super::super)`; `pub(in super)` likewise. Multi-segment
/// `pub(in …)` paths were already re-anchored by [`rewrite_child_paths`].
fn rewrite_field_visibility(fields: &mut Fields) {
    for field in fields.iter_mut() {
        let Visibility::Restricted(restricted) = &field.vis
        else {
            continue;
        };
        if restricted.path.is_ident("super") {
            field.vis = parse_quote! { pub(in super::super) };
        }
    }
}

/// Attribute strings are not rewritten (typekin cannot tell which strings are
/// paths), so a `"super::…"` string would silently resolve one module lower.
fn reject_super_strings<'a>(
    attributes: impl IntoIterator<Item = &'a Attribute>
) -> syn::Result<()> {
    for attribute in attributes {
        if attribute.path().is_ident("doc") {
            continue;
        }
        let tokens = match &attribute.meta {
            Meta::Path(_) => continue,
            Meta::List(list) => list.tokens.clone(),
            Meta::NameValue(name_value) => match &name_value.value {
                Expr::Lit(literal) => quote! { #literal },
                other => quote! { #other },
            },
        };
        reject_super_strings_in(tokens)?;
    }

    return Ok(());
}

fn reject_super_strings_in(tokens: TokenStream) -> syn::Result<()> {
    for token in tokens {
        match token {
            TokenTree::Group(group) => reject_super_strings_in(group.stream())?,
            TokenTree::Literal(literal) => {
                let Ok(Lit::Str(string)) =
                    syn::parse2::<Lit>(quote! { #literal })
                else {
                    continue;
                };
                if string.value().starts_with("super::") {
                    return string.span().fail(
                        "\"super::…\" would resolve inside hide_in's module; \
                        write a crate:: path or set hide_in = none",
                    );
                }
            }
            _ => {}
        }
    }

    return Ok(());
}

/// `#[allow(unused_imports)] #visibility use #module::#name;`
pub(crate) fn reexport(
    visibility: &impl ToTokens,
    module: &Ident,
    name: &Ident,
) -> TokenStream {
    return quote! {
        #[allow(unused_imports)]
        #visibility use #module::#name;
    };
}

/// Wraps `item` and its generated code `rest` in the private module, with
/// `item` first, and re-exports each `(visibility, name)` at the call site.
fn wrap(
    module: &Ident,
    exports: &[(Visibility, Ident)],
    item: TokenStream,
    rest: TokenStream,
) -> TokenStream {
    let exports = exports
        .iter()
        .map(|(visibility, export)| reexport(visibility, module, export));

    return quote! {
        #[doc(hidden)]
        mod #module {
            #item

            #[allow(unused_imports)]
            use super::*;

            #rest
        }

        #(#exports)*
    };
}

/// The annotated item's visibility inside the hidden module: the declared
/// visibility re-anchored one module deeper, so the nominal visibility seen
/// by privacy checks (E0446 on associated types) is unchanged. Private
/// becomes `pub(super)`, `pub(super)` becomes `pub(in super::super)`;
/// `pub` and `pub(crate)` stay.
pub(crate) fn inner_visibility(declared: &Visibility) -> Visibility {
    let Visibility::Restricted(restricted) = declared
    else {
        return match declared {
            Visibility::Inherited => parse_quote! { pub(super) },
            other => other.clone(),
        };
    };

    let path = &restricted.path;
    if path.is_ident("crate") {
        return declared.clone();
    }
    if path.is_ident("self") {
        return parse_quote! { pub(super) };
    }
    if path.is_ident("super") {
        return parse_quote! { pub(in super::super) };
    }
    let path = rewrite_child_paths(quote! { #path });
    return parse_quote! { pub(in #path) };
}

/// `_blind_trust` stays private to the type's module, except that a hidden
/// type which asked for it (`with_blind_trust`) exposes it to the defining
/// module: the one explicitly named way in that skips validation.
pub(crate) fn blind_trust_visibility(
    hide_in: &HideIn,
    requested: bool,
) -> TokenStream {
    return match requested && *hide_in != HideIn::Off {
        true => quote! { pub(super) },
        false => quote! { pub(self) },
    };
}

/// An annotated item `hide_in` can move: its name picks the default module and
/// its declared visibility goes on the re-export.
pub(crate) trait Hideable: Parse + ToTokens {
    fn ident(&self) -> &Ident;
    fn visibility_mut(&mut self) -> &mut Visibility;
    /// Its own and its fields' or variants' attributes.
    fn attributes(&self) -> impl Iterator<Item = &Attribute>;
    /// Re-anchors nested visibilities one module deeper: a struct's fields;
    /// an enum has none.
    fn rewrite_nested_visibility(&mut self);
}

impl Hideable for syn::ItemStruct {
    fn ident(&self) -> &Ident {
        return &self.ident;
    }

    fn visibility_mut(&mut self) -> &mut Visibility {
        return &mut self.vis;
    }

    fn attributes(&self) -> impl Iterator<Item = &Attribute> {
        return self
            .attrs
            .iter()
            .chain(self.fields.iter().flat_map(|field| &field.attrs));
    }

    fn rewrite_nested_visibility(&mut self) {
        rewrite_field_visibility(&mut self.fields);
    }
}

impl Hideable for syn::ItemEnum {
    fn ident(&self) -> &Ident {
        return &self.ident;
    }

    fn visibility_mut(&mut self) -> &mut Visibility {
        return &mut self.vis;
    }

    fn attributes(&self) -> impl Iterator<Item = &Attribute> {
        return self
            .attrs
            .iter()
            .chain(self.variants.iter().flat_map(|variant| &variant.attrs));
    }

    fn rewrite_nested_visibility(&mut self) {}
}

/// A family's embedded marker tree (`tree = ...`).
pub(crate) trait EmbeddedTree {
    /// The tree's nodes for `owner`, declared with `visibility`.
    fn embed(
        &self,
        owner: &Ident,
        visibility: &Visibility,
    ) -> syn::Result<TokenStream>;
}

/// How an annotated item and its generated code leave the macro: in place,
/// or moved into the `hide_in` module with re-exports, followed by the
/// embedded tree.
pub(crate) struct Envelope {
    module: Option<Ident>,
    declared: Visibility,
    /// Re-exported after the item's own name, in order.
    exports: Vec<(Visibility, Ident)>,
    tree: Option<TokenStream>,
}

impl Envelope {
    pub(crate) fn new(
        hide_in: &HideIn,
        item: &impl Hideable,
        declared: &Visibility,
    ) -> Self {
        return Self {
            module: hide_in.module_for(item.ident()),
            declared: declared.clone(),
            exports: Vec::new(),
            tree: None,
        };
    }

    pub(crate) fn is_hidden(&self) -> bool {
        return self.module.is_some();
    }

    /// The item's visibility as written.
    pub(crate) fn declared(&self) -> &Visibility {
        return &self.declared;
    }

    /// Hidden: rejects `"super::…"` attribute strings and re-anchors nested
    /// visibilities. Always: expands the embedded tree.
    pub(crate) fn prepare(
        &mut self,
        item: &mut impl Hideable,
        tree: Option<&impl EmbeddedTree>,
    ) -> syn::Result<()> {
        if self.is_hidden() {
            reject_super_strings(item.attributes())?;
            item.rewrite_nested_visibility();
        }
        self.tree = tree
            .map(|tree| tree.embed(item.ident(), &self.declared))
            .transpose()?;
        return Ok(());
    }

    /// Re-exports `name` with `visibility` after the item.
    pub(crate) fn export(
        &mut self,
        visibility: Visibility,
        name: Ident,
    ) {
        self.exports.push((visibility, name));
    }

    /// The literal macro's definition, re-exported with the item's
    /// visibility.
    pub(crate) fn lit_definition(
        &mut self,
        lit: &LitMacro,
    ) -> TokenStream {
        let definition = lit.definition(&self.declared, self.is_hidden());
        self.exports
            .push((self.declared.clone(), lit.name().clone()));
        return definition;
    }

    /// In place: the attributed item, the tree, then the implementation.
    /// Hidden: the item, re-anchored one module deeper, and the
    /// implementation in the module, then the re-exports and the tree.
    pub(crate) fn finish(
        self,
        attributes: TokenStream,
        mut item: impl Hideable,
        implementation: TokenStream,
    ) -> TokenStream {
        let tree = &self.tree;
        let Some(module) = &self.module
        else {
            return quote! {
                #attributes
                #item
                #tree

                #implementation
            };
        };

        *item.visibility_mut() = inner_visibility(&self.declared);
        let mut exports = vec![(self.declared.clone(), item.ident().clone())];
        exports.extend(self.exports);
        let wrapped = wrap(
            module,
            &exports,
            quote! { #attributes #item },
            implementation,
        );
        return quote! {
            #wrapped
            #tree
        };
    }
}

/// Re-parses attribute and item tokens after re-anchoring their paths. The
/// item's own visibility is kept as written: it applies to the re-export at
/// the call site, not inside the module.
pub(crate) fn reparse<A, I>(
    attr: TokenStream,
    item: TokenStream,
    mut original: I,
    parse_attr: impl FnOnce(TokenStream) -> syn::Result<A>,
) -> syn::Result<(A, I)>
where
    I: Hideable,
{
    let attr = parse_attr(rewrite_child_paths(attr))?;
    let mut item = syn::parse2::<I>(rewrite_child_paths(item))?;
    std::mem::swap(item.visibility_mut(), original.visibility_mut());
    return Ok((attr, item));
}
