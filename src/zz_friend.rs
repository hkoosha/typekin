use std::cmp::Ordering;
use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::fmt::Formatter;

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    ToTokens,
    format_ident,
    quote,
};
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::{
    ImplGenerics,
    Path,
    Token,
    Type,
    TypeGenerics,
    WhereClause,
    bracketed,
};

use crate::zz_lib as zz;
use crate::zz_lib::{
    Konst,
    PathIdentity,
    arg_or_list,
    one_or_list,
};
use crate::zz_runner::{
    MkErr,
    ToCollection,
};
use crate::zz_values::Operator;

#[derive(Clone)]
pub(crate) struct Friend {
    pub(crate) ty: PathIdentity,
    pub(crate) capabilities: BTreeSet<Ident>,
    pub(crate) conv: Option<Path>,
}

impl Parse for Friend {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return Self::parse_with_defaults(input, &[]);
    }
}

/// A capability name with a fixed meaning. Friend capability sets stay
/// `Ident`s because standalone `friends` accepts any capability name.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Capability {
    Make,
    Math,
    Bit,
    Rel,
    Cmp,
    Unsafe,
    /// `Make` without validation, after rustc has proven at compile time
    /// that every source value lies in the target's domain.
    Within,
}

impl Capability {
    const ITEMS: [Self; 7] = [
        Self::Make,
        Self::Math,
        Self::Bit,
        Self::Rel,
        Self::Cmp,
        Self::Unsafe,
        Self::Within,
    ];

    pub(crate) fn name(self) -> &'static str {
        return match self {
            Self::Make => "Make",
            Self::Math => "Math",
            Self::Bit => "Bit",
            Self::Rel => "Rel",
            Self::Cmp => "Cmp",
            Self::Unsafe => "Unsafe",
            Self::Within => "Within",
        };
    }

    pub(crate) fn of(name: &Ident) -> Option<Self> {
        return Self::ITEMS.into_iter().find(|it| *name == it.name());
    }

    pub(crate) fn ident(self) -> Ident {
        return format_ident!("{}", self.name());
    }

    /// The capability's trait name in a bitflag root protocol.
    pub(crate) fn flag_ident(self) -> Ident {
        return format_ident!("Flag{}", self.name());
    }

    /// What `self` grants wherever it is written: `unsafe` implies `Make`.
    pub(crate) fn implies(self) -> &'static [Self] {
        return match self {
            Self::Unsafe => &[Self::Make],
            Self::Make
            | Self::Math
            | Self::Bit
            | Self::Rel
            | Self::Cmp
            | Self::Within => &[],
        };
    }

    /// `Math` and `Bit`, the capabilities that grant operators.
    pub(crate) fn is_operator(self) -> bool {
        return matches!(self, Self::Math | Self::Bit);
    }
}

/// Which capabilities a friend site accepts, grants, and rejects.
/// Messages are diagnostics.
pub(crate) struct CapabilityPolicy {
    pub(crate) accepts: &'static [Capability],
    /// Granted to a friend declared without `-> ...`.
    pub(crate) defaults: &'static [Capability],
    /// Site-specific; universal implications live in `Capability::implies`.
    pub(crate) implies: &'static [(Capability, &'static [Capability])],
    /// Checked after `implies`, over the whole list, row by row per friend.
    pub(crate) rejects: &'static [(Capability, &'static str)],
    /// Capabilities only a `Self` friend may hold.
    pub(crate) self_only: &'static [(Capability, &'static str)],
    /// A name outside `accepts`.
    pub(crate) unknown: &'static str,
}

/// The capabilities an integral friend can name.
const INTEGRAL_ACCEPTS: &[Capability] = &[
    Capability::Make,
    Capability::Math,
    Capability::Bit,
    Capability::Rel,
    Capability::Unsafe,
    Capability::Within,
];

