use crate::flag::maker::FriendReq;
use crate::flag::maker::FriendshipLevel;
use crate::flag::maker::Generator;
use crate::flag::maker::IntegralCfg;
use crate::integral_types::N;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::Fields;
use syn::ItemStruct;
use syn::LitBool;
use syn::Result;
use syn::Token;
use syn::Type;
use syn::bracketed;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;

pub(crate) fn ekran(
    args: IntegralCfgParser,
    input: ItemStruct,
) -> Result<TokenStream> {
    let el = find_inner_type(&input)?;

    let generator = Generator::new(&input, el, args.0)?;
    let stream = generator.ekran()?;

    let mut fin = quote! { #input };
    fin.extend(stream);
    return Ok(fin);
}
