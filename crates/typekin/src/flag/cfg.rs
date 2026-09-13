use crate::integral::cfg::IntegralCfg;
use crate::runner;
use crate::runner::{MkErr, mk_flags};
use crate::type_friendship::FriendReq;
use syn::parse::{Parse, ParseStream};

mk_flags! {
    #[flag_default(bool=true, str="")]
    #[derive(Debug, Clone)]
    pub(crate) struct BitFlags {
        pub make_value: bool,
        pub impl_value: bool,
        pub value_name: String,
        pub value_name_suffix: String = "Value",

        pub trait_seal: String = "BitSeal",
        pub trait_friend_make: String = "BitFriendMake",
        pub trait_friend_math: String = "BitFriendMath",
        pub trait_friend_bit: String = "BitFriendBit",
        pub trait_friend_rel: String = "BitFriendRel",
    }
}

#[derive(Default)]
pub(crate) struct BitflagCfg {
    pub(crate) friends: Vec<FriendReq>,
    pub(crate) bit: Box<BitFlags>,
    pub(crate) int: Box<IntegralCfg>,
}

impl Parse for BitflagCfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        runner::parse_inner_attributes(input, |attr, span, rest| {
            match attr {
                "with" => this.bit.parse_from(rest, true)?,
                "without" => this.bit.parse_from(rest, false)?,
                "friends" => this.friends = runner::list(rest)?.collect(),
                "integral" => {
                    this.int = Box::new(IntegralCfg::parse(
                        &runner::unbracket(&input)?,
                    )?)
                }
                _ => return span.fail("unknown bitflag arg"),
            };

            return Ok(());
        })?;

        return Ok(this);
    }
}
