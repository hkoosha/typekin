use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::fmt::Debug;

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
use syn::parse::discouraged::Speculative;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::punctuated::Punctuated;
use syn::{
    Attribute,
    ItemTrait,
    Token,
    Type,
    TypeParamBound,
    Visibility,
    braced,
    bracketed,
    parse_quote,
};

use crate::zz_flags::{
    Flag,
    FlagGrammar,
    FlagSettings,
    Flags,
};
use crate::zz_hide::{
    EmbeddedTree,
    inner_visibility,
    reexport,
    rewrite_child_paths,
};
use crate::zz_lib as zz;
use crate::zz_runner::{
    MkErr,
    ToCollection,
};

#[derive(Clone)]
pub(crate) struct Cfg {
    nodes: Vec<Node>,
    flags: TreeFlags,
}

impl Debug for Cfg {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        return formatter.debug_struct("MarkerTreeCfg").finish();
    }
}

pub(crate) type TreeFlags = Flags<TreeFlag>;

/// A trait every leaf derives and every top-level branch requires, so that
/// code generic over a branch can use it. All are on; `without` removes them.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum TreeFlag {
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
}

impl Flag for TreeFlag {
    const ITEMS: &'static [Self] = &[
        Self::Clone,
        Self::Copy,
        Self::Debug,
        Self::Default,
        Self::PartialEq,
        Self::Eq,
        Self::Hash,
        Self::PartialOrd,
        Self::Ord,
    ];
    const DEFAULT_ON: &'static [Self] = Self::ITEMS;

    fn name(self) -> &'static str {
        return match self {
            Self::Clone => "clone",
            Self::Copy => "copy",
            Self::Debug => "debug",
            Self::Default => "default",
            Self::PartialEq => "partial_eq",
            Self::Eq => "eq",
            Self::Hash => "hash",
            Self::PartialOrd => "partial_ord",
            Self::Ord => "ord",
        };
    }

    fn index(self) -> u32 {
        return self as u32;
    }
}

// Every variant is in `ITEMS`, at its own index.
const _: () = {
    let items = <TreeFlag as Flag>::ITEMS;
    assert!(items.len() == TreeFlag::Ord as usize + 1);
    let mut index = 0;
    while index < items.len() {
        assert!(items[index] as usize == index);
        index += 1;
    }
};

impl TreeFlag {
    /// `(flag, needed)`: the trait of `flag` requires the trait of `needed`.
    const REQUIRES: &'static [(Self, Self)] = &[
        (Self::Copy, Self::Clone),
        (Self::Eq, Self::PartialEq),
        (Self::PartialOrd, Self::PartialEq),
        (Self::Ord, Self::Eq),
        (Self::Ord, Self::PartialOrd),
    ];

    /// The trait's path, which is also its derive macro's.
    fn path(self) -> TokenStream {
        return match self {
            Self::Clone => quote! { ::core::clone::Clone },
            Self::Copy => quote! { ::core::marker::Copy },
            Self::Debug => quote! { ::core::fmt::Debug },
            Self::Default => quote! { ::core::default::Default },
            Self::PartialEq => quote! { ::core::cmp::PartialEq },
            Self::Eq => quote! { ::core::cmp::Eq },
            Self::Hash => quote! { ::core::hash::Hash },
            Self::PartialOrd => quote! { ::core::cmp::PartialOrd },
            Self::Ord => quote! { ::core::cmp::Ord },
        };
    }
}

/// The `without` grammar of a tree: its derive flags only.
const GRAMMAR: FlagGrammar = FlagGrammar {
    konsts: &[],
    removals: false,
};

/// A node of the tree: a branch trait or a leaf struct, with its doc
/// comments.
#[derive(Clone)]
struct Node {
    docs: Vec<Attribute>,
    visibility: Visibility,
    ident: Ident,
    kind: NodeKind,
}

#[derive(Clone)]
enum NodeKind {
    /// A leaf and its `type NAME = TYPE;` bindings.
    Leaf(Vec<Binding>),
    /// A branch, the associated types it declares, and its children.
    Branch {
        declarations: Vec<Declaration>,
        children: Vec<Node>,
    },
}

/// `type NAME = TYPE;` in a leaf.
#[derive(Clone)]
struct Binding {
    name: Ident,
    value: Type,
}

