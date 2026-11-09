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
    PanicAt,
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
    Named,
    Site,
};
use crate::zz_runner::{
    MkErr,
    MySpanned,
    snake_case_of,
};
use crate::zz_values::{
    Op,
    OpKind,
    OpSpec,
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
        it: &OpSpec,
    ) -> TokenStream {
        let symbol = it.token();
        return match self {
            Arithmetic::Primitive => quote! { lhs #symbol rhs },
            Arithmetic::PrimitiveWrapping => {
                let method = format_ident!("wrapping_{}", it.method);
                let rhs = Self::rhs_for(it);
                quote! { lhs.#method(#rhs) }
            }
            Arithmetic::WrapperWrapping => {
                let rhs = match it.kind == OpKind::Shift {
                    true => quote! { rhs },
                    false => quote! { ::core::num::Wrapping(rhs) },
                };
                quote! {
                    (
                        ::core::num::Wrapping(lhs)
                            #symbol #rhs
                    ).0
                }
            }
            Arithmetic::Checked => {
                let method = format_ident!("checked_{}", it.method);
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
        it: &OpSpec,
    ) -> TokenStream {
        return match self {
            Self::WrapperWrapping => Self::PrimitiveWrapping.make_expr(it),
            _ => self.make_expr(it),
        };
    }

    pub(crate) fn make_operator_expr(
        self,
        operator: &OpSpec,
        is_konst: bool,
    ) -> TokenStream {
        if operator.kind == OpKind::Bitwise {
            let symbol = operator.token();
            return quote! { lhs #symbol rhs };
        }
        return match is_konst {
            true => self.make_const_expr(operator),
            false => self.make_expr(operator),
        };
    }

    /// `operator` on the raw `lhs` and `rhs` as an `Option` of the raw
    /// result: `None` where `level` reports a failure instead of panicking.
    /// `sequential` always checks; `int` checks division (zero and
    /// `MIN / -1`) from `overflow` and everything at `nothing`; the wrapping
    /// profiles only have division by zero to check. Floats never panic.
    pub(crate) fn make_fallible_expr(
        self,
        operator: &OpSpec,
        level: PanicAt,
        float: bool,
        is_konst: bool,
    ) -> TokenStream {
        let some = |value: TokenStream| {
            return quote! { ::core::option::Option::Some(#value) };
        };
        if operator.kind == OpKind::Bitwise {
            let symbol = operator.token();
            return some(quote! { lhs #symbol rhs });
        }
        return self.make_fallible_arithmetic(operator, level, float, is_konst);
    }

    /// `make_fallible_expr` of an arithmetic operator or a shift.
    fn make_fallible_arithmetic(
        self,
        operation: &OpSpec,
        level: PanicAt,
        float: bool,
        is_konst: bool,
    ) -> TokenStream {
        let some = |value: TokenStream| {
            return quote! { ::core::option::Option::Some(#value) };
        };
        if float {
            return some(Self::Primitive.make_expr(operation));
        }
        let arithmetic = match (self, is_konst) {
            (Self::WrapperWrapping, true) => Self::PrimitiveWrapping,
            (arithmetic, _) => arithmetic,
        };
        let division = matches!(operation.op, Op::Div | Op::Rem);
        let checked = match arithmetic {
            Self::Checked => true,
            Self::Primitive => match division {
                true => level.checks_division(),
                false => level.checks_overflow(),
            },
            Self::PrimitiveWrapping | Self::WrapperWrapping => false,
        };
        if checked {
            return Self::make_checked_option(operation);
        }
        let value = arithmetic.make_expr(operation);
        if division && level.checks_division() {
            return quote! {
                if rhs == 0 {
                    ::core::option::Option::None
                }
                else {
                    ::core::option::Option::Some(#value)
                }
            };
        }
        return some(value);
    }

    /// `checked_{operation}` of the raw operands; a shift amount past
    /// `u32` is `None` rather than truncated.
    fn make_checked_option(operation: &OpSpec) -> TokenStream {
        let method = format_ident!("checked_{}", operation.method);
        return match operation.kind == OpKind::Shift {
            true => quote! {
                if rhs > u32::MAX as usize {
                    ::core::option::Option::None
                }
                else {
                    lhs.#method(rhs as u32)
                }
            },
            false => quote! { lhs.#method(rhs) },
        };
    }

    fn rhs_for(operation: &OpSpec) -> TokenStream {
        return match operation.kind == OpKind::Shift {
            true => quote! { rhs as u32 },
            false => quote! { rhs },
        };
    }
}

/// What a profile is: one row of [`PROFILES`] per [`ProfileKind`].
pub(crate) struct ProfileSpec {
    kind: ProfileKind,
    /// The arithmetic the profile selects; `None` keeps the one selected
    /// before it.
    arithmetic: Option<Arithmetic>,
    /// Flags the preset turns off; layering the profile also resets their
    /// explicit settings.
    preset_off: &'static [&'static [IntegralFlag]],
    /// Whether an `f32`/`f64` value field accepts the profile.
    pub(crate) float: bool,
    /// Whether a facet can select the profile.
    facet: bool,
    /// Whether the profile selects `Wrapping` storage.
    wrapping_storage: bool,
    /// Whether values wrap around: no linear ordering and no `range`.
    circular: bool,
    /// Whether `precedes` is generated by default; only such a profile
    /// takes the `precedes` key.
    precedes: bool,
}

/// Every profile's row, at its kind's index.
const PROFILES: [ProfileSpec; 7] = [
    ProfileSpec {
        kind: ProfileKind::Int,
        arithmetic: Some(Arithmetic::Primitive),
        preset_off: &[],
        float: true,
        facet: true,
        wrapping_storage: false,
        circular: false,
        precedes: false,
    },
    ProfileSpec {
        kind: ProfileKind::Circular,
        arithmetic: Some(Arithmetic::PrimitiveWrapping),
        preset_off: &[
            IntegralFlag::RESTRICTED_MATH_FLAGS,
            IntegralFlag::ORDERING,
        ],
        float: false,
        facet: true,
        wrapping_storage: false,
        circular: true,
        precedes: true,
    },
    // `circular` with `precedes` off.
    ProfileSpec {
        kind: ProfileKind::Delta,
        arithmetic: Some(Arithmetic::PrimitiveWrapping),
        preset_off: &[
            IntegralFlag::RESTRICTED_MATH_FLAGS,
            IntegralFlag::ORDERING,
        ],
        float: false,
        facet: true,
        wrapping_storage: false,
        circular: true,
        precedes: false,
    },
    ProfileSpec {
        kind: ProfileKind::Sequential,
        arithmetic: Some(Arithmetic::Checked),
        preset_off: &[IntegralFlag::RESTRICTED_MATH_FLAGS],
        float: false,
        facet: true,
        wrapping_storage: false,
        circular: false,
        precedes: false,
    },
    ProfileSpec {
        kind: ProfileKind::Wrapping,
        arithmetic: Some(Arithmetic::WrapperWrapping),
        preset_off: &[],
        float: false,
        facet: true,
        wrapping_storage: true,
        circular: false,
        precedes: false,
    },
    ProfileSpec {
        kind: ProfileKind::Point,
        arithmetic: None,
        preset_off: &[
            IntegralFlag::RESTRICTED_MATH_FLAGS,
            IntegralFlag::POINT_OFF,
        ],
        float: true,
        facet: true,
        wrapping_storage: false,
        circular: false,
        precedes: false,
    },
    ProfileSpec {
        kind: ProfileKind::Nominal,
        arithmetic: None,
        preset_off: &[
            IntegralFlag::RESTRICTED_MATH_FLAGS,
            IntegralFlag::ORDERING,
            IntegralFlag::NOMINAL_OFF,
        ],
        float: false,
        facet: false,
        wrapping_storage: false,
        circular: false,
        precedes: false,
    },
];

// Every row of `PROFILES` is at its kind's index.
const _: () = {
    let mut index = 0;
    while index < PROFILES.len() {
        assert!(PROFILES[index].kind as usize == index);
        index += 1;
    }
};

/// A profile name: `profile = NAME`, a facet, or a fixed-profile macro.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ProfileKind {
    Int,
    Circular,
    Delta,
    Sequential,
    Wrapping,
    Point,
    Nominal,
}

impl Named for ProfileKind {
    const TABLE: &'static [(Self, &'static str)] = &[
        (Self::Int, "int"),
        (Self::Circular, "circular"),
        (Self::Delta, "delta"),
        (Self::Sequential, "sequential"),
        (Self::Wrapping, "wrapping"),
        (Self::Point, "point"),
        (Self::Nominal, "nominal"),
    ];

    fn index(self) -> usize {
        return self as usize;
    }
}

// Every row of `TABLE` is at its variant's index.
const _: () = {
    let table = ProfileKind::TABLE;
    let mut index = 0;
    while index < table.len() {
        assert!(table[index].0 as usize == index);
        index += 1;
    }
};

impl ProfileKind {
    pub(crate) fn spec(self) -> &'static ProfileSpec {
        return &PROFILES[self as usize];
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

/// A selected profile: its kind, the settings of its body, and whether it
/// generates `precedes`.
#[derive(Clone)]
pub(crate) struct Profile {
    kind: ProfileKind,
    settings: FlagSettings<IntegralFlag>,
    precedes: bool,
}

impl Profile {
    pub(crate) fn from_name(kind: ProfileKind) -> Self {
        return Self {
            kind,
            settings: FlagSettings::default(),
            precedes: kind.spec().precedes,
        };
    }

    pub(crate) fn parse_attr(
        &mut self,
        attr: Key,
        rest: ParseStream,
    ) -> syn::Result<bool> {
        match attr {
            Key::With => self.settings.parse(rest, true, &GRAMMAR)?,
            Key::Without => self.settings.parse(rest, false, &GRAMMAR)?,
            Key::Precedes => {
                if !self.kind.spec().precedes {
                    return Ok(false);
                }
                self.precedes = rest.parse::<LitBool>()?.value;
            }
            _ => return Ok(false),
        }

        return Ok(true);
    }

    pub(crate) fn arithmetic(&self) -> Option<Arithmetic> {
        return self.kind.spec().arithmetic;
    }

    pub(crate) fn accepts_float(&self) -> bool {
        return self.kind.spec().float;
    }

    pub(crate) fn settings(&self) -> &FlagSettings<IntegralFlag> {
        return &self.settings;
    }

    /// Flags the preset turns off; layering the profile also resets their
    /// explicit settings.
    fn preset_off(&self) -> &'static [&'static [IntegralFlag]] {
        return self.kind.spec().preset_off;
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
            zz::parse_inner(&content, Site::Root, |attr, rest| {
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
            .is_some_and(|profile| profile.kind.spec().wrapping_storage);
    }

    pub(crate) fn is_circular(&self) -> bool {
        return self
            .effective()
            .is_some_and(|profile| profile.kind.spec().circular);
    }

    pub(crate) fn is_default(&self) -> bool {
        return self.0.is_empty();
    }

    pub(crate) fn emits_precedes(&self) -> bool {
        return self.effective().is_some_and(|profile| profile.precedes);
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
            .finish();
    }
}

impl Parse for FacetCfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let kind = input.parse::<ProfileKind>()?;
        if !kind.spec().facet {
            return span.fail("a nominal facet has no operators to select");
        }
        let mut profile = Profile::from_name(kind);
        let mut friends = BTreeSet::new();
        let mut ty_out = None;

        if input.peek(syn::token::Brace) {
            let content;
            let _ = braced!(content in input);
            zz::parse_inner(&content, Site::Facet, |attr, rest| {
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
    pub(crate) operator: Op,
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
    /// Whether the slot also gets `{Op}Assign`: its owner has `assign_*`
    /// on and its output is `Self`.
    pub(crate) assign: bool,
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
            .field("assign", &self.assign)
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
