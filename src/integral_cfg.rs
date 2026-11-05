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
    Span,
};
use quote::ToTokens;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::spanned::Spanned as _;
use syn::{
    Path,
    parse_quote,
};

use crate::integral_proof::ProveItem;
use crate::zz_conv::SelfConverter;
use crate::zz_domain_float::MathProvider;
use crate::zz_flags::{
    Flag,
    FlagGrammar,
    FlagSettings,
    Flags,
    KonstFlags,
};
use crate::zz_friend::{
    Capability,
    FACET_OPERATORS,
    FLOAT,
    Friend,
    INTEGRAL,
    check_within,
    upsert_friend,
};
use crate::zz_hide::{
    HideCfg,
    HideIn,
};
use crate::zz_lib::{
    Key,
    Konst,
    Konsts,
    Removals,
    ValidationCfg,
};
use crate::zz_names::NameKey;
use crate::zz_profile_integral::{
    Arithmetic,
    DefaultFacetCfg,
    FacetCfg,
    FacetOwner,
    FacetSlot,
    FacetSlotKey,
    Profile,
    ProfileKind,
    Profiles,
};
use crate::zz_runner::{
    MkErr,
    snake_case_of,
};
use crate::zz_values::{
    FnName,
    N,
    Num,
    Operator,
};
use crate::{
    marker_tree,
    zz_lib as zz,
};

/// The `with`/`without` grammar of an integral configuration level.
pub(crate) const GRAMMAR: FlagGrammar = FlagGrammar {
    konsts: Konsts::INTEGRAL,
    removals: true,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum NonZeroPolicy {
    #[default]
    Infer,
    Require,
    Suppress,
}

/// The value field's numeric kind, fixed by the entry point before parsing.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum Kind {
    #[default]
    Int,
    Float,
}

/// Where a configuration is parsed, which decides the keys it accepts.
#[derive(Clone, Copy, Eq, PartialEq)]
enum CfgSite {
    /// `integral` and `float`.
    Root,
    /// A fixed-profile macro (`circular`, `point`, …): no `profile`, but
    /// `precedes`.
    FixedProfile,
    /// Bitflag's `integral = [..]`: no `konst`, `tree`, or `hide_in`.
    Nested,
}

/// The operand source of a facet level's slots.
struct SlotSource<'a> {
    flags: &'a IntegralFlags,
    arithmetic: Arithmetic,
    ty_out: &'a SelfConverter,
    owner: FacetOwner,
}

#[derive(Default)]
pub(crate) struct Cfg {
    pub(crate) konst: Konst,
    konst_flags: KonstFlags,
    root: FlagSettings<IntegralFlag>,
    /// `without = fn_{name}` from the profiles and the root.
    pub(crate) removals: Removals,
    pub(crate) get_raw: Option<Path>,
    pub(crate) tree: Option<marker_tree::Cfg>,
    pub(crate) validation: ValidationCfg,
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) flags: IntegralFlags,
    pub(crate) non_zero: NonZeroPolicy,
    wrapping_storage: bool,
    pub(crate) ty_out: SelfConverter,
    pub(crate) profile: Profiles,
    pub(crate) facets: Vec<FacetCfg>,
    pub(crate) default_facet: Option<DefaultFacetCfg>,
    pub(crate) facet_slots: BTreeMap<FacetSlotKey, FacetSlot>,
    facets_present: bool,
    /// Set by bitflag, whose named values are `const` items built through
    /// `_or_panic`: its validation must stay const-callable.
    pub(crate) const_validation_required: bool,
    pub(crate) hide_in: crate::zz_hide::HideIn,
    /// `lit_name = ident`: overrides the literal macro's name.
    pub(crate) lit_name: Option<Ident>,
    /// `lit_path = crate::…::Type`: the path the literal macro expands to.
    pub(crate) lit_path: Option<Path>,
    /// `prove = [...]`: operations that must be proven closed.
    pub(crate) prove: Vec<ProveItem>,
    pub(crate) kind: Kind,
    /// `math = core | std | nightly` (floats only).
    pub(crate) math: MathProvider,
    /// The value field is not private, so code outside the generated API
    /// can store any value: generated code must not assume a valid one.
    pub(crate) visible_value: bool,
    math_span: Option<Span>,
}

impl Cfg {
    pub(crate) fn requires_non_zero_storage(&self) -> bool {
        return match self.non_zero {
            NonZeroPolicy::Infer => self.validation.domain_excludes_zero(),
            NonZeroPolicy::Require => true,
            NonZeroPolicy::Suppress => false,
        };
    }

    pub(crate) fn requires_wrapping_storage(&self) -> bool {
        return self.wrapping_storage;
    }

    pub(crate) fn facets_present(&self) -> bool {
        return self.facets_present;
    }

    fn validate_facet(it: &FacetCfg) -> syn::Result<()> {
        if let Some(span) = it.precedes_span {
            return span.fail(
                "`precedes` is wrapper-wide and cannot be configured inside a facet",
            );
        }

        for profile in it.profile.iter() {
            Self::validate_facet_settings(profile.settings())?;
        }

        return Self::validate_facet_friends(&it.friends);
    }

    /// A facet level's settings: konst flags, removals, then the flags in
    /// name order.
    fn validate_facet_settings(
        settings: &FlagSettings<IntegralFlag>
    ) -> syn::Result<()> {
        if let Some(span) = settings.konsts().first_span() {
            return span.fail(
                "constness is wrapper-wide and cannot be configured inside a facet",
            );
        }
        if let Some(span) = settings.removals().first_span() {
            return span.fail(
                "function removals are wrapper-wide and cannot be \
                configured inside a facet",
            );
        }

        for (flag, enabled, span) in settings.flags_by_name() {
            if flag == IntegralFlag::NonZero {
                return span.fail(
                    "`non_zero` is wrapper-wide and cannot be configured inside a facet",
                );
            }

            if IntegralFlag::ASSIGN_FLAGS.contains(&flag) {
                if enabled {
                    return span
                        .fail("facets do not support assignment operators");
                }
                continue;
            }

            if !IntegralFlag::FACET_OPERANDS.contains(&flag) {
                return span.fail(
                    "attribute is wrapper-wide or has no operand \
                    slot and cannot be configured inside a facet",
                );
            }
        }

        return Ok(());
    }

    /// Facet friends open operand slots only.
    fn validate_facet_friends(friends: &BTreeSet<Friend>) -> syn::Result<()> {
        for friend in friends {
            for capability in &friend.capabilities {
                if !FACET_OPERATORS
                    .iter()
                    .any(|(slot, _)| Capability::of(capability) == Some(*slot))
                {
                    return capability.fail(
                        "facet friends only support `Math` and `Bit` capabilities",
                    );
                }
            }
        }

        return Ok(());
    }

