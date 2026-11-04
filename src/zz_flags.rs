//! Typed `with`/`without` flags: a family's flag enum, the set of its
//! resolved flags, and the explicit settings of one configuration level.

use std::collections::BTreeMap;
use std::fmt::{
    Debug,
    Formatter,
};
use std::marker::PhantomData;

use proc_macro2::{
    Ident,
    Span,
};
use syn::parse::ParseStream;

use crate::zz_lib::{
    Konsts,
    Removals,
    one_or_list,
};
use crate::zz_runner::MkErr;
use crate::zz_values::FnName;

/// The flag prefix of a function removal: `without = fn_{name}`.
const REMOVAL_PREFIX: &str = "fn_";

/// A family's flag enum, hand-written with its name table (the `FnName`
/// convention); `of` is the parse boundary.
pub(crate) trait Flag: Copy + Ord + 'static {
    /// Every variant, in declaration order: `ITEMS[i].index() == i`.
    const ITEMS: &'static [Self];
    /// The flags a configuration starts with.
    const DEFAULT_ON: &'static [Self];

    fn name(self) -> &'static str;

    /// `self as u32`: the position in `ITEMS`, below 128.
    fn index(self) -> u32;

    fn of(name: &str) -> Option<Self> {
        return Self::ITEMS.iter().copied().find(|it| it.name() == name);
    }
}

/// A set of one family's flags: fixed order, `Copy`, no allocation. The
/// default is the family's `DEFAULT_ON`.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct Flags<F> {
    bits: u128,
    flag: PhantomData<F>,
}

impl<F: Flag> Flags<F> {
    pub(crate) fn of(on: &[F]) -> Self {
        let mut this = Self {
            bits: 0,
            flag: PhantomData,
        };
        for flag in on {
            this.set(*flag, true);
        }
        return this;
    }

    pub(crate) fn has(
        self,
        flag: F,
    ) -> bool {
        return self.bits & (1 << flag.index()) != 0;
    }

    pub(crate) fn set(
        &mut self,
        flag: F,
        on: bool,
    ) {
        match on {
            true => self.bits |= 1 << flag.index(),
            false => self.bits &= !(1 << flag.index()),
        }
    }

    /// Turns every flag of `flags` off.
    pub(crate) fn clear(
        &mut self,
        flags: &[F],
    ) {
        for flag in flags {
            self.set(*flag, false);
        }
    }

    /// The flags that are on, in `ITEMS` order.
    pub(crate) fn iter(self) -> impl Iterator<Item = F> {
        return F::ITEMS.iter().copied().filter(move |it| self.has(*it));
    }
}

impl<F: Flag> Default for Flags<F> {
    fn default() -> Self {
        return Self::of(F::DEFAULT_ON);
    }
}

impl<F: Flag> Debug for Flags<F> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return f.debug_set().entries(self.iter().map(F::name)).finish();
    }
}

/// What a family accepts in `with`/`without` besides its own flags.
pub(crate) struct FlagGrammar {
    /// The `konst_*` names it accepts.
    pub(crate) konsts: &'static [Konsts],
    /// `without = fn_{name}` removes a generated forward.
    pub(crate) removals: bool,
}

/// One `with`/`without` name, typed once.
#[derive(Clone, Copy)]
enum Setting<F> {
    Konst(Konsts),
    Flag(F),
    Removal(FnName),
}

/// Resolved or explicit `konst_*` settings, each with the span that set it.
#[derive(Clone, Default)]
pub(crate) struct KonstFlags(BTreeMap<Konsts, (bool, Span)>);

impl KonstFlags {
    pub(crate) fn get(
        &self,
        flag: Konsts,
    ) -> Option<(bool, Span)> {
        return self.0.get(&flag).copied();
    }

    pub(crate) fn insert(
        &mut self,
        flag: Konsts,
        value: bool,
        span: Span,
    ) {
        self.0.insert(flag, (value, span));
    }

    /// The first of `flags` set explicitly, else `fallback`.
    pub(crate) fn konst_for(
        &self,
        flags: &[Konsts],
        fallback: impl FnOnce() -> bool,
    ) -> bool {
        return flags
            .iter()
            .find_map(|flag| self.get(*flag).map(|(value, _)| value))
            .unwrap_or_else(fallback);
    }

    /// The span of the first setting in `Konsts` order.
    pub(crate) fn first_span(&self) -> Option<Span> {
        return self.0.values().next().map(|(_, span)| *span);
    }

