use crate::friendship::cfg::{
    Cfg,
    Friend,
    MakeCfg,
};
use crate::runner;
use crate::runner::MkErr;
use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::Type;
use syn::spanned::Spanned;

pub(crate) mod cfg {
    use crate::runner;
    use crate::runner::MkErr;
    use proc_macro2::Ident;
    use quote::{
        ToTokens,
        format_ident,
    };
    use std::{
        cmp::Ordering,
        collections::BTreeSet,
    };
    use syn::{
        Path,
        Type,
        parse::{
            Parse,
            ParseStream,
        },
    };

    #[derive(Clone)]
    pub(crate) struct Friend {
        pub(crate) identity: String,
        pub(crate) ty: Option<Path>,
        pub(crate) conv: Option<Path>,
        pub(crate) capabilities: BTreeSet<Ident>,
    }

    impl Parse for Friend {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let ty = if input.peek(syn::Token![_]) {
                let _: syn::Token![_] = input.parse()?;
                None
            }
            else {
                Some(input.parse::<Path>()?)
            };

            let mut conv = None;
            let mut capabilities = None;
            runner::parse_optional_attributes(input, |name, stream| {
                match name {
                    "conv" => conv = Some(stream.parse()?),
                    "cap" => {
                        capabilities = Some(
                            runner::list(stream)?.collect::<BTreeSet<_>>(),
                        );
                    }
                    _ => return Ok(false),
                };
                return Ok(true);
            })?;

            return Ok(Self {
                identity: ty
                    .as_ref()
                    .map(ToTokens::to_token_stream)
                    .map(|tokens| tokens.to_string())
                    .unwrap_or_else(|| "_".to_string()),
                ty,
                conv,
                capabilities: capabilities.unwrap_or_else(|| BTreeSet::new()),
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
                ty: Some(it.clone()),
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

    #[derive(Clone)]
    pub(crate) struct MakeCfg {
        pub(super) of_relation: Path,
        pub(super) of_friend: Ident,
        pub(super) friends: BTreeSet<MakeFriend>,
    }

    #[derive(Clone)]
    pub(crate) struct MakeFriend {
        identity: String,
        pub(super) ty: Path,
    }

    impl MakeFriend {
        pub(super) fn resolved_identity(
            &self,
            target: &Ident,
        ) -> String {
            return match self.ty.is_ident("Self") {
                true => target.to_string(),
                false => self.identity.clone(),
            };
        }
    }

    impl Parse for MakeFriend {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let ty: Path = input.parse()?;

            return Ok(Self {
                identity: ty.to_token_stream().to_string(),
                ty,
            });
        }
    }

    impl Eq for MakeFriend {}

    impl PartialOrd for MakeFriend {
        fn partial_cmp(
            &self,
            other: &Self,
        ) -> Option<Ordering> {
            return Some(self.cmp(other));
        }
    }

    impl PartialEq for MakeFriend {
        fn eq(
            &self,
            other: &Self,
        ) -> bool {
            return self.identity == other.identity;
        }
    }

    impl Ord for MakeFriend {
        fn cmp(
            &self,
            other: &Self,
        ) -> Ordering {
            return self.identity.cmp(&other.identity);
        }
    }

    impl Parse for MakeCfg {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let mut of_relation = None;
            let mut of_friend = format_ident!("of");
            let mut friends = BTreeSet::new();

            runner::parse_inner_attributes(input, |attr, stream| {
                match attr {
                    "of_relation" => of_relation = Some(stream.parse()?),
                    "of_friend" => of_friend = stream.parse()?,
                    "friends" => friends = runner::list(stream)?.collect(),
                    _ => return Ok(false),
                };

                return Ok(true);
            })?;

            let of_relation = of_relation.ok_or_else(|| {
                syn::Error::new(
                    input.span(),
                    "missing required `of_relation` argument",
                )
            })?;

            return Ok(Self {
                of_relation,
                of_friend,
                friends,
            });
        }
    }

    // =========================================================================

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

    // =========================================================================

    #[derive(Default)]
    pub(crate) struct Cfg {
        pub(super) relation: Option<Type>,
        pub(super) value: Option<Type>,
        pub(super) seal: Option<Ident>,
        pub(super) conversion: Option<Ident>,
        pub(super) module: Option<ModuleCfg>,
        pub(super) friends: BTreeSet<Friend>,
    }

    impl Cfg {
        pub(super) fn capabilities(
            &self,
            include_constructor: bool,
        ) -> BTreeSet<Ident> {
            let mut capabilities = self
                .friends
                .iter()
                .flat_map(|friend| friend.capabilities.iter().cloned())
                .collect::<BTreeSet<_>>();

            if include_constructor {
                capabilities.insert(format_ident!("Make"));
            }

            return capabilities;
        }
    }