    pub(crate) fn resolved_facet_flags(
        baseline: &IntegralFlags,
        facet: &FacetCfg,
    ) -> IntegralFlags {
        let mut flags = *baseline;
        for profile in facet.profile.iter() {
            profile.apply(&mut flags);
            profile.settings().apply(&mut flags);
        }
        return flags;
    }

    fn insert_explicit_slot(
        slots: &mut BTreeMap<FacetSlotKey, FacetSlot>,
        key: FacetSlotKey,
        slot: FacetSlot,
    ) -> syn::Result<()> {
        if let Some(previous) = slots.get(&key) {
            return slot.fail_combi(
                "duplicate explicit facet slot",
                previous.span,
                "the slot was first claimed here",
            );
        }

        slots.insert(key, slot);
        return Ok(());
    }

    /// One slot per enabled operator of `operations` for `rhs` (`Self` when
    /// `None`). An explicit facet rejects a claimed slot; the default facet
    /// keeps it.
    fn insert_slots(
        slots: &mut BTreeMap<FacetSlotKey, FacetSlot>,
        source: &SlotSource,
        operations: &[Operator],
        rhs: Option<&Path>,
        conversion: Option<&Path>,
        span: Span,
    ) -> syn::Result<()> {
        let identity = rhs.map_or_else(
            || "Self".to_string(),
            |rhs| rhs.to_token_stream().to_string(),
        );

        for operator in operations
            .iter()
            .copied()
            .filter(|operator| source.flags.has(IntegralFlag::math(*operator)))
        {
            let key = FacetSlotKey {
                operator,
                rhs: identity.clone(),
            };
            let slot = FacetSlot {
                rhs: rhs.cloned(),
                conversion: conversion.cloned(),
                arithmetic: source.arithmetic,
                ty_out: source.ty_out.clone(),
                owner: source.owner,
                span,
            };

            if matches!(source.owner, FacetOwner::Explicit(_)) {
                Self::insert_explicit_slot(slots, key, slot)?;
            }
            else {
                slots.entry(key).or_insert(slot);
            }
        }

        return Ok(());
    }

    /// A facet level's slots: each friend's capabilities in
    /// `FACET_OPERATORS` order, then `self_math` and `self_bit` at their
    /// spans.
    fn insert_level_slots(
        slots: &mut BTreeMap<FacetSlotKey, FacetSlot>,
        source: &SlotSource,
        friends: &BTreeSet<Friend>,
        self_math: Option<Span>,
        self_bit: Option<Span>,
    ) -> syn::Result<()> {
        for friend in friends {
            let span = friend.ty.path().span();
            for (capability, operations) in FACET_OPERATORS {
                if friend.has(capability) {
                    Self::insert_slots(
                        slots,
                        source,
                        operations(),
                        Some(friend.ty.path()),
                        friend.conv.as_ref(),
                        span,
                    )?;
                }
            }
        }

        let conversion: Path = parse_quote! { Self::raw };
        // `FACET_OPERATORS` lists `Math` first.
        let self_slots = [
            self_math,
            self_bit,
        ];
        for ((_, operations), span) in
            FACET_OPERATORS.into_iter().zip(self_slots)
        {
            if let Some(span) = span {
                Self::insert_slots(
                    slots,
                    source,
                    operations(),
                    None,
                    Some(&conversion),
                    span,
                )?;
            }
        }

        return Ok(());
    }

    fn resolve_facets(&mut self) -> syn::Result<()> {
        self.facet_slots.clear();
        if !self.facets_present() {
            return Ok(());
        }

        let default = self
            .default_facet
            .as_ref()
            .expect("facets require a resolved default facet");
        let mut slots = BTreeMap::new();

        for (index, facet) in self.facets.iter().enumerate() {
            Self::validate_facet(facet)?;

            let flags = Self::resolved_facet_flags(&default.flags, facet);
            let source = SlotSource {
                flags: &flags,
                arithmetic: facet
                    .profile
                    .arithmetic_or(default.profile.arithmetic()),
                ty_out: facet.ty_out.as_ref().unwrap_or(&default.ty_out),
                owner: FacetOwner::Explicit(index),
            };
            let self_slot = |flag| {
                return match facet.explicit_flag(flag) {
                    Some((true, span)) => Some(span),
                    _ => None,
                };
            };
            Self::insert_level_slots(
                &mut slots,
                &source,
                &facet.friends,
                self_slot(IntegralFlag::SelfMath),
                self_slot(IntegralFlag::SelfBit),
            )?;
        }

        let source = SlotSource {
            flags: &default.flags,
            arithmetic: default.profile.arithmetic(),
            ty_out: &default.ty_out,
            owner: FacetOwner::Default,
        };
        let self_slot = |flag| default.flags.has(flag).then(Span::call_site);
        Self::insert_level_slots(
            &mut slots,
            &source,
            &default.friends,
            self_slot(IntegralFlag::SelfMath),
            self_slot(IntegralFlag::SelfBit),
        )?;

        self.facet_slots = slots;
        return Ok(());
    }

    pub(crate) fn explicit_flag(
        &self,
        flag: IntegralFlag,
    ) -> Option<(bool, Span)> {
        return self
            .root
            .flag(flag)
            .or_else(|| self.profile.explicit_flag(flag));
    }

    /// The span of an explicit setting of `flag`, else the call site.
    pub(crate) fn explicit_span(
        &self,
        flag: IntegralFlag,
    ) -> Span {
        return self
            .explicit_flag(flag)
            .map_or_else(Span::call_site, |(_, span)| span);
    }

    /// The first of `flags` explicitly turned on.
    fn first_explicit_on(
        &self,
        flags: &[IntegralFlag],
    ) -> Option<(IntegralFlag, Span)> {
        return flags.iter().find_map(|&flag| {
            return match self.explicit_flag(flag) {
                Some((true, span)) => Some((flag, span)),
                _ => None,
            };
        });
    }

    /// Turns `flags` off; an explicit request for one fails with `message`.
    fn deny_flags(
        &mut self,
        flags: &[IntegralFlag],
        message: impl Fn(IntegralFlag) -> String,
    ) -> syn::Result<()> {
        if let Some((flag, span)) = self.first_explicit_on(flags) {
            return span.fail(message(flag));
        }
        self.flags.clear(flags);
        return Ok(());
    }

