use crate::integral::maker::FriendReq;
use crate::integral::maker::FriendshipLevel;
use crate::integral::maker::Generator;
use crate::integral::maker::IntegralCfg;
use crate::integral_token::err;
use crate::integral_token::find_unit_struct_inner_type;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::ExprPath;
use syn::ItemStruct;
use syn::LitBool;
use syn::Path;
use syn::Result;
use syn::Token;
use syn::bracketed;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;

#[derive(Debug, Clone, Copy)]
struct FriendshipLevelT(pub(crate) FriendshipLevel);

impl Parse for FriendshipLevelT {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;

        let it = match ident.to_string().as_str() {
            "None" => FriendshipLevel::None,
            "Make" => FriendshipLevel::Make,
            "Rel" => FriendshipLevel::Rel,
            "Bit" => FriendshipLevel::Bit,
            "Math" => FriendshipLevel::Math,
            "MathRel" => FriendshipLevel::MathRel,
            "MathBit" => FriendshipLevel::MathBit,
            "Full" => FriendshipLevel::Full,
            _ => return err(ident, "unknown friendship level"),
        };

        return Ok(Self(it));
    }
}

struct FriendReqT(pub(crate) FriendReq);

impl Parse for FriendReqT {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty: Path = input.parse()?;

        let mut level: Option<FriendshipLevel> = None;
        let mut conv: Option<ExprPath> = None;

        if input.peek(syn::token::Paren) {
            let args;
            syn::parenthesized!(args in input);

            while !args.is_empty() {
                let key: Ident = args.parse()?;
                let _: Token![=] = args.parse()?;

                match key.to_string().as_str() {
                    "level" => {
                        if level.is_some() {
                            return err(key, "duplicate arg");
                        }
                        level = Some(args.parse::<FriendshipLevelT>()?.0)
                    }
                    "conv" => {
                        if conv.is_some() {
                            return err(key, "duplicate arg");
                        }
                        conv = Some(args.parse()?)
                    }
                    _ => return err(key, "unknown arg"),
                }

                if args.peek(Token![,]) {
                    let _: Token![,] = args.parse()?;
                }
            }
        }

        let it = FriendReq {
            ty,
            conv,
            level: level.unwrap_or(FriendshipLevel::None),
        };

        return Ok(Self(it));
    }
}

pub(crate) struct IntegralCfgParser(IntegralCfg);

impl IntegralCfgParser {
    fn parse_friends(
        it: &mut IntegralCfg,
        input: &ParseStream,
    ) -> Result<()> {
        let content;
        bracketed!(content in input);

        it.friends =
            Punctuated::<FriendReqT, Token![,]>::parse_terminated(&content)?
                .into_iter()
                .map(|it| it.0)
                .collect::<Vec<_>>();

        return Ok(());
    }

    fn parse_get_raw_fn(
        it: &mut IntegralCfg,
        input: &ParseStream,
    ) -> Result<()> {
        it.fn_get_raw = input.parse()?;

        return Ok(());
    }

    fn parse_validator_fn(
        it: &mut IntegralCfg,
        input: &ParseStream,
    ) -> Result<()> {
        it.fn_validator = Some(input.parse()?);

        return Ok(());
    }

    fn parse_with_const(
        it: &mut IntegralCfg,
        input: &ParseStream,
    ) -> Result<()> {
        it.flags.with_const = input.parse::<LitBool>()?.value;

        return Ok(());
    }

    fn parse_attr(
        it: &mut IntegralCfg,
        input: &ParseStream,
        key: &str,
    ) -> Result<()> {
        const ARG_FRIENDS: &str = "friends";
        const ARG_VALIDATOR_FN: &str = "fn_validator";
        const ARG_WITH_CONST: &str = "with_const";
        const ARG_GET_RAW_FN: &str = "fn_get_raw";

        return match key {
            ARG_FRIENDS => Self::parse_friends(it, input),
            ARG_GET_RAW_FN => Self::parse_get_raw_fn(it, input),
            ARG_VALIDATOR_FN => Self::parse_validator_fn(it, input),
            ARG_WITH_CONST => Self::parse_with_const(it, input),
            _ => err(Span::call_site(), "unknown arg"),
        };
    }

    fn try_parse(input: ParseStream<'_>) -> Result<Self> {
        let mut seen = HashSet::with_capacity(5);
        let mut it = IntegralCfg::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;

            if seen.contains(key.to_string().as_str()) {
                return err(key, "duplicated arg");
            }
            else {
                seen.insert(key.to_string());
            }

            input.parse::<Token![=]>()?;

            Self::parse_attr(&mut it, &input, key.to_string().as_str())?;

            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }

        return Ok(Self(it));
    }
}

impl Parse for IntegralCfgParser {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        return Self::try_parse(input);
    }
}

pub(crate) fn ekran(
    args: IntegralCfgParser,
    input: ItemStruct,
) -> Result<TokenStream> {
    let el = find_unit_struct_inner_type(&input)?;

    let generator = Generator::new(&input, el, args.0)?;
    let stream = generator.ekran()?;

    let mut fin = quote! { #input };
    fin.extend(stream);
    return Ok(fin);
}
