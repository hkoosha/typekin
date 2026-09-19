use crate::{
    runner,
    runner::MkErr,
};
use proc_macro2::Ident;

use quote::ToTokens;
use std::cmp::Ordering;
use std::{
    collections::BTreeSet,
    fmt::{
        Debug,
        Formatter,
    },
};
use syn::{
    Path,
    parse::{
        Parse,
        ParseStream,
    },
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
    fn default_level() -> BTreeSet<Self> {
        let mut set = BTreeSet::new();

        for it in [Self::Make, Self::Rel, Self::Bit, Self::Math] {
            set.insert(it);
        }

        return set;
    }

    pub(crate) fn normalize(&self) -> BTreeSet<Self> {
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
    repr: String,
    pub(crate) ty: Path,
    pub(crate) level: BTreeSet<FriendshipLevel>,
    pub(crate) conv: Option<Path>,
}

impl FriendReq {
    pub(crate) fn new(
        ty: Path,
        level: BTreeSet<FriendshipLevel>,
        conv: Option<Path>,
    ) -> Self {
        return Self {
            repr: ty.to_token_stream().to_string(),
            ty,
            conv,
            level,
        };
    }
}

impl Eq for FriendReq {}

impl PartialOrd for FriendReq {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FriendReq {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        return self.repr == other.repr;
    }
}

impl Ord for FriendReq {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        return self.repr.cmp(&other.repr);
    }
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

        let mut level = BTreeSet::<FriendshipLevel>::new();
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

        let it = FriendReq::new(ty, level, conv);

        return Ok(it);
    }
}