/// `type NAME: BOUNDS` in a branch, written or from `type NAME = A <-> B`.
#[derive(Clone)]
struct Declaration {
    docs: Vec<Attribute>,
    name: Ident,
    bounds: Punctuated<TypeParamBound, Token![+]>,
}

/// One entry of a node's braces.
enum Entry {
    Node(Node),
    Binding(Box<Binding>),
    Declaration(Declaration),
    /// `type NAME = A <-> B` or `type NAME = [A <-> B, ...]`: a declaration
    /// and the leaves it pairs.
    Pairs(Declaration, Vec<(PairMember, PairMember)>),
}

/// One side of `A <-> B`: a leaf the pair declares.
#[derive(Clone)]
struct PairMember {
    docs: Vec<Attribute>,
    visibility: Visibility,
    ident: Ident,
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return Self::parse_entries(input);
    }
}

/// Outer attributes, which must all be doc comments.
fn parse_docs(input: ParseStream) -> syn::Result<Vec<Attribute>> {
    let attributes = input.call(Attribute::parse_outer)?;
    if let Some(other) = attributes.iter().find(|it| !it.path().is_ident("doc"))
    {
        return other.fail("marker tree nodes accept only doc comments");
    }
    return Ok(attributes);
}

impl Parse for PairMember {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return Ok(Self {
            docs: parse_docs(input)?,
            visibility: input.parse()?,
            ident: input.parse()?,
        });
    }
}

/// `A <-> B`.
fn parse_pair(input: ParseStream) -> syn::Result<(PairMember, PairMember)> {
    let lhs = input.parse::<PairMember>()?;
    input.parse::<Token![<]>()?;
    input.parse::<Token![->]>()?;
    let rhs = input.parse::<PairMember>()?;
    return Ok((lhs, rhs));
}

impl Entry {
    /// One entry of a node's braces; `branch` names the node, which a pair
    /// declaration bounds itself by. A binding consumes its `;`.
    fn parse(
        input: ParseStream,
        branch: &Ident,
    ) -> syn::Result<Self> {
        let docs = parse_docs(input)?;
        if !input.peek(Token![type]) {
            let mut node = input.parse::<Node>()?;
            node.docs = docs;
            return Ok(Self::Node(node));
        }

        input.parse::<Token![type]>()?;
        let name: Ident = input.parse()?;
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            if let Some(pairs) = Self::parse_pairs(input)? {
                let bounds = parse_quote! { #branch<#name = Self> };
                let declaration = Declaration { docs, name, bounds };
                return Ok(Self::Pairs(declaration, pairs));
            }
            let value: Type = input.parse()?;
            input.parse::<Token![;]>()?;
            return Ok(Self::Binding(Box::new(Binding { name, value })));
        }

        let bounds = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            Punctuated::parse_separated_nonempty(input)?
        }
        else {
            Punctuated::new()
        };
        return Ok(Self::Declaration(Declaration { docs, name, bounds }));
    }

    /// `A <-> B` or `[A <-> B, ...]` when the input holds one, consumed.
    fn parse_pairs(
        input: ParseStream
    ) -> syn::Result<Option<Vec<(PairMember, PairMember)>>> {
        let fork = input.fork();
        let pairs = if fork.peek(syn::token::Bracket) {
            let content;
            let _ = bracketed!(content in fork);
            Punctuated::<_, Token![,]>::parse_terminated_with(
                &content,
                parse_pair,
            )
            .map(|pairs| pairs.into_iter().vec())
        }
        else {
            parse_pair(&fork).map(|pair| vec![pair])
        };
        let Ok(pairs) = pairs
        else {
            return Ok(None);
        };
        input.advance_to(&fork);
        return Ok(Some(pairs));
    }
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let docs = parse_docs(input)?;
        let visibility: Visibility = input.parse()?;
        let ident: Ident = input.parse()?;
        if !input.peek(syn::token::Brace) {
            return Ok(Self {
                docs,
                visibility,
                ident,
                kind: NodeKind::Leaf(Vec::new()),
            });
        }

        let content;
        let _ = braced!(content in input);
        let mut entries = Vec::new();
        while !content.is_empty() {
            let entry = Entry::parse(&content, &ident)?;
            let binding = matches!(entry, Entry::Binding(_));
            entries.push(entry);
            if !binding && !content.is_empty() {
                content.parse::<Token![,]>()?;
            }
        }

        return Ok(Self {
            docs,
            visibility,
            kind: NodeKind::of(&ident, entries)?,
            ident,
        });
    }
}

