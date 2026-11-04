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

use crate::integral_cfg::{
    GRAMMAR,
    IntegralFlag,
    IntegralFlags,
};
use crate::zz_conv::SelfConverter;
use crate::zz_flags::FlagSettings;
use crate::zz_friend::{
    FACET,
    Friend,
};
use crate::zz_lib::{
    self as zz,
    Key,
};
use crate::zz_runner::{
    MkErr,
    MySpanned,
    snake_case_of,
};
use crate::zz_values::{
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
        let symbol = it.symbol();
        return match self {
            Arithmetic::Primitive => quote! { lhs #symbol rhs },
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
                quote! {
                    (
                        ::core::num::Wrapping(lhs)
                            #symbol #rhs
                    ).0
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
        return match operator.arithmetic_op() {
            Some(operation) => make_expr(operation),
            None => {
                let symbol = operator.symbol();
                quote! { lhs #symbol rhs }
            }
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
    settings: FlagSettings<IntegralFlag>,
}

#[derive(Clone)]
pub(crate) struct Circular {
    settings: FlagSettings<IntegralFlag>,
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
    settings: FlagSettings<IntegralFlag>,
}

#[derive(Clone, Default)]
pub(crate) struct Wrapping {
    settings: FlagSettings<IntegralFlag>,
}

#[derive(Clone, Default)]
pub(crate) struct Point {
    settings: FlagSettings<IntegralFlag>,
}

/// A profile name: `profile = NAME`, a facet, or a fixed-profile macro.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ProfileKind {
    Int,
    Circular,
    Delta,
    Sequential,
    Wrapping,
    Point,
}

impl ProfileKind {
    const ITEMS: [Self; 6] = [
        Self::Int,
        Self::Circular,
        Self::Delta,
        Self::Sequential,
        Self::Wrapping,
        Self::Point,
    ];

    pub(crate) fn name(self) -> &'static str {
        return match self {
            Self::Int => "int",
            Self::Circular => "circular",
            Self::Delta => "delta",
            Self::Sequential => "sequential",
            Self::Wrapping => "wrapping",
            Self::Point => "point",
        };
    }

    pub(crate) fn of(name: &str) -> Option<Self> {
        return Self::ITEMS.into_iter().find(|it| it.name() == name);
    }
}

impl Parse for ProfileKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        return match Self::of(&snake_case_of(&name.to_string())) {
            Some(it) => Ok(it),
            None => name.fail("unknown profile"),
        };
    }
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
    pub(crate) fn from_name(kind: ProfileKind) -> Self {
        return match kind {
            ProfileKind::Int => Self::Int(Int::default()),
            ProfileKind::Circular => Self::Circular(Circular::default()),
            ProfileKind::Delta => Self::Delta(Circular::without_precedes()),
            ProfileKind::Sequential => Self::Sequential(Sequential::default()),
            ProfileKind::Wrapping => Self::Wrapping(Wrapping::default()),
            ProfileKind::Point => Self::Point(Point::default()),
        };
    }

    pub(crate) fn parse_attr(
        &mut self,
        attr: Key,
        rest: ParseStream,
    ) -> syn::Result<bool> {
        match attr {
            Key::With => self.settings_mut().parse(rest, true, &GRAMMAR)?,
            Key::Without => self.settings_mut().parse(rest, false, &GRAMMAR)?,
            Key::Precedes => {
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

    pub(crate) fn settings(&self) -> &FlagSettings<IntegralFlag> {
        return match self {
            Self::Int(Int { settings, .. })
            | Self::Circular(Circular { settings, .. })
            | Self::Delta(Circular { settings, .. })
            | Self::Sequential(Sequential { settings, .. })
            | Self::Wrapping(Wrapping { settings, .. })
            | Self::Point(Point { settings, .. }) => settings,
        };
    }

    fn settings_mut(&mut self) -> &mut FlagSettings<IntegralFlag> {
        return match self {
            Self::Int(Int { settings, .. })
            | Self::Circular(Circular { settings, .. })
            | Self::Delta(Circular { settings, .. })
            | Self::Sequential(Sequential { settings, .. })
            | Self::Wrapping(Wrapping { settings, .. })
            | Self::Point(Point { settings, .. }) => settings,
        };
    }

    /// Flags the preset turns off; layering the profile also resets their
    /// explicit settings.
    fn preset_off(&self) -> &'static [&'static [IntegralFlag]] {
        return match self {
            Self::Int(_) | Self::Wrapping(_) => &[],
            Self::Circular(_) | Self::Delta(_) => &[
                IntegralFlag::RESTRICTED_MATH_FLAGS,
                IntegralFlag::ORDERING,
            ],
            Self::Sequential(_) => &[IntegralFlag::RESTRICTED_MATH_FLAGS],
            Self::Point(_) => &[
                IntegralFlag::RESTRICTED_MATH_FLAGS,
                IntegralFlag::POINT_OFF,
            ],
        };
    }

    /// Applies the preset.
    pub(crate) fn apply(
        &self,
        it: &mut IntegralFlags,
    ) {
        for flags in self.preset_off() {
            it.clear(flags);
        }
    }

    pub(crate) fn resets_flag(
        &self,
        flag: IntegralFlag,
    ) -> bool {
        return self.preset_off().iter().any(|flags| flags.contains(&flag));
    }
}

impl Parse for Profile {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut profile = Self::from_name(input.parse()?);

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

    /// The explicit setting of `flag` that survives the profiles in order:
    /// a profile whose preset resets `flag` drops earlier settings before
    /// its own apply.
    pub(crate) fn explicit_flag(
        &self,
        flag: IntegralFlag,
    ) -> Option<(bool, Span)> {
        let mut setting = None;
        for profile in self.iter() {
            if profile.resets_flag(flag) {
                setting = None;
            }
            if let Some(value) = profile.settings().flag(flag) {
                setting = Some(value);
            }
        }
        return setting;
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
        let mut profile = Profile::from_name(input.parse()?);
        let mut friends = BTreeSet::new();
        let mut ty_out = None;
        let mut precedes_span = None;

        if input.peek(syn::token::Brace) {
            let content;
            let _ = braced!(content in input);
            zz::parse_inner(&content, |attr, rest| {
                if attr == Key::Precedes {
                    precedes_span = Some(rest.span());
                }
                if profile.parse_attr(attr, rest)? {
                    return Ok(true);
                }

                match attr {
                    Key::Friend => {
                        // Facets select Math and Bit operations independently.
                        friends = FACET.parse_list(rest)?;
                    }
                    Key::Produce => ty_out = Some(rest.parse()?),
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
        flag: IntegralFlag,
    ) -> Option<(bool, Span)> {
        return self.profile.explicit_flag(flag);
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
    pub(crate) flags: IntegralFlags,
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