    /// Kind-dependent configuration that does not depend on flag resolution.
    fn check_kind_settings(&self) -> syn::Result<()> {
        match self.kind {
            Kind::Int => {
                if let Some(span) = self.math_span {
                    return span.fail(
                        "math selects the float math provider; integer domain \
                        functions are always available",
                    );
                }
                if let Some((flag, span)) =
                    self.first_explicit_on(IntegralFlag::FLOAT_ONLY_FLAGS)
                {
                    let name = flag.name();
                    return span.fail(format!("{name} requires a float type"));
                }
            }
            Kind::Float => {
                let float_profile = |it: &Profile| {
                    return matches!(it, Profile::Int(_) | Profile::Point(_));
                };
                if !self.profile.iter().all(float_profile)
                    || !self
                        .facets
                        .iter()
                        .all(|facet| facet.profile.iter().all(float_profile))
                {
                    return Span::call_site().fail(
                        "float types support only the int and point profiles",
                    );
                }
                if let Some((true, span)) =
                    self.explicit_flag(IntegralFlag::NonZero)
                {
                    return span.fail(
                        "non_zero requires an integer type; exclude zero with \
                        domain = !{0.0}",
                    );
                }
                if let Some((flag, span)) =
                    self.first_explicit_on(IntegralFlag::INT_ONLY_FLAGS)
                {
                    let name = flag.name();
                    return span
                        .fail(format!("{name} requires an integer type"));
                }
                for facet in &self.facets {
                    FLOAT.reject(&facet.friends)?;
                }
            }
        }
        return Ok(());
    }

    /// Float kind defaults, applied after the struct defaults and before
    /// profiles. Totality flags start on and are settled after the user's
    /// flags (`apply_float_totality`).
    fn apply_float_defaults(&mut self) {
        self.flags.clear(IntegralFlag::INT_ONLY_FLAGS);
        for &flag in IntegralFlag::FLOAT_DEFAULT_ON {
            self.flags.set(flag, true);
        }
        self.flags
            .set(IntegralFlag::FloatMath, self.math != MathProvider::Core);
    }

    /// `Eq`, `Ord`, `Hash`, and `_cmp` need a domain that rejects NaN. They
    /// default off otherwise, and asking for them is an error.
    fn apply_float_totality(&mut self) -> syn::Result<()> {
        if !self.validation.admits_nan() {
            return Ok(());
        }
        let hint = match self.validation.domain.as_ref() {
            Some(_) => "it admits NaN; write (domain) & !is_nan",
            None => "add a domain that rejects NaN, such as !is_nan",
        };
        return self.deny_flags(IntegralFlag::TOTALITY_FLAGS, |flag| {
            let name = flag.name();
            return format!("{name} requires a NaN-free float type; {hint}");
        });
    }

    /// Settles what depends on the value type: resolves the domain, then
    /// `Neg`'s, `proven`'s, and `AsRef`'s defaults.
    pub(crate) fn settle(
        &mut self,
        raw: Num,
    ) -> syn::Result<()> {
        self.validation.resolve(raw, self.math)?;
        self.settle_neg(raw)?;
        self.settle_proven(raw)?;
        self.settle_as_ref()?;
        return Ok(());
    }

    /// `NonZero` storage cannot lend `&raw` without `unsafe`, so `AsRef`
    /// defaults off there, and asking for it is an error.
    fn settle_as_ref(&mut self) -> syn::Result<()> {
        if !self.requires_non_zero_storage() {
            return Ok(());
        }
        return self.deny_flags(&[IntegralFlag::AsRef], |flag| {
            let name = flag.name();
            return format!(
                "{name} cannot borrow the raw value of NonZero storage; \
                use `without = non_zero` to store the primitive"
            );
        });
    }

    /// Closure proofs (`proven`) are on by default for 8-bit integers only;
    /// wider integers opt in with `with = proven`, which `prove = [..]`
    /// implies.
    fn settle_proven(
        &mut self,
        raw: Num,
    ) -> syn::Result<()> {
        let Num::Int(int) = raw
        else {
            return Ok(());
        };
        let proven = match self.explicit_flag(IntegralFlag::Proven) {
            Some((false, span)) if !self.prove.is_empty() => {
                return span.fail(
                    "`prove` needs closure proofs; remove `without = proven`",
                );
            }
            Some((value, _)) => value,
            None => !self.prove.is_empty() || matches!(int, N::U008 | N::I008),
        };
        self.flags.set(IntegralFlag::Proven, proven);
        return Ok(());
    }

    /// `Neg` exists for signed integers and floats. Unsigned types drop the
    /// default, and asking for it is an error.
    pub(crate) fn settle_neg(
        &mut self,
        raw: Num,
    ) -> syn::Result<()> {
        if !matches!(raw, Num::Int(it) if !it.is_signed()) {
            return Ok(());
        }
        return self.deny_flags(IntegralFlag::NEG, |flag| {
            let name = flag.name();
            return format!("{name} requires a signed integer or float type");
        });
    }

    /// A visible value field lets code outside the type store NaN, which
    /// would break `Eq`, `Ord`, and `Hash`; they default off, and asking for
    /// them is an error.
    pub(crate) fn require_private_totality(&mut self) -> syn::Result<()> {
        return self.deny_flags(IntegralFlag::TOTALITY_FLAGS, |flag| {
            let name = flag.name();
            return format!(
                "{name} on a float type requires a private value field"
            );
        });
    }

    pub(crate) fn resolve(&mut self) -> syn::Result<()> {
        self.check_kind_settings()?;
        self.resolve_non_zero();
        self.resolve_layers();
        self.settle_flags()?;
        self.resolve_facet_layer()?;
        return Ok(());
    }

    /// The `non_zero` storage policy, which validation also needs.
    fn resolve_non_zero(&mut self) {
        self.non_zero = match self.explicit_flag(IntegralFlag::NonZero) {
            Some((true, _)) => NonZeroPolicy::Require,
            Some((false, _)) => NonZeroPolicy::Suppress,
            None => NonZeroPolicy::Infer,
        };
        self.validation
            .require_non_zero(self.non_zero == NonZeroPolicy::Require);
    }

    /// The flag layers in order: defaults, kind defaults, each profile
    /// (preset, then its settings), then the root settings.
    fn resolve_layers(&mut self) {
        self.flags = IntegralFlags::default();
        match self.kind {
            // Signedness is known once the value field is: `settle_neg`.
            Kind::Int => {
                for &flag in IntegralFlag::NEG {
                    self.flags.set(flag, true);
                }
            }
            Kind::Float => self.apply_float_defaults(),
        }
        self.konst_flags = KonstFlags::default();
        self.removals = Removals::default();
        for profile in self.profile.iter() {
            profile.apply(&mut self.flags);
            profile.settings().layer(
                &mut self.flags,
                &mut self.konst_flags,
                &mut self.removals,
            );
        }
        self.root.layer(
            &mut self.flags,
            &mut self.konst_flags,
            &mut self.removals,
        );
    }