impl NodeKind {
    /// A leaf when every entry binds, else a branch with at least one child.
    fn of(
        ident: &Ident,
        entries: Vec<Entry>,
    ) -> syn::Result<Self> {
        if !entries.is_empty()
            && entries.iter().all(|it| matches!(it, Entry::Binding(_)))
        {
            let bindings = entries.into_iter().filter_map(|it| match it {
                Entry::Binding(binding) => Some(*binding),
                _ => None,
            });
            return Ok(Self::Leaf(bindings.vec()));
        }

        let mut declarations = Vec::new();
        let mut children = Vec::new();
        for entry in entries {
            match entry {
                Entry::Node(node) => children.push(node),
                Entry::Declaration(declaration) => {
                    declarations.push(declaration);
                }
                Entry::Pairs(declaration, pairs) => {
                    children.extend(Node::paired(&declaration.name, pairs));
                    declarations.push(declaration);
                }
                Entry::Binding(binding) => {
                    return binding.name.fail(
                        "a node holds either children or `type NAME = \
                        TYPE;` bindings",
                    );
                }
            }
        }
        if children.is_empty() {
            return ident
                .fail("marker tree branches require at least one child");
        }
        return Ok(Self::Branch {
            declarations,
            children,
        });
    }
}

impl Node {
    /// The leaves `type NAME = A <-> B` declares: each binds `NAME` to the
    /// other; `A <-> A` declares one leaf bound to itself.
    fn paired(
        name: &Ident,
        pairs: Vec<(PairMember, PairMember)>,
    ) -> Vec<Self> {
        let leaf = |member: PairMember, other: &Ident| {
            return Self {
                docs: member.docs,
                visibility: member.visibility,
                ident: member.ident,
                kind: NodeKind::Leaf(vec![
                    Binding {
                        name: name.clone(),
                        value: parse_quote! { #other },
                    },
                ]),
            };
        };

        let mut leaves = Vec::new();
        for (lhs, rhs) in pairs {
            if lhs.ident == rhs.ident {
                let ident = lhs.ident.clone();
                leaves.push(leaf(lhs, &ident));
                continue;
            }
            let (lhs_ident, rhs_ident) = (lhs.ident.clone(), rhs.ident.clone());
            leaves.push(leaf(lhs, &rhs_ident));
            leaves.push(leaf(rhs, &lhs_ident));
        }
        return leaves;
    }
}

impl Cfg {
    /// `NODE`, or `[ENTRY, ...]` where an entry is a node or
    /// `without = FLAG | [FLAG, ...]`.
    pub(crate) fn parse_attribute(input: ParseStream) -> syn::Result<Self> {
        if !input.peek(syn::token::Bracket) {
            let node = input.parse::<Node>()?;
            return Self::from_parts(
                vec![node],
                FlagSettings::default(),
                input.span(),
            );
        }

        let content;
        let _ = bracketed!(content in input);
        return Self::parse_entries(&content);
    }

    /// Comma-separated nodes and at most one `without = FLAGS`, in any
    /// order.
    fn parse_entries(input: ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();
        let mut settings = FlagSettings::default();
        let mut without = None;
        while !input.is_empty() {
            if input.peek2(Token![=]) {
                let key: Ident = input.parse()?;
                if zz::Key::of(&key.to_string()) != Some(zz::Key::Without) {
                    return key.span().fail("unknown attribute");
                }
                if without.replace(key.span()).is_some() {
                    return key.span().fail("duplicated arg");
                }
                input.parse::<Token![=]>()?;
                settings.parse(input, false, &GRAMMAR)?;
            }
            else {
                nodes.push(input.parse::<Node>()?);
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        return Self::from_parts(nodes, settings, input.span());
    }

    /// The nodes and the derive flags `settings` leaves on; a flag whose
    /// trait needs a removed one is an error at the removal.
    fn from_parts(
        nodes: Vec<Node>,
        settings: FlagSettings<TreeFlag>,
        span: Span,
    ) -> syn::Result<Self> {
        if nodes.is_empty() {
            return Err(syn::Error::new(
                span,
                "marker tree requires at least one node",
            ));
        }

        let mut flags = TreeFlags::default();
        settings.apply(&mut flags);
        for &(flag, needed) in TreeFlag::REQUIRES {
            if flags.has(flag) && !flags.has(needed) {
                let (_, removed) = settings
                    .flag(needed)
                    .expect("only `without` turns a tree flag off");
                return removed.fail(format!(
                    "`{}` needs `{}`; remove both",
                    flag.name(),
                    needed.name(),
                ));
            }
        }

        return Ok(Self { nodes, flags });
    }

    /// The tree in a module below the call site, re-exported there.
    /// `anchored`: its paths were already re-anchored one module deeper,
    /// as an owner's `hide_in` does.
    pub(crate) fn expand_embedded(
        &self,
        owner: &Ident,
        visibility: &Visibility,
        anchored: bool,
    ) -> syn::Result<TokenStream> {
        let seal = format_ident!("MarkerTreeRoot");
        let nodes = self.resolve(visibility, &seal)?;
        let definitions = EmittedNode::emit_definitions(
            &nodes,
            Placement::Embedded { anchored },
            self.flags,
        );
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
                #[allow(unused_imports)]
                use super::*;

                trait #seal {}

                #definitions
            }

            #(#exports)*
        });
    }

