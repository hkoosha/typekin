use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::{
    Item,
    Type,
};

pub(crate) use self::cfg::Cfg;
use crate::friendship::cfg::Friend;
use crate::runner::MkErr;
use crate::{
    constructor,
    runner,
    zz,
};

pub(crate) mod cfg {
    use std::cmp::Ordering;
    use std::collections::BTreeSet;

    use proc_macro2::Ident;
    use quote::{
        ToTokens,
        format_ident,
    };
    use syn::parse::{
        Parse,
        ParseStream,
    };
    use syn::{
        Path,
        Token,
        Type,
    };

    use crate::runner::{
        MkErr,
        ToCollection,
    };
    use crate::zz::{
        arg_or_list,
        one_or_list,
    };
    use crate::{
        constructor,
        zz,
    };

    #[derive(Clone)]
    pub(crate) struct Friend {
        pub(crate) identity: String,
        pub(crate) ty: Path,
        pub(crate) capabilities: BTreeSet<Ident>,
        pub(crate) conv: Option<Path>,
    }

    impl Parse for Friend {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            return Self::parse_with_defaults(input, BTreeSet::new());
        }
    }

    impl Friend {
        pub(crate) fn parse_with_defaults(
            input: ParseStream,
            capabilities: BTreeSet<Ident>,
        ) -> syn::Result<Self> {
            let conv = input.parse::<Path>()?;

            let here = input.span();
            let mut ty = arg_or_list::<Path>(input)?;
            let ty =
                match (ty.pop(), ty) {
                    (Some(it), rest) if rest.is_empty() => it,
                    _ => return here.fail(
                        "friend conversion requires exactly one source type",
                    ),
                };

            let capabilities = if input.peek(Token![->]) {
                let _ = input.parse::<Token![->]>()?;
                one_or_list(input)?.set()
            }
            else {
                capabilities
            };

            return Ok(Self {
                identity: ty.to_token_stream().to_string(),
                conv: Some(conv),
                capabilities,
                ty,
            });
        }
    }

    impl Eq for Friend {}

    impl PartialOrd for Friend {
        fn partial_cmp(
            &self,
            other: &Self,
        ) -> Option<Ordering> {
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
        ) -> Ordering {
            return self.identity.cmp(&other.identity);
        }
    }

    impl From<&Path> for Friend {
        fn from(it: &Path) -> Self {
            return Self {
                identity: it.to_token_stream().to_string(),
                ty: it.clone(),
                conv: None,
                capabilities: BTreeSet::new(),
            };
        }
    }

    impl ::std::fmt::Debug for Friend {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "Friend[{}::{:?}]", self.identity, self.capabilities)
        }
    }

    impl From<&Ident> for Friend {
        fn from(it: &Ident) -> Self {
            let ty: Path = syn::parse_quote! { #it };
            return <Self as From<&Path>>::from(&ty);
        }
    }

    // =========================================================================

    pub(crate) struct Cfg {
        pub(super) relation: Option<Type>,
        pub(super) value: Option<Type>,
        pub(super) seal: Option<Ident>,
        pub(super) conversion: Option<Ident>,
        pub(super) module: Option<constructor::ModuleCfg>,
        pub(super) scope: Option<constructor::Scope>,
        pub(super) friends: BTreeSet<Friend>,
        pub(super) extra_caps: BTreeSet<Ident>,
    }

    impl Cfg {
        pub(super) fn constructor(&self) -> constructor::Cfg {
            let friends = self
                .friends
                .iter()
                .map(|friend| {
                    return constructor::Friend {
                        identity: friend.identity.clone(),
                        ty: friend.ty.clone(),
                    };
                })
                .set();

            return constructor::Cfg {
                relationship: syn::parse_quote! { Self::of_parts },
                maker: format_ident!("of"),
                friends,
                scope: self.scope.clone().unwrap_or(constructor::Scope::Const),
            };
        }

        pub(super) fn capabilities(
            &self,
            include_constructor: bool,
        ) -> BTreeSet<Ident> {
            let mut capabilities = self
                .friends
                .iter()
                .flat_map(|friend| friend.capabilities.iter().cloned())
                .set();

            if include_constructor {
                capabilities.insert(format_ident!("Make"));
            }

            return capabilities;
        }
    }

    impl Parse for Cfg {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let mut this = Self {
                relation: None,
                value: None,
                seal: None,
                conversion: None,
                module: None,
                scope: None,
                friends: BTreeSet::new(),
                extra_caps: BTreeSet::new(),
            };

            zz::parse_inner(input, |attr, stream| {
                match attr {
                    "relation" => this.relation = Some(stream.parse()?),
                    "value" => this.value = Some(stream.parse()?),
                    "seal" => this.seal = Some(stream.parse()?),
                    "conversion" => this.conversion = Some(stream.parse()?),
                    "mod" => {
                        if stream.peek(Token![_]) {
                            let _: Token![_] = stream.parse()?;
                        }
                        else {
                            this.module = Some(stream.parse()?);
                        }
                    }
                    "scope" => this.scope = Some(stream.parse()?),
                    "friends" => {
                        for friend in one_or_list(stream)? {
                            if let Some(mut existing) =
                                this.friends.take(&friend)
                            {
                                if !existing
                                    .capabilities
                                    .is_disjoint(&friend.capabilities)
                                {
                                    return stream.span().fail(
                                        "friend capabilities overlap for a source type",
                                    );
                                }

                                existing
                                    .capabilities
                                    .extend(friend.capabilities);
                                existing.conv = friend.conv;
                                this.friends.insert(existing);
                            }
                            else {
                                this.friends.insert(friend);
                            }
                        }
                    }
                    "extra_caps" => {
                        this.extra_caps = one_or_list::<Ident>(stream)?.set()
                    }
                    _ => return Ok(false),
                };

                return Ok(true);
            })?;

            if this.module.is_some() && this.scope.is_some() {
                return input
                    .span()
                    .fail("`mod` and `scope` cannot be used together");
            }

            return Ok(this);
        }
    }
}

