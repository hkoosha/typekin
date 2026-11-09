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
use syn::Path;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::spanned::Spanned as _;

use crate::integral_proof::ProveItem;
use crate::zz_conv::SelfConverter;
use crate::zz_domain_float::MathProvider;
use crate::zz_flags::{
    Flag,
    FlagGrammar,
    FlagRow,
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
    Form,
    Key,
    Konst,
    KonstFamily,
    Konsts,
    Named,
    Removals,
    Site,
    ValidationCfg,
};
use crate::zz_names::NameKey;
use crate::zz_profile_integral::{
    Arithmetic,
    FacetCfg,
    Profile,
    ProfileKind,
    Profiles,
    RhsKind,
    Slot,
    Slots,
};
use crate::zz_runner::{
    MkErr,
    snake_case_of,
};
use crate::zz_values::{
    FnName,
    N,
    Num,
    OPS,
    Op,
    OpKind,
    OpSpec,
    Width,
};
use crate::{
    marker_tree,
    zz_lib as zz,
};

/// The `with`/`without` grammar of an integral configuration level.
pub(crate) const GRAMMAR: FlagGrammar = FlagGrammar {
    konsts: Some(KonstFamily::Integral),
    removals: &[Form::Regular],
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

/// `panic_at = anything | arithmetic | overflow | nothing`: what generated
/// code may still panic on. Below `anything`, an operation that could fail
/// returns `Result<T, Option<Raw>>`: `Some(raw)` for an invalid result,
/// `None` when there is no result (division by zero, checked overflow).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum PanicAt {
    /// Validation failures and arithmetic panic.
    #[default]
    Anything,
    /// Only Rust's arithmetic panics: division by zero and overflow.
    Arithmetic,
    /// Only Rust's `overflow-checks`, which release builds turn off.
    Overflow,
    /// Nothing panics.
    Nothing,
}

impl Named for PanicAt {
    const TABLE: &'static [(Self, &'static str)] = &[
        (Self::Anything, "anything"),
        (Self::Arithmetic, "arithmetic"),
        (Self::Overflow, "overflow"),
        (Self::Nothing, "nothing"),
    ];

    fn index(self) -> usize {
        return self as usize;
    }
}

// Every row of `TABLE` is at its variant's index.
const _: () = {
    let table = PanicAt::TABLE;
    let mut index = 0;
    while index < table.len() {
        assert!(table[index].0 as usize == index);
        index += 1;
    }
};

impl PanicAt {
    /// Whether operations that could fail return `Result` instead of
    /// panicking.
    pub(crate) fn fallible(self) -> bool {
        return self != Self::Anything;
    }

    /// Whether division and remainder check for a zero divisor and
    /// `MIN / -1`.
    pub(crate) fn checks_division(self) -> bool {
        return matches!(self, Self::Overflow | Self::Nothing);
    }

    /// Whether overflow is checked rather than left to `overflow-checks`.
    pub(crate) fn checks_overflow(self) -> bool {
        return self == Self::Nothing;
    }
}

/// What a form does with a body that may fail.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Translation {
    /// The body's value is the form's output: nothing can fail, or the
    /// form returns the `Result`.
    Pass,
    /// A failure panics with this message.
    Unwrap(&'static str),
}

/// How a form whose body `fails` (an invalid result, or none) turns it into
/// its output; `None` when no such form is generated. A failure panics at
/// `panic_at = anything`. Below it a form returns the `Result`, unless its
/// output is fixed (an assignment, `of`, `map`, `Step`): a function never
/// changes its signature with the level, so that form is left out.
pub(crate) fn translate(
    fails: bool,
    panic_at: PanicAt,
    fixed_output: bool,
) -> Option<Translation> {
    if !fails {
        return Some(Translation::Pass);
    }
    if !panic_at.fallible() {
        return Some(Translation::Unwrap("invalid value"));
    }
    return match fixed_output {
        true => None,
        false => Some(Translation::Pass),
    };
}

/// The settings that hold for a whole wrapper: `konst`, the resolved
/// `konst_*` settings, and `panic_at`.
#[derive(Default)]
pub(crate) struct WrapperWide {
    pub(crate) konst: Konst,
    pub(crate) konsts: KonstFlags,
    /// `panic_at = ...`: what generated code may still panic on.
    pub(crate) panic_at: PanicAt,
}

