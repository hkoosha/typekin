use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::fmt::{
    Debug,
    Formatter,
};

use proc_macro2::{
    Ident,
    TokenStream,
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
    Item,
    Path,
    Token,
    Type,
};

use crate::zz_friend::{
    Capability,
    Friend,
    parse_capabilities,
    upsert_friend,
};
use crate::zz_lib::{
    Key,
    PathIdentity,
    one_or_list,
};
use crate::zz_names::{
    NameKey,
    Names,
    ResolvedNames,
};
use crate::zz_runner::{
    MkErr,
    ToCollection,
};
use crate::zz_values::Num;
use crate::{
    constructor,
    zz_lib as zz,
    zz_runner,
};

/// A name the `friends` macro generates.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum FriendsName {
    /// `Seal` unless `seal` names one.
    Seal,
    /// `to_{snake}` of the relation's last path segment, or `convert`,
    /// unless `conversion` names one.
    Conversion,
}

impl NameKey for FriendsName {
    /// The relation's last path segment.
    type Owner = Option<Ident>;

    fn default_name(
        self,
        relation: &Option<Ident>,
    ) -> String {
        return match self {
            Self::Seal => "Seal".to_string(),
            Self::Conversion => relation.as_ref().map_or_else(
                || "convert".to_string(),
                |relation| {
                    format!(
                        "to_{}",
                        zz_runner::snake_case_of(&relation.to_string())
                    )
                },
            ),
        };
    }
}

pub(crate) struct Cfg {
    relation: Option<Type>,
    value: Option<Type>,
    /// `seal` and `conversion`.
    names: Names<FriendsName>,
    module: Option<constructor::ModuleCfg>,
    scope: Option<constructor::Scope>,
    friends: BTreeSet<Friend>,
    /// Per friend, the conversion each capability was declared with:
    /// merged declarations (`[a(T) -> Make, b(T) -> Math]`) keep their own.
    conversions: BTreeMap<PathIdentity, BTreeMap<Ident, Path>>,
    extra_caps: BTreeSet<Ident>,
}

/// A `friend` entry and the conversion it names (`self` included) for
/// each capability it grants.
struct Declared {
    friend: Friend,
    conversions: BTreeMap<Ident, Path>,
}

impl Parse for Declared {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let conversion = match input.peek(Token![_]) {
            true => None,
            false => input.fork().parse::<Path>().ok(),
        };
        let friend = input.parse::<Friend>()?;
        let conversions = conversion
            .map(|conversion| {
                return friend
                    .capabilities
                    .iter()
                    .map(|capability| (capability.clone(), conversion.clone()))
                    .collect();
            })
            .unwrap_or_default();

