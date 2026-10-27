use std::collections::BTreeSet;
use std::fmt::{
    Debug,
    Formatter,
};

use proc_macro2::{
    Ident,
    Span,
    TokenStream,
};
use quote::{
    ToTokens,
    format_ident,
    quote,
};
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::{
    LitBool,
    Path,
    braced,
};

use crate::friendship::cfg::Friend;
use crate::integral::{
    Cfg,
    IntSettings,
    IntegralFlags,
};
use crate::zz_conv::SelfConverter;
use crate::zz_lib as zz;
use crate::zz_runner::{
    MkErr,
    MySpanned,
    snake_case_of,
};
use crate::zz_value_type::{
    ArithmeticOp,
    Operator,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Arithmetic {
    Primitive,
    PrimitiveWrapping,
    WrapperWrapping,
    Checked,
}

impl Arithmetic {
    pub(crate) fn make_expr(
        self,
        it: ArithmeticOp,
    ) -> TokenStream {
        let primitive = match it {
            ArithmeticOp::Add => quote! { lhs + rhs },
            ArithmeticOp::Sub => quote! { lhs - rhs },
            ArithmeticOp::Mul => quote! { lhs * rhs },
            ArithmeticOp::Div => quote! { lhs / rhs },
            ArithmeticOp::Rem => quote! { lhs % rhs },
            ArithmeticOp::Shl => quote! { lhs << rhs },
            ArithmeticOp::Shr => quote! { lhs >> rhs },
        };

        return match self {
            Arithmetic::Primitive => primitive,
            Arithmetic::PrimitiveWrapping => {
                let method = format_ident!("wrapping_{}", it.name());
                let rhs = Self::rhs_for(it);
                quote! { lhs.#method(#rhs) }
            }
            Arithmetic::WrapperWrapping => {
                let rhs = match it {
                    ArithmeticOp::Shl | ArithmeticOp::Shr => quote! { rhs },
                    ArithmeticOp::Add
                    | ArithmeticOp::Sub
                    | ArithmeticOp::Mul
                    | ArithmeticOp::Div
                    | ArithmeticOp::Rem => {
                        quote! { ::core::num::Wrapping(rhs) }
                    }
                };
                match it {
                    ArithmeticOp::Add => {
                        quote! {
                            (
                                ::core::num::Wrapping(lhs)
                                    + #rhs
                            ).0
                        }
                    }
                    ArithmeticOp::Sub => {
                        quote! {
                            (
                                ::core::num::Wrapping(lhs)
                                    - #rhs
                            ).0
                        }
                    }
                    ArithmeticOp::Mul => {
                        quote! {
                            (
                                ::core::num::Wrapping(lhs)
                                    * #rhs
                            ).0
                        }
                    }
                    ArithmeticOp::Div => {
                        quote! {
                            (
                                ::core::num::Wrapping(lhs)
                                    / #rhs
                            ).0
                        }
                    }
                    ArithmeticOp::Rem => {
                        quote! {
                            (
                                ::core::num::Wrapping(lhs)
                                    % #rhs
                            ).0
                        }
                    }
                    ArithmeticOp::Shl => {
                        quote! {
                            (
                                ::core::num::Wrapping(lhs)
                                    << #rhs
                            ).0
                        }
                    }
                    ArithmeticOp::Shr => {
                        quote! {
                            (
                                ::core::num::Wrapping(lhs)
                                    >> #rhs
                            ).0
                        }
                    }
                }
            }
            Arithmetic::Checked => {
                let method = format_ident!("checked_{}", it.name());
                let rhs = Self::rhs_for(it);
                quote! {
                    match lhs.#method(#rhs) {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => {
                            ::core::panic!(
                                "operation overflowed",
                            );
                        }
                    }
                }
            }
        };
    }

    pub(crate) fn make_const_expr(
        self,
        it: ArithmeticOp,
    ) -> TokenStream {
        return match self {
            Self::WrapperWrapping => Self::PrimitiveWrapping.make_expr(it),
            _ => self.make_expr(it),
        };
    }

    pub(crate) fn make_operator_expr(
        self,
        operator: Operator,
        is_konst: bool,
    ) -> TokenStream {
        let make_expr = |operation| {
            return if is_konst {
                self.make_const_expr(operation)
            }
            else {
                self.make_expr(operation)
            };
        };
        return match operator {
            Operator::BitAnd => quote! { lhs & rhs },
            Operator::BitOr => quote! { lhs | rhs },
            Operator::BitXor => quote! { lhs ^ rhs },
            Operator::Add => make_expr(ArithmeticOp::Add),
            Operator::Sub => make_expr(ArithmeticOp::Sub),
            Operator::Mul => make_expr(ArithmeticOp::Mul),
            Operator::Div => make_expr(ArithmeticOp::Div),
            Operator::Rem => make_expr(ArithmeticOp::Rem),
        };
    }

    fn rhs_for(operation: ArithmeticOp) -> TokenStream {
        return match operation {
            ArithmeticOp::Shl | ArithmeticOp::Shr => quote! { rhs as u32 },
            ArithmeticOp::Add
            | ArithmeticOp::Sub
            | ArithmeticOp::Mul
            | ArithmeticOp::Div
            | ArithmeticOp::Rem => quote! { rhs },
        };
    }
}