    /// Layers `that` over these settings.
    pub(crate) fn extend(
        &mut self,
        that: &Self,
    ) {
        self.0
            .extend(that.0.iter().map(|(flag, setting)| (*flag, *setting)));
    }
}

/// The explicit `with`/`without` settings of one configuration level.
#[derive(Clone)]
pub(crate) struct FlagSettings<F> {
    flags: BTreeMap<F, (bool, Span)>,
    konsts: KonstFlags,
    removals: Removals,
}

impl<F> Default for FlagSettings<F> {
    fn default() -> Self {
        return Self {
            flags: BTreeMap::new(),
            konsts: KonstFlags::default(),
            removals: Removals::default(),
        };
    }
}

impl<F: Flag> Debug for FlagSettings<F> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return f
            .debug_map()
            .entries(
                self.flags
                    .iter()
                    .map(|(flag, (value, _))| (flag.name(), value)),
            )
            .finish();
    }
}

impl<F: Flag> FlagSettings<F> {
    /// One `with` (`value = true`) or `without` list. A name is a `konst_*`
    /// flag of `grammar`, else a flag of `F`, else (when `grammar` allows)
    /// a removal; a setting this level already holds is a duplicate, or a
    /// conflict when its value differs.
    pub(crate) fn parse(
        &mut self,
        input: ParseStream,
        value: bool,
        grammar: &FlagGrammar,
    ) -> syn::Result<()> {
        for ident in one_or_list::<Ident>(input)? {
            let name = ident.to_string();
            let span = ident.span();
            let setting = match Self::classify(&name, grammar) {
                Some(setting) => setting,
                None => return span.fail("unknown flag"),
            };

            let previous = match setting {
                Setting::Konst(flag) => {
                    self.konsts.get(flag).map(|(value, _)| value)
                }
                Setting::Flag(flag) => {
                    self.flags.get(&flag).map(|(value, _)| *value)
                }
                Setting::Removal(function) => {
                    self.removals.removes(function).then_some(false)
                }
            };
            match previous {
                Some(previous) if previous != value => {
                    return span
                        .fail("flag specified in both `with` and `without`");
                }
                Some(_) => return span.fail("duplicated flags"),
                None => {}
            }

            match setting {
                Setting::Konst(flag) => self.konsts.insert(flag, value, span),
                Setting::Flag(flag) => {
                    self.flags.insert(flag, (value, span));
                }
                Setting::Removal(function) => {
                    if value {
                        let function = function.name();
                        return span.fail(format!(
                            "`{name}` only removes the generated `{function}`; write `without = {name}`",
                        ));
                    }
                    self.removals.insert(function, span);
                }
            }
        }

        return Ok(());
    }

    fn classify(
        name: &str,
        grammar: &FlagGrammar,
    ) -> Option<Setting<F>> {
        if let Some(flag) = Konsts::of_in(name, grammar.konsts) {
            return Some(Setting::Konst(flag));
        }
        if let Some(flag) = F::of(name) {
            return Some(Setting::Flag(flag));
        }
        if !grammar.removals {
            return None;
        }
        return name
            .strip_prefix(REMOVAL_PREFIX)
            .and_then(FnName::of)
            .map(Setting::Removal);
    }

    pub(crate) fn flag(
        &self,
        flag: F,
    ) -> Option<(bool, Span)> {
        return self.flags.get(&flag).copied();
    }

    /// The explicit flags in name order.
    pub(crate) fn flags_by_name(&self) -> Vec<(F, bool, Span)> {
        let mut flags = self
            .flags
            .iter()
            .map(|(flag, (value, span))| (*flag, *value, *span))
            .collect::<Vec<_>>();
        flags.sort_by_key(|(flag, ..)| flag.name());
        return flags;
    }

    pub(crate) fn konsts(&self) -> &KonstFlags {
        return &self.konsts;
    }

    pub(crate) fn removals(&self) -> &Removals {
        return &self.removals;
    }

    /// Sets every explicit flag of this level in `flags`.
    pub(crate) fn apply(
        &self,
        flags: &mut Flags<F>,
    ) {
        for (flag, (value, _)) in &self.flags {
            flags.set(*flag, *value);
        }
    }

    /// Layers this level: flags, removals, konst flags.
    pub(crate) fn layer(
        &self,
        flags: &mut Flags<F>,
        konsts: &mut KonstFlags,
        removals: &mut Removals,
    ) {
        self.apply(flags);
        removals.extend(&self.removals);
        konsts.extend(&self.konsts);
    }
}