    /// What the layered flags rule out: float totality, linear ordering on
    /// circular profiles, `range`, and assignment under `produce`.
    fn settle_flags(&mut self) -> syn::Result<()> {
        if self.kind == Kind::Float {
            self.apply_float_totality()?;
        }

        if self.profile.is_circular() {
            self.deny_flags(IntegralFlag::ORDERING, |_| {
                return "circular profiles do not support linear ordering"
                    .to_string();
            })?;
        }

        if self.flags.has(IntegralFlag::Range) {
            let span = self.explicit_span(IntegralFlag::Range);
            if self.validation.has_validation() {
                return span
                    .fail("range requires an unvalidated integral type");
            }
            if self.profile.is_circular() {
                return span
                    .fail("range is incompatible with circular profiles");
            }
        }

        if !self.ty_out.is_self() {
            self.deny_flags(IntegralFlag::ASSIGN_FLAGS, |_| {
                return "produce cannot generate assignment operators"
                    .to_string();
            })?;
        }

        return Ok(());
    }

    /// The default facet's snapshot of the resolved root, the facet slots,
    /// then the storage the profiles require. Validation turns `auto_of_raw`
    /// off for the root only, after the snapshot: facets inherit it as set.
    fn resolve_facet_layer(&mut self) -> syn::Result<()> {
        self.default_facet = self.facets_present().then(|| DefaultFacetCfg {
            profile: self.profile.clone(),
            friends: self.friends.clone(),
            flags: self.flags,
            ty_out: self.ty_out.clone(),
        });
        self.resolve_facets()?;
        if self.validation.has_validation() {
            self.flags.set(IntegralFlag::AutoOfRaw, false);
        }
        self.wrapping_storage = self.profile.requires_wrapping_storage()
            || self.facets.iter().any(FacetCfg::requires_wrapping_storage);
        return Ok(());
    }

    pub(crate) fn konst_for(
        &self,
        flags: &[Konsts],
    ) -> bool {
        return self
            .konst_flags
            .konst_for(flags, || self.konst.enables(flags));
    }

    /// Hands constness to the outer family: each `konst_*` flag takes the
    /// outer value, and a disagreeing nested explicit setting fails at its
    /// span. Call after `resolve`, which clears the konst flags.
    pub(crate) fn embed_konst(
        &mut self,
        konst: Konst,
        outer: &KonstFlags,
    ) -> syn::Result<()> {
        for &flag in Konsts::INTEGRAL {
            let required = outer.konst_for(&[flag], || konst.enables(&[flag]));
            if let Some((value, span)) = self.konst_flags.get(flag)
                && value != required
            {
                return span.fail(
                    "this value is not compatible with own konst config of bitflag",
                );
            }
            self.konst_flags.insert(flag, required, Span::call_site());
        }
        self.konst = konst;
        return Ok(());
    }

    pub(crate) fn konst_friendship(&self) -> bool {
        if self.konst_for(&[Konsts::Friendship]) {
            return true;
        }

        return Konsts::INTEGRAL
            .iter()
            .filter(|flag| **flag != Konsts::Friendship && !flag.is_stable())
            .any(|flag| self.konst_for(&[*flag]));
    }

    /// Whether the private helpers that run validation (`_or_panic` and the
    /// arithmetic and bitwise helpers) are `const fn`.
    ///
    /// Constness follows the configuration only, never what validation
    /// calls (D16): the helpers are const when a const item consumes them.
    /// Every consumer (operators, assignments, `Step`, `of`/`make`, friend
    /// impls) uses a non-stable konst flag, which is exactly what
    /// `konst_friendship` tests. Without validation the helpers only
    /// construct and stay const.
    pub(crate) fn helper_konst(&self) -> bool {
        return !self.validation.has_validation()
            || self.const_validation_required
            || self.konst_friendship();
    }

    /// Whether the literal macro (`zz_lit`) is generated. It evaluates
    /// `try_make` in a `const`, so it needs a const `try_make`; by default it
    /// is generated exactly when there is something to check at compile time.
    pub(crate) fn lit_macro(&self) -> syn::Result<bool> {
        let const_try_make = self.flags.has(IntegralFlag::FnTryMake)
            && (!self.validation.has_validation()
                || self.konst_for(&[Konsts::TryMake]));
        let enabled = match self.explicit_flag(IntegralFlag::LitMacro) {
            Some((true, span)) if !const_try_make => {
                return span.fail(
                    "`lit_macro` needs a const `try_make`; remove \
                    `konst = none` or add `without = lit_macro`",
                );
            }
            Some((enabled, _)) => enabled,
            None => const_try_make && self.validation.has_validation(),
        };
        if !enabled {
            if let Some(name) = &self.lit_name {
                return name.fail("`lit_name` needs the literal macro");
            }
            if let Some(path) = &self.lit_path {
                return path.fail("`lit_path` needs the literal macro");
            }
        }
        return Ok(enabled);
    }

    /// The span of an explicit request for the literal macro: `with =
    /// lit_macro`, `lit_name`, or `lit_path`.
    pub(crate) fn lit_requested(&self) -> Option<Span> {
        if let Some((true, span)) = self.explicit_flag(IntegralFlag::LitMacro) {
            return Some(span);
        }
        return self.lit_name.as_ref().map(Ident::span).or_else(|| {
            self.lit_path.as_ref().map(syn::spanned::Spanned::span)
        });
    }

    /// Root friends: `Math` also grants `Make`, and `Bit` for integers;
    /// `Within` also grants `Make`.
    pub(crate) fn parse_friends(
        input: ParseStream,
        kind: Kind,
    ) -> syn::Result<BTreeSet<Friend>> {
        let friends = match kind {
            Kind::Int => INTEGRAL.parse_list(input)?,
            Kind::Float => FLOAT.parse_list(input)?,
        };
        check_within(&friends)?;
        return Ok(friends);
    }

    pub(crate) fn add_friend(
        &mut self,
        ty: &Ident,
        capabilities: impl IntoIterator<Item = Ident>,
        conv: Path,
    ) {
        upsert_friend(&mut self.friends, Friend::from(ty), |friend| {
            friend.capabilities.extend(capabilities);
            friend.conv = Some(conv);
        });
    }

