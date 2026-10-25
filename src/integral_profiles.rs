use std::str::FromStr;

use proc_macro2::Ident;
use syn::parse::ParseStream;
use syn::{
    LitBool,
    braced,
};

use crate::integral::{
    Cfg,
    IntegralFlags,
    Settings,
};
use crate::zz;
use crate::zz::UcFirst;
use crate::zz_runner::MkErr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProfileKind {
    Default,
    Circular,
    Sequential,
}

impl FromStr for ProfileKind {
    type Err = &'static str;

    fn from_str(it: &str) -> Result<Self, Self::Err> {
        return match it {
            "Default" => Ok(Self::Default),
            "Circular" => Ok(Self::Circular),
            "Sequential" => Ok(Self::Sequential),
            _ => Err("unknown profile"),
        };
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Arithmetic {
    Primitive,
    Wrapping,
    Checked,
}

#[derive(Clone)]
pub(crate) struct Profile {
    pub(crate) kind: ProfileKind,
    pub(crate) arithmetic: Arithmetic,
    pub(crate) circular_precedes: bool,
    pub(crate) settings: Settings,
}

impl Default for Profile {
    fn default() -> Self {
        return Self {
            kind: ProfileKind::Default,
            arithmetic: Arithmetic::Primitive,
            circular_precedes: false,
            settings: Default::default(),
        };
    }
}

impl Profile {
    fn arithmetic_for(it: ProfileKind) -> Arithmetic {
        return match it {
            ProfileKind::Default => Arithmetic::Primitive,
            ProfileKind::Circular => Arithmetic::Wrapping,
            ProfileKind::Sequential => Arithmetic::Checked,
        };
    }

    pub(crate) fn parse(it: ParseStream) -> syn::Result<Self> {
        let name = it.parse::<Ident>()?;

        let kind = ProfileKind::from_str(&name.to_string().ucfirst())
            .map_err(|err| name.errorful::<ProfileKind>(err))?;

        let content;
        let _ = braced!(content in it);
        let mut profile = Self {
            kind,
            arithmetic: Self::arithmetic_for(kind),
            circular_precedes: matches!(kind, ProfileKind::Circular),
            settings: Default::default(),
        };

        zz::parse_inner(&content, |attr, rest| {
            match attr {
                "with" => {
                    Cfg::parse_flags_into(rest, true, &mut profile.settings)?
                }
                "without" => {
                    Cfg::parse_flags_into(rest, false, &mut profile.settings)?
                }
                "precedes" if matches!(profile.kind, ProfileKind::Circular) => {
                    profile.circular_precedes = rest.parse::<LitBool>()?.value;
                }
                _ => return Ok(false),
            }

            return Ok(true);
        })?;

        return Ok(profile);
    }

    pub(crate) fn apply_defaults(
        &self,
        flags: &mut IntegralFlags,
    ) {
        if !matches!(self.kind, ProfileKind::Circular | ProfileKind::Sequential)
        {
            return;
        }

        for name in [
            "self_friend_math_ops",
            "self_friend_math_bit",
            "self_friend_rel",
            "self_bit",
            "bit_access",
            "core_int",
            "math_and",
            "math_div",
            "math_mul",
            "math_not",
            "math_or",
            "math_rem",
            "math_shl",
            "math_shr",
            "math_xor",
            "assign_and",
            "assign_div",
            "assign_mul",
            "assign_or",
            "assign_rem",
            "assign_shl",
            "assign_shr",
        ] {
            flags
                .set_bool(name, false)
                .expect("integral profile contains an unknown flag");
        }

        if matches!(self.kind, ProfileKind::Circular) {
            flags.ord = false;
            flags.partial_ord = false;
        }
    }
}
