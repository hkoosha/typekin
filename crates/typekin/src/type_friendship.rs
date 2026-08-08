use crate::runner;
use crate::runner::MkErr;
use proc_macro2::Ident;
use std::collections::HashSet;
use syn::ExprPath;
use syn::Path;
use syn::parse::{Parse, ParseStream};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum FriendshipLevel {
    None,
    Make,
    Rel,
    Bit,
    Math,
    Full,
    Custom(String),
}

impl FriendshipLevel {
    fn default_level() -> HashSet<Self> {
        let mut set = HashSet::with_capacity(4);

        for it in [Self::Make, Self::Rel, Self::Bit, Self::Math] {
            set.insert(it);
        }

        return set;
    }

    pub(crate) fn to_set(self) -> HashSet<Self> {
        let mut set = HashSet::with_capacity(1);
        set.insert(self);
        return set;
    }
}

impl Parse for FriendshipLevel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        let it = match ident.to_string().as_str() {
            "None" => Self::None,
            "Make" => Self::Make,
            "Rel" => Self::Rel,
            "Bit" => Self::Bit,
            "Math" => Self::Math,
            "Full" => Self::Full,
            custom => Self::Custom(custom.to_string()),
        };

        return Ok(it);
    }
}

// -------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct FriendReq {
    pub(crate) ty: Path,
    pub(crate) level: HashSet<FriendshipLevel>,
    pub(crate) conv: Option<ExprPath>,
}

impl Parse for FriendReq {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Path = input.parse()?;

        let mut level = HashSet::<FriendshipLevel>::with_capacity(2);
        let mut conv = None::<ExprPath>;

        runner::parse_optional_attributes(input, |name, span, stream| {
            match name {
                "level" => level = runner::list(stream)?.collect(),
                "conv" => conv = Some(stream.parse()?),
                _ => return span.fail("unknown arg"),
            }

            return Ok(());
        })?;

        if level.contains(&FriendshipLevel::Full) {
            level.extend(FriendshipLevel::default_level());
            level.remove(&FriendshipLevel::Full);
        }

        if level.is_empty() {
            level = FriendshipLevel::default_level();
        }

        level.remove(&FriendshipLevel::None);

        let it = FriendReq { ty, conv, level };

        return Ok(it);
    }
}