    /// The nodes to emit under `seal`, checked: names are unique, every
    /// branch's associated types are written or inferred, and every leaf
    /// binds exactly what its ancestors declare.
    fn resolve(
        &self,
        visibility: &Visibility,
        seal: &Ident,
    ) -> syn::Result<Vec<EmittedNode>> {
        let root = vec![seal.clone()];
        let mut names = BTreeSet::from([seal.to_string()]);
        let mut chains = BTreeMap::new();
        for node in &self.nodes {
            node.index(&root, &mut names, &mut chains)?;
        }

        let mut emitted = Vec::new();
        for node in &self.nodes {
            node.resolve(visibility, &root, &[], &chains, &mut emitted)?;
        }
        return Ok(emitted);
    }
}

/// A leaf's ancestors, the seal first, by leaf name.
type Chains = BTreeMap<String, Vec<Ident>>;

/// An associated type name visible to a node and the branch declaring it.
type Declared = Vec<(Ident, Ident)>;

impl Node {
    /// Rejects a repeated name and records each leaf's ancestors.
    fn index(
        &self,
        ancestors: &[Ident],
        names: &mut BTreeSet<String>,
        chains: &mut Chains,
    ) -> syn::Result<()> {
        if !names.insert(self.ident.to_string()) {
            return self.ident.fail("duplicated marker tree node");
        }
        match &self.kind {
            NodeKind::Leaf(_) => {
                chains.insert(self.ident.to_string(), ancestors.to_vec());
            }
            NodeKind::Branch { children, .. } => {
                let mut ancestors = ancestors.to_vec();
                ancestors.push(self.ident.clone());
                for child in children {
                    child.index(&ancestors, names, chains)?;
                }
            }
        }
        return Ok(());
    }

    fn resolve(
        &self,
        parent_visibility: &Visibility,
        ancestors: &[Ident],
        declared: &[(Ident, Ident)],
        chains: &Chains,
        emitted: &mut Vec<EmittedNode>,
    ) -> syn::Result<()> {
        let visibility = match &self.visibility {
            Visibility::Inherited => parent_visibility.clone(),
            visibility => visibility.clone(),
        };
        let mut node = EmittedNode {
            docs: self.docs.clone(),
            visibility: visibility.clone(),
            ident: self.ident.clone(),
            ancestors: ancestors.to_vec(),
            branch: None,
            bindings: Vec::new(),
        };

        let (declarations, children) = match &self.kind {
            NodeKind::Leaf(bindings) => {
                node.bindings = self.bind(bindings, ancestors, declared)?;
                emitted.push(node);
                return Ok(());
            }
            NodeKind::Branch {
                declarations,
                children,
            } => (declarations, children),
        };

        let mut declared = declared.to_vec();
        let mut own = Vec::new();
        for declaration in declarations {
            self.declare(&mut declared, &mut own, declaration.clone())?;
        }
        for declaration in Self::infer(children, &declared, chains) {
            self.declare(&mut declared, &mut own, declaration)?;
        }
        node.branch = Some(own);
        emitted.push(node);

        let mut ancestors = ancestors.to_vec();
        ancestors.push(self.ident.clone());
        for child in children {
            child.resolve(
                &visibility,
                &ancestors,
                &declared,
                chains,
                emitted,
            )?;
        }
        return Ok(());
    }