impl WrapperWide {
    /// Parses the value of a wrapper-wide key; `false` for any other key.
    fn parse_attr(
        &mut self,
        attr: Key,
        rest: ParseStream,
    ) -> syn::Result<bool> {
        match attr {
            Key::Konst => self.konst = rest.parse()?,
            Key::PanicAt => self.panic_at = zz::parse_choice(rest)?,
            _ => return Ok(false),
        }

        return Ok(true);
    }

    /// Takes the constness of the `outer` family this wrapper is nested in:
    /// each `konst_*` flag takes the outer value, and a disagreeing nested
    /// explicit setting fails at its span. `panic_at` stays: no outer family
    /// has one. Call after `Cfg::resolve`, which clears the konst flags.
    pub(crate) fn embed(
        &mut self,
        outer: &Self,
    ) -> syn::Result<()> {
        for flag in KonstFamily::Integral.konsts() {
            let required = outer
                .konsts
                .konst_for(&[flag], || outer.konst.enables(&[flag]));
            if let Some((value, span)) = self.konsts.get(flag)
                && value != required
            {
                return span.fail(
                    "this value is not compatible with own konst config of bitflag",
                );
            }
            self.konsts.insert(flag, required, Span::call_site());
        }
        self.konst = outer.konst;
        return Ok(());
    }
}

/// Who opens a slot.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Owner {
    /// The root of a wrapper without facets.
    Root,
    /// The root as the default facet: it takes the slots no facet claimed.
    Default,
    /// The facet at this index.
    Facet(usize),
}

/// A configuration level that opens operator slots: the root, or a facet
/// over it.
struct Level<'a> {
    flags: IntegralFlags,
    arithmetic: Arithmetic,
    produce: &'a SelfConverter,
    owner: Owner,
}

impl Level<'_> {
    /// The level's slot of `operator` for `rhs`, with the forms its flags
    /// turn on; `None` when it opens none. An assignment needs a `Self`
    /// output. A facet opens a slot only for an operator it has on, while
    /// the root keeps an assignment whose operator is off.
    fn slot(
        &self,
        operator: &OpSpec,
        rhs: RhsKind,
    ) -> Option<Slot> {
        let flags = IntegralFlag::of_op(operator.op);
        let root = self.owner == Owner::Root;
        let regular = self.flags.has(flags.math);
        let assign = self.produce.is_self()
            && flags.assign.is_some_and(|flag| self.flags.has(flag));
        if !regular && !(root && assign) {
            return None;
        }
        return Some(Slot {
            rhs,
            arithmetic: self.arithmetic,
            produce: self.produce.clone(),
            regular,
            assign,
        });
    }
}

/// A facet slot while the facets resolve: who claimed it, and where.
struct Claim {
    slot: Slot,
    /// The right-hand type as written; `Self` for the wrapper itself.
    identity: String,
    owner: Owner,
    span: Span,
}

#[derive(Default)]
pub(crate) struct Cfg {
    pub(crate) wide: WrapperWide,
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
    /// Every operator's slots: the facets' rows from `resolve`, the root's
    /// from `settle`.
    pub(crate) slots: Slots,
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

