use proc_macro2::Ident;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::{
    LitBool,
    braced,
};

use crate::integral::{
    Cfg,
    IntSettings,
    IntegralFlags,
};
use crate::zz;
use crate::zz_runner::{
    MkErr,
    snake_case_of,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Arithmetic {
    Primitive,
    Wrapping,
    Checked,
}

#[derive(Clone)]
pub(crate) enum Profile {
    Default {
        settings: IntSettings,
    },
    Circular {
        settings: IntSettings,
        precedes: bool,
    },
    Sequential {
        settings: IntSettings,
    },
}

impl Default for Profile {
    fn default() -> Self {
        return Self::Default {
            settings: Default::default(),
        };
    }
}

impl Profile {
    pub(crate) fn arithmetic(&self) -> Arithmetic {
        return match self {
            Self::Default { .. } => Arithmetic::Primitive,
            Self::Circular { .. } => Arithmetic::Wrapping,
            Self::Sequential { .. } => Arithmetic::Checked,
        };
    }

    pub(crate) fn is_circular(&self) -> bool {
        return matches!(self, Self::Circular { .. });
    }

    pub(crate) fn emits_precedes(&self) -> bool {
        return match self {
            Self::Circular { precedes, .. } => *precedes,
            Self::Default { .. } | Self::Sequential { .. } => false,
        };
    }

    pub(crate) fn int_settings(&self) -> &IntSettings {
        return match self {
            Self::Default { settings }
            | Self::Circular { settings, .. }
            | Self::Sequential { settings } => settings,
        };
    }

    fn settings_mut(&mut self) -> &mut IntSettings {
        return match self {
            Self::Default { settings }
            | Self::Circular { settings, .. }
            | Self::Sequential { settings } => settings,
        };
    }

    pub(crate) fn apply_defaults(
        &self,
        flags: &mut IntegralFlags,
    ) {
        return match self {
            Self::Default { .. } => profile_apply_default(flags),
            Self::Circular { .. } => profile_apply_circular(flags),
            Self::Sequential { .. } => profile_apply_sequential(flags),
        };
    }
}

impl Parse for Profile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let mut profile = match snake_case_of(&name.to_string()).as_str() {
            "default" => Self::Default {
                settings: Default::default(),
            },
            "circular" => Self::Circular {
                settings: Default::default(),
                precedes: true,
            },
            "sequential" => Self::Sequential {
                settings: Default::default(),
            },
            _ => return Err(name.errorful::<Self>("unknown profile")),
        };

        let content;
        let _ = braced!(content in input);
        zz::parse_inner(&content, |attr, rest| {
            match attr {
                "with" => {
                    Cfg::parse_flags_into(rest, true, profile.settings_mut())?
                }
                "without" => {
                    Cfg::parse_flags_into(rest, false, profile.settings_mut())?
                }
                "precedes" => {
                    let Self::Circular { precedes, .. } = &mut profile
                    else {
                        return Ok(false);
                    };
                    *precedes = rest.parse::<LitBool>()?.value;
                }
                _ => return Ok(false),
            }

            return Ok(true);
        })?;

        return Ok(profile);
    }
}

fn profile_apply_default(_: &mut IntegralFlags) {}

fn profile_apply_circular(it: &mut IntegralFlags) {
    profile_apply_restricted_math(it);
    it.ord = false;
    it.partial_ord = false;
}

fn profile_apply_sequential(it: &mut IntegralFlags) {
    profile_apply_restricted_math(it);
}

fn profile_apply_restricted_math(it: &mut IntegralFlags) {
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
        it.set_bool(name, false)
            .expect("integral profile contains an unknown flag");
    }
}