    impl Parse for Cfg {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let mut this = Self::default();

            runner::parse_inner_attributes(input, |attr, stream| {
                match attr {
                    "relation" => this.relation = Some(stream.parse()?),
                    "value" => this.value = Some(stream.parse()?),
                    "seal" => this.seal = Some(stream.parse()?),
                    "conversion" => this.conversion = Some(stream.parse()?),
                    "mod" => {
                        if stream.peek(syn::Token![_]) {
                            let _: syn::Token![_] = stream.parse()?;
                        }
                        else {
                            this.module = Some(stream.parse()?);
                        }
                    }
                    "friends" => {
                        for friend in runner::list(stream)? {
                            if !this.friends.insert(friend) {
                                return stream.span().fail("duplicated friend");
                            }
                        }
                    }
                    _ => return Ok(false),
                };

                return Ok(true);
            })?;
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

pub(crate) struct Protocol {
    pub(crate) target: Ident,
    pub(crate) relation: Type,
    pub(crate) seal: Ident,
    pub(crate) conversion: Ident,
    pub(crate) capabilities: Vec<Ident>,
    pub(crate) friends: Vec<ProtocolFriend>,
    pub(crate) emit_seal: bool,
    pub(crate) target_conversion: Option<TokenStream>,
    pub(crate) konst: bool,
}

pub(crate) fn emit_protocol(protocol: Protocol) -> TokenStream {
    let target = &protocol.target;
    let relation = &protocol.relation;
    let seal = &protocol.seal;
    let conversion = &protocol.conversion;
    let capabilities = &protocol.capabilities;
    let konst = runner::konst(protocol.konst);
    let bonst = runner::bonst(protocol.konst);
    let destruct = runner::destruct(protocol.konst);

    let seal_declaration = protocol.emit_seal.then(|| {
        quote! {
            #konst trait #seal {
                fn #conversion(&self) -> #relation;
            }
        }
    });

    let target_seal = protocol.target_conversion.map(|body| {
        quote! {
            #konst impl #seal for #target {
                #[inline(always)]
                fn #conversion(&self) -> #relation {
                    #body
                }
            }
        }
    });

    let friend_impls = protocol.friends.iter().map(|friend| {
        let ty = &friend.ty;
        let capabilities = &friend.capabilities;
        let seal_impl = friend.conversion.as_ref().map(|body| {
            quote! {
                #konst impl #seal for #ty {
                    #[inline(always)]
                    fn #conversion(&self) -> #relation {
                        #body
                    }
                }
            }
        });

        quote! {
            #seal_impl
            #(#konst impl #capabilities for #ty {})*
        }
    });

    return quote! {
        #seal_declaration

        #(
            #konst trait #capabilities: #bonst #seal {}
        )*

        #target_seal
        #(#friend_impls)*

        #konst impl<T> #seal for &T
        where
            T: #bonst #seal #destruct,
        {
            #[inline(always)]
            fn #conversion(&self) -> #relation {
                return #seal::#conversion(&**self);
            }
        }

        #konst impl<T> #seal for &mut T
        where
            T: #bonst #seal #destruct,
        {
            #[inline(always)]
            fn #conversion(&self) -> #relation {
                return #seal::#conversion(&**self);
            }
        }

        #(
            #konst impl<T> #capabilities for &T
            where
                T: #bonst #capabilities #destruct,
            {}

            #konst impl<T> #capabilities for &mut T
            where
                T: #bonst #capabilities #destruct,
            {}
        )*
    };
}

pub(crate) fn friendship(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let friendship = syn::parse_macro_input!(attr as Cfg);
    let mut item = syn::parse_macro_input!(item as syn::Item);

    return runner::catching(move || {
        let (_, _, attr) = runner::get_concrete_type(&mut item)?;

        let constructor = runner::pop_attr(attr, "constructor")?
            .map(|attr| attr.parse_args::<MakeCfg>())
            .transpose()?;

        return expand(item, friendship, constructor);
    });
}

pub(crate) fn constructor(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let constructor = syn::parse_macro_input!(attr as MakeCfg);
    let mut item = syn::parse_macro_input!(item as syn::Item);

    return runner::catching(move || {
        let (_, _, attr) = runner::get_concrete_type(&mut item)?;

        let friendship =
            match runner::pop_attr(attr, "friendship")? {
                None => return item.span().fail(
                    "missing required `#[typekin::friendship(...)]` attribute",
                ),
                Some(it) => it,
            }
            .parse_args::<Cfg>()?;

        return expand(item, friendship, Some(constructor));
    });
}