#[derive(Clone, Default)]
pub(crate) struct Int {
    settings: IntSettings,
}

#[derive(Clone)]
pub(crate) struct Circular {
    settings: IntSettings,
    precedes: bool,
}

impl Default for Circular {
    fn default() -> Self {
        return Self {
            settings: Default::default(),
            precedes: true,
        };
    }
}

impl Circular {
    fn without_precedes() -> Self {
        return Self {
            settings: Default::default(),
            precedes: false,
        };
    }
}

#[derive(Clone, Default)]
pub(crate) struct Sequential {
    settings: IntSettings,
}

#[derive(Clone, Default)]
pub(crate) struct Wrapping {
    settings: IntSettings,
}

#[derive(Clone, Default)]
pub(crate) struct Point {
    settings: IntSettings,
}

#[derive(Clone)]
pub(crate) enum Profile {
    Int(Int),
    Circular(Circular),
    Delta(Circular),
    Sequential(Sequential),
    Wrapping(Wrapping),
    Point(Point),
}

impl Profile {
    pub(crate) fn from_name(it: Ident) -> syn::Result<Self> {
        let this = match snake_case_of(&it.to_string()).as_str() {
            "int" => Self::Int(Int::default()),
            "circular" => Self::Circular(Circular::default()),
            "delta" => Self::Delta(Circular::without_precedes()),
            "sequential" => Self::Sequential(Sequential::default()),
            "wrapping" => Self::Wrapping(Wrapping::default()),
            "point" => Self::Point(Point::default()),
            _ => return it.fail("unknown profile"),
        };

        return Ok(this);
    }

    pub(crate) fn parse_attr(
        &mut self,
        attr: &str,
        rest: ParseStream,
    ) -> syn::Result<bool> {
        match attr {
            "with" => Cfg::parse_flags_into(rest, true, self.settings_mut())?,
            "without" => {
                Cfg::parse_flags_into(rest, false, self.settings_mut())?
            }
            "precedes" => {
                let Self::Circular(Circular { precedes, .. }) = self
                else {
                    return Ok(false);
                };
                *precedes = rest.parse::<LitBool>()?.value;
            }
            _ => return Ok(false),
        }

        return Ok(true);
    }

    pub(crate) fn arithmetic(&self) -> Option<Arithmetic> {
        return match self {
            Self::Int { .. } => Some(Arithmetic::Primitive),
            Self::Circular { .. } | Self::Delta(_) => {
                Some(Arithmetic::PrimitiveWrapping)
            }
            Self::Sequential { .. } => Some(Arithmetic::Checked),
            Self::Wrapping { .. } => Some(Arithmetic::WrapperWrapping),
            Self::Point { .. } => None,
        };
    }

    pub(crate) fn is_circular(&self) -> bool {
        return matches!(self, Self::Circular(_) | Self::Delta(_));
    }

