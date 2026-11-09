use std::collections::{
    BTreeMap,
    BTreeSet,
};

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::{
    Generics,
    Path,
    parse_quote,
};

use crate::integral::{
    Maker,
    pointer_width_cfg,
};
use crate::integral_cfg::{
    Cfg,
    IntegralFlag,
    IntegralFlags,
    IntegralName,
};
use crate::zz_friend::{
    Capability,
    Construction,
    Friend,
    Protocol,
    ProtocolCapability,
    ProtocolFriend,
    any_friend_has,
    emit_protocol,
    upsert_friend,
};
use crate::zz_lib::Konst;
use crate::zz_profile_integral::{
    FacetSlot,
    FacetSlotKey,
};
use crate::zz_runner::{
    Merged,
    MkErr,
    ToCollection,
};
use crate::zz_values::{
    Num,
    OPS,
    Op,
    Width,
};

/// Integral friend capabilities are validated: each has a protocol trait.
const TRAITS: &str = "integral friend capabilities are validated";

/// The friend a built-in friendship is granted to.
#[derive(Clone, Copy)]
struct Builtin {
    is_target: bool,
    is_generic: bool,
    is_validated: bool,
}

/// A capability every built-in friend gets when `applies` holds.
struct BuiltinRule {
    capability: Capability,
    applies: fn(&IntegralFlags, Builtin) -> bool,
}

/// Generic Self operations share the RHS protocol: separate Self impls would
/// overlap when downstream parameters satisfy it.
const BUILTIN_RULES: &[BuiltinRule] = &[
    BuiltinRule {
        capability: Capability::Make,
        applies: |flags, it| {
            return !it.is_validated && flags.has(IntegralFlag::SelfFriendMake);
        },
    },
    BuiltinRule {
        capability: Capability::Math,
        applies: |flags, it| {
            return !it.is_target && flags.has(IntegralFlag::SelfFriendMathOps);
        },
    },
    BuiltinRule {
        capability: Capability::Rel,
        applies: |flags, it| {
            return it.is_target || flags.has(IntegralFlag::SelfFriendRel);
        },
    },
    BuiltinRule {
        capability: Capability::Bit,
        applies: |flags, it| {
            return !it.is_target && flags.has(IntegralFlag::SelfFriendMathBit);
        },
    },
    BuiltinRule {
        capability: Capability::Math,
        applies: |flags, it| {
            return it.is_generic
                && it.is_target
                && flags.has(IntegralFlag::SelfMath);
        },
    },
    BuiltinRule {
        capability: Capability::Bit,
        applies: |flags, it| {
            return it.is_generic
                && it.is_target
                && flags.has(IntegralFlag::SelfBit);
        },
    },
];