/// Integral root friends: `Math` implies `Make` and `Bit`, and `Within`
/// constructs, so it implies `Make`.
pub(crate) const INTEGRAL: CapabilityPolicy = CapabilityPolicy {
    accepts: INTEGRAL_ACCEPTS,
    defaults: &[Capability::Math],
    implies: &[
        (
            Capability::Math,
            &[
                Capability::Make,
                Capability::Bit,
            ],
        ),
        (Capability::Within, &[Capability::Make]),
    ],
    rejects: &[],
    self_only: &[],
    unknown: "unknown capability",
};

/// Float root friends: floats have no bitwise operations, so `Math`
/// implies `Make` only and `Bit` is rejected.
pub(crate) const FLOAT: CapabilityPolicy = CapabilityPolicy {
    accepts: INTEGRAL_ACCEPTS,
    defaults: &[Capability::Math],
    implies: &[(Capability::Math, &[Capability::Make])],
    rejects: &[
        (
            Capability::Bit,
            "the Bit capability requires an integer type",
        ),
        (
            Capability::Within,
            "`Within` supports integer wrappers only",
        ),
    ],
    self_only: &[],
    unknown: "unknown capability",
};

/// Facet friends select `Math` and `Bit` operations independently; the
/// facet checks run where the facet resolves.
pub(crate) const FACET: CapabilityPolicy = CapabilityPolicy {
    accepts: INTEGRAL_ACCEPTS,
    defaults: &[Capability::Math],
    implies: &[],
    rejects: &[],
    self_only: &[],
    unknown: "unknown capability",
};

/// Text friends: `Make` (with a conversion), `unsafe`, and `Rel` on `Self`.
pub(crate) const TEXT: CapabilityPolicy = CapabilityPolicy {
    accepts: &[
        Capability::Make,
        Capability::Unsafe,
        Capability::Rel,
    ],
    defaults: &[],
    implies: &[],
    rejects: &[],
    self_only: &[(Capability::Rel, "text `Rel` is only available for Self")],
    unknown: "unknown text capability",
};

const BITFLAG_ROOT_VALUE: &str = "bitflag root friends only support `Bit`; \
    declare value capabilities in `integral = [friend = ...]`";

/// Bitflag root friends bound the enum's bit operators only: every other
/// capability would be accepted without generating anything. `unsafe`
/// is reported first, because the `Make` it implies has no span of its own.
pub(crate) const BITFLAG_ROOT: CapabilityPolicy = CapabilityPolicy {
    accepts: &[
        Capability::Make,
        Capability::Math,
        Capability::Bit,
        Capability::Cmp,
        Capability::Unsafe,
        Capability::Within,
    ],
    defaults: &[],
    implies: &[],
    rejects: &[
        (Capability::Unsafe, BITFLAG_ROOT_VALUE),
        (
            Capability::Within,
            "`Within` belongs in the nested `integral = [...]` friend list",
        ),
        (Capability::Cmp, BITFLAG_ROOT_VALUE),
        (Capability::Make, BITFLAG_ROOT_VALUE),
        (Capability::Math, BITFLAG_ROOT_VALUE),
    ],
    self_only: &[],
    unknown: "unknown capability",
};

/// The operators a facet slot capability opens.
pub(crate) type SlotOperators = fn() -> &'static [Operator];

/// The facet slot capabilities and the operators each one opens, `Math`
/// first.
pub(crate) const FACET_OPERATORS: [(Capability, SlotOperators); 2] = [
    (Capability::Math, Operator::math_items),
    (Capability::Bit, Operator::bit_items),
];