#[derive(Clone)]
pub(crate) struct ProtocolFriend {
    pub(crate) ty: syn::Path,
    pub(crate) capabilities: std::collections::BTreeSet<Ident>,
    pub(crate) conversion: Option<TokenStream>,
}

pub(crate) struct Construction {
    pub(crate) make: Ident,
    pub(crate) trusted: Ident,
    pub(crate) checked: TokenStream,
}

pub(crate) struct Protocol {
    pub(crate) target: Ident,
    pub(crate) relation: Type,
    pub(crate) seal: Ident,
    pub(crate) conversion: Ident,
    pub(crate) capabilities: Vec<Ident>,
    pub(crate) friends: Vec<ProtocolFriend>,
    pub(crate) emit_seal: bool,
    pub(crate) target_conversion: Option<TokenStream>,
    pub(crate) construction: Option<Construction>,
    pub(crate) konst: bool,
}

pub(crate) fn emit_protocol(protocol: Protocol) -> TokenStream {
    let target = &protocol.target;
    let relation = &protocol.relation;
    let seal = &protocol.seal;
    let conversion = &protocol.conversion;
    let konst = runner::konst(protocol.konst);
    let bonst = runner::bonst(protocol.konst);

    let seal_declaration = protocol.emit_seal.then(|| {
        quote! {
            #konst trait #seal {
                fn #conversion(self) -> #relation;
            }
        }
    });

    let capability_declarations =
        protocol.capabilities.iter().map(|capability| {
            let method =
                protocol.construction.as_ref().and_then(|construction| {
                    (capability == &construction.make).then(|| {
                        quote! { fn make(self) -> #target; }
                    })
                });

            quote! {
                #konst trait #capability: #bonst #seal {
                    #method
                }
            }
        });

    let target_seal = protocol.target_conversion.as_ref().map(|body| {
        quote! {
            #konst impl #seal for #target {
                #[inline(always)]
                fn #conversion(self) -> #relation {
                    #body
                }
            }
        }
    });

    let friend_impls = protocol.friends.iter().map(|friend| {
        let ty = &friend.ty;
        let seal_impl = friend.conversion.as_ref().map(|body| {
            quote! {
                #konst impl #seal for #ty {
                    #[inline(always)]
                    fn #conversion(self) -> #relation {
                        #body
                    }
                }
            }
        });
        let capability_impls = friend.capabilities.iter().map(|capability| {
            let method =
                protocol.construction.as_ref().and_then(|construction| {
                    if capability != &construction.make {
                        return None;
                    }

                    let constructor = match friend
                        .capabilities
                        .contains(&construction.trusted)
                    {
                        true => quote! { #target },
                        false => construction.checked.clone(),
                    };

                    Some(quote! {
                        #[inline(always)]
                        fn make(self) -> #target {
                            let raw = <Self as #seal>::#conversion(self);
                            return #constructor(raw);
                        }
                    })
                });

            quote! {
                #konst impl #capability for #ty {
                    #method
                }
            }
        });

        quote! {
            #seal_impl
            #(#capability_impls)*
        }
    });

    return quote! {
        #seal_declaration
        #(#capability_declarations)*

        #target_seal
        #(#friend_impls)*
    };
}