impl Maker {
    fn friend_conversion_expression(
        &self,
        ty: &Path,
        conversion: Option<&Path>,
        value: TokenStream,
    ) -> TokenStream {
        if self.target.is_target(ty) {
            let path = &self.ctx.get_raw;
            let get_raw = self.target.qualify_path(path);
            return quote! { #get_raw(#value) };
        }

        let relation = &self.el;
        let primitive = ty.get_ident().and_then(|it| Num::of(it.to_string()));
        if let Some(primitive) = primitive
            && (matches!(primitive, Num::Int(_))
                || matches!(self.num, Num::Float(_)))
        {
            return quote! { #value as #relation };
        }

        if conversion.is_some_and(|conversion| conversion.is_ident("self")) {
            return quote! {
                {
                    let relation: #relation = #value;
                    relation
                }
            };
        }

        return match conversion {
            Some(conversion) => {
                let conversion = self.target.qualify_path(conversion);
                quote! { #conversion(#value) }
            }
            None => quote! {
                {
                    let relation: #relation = #value.into();
                    relation
                }
            },
        };
    }

    pub(crate) fn make_empty_friend_checks(&self) -> TokenStream {
        let friends = self.empty_friends();
        if friends.is_empty() {
            return TokenStream::new();
        }

        let has_const_check = friends.iter().any(|(_, konst)| *konst);
        let callback_name = self.target.fresh_ident("__typekin_callback");
        let checks = friends
            .iter()
            .map(|(friend, konst)| {
                return self.empty_friend_check(friend, *konst, &callback_name);
            })
            .merged();

        if !self.target.has_generics() {
            return checks;
        }

        let check_name = self.empty_friend_check_name(&friends);
        let generics = self.target.external_generics();
        let (parameters, _, where_clause) = generics.split_for_impl();
        let konst = Konst::konst(has_const_check);

        return quote! {
            #konst fn #check_name #parameters() #where_clause {
                #checks
            }
        };
    }

    /// Friends declared with a conversion but no capability, each with
    /// whether its check must be const-callable.
    fn empty_friends(&self) -> Vec<(&Friend, bool)> {
        let cfg = &self.ctx.cfg;
        let mut friends = cfg
            .friends
            .iter()
            .filter(|friend| friend.is_check_only())
            .map(|friend| (friend, cfg.konst_friendship()))
            .collect::<Vec<_>>();

        for facet in &cfg.facets {
            let mut facet_friends = facet
                .friends
                .iter()
                .filter(|friend| friend.is_check_only())
                .peekable();
            if facet_friends.peek().is_none() {
                continue;
            }

            let default = cfg
                .default_facet
                .as_ref()
                .expect("facets require a resolved default facet");
            let flags = Cfg::resolved_facet_flags(&default.flags, facet);
            let konst =
                OPS.iter().filter(|it| it.kind.has_operand()).any(|it| {
                    return flags.has(IntegralFlag::of_op(it.op).math)
                        && cfg.konst_for(&it.konsts());
                });
            friends.extend(facet_friends.map(|friend| (friend, konst)));
        }

        return friends;
    }

    fn empty_friend_check(
        &self,
        friend: &Friend,
        konst: bool,
        callback_name: &Ident,
    ) -> TokenStream {
        let relation = &self.el;
        let conversion = self.target.qualify_path(
            friend.conv.as_ref().expect("explicit callback missing"),
        );
        let source = friend.ty.path();
        let source = self.target.external_type(&parse_quote! { #source });
        if !konst {
            return quote! {
                let _: fn(#source) -> #relation = #conversion;
            };
        }

        // A const FnOnce bound checks const-callability without calling it.
        return quote! {
            {
                let #callback_name = #conversion;
                {
                    const fn __typekin_assert_const_fn<I, O, F>(_: F)
                    where
                        F: [const] ::core::ops::FnOnce(I) -> O
                            + ::core::marker::Copy,
                    {}
                    __typekin_assert_const_fn::<#source, #relation, _>(
                        #callback_name,
                    );
                }
            }
        };
    }

    /// A fresh name that no friend's conversion path starts with.
    fn empty_friend_check_name(
        &self,
        friends: &[(&Friend, bool)],
    ) -> Ident {
        let mut name = "__typekin_check_empty_friends".to_owned();
        loop {
            let candidate = self.target.fresh_ident(&name);
            let conflicts = friends.iter().any(|(friend, _)| {
                friend
                    .conv
                    .as_ref()
                    .and_then(|path| path.segments.first())
                    .is_some_and(|segment| segment.ident == candidate)
            });
            if !conflicts {
                return candidate;
            }
            name.push('_');
        }
    }

    pub(crate) fn fix_friendship(&mut self) -> syn::Result<()> {
        let target: Path = syn::parse2(self.target.ty())
            .expect("integral targets does not emit valid type paths");

        if matches!(self.num, Num::Float(_)) {
            self.reject_lossy_float_relations()?;
        }

        let target_friend = self
            .ctx
            .cfg
            .friends
            .iter()
            .find(|friend| self.target.is_target(friend.ty.path()))
            .cloned()
            .unwrap_or_else(|| Friend::from(&target));
        self.reject_self_operator_friend(&target_friend)?;

        let is_generic = self.target.has_generics();
        for (f_def, conv, is_validated) in [
            (target_friend, Some(parse_quote! { Self::raw }), false),
            (
                Friend::from(&self.el),
                Some(parse_quote! { self }),
                self.ctx.cfg.validation.has_validation(),
            ),
        ] {
            let builtin = Builtin {
                is_target: self.target.is_target(f_def.ty.path()),
                is_generic,
                is_validated,
            };
            let flags = self.ctx.cfg.flags;
            upsert_friend(&mut self.ctx.cfg.friends, f_def, |it| {
                if it.conv.is_none() {
                    it.conv = conv;
                }

                for rule in BUILTIN_RULES {
                    if (rule.applies)(&flags, builtin) {
                        it.capabilities.insert(rule.capability.ident());
                    }
                }
            });
        }

        self.add_safe_cast_make_friends();

        return Ok(());
    }

    /// Self operators come from `self_math`/`self_bit`, not from a friend.
    fn reject_self_operator_friend(
        &self,
        target_friend: &Friend,
    ) -> syn::Result<()> {
        let Some(it) = self.ctx.cfg.friends.get(target_friend)
        else {
            return Ok(());
        };
        if it.has(Capability::Math) {
            return it
                .ty
                .path()
                .fail("self math operations use `self_math`, not `friend`");
        }
        if it.has(Capability::Bit) {
            return it
                .ty
                .path()
                .fail("self bit operations use `self_bit`, not `friend`");
        }
        return Ok(());
    }

    /// Every primitive that casts safely into the target becomes a `Make`
    /// friend, gated by `cfg` when only some pointer widths allow it.
    fn add_safe_cast_make_friends(&mut self) {
        if self.ctx.cfg.validation.has_validation()
            || !self.ctx.cfg.flags.has(IntegralFlag::SelfFriendMake)
        {
            return;
        }

        for source in Num::items() {
            if source == self.num {
                continue;
            }
            let Some(cfg) = pointer_width_cfg(|width| {
                source.can_safe_cast_to(self.num, width)
            })
            else {
                continue;
            };
            let ty = format_ident!("{}", source.name());
            let mut newly_added = false;
            upsert_friend(
                &mut self.ctx.cfg.friends,
                Friend::from(&ty),
                |friend| {
                    newly_added =
                        friend.capabilities.insert(Capability::Make.ident());
                },
            );
            if newly_added && !cfg.is_empty() {
                self.conditional_make.insert(source, cfg);
            }
        }
    }

    /// `as` may round a primitive into a float. That is the documented
    /// meaning of a `Make` friend; comparisons and arithmetic must not
    /// silently round, so `Rel` and `Math` need an exact source.
    fn reject_lossy_float_relations(&self) -> syn::Result<()> {
        for friend in &self.ctx.cfg.friends {
            let Some(ident) = friend.ty.path().get_ident()
            else {
                continue;
            };
            let Some(source) = Num::of(ident.to_string())
            else {
                continue;
            };
            let exact = Width::ALL
                .into_iter()
                .all(|width| source.can_safe_cast_to(self.num, width));
            let relation = friend.capabilities.iter().find(|it| {
                return matches!(
                    Capability::of(it),
                    Some(Capability::Math | Capability::Rel)
                );
            });
            if let Some(capability) = relation
                && !exact
            {
                return capability.fail(format!(
                    "{} -> {} is lossy; float Rel and Math friends must \
                    convert exactly",
                    source.name(),
                    self.num.name(),
                ));
            }
        }
        return Ok(());
    }

    fn make_facet_dispatch(&self) -> TokenStream {
        if !self.ctx.cfg.facets_present() {
            return TokenStream::new();
        }

        let target = self.target.ty();
        let generics = self.target.external_generics();
        let operators = self
            .ctx
            .cfg
            .facet_slots
            .keys()
            .map(|slot| slot.operator)
            .collect::<BTreeSet<_>>();
        let traits = operators.iter().map(|operator| {
            return self.facet_rhs_trait_decl(*operator, &target, &generics);
        });
        let implementations =
            self.ctx.cfg.facet_slots.iter().map(|(key, slot)| {
                return self.facet_rhs_impl(key, slot, &target, &generics);
            });
        let operator_impls = operators.iter().map(|operator| {
            return self.facet_operator_impl(*operator, &target, &generics);
        });
        let assign_impls = self
            .ctx
            .cfg
            .facet_slots
            .iter()
            .filter(|(_, slot)| slot.assign)
            .map(|(key, slot)| {
                return self.facet_assign_impl(key, slot, &target, &generics);
            });

        return quote! {
            #(#traits)*
            #(#implementations)*
            #(#operator_impls)*
            #(#assign_impls)*
        };
    }

    /// The `{Op}Rhs` trait a facet operator dispatches through.
    fn facet_rhs_trait(
        &self,
        operator: Op,
    ) -> Ident {
        return self.ctx.names.get(IntegralName::RhsTrait(operator));
    }

    fn facet_rhs_trait_decl(
        &self,
        operator: Op,
        target: &TokenStream,
        generics: &Generics,
    ) -> TokenStream {
        let (parameters, _, where_clause) = generics.split_for_impl();
        let trait_name = self.facet_rhs_trait(operator);
        let operator = operator.spec();
        let method = format_ident!("{}", operator.method);
        let is_konst = self.ctx.cfg.konst_for(&operator.konsts());
        let konst = Konst::konst(is_konst);

        return quote! {
            #[doc(hidden)]
            #[allow(private_interfaces)]
            pub #konst trait #trait_name #parameters #where_clause {
                type Output;

                fn #method(
                    lhs: #target,
                    rhs: Self,
                ) -> Self::Output;
            }
        };
    }

    /// The type a slot's right-hand side is: the friend's, else the target.
    fn facet_rhs_ty(
        &self,
        slot: &FacetSlot,
        target: &TokenStream,
    ) -> TokenStream {
        return slot.rhs.as_ref().map_or_else(
            || target.clone(),
            |rhs| self.target.external_type(&parse_quote! { #rhs }),
        );
    }

    /// Whether a slot's result is proven valid: for every operand of a
    /// friend slot, or for same-type operands of the `Self` slot.
    fn facet_proven(
        &self,
        key: &FacetSlotKey,
        slot: &FacetSlot,
    ) -> bool {
        let proof =
            self.operator_proof_with(key.operator.spec(), slot.arithmetic);
        return match slot.rhs {
            Some(_) => proof.full,
            None => proof.same,
        };
    }

    fn facet_rhs_impl(
        &self,
        key: &FacetSlotKey,
        slot: &FacetSlot,
        target: &TokenStream,
        generics: &Generics,
    ) -> TokenStream {
        let (parameters, arguments, where_clause) = generics.split_for_impl();
        let operator = key.operator.spec();
        let trait_name = self.facet_rhs_trait(operator.op);
        let method = format_ident!("{}", operator.method);
        let rhs_ty = self.facet_rhs_ty(slot, target);
        let get_raw = self.target.qualify_path(&self.ctx.get_raw);
        let rhs = match &slot.rhs {
            Some(rhs) => self.friend_conversion_expression(
                rhs,
                slot.conversion.as_ref(),
                quote! { rhs },
            ),
            None => quote! { #get_raw(rhs) },
        };
        let proven = self.facet_proven(key, slot);
        let path = match (&self.ctx.failure, proven) {
            (Some(_), _) => {
                let or_err = self.ctx.names.get(IntegralName::OrErr);
                parse_quote! { Self::#or_err }
            }
            (None, true) => parse_quote! { Self::_proven },
            (None, false) => self.ctx.fp_or_panic.clone(),
        };
        let constructor = self.target.qualify_path(&path);
        let doc = self.proof_doc(proven);
        let output = self.target.operator_output(
            &slot.ty_out,
            true,
            self.ctx.failure.as_ref(),
        );
        let output_ty = output.ty();
        let result = output.convert(quote! { value });
        let is_konst = self.ctx.cfg.konst_for(&operator.konsts());
        let operation = match self.ctx.failure {
            Some(_) => slot.arithmetic.make_fallible_expr(
                operator,
                self.ctx.cfg.wide.panic_at,
                matches!(self.num, Num::Float(_)),
                is_konst,
            ),
            None => slot.arithmetic.make_operator_expr(operator, is_konst),
        };
        let konst = Konst::konst(is_konst);

        return quote! {
            #konst impl #parameters #trait_name #arguments for #rhs_ty
            #where_clause {
                type Output = #output_ty;

                #doc
                #[inline(always)]
                fn #method(
                    lhs: #target,
                    rhs: #rhs_ty,
                ) -> Self::Output {
                    let lhs = #get_raw(lhs);
                    let rhs = #rhs;
                    let value = #operation;
                    let value = #constructor(value);
                    return #result;
                }
            }
        };
    }

    /// `core::ops::{Op}<T> for Target` forwarding to `T`'s `{Op}Rhs` impl.
    fn facet_operator_impl(
        &self,
        operator: Op,
        target: &TokenStream,
        generics: &Generics,
    ) -> TokenStream {
        let (_, arguments, _) = generics.split_for_impl();
        let trait_name = self.facet_rhs_trait(operator);
        let operator = operator.spec();
        let method = format_ident!("{}", operator.method);
        let operation = format_ident!("{}", operator.trait_name);
        let rhs = self.target.fresh_ident("T");
        let is_konst = self.ctx.cfg.konst_for(&operator.konsts());
        let konst = Konst::konst(is_konst);
        let bound = Konst::bound(is_konst, quote! { #trait_name #arguments });
        let (impl_generics, where_clause) = self.target.impl_header(
            &[parse_quote! { #rhs }],
            &[parse_quote! { #rhs: #bound }],
        );

        return quote! {
            #[allow(private_bounds)]
            #konst impl #impl_generics ::core::ops::#operation<#rhs>
                for #target
            #where_clause {
                type Output = <#rhs as #trait_name #arguments>::Output;

                #[inline(always)]
                fn #method(
                    self,
                    rhs: #rhs,
                ) -> Self::Output {
                    return <#rhs as #trait_name #arguments>::#method(self, rhs);
                }
            }
        };
    }

    /// `core::ops::{Op}Assign<Rhs> for Target` of a slot whose output is
    /// `Self`, forwarding to the slot's `{Op}Rhs` impl.
    fn facet_assign_impl(
        &self,
        key: &FacetSlotKey,
        slot: &FacetSlot,
        target: &TokenStream,
        generics: &Generics,
    ) -> TokenStream {
        let (parameters, arguments, where_clause) = generics.split_for_impl();
        let operator = key.operator.spec();
        let trait_name = self.facet_rhs_trait(operator.op);
        let method = format_ident!("{}", operator.method);
        let assign_trait = format_ident!("{}Assign", operator.trait_name);
        let assign_method = format_ident!("{}_assign", operator.method);
        let rhs_ty = self.facet_rhs_ty(slot, target);
        let doc = self.proof_doc(self.facet_proven(key, slot));
        let is_konst = self.ctx.cfg.konst_for(&operator.assign_konsts());
        let konst = Konst::konst(is_konst);

        return quote! {
            #konst impl #parameters ::core::ops::#assign_trait<#rhs_ty>
                for #target
            #where_clause {
                #doc
                #[inline(always)]
                fn #assign_method(
                    &mut self,
                    rhs: #rhs_ty,
                ) {
                    *self = <#rhs_ty as #trait_name #arguments>::#method(
                        *self,
                        rhs,
                    );
                }
            }
        };
    }

    pub(crate) fn ekran_friendship(&self) -> TokenStream {
        let el = &self.el;
        let get_raw = &self.ctx.get_raw;
        let target_conversion =
            self.ctx.cfg.flags.has(IntegralFlag::FriendSeal).then(|| {
                quote! {
                    return #get_raw(self);
                }
            });
        let friends = self
            .ctx
            .cfg
            .friends
            .iter()
            .filter_map(|friend| self.protocol_friend(friend))
            .vec();

        let protocol = emit_protocol(Protocol {
            target: self.ctx.ty.clone(),
            generics: self.target.external_generics(),
            relation: parse_quote! { #el },
            seal: self.ctx.friends.seal().clone(),
            conversion: self.ctx.friends.conversion().clone(),
            capabilities: self.protocol_capabilities(),
            friends,
            emit_seal: self.ctx.cfg.flags.has(IntegralFlag::FriendzoneSeal),
            target_conversion,
            construction: Some(self.protocol_construction()),
            konst: self.ctx.cfg.konst_friendship(),
        });

        let dispatch = self.make_facet_dispatch();

        return quote! {
            #protocol

            #dispatch
        };
    }

    /// The capability traits the protocol declares, in declaration order;
    /// `Unsafe` only when a friend is blind-trusted, since only that friend's
    /// impl names it.
    fn protocol_capabilities(&self) -> Vec<ProtocolCapability> {
        let flags = &self.ctx.cfg.flags;
        let blind_trust =
            any_friend_has(&self.ctx.cfg.friends, Capability::Unsafe);
        return [
            (
                flags.has(IntegralFlag::FriendzoneFriendMake),
                Capability::Make,
            ),
            (
                flags.has(IntegralFlag::FriendzoneFriendMathOps),
                Capability::Math,
            ),
            (
                flags.has(IntegralFlag::FriendzoneFriendMathBit),
                Capability::Bit,
            ),
            (
                flags.has(IntegralFlag::FriendzoneFriendRel),
                Capability::Rel,
            ),
            (blind_trust, Capability::Unsafe),
        ]
        .into_iter()
        .filter_map(|(enabled, capability)| enabled.then_some(capability))
        .filter_map(|capability| {
            let name = self.emitted_trait(capability)?;
            return Some(ProtocolCapability { capability, name });
        })
        .vec();
    }

    /// The protocol trait of `capability`, unless facets dispatch operators;
    /// `Within` changes how `Make` constructs and has no trait.
    fn emitted_trait(
        &self,
        capability: Capability,
    ) -> Option<Ident> {
        if (self.ctx.cfg.facets_present() && capability.is_operator())
            || capability == Capability::Within
        {
            return None;
        }
        return Some(self.ctx.friends.of(capability).expect(TRAITS).clone());
    }

    /// The written capabilities that get a protocol impl, by trait name.
    fn emitted_capabilities<'a>(
        &self,
        written: impl Iterator<Item = &'a Ident>,
    ) -> BTreeMap<Ident, Capability> {
        return written
            .filter_map(|written| {
                let capability = Capability::of(written).expect(TRAITS);
                let name = self.emitted_trait(capability)?;
                return Some((name, capability));
            })
            .collect();
    }

    fn protocol_friend(
        &self,
        friend: &Friend,
    ) -> Option<ProtocolFriend> {
        let capabilities =
            self.emitted_capabilities(friend.capabilities.iter());
        if capabilities.is_empty() {
            return None;
        }

        let make_cfg = friend
            .ty
            .path()
            .get_ident()
            .and_then(|ident| Num::of(ident.to_string()))
            .and_then(|source| self.conditional_make.get(&source));
        let seal_cfg = if capabilities.len() == 1 {
            make_cfg.cloned().unwrap_or_default()
        }
        else {
            TokenStream::new()
        };
        let mut capability_cfgs = BTreeMap::new();
        if let Some(cfg) = make_cfg {
            capability_cfgs.insert(Capability::Make, cfg.clone());
        }

        let within = friend.has(Capability::Within);
        let conversion = if self.target.is_target(friend.ty.path()) {
            None
        }
        else if within {
            Some(self.within_conversion(friend))
        }
        else {
            let value = self.friend_conversion_expression(
                friend.ty.path(),
                friend.conv.as_ref(),
                quote! { self },
            );
            Some(quote! { return #value; })
        };

        let friend_ty = friend.ty.path();
        return Some(ProtocolFriend {
            ty: syn::parse2(
                self.target.external_type(&parse_quote! { #friend_ty }),
            )
            .expect("friend types remain valid after target lowering"),
            capabilities,
            conversion,
            seal_cfg,
            capability_cfgs,
            within: within.then(|| self.within_doc(friend)),
        });
    }

    /// The target's constructors the `Make`, `Unsafe`, and `Within` impls
    /// call.
    fn protocol_construction(&self) -> Construction {
        let or_panic = self.target.qualify_path(&self.ctx.fp_or_panic);
        let blind_trust = self.ctx.names.get(IntegralName::BlindTrust);
        let blind_trust = self
            .target
            .qualify_path(&parse_quote! { Self::#blind_trust });
        let within = self.ctx.names.get(IntegralName::WithinFn);
        let within = self.target.qualify_path(&parse_quote! { Self::#within });
        let or_err = self.ctx.failure.as_ref().map(|_| {
            let or_err = self.ctx.names.get(IntegralName::OrErr);
            return self.target.qualify_path(&parse_quote! { Self::#or_err });
        });
        return Construction {
            blind_trust,
            or_panic,
            within,
            or_err,
        };
    }
}