    fn parse_attrs(
        &mut self,
        input: ParseStream,
        site: CfgSite,
    ) -> syn::Result<()> {
        let outer = site != CfgSite::Nested;
        let fixed_profile = site == CfgSite::FixedProfile;
        return zz::parse_inner(input, |attr, rest| {
            if self.validation.parse_attr(attr, rest)? {
                return Ok(true);
            }

            match attr {
                Key::Konst if outer => self.konst = rest.parse()?,
                Key::Konst => {
                    return Span::call_site()
                        .fail("constness is specified in multiple places");
                }
                Key::Error => zz::parse_unit_error(rest)?,
                Key::With => self.root.parse(rest, true, &GRAMMAR)?,
                Key::Without => self.root.parse(rest, false, &GRAMMAR)?,
                Key::Friend => {
                    self.friends = Self::parse_friends(rest, self.kind)?;
                }
                Key::Math => {
                    self.math_span = Some(rest.span());
                    self.math = rest.parse()?;
                }
                Key::Tree if outer => {
                    self.tree = Some(marker_tree::Cfg::parse_attribute(rest)?);
                }
                Key::Tree => {
                    return Span::call_site().fail(
                        "bitflag integral configuration does not support tree",
                    );
                }
                Key::HideIn if outer => self.hide_in = rest.parse()?,
                Key::HideIn => {
                    return Span::call_site().fail(
                        "hide_in belongs on the bitflag root, not in integral = [..]",
                    );
                }
                Key::Facet => {
                    self.facets = zz::one_or_list::<FacetCfg>(rest)?.collect();
                    self.facets_present = true;
                }
                Key::Precedes if fixed_profile => {
                    if !self.profile.single_mut().parse_attr(attr, rest)? {
                        return Ok(false);
                    }
                }
                Key::Profile if fixed_profile => {
                    return Span::call_site()
                        .fail("profile is selected by this macro");
                }
                Key::GetRaw => self.get_raw = Some(rest.parse()?),
                Key::LitName => self.lit_name = Some(rest.parse()?),
                Key::LitPath => self.lit_path = Some(rest.parse()?),
                Key::Prove => {
                    self.prove = zz::one_or_list::<ProveItem>(rest)?.collect();
                }
                Key::Produce => self.ty_out = rest.parse()?,
                Key::Profile => self.profile = rest.parse()?,
                _ => return Ok(false),
            };

            return Ok(true);
        });
    }

    #[cfg(test)]
    pub(crate) fn parse_with_profile(
        input: ParseStream,
        profile: ProfileKind,
    ) -> syn::Result<Self> {
        return Self::parse_kind_with_profile(input, profile, Kind::Int);
    }

    /// `#[typekin::float]` configuration: the kind is fixed before any
    /// attribute is parsed, so friend expansion and flag defaults know it.
    pub(crate) fn parse_float(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self {
            kind: Kind::Float,
            ..Default::default()
        };
        this.parse_attrs(input, CfgSite::Root)?;
        this.resolve()?;
        return Ok(this);
    }

    pub(crate) fn parse_kind_with_profile(
        input: ParseStream,
        profile: ProfileKind,
        kind: Kind,
    ) -> syn::Result<Self> {
        let mut this = Self {
            profile: Profiles::one(Profile::from_name(profile)),
            kind,
            ..Default::default()
        };
        this.parse_attrs(input, CfgSite::FixedProfile)?;
        this.resolve()?;
        return Ok(this);
    }

    pub(crate) fn parse_with_konst_unresolved(
        input: ParseStream
    ) -> syn::Result<Self> {
        let mut this = Self::without_konst();
        this.parse_attrs(input, CfgSite::Nested)?;
        return Ok(this);
    }

    pub(crate) fn without_konst() -> Self {
        return Self {
            konst: Konst::None,
            ..Default::default()
        };
    }
}

impl Debug for Cfg {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return f
            .debug_struct("IntegralCfg")
            .field("konst", &self.konst)
            .field("get_raw", &self.get_raw.to_token_stream().to_string())
            .field("validation", &self.validation)
            .field("friends", &self.friends)
            .field("non_zero", &self.non_zero)
            .field("ty_out", &!self.ty_out.is_self())
            .field("profile", &self.profile.arithmetic())
            .field("tree", &self.tree.is_some())
            .field("facet_slots", &self.facet_slots)
            .field("flags", &self.flags)
            .finish_non_exhaustive();
    }
}

impl HideCfg for Cfg {
    fn hide_in(&self) -> &HideIn {
        return &self.hide_in;
    }
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();
        this.parse_attrs(input, CfgSite::Root)?;
        this.resolve()?;
        return Ok(this);
    }
}

// =============================================================================

