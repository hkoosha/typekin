use crate::runner;
use crate::runner::{MkErr, mk_flags};
use crate::type_friendship::FriendReq;
use quote::ToTokens;
use std::fmt::{Debug, Formatter};
use syn::Path;
use syn::parse::{Parse, ParseStream};

mk_flags! {
    #[flag_default(bool=true, str="")]
    #[derive(Debug, Clone)]
    pub(crate) struct IntegralFlags {
        pub impl_range: bool = false,

        pub konst: bool,
        pub assertions: bool,

        pub impl_debug: bool,
        pub impl_eq: bool,
        pub impl_fmt_binary: bool,
        pub impl_fmt_hex_lower: bool,
        pub impl_fmt_hex_upper: bool,
        pub impl_fmt_octal: bool,
        pub impl_into: bool,
        pub impl_ord: bool,
        pub impl_partial_eq: bool,
        pub impl_partial_ord: bool,
        pub impl_try_into: bool,

        pub impl_assign_add: bool,
        pub impl_assign_and: bool,
        pub impl_assign_div: bool,
        pub impl_assign_mul: bool,
        pub impl_assign_or: bool,
        pub impl_assign_rem: bool,
        pub impl_assign_shl: bool,
        pub impl_assign_shr: bool,
        pub impl_assign_sub: bool,
        pub impl_math_add: bool,
        pub impl_math_and: bool,
        pub impl_math_div: bool,
        pub impl_math_mul: bool,
        pub impl_math_not: bool,
        pub impl_math_or: bool,
        pub impl_math_rem: bool,
        pub impl_math_shl: bool,
        pub impl_math_shr: bool,
        pub impl_math_sub: bool,
        pub impl_math_xor: bool,

        pub impl_friend: bool,
        pub impl_friend_make: bool,
        pub impl_friend_math_bit: bool,
        pub impl_friend_math_ops: bool,
        pub impl_friend_math_rel: bool,
        pub impl_friend_seal: bool,
        pub impl_friendzone_friend_make: bool,
        pub impl_friendzone_friend_math_bit: bool,
        pub impl_friendzone_friend_math_ops: bool,
        pub impl_friendzone_friend_math_rel: bool,
        pub impl_friendzone_seal: bool,

        pub fn_conv_into: bool,
        pub fn_conv_of: bool,
        pub fn_conv_raw: bool,
        pub fn_conv_try_into_checked: bool,
        pub fn_conv_try_into_unchecked: bool,
        pub fn_make_checked: bool,
        pub fn_make_checked_try: bool,
        pub fn_make_unchecked: bool,
        pub fn_make_unchecked_try: bool,
        pub fn_math_add: bool,
        pub fn_math_and: bool,
        pub fn_math_div: bool,
        pub fn_math_mul: bool,
        pub fn_math_not: bool,
        pub fn_math_or: bool,
        pub fn_math_rem: bool,
        pub fn_math_shl: bool,
        pub fn_math_shr: bool,
        pub fn_math_sub: bool,
        pub fn_math_xor: bool,
        pub fn_op_cmp: bool,
        pub fn_op_eq: bool,

        pub fp_unchecked: String = "Self::_unchecked",

        pub trait_seal: String = "Seal",
        pub trait_friend_make: String = "FriendMake",
        pub trait_friend_math: String = "FriendMath",
        pub trait_friend_bit: String = "FriendBit",
        pub trait_friend_rel: String = "FriendRel",

        pub fn_prefix_seal: String = "into_",
    }
}

#[derive(Default, Clone)]
pub(crate) struct IntegralCfg {
    pub(crate) flags: Box<IntegralFlags>,
    pub(crate) fn_get_raw: Option<Path>,
    pub(crate) fn_validator: Option<Path>,
    pub(crate) friends: Vec<FriendReq>,
}

impl Debug for IntegralCfg {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "IntegralCfg[int: {:?}, friends: {:?}, fn_get_raw: {}, fn_validator: {}",
            self.flags,
            self.friends,
            self.fn_get_raw
                .as_ref()
                .map(|it| it.to_token_stream().to_string())
                .unwrap_or_default(),
            self.fn_validator
                .as_ref()
                .map(|it| it.to_token_stream().to_string())
                .unwrap_or_default(),
        )
    }
}

impl Parse for IntegralCfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        runner::parse_inner_attributes(input, |attr, span, rest| {
            match attr {
                "with" => this.flags.parse_from(rest, true)?,
                "without" => this.flags.parse_from(rest, false)?,
                "friends" => this.friends = runner::list(rest)?.collect(),
                "fn_get_raw" => this.fn_get_raw = Some(rest.parse()?),
                "fn_validator" => this.fn_validator = Some(rest.parse()?),
                _ => return span.fail("unknown integral arg"),
            };

            return Ok(());
        })?;

        return Ok(this);
    }
}