pub(crate) fn ekran(
    attr: Cfg,
    item: Item,
) -> proc_macro::TokenStream {
    return runner::catching(move || {
        let constructor = attr.constructor();
        let scope = constructor.scope.clone();

        return expand(item, attr, Some(constructor), Some(scope));
    });
}

pub(crate) fn expand_with_constructor(
    item: Item,
    friendship: Cfg,
    constructor: constructor::Cfg,
) -> syn::Result<TokenStream> {
    if friendship.module.is_some()
        && !matches!(constructor.scope, constructor::Scope::Self_,)
    {
        return item.fail(
            "`constructor mod` conflicts with the friendship protocol module",
        );
    }

    let scope = constructor.scope.clone();
    return expand(item, friendship, Some(constructor), Some(scope));
}

fn expand(
    mut item: Item,
    mut friendship: Cfg,
    constructor: Option<constructor::Cfg>,
    scope: Option<constructor::Scope>,
) -> syn::Result<TokenStream> {
    let (target, visibility, _) = zz::get_concrete_type(&mut item)?;
    let target = target.clone();
    let visibility = visibility.clone();
    let module = friendship
        .module
        .clone()
        .or_else(|| scope.as_ref().and_then(|scope| scope.module().cloned()));
    let in_self_scope = module.is_none()
        && scope.as_ref().is_some_and(|scope| {
            return matches!(scope, constructor::Scope::Self_);
        });

    if friendship.relation.is_none() {
        return item.fail("missing required argument: `relation`");
    }

    if let Some(constructor) = &constructor {
        grant_constructor(&mut friendship, constructor, &target)?;
    }

    if friendship.capabilities(constructor.is_some()).is_empty() {
        return item.fail("friendship requires at least one friend capability");
    }

    for friend in &friendship.friends {
        if friend.conv.is_none() {
            return item.fail(
                "concrete friendship requires `conversion(Type) -> Capabilities`",
            );
        }
    }

    let constructor = constructor.map(|constructor| {
        let relation = friendship
            .relation
            .as_ref()
            .expect("friendship relation is empty");
        let in_module = module.is_some();
        let relation = match relation {
            Type::Path(path) if path.qself.is_none() => {
                parent_path(&path.path, in_module)
            }
            _ => quote! { #relation },
        };
        let value = friendship
            .value
            .as_ref()
            .map_or_else(|| relation.clone(), |value| quote! { #value });
        let target = match in_module {
            true => quote! { super::#target },
            false => quote! { #target },
        };
        let seal = friendship
            .seal
            .clone()
            .unwrap_or_else(|| format_ident!("Seal"));
        let conversion = friendship.conversion.clone().unwrap_or_else(|| {
            let name =
                friendship.relation.as_ref().and_then(
                    |relation| match relation {
                        Type::Path(path) if path.qself.is_none() => path
                            .path
                            .segments
                            .last()
                            .map(|segment| segment.ident.to_string()),
                        _ => None,
                    },
                );

            name.map(|name| {
                format_ident!("to_{}", runner::snake_case_of(&name))
            })
            .unwrap_or_else(|| format_ident!("convert"))
        });

        return constructor::expand(
            &constructor.in_self_scope(),
            constructor::Protocol {
                target,
                visibility: visibility.clone(),
                relation,
                value,
                seal,
                conversion,
            },
        );
    });
    let protocol = emit_friendship(
        &friendship,
        &target,
        constructor.is_some(),
        constructor,
        module.as_ref(),
        in_self_scope,
    );

    return Ok(quote! {
        #item

        #protocol
    });
}

fn grant_constructor(
    cfg: &mut Cfg,
    constructor: &constructor::Cfg,
    target: &Ident,
) -> syn::Result<()> {
    for requested in &constructor.friends {
        let identity = match requested.ty.is_ident("Self") {
            true => target.to_string(),
            false => requested.identity.clone(),
        };
        let friend = cfg
            .friends
            .iter()
            .find(|friend| {
                return friend.identity == identity
                    || requested.ty.is_ident("Self")
                        && friend.ty.is_ident("Self");
            })
            .cloned()
            .ok_or_else(|| {
                syn::Error::new(
                    target.span(),
                    "constructor friend is not declared by friendship",
                )
            })?;

        let mut friend = cfg
            .friends
            .take(&friend)
            .expect("matched friendship not present");
        friend.capabilities.insert(format_ident!("Make"));
        cfg.friends.insert(friend);
    }

    return Ok(());
}

pub(crate) fn emit_friendship(
    cfg: &Cfg,
    target: &Ident,
    include_constructor: bool,
    constructor: Option<TokenStream>,
    module: Option<&constructor::ModuleCfg>,
    in_self_scope: bool,
) -> TokenStream {
    let relation = cfg.relation.as_ref().expect("friendship relation is empty");

    let in_module = module.is_some();
    let seal = cfg.seal.clone().unwrap_or_else(|| format_ident!("Seal"));
    let capabilities = cfg.capabilities(include_constructor);

    let relation_value = match relation {
        Type::Path(path) if path.qself.is_none() => {
            parent_path(&path.path, in_module)
        }
        _ => quote! { #relation },
    };
    let value = match &cfg.value {
        Some(value) => quote! { #value },
        None => relation_value,
    };

    let target = match in_module {
        true => quote! { super::#target },
        false => quote! { #target },
    };

    let trait_visibility = match in_module {
        true => quote! { pub(super) },
        false => TokenStream::new(),
    };

    let conversion = cfg.conversion.clone().unwrap_or_else(|| {
        let name = match relation {
            Type::Path(path) if path.qself.is_none() => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string()),
            _ => None,
        };

        name.map(|name| format_ident!("to_{}", runner::snake_case_of(&name)))
            .unwrap_or_else(|| format_ident!("convert"))
    });

    let friend_impls = cfg.friends.iter().map(|friend| {
        emit_friend(friend, &seal, &conversion, &value, &target, in_module)
    });

    let items = quote! {
        #trait_visibility trait #seal {
            fn #conversion(self) -> #value;
        }

        #(
            #[allow(unused, dead_code)]
            #trait_visibility trait #capabilities: #seal {}
        )*

        #(#friend_impls)*

        #constructor
    };

    return match module {
        None if in_self_scope => quote! {
            #items
        },
        None => quote! {
            const _: () = {
                #items
            };
        },
        Some(module) => {
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

fn emit_friend(
    cfg: &Friend,
    seal: &Ident,
    conversion: &Ident,
    value: &TokenStream,
    target: &TokenStream,
    in_module: bool,
) -> TokenStream {
    let ty = match cfg.ty.is_ident("Self") {
        true => target.clone(),
        false => parent_path(&cfg.ty, in_module),
    };

    let conv = parent_path(
        cfg.conv.as_ref().expect("friend conversion missing"),
        in_module,
    );

    let capabilities = &cfg.capabilities;

    return quote! {
        impl #seal for #ty {
            #[inline(always)]
            fn #conversion(self) -> #value {
                return #conv(self);
            }
        }

        #(impl #capabilities for #ty {})*
    };
}

fn parent_path(
    path: &syn::Path,
    in_module: bool,
) -> TokenStream {
    let is_explicit = path.leading_colon.is_some()
        || path.segments.first().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                "crate" | "self" | "super"
            )
        });

    return match in_module && !is_explicit {
        true => quote! { super::#path },
        false => quote! { #path },
    };
}
