use std::collections::{
    BTreeMap,
    BTreeSet,
};

use proc_macro2::{
    Ident,
    Span,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::ext::IdentExt as _;
use syn::parse::{
    Parse,
    ParseStream,
    Parser,
};
use syn::spanned::Spanned;
use syn::{
    Fields,
    ItemEnum,
    Visibility,
    bracketed,
    parse_quote,
};

use crate::integral_cfg::IntegralFlag;
use crate::zz_flags::{
    Flag,
    FlagGrammar,
    FlagRow,
    FlagSettings,
    Flags,
};
use crate::zz_friend::{
    BITFLAG_ROOT,
    Capability,
    Friend,
    Protocol,
    ProtocolCapability,
    ProtocolFriend,
    upsert_friend,
};
use crate::zz_hide::{
    Envelope,
    HideCfg,
    HideIn,
};
use crate::zz_lib::{
    Key,
    Konst,
    KonstFamily,
    Konsts,
    NUMERIC_LINTS,
    Named as _,
    lint_scope,
    reject_generics,
    repr_transparent,
};
use crate::zz_lit::LitMacro;
use crate::zz_names::{
    NameKey,
    Names,
    ResolvedNames,
};
use crate::zz_runner::{
    self,
    MkErr,
    ToCollection,
};
use crate::zz_values::{
    N,
    OPS,
    Op,
    OpKind,
    OpSpec,
};
use crate::{
    integral,
    integral_cfg,
    marker_tree,
    zz_lib as zz,
};

/// The whole bitflag expansion: the enum first, then its generated code.
pub(crate) fn ekran(
    attr: Box<Cfg>,
    mut item: ItemEnum,
) -> syn::Result<TokenStream> {
    let attributes = derive_attributes(&attr);

    let attrs = &item.attrs;
    let span = item.span();
    let item_repr = zz::find_repr_n(attrs, &span)?;
    let repr = resolve_repr(attr.repr, item_repr, span)?;
    let repr_attr = attr.repr.map(|it| {
        let it = format_ident!("{}", it.name());
        quote! { #[repr(#it)] }
    });
    let ty = item.ident.clone();
    let items = item.variants.iter().map(|it| it.ident.clone()).vec();

    reject_generics(&item.generics, "bitflag does not support generic enums")?;
    reject_non_unit_variants(&item)?;

    let mut envelope = Envelope::new(&attr.hide_in, &item, &item.vis);
    envelope.prepare(&mut item, attr.tree.as_ref())?;

    let makes_value = attr.bit.has(BitFlag::MakeValue);
    let maker = Maker::new(ty, repr, attr, items);
    let value = maker.vl.clone();
    let lit = lit_macro(&maker, envelope.declared(), makes_value)?;
    if makes_value {
        // The value type is declared `pub` (`ekran_vl`); keep that.
        envelope.export(syn::parse_quote! { pub }, value.clone());
    }
    let lit_items = lit.as_ref().map(|lit| {
        let guard = lit.guard(quote! {}, &value, quote! {});
        let definition = envelope.lit_definition(lit);
        quote! { #definition #guard }
    });
    let stream = maker.ekran()?;

    return Ok(envelope.finish(
        quote! { #attributes #repr_attr },
        item,
        quote! { #stream #lit_items },
    ));
}

/// The enum's derives; `Clone` and the comparisons follow `konst`.
fn derive_attributes(cfg: &Cfg) -> TokenStream {
    let derive_clone = cfg
        .bit
        .has(BitFlag::Clone)
        .then(|| cfg.konst.derive(quote! { Clone }));
    let derive_comparison =
        cfg.konst.derive(quote! { Eq, PartialEq, Ord, PartialOrd });
    let derive_copy = cfg
        .bit
        .has(BitFlag::Copy)
        .then(|| quote! { #[derive(Copy)] });
    let derive_hash = cfg
        .bit
        .has(BitFlag::Hash)
        .then(|| quote! { #[derive(Hash)] });
    let derive_debug = cfg
        .bit
        .has(BitFlag::Debug)
        .then(|| quote! { #[derive(Debug)] });

    return quote! {
        #derive_clone
        #derive_comparison
        #derive_copy
        #derive_hash
        #derive_debug
    };
}

/// Bitflag variants carry no data.
fn reject_non_unit_variants(item: &ItemEnum) -> syn::Result<()> {
    if let Some(bad) = item
        .variants
        .iter()
        .find(|it| !matches!(it.fields, Fields::Unit))
    {
        return bad.fail("only unit variants are supported");
    }

    return Ok(());
}

/// The value type's literal macro, if generated; requesting it without a
/// generated value type fails.
fn lit_macro(
    maker: &Maker,
    declared: &Visibility,
    makes_value: bool,
) -> syn::Result<Option<LitMacro>> {
    let int = &maker.cfg.int;
    return match makes_value && maker.cfg.bit.has(BitFlag::ImplValue) {
        true if int.lit_macro()? => Ok(Some(LitMacro::new(
            &maker.vl,
            &syn::Generics::default(),
            declared,
            int.lit_name.as_ref(),
            int.lit_path.as_ref(),
        )?)),
        true => Ok(None),
        false => match int.lit_requested() {
            Some(span) => {
                span.fail("the literal macro needs the generated value type")
            }
            None => Ok(None),
        },
    };
}

pub(crate) fn resolve_repr(
    attr_repr: Option<N>,
    item_repr: Option<N>,
    span: impl Spanned,
) -> syn::Result<N> {
    if attr_repr.is_some() && item_repr.is_some() {
        return span
            .fail("repr cannot be specified on both bitflag and the enum");
    }

    return match attr_repr.or(item_repr) {
        Some(repr) => Ok(repr),
        None => span.fail(
            "bitflag requires `repr = <numeric type>` or \
             `#[repr(<numeric type>)]` on the enum",
        ),
    };
}

/// Value text impls that bitflag emits itself; the nested integral never
/// does.
struct ValueText {
    /// `Display for #vl`: integral `display` or bitflag `display`.
    display: bool,
    /// `FromStr` for the enum and the value type: integral `from_str`.
    from_str: bool,
}

/// `const`, or nothing, for each group of generated items.
struct FlagKonst {
    /// `[Value, Bit]`: the value type's and the enum's methods.
    value: Option<TokenStream>,
    /// `[Not, Bit]`: `Not` for the value type and the enum.
    not: Option<TokenStream>,
    /// `[Conversion]`: `from_name`.
    conversion: Option<TokenStream>,
    /// `[Iterator]`: the iterators and the methods returning them.
    iterator: Option<TokenStream>,
}

impl FlagKonst {
    fn new(int: &integral_cfg::Cfg) -> Self {
        let konst = |flags: &[Konsts]| Konst::konst(int.konst_for(flags));
        return Self {
            value: konst(&[
                Konsts::Value,
                Konsts::Bit,
            ]),
            not: konst(&[
                Konsts::Not,
                Konsts::Bit,
            ]),
            conversion: konst(&[Konsts::Conversion]),
            iterator: konst(&[Konsts::Iterator]),
        };
    }
}

pub(crate) struct Maker {
    repr: N,
    el: Ident,
    ty: Ident,
    vl: Ident,
    cfg: Box<Cfg>,
    items: Vec<Ident>,
    names: ResolvedNames<BitflagName>,
    /// The root friend protocol's seal (`FlagSeal`).
    seal: Ident,
    /// The seal's conversion (`conv_{snake}`).
    fn_conv: Ident,
    /// The value type's raw accessor: `get_raw`, else `#vl::raw`.
    raw: TokenStream,
    /// The unsigned type of the repr's width, for hexadecimal bits.
    unsigned: Ident,
    konst: FlagKonst,
    value_text: ValueText,
}

impl Maker {
    pub(crate) fn new(
        ty: Ident,
        repr: N,
        mut cfg: Box<Cfg>,
        items: Vec<Ident>,
    ) -> Self {
        // `value_name` and `suffix` choose the value type's name.
        let mut names = Names::default();
        match (cfg.value_name.as_str(), cfg.suffix.as_str()) {
            ("", "") => {}
            ("", suffix) => names
                .choose(BitflagName::Value, format_ident!("{}{}", ty, suffix)),
            (prefix, "") => {
                names.choose(BitflagName::Value, format_ident!("{}", prefix));
            }
            (prefix, suffix) => names.choose(
                BitflagName::Value,
                format_ident!("{}{}", prefix, suffix),
            ),
        }
        let names = names.resolve(ty.clone(), BTreeSet::new());
        let vl = names.get(BitflagName::Value);
        let raw = match &cfg.int.get_raw {
            Some(it) => quote! { #it },
            None => quote! { #vl::raw },
        };
        let value_text = Self::override_nested(&mut cfg);
        let mut this = Self {
            seal: names.get(BitflagName::SealTrait),
            fn_conv: names.get(BitflagName::Conversion),
            raw,
            unsigned: format_ident!("{}", repr.unsigned().name()),
            konst: FlagKonst::new(&cfg.int),
            value_text,
            vl,
            el: format_ident!("{}", repr.name()),
            repr,
            ty,
            cfg,
            items,
            names,
        };

        this.fix_friendship();

        return this;
    }

    /// The nested integral's overrides, in one place: bitflag's own `Not`
    /// replaces integral's, bitflag emits the value text impls itself, and
    /// bitflag serde replaces integral's.
    fn override_nested(cfg: &mut Cfg) -> ValueText {
        let value_text = ValueText {
            display: cfg.int.flags.has(IntegralFlag::Display)
                || cfg.bit.has(BitFlag::Display),
            from_str: cfg.int.flags.has(IntegralFlag::FromStr),
        };
        let flags = &mut cfg.int.flags;
        for flag in [
            IntegralFlag::MathNot,
            IntegralFlag::Display,
            IntegralFlag::FromStr,
        ] {
            flags.set(flag, false);
        }
        if cfg.serde.is_some() {
            flags.set(IntegralFlag::Serde, false);
        }

        return value_text;
    }

    fn fix_friendship(&mut self) {
        let ty = &self.ty;
        let vl = &self.vl;

        // enum :: enum, Bit.
        upsert_friend(&mut self.cfg.friends, Friend::from(ty), |req| {
            req.capabilities.insert(Capability::Bit.ident());
            req.conv = Some(parse_quote! { #ty::raw });
        });

        // enum :: value, Bit.
        upsert_friend(&mut self.cfg.friends, Friend::from(vl), |req| {
            req.capabilities.insert(Capability::Bit.ident());
            req.conv = Some(parse_quote! { #vl::raw });
        });

        // value :: enum, Make & Bit.
        self.cfg.int.add_friend(
            ty,
            [
                Capability::Bit.ident(),
                Capability::Make.ident(),
            ],
            parse_quote! { #ty::raw },
        );
    }

    pub(crate) fn ekran(mut self) -> syn::Result<TokenStream> {
        self.cfg.int.validation.resolve(
            crate::zz_values::Num::Int(self.repr),
            crate::zz_domain_float::MathProvider::Core,
        )?;
        // The parse-time check covers domains that exclude zero regardless of
        // the type; this one covers the rest (`count_zeros == 8`).
        if self.cfg.int.validation.domain_excludes_zero()
            || self.cfg.int.requires_non_zero_storage()
        {
            return self.ty.fail("bitflag integral domain must include zero");
        }
        let validation = self.cfg.int.validation.emit(&quote! { bits });
        let vl = self.ekran_vl();
        let vl_impl = self.ekran_vl_impl();
        let vl_impls = self.ekran_vl_impls();
        let ty_delegate = self.ekran_ty_delegate();
        let ty_impl = self.ekran_ty_impl();
        let ty_impls = self.ekran_ty_impls();
        let iter_impl = self.ekran_iter();
        let friends = self.ekran_friendship();
        let vl_display = self.vl_display_impl();
        let from_str = self.ekran_from_str(&validation);
        let serde_impls = self.ekran_serde(&validation);

        let impl_int = match self.cfg.bit.has(BitFlag::ImplValue) {
            false => TokenStream::new(),
            true => {
                let storage = self.vl_storage();
                integral::ekran_with(self.vl, self.el, self.cfg.int, storage)?
            }
        };

        let scope = lint_scope(
            NUMERIC_LINTS,
            quote! {
                #impl_int

                #vl_impl

                #vl_impls

                #ty_impl

                #ty_delegate

                #ty_impls

                #iter_impl

                #friends

                #vl_display

                #from_str

                #serde_impls
            },
        );
        let it = quote::quote! {
            #vl

            #scope
        };

        return Ok(it);
    }

    // ---------------------------------

    fn ekran_vl(&self) -> TokenStream {
        let ty_value = &self.vl;
        let el = &self.el;
        let derive_clone = self.cfg.int.wide.konst.derive(quote! { Clone });
        let repr = repr_transparent(self.cfg.bit.has(BitFlag::Repr));

        return match self.cfg.bit.has(BitFlag::MakeValue) {
            true => quote! {
                #derive_clone
                #[derive(Copy)]
                #repr
                pub struct #ty_value(#el);
            },
            false => TokenStream::new(),
        };
    }

    /// The value type's inherent methods, one group after the other.
    fn ekran_vl_impl(&self) -> TokenStream {
        let vl = &self.vl;
        let named = self.vl_named_values();
        let construction = self.vl_construction_fns();
        let known_bits = self.vl_known_bits_fns();
        let names = self.vl_name_fns();
        let iteration = self.vl_iteration_fns();
        let queries = self.vl_query_fns();
        let algebra = self.vl_set_algebra_fns();
        let operations = self.vl_set_operation_fns();

        return quote! {
            impl #vl {
                #named
                #construction
                #known_bits
                #names
                #iteration
                #queries
                #algebra
                #operations
            }
        };
    }

    /// One `const` per variant, named like it.
    fn vl_named_values(&self) -> TokenStream {
        let ty = &self.ty;
        let el = &self.el;
        return self
            .items
            .iter()
            .map(|it| {
                return quote! {
                    #[allow(non_upper_case_globals)]
                    pub const #it: Self = Self::_or_panic(#ty::#it as #el);
                };
            })
            .collect();
    }

    /// The value type's storage: the primitive itself.
    fn vl_storage(&self) -> integral::StoragePlan {
        return integral::StoragePlan::primitive(self.repr);
    }

    /// `constructor`, a path of the value type, applied to the validated
    /// `bits`.
    fn vl_of_bits(
        &self,
        constructor: TokenStream,
    ) -> TokenStream {
        let storage = self.vl_storage().storage_expression(quote! { bits });
        return quote! { #constructor(#storage) };
    }

    /// `from_bits_retain`'s body: `try_make`-or-panic with validation,
    /// `Self::_or_panic(bits)` without.
    fn vl_from_bits_body(&self) -> TokenStream {
        if !self.cfg.int.validation.has_validation() {
            return quote! { Self::_or_panic(bits) };
        }

        return zz::unwrap_or_panic(
            quote! { Self::try_make(bits) },
            "invalid value",
        );
    }

    /// `bits`, `from_bits_retain`, `empty` and the `all*` values.
    fn vl_construction_fns(&self) -> TokenStream {
        let ty = &self.ty;
        let el = &self.el;
        let raw = &self.raw;
        let konst = &self.konst.value;
        let from_bits = self.vl_from_bits_body();

        return quote! {
            #[must_use]
            #[inline(always)]
            pub #konst fn bits(self) -> #el {
                return #raw(self);
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn from_bits_retain(bits: #el) -> Self {
                return #from_bits;
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn empty() -> Self {
                return Self::from_bits_retain(0);
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn all() -> Self {
                return #ty::all();
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn all_named() -> Self {
                return Self::all();
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn all_known() -> Self {
                return Self::all();
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn all_unknown() -> Self {
                return Self::from_bits_retain(!Self::all().bits());
            }
        };
    }

    /// Splitting bits into the declared (known) and the other ones.
    fn vl_known_bits_fns(&self) -> TokenStream {
        let el = &self.el;
        let konst = &self.konst.value;

        return quote! {
            #[must_use]
            #[inline(always)]
            pub #konst fn known_bits(self) -> #el {
                return self.bits() & Self::all().bits();
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn unknown_bits(self) -> #el {
                return self.bits() & !Self::all().bits();
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn into_known_bits(self) -> Self {
                return Self::from_bits_retain(self.known_bits());
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn into_unknown_bits(self) -> Self {
                return Self::from_bits_retain(self.unknown_bits());
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn contains_unknown_bits(self) -> bool {
                return self.unknown_bits() != Self::empty().bits();
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn from_bits(bits: #el) -> Option<Self> {
                let value = Self::from_bits_retain(bits);
                return if value.contains_unknown_bits() {
                    None
                }
                else {
                    Some(value)
                };
            }

            #[inline(always)]
            pub #konst fn try_as_known_bits_only(self) -> Result<Self, Self> {
                return if self.contains_unknown_bits() {
                    Err(self)
                }
                else {
                    Ok(self)
                };
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn from_bits_truncate(bits: #el) -> Self {
                return Self::from_bits_retain(bits).truncated();
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn truncated_into_known_bits(self) -> Self {
                return self.truncated();
            }

            #[inline(always)]
            pub #konst fn truncate_into_known_bits(&mut self) {
                self.truncate();
            }
        };
    }

    /// `from_name` and `into_flag`: between flag names, flags and values.
    fn vl_name_fns(&self) -> TokenStream {
        let ty = &self.ty;
        let konst = &self.konst.value;
        let from_konst = &self.konst.conversion;

        return quote! {
            #[must_use]
            #[inline(always)]
            pub #from_konst fn from_name(name: &str) -> Option<Self> {
                return match #ty::from_name(name) {
                    Some(flag) => Some(flag.into_value()),
                    None => None,
                };
            }

            #[inline(always)]
            pub #konst fn into_flag(self) -> Result<#ty, Self> {
                let items = #ty::items();
                let max = items.len();
                let mut i = 0;

                while i < max {
                    let flag = items[i];
                    if flag.into_value().bits() == self.bits() {
                        return Ok(flag);
                    }
                    i += 1;
                }

                return Err(self);
            }
        };
    }

    /// The iterators over flags, values and names.
    fn vl_iteration_fns(&self) -> TokenStream {
        let ty = &self.ty;
        let iterator_konst = &self.konst.iterator;

        return quote! {
            #[must_use]
            #[inline(always)]
            pub #iterator_konst fn iter_known_flags(self) -> impl Iterator<Item = #ty> {
                return IterFlags {
                    value: self,
                    index: 0,
                };
            }

            #[must_use]
            #[inline(always)]
            pub #iterator_konst fn iter(self) -> impl Iterator<Item = Self> {
                return IterValues {
                    value: self,
                    index: 0,
                };
            }

            #[must_use]
            #[inline(always)]
            pub fn iter_names(self) -> impl Iterator<Item = (&'static str, Self)> {
                return self
                    .iter_known_flags()
                    .map(|flag| (flag.name(), flag.into_value()));
            }

            #[must_use]
            #[inline(always)]
            pub fn iter_defined_names() -> impl Iterator<Item = (&'static str, Self)> {
                return #ty::iter().map(|flag| (flag.name(), flag.into_value()));
            }

            #[must_use]
            #[inline(always)]
            pub fn iter_equal_names(self) -> impl Iterator<Item = &'static str> {
                return #ty::iter()
                    .filter(move |flag| flag.into_value() == self)
                    .map(|flag| flag.name());
            }
        };
    }

    /// `is_empty` … `contains_any`.
    fn vl_query_fns(&self) -> TokenStream {
        let konst = &self.konst.value;

        return quote! {
            #[must_use]
            #[inline(always)]
            pub #konst fn is_empty(self) -> bool {
                return self.bits() == 0;
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn is_all(self) -> bool {
                return self.contains_all(Self::all());
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn is_exactly_all_known_bits(self) -> bool {
                return self.bits() == Self::all().bits();
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn intersects(self, that: Self) -> bool {
                return self.bits() & that.bits() != 0;
            }

            #[must_use]
            #[inline(always)]
            pub fn contains<T>(self, that: T) -> bool
            where
                T: Into<Self>,
            {
                return self.contains_all(that.into());
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn contains_all(self, that: Self) -> bool {
                return self.bits() & that.bits() == that.bits();
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn contains_any(self, that: Self) -> bool {
                return self.intersects(that);
            }
        };
    }

    /// The set algebra: `truncated` … `clear`.
    fn vl_set_algebra_fns(&self) -> TokenStream {
        let konst = &self.konst.value;

        return quote! {
            #[must_use]
            #[inline(always)]
            pub #konst fn truncated(self) -> Self {
                return Self::from_bits_retain(self.known_bits());
            }

            #[inline(always)]
            pub #konst fn truncate(&mut self) {
                *self = self.truncated();
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn inserted(self, that: Self) -> Self {
                return Self::from_bits_retain(self.bits() | that.bits());
            }

            #[inline(always)]
            pub #konst fn insert(&mut self, that: Self) {
                *self = self.inserted(that);
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn removed(self, that: Self) -> Self {
                return Self::from_bits_retain(self.bits() & !that.bits());
            }

            #[inline(always)]
            pub #konst fn remove(&mut self, that: Self) {
                *self = self.removed(that);
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn toggled(self, that: Self) -> Self {
                return Self::from_bits_retain(self.bits() ^ that.bits());
            }

            #[inline(always)]
            pub #konst fn toggle(&mut self, that: Self) {
                *self = self.toggled(that);
            }

            #[inline(always)]
            pub #konst fn set(&mut self, that: Self, value: bool) {
                if value {
                    self.insert(that);
                }
                else {
                    self.remove(that);
                }
            }

            #[inline(always)]
            pub #konst fn clear(&mut self) {
                *self = Self::empty();
            }
        };
    }

    /// The set algebra, continued: `with` … `complement`.
    fn vl_set_operation_fns(&self) -> TokenStream {
        let konst = &self.konst.value;

        return quote! {
            #[must_use]
            #[inline(always)]
            pub #konst fn with(self, that: Self) -> Self {
                return self.inserted(that);
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn without(self, that: Self) -> Self {
                return self.removed(that);
            }

            #[inline(always)]
            pub #konst fn unset(&mut self, that: Self) {
                self.remove(that);
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn intersection(self, that: Self) -> Self {
                return Self::from_bits_retain(self.bits() & that.bits());
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn union(self, that: Self) -> Self {
                return self.inserted(that);
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn difference(self, that: Self) -> Self {
                return self.removed(that);
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn symmetric_difference(self, that: Self) -> Self {
                return self.toggled(that);
            }

            #[must_use]
            #[inline(always)]
            pub #konst fn complemented(self) -> Self {
                return Self::from_bits_retain(!self.bits() & Self::all().bits());
            }

            #[inline(always)]
            pub #konst fn complement(&mut self) {
                *self = self.complemented();
            }
        };
    }

    /// The body of a `Display` for flag bits: the names of the set flags
    /// joined by ` | `, then the remaining bits in hexadecimal. Shared by
    /// `Display for #vl` (`self.bits()`) and `SerdeFlagValue`
    /// (`self.0.bits()`).
    fn flag_format_body(
        &self,
        bits: TokenStream,
    ) -> TokenStream {
        let ty = &self.ty;
        let unsigned = &self.unsigned;

        return quote! {
            let mut remaining = #bits;
            let mut first = true;
            let items = #ty::items();
            let mut index = 0;

            while index < items.len() {
                let flag = items[index];
                let bits = flag.raw();
                index += 1;

                if bits != 0 && remaining & bits == bits {
                    if !first {
                        formatter.write_str(" | ")?;
                    }
                    formatter.write_str(flag.name())?;
                    first = false;
                    remaining &= !bits;
                }
            }

            if remaining != 0 {
                if !first {
                    formatter.write_str(" | ")?;
                }
                return ::core::fmt::Write::write_fmt(
                    formatter,
                    ::core::format_args!(
                        "0x{:x}",
                        remaining as #unsigned,
                    ),
                );
            }

            return ::core::result::Result::Ok(());
        };
    }

    /// The body that parses `source`, `|`-separated flag names and
    /// hexadecimal bits, into `bits`. Shared by `FromStr for #vl` and
    /// `SerdeFlagValueVisitor::visit_str`; `fail` builds each error, and
    /// the body ends by validating `bits`: `ok`, else `fail("invalid
    /// value")`.
    fn flag_parse_body(
        &self,
        validation: &zz::Validation,
        fail: impl Fn(&'static str) -> TokenStream,
        ok: TokenStream,
    ) -> TokenStream {
        let ty = &self.ty;
        let el = &self.el;
        let unsigned = &self.unsigned;
        let empty = fail("empty flag");
        let invalid_hex = fail("invalid hexadecimal flag bits");
        let unknown = fail("unknown flag name");
        let parsed = validation.construct(
            zz::Check::Validate,
            zz::OnInvalid::Err(fail("invalid value")),
            ok,
        );

        return quote! {
            let source = source.trim();
            let mut bits: #el = 0;

            if !source.is_empty() {
                for part in source.split('|') {
                    let part = part.trim();
                    if part.is_empty() {
                        return ::core::result::Result::Err(#empty);
                    }

                    let value = if let Some(hex) =
                        part.strip_prefix("0x")
                    {
                        if hex.is_empty() {
                            return ::core::result::Result::Err(#invalid_hex);
                        }
                        match <#unsigned>::from_str_radix(hex, 16) {
                            ::core::result::Result::Ok(value) => {
                                value as #el
                            }
                            ::core::result::Result::Err(_) => {
                                return ::core::result::Result::Err(#invalid_hex);
                            }
                        }
                    }
                    else {
                        match #ty::from_name(part) {
                            ::core::option::Option::Some(flag) => {
                                flag.raw()
                            }
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(#unknown);
                            }
                        }
                    };

                    bits |= value;
                }
            }

            #parsed
        };
    }

    /// `Display for #vl` (integral or bitflag `display`).
    fn vl_display_impl(&self) -> Option<TokenStream> {
        if !self.value_text.display {
            return None;
        }

        let vl = &self.vl;
        let body = self.flag_format_body(quote! { self.bits() });
        return Some(quote! {
            impl ::core::fmt::Display for #vl {
                #[inline]
                fn fmt(
                    &self,
                    formatter: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    #body
                }
            }
        });
    }

    /// `FromStr` for the enum (a name) and the value type (integral
    /// `from_str`); both fail with `()`.
    fn ekran_from_str(
        &self,
        validation: &zz::Validation,
    ) -> Option<TokenStream> {
        if !self.value_text.from_str {
            return None;
        }

        let ty = &self.ty;
        let vl = &self.vl;
        let body = self.flag_parse_body(
            validation,
            |_| quote! { () },
            self.vl_of_bits(quote! { Self }),
        );
        return Some(quote! {
            impl ::core::str::FromStr for #ty {
                type Err = ();

                #[inline]
                fn from_str(
                    source: &str,
                ) -> ::core::result::Result<Self, Self::Err> {
                    return Self::from_name(source.trim()).ok_or(());
                }
            }

            impl ::core::str::FromStr for #vl {
                type Err = ();

                #[inline]
                fn from_str(
                    source: &str,
                ) -> ::core::result::Result<Self, Self::Err> {
                    #body
                }
            }
        });
    }

    /// Bitflag serde: the name and value helpers, then `Serialize` and
    /// `Deserialize` for the enum and the value type.
    fn ekran_serde(
        &self,
        validation: &zz::Validation,
    ) -> TokenStream {
        let Some(repr) = self.cfg.serde
        else {
            return TokenStream::new();
        };

        let flag_value = self.serde_flag_value();
        let visitors = self.serde_visitors(validation);
        let ty_impls = self.serde_ty_impls(repr);
        let vl_impls = self.serde_vl_impls(repr, validation);

        return quote! {
            #flag_value
            #visitors
            #ty_impls
            #vl_impls
        };
    }

    /// `SerdeFlagValue`: a value type's names, through `Display`.
    fn serde_flag_value(&self) -> TokenStream {
        let vl = &self.vl;
        let body = self.flag_format_body(quote! { self.0.bits() });

        return quote! {
            struct SerdeFlagValue(#vl);

            impl ::core::fmt::Display for SerdeFlagValue {
                fn fmt(
                    &self,
                    formatter: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    #body
                }
            }
        };
    }

    /// `SerdeFlagVisitor` (a flag by name) and `SerdeFlagValueVisitor` (a
    /// value by names and hexadecimal bits).
    fn serde_visitors(
        &self,
        validation: &zz::Validation,
    ) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let body = self.flag_parse_body(
            validation,
            |message| quote! { E::custom(#message) },
            self.vl_of_bits(quote! { #vl }),
        );

        return quote! {
            struct SerdeFlagVisitor;

            impl<'de> ::serde::de::Visitor<'de> for SerdeFlagVisitor {
                type Value = #ty;

                fn expecting(
                    &self,
                    formatter: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    return formatter.write_str("a declared flag name");
                }

                fn visit_str<E>(
                    self,
                    value: &str,
                ) -> ::core::result::Result<Self::Value, E>
                where
                    E: ::serde::de::Error,
                {
                    return #ty::from_name(value).ok_or_else(|| {
                        E::custom("unknown flag name")
                    });
                }
            }

            struct SerdeFlagValueVisitor;

            impl<'de> ::serde::de::Visitor<'de> for SerdeFlagValueVisitor {
                type Value = #vl;

                fn expecting(
                    &self,
                    formatter: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    return formatter.write_str(
                        "a `|` separated string of flag names and hexadecimal bits",
                    );
                }

                fn visit_str<E>(
                    self,
                    source: &str,
                ) -> ::core::result::Result<Self::Value, E>
                where
                    E: ::serde::de::Error,
                {
                    #body
                }
            }
        };
    }

    /// `Serialize` and `Deserialize` for the enum: its name or its raw value.
    fn serde_ty_impls(
        &self,
        repr: SerdeRepr,
    ) -> TokenStream {
        let ty = &self.ty;
        let el = &self.el;
        let serialize_as_name = repr.as_name(quote! { serializer });
        let deserialize_as_name = repr.as_name(quote! { deserializer });

        return quote! {
            impl ::serde::Serialize for #ty {
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    if #serialize_as_name {
                        return serializer.serialize_str(self.name());
                    }
                    return <#el as ::serde::Serialize>::serialize(
                        &self.raw(),
                        serializer,
                    );
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #ty {
                fn deserialize<D>(
                    deserializer: D,
                ) -> ::core::result::Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    if #deserialize_as_name {
                        return deserializer.deserialize_str(SerdeFlagVisitor);
                    }

                    let bits =
                        <#el as ::serde::Deserialize<'de>>::deserialize(
                            deserializer,
                        )?;
                    let items = Self::items();
                    let mut index = 0;
                    while index < items.len() {
                        let flag = items[index];
                        if flag.raw() == bits {
                            return ::core::result::Result::Ok(flag);
                        }
                        index += 1;
                    }
                    return ::core::result::Result::Err(
                        <D::Error as ::serde::de::Error>::custom(
                            "raw value is not a declared flag",
                        ),
                    );
                }
            }
        };
    }

    /// `Serialize` and `Deserialize` for the value type: its names or its
    /// bits.
    fn serde_vl_impls(
        &self,
        repr: SerdeRepr,
        validation: &zz::Validation,
    ) -> TokenStream {
        let vl = &self.vl;
        let el = &self.el;
        let serialize_as_name = repr.as_name(quote! { serializer });
        let deserialize_as_name = repr.as_name(quote! { deserializer });
        let deserialized = validation.construct(
            zz::Check::Validate,
            zz::OnInvalid::Err(quote! {
                <D::Error as ::serde::de::Error>::custom("invalid value")
            }),
            self.vl_of_bits(quote! { Self }),
        );

        return quote! {
            impl ::serde::Serialize for #vl {
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    if #serialize_as_name {
                        return serializer.collect_str(&SerdeFlagValue(*self));
                    }
                    return <#el as ::serde::Serialize>::serialize(
                        &self.bits(),
                        serializer,
                    );
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #vl {
                fn deserialize<D>(
                    deserializer: D,
                ) -> ::core::result::Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    if #deserialize_as_name {
                        return deserializer.deserialize_str(
                            SerdeFlagValueVisitor,
                        );
                    }

                    let bits =
                        <#el as ::serde::Deserialize<'de>>::deserialize(
                            deserializer,
                        )?;
                    #deserialized
                }
            }
        };
    }

    fn ekran_vl_impls(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let konst = &self.konst.not;

        return quote! {
            impl ::core::convert::From<#ty> for #vl {
                #[inline(always)]
                fn from(flag: #ty) -> Self {
                    return flag.into_value();
                }
            }

            #konst impl ::core::ops::Not for #vl {
                type Output = Self;

                #[inline(always)]
                fn not(self) -> Self::Output {
                    return self.complemented();
                }
            }
        };
    }

    // ---------------------------------

    fn ekran_ty_delegate(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let el = &self.el;
        let konst = &self.konst.value;

        return quote! {
            impl #ty {
                #[must_use]
                #[inline(always)]
                pub #konst fn from_bits_retain(bits: #el) -> #vl {
                    return #vl::from_bits_retain(bits);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn empty() -> #vl {
                    return #vl::empty();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all_named() -> #vl {
                    return #vl::all();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all_known() -> #vl {
                    return #vl::all();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all_unknown() -> #vl {
                    return #vl::from_bits_retain(!#vl::all().bits());
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn from_bits(bits: #el) -> Option<#vl> {
                    return #vl::from_bits(bits);
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn from_bits_truncate(bits: #el) -> #vl {
                    return #vl::from_bits_truncate(bits);
                }

                #[must_use]
                #[inline(always)]
                pub fn iter_defined_names() -> impl Iterator<Item = (&'static str, #vl)> {
                    return #vl::iter_defined_names();
                }
            }
        };
    }

    fn ekran_ty_impl(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let el = &self.el;
        let all_bits = self.items.iter();
        let konst = &self.konst.value;
        let from_konst = &self.konst.conversion;
        let iterator_konst = &self.konst.iterator;

        let name_arms = self
            .items
            .iter()
            .map(|it| {
                let name = it.to_string();
                return quote! { #ty::#it => #name, };
            })
            .vec();

        let from_name_arms = self
            .items
            .iter()
            .map(|it| {
                let name = it.to_string();
                return quote! { #name => Some(#ty::#it), };
            })
            .vec();

        let item_items = self.items.iter().map(|it| quote! { #ty::#it }).vec();

        return quote! {
            impl #ty {
                #[inline(always)]
                #[must_use]
                pub #konst fn name(self) -> &'static str {
                    return match self {
                        #(#name_arms)*
                    };
                }

                #[inline(always)]
                #[must_use]
                pub #konst fn items() -> &'static [Self] {
                    const ITEMS: &'static [#ty] = &[#(#item_items),*];

                    return ITEMS;
                }

                #[inline]
                #[must_use]
                pub #from_konst fn from_name(name: &str) -> Option<Self> {
                    return match name {
                        #(#from_name_arms)*

                        _ => ::core::option::Option::None,
                    };
                }

                #[inline(always)]
                #[must_use]
                pub #konst fn raw(self) -> #el {
                    return self as #el;
                }

                #[inline(always)]
                #[must_use]
                pub #konst fn into_value(self) -> #vl {
                    return #vl::from_bits_retain(self.raw());
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn all() -> #vl {
                    #[allow(clippy::unnecessary_cast)]
                    return #vl::from_bits_retain(0 as #el #(| (#ty::#all_bits as #el))*);
                }

                #[must_use]
                #[inline(always)]
                pub #iterator_konst fn iter() -> impl Iterator<Item = Self> {
                    return IterItems { index: 0 };
                }

                #[must_use]
                #[inline(always)]
                pub #iterator_konst fn iter_values() -> impl Iterator<Item = #vl> {
                    return #vl::all().iter();
                }

                #[must_use]
                #[inline(always)]
                pub #konst fn inserted(self, that: Self) -> #vl {
                    return self.into_value().inserted(that.into_value());
                }
            }
        };
    }

    /// The enum's operator and comparison impls, in output order.
    fn ekran_ty_impls(&self) -> TokenStream {
        let display = self.ty_display_impl();
        let operators = self.ty_operator_impls();
        let comparisons = self.ty_comparison_impls();
        let not = self.ty_not_impl();

        return quote! {
            #display
            #operators
            #comparisons
            #not
        };
    }

    /// `Display for #ty`: the flag's name (bitflag `display`).
    fn ty_display_impl(&self) -> Option<TokenStream> {
        if !self.cfg.bit.has(BitFlag::Display) {
            return None;
        }

        let ty = &self.ty;
        return Some(quote! {
            impl ::core::fmt::Display for #ty {
                #[inline(always)]
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    return ::core::fmt::Display::fmt(self.name(), f);
                }
            }
        });
    }

    /// The enum's binary operators whose `math_*` flag is on, in `OPS`
    /// order, each through the value type.
    fn ty_operator_impls(&self) -> TokenStream {
        let flags = &self.cfg.int.flags;
        return OPS
            .iter()
            .filter(|op| flags.has(IntegralFlag::of_op(op.op).math))
            .map(|op| {
                return match op.kind {
                    OpKind::Bitwise => self.ty_bit_operator_impl(op),
                    OpKind::Shift => self.ty_shift_impl(op),
                    OpKind::Arith | OpKind::Unary => TokenStream::new(),
                };
            })
            .collect();
    }

    /// `Shl<usize>` or `Shr<usize>` for the enum, through the value type.
    fn ty_shift_impl(
        &self,
        op: &OpSpec,
    ) -> TokenStream {
        let ty = &self.ty;
        let value = &self.vl;
        let konst = Konst::konst(self.cfg.int.konst_for(&op.konsts()));
        let operator = format_ident!("{}", op.trait_name);
        let method = format_ident!("{}", op.method);
        let symbol = op.token();
        return quote! {
            #konst impl ::core::ops::#operator<usize> for #ty {
                type Output = #value;

                #[inline(always)]
                fn #method(self, rhs: usize) -> Self::Output {
                    return self.into_value() #symbol rhs;
                }
            }
        };
    }

    /// `BitAnd`, `BitOr` or `BitXor` for the enum: any `FlagBit` friend on
    /// the right, combined through the value type's set method.
    fn ty_bit_operator_impl(
        &self,
        op: &OpSpec,
    ) -> TokenStream {
        let ty = &self.ty;
        let value = &self.vl;
        let seal = &self.seal;
        let fn_conv = &self.fn_conv;
        let bit_capability = self.names.get(BitflagName::BitTrait);
        let is_konst = self.cfg.int.konst_for(&op.konsts());
        let konst = Konst::konst(is_konst);
        let bounds = Konst::bound(is_konst, quote! { #seal });
        let operator = format_ident!("{}", op.trait_name);
        let method = format_ident!("{}", op.method);
        let set_method = set_method(op.op);
        return quote! {
            #konst impl<T> ::core::ops::#operator<T> for #ty
            where
                T: #bit_capability + #bounds,
            {
                type Output = #value;

                #[inline(always)]
                fn #method(self, rhs: T) -> Self::Output {
                    let rhs = #seal::#fn_conv(rhs);
                    return self.into_value().#set_method(
                        #value::from_bits_retain(rhs)
                    );
                }
            }
        };
    }

    /// `PartialEq<#vl>` and `PartialOrd<#vl>` for the enum.
    fn ty_comparison_impls(&self) -> TokenStream {
        let ty = &self.ty;
        let value = &self.vl;
        let int = &self.cfg.int;

        let eq = int.flags.has(IntegralFlag::PartialEq).then(|| {
            let konst = Konst::konst(int.konst_for(&[
                Konsts::PartialEq,
                Konsts::Comparison,
            ]));
            return quote! {
                #konst impl ::core::cmp::PartialEq<#value> for #ty {
                    #[inline(always)]
                    fn eq(&self, rhs: &#value) -> bool {
                        return self.raw() == rhs.raw();
                    }
                }
            };
        });

        let ord = int.flags.has(IntegralFlag::PartialOrd).then(|| {
            let konst = Konst::konst(int.konst_for(&[
                Konsts::PartialOrd,
                Konsts::Comparison,
            ]));
            return quote! {
                #konst impl ::core::cmp::PartialOrd<#value> for #ty {
                    #[inline(always)]
                    fn partial_cmp(
                        &self,
                        rhs: &#value,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        return self.raw().partial_cmp(&rhs.raw());
                    }
                }
            };
        });

        return quote! {
            #eq
            #ord
        };
    }

    /// `Not for #ty`: the complement of the flag's value (always).
    fn ty_not_impl(&self) -> TokenStream {
        let ty = &self.ty;
        let value = &self.vl;
        let konst = &self.konst.not;

        return quote! {
            #konst impl ::core::ops::Not for #ty {
                type Output = #value;

                #[inline(always)]
                fn not(self) -> Self::Output {
                    return self.into_value().complemented();
                }
            }
        };
    }

    // ---------------------------------

    fn ekran_iter(&self) -> TokenStream {
        let ty = &self.ty;
        let vl = &self.vl;
        let konst = &self.konst.iterator;
        let raw = &self.raw;

        return quote! {
            struct IterItems {
                index: usize,
            }

            #konst impl core::iter::Iterator for IterItems {
                type Item = #ty;

                fn next(&mut self) -> Option<Self::Item> {
                    let items = #ty::items();
                    let max = items.len();

                    while self.index < max {
                        let next = items[self.index];
                        self.index += 1;
                        return Some(next);
                    }

                    return None;
                }

                #[inline(always)]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    let max = #ty::items().len();

                    return (max, Some(max));
                }
            }

            struct IterFlags {
                value: #vl,
                index: usize,
            }

            #konst impl core::iter::Iterator for IterFlags {
                type Item = #ty;

                fn next(&mut self) -> Option<Self::Item> {
                    let items = #ty::items();
                    let max = items.len();

                    while self.index < max {
                        let next = items[self.index];
                        self.index += 1;
                        if self.value.contains_all(next.into_value()) {
                            self.value = self.value.without(next.into_value());
                            return Some(next);
                        }
                    }

                    return None;
                }

                #[inline]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    let bound = #raw(self.value).count_ones() as usize;
                    return (bound, Some(bound));
                }
            }

            pub struct IterValues {
                value: #vl,
                index: usize,
            }

            #konst impl Iterator for IterValues {
                type Item = #vl;

                fn next(&mut self) -> Option<Self::Item> {
                    let items = #ty::items();
                    let max = items.len();

                    while self.index < max {
                        let next = items[self.index].into_value();
                        self.index += 1;
                        if self.value.contains_all(next) {
                            self.value = self.value.without(next);
                            return Some(next);
                        }
                    }

                    if !self.value.is_empty() {
                        let it = self.value;
                        self.value = Self::Item::empty();
                        return Some(it);
                    }

                    return None;
                }

                #[inline]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    let bound = (#raw(self.value).count_ones() + 1) as usize;
                    return (bound, Some(bound));
                }
            }
        };
    }

    fn ekran_friendship(&self) -> TokenStream {
        let relation_ident = &self.el;
        let relation: syn::Type = parse_quote! { #relation_ident };
        let friends = self
            .cfg
            .friends
            .iter()
            .map(|friend| self.protocol_friend(friend))
            .vec();
        let target = &self.ty;

        return crate::zz_friend::emit_protocol(Protocol {
            target: quote! { #target },
            generics: syn::Generics::default(),
            relation,
            seal: self.seal.clone(),
            conversion: self.fn_conv.clone(),
            capabilities: vec![
                ProtocolCapability {
                    capability: Capability::Bit,
                    name: self.names.get(BitflagName::BitTrait),
                },
            ],
            friends,
            emit_seal: true,
            target_conversion: None,
            construction: None,
            konst: self.cfg.int.konst_friendship(),
        });
    }

    /// A root friend: its validated capabilities and its conversion to the
    /// repr.
    fn protocol_friend(
        &self,
        friend: &Friend,
    ) -> ProtocolFriend {
        let capabilities = friend
            .capabilities
            .iter()
            .map(|written| {
                let capability = Capability::of(written)
                    .expect("bitflag root capabilities are validated");
                // Root friends hold `Bit` only; keep its span.
                let mut name = self.names.get(BitflagName::BitTrait);
                name.set_span(written.span());
                return (name, capability);
            })
            .collect::<BTreeMap<_, _>>();
        let relation = &self.el;
        let conversion = match &friend.conv {
            Some(conv) if conv.is_ident("self") => quote! {
                let relation: #relation = self;
                return relation;
            },
            Some(conv) => quote! {
                return #conv(self);
            },
            None => quote! {
                let relation: #relation = self.into();
                return relation;
            },
        };

        let ty = friend.ty.path();
        return ProtocolFriend {
            ty: syn::parse_quote! { #ty },
            capabilities,
            conversion: Some(conversion),
            seal_cfg: TokenStream::new(),
            capability_cfgs: Default::default(),
            within: None,
        };
    }
}

/// The value type's set method behind each bit operator on the enum.
fn set_method(op: Op) -> Ident {
    let name = match op {
        Op::BitAnd => "intersection",
        Op::BitOr => "union",
        Op::BitXor => "symmetric_difference",
        Op::Add
        | Op::Sub
        | Op::Mul
        | Op::Div
        | Op::Rem
        | Op::Shl
        | Op::Shr
        | Op::Neg
        | Op::Not => unreachable!("not a bit operator: {op:?}"),
    };
    return format_ident!("{}", name);
}

// =============================================================================

/// The `with`/`without` grammar of a bitflag root.
const GRAMMAR: FlagGrammar = FlagGrammar {
    konsts: Some(KonstFamily::Integral),
    removals: &[],
};

/// How bitflag serde represents the enum and the value type.
#[derive(Clone, Copy, Debug)]
enum SerdeRepr {
    /// `serde` or `serde_with_auto`: names in human-readable formats, the
    /// raw value otherwise.
    Auto,
    /// `serde_with_name`.
    Name,
    /// `serde_with_value`.
    Value,
}

impl SerdeRepr {
    /// Whether to use names: `true`, `false`, or the format's choice through
    /// `#receiver.is_human_readable()`.
    fn as_name(
        self,
        receiver: TokenStream,
    ) -> TokenStream {
        return match self {
            Self::Auto => quote! { #receiver.is_human_readable() },
            Self::Name => quote! { true },
            Self::Value => quote! { false },
        };
    }
}

#[derive(Debug)]
pub(crate) struct Cfg {
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) bit: BitFlags,
    /// The serde representation; `None`: no bitflag serde impls.
    serde: Option<SerdeRepr>,
    pub(crate) int: Box<integral_cfg::Cfg>,
    pub(crate) tree: Option<marker_tree::Cfg>,
    repr: Option<N>,
    konst: Konst,
    settings: FlagSettings<BitFlag>,
    /// `value_name = "..."`: the value type's name, else the enum's.
    value_name: String,
    /// `suffix = "..."`: appended to the value type's name.
    suffix: String,
    pub(crate) hide_in: crate::zz_hide::HideIn,
}

impl HideCfg for Cfg {
    fn hide_in(&self) -> &HideIn {
        return &self.hide_in;
    }
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self {
            friends: Default::default(),
            bit: BitFlags::default(),
            serde: None,
            int: Box::new(integral_cfg::Cfg::without_konst()),
            tree: None,
            repr: None,
            konst: Konst::default(),
            settings: FlagSettings::default(),
            value_name: String::new(),
            suffix: "Value".to_string(),
            hide_in: Default::default(),
        };

        let mut nested = false;
        zz::parse_inner(input, zz::Site::Root, |attr, rest| {
            nested |= attr == Key::Integral;
            return this.parse_attr(attr, rest);
        })?;
        this.settings.apply(&mut this.bit);
        // Without `integral = [..]`, the value type resolves exactly as with
        // `integral = []`.
        if !nested {
            *this.int = parse_integral.parse2(TokenStream::new())?;
        }

        this.serde = this.serde_repr(input)?;

        let outer = integral_cfg::WrapperWide {
            konst: this.konst,
            konsts: this.settings.konsts().clone(),
            ..Default::default()
        };
        this.int.wide.embed(&outer)?;
        // Named values are `const` items built through `_or_panic`.
        this.int.const_validation_required = true;
        // The nested integral emits into the root's module, hidden or not.
        this.int.hide_in = this.hide_in.clone();

        return Ok(this);
    }
}

impl Cfg {
    /// The serde representation: exactly one `serde_with_*` mode gives that
    /// mode, even with `without = serde`; none with `serde` gives `Auto`.
    fn serde_repr(
        &self,
        input: ParseStream,
    ) -> syn::Result<Option<SerdeRepr>> {
        let mut modes = [
            (BitFlag::SerdeWithAuto, SerdeRepr::Auto),
            (BitFlag::SerdeWithName, SerdeRepr::Name),
            (BitFlag::SerdeWithValue, SerdeRepr::Value),
        ]
        .into_iter()
        .filter(|(flag, _)| self.bit.has(*flag))
        .map(|(_, repr)| repr);

        return match (modes.next(), modes.next()) {
            (Some(_), Some(_)) => Err(syn::Error::new(
                input.span(),
                "bitflag serde representation flags are mutually exclusive",
            )),
            (Some(repr), None) => Ok(Some(repr)),
            (None, _) => {
                Ok(self.bit.has(BitFlag::Serde).then_some(SerdeRepr::Auto))
            }
        };
    }

    fn parse_attr(
        &mut self,
        attr: Key,
        rest: ParseStream,
    ) -> syn::Result<bool> {
        match attr {
            Key::Konst => {
                self.konst = rest.parse()?;
            }
            Key::Repr => {
                let repr: Ident = rest.parse()?;
                self.repr = N::of(&repr.to_string());
                if self.repr.is_none() {
                    return repr.fail("repr must be a Rust numeric type");
                }
            }
            Key::With => self.settings.parse(rest, true, &GRAMMAR)?,
            Key::Without => self.settings.parse(rest, false, &GRAMMAR)?,
            Key::Friend => self.friends = BITFLAG_ROOT.parse_list(rest)?,
            Key::Tree => {
                self.tree = Some(marker_tree::Cfg::parse_attribute(rest)?);
            }
            Key::Suffix => {
                let as_str: syn::LitStr = rest.parse()?;
                if as_str.value().is_empty() {
                    self.suffix = "".to_string();
                }
                else {
                    let as_idn: Ident = syn::parse_str(&as_str.value())?;
                    self.suffix = as_idn.to_string();
                }
            }
            Key::Integral => {
                let content;
                let _ = bracketed!(content in rest);
                *self.int = parse_integral(&content)?;
            }
            Key::ValueName => {
                let as_str: syn::LitStr = rest.parse()?;
                let as_idn: Ident = syn::parse_str(&as_str.value())?;
                self.value_name = as_idn.to_string();
            }
            Key::HideIn => self.hide_in = rest.parse()?,
            _ => return Ok(false),
        };

        return Ok(true);
    }
}

/// The nested `integral = [..]` configuration (also `[]` when absent): the
/// default profile, resolved, without facets, `produce`, `non_zero` or a
/// domain that excludes zero.
fn parse_integral(content: ParseStream) -> syn::Result<integral_cfg::Cfg> {
    let mut int = integral_cfg::Cfg::parse_with_konst_unresolved(content)?;
    if int.facets_present() {
        return content
            .span()
            .fail("bitflag integral configuration does not support facets");
    }
    if !int.ty_out.is_self() {
        return int
            .ty_out
            .conversion
            .fail("bitflag integral configuration does not support produce");
    }
    if !int.profile.is_default() {
        return content.span().fail(
            "bitflag integral configuration only supports the default profile",
        );
    }
    int.resolve()?;
    if int.non_zero == integral_cfg::NonZeroPolicy::Require {
        return content.span().fail(
            "bitflag integral configuration does not support `non_zero`",
        );
    }
    if int.validation.domain_excludes_zero() {
        return content
            .span()
            .fail("bitflag integral domain must include zero");
    }
    if int.requires_non_zero_storage() {
        return content
            .span()
            .fail("bitflag integral configuration cannot use nonzero storage");
    }
    return Ok(int);
}

// =============================================================================

/// A name bitflag generates.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum BitflagName {
    /// `FlagSeal`: the root friend protocol's seal.
    SealTrait,
    /// `FlagBit`: the root friend capability for the enum's bit operators.
    BitTrait,
    /// `conv_{snake}`: the seal's conversion.
    Conversion,
    /// `{Enum}Value`: the value type; `value_name` and `suffix` choose it.
    Value,
}