impl CapabilityPolicy {
    /// One friend or a bracketed list: an `accepts` pass over every friend,
    /// then `implies`, then a `rejects` pass. A repeated friend keeps its
    /// first declaration.
    pub(crate) fn parse_list(
        &self,
        input: ParseStream,
    ) -> syn::Result<BTreeSet<Friend>> {
        let mut friends = BTreeSet::new();
        if input.peek(syn::token::Bracket) {
            let content;
            let _ = bracketed!(content in input);
            while !content.is_empty() {
                let friend =
                    Friend::parse_with_defaults(&content, self.defaults)?;
                friends.insert(friend);

                if !content.is_empty() {
                    let _ = content.parse::<Token![,]>()?;
                }
            }
        }
        else {
            friends.insert(Friend::parse_with_defaults(input, self.defaults)?);
        }

        for friend in &friends {
            self.check_accepts(friend)?;
        }

        let friends = friends
            .into_iter()
            .map(|mut friend| {
                for (capability, implied) in self.implies {
                    if friend.has(*capability) {
                        friend
                            .capabilities
                            .extend(implied.iter().map(|it| it.ident()));
                    }
                }
                return friend;
            })
            .collect::<BTreeSet<_>>();

        self.reject(&friends)?;
        return Ok(friends);
    }

    /// The `rejects` pass: per friend, the first row it holds.
    pub(crate) fn reject(
        &self,
        friends: &BTreeSet<Friend>,
    ) -> syn::Result<()> {
        for friend in friends {
            for (capability, message) in self.rejects {
                if let Some(written) = friend.capability(*capability) {
                    return written.fail(*message);
                }
            }
        }

        return Ok(());
    }

    /// `accepts` and `self_only` for one friend, in capability-name order.
    pub(crate) fn check(
        &self,
        friend: &Friend,
    ) -> syn::Result<()> {
        for written in &friend.capabilities {
            let capability =
                Capability::of(written).filter(|it| self.accepts.contains(it));
            let Some(capability) = capability
            else {
                return written.fail(self.unknown);
            };
            if let Some((_, message)) =
                self.self_only.iter().find(|(it, _)| *it == capability)
                && !friend.ty.path().is_ident("Self")
            {
                return written.fail(*message);
            }
        }

        return Ok(());
    }

    fn check_accepts(
        &self,
        friend: &Friend,
    ) -> syn::Result<()> {
        for written in &friend.capabilities {
            if !Capability::of(written)
                .is_some_and(|it| self.accepts.contains(&it))
            {
                return written.fail(self.unknown);
            }
        }

        return Ok(());
    }
}

struct CapabilityName(Ident);

impl Parse for CapabilityName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![unsafe]) {
            let keyword = input.parse::<Token![unsafe]>()?;
            return Ok(Self(Ident::new(
                Capability::Unsafe.name(),
                keyword.span,
            )));
        }

        return input.parse::<Ident>().map(Self);
    }
}

pub(crate) fn parse_capabilities(
    input: ParseStream
) -> syn::Result<BTreeSet<Ident>> {
    let capabilities = one_or_list::<CapabilityName>(input)?
        .map(|CapabilityName(capability)| capability)
        .set();
    return Ok(capabilities);
}

impl Friend {
    pub(crate) fn has(
        &self,
        capability: Capability,
    ) -> bool {
        return self.capabilities.contains(&capability.ident());
    }

    /// The written name of `capability`, when the friend holds it.
    pub(crate) fn capability(
        &self,
        capability: Capability,
    ) -> Option<&Ident> {
        return self
            .capabilities
            .iter()
            .find(|it| Capability::of(it) == Some(capability));
    }

    /// A friend declared with a conversion and no capability: generated
    /// code only checks that the conversion exists.
    pub(crate) fn is_check_only(&self) -> bool {
        return self.capabilities.is_empty() && self.conv.is_some();
    }

    pub(crate) fn parse_with_defaults(
        input: ParseStream,
        defaults: &[Capability],
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
        let ty = match (ty.pop(), ty) {
            (Some(it), rest) if rest.is_empty() => it,
            _ => {
                return here.fail(
                    "friend conversion requires exactly one source type",
                );
            }
        };

        let mut capabilities = if input.peek(Token![->]) {
            let _ = input.parse::<Token![->]>()?;
            parse_capabilities(input)?
        }
        else {
            defaults.iter().map(|it| it.ident()).collect()
        };
        let implied = capabilities
            .iter()
            .filter_map(Capability::of)
            .flat_map(|it| it.implies())
            .map(|it| it.ident())
            .collect::<Vec<_>>();
        capabilities.extend(implied);

        return Ok(Self {
            conv: conv.filter(|conv| !conv.is_ident("self")),
            capabilities,
            ty: PathIdentity::new(ty),
        });
    }
}

