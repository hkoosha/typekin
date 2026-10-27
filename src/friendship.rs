use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    ToTokens,
    format_ident,
    quote,
};
use syn::{
    Item,
    Type,
};

pub(crate) use self::cfg::Cfg;
use crate::friendship::cfg::Friend;
use crate::zz_lib::Konst;
use crate::zz_runner::MkErr;
use crate::zz_value_type::N;
use crate::{
    constructor,
    zz_lib as zz,
    zz_runner,
};

pub(crate) mod cfg {
    use std::cmp::Ordering;
    use std::collections::{
        BTreeMap,
        BTreeSet,
        HashMap,
    };
    use std::fmt::{
        Debug,
        Formatter,
    };

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
        bracketed,
    };

    use crate::zz_lib::{
        arg_or_list,
        one_or_list,
    };
    use crate::zz_runner::{
        MkErr,
        ToCollection,
    };
    use crate::{
        constructor,
        zz_lib as zz,
    };

    #[derive(Clone)]
    pub(crate) struct Friend {
        pub(crate) identity: String,
        pub(crate) ty: Path,
        pub(crate) capabilities: BTreeSet<Ident>,
        pub(crate) conv: Option<Path>,
        pub(crate) make_conv: Option<Path>,
        pub(crate) capability_conversions: BTreeMap<Ident, Path>,
    }

    impl Parse for Friend {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            return Self::parse_with_defaults(input, &BTreeSet::new());
        }
    }

    pub(crate) fn trust_capability() -> Ident {
        return format_ident!("Unsafe");
    }

    pub(crate) fn is_trust_capability(capability: &Ident) -> bool {
        return capability == &trust_capability()
            || capability == &format_ident!("FlagUnsafe");
    }

    struct Capability(Ident);

    impl Parse for Capability {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            if input.peek(Token![unsafe]) {
                input.parse::<Token![unsafe]>()?;
                return Ok(Self(trust_capability()));
            }

            return input.parse::<Ident>().map(Self);
        }
    }

    fn parse_capabilities(input: ParseStream) -> syn::Result<BTreeSet<Ident>> {
        let capabilities = one_or_list::<Capability>(input)?
            .map(|Capability(capability)| capability)
            .set();
        return Ok(capabilities);
    }

    impl Friend {
        pub(crate) fn parse_with_defaults(
            input: ParseStream,
            capabilities: &BTreeSet<Ident>,
        ) -> syn::Result<Self> {
            let implicit = input.peek(Token![_]);
            let conv = if implicit {
                input.parse::<Token![_]>()?;
                None
            }
            else {
                let conv = input.parse::<Path>()?;
                if conv.is_ident("Self") {
                    return conv.fail(
                        "friend implicit conversion is `_(Type)`, not `Self(Type)`",
                    );
                }
                Some(conv)
            };

            let here = input.span();
            let mut ty = if implicit {
                zz::tuple::<Path>(input)?.vec()
            }
            else {
                arg_or_list::<Path>(input)?
            };
            let ty =
                match (ty.pop(), ty) {
                    (Some(it), rest) if rest.is_empty() => it,
                    _ => return here.fail(
                        "friend conversion requires exactly one source type",
                    ),
                };

            let mut capabilities = if input.peek(Token![->]) {
                let _ = input.parse::<Token![->]>()?;
                parse_capabilities(input)?
            }
            else {
                capabilities.clone()
            };
            if capabilities.contains(&trust_capability()) {
                capabilities.insert(format_ident!("Make"));
            }

            let capability_conversions = capabilities
                .iter()
                .cloned()
                .filter_map(|capability| {
                    conv.clone().map(|conv| (capability, conv))
                })
                .collect();

            return Ok(Self {
                identity: ty.to_token_stream().to_string(),
                conv: conv.clone().filter(|conv| !conv.is_ident("self")),
                make_conv: capabilities
                    .contains(&format_ident!("Make"))
                    .then(|| conv.clone())
                    .flatten(),
                capability_conversions,
                capabilities,
                ty,
            });
        }

        pub(crate) fn parse_with_valid(
            input: ParseStream,
            valid: &[Ident],
            defaults: &BTreeSet<Ident>,
        ) -> syn::Result<BTreeSet<Friend>> {
            let mut friends = BTreeSet::new();
            if input.peek(syn::token::Bracket) {
                let content;
                let _ = bracketed!(content in input);
                while !content.is_empty() {
                    let friend = Self::parse_with_defaults(&content, defaults)?;
                    friends.insert(friend);

                    if !content.is_empty() {
                        let _ = content.parse::<Token![,]>()?;
                    }
                }
            }
            else {
                friends.insert(Self::parse_with_defaults(input, defaults)?);
            }

            for friend in &friends {
                for capability in &friend.capabilities {
                    if !valid.contains(capability) {
                        return capability.fail("unknown capability");
                    }
                }
            }

            return Ok(friends);
        }

        pub(crate) fn parse_with_expansion(
            input: ParseStream,
            valid: &[Ident],
            defaults: &BTreeSet<Ident>,
            expansions: &HashMap<Ident, Vec<Ident>>,
        ) -> syn::Result<BTreeSet<Friend>> {
            let this = Self::parse_with_valid(input, valid, defaults)?
                .into_iter()
                .map(|mut friend| {
                    for (cap, exp) in expansions {
                        if friend.capabilities.contains(cap) {
                            friend.capabilities.extend(exp.clone());
                        }
                    }
                    return friend;
                })
                .collect();

            return Ok(this);
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
                make_conv: None,
                capability_conversions: BTreeMap::new(),
                capabilities: BTreeSet::new(),
            };
        }
    }

    impl ::std::fmt::Debug for Friend {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            return f
                .debug_struct("Friend")
                .field("identity", &self.identity)
                .field("capabilities", &self.capabilities)
                .finish();
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

    impl Debug for Cfg {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            return f.debug_struct("Cfg").finish_non_exhaustive();
        }
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
            let mut capabilities = self.extra_caps.clone();
            capabilities.extend(
                self.friends
                    .iter()
                    .flat_map(|friend| friend.capabilities.iter().cloned()),
            );

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
                    "friend" => {
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
                                if friend.make_conv.is_some() {
                                    existing.make_conv = friend.make_conv;
                                }
                                existing
                                    .capability_conversions
                                    .extend(friend.capability_conversions);
                                existing.conv = friend.conv;
                                this.friends.insert(existing);
                            }
                            else {
                                this.friends.insert(friend);
                            }
                        }
                    }
                    "extra_caps" => {
                        this.extra_caps = parse_capabilities(stream)?
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
    pub(crate) ty: Type,
    pub(crate) capabilities: std::collections::BTreeSet<Ident>,
    pub(crate) conversion: Option<TokenStream>,
}

