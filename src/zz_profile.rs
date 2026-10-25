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

#[derive(Clone, Default)]
pub(crate) enum Profile {
    #[default]
    Default,
    Integral {
        settings: IntSettings,
        span: Option<Span>,
    },
    Circular {
        settings: IntSettings,
        precedes: bool,
        span: Option<Span>,
    },
    Sequential {
        settings: IntSettings,
        span: Option<Span>,
    },
}

impl Profile {
    fn unresolved() -> ! {
        unreachable!("default profiles must be resolved");
    }

    fn ensure_resolved(&self) {
        if matches!(self, Self::Default) {
            Self::unresolved();
        }
    }

    pub(crate) fn arithmetic(&self) -> Arithmetic {
        return match self {
            Self::Default => Self::unresolved(),
            Self::Integral { .. } => Arithmetic::Primitive,
            Self::Circular { .. } => Arithmetic::Wrapping,
            Self::Sequential { .. } => Arithmetic::Checked,
        };
    }

    pub(crate) fn is_circular(&self) -> bool {
        self.ensure_resolved();

        return matches!(self, Self::Circular { .. });
    }

    pub(crate) fn is_default(&self) -> bool {
        return matches!(self, Self::Default);
    }

    pub(crate) fn span(&self) -> Span {
        return match self {
            Self::Default => Span::call_site(),

            Self::Integral { span, .. }
            | Self::Circular { span, .. }
            | Self::Sequential { span, .. } => {
                span.unwrap_or_else(Span::call_site)
            }
        };
    }

    pub(crate) fn emits_precedes(&self) -> bool {
        self.ensure_resolved();

        return matches!(self, Self::Circular { precedes: true, .. });
    }

    pub(crate) fn int_settings(&self) -> &IntSettings {
        return match self {
            Self::Default => Self::unresolved(),

            Self::Integral { settings, .. }
            | Self::Circular { settings, .. }
            | Self::Sequential { settings, .. } => settings,
        };
    }

    fn settings_mut(&mut self) -> &mut IntSettings {
        return match self {
            Self::Default => Self::unresolved(),

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
            Self::Default => Self::unresolved(),

            Self::Integral { .. } => {}
            Self::Circular { .. } => profile_apply_circular(flags),
            Self::Sequential { .. } => profile_apply_sequential(flags),
        };
    }

    pub(crate) fn resolve(&mut self) -> &mut Self {
        if matches!(self, Self::Default) {
            *self = Self::Integral {
                settings: IntSettings::default(),
                span: None,
            };
        }

        return self;
    }
}

impl Parse for Profile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let mut profile = match snake_case_of(&name.to_string()).as_str() {
            "default" => Self::Integral {
                settings: IntSettings::default(),
                span: Some(name.span()),
            },
            "circular" => Self::Circular {
                settings: IntSettings::default(),
                precedes: true,
                span: Some(name.span()),
            },
            "sequential" => Self::Sequential {
                settings: IntSettings::default(),
                span: Some(name.span()),
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