/// Takes `key`'s friend out of `friends` (else `key` itself), edits it, and
/// puts it back.
pub(crate) fn upsert_friend(
    friends: &mut BTreeSet<Friend>,
    key: Friend,
    edit: impl FnOnce(&mut Friend),
) {
    let mut friend = friends.take(&key).unwrap_or(key);
    edit(&mut friend);
    assert!(friends.insert(friend));
}

/// Whether any of `friends` holds `capability`.
pub(crate) fn any_friend_has(
    friends: &BTreeSet<Friend>,
    capability: Capability,
) -> bool {
    return friends.iter().any(|friend| friend.has(capability));
}

/// `Within` friends convert through the source's raw value and need no
/// trust: an explicit conversion or `unsafe` beside `Within` is an error.
pub(crate) fn check_within(friends: &BTreeSet<Friend>) -> syn::Result<()> {
    for friend in friends {
        let Some(within) = friend.capability(Capability::Within)
        else {
            continue;
        };
        if let Some(conversion) = &friend.conv {
            let source = friend.ty.path().to_token_stream().to_string();
            return conversion.fail(format!(
                "`Within` converts through the source's raw value; write \
                `_({})`",
                source.replace(' ', "")
            ));
        }
        if friend.has(Capability::Unsafe) {
            return within
                .fail("`Within` proves what `unsafe` trusts; use one");
        }
    }
    return Ok(());
}

impl Eq for Friend {}

impl PartialOrd for Friend {
    fn partial_cmp(
        &self,
        that: &Self,
    ) -> Option<Ordering> {
        return Some(self.cmp(that));
    }
}

impl PartialEq for Friend {
    fn eq(
        &self,
        that: &Self,
    ) -> bool {
        return self.ty == that.ty;
    }
}

impl Ord for Friend {
    fn cmp(
        &self,
        that: &Self,
    ) -> Ordering {
        return self.ty.cmp(&that.ty);
    }
}