impl IntegralFlag {
    pub(crate) const RESTRICTED_MATH_FLAGS: &'static [Self] = &[
        Self::SelfFriendMathOps,
        Self::SelfFriendMathBit,
        Self::SelfFriendRel,
        Self::SelfBit,
        Self::BitAccess,
        Self::CoreInt,
        Self::MathAnd,
        Self::MathDiv,
        Self::MathMul,
        Self::MathNot,
        Self::MathOr,
        Self::MathRem,
        Self::MathShl,
        Self::MathShr,
        Self::MathXor,
        Self::AssignAnd,
        Self::AssignDiv,
        Self::AssignMul,
        Self::AssignOr,
        Self::AssignRem,
        Self::AssignShl,
        Self::AssignShr,
    ];

    /// The assignment operator flags.
    pub(crate) const ASSIGN_FLAGS: &'static [Self] = &[
        Self::AssignAdd,
        Self::AssignAnd,
        Self::AssignDiv,
        Self::AssignMul,
        Self::AssignOr,
        Self::AssignRem,
        Self::AssignShl,
        Self::AssignShr,
        Self::AssignSub,
    ];

    /// Integer-only flags: off for floats, and an error when enabled there.
    pub(crate) const INT_ONLY_FLAGS: &'static [Self] = &[
        Self::BitAccess,
        Self::CoreInt,
        Self::FmtBinary,
        Self::FmtOctal,
        Self::FmtHexLower,
        Self::FmtHexUpper,
        Self::Range,
        Self::NonZero,
        Self::Proven,
        Self::WithinSource,
        Self::MathAnd,
        Self::MathOr,
        Self::MathXor,
        Self::MathNot,
        Self::MathShl,
        Self::MathShr,
        Self::AssignAnd,
        Self::AssignOr,
        Self::AssignShl,
        Self::AssignShr,
        Self::FnMathAnd,
        Self::FnMathOr,
        Self::FnMathXor,
        Self::FnMathNot,
        Self::FnMathShl,
        Self::FnMathShr,
        Self::SelfBit,
        Self::SelfFriendMathBit,
        Self::FriendzoneFriendMathBit,
    ];

    /// Float-only flags: off for integers, and an error when enabled there.
    pub(crate) const FLOAT_ONLY_FLAGS: &'static [Self] = &[
        Self::CoreFloat,
        Self::FloatMath,
        Self::Sum,
        Self::Product,
    ];

    /// Totality needs a NaN-free float type.
    pub(crate) const TOTALITY_FLAGS: &'static [Self] = &[
        Self::Eq,
        Self::Ord,
        Self::Hash,
        Self::FnOpCmp,
    ];

    /// `Neg`: on by default, settled by signedness.
    pub(crate) const NEG: &'static [Self] = &[
        Self::MathNeg,
        Self::FnMathNeg,
    ];

    /// Linear ordering, which circular profiles lack.
    pub(crate) const ORDERING: &'static [Self] = &[
        Self::Ord,
        Self::PartialOrd,
    ];

    /// The float kind's defaults beyond `DEFAULT_ON`; `float_math` follows
    /// the math provider.
    pub(crate) const FLOAT_DEFAULT_ON: &'static [Self] = &[
        Self::MathNeg,
        Self::FnMathNeg,
        Self::CoreFloat,
        Self::FmtExpLower,
        Self::FmtExpUpper,
        Self::Sum,
        Self::Product,
    ];

    /// What the `point` preset turns off beyond the restricted math: points
    /// are not added, negated, summed, or multiplied, and their float
    /// helpers follow the same restricted surface.
    pub(crate) const POINT_OFF: &'static [Self] = &[
        Self::MathAdd,
        Self::AssignAdd,
        Self::MathNeg,
        Self::FnMathNeg,
        Self::CoreFloat,
        Self::FloatMath,
        Self::Sum,
        Self::Product,
    ];

    /// What the `nominal` preset turns off beyond the restricted math and
    /// linear ordering: a nominal value is only told apart from others, so
    /// it has no operator left and none of the surface that reads it as a
    /// quantity (conversions, radix formatting, `AsRef`, `FromStr`, `map`,
    /// and the `Within` source constants). `raw()` stays.
    pub(crate) const NOMINAL_OFF: &'static [Self] = &[
        Self::MathAdd,
        Self::MathSub,
        Self::AssignAdd,
        Self::AssignSub,
        Self::MathNeg,
        Self::FnMathNeg,
        Self::FnOpCmp,
        Self::FmtBinary,
        Self::FmtOctal,
        Self::FmtHexLower,
        Self::FmtHexUpper,
        Self::AsRef,
        Self::FromStr,
        Self::Map,
        Self::Into,
        Self::TryInto,
        Self::FnConvInto,
        Self::FnConvTryIntoLossy,
        Self::FnConvTryIntoLossless,
        Self::WithinSource,
    ];

    /// The flags a facet may set: its operand slots.
    pub(crate) const FACET_OPERANDS: &'static [Self] = &[
        Self::MathAdd,
        Self::MathSub,
        Self::MathMul,
        Self::MathDiv,
        Self::MathRem,
        Self::MathAnd,
        Self::MathOr,
        Self::MathXor,
        Self::SelfMath,
        Self::SelfBit,
    ];

    /// The flag that generates `op`'s operator.
    pub(crate) fn math(op: Operator) -> Self {
        return match op {
            Operator::Add => Self::MathAdd,
            Operator::Sub => Self::MathSub,
            Operator::Mul => Self::MathMul,
            Operator::Div => Self::MathDiv,
            Operator::Rem => Self::MathRem,
            Operator::BitAnd => Self::MathAnd,
            Operator::BitOr => Self::MathOr,
            Operator::BitXor => Self::MathXor,
        };
    }

    /// The flag that generates `op`'s assignment operator; `BitXor` has
    /// none.
    pub(crate) fn assign(op: Operator) -> Option<Self> {
        return match op {
            Operator::Add => Some(Self::AssignAdd),
            Operator::Sub => Some(Self::AssignSub),
            Operator::Mul => Some(Self::AssignMul),
            Operator::Div => Some(Self::AssignDiv),
            Operator::Rem => Some(Self::AssignRem),
            Operator::BitAnd => Some(Self::AssignAnd),
            Operator::BitOr => Some(Self::AssignOr),
            Operator::BitXor => None,
        };
    }
}

/// A name integral and float generate.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum IntegralName {
    SealTrait,
    MakeTrait,
    MathTrait,
    BitTrait,
    RelTrait,
    UnsafeTrait,
    /// `{Op}Rhs`: a facet's right-hand-side trait.
    RhsTrait(Operator),
    /// `conv_{snake}`: the seal's conversion.
    Conversion,
    /// `make`: the value-type constructor `auto_of_raw` generates.
    MakeFn,
    /// `of`: the friend constructor.
    OfFn,
    /// `_or_panic`: the validating, panicking constructor.
    OrPanic,
    /// `_blind_trust`: the constructor that skips validation.
    BlindTrust,
    /// `into_{type}`: a lossless conversion.
    Into(Num),
    /// `try_into_{type}`: a checked conversion.
    TryInto(Num),
    /// `byte{i}`, `word{i}`, `dword{i}`, `qword{i}`: the `index`th part of
    /// type `part` (`u8` … `u64`).
    BitPart(N, usize),
    /// `lo{bits}`: the low half, of type `half`.
    LowHalf(N),
    /// `hi{bits}`: the high half, of type `half`.
    HighHalf(N),
    /// A forward, keyed as `without = fn_*` is.
    Fn(FnName),
    /// `_within`: the constructor for proven `Within` sources.
    WithinFn,
    /// `__TYPEKIN_WITHIN_PROOF_{n}`: the `n`th `Within` friend's proof.
    WithinProof(usize),
    /// `__TYPEKIN_WITHIN_V1_RAW`: a source's signedness and bit width.
    WithinRaw,
    /// `__TYPEKIN_WITHIN_V1_FORM`: a source's form, if it has one.
    WithinForm,
    /// `__TYPEKIN_WITHIN_V1_WHY`: why a source has no form.
    WithinWhy,
    /// `__TYPEKIN_WITHIN_V1_TABLE`: an 8-bit source's validation table.
    WithinTable,
    /// `__typekin_within_v1_raw`: a source's raw value.
    WithinRawFn,
}

impl NameKey for IntegralName {
    type Owner = Ident;

    fn default_name(
        self,
        owner: &Ident,
    ) -> String {
        return match self {
            Self::SealTrait => "Seal".to_string(),
            Self::MakeTrait => Capability::Make.name().to_string(),
            Self::MathTrait => Capability::Math.name().to_string(),
            Self::BitTrait => Capability::Bit.name().to_string(),
            Self::RelTrait => Capability::Rel.name().to_string(),
            Self::UnsafeTrait => Capability::Unsafe.name().to_string(),
            Self::RhsTrait(operator) => format!("{}Rhs", operator.name()),
            Self::Conversion => {
                format!("conv_{}", snake_case_of(&owner.to_string()))
            }
            Self::MakeFn => "make".to_string(),
            Self::OfFn => "of".to_string(),
            Self::OrPanic => "_or_panic".to_string(),
            Self::BlindTrust => "_blind_trust".to_string(),
            Self::Into(target) => format!("into_{}", target.rust_name()),
            Self::TryInto(target) => format!("try_into_{}", target.rust_name()),
            Self::BitPart(part, index) => {
                let prefix = match part {
                    N::U008 => "byte",
                    N::U016 => "word",
                    N::U032 => "dword",
                    N::U064 => "qword",
                    _ => unreachable!("bit access parts are u8 to u64"),
                };
                format!("{prefix}{index}")
            }
            Self::LowHalf(half) => {
                format!("lo{}", half.bits_for_pointer_width(64))
            }
            Self::HighHalf(half) => {
                format!("hi{}", half.bits_for_pointer_width(64))
            }
            Self::Fn(function) => function.name().to_string(),
            Self::WithinFn => "_within".to_string(),
            Self::WithinProof(index) => {
                format!("__TYPEKIN_WITHIN_PROOF_{index}")
            }
            Self::WithinRaw => "__TYPEKIN_WITHIN_V1_RAW".to_string(),
            Self::WithinForm => "__TYPEKIN_WITHIN_V1_FORM".to_string(),
            Self::WithinWhy => "__TYPEKIN_WITHIN_V1_WHY".to_string(),
            Self::WithinTable => "__TYPEKIN_WITHIN_V1_TABLE".to_string(),
            Self::WithinRawFn => "__typekin_within_v1_raw".to_string(),
        };
    }