    pub(crate) fn emits_precedes(&self) -> bool {
        return matches!(self, Self::Circular(Circular { precedes: true, .. }));
    }

    pub(crate) fn int_settings(&self) -> &IntSettings {
        return match self {
            Self::Int(Int { settings, .. })
            | Self::Circular(Circular { settings, .. })
            | Self::Delta(Circular { settings, .. })
            | Self::Sequential(Sequential { settings, .. })
            | Self::Wrapping(Wrapping { settings, .. })
            | Self::Point(Point { settings, .. }) => settings,
        };
    }

    fn settings_mut(&mut self) -> &mut IntSettings {
        return match self {
            Self::Int(Int { settings, .. })
            | Self::Circular(Circular { settings, .. })
            | Self::Delta(Circular { settings, .. })
            | Self::Sequential(Sequential { settings, .. })
            | Self::Wrapping(Wrapping { settings, .. })
            | Self::Point(Point { settings, .. }) => settings,
        };
    }

    pub(crate) fn apply(
        &self,
        it: &mut IntegralFlags,
    ) {
        return match self {
            Self::Int { .. } => {}
            Self::Circular { .. } | Self::Delta(_) => {
                profile_apply_circular(it)
            }
            Self::Sequential { .. } => profile_apply_sequential(it),
            Self::Wrapping { .. } => {}
            Self::Point { .. } => profile_apply_point(it),
        };
    }

    pub(crate) fn resets_flag(
        &self,
        name: &str,
    ) -> bool {
        return match self {
            Self::Circular(_) | Self::Delta(_) => {
                matches!(name, "ord" | "partial_ord")
                    || IntegralFlags::restricted_math_flags().contains(&name)
            }
            Self::Sequential { .. } => {
                IntegralFlags::restricted_math_flags().contains(&name)
            }
            Self::Point { .. } => {
                matches!(name, "math_add" | "assign_add")
                    || IntegralFlags::restricted_math_flags().contains(&name)
            }
            _ => false,
        };
    }
}

impl Parse for Profile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let mut profile = Self::from_name(name)?;

        if input.peek(syn::token::Brace) {
            let content;
            let _ = braced!(content in input);
            zz::parse_inner(&content, |attr, rest| {
                return profile.parse_attr(attr, rest);
            })?;
        }

        return Ok(profile);
    }
}

#[derive(Default, Clone)]
pub(crate) struct Profiles(Vec<Profile>);

impl Profiles {
    pub(crate) fn one(it: Profile) -> Self {
        return Self(vec![it]);
    }

    fn effective(&self) -> Option<&Profile> {
        return self
            .0
            .iter()
            .rev()
            .find(|profile| profile.arithmetic().is_some());
    }

    pub(crate) fn arithmetic(&self) -> Arithmetic {
        return self.arithmetic_or(Arithmetic::Primitive);
    }

    pub(crate) fn arithmetic_or(
        &self,
        fallback: Arithmetic,
    ) -> Arithmetic {
        return self
            .0
            .iter()
            .rev()
            .find_map(Profile::arithmetic)
            .unwrap_or(fallback);
    }

    pub(crate) fn requires_wrapping_storage(&self) -> bool {
        return self
            .effective()
            .is_some_and(|profile| matches!(profile, Profile::Wrapping(_)));
    }

    pub(crate) fn is_circular(&self) -> bool {
        return self.effective().is_some_and(Profile::is_circular);
    }

    pub(crate) fn is_default(&self) -> bool {
        return self.0.is_empty();
    }