        for (flag, _, span) in settings.flags_by_name() {
            if flag == IntegralFlag::NonZero {
                return span.fail(
                    "`non_zero` is wrapper-wide and cannot be configured inside a facet",
                );
            }

            if !IntegralFlag::FACET_OPERANDS.contains(&flag)
                && !IntegralFlag::FACET_ASSIGNMENTS.contains(&flag)
            {
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

    /// One slot per operator of `kind` that `level` has on, for the
    /// right-hand type `ty` (`None`: the wrapper itself). An explicit facet
    /// rejects a claimed slot; the default facet keeps it.
    fn claim_slots(
        claims: &mut BTreeMap<Op, Vec<Claim>>,
        level: &Level,
        kind: OpKind,
        ty: Option<&Path>,
        conversion: Option<&Path>,
        span: Span,
    ) -> syn::Result<()> {
        let identity = ty.map_or_else(
            || "Self".to_string(),
            |ty| ty.to_token_stream().to_string(),
        );

        for operator in OPS.iter().filter(|it| it.kind == kind) {
            let rhs = RhsKind::Facet {
                ty: ty.cloned(),
                conversion: conversion.cloned(),
            };
            let Some(slot) = level.slot(operator, rhs)
            else {
                continue;
            };
            let row = claims.entry(operator.op).or_default();
            let previous = row.iter().find(|it| it.identity == identity);
            match (previous, level.owner) {
                (Some(previous), Owner::Facet(_)) => {
                    return span.fail_combi(
                        "duplicate explicit facet slot",
                        previous.span,
                        "the slot was first claimed here",
                    );
                }
                (Some(_), Owner::Root | Owner::Default) => {}
                (None, owner) => row.push(Claim {
                    slot,
                    identity: identity.clone(),
                    owner,
                    span,
                }),
            }
        }

        return Ok(());
    }

    /// A facet level's slots: each friend's capabilities in
    /// `FACET_OPERATORS` order, then `self_math` and `self_bit` at their
    /// spans.
    fn claim_level_slots(
        claims: &mut BTreeMap<Op, Vec<Claim>>,
        level: &Level,
        friends: &BTreeSet<Friend>,
        self_math: Option<Span>,
        self_bit: Option<Span>,
    ) -> syn::Result<()> {
        for friend in friends {
            let span = friend.ty.path().span();
            for (capability, kind) in FACET_OPERATORS {
                if friend.has(capability) {
                    Self::claim_slots(
                        claims,
                        level,
                        kind,
                        Some(friend.ty.path()),
                        friend.conv.as_ref(),
                        span,
                    )?;
                }
            }
        }

        // `FACET_OPERATORS` lists `Math` first.
        let self_slots = [
            self_math,
            self_bit,
        ];
        for ((_, kind), span) in FACET_OPERATORS.into_iter().zip(self_slots) {
            if let Some(span) = span {
                Self::claim_slots(claims, level, kind, None, None, span)?;
            }
        }

        return Ok(());
    }

    /// The facets' rows of the slot table: each explicit facet's slots in
    /// declaration order, then, as the default facet, the root's for what
    /// no facet claimed. In a row the same type comes first; the friends
    /// follow as claimed, in the order a level's friend set keeps them. A
    /// wrapper without facets has no such row: its rows come from the root
    /// once its flags are final (`settle_slots`).
    fn resolve_facets(&mut self) -> syn::Result<()> {
        self.slots = Slots::new(self.facets_present());
        if !self.slots.by_facets() {
            return Ok(());
        }

        let mut claims = BTreeMap::new();
        for (index, facet) in self.facets.iter().enumerate() {
            Self::validate_facet(facet)?;

            let level = Level {
                flags: Self::resolved_facet_flags(&self.flags, facet),
                arithmetic: facet
                    .profile
                    .arithmetic_or(self.profile.arithmetic()),
                produce: facet.ty_out.as_ref().unwrap_or(&self.ty_out),
                owner: Owner::Facet(index),
            };
            let self_slot = |flag| {
                return match facet.explicit_flag(flag) {
                    Some((true, span)) => Some(span),
                    _ => None,
                };
            };
            Self::claim_level_slots(
                &mut claims,
                &level,
                &facet.friends,
                self_slot(IntegralFlag::SelfMath),
                self_slot(IntegralFlag::SelfBit),
            )?;
            Self::check_facet_assignments(
                &claims,
                &level,
                self.wide.panic_at,
                |flag| facet.explicit_flag(flag),
            )?;
        }

        let level = Level {
            flags: self.flags,
            arithmetic: self.profile.arithmetic(),
            produce: &self.ty_out,
            owner: Owner::Default,
        };
        let self_slot = |flag| self.flags.has(flag).then(Span::call_site);
        Self::claim_level_slots(
            &mut claims,
            &level,
            &self.friends,
            self_slot(IntegralFlag::SelfMath),
            self_slot(IntegralFlag::SelfBit),
        )?;
        Self::check_facet_assignments(
            &claims,
            &level,
            self.wide.panic_at,
            |flag| self.explicit_flag(flag),
        )?;

        for (op, mut row) in claims {
            row.sort_by_key(|claim| {
                return !matches!(
                    claim.slot.rhs,
                    RhsKind::Facet { ty: None, .. }
                );
            });
            let row = row.into_iter().map(|claim| claim.slot).collect();
            self.slots.set(op, row);
        }
        return self.check_facet_assignment_konst();
    }

    /// The root's rows of the slot table, built once its flags are final:
    /// every operator's in a wrapper without facets, else only those that
    /// facets do not dispatch (shifts and unary operators). A row is the
    /// same type (`self_math`, `self_bit`) and then the capability's
    /// friends, or a shift's amount. A `generic` wrapper has no same-type
    /// slot: a separate `Self` impl would overlap its friend impl, so it is
    /// its own `Math` or `Bit` friend instead.
    fn settle_slots(
        &mut self,
        generic: bool,
    ) {
        let level = Level {
            flags: self.flags,
            arithmetic: self.profile.arithmetic(),
            produce: &self.ty_out,
            owner: Owner::Root,
        };
        let same = |flag| {
            return (self.flags.has(flag) && !generic).then_some(RhsKind::Same);
        };
        for operator in OPS {
            if operator.kind.has_operand() && self.slots.by_facets() {
                continue;
            }
            let rhs = match operator.kind {
                OpKind::Arith => [
                    same(IntegralFlag::SelfMath),
                    Some(RhsKind::Friend(Capability::Math)),
                ],
                OpKind::Bitwise => [
                    same(IntegralFlag::SelfBit),
                    Some(RhsKind::Friend(Capability::Bit)),
                ],
                OpKind::Shift => [
                    None,
                    Some(RhsKind::Amount),
                ],
                OpKind::Unary => [
                    None,
                    Some(RhsKind::Unary),
                ],
            };
            let row = rhs
                .into_iter()
                .flatten()
                .filter_map(|rhs| level.slot(operator, rhs))
                .collect();
            self.slots.set(operator.op, row);
        }
    }

    /// Each assignment flag a facet level explicitly enables assigns through
    /// that level's slots of its operator, which must output `Self`, and
    /// only where a failure may panic: an assignment's output is fixed.
    fn check_facet_assignments(
        claims: &BTreeMap<Op, Vec<Claim>>,
        level: &Level,
        panic_at: PanicAt,
        explicit: impl Fn(IntegralFlag) -> Option<(bool, Span)>,
    ) -> syn::Result<()> {
        for operator in OPS.iter().filter(|it| it.kind.has_operand()) {
            let flags = IntegralFlag::of_op(operator.op);
            let Some(flag) = flags.assign
            else {
                continue;
            };
            let Some((true, span)) = explicit(flag)
            else {
                continue;
            };
            if translate(true, panic_at, true).is_none() {
                return span.fail(format!(
                    "{} cannot report failure; panic_at = {} removes \
                    assignment operators",
                    flag.name(),
                    panic_at.name(),
                ));
            }
            if !level.produce.is_self() {
                return span
                    .fail("produce cannot generate assignment operators");
            }
            let owned = claims.get(&operator.op).is_some_and(|row| {
                return row.iter().any(|claim| claim.owner == level.owner);
            });
            if !owned {
                let math = flags.math.name();
                return span.fail(format!(
                    "`{}` needs a `{math}` slot to assign through",
                    flag.name(),
                ));
            }
        }
        return Ok(());
    }

    /// A facet assignment forwards to its operator's dispatch, so it can be
    /// const only when that dispatch is. Only explicit `konst_*` settings
    /// can split them; the first one deciding the assignment is reported.
    fn check_facet_assignment_konst(&self) -> syn::Result<()> {
        for operator in OPS {
            if !self.slots.of(operator.op).iter().any(|slot| slot.assign) {
                continue;
            }
            let assign = operator.assign_konsts();
            if !self.konst_for(&assign) || self.konst_for(&operator.konsts()) {
                continue;
            }
            let span = assign
                .iter()
                .find_map(|flag| self.wide.konsts.get(*flag))
                .map_or_else(Span::call_site, |(_, span)| span);
            return span.fail(
                "a facet assignment forwards to its operator and can be \
                const only when the operator is",
            );
        }
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
                if !self.profile.iter().all(Profile::accepts_float)
                    || !self.facets.iter().all(|facet| {
                        return facet
                            .profile
                            .iter()
                            .all(Profile::accepts_float);
                    })
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

    /// Settles what depends on the declaration: resolves the domain for
    /// the value type, then `Neg`'s, `proven`'s, and `AsRef`'s defaults,
    /// and the root's slots (`generic`: the wrapper has generic parameters).
    pub(crate) fn settle(
        &mut self,
        raw: Num,
        generic: bool,
    ) -> syn::Result<()> {
        self.validation.resolve(raw, self.math)?;
        self.settle_neg(raw)?;
        self.settle_proven(raw)?;
        self.settle_as_ref()?;
        self.settle_slots(generic);
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
        if self.wide.panic_at.fallible() {
            let level = self.wide.panic_at.name();
            if let Some(item) = self.prove.first() {
                return item.name.fail(format!(
                    "panic_at = {level} validates every result and takes no \
                    `prove`"
                ));
            }
            return self.deny_flags(&[IntegralFlag::Proven], |_| {
                return format!(
                    "panic_at = {level} validates every result and takes no \
                    closure proofs"
                );
            });
        }
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
        self.wide.konsts = KonstFlags::default();
        self.removals = Removals::default();
        for profile in self.profile.iter() {
            profile.apply(&mut self.flags);
            profile.settings().layer(
                &mut self.flags,
                &mut self.wide.konsts,
                &mut self.removals,
            );
        }
        self.root.layer(
            &mut self.flags,
            &mut self.wide.konsts,
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

        // A form whose output is fixed cannot report a failure.
        if translate(true, self.wide.panic_at, true).is_none() {
            self.settle_panic_at()?;
        }

        return Ok(());
    }

    /// What `panic_at` below `anything` rules out: assignment operators,
    /// which cannot report failure, and construction without validation
    /// (`unsafe` friends, `with_blind_trust`).
    fn settle_panic_at(&mut self) -> syn::Result<()> {
        let level = self.wide.panic_at.name();
        let unsafe_friend = self.friends.iter().find_map(|friend| {
            return friend.capabilities.iter().find(|it| {
                return Capability::of(it) == Some(Capability::Unsafe);
            });
        });
        if let Some(capability) = unsafe_friend {
            return capability.fail(format!(
                "`unsafe` friends construct without validation, which \
                panic_at = {level} rules out"
            ));
        }
        self.deny_flags(&[IntegralFlag::WithBlindTrust], |flag| {
            return format!(
                "{} constructs without validation, which panic_at = {level} \
                rules out",
                flag.name(),
            );
        })?;
        self.deny_flags(&[IntegralFlag::FloatMath], |flag| {
            return format!(
                "{} returns `Self` from results it would have to panic on; \
                panic_at = {level} leaves it out",
                flag.name(),
            );
        })?;
        return self.deny_flags(IntegralFlag::ASSIGN_FLAGS, |flag| {
            return format!(
                "{} cannot report failure; panic_at = {level} removes \
                assignment operators",
                flag.name(),
            );
        });
    }

    /// The facets' slots, then the storage the profiles require.
    fn resolve_facet_layer(&mut self) -> syn::Result<()> {
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
            .wide
            .konsts
            .konst_for(flags, || self.wide.konst.enables(flags));
    }

    pub(crate) fn konst_friendship(&self) -> bool {
        if self.konst_for(&[Konsts::Friendship]) {
            return true;
        }

        return KonstFamily::Integral
            .konsts()
            .filter(|flag| *flag != Konsts::Friendship && !flag.is_stable())
            .any(|flag| self.konst_for(&[flag]));
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
        site: Site,
    ) -> syn::Result<()> {
        let fixed_profile = site == Site::FixedProfile;
        return zz::parse_inner(input, site, |attr, rest| {
            if self.validation.parse_attr(attr, rest)?
                || self.wide.parse_attr(attr, rest)?
            {
                return Ok(true);
            }

            match attr {
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
                Key::Tree => {
                    self.tree = Some(marker_tree::Cfg::parse_attribute(rest)?);
                }
                Key::HideIn => self.hide_in = rest.parse()?,
                Key::Facet => {
                    self.facets = zz::one_or_list::<FacetCfg>(rest)?.collect();
                    self.facets_present = true;
                }
                Key::Precedes if fixed_profile => {
                    if !self.profile.single_mut().parse_attr(attr, rest)? {
                        return Ok(false);
                    }
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
        this.parse_attrs(input, Site::Root)?;
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
        this.parse_attrs(input, Site::FixedProfile)?;
        this.resolve()?;
        return Ok(this);
    }

    pub(crate) fn parse_with_konst_unresolved(
        input: ParseStream
    ) -> syn::Result<Self> {
        let mut this = Self::without_konst();
        this.parse_attrs(input, Site::Nested)?;
        return Ok(this);
    }

    pub(crate) fn without_konst() -> Self {
        return Self {
            wide: WrapperWide {
                konst: Konst::None,
                ..Default::default()
            },
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
            .field("konst", &self.wide.konst)
            .field("get_raw", &self.get_raw.to_token_stream().to_string())
            .field("validation", &self.validation)
            .field("friends", &self.friends)
            .field("non_zero", &self.non_zero)
            .field("ty_out", &!self.ty_out.is_self())
            .field("profile", &self.profile.arithmetic())
            .field("tree", &self.tree.is_some())
            .field("slots", &self.slots)
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
        this.parse_attrs(input, Site::Root)?;
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
        Self::AssignXor,
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
        Self::AssignXor,
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
        Self::AssignXor,
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

    /// The float kind's defaults beyond the rows that are `on`;
    /// `float_math` follows the math provider.
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

    /// The assignment flags a facet may set: those of its operand operators
    /// (shifts are wrapper-wide).
    pub(crate) const FACET_ASSIGNMENTS: &'static [Self] = &[
        Self::AssignAdd,
        Self::AssignSub,
        Self::AssignMul,
        Self::AssignDiv,
        Self::AssignRem,
        Self::AssignAnd,
        Self::AssignOr,
        Self::AssignXor,
    ];

    /// The flags of `op`: its operator, its assignment and its helper.
    pub(crate) fn of_op(op: Op) -> OpFlags {
        let (math, assign, helper) = match op {
            Op::Add => (Self::MathAdd, Some(Self::AssignAdd), Self::FnMathAdd),
            Op::Sub => (Self::MathSub, Some(Self::AssignSub), Self::FnMathSub),
            Op::Mul => (Self::MathMul, Some(Self::AssignMul), Self::FnMathMul),
            Op::Div => (Self::MathDiv, Some(Self::AssignDiv), Self::FnMathDiv),
            Op::Rem => (Self::MathRem, Some(Self::AssignRem), Self::FnMathRem),
            Op::BitAnd => {
                (Self::MathAnd, Some(Self::AssignAnd), Self::FnMathAnd)
            }
            Op::BitOr => (Self::MathOr, Some(Self::AssignOr), Self::FnMathOr),
            Op::BitXor => {
                (Self::MathXor, Some(Self::AssignXor), Self::FnMathXor)
            }
            Op::Shl => (Self::MathShl, Some(Self::AssignShl), Self::FnMathShl),
            Op::Shr => (Self::MathShr, Some(Self::AssignShr), Self::FnMathShr),
            Op::Neg => (Self::MathNeg, None, Self::FnMathNeg),
            Op::Not => (Self::MathNot, None, Self::FnMathNot),
        };
        return OpFlags {
            math,
            assign,
            helper,
        };
    }
}

/// The integral flags of one operator.
pub(crate) struct OpFlags {
    /// `math_*`: generates the operator.
    pub(crate) math: IntegralFlag,
    /// `assign_*`: generates the assignment operator; the unary operators
    /// have none.
    pub(crate) assign: Option<IntegralFlag>,
    /// `fn_math_*`: generates the validating helper the operator calls.
    pub(crate) helper: IntegralFlag,
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
    RhsTrait(Op),
    /// `conv_{snake}`: the seal's conversion.
    Conversion,
    /// `make`: the value-type constructor `auto_of_raw` generates.
    MakeFn,
    /// `of`: the friend constructor.
    OfFn,
    /// `try_of`: the friend constructor reporting failure.
    TryOfFn,
    /// `_or_panic`: the validating, panicking constructor.
    OrPanic,
    /// `_or_err`: validates an operation's `Option` of a raw result.
    OrErr,
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
            Self::RhsTrait(operator) => {
                format!("{}Rhs", operator.spec().trait_name)
            }
            Self::Conversion => {
                format!("conv_{}", snake_case_of(&owner.to_string()))
            }
            Self::MakeFn => "make".to_string(),
            Self::OfFn => "of".to_string(),
            Self::TryOfFn => "try_of".to_string(),
            Self::OrPanic => "_or_panic".to_string(),
            Self::OrErr => "_or_err".to_string(),
            Self::BlindTrust => "_blind_trust".to_string(),
            Self::Into(target) => format!("into_{}", target.name()),
            Self::TryInto(target) => format!("try_into_{}", target.name()),
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
                format!("lo{}", half.bits_for_pointer_width(Width::W64))
            }
            Self::HighHalf(half) => {
                format!("hi{}", half.bits_for_pointer_width(Width::W64))
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
    AssignXor,
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
    const ROWS: &'static [FlagRow<Self>] = &[
        FlagRow::off(Self::Range, "range"),
        FlagRow::on(Self::AsRef, "as_ref_"),
        FlagRow::on(Self::AutoOfRaw, "auto_of_raw"),
        FlagRow::on(Self::Assertions, "assertions"),
        FlagRow::on(Self::BitAccess, "bit_access"),
        FlagRow::on(Self::Clone, "clone"),
        FlagRow::on(Self::Copy, "copy"),
        FlagRow::on(Self::Debug, "debug"),
        FlagRow::off(Self::Display, "display"),
        FlagRow::on(Self::Hash, "hash"),
        FlagRow::on(Self::Eq, "eq"),
        FlagRow::on(Self::FmtBinary, "fmt_binary"),
        FlagRow::on(Self::FmtHexLower, "fmt_hex_lower"),
        FlagRow::on(Self::FmtHexUpper, "fmt_hex_upper"),
        FlagRow::on(Self::FmtOctal, "fmt_octal"),
        FlagRow::on(Self::FromStr, "from_str"),
        FlagRow::on(Self::CoreInt, "core_int"),
        FlagRow::on(Self::Into, "into"),
        FlagRow::on(Self::Map, "map"),
        FlagRow::on(Self::Ord, "ord"),
        FlagRow::on(Self::PartialEq, "partial_eq"),
        FlagRow::on(Self::PartialOrd, "partial_ord"),
        FlagRow::on(Self::Repr, "repr"),
        FlagRow::off(Self::Serde, "serde"),
        FlagRow::on(Self::TryInto, "try_into"),
        FlagRow::off(Self::WithConstMapFn, "with_const_map_fn"),
        FlagRow::off(Self::WithBlindTrust, "with_blind_trust"),
        FlagRow::off(Self::LitMacro, "lit_macro"),
        FlagRow::off(Self::NonZero, "non_zero"),
        FlagRow::off(Self::Proven, "proven"),
        FlagRow::on(Self::WithinSource, "within_source"),
        FlagRow::off(Self::MathNeg, "math_neg"),
        FlagRow::off(Self::FnMathNeg, "fn_math_neg"),
        FlagRow::off(Self::CoreFloat, "core_float"),
        FlagRow::off(Self::FloatMath, "float_math"),
        FlagRow::off(Self::FmtExpLower, "fmt_exp_lower"),
        FlagRow::off(Self::FmtExpUpper, "fmt_exp_upper"),
        FlagRow::off(Self::Sum, "sum"),
        FlagRow::off(Self::Product, "product"),
        FlagRow::on(Self::AssignAdd, "assign_add"),
        FlagRow::on(Self::AssignAnd, "assign_and"),
        FlagRow::on(Self::AssignDiv, "assign_div"),
        FlagRow::on(Self::AssignMul, "assign_mul"),
        FlagRow::on(Self::AssignOr, "assign_or"),
        FlagRow::on(Self::AssignRem, "assign_rem"),
        FlagRow::on(Self::AssignShl, "assign_shl"),
        FlagRow::on(Self::AssignShr, "assign_shr"),
        FlagRow::on(Self::AssignSub, "assign_sub"),
        FlagRow::on(Self::AssignXor, "assign_xor"),
        FlagRow::on(Self::MathAdd, "math_add"),
        FlagRow::on(Self::MathAnd, "math_and"),
        FlagRow::on(Self::MathDiv, "math_div"),
        FlagRow::on(Self::MathMul, "math_mul"),
        FlagRow::on(Self::MathNot, "math_not"),
        FlagRow::on(Self::MathOr, "math_or"),
        FlagRow::on(Self::MathRem, "math_rem"),
        FlagRow::on(Self::MathShl, "math_shl"),
        FlagRow::on(Self::MathShr, "math_shr"),
        FlagRow::on(Self::MathSub, "math_sub"),
        FlagRow::on(Self::MathXor, "math_xor"),
        FlagRow::on(Self::SelfFriendMake, "self_friend_make"),
        FlagRow::on(Self::SelfFriendMathBit, "self_friend_math_bit"),
        FlagRow::on(Self::SelfFriendMathOps, "self_friend_math_ops"),
        FlagRow::on(Self::SelfFriendRel, "self_friend_rel"),
        FlagRow::on(Self::FriendSeal, "friend_seal"),
        FlagRow::on(Self::FriendzoneFriendMake, "friendzone_friend_make"),
        FlagRow::on(
            Self::FriendzoneFriendMathBit,
            "friendzone_friend_math_bit",
        ),
        FlagRow::on(
            Self::FriendzoneFriendMathOps,
            "friendzone_friend_math_ops",
        ),
        FlagRow::on(Self::FriendzoneFriendRel, "friendzone_friend_rel"),
        FlagRow::on(Self::FriendzoneSeal, "friendzone_seal"),
        FlagRow::on(Self::SelfMath, "self_math"),
        FlagRow::on(Self::SelfBit, "self_bit"),
        FlagRow::on(Self::FnConvInto, "fn_conv_into"),
        FlagRow::on(Self::FnConvOf, "fn_conv_of"),
        FlagRow::on(Self::FnConvRaw, "fn_conv_raw"),
        FlagRow::on(Self::FnConvTryIntoLossy, "fn_conv_try_into_lossy"),
        FlagRow::on(Self::FnConvTryIntoLossless, "fn_conv_try_into_lossless"),
        FlagRow::on(Self::FnOrPanic, "fn_or_panic"),
        FlagRow::on(Self::FnTryMake, "fn_try_make"),
        FlagRow::on(Self::FnMathAdd, "fn_math_add"),
        FlagRow::on(Self::FnMathAnd, "fn_math_and"),
        FlagRow::on(Self::FnMathDiv, "fn_math_div"),
        FlagRow::on(Self::FnMathMul, "fn_math_mul"),
        FlagRow::on(Self::FnMathNot, "fn_math_not"),
        FlagRow::on(Self::FnMathOr, "fn_math_or"),
        FlagRow::on(Self::FnMathRem, "fn_math_rem"),
        FlagRow::on(Self::FnMathShl, "fn_math_shl"),
        FlagRow::on(Self::FnMathShr, "fn_math_shr"),
        FlagRow::on(Self::FnMathSub, "fn_math_sub"),
        FlagRow::on(Self::FnMathXor, "fn_math_xor"),
        FlagRow::on(Self::FnOpCmp, "fn_op_cmp"),
        FlagRow::on(Self::FnOpEq, "fn_op_eq"),
    ];

    fn index(self) -> u32 {
        return self as u32;
    }
}

// Every row of `ROWS` is at its variant's index.
const _: () = {
    let rows = <IntegralFlag as Flag>::ROWS;
    let mut index = 0;
    while index < rows.len() {
        assert!(rows[index].flag as usize == index);
        index += 1;
    }
};
