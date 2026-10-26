use std::collections::BTreeSet;
use std::fmt::Debug;

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::quote;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::punctuated::Punctuated;
use syn::{
    ItemTrait,
    Token,
    Visibility,
    braced,
};

use crate::zz_runner;

pub(crate) struct Cfg {
    nodes: Vec<Node>,
}

impl Debug for Cfg {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        return formatter.debug_struct("MarkerTreeCfg").finish();
    }
}

struct Node {
    visibility: Visibility,
    ident: Ident,
    children: Option<Vec<Self>>,
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let nodes = Punctuated::<Node, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect::<Vec<_>>();
        if nodes.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "marker tree requires at least one node",
            ));
        }

        return Ok(Self { nodes });
    }
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility: Visibility = input.parse()?;
        let ident: Ident = input.parse()?;
        let children = if input.peek(syn::token::Brace) {
            let content;
            let _ = braced!(content in input);
            let children =
                Punctuated::<Self, Token![,]>::parse_terminated(&content)?
                    .into_iter()
                    .collect::<Vec<_>>();
            if children.is_empty() {
                return Err(syn::Error::new(
                    ident.span(),
                    "marker tree branches require at least one child",
                ));
            }
            Some(children)
        }
        else {
            None
        };

        return Ok(Self {
            visibility,
            ident,
            children,
        });
    }
}

impl Cfg {
    fn validate(
        &self,
        root: &Ident,
    ) -> syn::Result<()> {
        let mut names = BTreeSet::new();
        assert!(names.insert(root.to_string()));
        for node in &self.nodes {
            node.validate(&mut names)?;
        }

        return Ok(());
    }
}

impl Node {
    fn validate(
        &self,
        names: &mut BTreeSet<String>,
    ) -> syn::Result<()> {
        if !names.insert(self.ident.to_string()) {
            return Err(syn::Error::new(
                self.ident.span(),
                "duplicated marker tree node",
            ));
        }
        if let Some(children) = &self.children {
            for child in children {
                child.validate(names)?;
            }
        }

        return Ok(());
    }

    fn emit(
        &self,
        ancestors: &[Ident],
        branches: &mut Vec<TokenStream>,
        leaves: &mut Vec<TokenStream>,
    ) {
        let visibility = &self.visibility;
        let ident = &self.ident;
        if let Some(children) = &self.children {
            let parent =
                ancestors.last().expect("marker tree branches have a root");
            branches.push(quote! {
                #[allow(private_bounds)]
                #visibility trait #ident: #parent {}
            });
            let mut child_ancestors = ancestors.to_vec();
            child_ancestors.push(ident.clone());
            for child in children {
                child.emit(&child_ancestors, branches, leaves);
            }
        }
        else {
            leaves.push(quote! {
                #visibility struct #ident;
                #(impl #ancestors for #ident {})*
            });
        }
    }
}

pub(crate) fn marker_tree(
    cfg: Cfg,
    root: ItemTrait,
) -> proc_macro::TokenStream {
    return zz_runner::catching(move || expand(cfg, root));
}

pub(crate) fn expand(
    cfg: Cfg,
    root: ItemTrait,
) -> syn::Result<TokenStream> {
    if root.unsafety.is_some() {
        return Err(syn::Error::new(
            root.ident.span(),
            "marker tree root must be a safe trait",
        ));
    }
    if root.modifiers.auto_token.is_some() {
        return Err(syn::Error::new(
            root.ident.span(),
            "marker tree root must not be an auto trait",
        ));
    }
    if !root.generics.params.is_empty() || root.generics.where_clause.is_some()
    {
        return Err(syn::Error::new(
            root.ident.span(),
            "marker tree root must not be generic",
        ));
    }
    if !root.supertraits.is_empty() {
        return Err(syn::Error::new(
            root.ident.span(),
            "marker tree root must not have supertraits",
        ));
    }
    if !root.items.is_empty() {
        return Err(syn::Error::new(
            root.ident.span(),
            "marker tree root must not define trait items",
        ));
    }

    cfg.validate(&root.ident)?;

    let mut branches = Vec::new();
    let mut leaves = Vec::new();
    let ancestors = vec![root.ident.clone()];
    for node in &cfg.nodes {
        node.emit(&ancestors, &mut branches, &mut leaves);
    }

    return Ok(quote! {
        #root
        #(#branches)*
        #(#leaves)*
    });
}
