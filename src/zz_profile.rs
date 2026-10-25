use std::collections::BTreeSet;
use std::fmt::{
    Debug,
    Formatter,
};

use proc_macro2::Ident;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::{
    LitBool,
    braced,
};

use crate::friendship::cfg::Friend;
use crate::integral::{
    Cfg,
    IntSettings,
    IntegralFlags,
};
use crate::zz;
use crate::zz_conv::SelfConverter;
use crate::zz_runner::{
    MkErr,
    snake_case_of,
};
use crate::zz_value_type::RESTRICTED_MATH_FLAGS;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Arithmetic {
    Primitive,
    Wrapping,
    Checked,
}

#[derive(Clone, Default)]
pub(crate) struct Int {
    settings: IntSettings,
}

#[derive(Clone)]
pub(crate) struct Circular {
    settings: IntSettings,
    precedes: bool,
}

impl Default for Circular {
    fn default() -> Self {
        return Self {
            settings: Default::default(),
            precedes: true,
        };
    }
}

#[derive(Clone, Default)]
pub(crate) struct Sequential {
    settings: IntSettings,
}

#[derive(Clone)]
pub(crate) enum Profile {
    Int(Int),
    Circular(Circular),
    Sequential(Sequential),
}

impl Profile {
    pub(crate) fn from_name(it: Ident) -> syn::Result<Self> {
        let this = match snake_case_of(&it.to_string()).as_str() {
            "int" => Self::Int(Int::default()),
            "circular" => Self::Circular(Circular::default()),
            "sequential" => Self::Sequential(Sequential::default()),
            _ => return it.fail("unknown profile"),
        };

        return Ok(this);
    }

    pub(crate) fn parse_attr(
        &mut self,
        attr: &str,
        rest: ParseStream,
    ) -> syn::Result<bool> {
        match attr {
            "with" => Cfg::parse_flags_into(rest, true, self.settings_mut())?,
            "without" => {
                Cfg::parse_flags_into(rest, false, self.settings_mut())?
            }
            "precedes" => {
                let Self::Circular(Circular { precedes, .. }) = self
                else {
                    return Ok(false);
                };
                *precedes = rest.parse::<LitBool>()?.value;
            }
            _ => return Ok(false),
        }

        return Ok(true);
    }

    pub(crate) fn arithmetic(&self) -> Arithmetic {
        return match self {
            Self::Int { .. } => Arithmetic::Primitive,
            Self::Circular { .. } => Arithmetic::Wrapping,
            Self::Sequential { .. } => Arithmetic::Checked,
        };
    }

    pub(crate) fn is_circular(&self) -> bool {
        return matches!(self, Self::Circular { .. });
    }

    pub(crate) fn emits_precedes(&self) -> bool {
        return matches!(self, Self::Circular(Circular { precedes: true, .. }));
    }

    pub(crate) fn int_settings(&self) -> &IntSettings {
        return match self {
            Self::Int(Int { settings, .. })
            | Self::Circular(Circular { settings, .. })
            | Self::Sequential(Sequential { settings, .. }) => settings,
        };
    }

    fn settings_mut(&mut self) -> &mut IntSettings {
        return match self {
            Self::Int(Int { settings, .. })
            | Self::Circular(Circular { settings, .. })
            | Self::Sequential(Sequential { settings, .. }) => settings,
        };
    }

    pub(crate) fn apply(
        &self,
        it: &mut IntegralFlags,
    ) {
        return match self {
            Self::Int { .. } => {}
            Self::Circular { .. } => profile_apply_circular(it),
            Self::Sequential { .. } => profile_apply_sequential(it),
        };
    }

    pub(crate) fn resets_flag(
        &self,
        name: &str,
    ) -> bool {
        return match self {
            Self::Circular { .. } => {
                matches!(name, "ord" | "partial_ord")
                    || RESTRICTED_MATH_FLAGS.contains(&name)
            }
            Self::Sequential { .. } => RESTRICTED_MATH_FLAGS.contains(&name),
            _ => false,
        };
    }
}

impl Parse for Profile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let mut profile = Self::from_name(name)?;

        let content;
        let _ = braced!(content in input);
        zz::parse_inner(&content, |attr, rest| {
            return profile.parse_attr(attr, rest);
        })?;

        return Ok(profile);
    }
}

#[derive(Default, Clone)]
pub(crate) struct Profiles(Vec<Profile>);

impl Profiles {
    pub(crate) fn one(it: Profile) -> Self {
        return Self(vec![it]);
    }

    fn effective(&self) -> Option<&Profile> {
        return self.0.last();
    }

    pub(crate) fn arithmetic(&self) -> Arithmetic {
        return self
            .effective()
            .map_or(Arithmetic::Primitive, Profile::arithmetic);
    }

    pub(crate) fn is_circular(&self) -> bool {
        return self.effective().is_some_and(Profile::is_circular);
    }

    pub(crate) fn is_default(&self) -> bool {
        return self.0.is_empty();
    }

    pub(crate) fn emits_precedes(&self) -> bool {
        return self.effective().is_some_and(Profile::emits_precedes);
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Profile> {
        return self.0.iter();
    }
}

impl Parse for Profiles {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return Ok(Self(zz::one_or_list::<Profile>(input)?.collect()));
    }
}

fn profile_apply_circular(it: &mut IntegralFlags) {
    profile_apply_restricted_math(it);
    it.ord = false;
    it.partial_ord = false;
}

fn profile_apply_sequential(it: &mut IntegralFlags) {
    profile_apply_restricted_math(it);
}

fn profile_apply_restricted_math(it: &mut IntegralFlags) {
    for name in RESTRICTED_MATH_FLAGS {
        it.set_bool(name, false)
            .expect("integral profile contains an unknown flag");
    }
}

// =============================================================================

pub(crate) struct FacetCfg {
    pub(crate) profile: Profiles,
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) ty_out: Option<SelfConverter>,
}

impl Debug for FacetCfg {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return f
            .debug_struct("FacetCfg")
            .field("profile", &self.profile.arithmetic())
            .field("friends", &self.friends)
            .field(
                "ty_out",
                &self.ty_out.as_ref().is_some_and(|it| !it.is_self()),
            )
            .finish();
    }
}

impl Parse for FacetCfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let mut profile = Profile::from_name(name)?;
        let mut friends = BTreeSet::new();
        let mut ty_out = None;

        let content;
        let _ = braced!(content in input);
        zz::parse_inner(&content, |attr, rest| {
            if profile.parse_attr(attr, rest)? {
                return Ok(true);
            }

            match attr {
                "friends" => {
                    // TODO unify friend config of int & flag,
                    // and then unify friendship for everything
                    // and finally unify this for everything:
                    friends = Cfg::parse_friends_unexpanded(rest)?;
                }
                "out_type" => ty_out = Some(rest.parse()?),
                _ => return Ok(false),
            }

            return Ok(true);
        })?;

        return Ok(Self {
            profile: Profiles::one(profile),
            friends,
            ty_out,
        });
    }
}
