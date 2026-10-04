use std::collections::BTreeSet;

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    ToTokens,
    quote,
};
use syn::{
    Item,
    Path,
    Token,
    parse::{
        Parse,
        ParseStream,
    },
    spanned::Spanned,
};

use crate::{
    runner,
    zz,
};

#[derive(Clone)]
pub(crate) struct Cfg {
    pub(crate) relationship: Path,
    pub(crate) maker: Ident,
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) scope: Scope,
}

#[derive(Clone)]
pub(crate) struct Friend {
    pub(crate) identity: String,
    pub(crate) ty: Path,
}

impl Parse for Friend {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Path = input.parse()?;

        return Ok(Self {
            identity: ty.to_token_stream().to_string(),
            ty,
        });
    }
}

impl Eq for Friend {}

impl PartialOrd for Friend {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Friend {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        return self.identity == other.identity;
    }
}

impl Ord for Friend {
    fn cmp(
        &self,
        other: &Self,
    ) -> std::cmp::Ordering {
        return self.identity.cmp(&other.identity);
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
        let mut maker = quote::format_ident!("of");
        let mut friends = BTreeSet::new();
        let mut scope = None;

        zz::parse_inner(input, |attr, stream| {
            match attr {
                "relationship" => relationship = Some(stream.parse()?),
                "maker" => maker = stream.parse()?,
                "friends" => friends = zz::one_or_list(stream)?.collect(),
                "scope" => scope = Some(stream.parse()?),
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
            scope: scope.or(None).ok_or_else(|| {
                syn::Error::new(input.span(), "missing required `mod` argument")
            })?,
            maker,
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
    let maker = &cfg.maker;
    let relation = &protocol.relation;
    let value = &protocol.value;
    let seal = &protocol.seal;
    let conversion = &protocol.conversion;
    let make = quote::format_ident!("Make");

    let items = quote! {
        impl #target {
            #[allow(private_bounds)]
            #[inline(always)]
            #visibility fn #maker<T>(it: T) -> Self
            where
                T: #make + #seal,
            {
                return #of_relation(<T as #seal>::#conversion(it));
            }
        }

        impl #seal for #relation {
            #[inline(always)]
            fn #conversion(self) -> #value {
                return self;
            }
        }

        impl #make for #relation {}
    };

    return cfg.scope.wrap(items);
}

pub(crate) fn ekran(
    attr: Cfg,
    mut item: Item,
) -> proc_macro::TokenStream {
    return runner::catching(move || {
        let (_, _, attrs) = zz::get_concrete_type(&mut item)?;
        let friendship = zz::pop_attr(attrs, "friendship")?
            .ok_or_else(|| {
                syn::Error::new(
                    item.span(),
                    "`#[typekin::constructor]` requires \
                     `#[typekin::friendship(...)]` immediately below it",
                )
            })?
            .parse_args::<crate::friendship::cfg::Cfg>()?;

        return crate::friendship::expand_with_constructor(
            item, friendship, attr,
        );
    });
}
