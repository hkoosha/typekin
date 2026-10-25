use proc_macro2::{
    Ident,
    Span,
};
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
    Integral {
        settings: IntSettings,
        span: Span,
    },
    Circular {
        settings: IntSettings,
        precedes: bool,
        span: Span,
    },
    Sequential {
        settings: IntSettings,
        span: Span,
    },
}

impl Profile {
    pub(crate) fn arithmetic(&self) -> Arithmetic {
        return match self {
            Self::Integral { .. } => Arithmetic::Primitive,
            Self::Circular { .. } => Arithmetic::Wrapping,
            Self::Sequential { .. } => Arithmetic::Checked,
        };
    }

    pub(crate) fn is_circular(&self) -> bool {
        return matches!(self, Self::Circular { .. });
    }

    pub(crate) fn span(&self) -> Span {
        return match self {
            Self::Integral { span, .. }
            | Self::Circular { span, .. }
            | Self::Sequential { span, .. } => *span,
        };
    }

    pub(crate) fn emits_precedes(&self) -> bool {
        return matches!(self, Self::Circular { precedes: true, .. });
    }

    pub(crate) fn int_settings(&self) -> &IntSettings {
        return match self {
            Self::Integral { settings, .. }
            | Self::Circular { settings, .. }
            | Self::Sequential { settings, .. } => settings,
        };
    }

    fn settings_mut(&mut self) -> &mut IntSettings {
        return match self {
            Self::Integral { settings, .. }
            | Self::Circular { settings, .. }
            | Self::Sequential { settings, .. } => settings,
        };
    }

    pub(crate) fn apply(
        &self,
        flags: &mut IntegralFlags,
    ) {
        return match self {
            Self::Integral { .. } => {}
            Self::Circular { .. } => profile_apply_circular(flags),
            Self::Sequential { .. } => profile_apply_sequential(flags),
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
        let mut profile = match snake_case_of(&name.to_string()).as_str() {
            "default" => Self::Integral {
                settings: IntSettings::default(),
                span: name.span(),
            },
            "circular" => Self::Circular {
                settings: IntSettings::default(),
                precedes: true,
                span: name.span(),
            },
            "sequential" => Self::Sequential {
                settings: IntSettings::default(),
                span: name.span(),
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

#[derive(Clone, Default)]
pub(crate) struct Profiles {
    items: Vec<Profile>,
}

impl Profiles {
    fn effective(&self) -> Option<&Profile> {
        return self.items.last();
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
        return self.items.is_empty();
    }

    pub(crate) fn span(&self) -> Span {
        return self
            .items
            .first()
            .map_or_else(Span::call_site, Profile::span);
    }

    pub(crate) fn emits_precedes(&self) -> bool {
        return self.effective().is_some_and(Profile::emits_precedes);
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Profile> {
        return self.items.iter();
    }
}

impl Parse for Profiles {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return Ok(Self {
            items: zz::one_or_list::<Profile>(input)?.collect(),
        });
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

const RESTRICTED_MATH_FLAGS: &[&str] = &[
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
];
