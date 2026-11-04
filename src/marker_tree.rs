use std::collections::BTreeSet;
use std::fmt::Debug;

use proc_macro2::{
    Ident,
    Span,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::ext::IdentExt;
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

use crate::zz_hide::{
    EmbeddedTree,
    reexport,
};
use crate::zz_lib as zz;
use crate::zz_runner::MkErr;

#[derive(Clone)]
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

#[derive(Clone)]
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

        return Self::from_nodes(nodes, input.span());
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
    pub(crate) fn parse_attribute(input: ParseStream) -> syn::Result<Self> {
        let nodes = zz::one_or_list::<Node>(input)?.collect();

        return Self::from_nodes(nodes, input.span());
    }

    fn from_nodes(
        nodes: Vec<Node>,
        span: Span,
    ) -> syn::Result<Self> {
        if nodes.is_empty() {
            return Err(syn::Error::new(
                span,
                "marker tree requires at least one node",
            ));
        }

        return Ok(Self { nodes });
    }

    pub(crate) fn expand_embedded(
        &self,
        owner: &Ident,
        visibility: &Visibility,
    ) -> syn::Result<TokenStream> {
        let seal = format_ident!("MarkerTreeRoot");
        self.validate(&seal)?;

        let nodes = self.collect(visibility, &seal);
        let definitions =
            EmittedNode::emit_definitions(&nodes, Placement::Embedded);
        let module = format_ident!(
            "__typekin_marker_tree_for_{}",
            owner.unraw().to_string(),
        );
        let exports = nodes.iter().map(|node| node.emit_export(&module));

        return Ok(quote! {
            #[doc(hidden)]
            #[allow(dead_code)]
            #[allow(non_snake_case)]
            mod #module {
                trait #seal {}

                #definitions
            }

            #(#exports)*
        });
    }

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

    fn collect(
        &self,
        visibility: &Visibility,
        seal: &Ident,
    ) -> Vec<EmittedNode> {
        let ancestors = vec![seal.clone()];
        let mut nodes = Vec::new();
        for node in &self.nodes {
            node.collect(visibility, &ancestors, &mut nodes);
        }

        return nodes;
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

    fn collect(
        &self,
        parent_visibility: &Visibility,
        ancestors: &[Ident],
        nodes: &mut Vec<EmittedNode>,
    ) {
        let visibility = match &self.visibility {
            Visibility::Inherited => parent_visibility.clone(),
            visibility => visibility.clone(),
        };
        nodes.push(EmittedNode {
            visibility: visibility.clone(),
            ident: self.ident.clone(),
            ancestors: ancestors.to_vec(),
            branch: self.children.is_some(),
        });

        if let Some(children) = &self.children {
            let mut child_ancestors = ancestors.to_vec();
            child_ancestors.push(self.ident.clone());
            for child in children {
                child.collect(&visibility, &child_ancestors, nodes);
            }
        }
    }
}

/// Where the marker tree's items live: beside the user's root trait, or in a
/// hidden module embedded by an owning macro (every item `pub`, re-exported).
#[derive(Clone, Copy)]
enum Placement {
    Standalone,
    Embedded,
}

struct EmittedNode {
    visibility: Visibility,
    ident: Ident,
    ancestors: Vec<Ident>,
    branch: bool,
}

impl EmittedNode {
    fn emit_definitions(
        nodes: &[Self],
        placement: Placement,
    ) -> TokenStream {
        let branches = nodes
            .iter()
            .filter(|node| node.branch)
            .map(|node| node.emit_branch(placement));
        let leaves = nodes
            .iter()
            .filter(|node| !node.branch)
            .map(|node| node.emit_leaf(placement));

        return quote! {
            #(#branches)*
            #(#leaves)*
        };
    }

    fn emit_branch(
        &self,
        placement: Placement,
    ) -> TokenStream {
        let visibility = self.emit_visibility(placement);
        let ident = &self.ident;
        let parent = self
            .ancestors
            .last()
            .expect("marker tree branches have a parent");
        let seal = self
            .ancestors
            .first()
            .expect("marker tree branches have a seal");
        let bounds = if parent == seal {
            quote! { #parent }
        }
        else {
            quote! { #parent + #seal }
        };

        return quote! {
            #[allow(private_bounds)]
            #visibility trait #ident: #bounds {}
        };
    }

    fn emit_leaf(
        &self,
        placement: Placement,
    ) -> TokenStream {
        let visibility = self.emit_visibility(placement);
        let ident = &self.ident;
        let ancestors = &self.ancestors;

        return quote! {
            #visibility struct #ident;
            #(impl #ancestors for #ident {})*
        };
    }

    fn emit_visibility(
        &self,
        placement: Placement,
    ) -> TokenStream {
        return match placement {
            Placement::Embedded => quote! { pub },
            Placement::Standalone => {
                let visibility = &self.visibility;
                quote! { #visibility }
            }
        };
    }

    fn emit_export(
        &self,
        module: &Ident,
    ) -> TokenStream {
        return reexport(&self.visibility, module, &self.ident);
    }
}

fn validate_root(root: &ItemTrait) -> syn::Result<()> {
    if root.unsafety.is_some() {
        return root.ident.fail("marker tree root must be a safe trait");
    }
    if root.modifiers.auto_token.is_some() {
        return root
            .ident
            .fail("marker tree root must not be an auto trait");
    }
    if !root.generics.params.is_empty() || root.generics.where_clause.is_some()
    {
        return root.ident.fail("marker tree root must not be generic");
    }
    if !root.supertraits.is_empty() {
        return root
            .ident
            .fail("marker tree root must not have supertraits");
    }
    if !root.items.is_empty() {
        return root
            .ident
            .fail("marker tree root must not define trait items");
    }

    return Ok(());
}

pub(crate) fn expand(
    cfg: Cfg,
    root: ItemTrait,
) -> syn::Result<TokenStream> {
    validate_root(&root)?;
    cfg.validate(&root.ident)?;

    let nodes = cfg.collect(&root.vis, &root.ident);
    let definitions =
        EmittedNode::emit_definitions(&nodes, Placement::Standalone);

    return Ok(quote! {
        #root
        #definitions
    });
}

impl EmbeddedTree for Cfg {
    fn embed(
        &self,
        owner: &Ident,
        visibility: &Visibility,
    ) -> syn::Result<TokenStream> {
        return self.expand_embedded(owner, visibility);
    }
}