        return Ok(Self {
            friend,
            conversions,
        });
    }
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
    /// The generated names; the defaults derive from the relation's last
    /// path segment.
    fn resolved_names(&self) -> ResolvedNames<FriendsName> {
        let relation = match &self.relation {
            Some(Type::Path(path)) if path.qself.is_none() => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.clone()),
            _ => None,
        };
        return self.names.clone().resolve(relation, BTreeSet::new());
    }

    fn constructor(&self) -> constructor::Cfg {
        let friends = self
            .friends
            .iter()
            .map(|friend| {
                return constructor::Friend {
                    ty: friend.ty.clone(),
                };
            })
            .set();

        return constructor::Cfg {
            relationship: syn::parse_quote! { Self::of_parts },
            names: Names::default(),
            friends,
            scope: self.scope.clone().unwrap_or(constructor::Scope::Const),
        };
    }

    /// Every capability trait the protocol declares; the constructor
    /// always adds `Make`.
    fn capabilities(&self) -> BTreeSet<Ident> {
        let mut capabilities = self.extra_caps.clone();
        capabilities.extend(
            self.friends
                .iter()
                .flat_map(|friend| friend.capabilities.iter().cloned()),
        );
        capabilities.insert(Capability::Make.ident());

        return capabilities;
    }

    /// Adds a `friend` entry; a repeated source type merges into its first
    /// entry when their capabilities are disjoint.
    fn declare(
        &mut self,
        stream: ParseStream,
        declared: Declared,
    ) -> syn::Result<()> {
        let Declared {
            friend,
            conversions,
        } = declared;
        let friend = match self.friends.take(&friend) {
            Some(mut existing) => {
                if !existing.capabilities.is_disjoint(&friend.capabilities) {
                    return stream
                        .span()
                        .fail("friend capabilities overlap for a source type");
                }

                existing.capabilities.extend(friend.capabilities);
                existing.conv = friend.conv;
                existing
            }
            None => friend,
        };

        self.conversions
            .entry(friend.ty.clone())
            .or_default()
            .extend(conversions);
        self.friends.insert(friend);
        return Ok(());
    }

    /// The conversion `friend` calls for `capability`.
    fn conversion<'a>(
        &'a self,
        friend: &'a Friend,
        capability: &Ident,
    ) -> &'a Path {
        return self
            .conversions
            .get(&friend.ty)
            .and_then(|conversions| conversions.get(capability))
            .or(friend.conv.as_ref())
            .expect("friend conversion missing");
    }
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self {
            relation: None,
            value: None,
            names: Names::default(),
            module: None,
            scope: None,
            friends: BTreeSet::new(),
            conversions: BTreeMap::new(),
            extra_caps: BTreeSet::new(),
        };

        zz::parse_inner(input, zz::Site::Root, |attr, stream| {
            match attr {
                Key::Relation => this.relation = Some(stream.parse()?),
                Key::Value => this.value = Some(stream.parse()?),
                Key::Seal => {
                    this.names.choose(FriendsName::Seal, stream.parse()?);
                }
                Key::Conversion => {
                    this.names.choose(FriendsName::Conversion, stream.parse()?);
                }
                Key::Mod => {
                    if stream.peek(Token![_]) {
                        let _: Token![_] = stream.parse()?;
                    }
                    else {
                        this.module = Some(stream.parse()?);
                    }
                }
                Key::Scope => this.scope = Some(stream.parse()?),
                Key::Friend => {
                    for declared in one_or_list(stream)? {
                        this.declare(stream, declared)?;
                    }
                }
                Key::ExtraCaps => this.extra_caps = parse_capabilities(stream)?,
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

pub(crate) fn ekran(
    attr: Cfg,
    item: Item,
) -> syn::Result<TokenStream> {
    let constructor = attr.constructor();

    return expand(item, attr, constructor);
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

    return expand(item, friendship, constructor);
}

/// `Module(friendship.module)`, else the constructor's scope.
fn resolve_scope(
    cfg: &Cfg,
    scope: &constructor::Scope,
) -> constructor::Scope {
    return match &cfg.module {
        Some(module) => constructor::Scope::Module(module.clone()),
        None => scope.clone(),
    };
}

/// The relation, value and target as the protocol's items name them.
struct FriendshipTypes {
    relation: TokenStream,
    value: TokenStream,
    target: TokenStream,
    /// The items sit in a module: outer paths take `super::`.
    in_module: bool,
}

impl FriendshipTypes {
    fn new(
        cfg: &Cfg,
        target: &Ident,
        in_module: bool,
    ) -> Self {
        let relation =
            cfg.relation.as_ref().expect("friendship relation is empty");
        let relation = match relation {
            Type::Path(path) if path.qself.is_none() => {
                outer_path(&path.path, in_module)
            }
            _ => quote! { #relation },
        };
        let value = cfg
            .value
            .as_ref()
            .map_or_else(|| relation.clone(), |value| quote! { #value });
        let target = match in_module {
            true => quote! { super::#target },
            false => quote! { #target },
        };

        return Self {
            relation,
            value,
            target,
            in_module,
        };
    }
}

fn expand(
    mut item: Item,
    mut friendship: Cfg,
    constructor: constructor::Cfg,
) -> syn::Result<TokenStream> {
    let (target, visibility, _) = zz::get_concrete_type(&mut item)?;
    let target = target.clone();
    let visibility = visibility.clone();
    let scope = resolve_scope(&friendship, &constructor.scope);

    if friendship.relation.is_none() {
        return item.fail("missing required argument: `relation`");
    }

    grant_constructor(&mut friendship, &constructor, &target)?;

    for friend in &friendship.friends {
        if friend.conv.is_none() {
            return item.fail(
                "concrete friendship requires `conversion(Type) -> Capabilities`",
            );
        }
    }

    let names = friendship.resolved_names();
    let types =
        FriendshipTypes::new(&friendship, &target, scope.module().is_some());
    let constructor = constructor::expand(
        &constructor.in_self_scope(),
        constructor::Protocol {
            target: types.target.clone(),
            visibility,
            relation: types.relation.clone(),
            value: types.value.clone(),
            seal: names.get(FriendsName::Seal),
            conversion: names.get(FriendsName::Conversion),
        },
    );
    let protocol =
        emit_friendship(&friendship, &names, &types, constructor, &scope);

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
        let is_self = requested.ty.path().is_ident("Self");
        let target_identity = is_self.then(|| target.to_string());
        let identity = target_identity
            .as_deref()
            .unwrap_or(requested.ty.identity());
        let friend = cfg
            .friends
            .iter()
            .find(|friend| {
                return friend.ty.identity() == identity
                    || is_self && friend.ty.path().is_ident("Self");
            })
            .cloned()
            .ok_or_else(|| {
                syn::Error::new(
                    target.span(),
                    "constructor friend is not declared by friendship",
                )
            })?;

        upsert_friend(&mut cfg.friends, friend, |friend| {
            friend.capabilities.insert(Capability::Make.ident());
        });
    }

    return Ok(());
}

fn emit_friendship(
    cfg: &Cfg,
    names: &ResolvedNames<FriendsName>,
    types: &FriendshipTypes,
    constructor: TokenStream,
    scope: &constructor::Scope,
) -> TokenStream {
    let seal = names.get(FriendsName::Seal);
    let conversion = names.get(FriendsName::Conversion);
    let value = &types.value;
    let trait_visibility = match types.in_module {
        true => quote! { pub(super) },
        false => TokenStream::new(),
    };

    let friend_impls = cfg
        .friends
        .iter()
        .map(|friend| emit_friend(cfg, friend, &seal, &conversion, types));

    let capability_declarations =
        cfg.capabilities().into_iter().map(|capability| {
            let unsafe_ = (Capability::of(&capability)
                == Some(Capability::Unsafe))
            .then(|| quote! { unsafe });

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

    return scope.wrap(items);
}

fn emit_friend(
    cfg: &Cfg,
    friend: &Friend,
    seal: &Ident,
    conversion: &Ident,
    types: &FriendshipTypes,
) -> TokenStream {
    let ty = match friend.ty.path().is_ident("Self") {
        true => types.target.clone(),
        false => outer_path(friend.ty.path(), types.in_module),
    };
    let value = &types.value;

    let capability_impls = friend.capabilities.iter().map(|capability| {
        let unsafe_ = (Capability::of(capability) == Some(Capability::Unsafe))
            .then(|| quote! { unsafe });
        let callback =
            outer_path(cfg.conversion(friend, capability), types.in_module);

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
/// `String` or a primitive number type, which never takes `super::`.
fn is_primitive_or_string(path: &Path) -> bool {
    let repr = path.to_token_stream().to_string();
    return repr == "String" || Num::of(repr).is_some();
}

/// `path` as seen from the protocol's items: `super::path` inside a module
/// unless it is explicit (`::`, `crate`, `self`, `super`).
fn outer_path(
    path: &Path,
    in_module: bool,
) -> TokenStream {
    if is_primitive_or_string(path) {
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