    /// Adds `declaration` to this branch, rejecting a name this branch or an
    /// ancestor already declares.
    fn declare(
        &self,
        declared: &mut Declared,
        own: &mut Vec<Declaration>,
        declaration: Declaration,
    ) -> syn::Result<()> {
        let name = &declaration.name;
        if let Some((_, owner)) = declared.iter().find(|(it, _)| it == name) {
            return match owner == &self.ident {
                true => name.fail(format!("duplicated `type {name}`")),
                false => name.fail(format!(
                    "`type {name}` is already declared by `{owner}`"
                )),
            };
        }
        declared.push((name.clone(), self.ident.clone()));
        own.push(declaration);
        return Ok(());
    }

    /// The declarations a branch gets from its leaf children's bindings of
    /// names no ancestor declares, in first-binding order. Each is bounded by
    /// the deepest node above every leaf it is bound to, or unbounded when
    /// a bound type is not a leaf of this tree.
    fn infer(
        children: &[Self],
        declared: &[(Ident, Ident)],
        chains: &Chains,
    ) -> Vec<Declaration> {
        let mut values = Vec::<(Ident, Vec<(&Ident, &Type)>)>::new();
        for child in children {
            let NodeKind::Leaf(bindings) = &child.kind
            else {
                continue;
            };
            for binding in bindings {
                if declared.iter().any(|(name, _)| name == &binding.name) {
                    continue;
                }
                let value = (&child.ident, &binding.value);
                match values.iter_mut().find(|(name, _)| name == &binding.name)
                {
                    Some((_, bound)) => bound.push(value),
                    None => values.push((binding.name.clone(), vec![value])),
                }
            }
        }

        return values
            .into_iter()
            .map(|(name, bound)| Declaration {
                docs: Vec::new(),
                bounds: Self::common_bound(&bound, chains),
                name,
            })
            .vec();
    }

    /// The deepest node above every leaf in `values` (`Self` is the binding
    /// leaf), or no bound when a value is not a leaf of this tree.
    fn common_bound(
        values: &[(&Ident, &Type)],
        chains: &Chains,
    ) -> Punctuated<TypeParamBound, Token![+]> {
        let mut common: Option<Vec<Ident>> = None;
        for (leaf, value) in values {
            let target = match value {
                Type::Path(path) if path.qself.is_none() => {
                    path.path.get_ident()
                }
                _ => None,
            };
            let Some(target) = target
            else {
                return Punctuated::new();
            };
            let target = if target == "Self" { *leaf } else { target };
            let Some(chain) = chains.get(&target.to_string())
            else {
                return Punctuated::new();
            };
            common = Some(match common {
                None => chain.clone(),
                Some(prefix) => prefix
                    .into_iter()
                    .zip(chain)
                    .take_while(|(lhs, rhs)| lhs == *rhs)
                    .map(|(lhs, _)| lhs)
                    .vec(),
            });
        }
        let deepest = common
            .and_then(|it| it.last().cloned())
            .expect("leaf chains start at the seal");
        return parse_quote! { #deepest };
    }

    /// A leaf's bindings per ancestor, in `ancestors` and declaration order:
    /// exactly one for every name `declared`.
    fn bind(
        &self,
        bindings: &[Binding],
        ancestors: &[Ident],
        declared: &[(Ident, Ident)],
    ) -> syn::Result<Vec<Vec<Binding>>> {
        let leaf = &self.ident;
        for (index, binding) in bindings.iter().enumerate() {
            let name = &binding.name;
            if bindings[..index].iter().any(|it| &it.name == name) {
                return name.fail(format!("duplicated `type {name}`"));
            }
            if !declared.iter().any(|(it, _)| it == name) {
                return name.fail(format!(
                    "`type {name}` needs a branch above `{leaf}` to declare it"
                ));
            }
        }
        for (name, owner) in declared {
            if !bindings.iter().any(|it| &it.name == name) {
                return leaf.fail(format!(
                    "`{leaf}` needs `type {name}`, declared by `{owner}`"
                ));
            }
        }

        return Ok(ancestors
            .iter()
            .map(|ancestor| {
                return declared
                    .iter()
                    .filter(|(_, owner)| owner == ancestor)
                    .filter_map(|(name, _)| {
                        bindings.iter().find(|it| &it.name == name).cloned()
                    })
                    .vec();
            })
            .vec());
    }
}

/// Where the marker tree's items live: beside the user's root trait, or in a
/// hidden module embedded by an owning macro, one module below the call site
/// and re-exported there. `anchored`: the owner already re-anchored the
/// tree's paths for that depth.
#[derive(Clone, Copy)]
enum Placement {
    Standalone,
    Embedded { anchored: bool },
}

impl Placement {
    /// `tokens` re-anchored to resolve from where the items live.
    fn anchor(
        self,
        tokens: &impl ToTokens,
    ) -> TokenStream {
        let tokens = quote! { #tokens };
        return match self {
            Self::Embedded { anchored: false } => rewrite_child_paths(tokens),
            Self::Standalone | Self::Embedded { anchored: true } => tokens,
        };
    }
}

struct EmittedNode {
    docs: Vec<Attribute>,
    visibility: Visibility,
    ident: Ident,
    ancestors: Vec<Ident>,
    /// A branch's associated types, written then inferred; `None` for a
    /// leaf.
    branch: Option<Vec<Declaration>>,
    /// A leaf's bindings for each of `ancestors`.
    bindings: Vec<Vec<Binding>>,
}

impl EmittedNode {
    fn emit_definitions(
        nodes: &[Self],
        placement: Placement,
        flags: TreeFlags,
    ) -> TokenStream {
        let traits = flags.iter().map(TreeFlag::path).vec();
        let branches = nodes.iter().filter_map(|node| {
            let declarations = node.branch.as_ref()?;
            return Some(node.emit_branch(placement, &traits, declarations));
        });
        let leaves = nodes
            .iter()
            .filter(|node| node.branch.is_none())
            .map(|node| node.emit_leaf(placement, &traits));

        return quote! {
            #(#branches)*
            #(#leaves)*
        };
    }

