//! Generated names: each family's name keys, the names a user chose, and
//! the defaults with today's spans and generic-parameter collision rule.
//! Emitters read every generated name they spell through a family table
//! (`names.get(IntegralName::SealTrait)`); no logic compares one.

use std::collections::{
    BTreeMap,
    BTreeSet,
};

use proc_macro2::{
    Ident,
    Span,
};

/// One family's name keys: `IntegralName`, `BitflagName`, `TextName`,
/// `FriendsName`, `ConstructorName`.
pub(crate) trait NameKey: Copy + Ord {
    /// What the defaults derive from: the target's identifier, or for
    /// `friends` its relation.
    type Owner;

    /// Today's name for `owner`: `Seal`, `make`, `conv_my_u32`,
    /// `MyFlagValue`.
    fn default_name(
        self,
        owner: &Self::Owner,
    ) -> String;

    /// Renamed `__Typekin…` on a generic-parameter collision, with a
    /// `mixed_site` span, as `fresh_ident` names are.
    fn avoids_generics(self) -> bool {
        return false;
    }

    /// Today's span: `mixed_site` for names that avoid generic parameters,
    /// else the call site.
    fn default_span(
        self,
        _owner: &Self::Owner,
    ) -> Span {
        return match self.avoids_generics() {
            true => Span::mixed_site(),
            false => Span::call_site(),
        };
    }
}

/// The names a user chose, each with its own span.
#[derive(Clone)]
pub(crate) struct Names<K> {
    chosen: BTreeMap<K, Ident>,
}

impl<K> Default for Names<K> {
    fn default() -> Self {
        return Self {
            chosen: BTreeMap::new(),
        };
    }
}

impl<K: NameKey> Names<K> {
    pub(crate) fn choose(
        &mut self,
        key: K,
        name: Ident,
    ) {
        self.chosen.insert(key, name);
    }

    /// Once the owner and the generic parameters' names are known.
    pub(crate) fn resolve(
        self,
        owner: K::Owner,
        occupied: BTreeSet<String>,
    ) -> ResolvedNames<K> {
        return ResolvedNames {
            chosen: self.chosen,
            owner,
            occupied,
        };
    }
}

/// The chosen names, else the defaults with today's spans and collision
/// rule.
pub(crate) struct ResolvedNames<K: NameKey> {
    chosen: BTreeMap<K, Ident>,
    owner: K::Owner,
    occupied: BTreeSet<String>,
}

impl<K: NameKey> ResolvedNames<K> {
    pub(crate) fn get(
        &self,
        key: K,
    ) -> Ident {
        if let Some(name) = self.chosen.get(&key) {
            return name.clone();
        }

        let mut name = key.default_name(&self.owner);
        if key.avoids_generics() && self.occupied.contains(&name) {
            name = format!("__Typekin{name}");
            while self.occupied.contains(&name) {
                name.push('_');
            }
        }
        return Ident::new(&name, key.default_span(&self.owner));
    }
}