    fn avoids_generics(self) -> bool {
        return matches!(
            self,
            Self::SealTrait
                | Self::MakeTrait
                | Self::MathTrait
                | Self::BitTrait
                | Self::RelTrait
                | Self::UnsafeTrait
                | Self::RhsTrait(_)
        );
    }
}

impl From<FnName> for IntegralName {
    fn from(function: FnName) -> Self {
        return Self::Fn(function);
    }
}

pub(crate) type IntegralFlags = Flags<IntegralFlag>;

/// An integral `with`/`without` flag.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum IntegralFlag {
    Range,
    AsRef,
    AutoOfRaw,
    Assertions,
    BitAccess,
    Clone,
    Copy,
    Debug,
    Display,
    Hash,
    Eq,
    FmtBinary,
    FmtHexLower,
    FmtHexUpper,
    FmtOctal,
    FromStr,
    CoreInt,
    Into,
    Map,
    Ord,
    PartialEq,
    PartialOrd,
    Repr,
    Serde,
    TryInto,
    WithConstMapFn,
    WithBlindTrust,
    LitMacro,
    NonZero,
    Proven,
    /// Publishes the hidden constants `Within` targets read.
    WithinSource,
    // Float kind: on by default for floats (see `Kind::Float`), rejected
    // or opt-in for integers.
    MathNeg,
    FnMathNeg,
    CoreFloat,
    FloatMath,
    FmtExpLower,
    FmtExpUpper,
    Sum,
    Product,
    AssignAdd,
    AssignAnd,
    AssignDiv,
    AssignMul,
    AssignOr,
    AssignRem,
    AssignShl,
    AssignShr,
    AssignSub,
    MathAdd,
    MathAnd,
    MathDiv,
    MathMul,
    MathNot,
    MathOr,
    MathRem,
    MathShl,
    MathShr,
    MathSub,
    MathXor,
    SelfFriendMake,
    SelfFriendMathBit,
    SelfFriendMathOps,
    SelfFriendRel,
    FriendSeal,
    FriendzoneFriendMake,
    FriendzoneFriendMathBit,
    FriendzoneFriendMathOps,
    FriendzoneFriendRel,
    FriendzoneSeal,
    SelfMath,
    SelfBit,
    FnConvInto,
    FnConvOf,
    FnConvRaw,
    FnConvTryIntoLossy,
    FnConvTryIntoLossless,
    FnOrPanic,
    FnTryMake,
    FnMathAdd,
    FnMathAnd,
    FnMathDiv,
    FnMathMul,
    FnMathNot,
    FnMathOr,
    FnMathRem,
    FnMathShl,
    FnMathShr,
    FnMathSub,
    FnMathXor,
    FnOpCmp,
    FnOpEq,
}

