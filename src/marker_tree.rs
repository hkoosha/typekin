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

use crate::{
    zz,
    zz_runner,
};

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
        let definitions = EmittedNode::emit_definitions(&nodes, true);
        let owner_name = owner.to_string();
        let owner_name = owner_name.trim_start_matches("r#");
        let module = format_ident!("__typekin_marker_tree_for_{}", owner_name,);
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

    fn expand(
        &self,
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
        if !root.generics.params.is_empty()
            || root.generics.where_clause.is_some()
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

        self.validate(&root.ident)?;

        let nodes = self.collect(&root.vis, &root.ident);
        let definitions = EmittedNode::emit_definitions(&nodes, false);

        return Ok(quote! {
            #root
            #definitions
        });
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
        let ident = self.ident.clone();

        if let Some(children) = &self.children {
            nodes.push(EmittedNode {
                visibility: visibility.clone(),
                ident: ident.clone(),
                ancestors: ancestors.to_vec(),
                branch: true,
            });

            let mut child_ancestors = ancestors.to_vec();
            child_ancestors.push(ident);
            for child in children {
                child.collect(&visibility, &child_ancestors, nodes);
            }
        }
        else {
            nodes.push(EmittedNode {
                visibility,
                ident,
                ancestors: ancestors.to_vec(),
                branch: false,
            });
        }
    }
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
        public: bool,
    ) -> TokenStream {
        let branches = nodes
            .iter()
            .filter(|node| node.branch)
            .map(|node| node.emit_branch(public));
        let leaves = nodes
            .iter()
            .filter(|node| !node.branch)
            .map(|node| node.emit_leaf(public));

        return quote! {
            #(#branches)*
            #(#leaves)*
        };
    }

    fn emit_branch(
        &self,
        public: bool,
    ) -> TokenStream {
        let visibility = self.emit_visibility(public);
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
        public: bool,
    ) -> TokenStream {
        let visibility = self.emit_visibility(public);
        let ident = &self.ident;
        let ancestors = &self.ancestors;

        return quote! {
            #visibility struct #ident;
            #(impl #ancestors for #ident {})*
        };
    }

    fn emit_visibility(
        &self,
        public: bool,
    ) -> TokenStream {
        if public {
            return quote! { pub };
        }

        let visibility = &self.visibility;
        return quote! { #visibility };
    }

    fn emit_export(
        &self,
        module: &Ident,
    ) -> TokenStream {
        let visibility = &self.visibility;
        let ident = &self.ident;

        return quote! {
            #[allow(unused_imports)]
            #visibility use #module::#ident;
        };
    }
}

pub(crate) fn marker_tree(
    cfg: Cfg,
    root: ItemTrait,
) -> proc_macro::TokenStream {
    return zz_runner::catching(move || expand(cfg, root)).into();
}

pub(crate) fn expand(
    cfg: Cfg,
    root: ItemTrait,
) -> syn::Result<TokenStream> {
    return cfg.expand(root);
}
