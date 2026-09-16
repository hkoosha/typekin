use crate::runner;
use crate::runner::MkErr;
use proc_macro2::Ident;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Formatter;
use syn::Path;
use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum FriendshipLevel {
    None,
    Make,
    Rel,
    Bit,
    Math,
    Full,
    XCustom(String),
}

impl FriendshipLevel {
    fn default_level() -> HashSet<Self> {
        let mut set = HashSet::with_capacity(4);

        for it in [Self::Make, Self::Rel, Self::Bit, Self::Math] {
            set.insert(it);
        }

        return set;
    }

    pub(crate) fn normalize(&self) -> HashSet<Self> {
        match self {
            Self::None => vec![],
            Self::Full => vec![Self::Make, Self::Rel, Self::Bit, Self::Math],
            _ => vec![self.clone()],
        }
        .into_iter()
        .collect()
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
            x_custom => {
                if !x_custom.starts_with("X") {
                    return ident.fail("custom level must start with `X`");
                }
                Self::XCustom(x_custom.to_string())
            }
        };

        return Ok(it);
    }
}

// -------------------------------------

#[derive(Clone)]
pub(crate) struct FriendReq {
    pub(crate) ty: Path,
    pub(crate) level: HashSet<FriendshipLevel>,
    pub(crate) conv: Option<Path>,
}

impl Debug for FriendReq {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "FriendReq[{:?}]", self.level)
    }
}

impl Parse for FriendReq {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Path = input.parse()?;

        let mut level = HashSet::<FriendshipLevel>::with_capacity(2);
        let mut conv = None::<Path>;

        runner::parse_optional_attributes(input, |name, stream| {
            match name {
                "level" => level = runner::list(stream)?.collect(),
                "conv" => conv = Some(stream.parse()?),
                _ => return stream.span().fail("unknown arg"),
            }

            return Ok(true);
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
