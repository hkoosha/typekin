use crate::integral::cfg::IntegralCfg;
use crate::runner;
use crate::runner::mk_flags;
use crate::type_friendship::FriendReq;
use syn::bracketed;
use syn::parse::Parse;
use syn::parse::ParseStream;

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
        let mut has_integral = false;

        runner::parse_inner_attributes(input, |attr, rest| {
            match attr {
                "with" => this.bit.parse_from(rest, true)?,
                "without" => this.bit.parse_from(rest, false)?,
                "friends" => this.friends = runner::list(rest)?.collect(),
                "value_name_suffix" => {
                    let as_str: syn::LitStr = input.parse()?;
                    if as_str.value().is_empty() {
                        this.bit.value_name_suffix = "".to_string();
                    }
                    else {
                        let as_idn: syn::Ident =
                            syn::parse_str(&as_str.value())?;
                        this.bit.value_name_suffix = as_idn.to_string();
                    }
                }
                "value_name" => {
                    let as_str: syn::LitStr = input.parse()?;
                    let as_idn: syn::Ident = syn::parse_str(&as_str.value())?;
                    this.bit.value_name = as_idn.to_string();
                }
                "integral" => {
                    let content;
                    let _ = bracketed!(content in input);
                    let cfg = IntegralCfg::parse(&content)?;
                    this.int = Box::new(cfg);
                    has_integral = true;
                }
                _ => {}
            };

            return Ok(true);
        })?;

        if !has_integral {
            return Err(syn::Error::new(
                input.span(),
                "missing required `konst` argument in `integral = [...]`",
            ));
        }

        return Ok(this);
    }
}

#[cfg(test)]
mod tests {
    use super::BitflagCfg;

    #[test]
    fn accepts_explicit_nested_konst() {
        let config = syn::parse_str::<BitflagCfg>("integral = [konst = false]")
            .expect("explicit nested konst should parse");
        assert!(!config.int.konst);
    }

    #[test]
    fn rejects_missing_konst() {
        let error = match syn::parse_str::<BitflagCfg>("friends = [u8]") {
            Ok(_) => panic!("konst must be explicit"),
            Err(error) => error,
        };
        assert_eq!(
            error.to_string(),
            "missing required `konst` argument in `integral = [...]`",
        );
    }
}