impl From<&Path> for Friend {
    fn from(it: &Path) -> Self {
        return Self {
            ty: PathIdentity::new(it.clone()),
            conv: None,
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
            .field("identity", &self.ty.identity())
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

#[derive(Clone)]
pub(crate) struct ProtocolFriend {
    pub(crate) ty: Type,
    /// Keyed by trait name: today's impl order, also after `__Typekin…`
    /// renames.
    pub(crate) capabilities: BTreeMap<Ident, Capability>,
    pub(crate) conversion: Option<TokenStream>,
    pub(crate) seal_cfg: TokenStream,
    pub(crate) capability_cfgs: BTreeMap<Capability, TokenStream>,
    /// A `Within` friend's `make` doc: it constructs through the proven
    /// constructor.
    pub(crate) within: Option<TokenStream>,
}

pub(crate) struct Construction {
    /// Path of the target's constructor that skips validation.
    pub(crate) blind_trust: TokenStream,
    /// Path of the target's validating, panicking constructor.
    pub(crate) or_panic: TokenStream,
    /// Path of the target's constructor for proven `Within` sources.
    pub(crate) within: TokenStream,
    /// Path of the target's `_or_err` when construction reports failure:
    /// `Make` then has `try_make(self) -> Result<Target, Relation>`, the
    /// relation being the rejected value, instead of `make`.
    pub(crate) or_err: Option<TokenStream>,
}

/// A capability trait the protocol declares, under its generated name.
pub(crate) struct ProtocolCapability {
    pub(crate) capability: Capability,
    pub(crate) name: Ident,
}

pub(crate) struct Protocol {
    pub(crate) target: TokenStream,
    pub(crate) generics: syn::Generics,
    pub(crate) relation: Type,
    pub(crate) seal: Ident,
    pub(crate) conversion: Ident,
    pub(crate) capabilities: Vec<ProtocolCapability>,
    pub(crate) friends: Vec<ProtocolFriend>,
    pub(crate) emit_seal: bool,
    pub(crate) target_conversion: Option<TokenStream>,
    pub(crate) construction: Option<Construction>,
    pub(crate) konst: bool,
}

/// What every protocol item repeats: the `const` markers and the split
/// generics.
struct ProtocolHeader<'a> {
    konst: Option<TokenStream>,
    bonst: Option<TokenStream>,
    parameters: ImplGenerics<'a>,
    arguments: TypeGenerics<'a>,
    where_clause: Option<&'a WhereClause>,
}

impl<'a> ProtocolHeader<'a> {
    fn new(protocol: &'a Protocol) -> Self {
        let (parameters, arguments, where_clause) =
            protocol.generics.split_for_impl();

        return Self {
            konst: Konst::konst(protocol.konst),
            bonst: Konst::bonst(protocol.konst),
            parameters,
            arguments,
            where_clause,
        };
    }
}

impl Protocol {
    /// `make` belongs to `Make` when the protocol constructs.
    fn constructs(
        &self,
        capability: Capability,
    ) -> bool {
        return self.construction.is_some() && capability == Capability::Make;
    }

    /// Whether construction reports failure (`try_make`) instead of
    /// panicking (`make`).
    fn reports_failure(&self) -> bool {
        return self
            .construction
            .as_ref()
            .is_some_and(|it| it.or_err.is_some());
    }
}

pub(crate) fn emit_protocol(protocol: Protocol) -> TokenStream {
    let header = ProtocolHeader::new(&protocol);
    let seal_declaration = emit_seal_declaration(&protocol, &header);
    let capability_declarations = protocol.capabilities.iter().map(|it| {
        return emit_capability_declaration(&protocol, &header, it);
    });
    let target_seal = protocol.target_conversion.as_ref().map(|body| {
        return emit_seal_impl(
            &protocol,
            &header,
            &protocol.target,
            &TokenStream::new(),
            body,
        );
    });
    let friend_impls = protocol.friends.iter().map(|friend| {
        return emit_protocol_friend(&protocol, &header, friend);
    });

    return quote! {
        #seal_declaration
        #(#capability_declarations)*

        #target_seal
        #(#friend_impls)*
    };
}

fn emit_seal_declaration(
    protocol: &Protocol,
    header: &ProtocolHeader,
) -> Option<TokenStream> {
    let ProtocolHeader {
        konst,
        parameters,
        where_clause,
        ..
    } = header;
    let seal = &protocol.seal;
    let conversion = &protocol.conversion;
    let relation = &protocol.relation;

    return protocol.emit_seal.then(|| {
        quote! {
            #konst trait #seal #parameters #where_clause {
                fn #conversion(self) -> #relation;
            }
        }
    });
}

/// The three `#[doc]` lines on the unsafe capability.
fn blind_trust_doc() -> TokenStream {
    return quote! {
        #[doc = "# Safety"]
        #[doc = ""]
        #[doc = "Implementing this trait asserts that values \
                        produced by the sealed conversion satisfy every \
                        invariant required by the target type."]
    };
}

/// `unsafe` marks the blind-trust capability.
fn emit_capability_declaration(
    protocol: &Protocol,
    header: &ProtocolHeader,
    capability: &ProtocolCapability,
) -> TokenStream {
    let ProtocolHeader {
        konst,
        bonst,
        parameters,
        arguments,
        where_clause,
    } = header;
    let name = &capability.name;
    let seal = &protocol.seal;
    let blind_trust = capability.capability == Capability::Unsafe;
    let unsafe_ = blind_trust.then(|| quote! { unsafe });
    let safety = blind_trust.then(blind_trust_doc);
    let method = protocol.constructs(capability.capability).then(|| {
        let target = &protocol.target;
        let relation = &protocol.relation;
        return match protocol.reports_failure() {
            true => quote! {
                fn try_make(self) -> ::core::result::Result<#target, #relation>;
            },
            false => quote! { fn make(self) -> #target; },
        };
    });

    return quote! {
        #safety
        #konst #unsafe_ trait #name #parameters:
            #bonst #seal #arguments
        #where_clause {
            #method
        }
    };
}

/// Target seal (no cfg) and friend seal impls share one shape.
fn emit_seal_impl(
    protocol: &Protocol,
    header: &ProtocolHeader,
    ty: &dyn ToTokens,
    cfg: &TokenStream,
    body: &TokenStream,
) -> TokenStream {
    let ProtocolHeader {
        konst,
        parameters,
        arguments,
        where_clause,
        ..
    } = header;
    let seal = &protocol.seal;
    let conversion = &protocol.conversion;
    let relation = &protocol.relation;

    return quote! {
        #cfg
        #konst impl #parameters #seal #arguments for #ty
        #where_clause {
            #[inline(always)]
            fn #conversion(self) -> #relation {
                #body
            }
        }
    };
}