    pub(crate) fn emits_precedes(&self) -> bool {
        return self.effective().is_some_and(Profile::emits_precedes);
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Profile> {
        return self.0.iter();
    }

    pub(crate) fn single_mut(&mut self) -> &mut Profile {
        assert_eq!(
            self.0.len(),
            1,
            "fixed profile configuration needs one profile"
        );
        return self.0.first_mut().expect("fixed profile cannot be empty");
    }
}

impl Parse for Profiles {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return Ok(Self(zz::one_or_list::<Profile>(input)?.collect()));
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

fn profile_apply_point(it: &mut IntegralFlags) {
    profile_apply_restricted_math(it);
    it.math_add = false;
    it.assign_add = false;
}

fn profile_apply_restricted_math(it: &mut IntegralFlags) {
    for name in IntegralFlags::restricted_math_flags() {
        it.set_bool(name, false)
            .expect("integral profile contains an unknown flag");
    }
}

// =============================================================================

#[derive(Clone)]
pub(crate) struct FacetCfg {
    pub(crate) profile: Profiles,
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) ty_out: Option<SelfConverter>,
    pub(crate) precedes_span: Option<Span>,
}

impl Debug for FacetCfg {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return f
            .debug_struct("FacetCfg")
            .field("profile", &self.profile.arithmetic())
            .field("friends", &self.friends)
            .field(
                "ty_out",
                &self.ty_out.as_ref().is_some_and(|it| !it.is_self()),
            )
            .field("precedes", &self.precedes_span.is_some())
            .finish();
    }
}

impl Parse for FacetCfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let mut profile = Profile::from_name(name)?;
        let mut friends = BTreeSet::new();
        let mut ty_out = None;
        let mut precedes_span = None;

        if input.peek(syn::token::Brace) {
            let content;
            let _ = braced!(content in input);
            zz::parse_inner(&content, |attr, rest| {
                if attr == "precedes" {
                    precedes_span = Some(rest.span());
                }
                if profile.parse_attr(attr, rest)? {
                    return Ok(true);
                }

                match attr {
                    "friend" => {
                        // Facets select Math and Bit operations independently.
                        friends = Cfg::parse_friends(rest, false)?;
                    }
                    "produce" => ty_out = Some(rest.parse()?),
                    _ => return Ok(false),
                }

                return Ok(true);
            })?;
        }

        return Ok(Self {
            profile: Profiles::one(profile),
            friends,
            ty_out,
            precedes_span,
        });
    }
}

impl FacetCfg {
    pub(crate) fn requires_wrapping_storage(&self) -> bool {
        return self.profile.requires_wrapping_storage();
    }

    pub(crate) fn explicit_flag(
        &self,
        name: &str,
    ) -> Option<(bool, Span)> {
        return self
            .profile
            .iter()
            // TODO untangle from integral
            .filter_map(|profile| profile.int_settings().flags.get(name))
            .copied()
            .last();
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct FacetSlotKey {
    pub(crate) operator: Operator,
    pub(crate) rhs: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FacetOwner {
    Default,
    Explicit(usize),
}

#[derive(Clone)]
pub(crate) struct FacetSlot {
    pub(crate) rhs: Option<Path>,
    pub(crate) conversion: Option<Path>,
    pub(crate) arithmetic: Arithmetic,
    pub(crate) ty_out: SelfConverter,
    pub(crate) owner: FacetOwner,
    pub(crate) span: Span,
}

impl MySpanned for FacetSlot {
    fn my_span(&self) -> Span {
        return self.span;
    }
}

impl Debug for FacetSlot {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return f
            .debug_struct("FacetSlot")
            .field("rhs", &self.rhs.as_ref().map(ToTokens::to_token_stream))
            .field(
                "conversion",
                &self.conversion.as_ref().map(ToTokens::to_token_stream),
            )
            .field("arithmetic", &self.arithmetic)
            .field("ty_out", &!self.ty_out.is_self())
            .field("owner", &self.owner)
            .finish_non_exhaustive();
    }
}

pub(crate) struct DefaultFacetCfg {
    pub(crate) profile: Profiles,
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) flags: Box<IntegralFlags>,
    pub(crate) ty_out: SelfConverter,
}

impl Debug for DefaultFacetCfg {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return f
            .debug_struct("DefaultFacetCfg")
            .field("profile", &self.profile.arithmetic())
            .field("friends", &self.friends)
            .field("flags", &self.flags)
            .field("ty_out", &!self.ty_out.is_self())
            .finish();
    }
}
