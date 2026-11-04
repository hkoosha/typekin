use std::collections::BTreeSet;
use std::fmt::{
    Debug,
    Formatter,
};

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::quote;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::spanned::Spanned;
use syn::{
    Item,
    Path,
    Token,
};

use crate::zz_friend::Capability;
use crate::zz_lib as zz;
use crate::zz_lib::{
    Key,
    PathIdentity,
};
use crate::zz_names::{
    NameKey,
    Names,
};
use crate::zz_runner::ToCollection;

/// A name the constructor generates.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ConstructorName {
    /// `of` unless `maker` names one.
    Maker,
}

impl NameKey for ConstructorName {
    type Owner = ();

    fn default_name(
        self,
        _owner: &(),
    ) -> String {
        return match self {
            Self::Maker => "of".to_string(),
        };
    }
}
/// The attribute carrying the paired friendship configuration.
const FRIENDSHIP: &str = "friendship";

#[derive(Clone)]
pub(crate) struct Cfg {
    pub(crate) relationship: Path,
    /// `maker`.
    pub(crate) names: Names<ConstructorName>,
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) scope: Scope,
}

impl Debug for Cfg {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return f.debug_struct("Cfg").finish_non_exhaustive();
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Friend {
    pub(crate) ty: PathIdentity,
}

impl Parse for Friend {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty = input.parse::<Path>()?;
        return Ok(Self {
            ty: PathIdentity::new(ty),
        });
    }
}

#[derive(Clone)]
pub(crate) struct ModuleCfg {
    pub(crate) visibility: syn::Visibility,
    pub(crate) name: Ident,
}

impl Parse for ModuleCfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility = input.parse()?;
        let name = input.parse()?;

        return Ok(Self { visibility, name });
    }
}

#[derive(Clone)]
pub(crate) enum Scope {
    Self_,
    Const,
    Module(ModuleCfg),
}

impl Scope {
    pub(crate) fn module(&self) -> Option<&ModuleCfg> {
        return match self {
            Self::Module(module) => Some(module),
            Self::Self_ | Self::Const => None,
        };
    }

    pub(crate) fn wrap(
        &self,
        items: TokenStream,
    ) -> TokenStream {
        return match self {
            Self::Self_ => items,
            Self::Const => quote! {
                const _: () = {
                    #items
                };
            },
            Self::Module(module) => {
                let visibility = &module.visibility;
                let name = &module.name;

                quote! {
                    #visibility mod #name {
                        #items
                    }
                }
            }
        };
    }
}

impl Parse for Scope {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![self]) {
            let _: Token![self] = input.parse()?;
            return Ok(Self::Self_);
        }
        if input.peek(Token![_]) {
            let _: Token![_] = input.parse()?;
            return Ok(Self::Const);
        }

        return Ok(Self::Module(input.parse()?));
    }
}

impl Cfg {
    pub(crate) fn in_self_scope(&self) -> Self {
        let mut this = self.clone();
        this.scope = Scope::Self_;
        return this;
    }
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut relationship: Option<Path> = None;
        let mut names = Names::default();
        let mut friends = BTreeSet::new();
        let mut scope = None;

        zz::parse_inner(input, |attr, stream| {
            match attr {
                Key::Relationship => relationship = Some(stream.parse()?),
                Key::Maker => {
                    names.choose(ConstructorName::Maker, stream.parse()?);
                }
                Key::Friend => friends = zz::one_or_list(stream)?.set(),
                Key::Scope => scope = Some(stream.parse()?),
                _ => return Ok(false),
            };

            return Ok(true);
        })?;

        let this = Self {
            relationship: relationship.ok_or_else(|| {
                syn::Error::new(
                    input.span(),
                    "missing required `relationship` argument",
                )
            })?,
            scope: scope.ok_or_else(|| {
                syn::Error::new(
                    input.span(),
                    "missing required `scope` argument",
                )
            })?,
            names,
            friends,
        };
        return Ok(this);
    }
}

pub(crate) struct Protocol {
    pub(crate) target: TokenStream,
    pub(crate) visibility: syn::Visibility,
    pub(crate) relation: TokenStream,
    pub(crate) value: TokenStream,
    pub(crate) seal: Ident,
    pub(crate) conversion: Ident,
}

pub(crate) fn expand(
    cfg: &Cfg,
    protocol: Protocol,
) -> TokenStream {
    let target = &protocol.target;
    let visibility = &protocol.visibility;
    let of_relation = &cfg.relationship;
    let maker = cfg
        .names
        .clone()
        .resolve((), BTreeSet::new())
        .get(ConstructorName::Maker);
    let relation = &protocol.relation;
    let value = &protocol.value;
    let seal = &protocol.seal;
    let conversion = &protocol.conversion;
    let make = Capability::Make.ident();

    let items = quote! {
        impl #target {
            #[allow(private_bounds)]
            #[inline(always)]
            #visibility fn #maker<T>(it: T) -> Self
            where
                T: #make + #seal,
            {
                return #of_relation(<T as #make>::#conversion(it));
            }
        }

        impl #seal for #relation {}

        impl #make for #relation {
            #[inline(always)]
            fn #conversion(self) -> #value {
                return self;
            }
        }
    };

    return cfg.scope.wrap(items);
}

pub(crate) fn ekran(
    attr: Cfg,
    mut item: Item,
) -> syn::Result<TokenStream> {
    let (_, _, attrs) = zz::get_concrete_type(&mut item)?;
    let friendship = zz::pop_attr(attrs, FRIENDSHIP)?
        .ok_or_else(|| {
            syn::Error::new(
                item.span(),
                "`#[typekin::constructor]` requires \
                 `#[typekin::friendship(...)]` immediately below it",
            )
        })?
        .parse_args::<crate::friendship::Cfg>()?;

    return crate::friendship::expand_with_constructor(item, friendship, attr);
}