impl NameKey for BitflagName {
    type Owner = Ident;

    fn default_name(
        self,
        owner: &Ident,
    ) -> String {
        return match self {
            Self::SealTrait => "FlagSeal".to_string(),
            Self::BitTrait => Capability::Bit.flag_ident().to_string(),
            Self::Conversion => {
                format!("conv_{}", zz_runner::snake_case_of(&owner.to_string()))
            }
            Self::Value => format!("{}Value", owner.unraw()),
        };
    }

    fn default_span(
        self,
        owner: &Ident,
    ) -> Span {
        return match self {
            Self::Value => owner.span(),
            Self::SealTrait | Self::BitTrait | Self::Conversion => {
                Span::call_site()
            }
        };
    }
}

pub(crate) type BitFlags = Flags<BitFlag>;

/// A bitflag `with`/`without` flag.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum BitFlag {
    Clone,
    Copy,
    Hash,
    Debug,
    MakeValue,
    ImplValue,
    Display,
    Repr,
    Serde,
    SerdeWithAuto,
    SerdeWithName,
    SerdeWithValue,
}

impl Flag for BitFlag {
    const ROWS: &'static [FlagRow<Self>] = &[
        FlagRow::on(Self::Clone, "clone"),
        FlagRow::on(Self::Copy, "copy"),
        FlagRow::on(Self::Hash, "hash"),
        FlagRow::on(Self::Debug, "debug"),
        FlagRow::on(Self::MakeValue, "make_value"),
        FlagRow::on(Self::ImplValue, "impl_value"),
        FlagRow::off(Self::Display, "display"),
        FlagRow::on(Self::Repr, "repr"),
        FlagRow::off(Self::Serde, "serde"),
        FlagRow::off(Self::SerdeWithAuto, "serde_with_auto"),
        FlagRow::off(Self::SerdeWithName, "serde_with_name"),
        FlagRow::off(Self::SerdeWithValue, "serde_with_value"),
    ];

    fn index(self) -> u32 {
        return self as u32;
    }
}

// Every row of `ROWS` is at its variant's index.
const _: () = {
    let rows = <BitFlag as Flag>::ROWS;
    let mut index = 0;
    while index < rows.len() {
        assert!(rows[index].flag as usize == index);
        index += 1;
    }
};