fn expand(
    mut item: syn::Item,
    mut friendship: Cfg,
    mut constructor: Option<MakeCfg>,
) -> syn::Result<TokenStream> {
    let (target, visibility, _) = runner::get_concrete_type(&mut item)?;

    if friendship.relation.is_none() {
        return item.fail("missing required argument: `relation`");
    }

    if let Some(constructor) = constructor.as_mut() {
        grant_constructor(&mut friendship, constructor, target)?;
    }

    if friendship.capabilities(constructor.is_some()).is_empty() {
        return item.fail("friendship requires at least one friend capability");
    }

    for friend in &friendship.friends {
        if friend.ty.is_some() && friend.conv.is_none() {
            return item.fail("friendship friend requires `conv = ...`");
        }
    }

    let protocol =
        emit_friendship(&friendship, target, visibility, constructor.as_ref());

    return Ok(quote! {
        #item

        #protocol
    });
}

fn grant_constructor(
    cfg: &mut Cfg,
    constructor: &MakeCfg,
    target: &Ident,
) -> syn::Result<()> {
    for it in &constructor.friends {
        let identity = it.resolved_identity(target);
        let friend = match cfg
            .friends
            .iter()
            .find(|friend| {
                if friend.identity == identity {
                    return true;
                }

                return it.ty.is_ident("Self")
                    && friend
                        .ty
                        .as_ref()
                        .is_some_and(|ty| ty.is_ident("Self"));
            })
            .cloned()
        {
            None => {
                return target
                    .fail("constructor friend is not declared by friendship");
            }
            Some(friend) => friend,
        };

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
    visibility: &syn::Visibility,
    constructor: Option<&MakeCfg>,
) -> TokenStream {
    let relation = cfg.relation.as_ref().expect("friendship relation is empty");

    let in_module = cfg.module.is_some();
    let constructor_capability = format_ident!("Make");
    let seal = cfg.seal.clone().unwrap_or_else(|| format_ident!("Seal"));
    let capabilities = cfg.capabilities(constructor.is_some());

    let relation_value = match relation {
        Type::Path(path) if path.qself.is_none() => {
            parent_path(&path.path, in_module)
        }
        _ => quote! { #relation },
    };

    let value = match &cfg.value {
        Some(value) => quote! { #value },
        None => relation_value.clone(),
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

    let friend_impls =
        cfg.friends
            .iter()
            .filter(|it| it.ty.is_some())
            .map(|friend| {
                emit_friend(
                    friend,
                    &seal,
                    &conversion,
                    &value,
                    &target,
                    in_module,
                )
            });

    // =========================================================================

    let constructor = constructor.map(|constructor| {
        let of_relation = &constructor.of_relation;
        let of_friend = &constructor.of_friend;

        return quote! {
            impl #target {
                #[allow(private_bounds)]
                #[inline(always)]
                #visibility fn #of_friend<T>(it: T) -> Self
                where
                    T: #constructor_capability + #seal,
                {
                    return #of_relation(#seal::#conversion(&it));
                }
            }

            impl #seal for #relation_value
            where
                #relation_value: ::core::marker::Copy,
            {
                #[inline(always)]
                fn #conversion(&self) -> #value {
                    return *self;
                }
            }

            impl #constructor_capability for #relation_value
            where
                #relation_value: ::core::marker::Copy,
            {}
        };
    });

    let items = quote! {
        #trait_visibility trait #seal {
            fn #conversion(&self) -> #value;
        }

        #(
            #[allow(unused, dead_code)]
            #trait_visibility trait #capabilities: #seal {}
        )*

        #(#friend_impls)*

        impl<T> #seal for &T
        where
            T: #seal,
        {
            #[inline(always)]
            fn #conversion(&self) -> #value {
                return #seal::#conversion(&**self);
            }
        }

        impl<T> #seal for &mut T
        where
            T: #seal,
        {
            #[inline(always)]
            fn #conversion(&self) -> #value {
                return #seal::#conversion(&**self);
            }
        }

        #(impl<T> #capabilities for &T
        where
            T: #capabilities,
        {})*

        #(impl<T> #capabilities for &mut T
        where
            T: #capabilities,
        {})*

        #constructor
    };

    return match &cfg.module {
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
    let path = cfg.ty.as_ref().unwrap();

    let ty = match path.is_ident("Self") {
        true => target.clone(),
        false => parent_path(path, in_module),
    };

    let conv = parent_path(
        cfg.conv.as_ref().expect("friend conversion missing"),
        in_module,
    );

    let capabilities = &cfg.capabilities;

    return quote! {
        impl #seal for #ty {
            #[inline(always)]
            fn #conversion(&self) -> #value {
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