impl Flag for IntegralFlag {
    const ITEMS: &'static [Self] = &[
        Self::Range,
        Self::AsRef,
        Self::AutoOfRaw,
        Self::Assertions,
        Self::BitAccess,
        Self::Clone,
        Self::Copy,
        Self::Debug,
        Self::Display,
        Self::Hash,
        Self::Eq,
        Self::FmtBinary,
        Self::FmtHexLower,
        Self::FmtHexUpper,
        Self::FmtOctal,
        Self::FromStr,
        Self::CoreInt,
        Self::Into,
        Self::Map,
        Self::Ord,
        Self::PartialEq,
        Self::PartialOrd,
        Self::Repr,
        Self::Serde,
        Self::TryInto,
        Self::WithConstMapFn,
        Self::WithBlindTrust,
        Self::LitMacro,
        Self::NonZero,
        Self::Proven,
        Self::WithinSource,
        Self::MathNeg,
        Self::FnMathNeg,
        Self::CoreFloat,
        Self::FloatMath,
        Self::FmtExpLower,
        Self::FmtExpUpper,
        Self::Sum,
        Self::Product,
        Self::AssignAdd,
        Self::AssignAnd,
        Self::AssignDiv,
        Self::AssignMul,
        Self::AssignOr,
        Self::AssignRem,
        Self::AssignShl,
        Self::AssignShr,
        Self::AssignSub,
        Self::MathAdd,
        Self::MathAnd,
        Self::MathDiv,
        Self::MathMul,
        Self::MathNot,
        Self::MathOr,
        Self::MathRem,
        Self::MathShl,
        Self::MathShr,
        Self::MathSub,
        Self::MathXor,
        Self::SelfFriendMake,
        Self::SelfFriendMathBit,
        Self::SelfFriendMathOps,
        Self::SelfFriendRel,
        Self::FriendSeal,
        Self::FriendzoneFriendMake,
        Self::FriendzoneFriendMathBit,
        Self::FriendzoneFriendMathOps,
        Self::FriendzoneFriendRel,
        Self::FriendzoneSeal,
        Self::SelfMath,
        Self::SelfBit,
        Self::FnConvInto,
        Self::FnConvOf,
        Self::FnConvRaw,
        Self::FnConvTryIntoLossy,
        Self::FnConvTryIntoLossless,
        Self::FnOrPanic,
        Self::FnTryMake,
        Self::FnMathAdd,
        Self::FnMathAnd,
        Self::FnMathDiv,
        Self::FnMathMul,
        Self::FnMathNot,
        Self::FnMathOr,
        Self::FnMathRem,
        Self::FnMathShl,
        Self::FnMathShr,
        Self::FnMathSub,
        Self::FnMathXor,
        Self::FnOpCmp,
        Self::FnOpEq,
    ];
    const DEFAULT_ON: &'static [Self] = &[
        Self::AsRef,
        Self::AutoOfRaw,
        Self::Assertions,
        Self::BitAccess,
        Self::Clone,
        Self::Copy,
        Self::Debug,
        Self::Hash,
        Self::Eq,
        Self::FmtBinary,
        Self::FmtHexLower,
        Self::FmtHexUpper,
        Self::FmtOctal,
        Self::FromStr,
        Self::CoreInt,
        Self::Into,
        Self::Map,
        Self::Ord,
        Self::PartialEq,
        Self::PartialOrd,
        Self::Repr,
        Self::TryInto,
        Self::WithinSource,
        Self::AssignAdd,
        Self::AssignAnd,
        Self::AssignDiv,
        Self::AssignMul,
        Self::AssignOr,
        Self::AssignRem,
        Self::AssignShl,
        Self::AssignShr,
        Self::AssignSub,
        Self::MathAdd,
        Self::MathAnd,
        Self::MathDiv,
        Self::MathMul,
        Self::MathNot,
        Self::MathOr,
        Self::MathRem,
        Self::MathShl,
        Self::MathShr,
        Self::MathSub,
        Self::MathXor,
        Self::SelfFriendMake,
        Self::SelfFriendMathBit,
        Self::SelfFriendMathOps,
        Self::SelfFriendRel,
        Self::FriendSeal,
        Self::FriendzoneFriendMake,
        Self::FriendzoneFriendMathBit,
        Self::FriendzoneFriendMathOps,
        Self::FriendzoneFriendRel,
        Self::FriendzoneSeal,
        Self::SelfMath,
        Self::SelfBit,
        Self::FnConvInto,
        Self::FnConvOf,
        Self::FnConvRaw,
        Self::FnConvTryIntoLossy,
        Self::FnConvTryIntoLossless,
        Self::FnOrPanic,
        Self::FnTryMake,
        Self::FnMathAdd,
        Self::FnMathAnd,
        Self::FnMathDiv,
        Self::FnMathMul,
        Self::FnMathNot,
        Self::FnMathOr,
        Self::FnMathRem,
        Self::FnMathShl,
        Self::FnMathShr,
        Self::FnMathSub,
        Self::FnMathXor,
        Self::FnOpCmp,
        Self::FnOpEq,
    ];

    fn name(self) -> &'static str {
        return match self {
            Self::Range => "range",
            Self::AsRef => "as_ref_",
            Self::AutoOfRaw => "auto_of_raw",
            Self::Assertions => "assertions",
            Self::BitAccess => "bit_access",
            Self::Clone => "clone",
            Self::Copy => "copy",
            Self::Debug => "debug",
            Self::Display => "display",
            Self::Hash => "hash",
            Self::Eq => "eq",
            Self::FmtBinary => "fmt_binary",
            Self::FmtHexLower => "fmt_hex_lower",
            Self::FmtHexUpper => "fmt_hex_upper",
            Self::FmtOctal => "fmt_octal",
            Self::FromStr => "from_str",
            Self::CoreInt => "core_int",
            Self::Into => "into",
            Self::Map => "map",
            Self::Ord => "ord",
            Self::PartialEq => "partial_eq",
            Self::PartialOrd => "partial_ord",
            Self::Repr => "repr",
            Self::Serde => "serde",
            Self::TryInto => "try_into",
            Self::WithConstMapFn => "with_const_map_fn",
            Self::WithBlindTrust => "with_blind_trust",
            Self::LitMacro => "lit_macro",
            Self::NonZero => "non_zero",
            Self::Proven => "proven",
            Self::WithinSource => "within_source",
            Self::MathNeg => "math_neg",
            Self::FnMathNeg => "fn_math_neg",
            Self::CoreFloat => "core_float",
            Self::FloatMath => "float_math",
            Self::FmtExpLower => "fmt_exp_lower",
            Self::FmtExpUpper => "fmt_exp_upper",
            Self::Sum => "sum",
            Self::Product => "product",
            Self::AssignAdd => "assign_add",
            Self::AssignAnd => "assign_and",
            Self::AssignDiv => "assign_div",
            Self::AssignMul => "assign_mul",
            Self::AssignOr => "assign_or",
            Self::AssignRem => "assign_rem",
            Self::AssignShl => "assign_shl",
            Self::AssignShr => "assign_shr",
            Self::AssignSub => "assign_sub",
            Self::MathAdd => "math_add",
            Self::MathAnd => "math_and",
            Self::MathDiv => "math_div",
            Self::MathMul => "math_mul",
            Self::MathNot => "math_not",
            Self::MathOr => "math_or",
            Self::MathRem => "math_rem",
            Self::MathShl => "math_shl",
            Self::MathShr => "math_shr",
            Self::MathSub => "math_sub",
            Self::MathXor => "math_xor",
            Self::SelfFriendMake => "self_friend_make",
            Self::SelfFriendMathBit => "self_friend_math_bit",
            Self::SelfFriendMathOps => "self_friend_math_ops",
            Self::SelfFriendRel => "self_friend_rel",
            Self::FriendSeal => "friend_seal",
            Self::FriendzoneFriendMake => "friendzone_friend_make",
            Self::FriendzoneFriendMathBit => "friendzone_friend_math_bit",
            Self::FriendzoneFriendMathOps => "friendzone_friend_math_ops",
            Self::FriendzoneFriendRel => "friendzone_friend_rel",
            Self::FriendzoneSeal => "friendzone_seal",
            Self::SelfMath => "self_math",
            Self::SelfBit => "self_bit",
            Self::FnConvInto => "fn_conv_into",
            Self::FnConvOf => "fn_conv_of",
            Self::FnConvRaw => "fn_conv_raw",
            Self::FnConvTryIntoLossy => "fn_conv_try_into_lossy",
            Self::FnConvTryIntoLossless => "fn_conv_try_into_lossless",
            Self::FnOrPanic => "fn_or_panic",
            Self::FnTryMake => "fn_try_make",
            Self::FnMathAdd => "fn_math_add",
            Self::FnMathAnd => "fn_math_and",
            Self::FnMathDiv => "fn_math_div",
            Self::FnMathMul => "fn_math_mul",
            Self::FnMathNot => "fn_math_not",
            Self::FnMathOr => "fn_math_or",
            Self::FnMathRem => "fn_math_rem",
            Self::FnMathShl => "fn_math_shl",
            Self::FnMathShr => "fn_math_shr",
            Self::FnMathSub => "fn_math_sub",
            Self::FnMathXor => "fn_math_xor",
            Self::FnOpCmp => "fn_op_cmp",
            Self::FnOpEq => "fn_op_eq",
        };
    }

    fn index(self) -> u32 {
        return self as u32;
    }
}

// Every variant is in `ITEMS`, at its own index.
const _: () = {
    let items = <IntegralFlag as Flag>::ITEMS;
    assert!(items.len() == IntegralFlag::FnOpEq as usize + 1);
    let mut index = 0;
    while index < items.len() {
        assert!(items[index] as usize == index);
        index += 1;
    }
};