pub(crate) struct Construction {
    pub(crate) make: Ident,
    pub(crate) trusted: Ident,
    pub(crate) trusted_constructor: TokenStream,
    pub(crate) checked: TokenStream,
}

pub(crate) struct Protocol {
    pub(crate) target: TokenStream,
    pub(crate) generics: syn::Generics,
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
    let konst = Konst::konst(protocol.konst);
    let bonst = Konst::bonst(protocol.konst);
    let (parameters, arguments, where_clause) =
        protocol.generics.split_for_impl();

    let seal_declaration = protocol.emit_seal.then(|| {
        quote! {
            #konst trait #seal #parameters #where_clause {
                fn #conversion(self) -> #relation;
            }
        }
    });

    let capability_declarations =
        protocol.capabilities.iter().map(|capability| {
            let is_unsafe = cfg::is_trust_capability(capability);
            let unsafe_ = is_unsafe.then(|| quote! { unsafe });
            let safety = is_unsafe.then(|| {
                quote! {
                    #[doc = "# Safety"]
                    #[doc = ""]
                    #[doc = "Implementing this trait asserts that values \
                        produced by the sealed conversion satisfy every \
                        invariant required by the target type."]
                }
            });
            let method =
                protocol.construction.as_ref().and_then(|construction| {
                    (capability == &construction.make).then(|| {
                        quote! { fn make(self) -> #target; }
                    })
                });

            quote! {
                #safety
                #konst #unsafe_ trait #capability #parameters:
                    #bonst #seal #arguments
                #where_clause {
                    #method
                }
            }
        });

    let target_seal = protocol.target_conversion.as_ref().map(|body| {
        quote! {
            #konst impl #parameters #seal #arguments for #target
            #where_clause {
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
                #konst impl #parameters #seal #arguments for #ty
                #where_clause {
                    #[inline(always)]
                    fn #conversion(self) -> #relation {
                        #body
                    }
                }
            }
        });
        let capability_impls = friend.capabilities.iter().map(|capability| {
            let unsafe_ =
                cfg::is_trust_capability(capability).then(|| quote! { unsafe });
            let method =
                protocol.construction.as_ref().and_then(|construction| {
                    if capability != &construction.make {
                        return None;
                    }

                    let constructor = match friend
                        .capabilities
                        .contains(&construction.trusted)
                    {
                        true => construction.trusted_constructor.clone(),
                        false => {
                            let checked = &construction.checked;
                            quote! { #checked(raw) }
                        }
                    };

                    Some(quote! {
                        #[inline(always)]
                        fn make(self) -> #target {
                            let raw = <Self as #seal #arguments>::#conversion(self);
                            return #constructor;
                        }
                    })
                });

            quote! {
                #konst #unsafe_ impl #parameters #capability #arguments for #ty
                #where_clause {
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
    return zz_runner::catching(move || {
        let constructor = attr.constructor();
        let scope = constructor.scope.clone();

        return expand(item, attr, Some(constructor), Some(scope));
    })
    .into();
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
                format_ident!("to_{}", zz_runner::snake_case_of(&name))
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

        name.map(|name| format_ident!("to_{}", zz_runner::snake_case_of(&name)))
            .unwrap_or_else(|| format_ident!("convert"))
    });

    let friend_impls = cfg.friends.iter().map(|friend| {
        emit_friend(friend, &seal, &conversion, &value, &target, in_module)
    });

    let capability_declarations = capabilities.iter().map(|capability| {
        let unsafe_ =
            cfg::is_trust_capability(capability).then(|| quote! { unsafe });

        return quote! {
            #[allow(unused, dead_code)]
            #trait_visibility #unsafe_ trait #capability: #seal {
                fn #conversion(self) -> #value;
            }
        };
    });

    let items = quote! {
        #trait_visibility trait #seal {
        }

        #(#capability_declarations)*

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

    let capabilities = &cfg.capabilities;
    let capability_impls = capabilities.iter().map(|capability| {
        let unsafe_ =
            cfg::is_trust_capability(capability).then(|| quote! { unsafe });
        let callback = cfg
            .capability_conversions
            .get(capability)
            .or_else(|| {
                (capability == &format_ident!("Make"))
                    .then_some(&cfg.make_conv)
                    .and_then(|conversion| conversion.as_ref())
            })
            .or(cfg.conv.as_ref())
            .expect("friend conversion missing");
        let callback = parent_path(callback, in_module);

        return quote! {
            #unsafe_ impl #capability for #ty {
                #[inline(always)]
                fn #conversion(self) -> #value {
                    return #callback(self);
                }
            }
        };
    });

    return quote! {
        impl #seal for #ty {}
        #(#capability_impls)*
    };
}

// TODO hm... very hacky
fn parent_path(
    path: &syn::Path,
    in_module: bool,
) -> TokenStream {
    let repr = path.to_token_stream().to_string();
    if repr == "String" || N::of(repr).is_some() {
        return quote! { #path };
    }

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