    /// A branch trait and its associated types. Top-level branches also
    /// require `traits`, which nested branches inherit through their parent.
    fn emit_branch(
        &self,
        placement: Placement,
        traits: &[TokenStream],
        declarations: &[Declaration],
    ) -> TokenStream {
        let docs = &self.docs;
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
            quote! { #parent #(+ #traits)* }
        }
        else {
            quote! { #parent + #seal }
        };
        let types = declarations.iter().map(|declaration| {
            let type_docs = &declaration.docs;
            let name = &declaration.name;
            let type_bounds = placement.anchor(&declaration.bounds);
            let colon = (!type_bounds.is_empty()).then(|| quote! { : });
            return quote! {
                #(#type_docs)*
                type #name #colon #type_bounds;
            };
        });

        return quote! {
            #(#docs)*
            #[allow(private_bounds)]
            #visibility trait #ident: #bounds {
                #(#types)*
            }
        };
    }

    /// A zero-sized leaf deriving `traits` and implementing its ancestors
    /// with its bindings.
    fn emit_leaf(
        &self,
        placement: Placement,
        traits: &[TokenStream],
    ) -> TokenStream {
        let docs = &self.docs;
        let visibility = self.emit_visibility(placement);
        let ident = &self.ident;
        let derive = (!traits.is_empty()).then(|| {
            quote! { #[derive(#(#traits),*)] }
        });
        let impls = self.ancestors.iter().zip(&self.bindings).map(
            |(ancestor, bindings)| {
                let types = bindings.iter().map(|Binding { name, value }| {
                    let value = placement.anchor(value);
                    return quote! { type #name = #value; };
                });
                return quote! {
                    impl #ancestor for #ident {
                        #(#types)*
                    }
                };
            },
        );

        return quote! {
            #(#docs)*
            #derive
            #visibility struct #ident;
            #(#impls)*
        };
    }

    /// The declared visibility, re-anchored when embedded so the reach seen
    /// by privacy checks (E0446 on bindings) is the declared one.
    fn emit_visibility(
        &self,
        placement: Placement,
    ) -> Visibility {
        return match placement {
            Placement::Embedded { .. } => inner_visibility(&self.visibility),
            Placement::Standalone => self.visibility.clone(),
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

    let nodes = cfg.resolve(&root.vis, &root.ident)?;
    let definitions =
        EmittedNode::emit_definitions(&nodes, Placement::Standalone, cfg.flags);

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
        anchored: bool,
    ) -> syn::Result<TokenStream> {
        return self.expand_embedded(owner, visibility, anchored);
    }
}