/// One capability impl of `friend`; `Make` constructs through the proven
/// constructor for a `Within` friend, the blind-trust one when the friend
/// also holds `Unsafe`, and the validating one otherwise. Reporting
/// failure, every friend validates and returns the rejected raw value.
fn emit_friend_capability(
    protocol: &Protocol,
    header: &ProtocolHeader,
    friend: &ProtocolFriend,
    name: &Ident,
    capability: Capability,
) -> TokenStream {
    let ProtocolHeader {
        konst,
        parameters,
        arguments,
        where_clause,
        ..
    } = header;
    let ty = &friend.ty;
    let capability_cfg = friend.capability_cfgs.get(&capability);
    let unsafe_ = (capability == Capability::Unsafe).then(|| quote! { unsafe });
    let method = protocol
        .construction
        .as_ref()
        .filter(|_| protocol.constructs(capability))
        .map(|construction| {
            let target = &protocol.target;
            let seal = &protocol.seal;
            let conversion = &protocol.conversion;
            let raw = quote! {
                let raw = <Self as #seal #arguments>::#conversion(self);
            };
            if let Some(or_err) = &construction.or_err {
                let relation = &protocol.relation;
                return quote! {
                    #[inline(always)]
                    fn try_make(
                        self,
                    ) -> ::core::result::Result<#target, #relation> {
                        #raw
                        return match #or_err(::core::option::Option::Some(raw)) {
                            ::core::result::Result::Ok(it) => {
                                ::core::result::Result::Ok(it)
                            }
                            ::core::result::Result::Err(_) => {
                                ::core::result::Result::Err(raw)
                            }
                        };
                    }
                };
            }

            let blind_trust = friend
                .capabilities
                .values()
                .any(|it| *it == Capability::Unsafe);
            let constructor = match (&friend.within, blind_trust) {
                (Some(_), _) => &construction.within,
                (None, true) => &construction.blind_trust,
                (None, false) => &construction.or_panic,
            };
            let doc = &friend.within;
            return quote! {
                #doc
                #[inline(always)]
                fn make(self) -> #target {
                    #raw
                    return #constructor(raw);
                }
            };
        });

    return quote! {
        #capability_cfg
        #konst #unsafe_ impl #parameters #name #arguments for #ty
        #where_clause {
            #method
        }
    };
}

/// `friend`'s seal impl, then its capability impls.
fn emit_protocol_friend(
    protocol: &Protocol,
    header: &ProtocolHeader,
    friend: &ProtocolFriend,
) -> TokenStream {
    let seal_impl = friend.conversion.as_ref().map(|body| {
        return emit_seal_impl(
            protocol,
            header,
            &friend.ty,
            &friend.seal_cfg,
            body,
        );
    });
    let capability_impls = friend.capabilities.iter().map(|(name, it)| {
        return emit_friend_capability(protocol, header, friend, name, *it);
    });

    return quote! {
        #seal_impl
        #(#capability_impls)*
    };
}
