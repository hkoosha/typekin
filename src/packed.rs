//! `packed`: a named-field struct becomes a transparent unsigned integer
//! whose fields are typed bit windows, `#[bits(LO..HI)]` or `#[bit(N)]`,
//! bit 0 least significant.
//!
//! A window holds the low bits of its field's raw value; a read restores
//! the raw value as `as` widens it. Every field value is valid for its type,
//! and rustc proves at compile time that each wrapper field's values fit its
//! window, so `make` and the `with_*` setters never validate; only raw
//! ingress (`try_make`, Serde) does. Unclaimed bits are always zero, which
//! keeps the representation canonical: raw equality is field equality.
//!
//! The integral core (`try_make`, `raw`, comparisons, `Hash`, Serde, and the
//! `Within` source items) comes from `integral::ekran_with`, validated by one
//! generated callback.

use std::collections::BTreeSet;

use proc_macro2::{
    Ident,
    Literal,
    Span,
    TokenStream,
};
use quote::{
    ToTokens,
    format_ident,
    quote,
};
use syn::ext::IdentExt as _;
use syn::parse::{
    Parse,
    ParseStream,
    Parser as _,
};
use syn::spanned::Spanned as _;
use syn::{
    Attribute,
    Fields,
    Item,
    ItemStruct,
    LitInt,
    Meta,
    Token,
    Type,
    Visibility,
    parse_quote,
};

use crate::integral::StoragePlan;
use crate::integral_cfg::{
    IntegralFlag,
    IntegralFlags,
    IntegralName,
    NonZeroPolicy,
};
use crate::zz_domain_within::{
    emit_check,
    emit_messages,
};
use crate::zz_domain_within_const::__TYPEKIN_WITHIN_BUDGET;
use crate::zz_flags::{
    Flag,
    FlagGrammar,
    FlagSettings,
    Flags,
    KonstFlags,
};
use crate::zz_hide::{
    Envelope,
    HideCfg,
    HideIn,
    Hideable,
    inner_visibility,
};
use crate::zz_lib::{
    Key,
    Konst,
    Konsts,
    NUMERIC_LINTS,
    lint_scope,
    one_or_list,
    parse_inner,
};
use crate::zz_names::{
    NameKey,
    Names,
    ResolvedNames,
};
use crate::zz_runner::MkErr;
use crate::zz_values::{
    F,
    N,
};
use crate::{
    integral,
    integral_cfg,
    marker_tree,
};

/// The `with`/`without` grammar of `packed`: its own flags only.
const GRAMMAR: FlagGrammar = FlagGrammar {
    konsts: &[],
    removals: false,
};

/// The item shape `packed` accepts.
const SHAPE: &str = "`packed` needs a non-generic struct with named fields";

/// A field's position attributes.
const BITS: &str = "bits";
const BIT: &str = "bit";
const POSITION: &str = "each field needs one `#[bits(..)]` or `#[bit(..)]`";

/// The field type that takes one bit.
const BOOL: &str = "bool";

/// Field types rejected by name: floats and text.
const FLOATS: [&str; 2] = [
    "f16",
    "f128",
];
const TEXT: &str = "String";

/// The integral inherent functions every packed record has, which a field
/// name cannot take.
const INTEGRAL_FNS: [&str; 2] = [
    "raw",
    "try_make",
];

/// The integral core every packed record keeps: storage and layout, raw
/// ingress and access, the `Within` source items, and the friend protocol
/// that carries the comparisons.
const CORE: &[IntegralFlag] = &[
    IntegralFlag::Clone,
    IntegralFlag::Copy,
    IntegralFlag::Repr,
    IntegralFlag::Assertions,
    IntegralFlag::FnTryMake,
    IntegralFlag::FnConvRaw,
    IntegralFlag::FnOrPanic,
    IntegralFlag::WithinSource,
    IntegralFlag::FriendSeal,
    IntegralFlag::FriendzoneSeal,
    IntegralFlag::FriendzoneFriendRel,
];

/// The integral flags each packed flag turns on.
const FORWARDED: &[(PackedFlag, &[IntegralFlag])] = &[
    (
        PackedFlag::Eq,
        &[
            IntegralFlag::PartialEq,
            IntegralFlag::Eq,
        ],
    ),
    (PackedFlag::Hash, &[IntegralFlag::Hash]),
    (
        PackedFlag::Ord,
        &[
            IntegralFlag::PartialOrd,
            IntegralFlag::Ord,
        ],
    ),
    (PackedFlag::Serde, &[IntegralFlag::Serde]),
];

/// The whole expansion: the record first, then the values struct and the
/// generated code.
pub(crate) fn ekran(
    cfg: Box<Cfg>,
    item: PackedItem,
) -> syn::Result<TokenStream> {
    let PackedItem(mut item) = item;
    let repr = cfg.repr()?;
    reject_generics(&item)?;

    let mut envelope = Envelope::new(&cfg.hide_in, &item, &item.vis);
    envelope.prepare(&mut item, None::<&marker_tree::Cfg>)?;
    let fields = parse_fields(&item, repr, envelope.is_hidden())?;
    let maker = Maker::new(cfg, item.ident.clone(), repr, fields)?;

    let declared = envelope.declared().clone();
    let values_visibility = match envelope.is_hidden() {
        true => inner_visibility(&declared),
        false => declared.clone(),
    };
    if maker.cfg.flags.has(PackedFlag::ValuesStruct) {
        envelope.export(declared, maker.names.get(PackedName::Values));
    }
    let attributes = maker.attributes();
    let storage = maker.storage.field_type();
    item.fields = Fields::Unnamed(parse_quote! { (#storage) });
    item.semi_token = Some(Default::default());
    let implementation = maker.ekran(&values_visibility)?;

    return Ok(envelope.finish(attributes, item, implementation));
}

/// `packed` takes no generic parameters and no `where` clause.
fn reject_generics(item: &ItemStruct) -> syn::Result<()> {
    if !item.generics.params.is_empty() {
        return item.generics.params.fail(SHAPE);
    }
    if let Some(clause) = &item.generics.where_clause {
        return clause.fail(SHAPE);
    }
    return Ok(());
}

// =============================================================================

/// The annotated struct; any other item fails at its span.
pub(crate) struct PackedItem(ItemStruct);

impl Parse for PackedItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return match input.parse::<Item>()? {
            Item::Struct(item) => Ok(Self(item)),
            other => other.fail(SHAPE),
        };
    }
}

impl ToTokens for PackedItem {
    fn to_tokens(
        &self,
        tokens: &mut TokenStream,
    ) {
        self.0.to_tokens(tokens);
    }
}

impl Hideable for PackedItem {
    fn ident(&self) -> &Ident {
        return Hideable::ident(&self.0);
    }

    fn visibility_mut(&mut self) -> &mut Visibility {
        return self.0.visibility_mut();
    }

    fn attributes(&self) -> impl Iterator<Item = &Attribute> {
        return Hideable::attributes(&self.0);
    }

    fn rewrite_nested_visibility(&mut self) {
        self.0.rewrite_nested_visibility();
    }
}

// =============================================================================

/// Bits `lo..hi` of the raw value, bit 0 least significant.
#[derive(Clone, Copy, Debug)]
struct Window {
    lo: u32,
    hi: u32,
    /// Written `#[bit(N)]`.
    single: bool,
}

impl Window {
    fn width(self) -> u32 {
        return self.hi - self.lo;
    }

    /// The window's bits in place.
    fn mask(self) -> u128 {
        return ones(self.width()) << self.lo;
    }

    fn overlaps(
        self,
        that: Self,
    ) -> bool {
        return self.lo < that.hi && that.lo < self.hi;
    }

    /// `bit 9` or `bits 10..16`, for diagnostics.
    fn text(self) -> String {
        return match self.single {
            true => format!("bit {}", self.lo),
            false => format!("bits {}..{}", self.lo, self.hi),
        };
    }

    /// `verb` after [`Self::text`]: `exceeds` after one bit, else `exceed`.
    fn agree(
        self,
        verb: &str,
    ) -> String {
        return match self.single {
            true => format!("{verb}s"),
            false => verb.to_string(),
        };
    }
}

/// The low `bits` bits set.
fn ones(bits: u32) -> u128 {
    return match bits {
        128 => u128::MAX,
        _ => (1u128 << bits) - 1,
    };
}

/// `n bit` or `n bits`.
fn bit_count(bits: u32) -> String {
    return match bits {
        1 => "1 bit".to_string(),
        _ => format!("{bits} bits"),
    };
}

/// A field type's encoding.
#[derive(Clone, Copy, Debug)]
enum Shape {
    /// One bit.
    Bool,
    /// Exactly its own width, cast.
    Primitive(N),
    /// A typekin integer type: built from its raw value with `try_make`, its
    /// fit proven at compile time.
    Wrapper,
}

/// One field: its accessors' name, visibility, and docs, and its window.
struct Field {
    name: Ident,
    vis: Visibility,
    docs: Vec<Attribute>,
    ty: Type,
    shape: Shape,
    window: Window,
}

/// Every field with its window, checked against `repr` and each other.
/// Hidden, a private field's accessors become visible to the defining
/// module (`pub(super)`), as the field itself was.
fn parse_fields(
    item: &ItemStruct,
    repr: N,
    hidden: bool,
) -> syn::Result<Vec<Field>> {
    let Fields::Named(named) = &item.fields
    else {
        return item.ident.fail(SHAPE);
    };
    if named.named.is_empty() {
        return item.ident.fail("`packed` needs at least one field");
    }
    let mut fields = Vec::<Field>::new();
    for field in &named.named {
        let name = field.ident.clone().expect("named fields have names");
        let (window, span) = window_of(field, repr)?;
        let shape = shape_of(&field.ty, window, span)?;
        if let Some(other) = fields.iter().find(|it| it.window.overlaps(window))
        {
            return span.fail(format!(
                "{} of `{}` {} {} of `{}`",
                window.text(),
                name.unraw(),
                window.agree("overlap"),
                other.window.text(),
                other.name.unraw(),
            ));
        }
        fields.push(Field {
            vis: accessor_visibility(&field.vis, hidden),
            docs: field
                .attrs
                .iter()
                .filter(|it| it.path().is_ident("doc"))
                .cloned()
                .collect(),
            ty: field.ty.clone(),
            name,
            shape,
            window,
        });
    }
    return Ok(fields);
}

/// The accessors' visibility: the field's, one module deeper when hidden.
/// `pub(super)` was already re-anchored (`Envelope::prepare`).
fn accessor_visibility(
    visibility: &Visibility,
    hidden: bool,
) -> Visibility {
    let private = match visibility {
        Visibility::Inherited => true,
        Visibility::Restricted(it) => it.path.is_ident("self"),
        Visibility::Public(_) => false,
    };
    return match hidden && private {
        true => parse_quote! { pub(super) },
        false => visibility.clone(),
    };
}

/// The field's one position attribute; anything but docs beside it fails.
fn window_of(
    field: &syn::Field,
    repr: N,
) -> syn::Result<(Window, Span)> {
    let mut found = None;
    for attribute in &field.attrs {
        let path = attribute.path();
        if path.is_ident("doc") {
            continue;
        }
        let single = path.is_ident(BIT);
        if !single && !path.is_ident(BITS) {
            return attribute.fail(
                "packed fields take only `#[bits(..)]`, `#[bit(..)]`, and \
                doc comments",
            );
        }
        if found.is_some() {
            return attribute.fail(POSITION);
        }
        let window = window_in(attribute, single, repr)?;
        found = Some((window, attribute.span()));
    }
    return match found {
        Some(it) => Ok(it),
        None => field.ident.fail(POSITION),
    };
}

/// `#[bits(LO..HI)]`, `#[bits(LO..=HI)]`, or `#[bit(N)]`, within `repr`.
fn window_in(
    attribute: &Attribute,
    single: bool,
    repr: N,
) -> syn::Result<Window> {
    let only = match single {
        true => "`#[bit(..)]` takes only a bit index",
        false => "`#[bits(..)]` takes only a bit range",
    };
    let Meta::List(list) = &attribute.meta
    else {
        return attribute.fail(only);
    };
    let parsed = match single {
        true => parse_bit.parse2(list.tokens.clone()),
        false => parse_range.parse2(list.tokens.clone()),
    };
    let (lo, end, written) = parsed.map_err(|_| list.errorful(only))?;
    let Some(hi) = end.filter(|hi| lo < *hi)
    else {
        return list.fail(format!("bits {written} are empty"));
    };
    let window = Window { lo, hi, single };
    let bits = repr_bits(repr);
    if hi > bits {
        return list.fail(format!(
            "{} {} `{}` ({bits} bits)",
            window.text(),
            window.agree("exceed"),
            repr.rust_name(),
        ));
    }
    return Ok(window);
}

/// `N`: the bit and its end.
fn parse_bit(input: ParseStream) -> syn::Result<(u32, Option<u32>, String)> {
    let bit = input.parse::<LitInt>()?.base10_parse::<u32>()?;
    return Ok((bit, bit.checked_add(1), bit.to_string()));
}

/// `LO..HI` or `LO..=HI`: the start, the exclusive end, and the range as
/// written.
fn parse_range(input: ParseStream) -> syn::Result<(u32, Option<u32>, String)> {
    let lo = input.parse::<LitInt>()?.base10_parse::<u32>()?;
    let inclusive = input.peek(Token![..=]);
    match inclusive {
        true => input.parse::<Token![..=]>().map(|_| ())?,
        false => input.parse::<Token![..]>().map(|_| ())?,
    }
    let hi = input.parse::<LitInt>()?.base10_parse::<u32>()?;
    return Ok(match inclusive {
        true => (lo, hi.checked_add(1), format!("{lo}..={hi}")),
        false => (lo, Some(hi), format!("{lo}..{hi}")),
    });
}

/// The bit width of a fixed-width integer type.
fn repr_bits(repr: N) -> u32 {
    return repr.bits_for_pointer_width(64) as u32;
}

/// The field type's encoding; `bool` and primitives must fill their window.
fn shape_of(
    ty: &Type,
    window: Window,
    span: Span,
) -> syn::Result<Shape> {
    let Type::Path(path) = ty
    else {
        return ty.fail(
            "packed fields are `bool`, fixed-width primitive integers, or \
            typekin integer types",
        );
    };
    let Some(name) = path.path.get_ident().filter(|_| path.qself.is_none())
    else {
        return Ok(Shape::Wrapper);
    };
    let text = name.to_string();
    if text == BOOL {
        if window.width() != 1 {
            return span.fail("`bool` takes exactly one bit; use `#[bit(N)]`");
        }
        return Ok(Shape::Bool);
    }
    if let Some(primitive) = N::of(&text) {
        if matches!(primitive, N::USIZ | N::ISIZ) {
            return ty.fail(format!(
                "`{text}` has a target-dependent width; use a fixed-width \
                integer"
            ));
        }
        let bits = repr_bits(primitive);
        if window.width() != bits {
            return span.fail(format!(
                "`{text}` needs exactly {bits} bits; use a wrapper with a \
                narrower `domain`"
            ));
        }
        return Ok(Shape::Primitive(primitive));
    }
    if F::of(&text).is_some() || FLOATS.contains(&text.as_str()) {
        return ty.fail("packed fields cannot be floats");
    }
    if text == TEXT {
        return ty.fail("packed fields cannot be text");
    }
    return Ok(Shape::Wrapper);
}

// =============================================================================

/// The record's generated code.
struct Maker {
    cfg: Box<Cfg>,
    ty: Ident,
    repr: N,
    /// The repr's identifier: `u32`.
    el: Ident,
    fields: Vec<Field>,
    names: ResolvedNames<PackedName>,
    /// The integral names the field types publish.
    integral: ResolvedNames<IntegralName>,
    storage: StoragePlan,
    int: Box<integral_cfg::Cfg>,
    /// `const` on the accessors (`konst_value`).
    konst: Option<TokenStream>,
    /// `const` on the validator whenever a const item calls it.
    valid_konst: Option<TokenStream>,
}

impl Maker {
    fn new(
        cfg: Box<Cfg>,
        ty: Ident,
        repr: N,
        fields: Vec<Field>,
    ) -> syn::Result<Self> {
        let owner = PackedOwner {
            ty: ty.clone(),
            fields: fields.iter().map(|it| it.name.clone()).collect(),
        };
        let mut names = Names::default();
        if let Some(name) = &cfg.values_name {
            names.choose(PackedName::Values, name.clone());
        }
        let names = names.resolve(owner, BTreeSet::new());
        let integral = Names::<IntegralName>::default()
            .resolve(ty.clone(), BTreeSet::new());
        let non_zero = cfg.flags.has(PackedFlag::NonZero);
        if non_zero
            && fields.iter().all(|it| !matches!(it.shape, Shape::Wrapper))
        {
            return cfg.explicit_span(PackedFlag::NonZero).fail(
                "`non_zero` needs a field whose type can exclude zero; `bool` \
                and primitive fields admit zero",
            );
        }
        let int = integral_cfg(&cfg, &names.get(PackedName::ValidFn))?;
        let konst = Konst::konst(int.konst_for(&[Konsts::Value]));
        let valid_konst = Konst::konst(
            int.konst_for(&[Konsts::TryMake])
                || int.konst_for(&[Konsts::Value])
                || int.helper_konst(),
        );
        let this = Self {
            el: format_ident!("{}", repr.rust_name()),
            storage: StoragePlan::new(repr, non_zero, false),
            cfg,
            ty,
            repr,
            fields,
            names,
            integral,
            int,
            konst,
            valid_konst,
        };
        this.reject_collisions()?;
        return Ok(this);
    }

    /// A field name that a generated function already takes fails.
    fn reject_collisions(&self) -> syn::Result<()> {
        let mut taken = INTEGRAL_FNS.map(str::to_string).to_vec();
        taken.push(self.integral.get(IntegralName::OrPanic).to_string());
        taken.push(self.integral.get(IntegralName::WithinRawFn).to_string());
        if self.cfg.flags.has(PackedFlag::ValuesStruct) {
            taken.push(self.names.get(PackedName::MakeFn).to_string());
            taken.push(self.names.get(PackedName::FieldsFn).to_string());
        }
        taken.extend((0..self.fields.len()).map(|index| {
            return self.names.get(PackedName::Setter(index)).to_string();
        }));
        for field in &self.fields {
            let name = field.name.unraw().to_string();
            if taken.contains(&name) {
                return field.name.fail(format!(
                    "field `{name}` collides with the generated `{name}()`"
                ));
            }
        }
        return Ok(());
    }

    /// `repr(transparent)`, `Copy`, and the (possibly const) `Clone` derive.
    fn attributes(&self) -> TokenStream {
        let derive_clone = self.cfg.konst.derive(quote! { Clone });
        return quote! {
            #[repr(transparent)]
            #[derive(Copy)]
            #derive_clone
        };
    }

    fn ekran(
        self,
        values_visibility: &Visibility,
    ) -> syn::Result<TokenStream> {
        let values = self.ekran_values(values_visibility);
        let validator = self.ekran_validator();
        let accessors = self.ekran_accessors();
        let debug = self.ekran_debug();
        let proofs = self.ekran_proofs();
        let core =
            integral::ekran_with(self.ty, self.el, self.int, self.storage)?;
        let scope = lint_scope(
            NUMERIC_LINTS,
            quote! {
                #core

                #validator

                #accessors

                #debug

                #proofs
            },
        );
        return Ok(quote! {
            #values

            #scope
        });
    }

    // -------------------------------------------------------------------------
    // Encoding

    /// A shift amount or bit count.
    fn amount(value: u32) -> Literal {
        return Literal::u32_unsuffixed(value);
    }

    /// A mask, in hexadecimal, typed by its context.
    fn mask(value: u128) -> LitInt {
        return LitInt::new(&format!("{value:#x}"), Span::call_site());
    }

    /// `field`'s bits of `raw`, shifted down.
    fn bits(
        &self,
        field: &Field,
        raw: &TokenStream,
    ) -> TokenStream {
        let lo = Self::amount(field.window.lo);
        let ones = Self::mask(ones(field.window.width()));
        return quote! { ((#raw >> #lo) & #ones) };
    }

    /// A wrapper field's raw value from its bits of `raw`, as its
    /// `try_make` takes it: sign-extended when its raw type is signed.
    fn wrapper_raw(
        &self,
        field: &Field,
        raw: &TokenStream,
    ) -> TokenStream {
        let bits = self.bits(field, raw);
        let pad = Self::amount(repr_bits(self.repr) - field.window.width());
        let signed = format_ident!("{}", self.repr.signed_rust_name());
        let ty = &field.ty;
        let raw_type = self.integral.get(IntegralName::WithinRaw);
        return quote! {
            if <#ty>::#raw_type.0 {
                (((#bits << #pad) as #signed) >> #pad) as _
            }
            else {
                #bits as _
            }
        };
    }

    /// `value`, a valid value of `field`'s type, in place in the raw type.
    /// An unsigned wrapper's value is proven to fit its window as it is; a
    /// signed one's sign extension is masked off.
    fn encode(
        &self,
        field: &Field,
        value: &TokenStream,
    ) -> TokenStream {
        let el = &self.el;
        let lo = Self::amount(field.window.lo);
        return match field.shape {
            Shape::Bool => quote! { ((#value as #el) << #lo) },
            Shape::Primitive(primitive) => {
                let unsigned =
                    format_ident!("{}", primitive.unsigned_rust_name());
                quote! { ((#value as #unsigned as #el) << #lo) }
            }
            Shape::Wrapper => {
                let ty = &field.ty;
                let raw = self.integral.get(IntegralName::WithinRawFn);
                let raw_type = self.integral.get(IntegralName::WithinRaw);
                let ones = Self::mask(ones(field.window.width()));
                quote! {
                    ({
                        let bits = <#ty>::#raw(#value) as #el;
                        if <#ty>::#raw_type.0 {
                            bits & #ones
                        }
                        else {
                            bits
                        }
                    } << #lo)
                }
            }
        };
    }

    /// The record built from `raw`, which every field value fits: debug
    /// builds check it, and storage takes the safe path, so a wrong proof
    /// cannot reach `NonZero::new_unchecked`.
    fn construct(&self) -> TokenStream {
        let valid = self.names.get(PackedName::ValidFn);
        let storage = self.storage.storage_expression(quote! { raw }, false);
        return quote! {
            ::core::debug_assert!(
                #valid(raw),
                "typekin packed record invariant violated",
            );
            return Self(#storage);
        };
    }

    // -------------------------------------------------------------------------
    // Emitters

    /// The field values of `make` and `fields`.
    fn ekran_values(
        &self,
        visibility: &Visibility,
    ) -> TokenStream {
        if !self.cfg.flags.has(PackedFlag::ValuesStruct) {
            return TokenStream::new();
        }

        let values = self.names.get(PackedName::Values);
        let flags = &self.cfg.flags;
        let debug = flags.has(PackedFlag::Debug).then(|| quote! { , Debug });
        let eq = flags
            .has(PackedFlag::Eq)
            .then(|| quote! { , PartialEq, Eq });
        let hash = flags.has(PackedFlag::Hash).then(|| quote! { , Hash });
        let doc = format!(
            " The field values of [`{}`], for `{}` and `{}`.",
            self.ty.unraw(),
            self.names.get(PackedName::MakeFn),
            self.names.get(PackedName::FieldsFn),
        );
        let fields = self.fields.iter().map(|field| {
            let Field {
                name,
                vis,
                docs,
                ty,
                ..
            } = field;
            return quote! {
                #(#docs)*
                #vis #name: #ty,
            };
        });
        return quote! {
            #[doc = #doc]
            #[derive(Clone, Copy #debug #eq #hash)]
            #visibility struct #values {
                #(#fields)*
            }
        };
    }

    /// The validation callback of raw ingress: unclaimed bits are zero, and
    /// each wrapper field's bits are a valid value of its type.
    fn ekran_validator(&self) -> TokenStream {
        let el = &self.el;
        let valid = self.names.get(PackedName::ValidFn);
        let konst = &self.valid_konst;
        let raw = quote! { raw };
        let claimed = self.fields.iter().fold(0, |it, field| {
            return it | field.window.mask();
        });
        let unclaimed = ones(repr_bits(self.repr)) & !claimed;
        let gaps = (unclaimed != 0).then(|| {
            let unclaimed = Self::mask(unclaimed);
            quote! {
                if raw & #unclaimed != 0 {
                    return false;
                }
            }
        });
        let checks = self
            .fields
            .iter()
            .filter(|it| matches!(it.shape, Shape::Wrapper))
            .map(|field| {
                let ty = &field.ty;
                let value = self.wrapper_raw(field, &raw);
                return quote! {
                    if <#ty>::try_make(#value).is_err() {
                        return false;
                    }
                };
            });
        return quote! {
            #[must_use]
            #[inline(always)]
            #konst fn #valid(raw: #el) -> bool {
                #gaps
                #(#checks)*
                return true;
            }
        };
    }

    /// `make` and `fields`, then each field's getter and setter.
    fn ekran_accessors(&self) -> TokenStream {
        let ty = &self.ty;
        let values = self.values_fns();
        let fields = self.fields.iter().enumerate().map(|(index, field)| {
            let getter = self.getter(field);
            let setter = self.setter(index, field);
            return quote! { #getter #setter };
        });
        return quote! {
            impl #ty {
                #values
                #(#fields)*
            }
        };
    }

    /// `make` from the values struct and `fields` back to it.
    fn values_fns(&self) -> Option<TokenStream> {
        if !self.cfg.flags.has(PackedFlag::ValuesStruct) {
            return None;
        }

        let el = &self.el;
        let konst = &self.konst;
        let values = self.names.get(PackedName::Values);
        let make = self.names.get(PackedName::MakeFn);
        let fields = self.names.get(PackedName::FieldsFn);
        let parts = self.fields.iter().map(|field| {
            let name = &field.name;
            return self.encode(field, &quote! { values.#name });
        });
        let names = self.fields.iter().map(|it| &it.name);
        let construct = self.construct();
        return Some(quote! {
            /// Builds the record from its field values. Cannot fail: each
            /// value is valid for its type, and each type's values fit its
            /// bits (proven at compile time).
            #[must_use]
            #[inline(always)]
            pub #konst fn #make(values: #values) -> Self {
                let raw: #el = #(#parts)|*;
                #construct
            }

            /// The record's field values.
            #[must_use]
            #[inline(always)]
            pub #konst fn #fields(self) -> #values {
                return #values {
                    #(#names: Self::#names(self),)*
                };
            }
        });
    }

    /// The field's value, read from its bits.
    fn getter(
        &self,
        field: &Field,
    ) -> TokenStream {
        let Field {
            name,
            vis,
            docs,
            ty,
            ..
        } = field;
        let el = &self.el;
        let konst = &self.konst;
        let self_raw = self.storage.extract(quote! { self.0 });
        let raw = quote! { raw };
        let lo = Self::amount(field.window.lo);
        let body = match field.shape {
            Shape::Bool => quote! { return ((raw >> #lo) & 1) != 0; },
            Shape::Primitive(_) => quote! { return (raw >> #lo) as #ty; },
            Shape::Wrapper => {
                let value = self.wrapper_raw(field, &raw);
                quote! {
                    return match <#ty>::try_make(#value) {
                        ::core::result::Result::Ok(value) => value,
                        ::core::result::Result::Err(_) => {
                            ::core::panic!("typekin packed field invariant violated")
                        }
                    };
                }
            }
        };
        return quote! {
            #(#docs)*
            #[must_use]
            #[inline(always)]
            #vis #konst fn #name(self) -> #ty {
                let raw: #el = #self_raw;
                #body
            }
        };
    }

    /// The record with the field replaced: its bits cleared, then set.
    fn setter(
        &self,
        index: usize,
        field: &Field,
    ) -> TokenStream {
        let Field {
            name,
            vis,
            docs,
            ty,
            ..
        } = field;
        let el = &self.el;
        let konst = &self.konst;
        let setter = self.names.get(PackedName::Setter(index));
        let self_raw = self.storage.extract(quote! { self.0 });
        let kept =
            Self::mask(ones(repr_bits(self.repr)) & !field.window.mask());
        let value = self.encode(field, &quote! { #name });
        let construct = self.construct();
        return quote! {
            #(#docs)*
            #[must_use]
            #[inline(always)]
            #vis #konst fn #setter(self, #name: #ty) -> Self {
                let raw: #el = (#self_raw & #kept) | #value;
                #construct
            }
        };
    }

    /// `Record { field: value, .. }` through the getters.
    fn ekran_debug(&self) -> TokenStream {
        if !self.cfg.flags.has(PackedFlag::Debug) {
            return TokenStream::new();
        }

        let ty = &self.ty;
        let text = ty.unraw().to_string();
        let fields = self.fields.iter().map(|field| {
            let name = &field.name;
            let text = name.unraw().to_string();
            return quote! { .field(#text, &Self::#name(*self)) };
        });
        return quote! {
            impl ::core::fmt::Debug for #ty {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    return f.debug_struct(#text) #(#fields)* .finish();
                }
            }
        };
    }

    // -------------------------------------------------------------------------
    // Proofs

    /// The shared check, then one fit proof per wrapper field and the
    /// `non_zero` proof, each a constant rustc evaluates during `cargo
    /// check`.
    fn ekran_proofs(&self) -> TokenStream {
        let fits = self
            .fields
            .iter()
            .filter(|it| matches!(it.shape, Shape::Wrapper))
            .map(|field| self.fit_proof(field))
            .collect::<Vec<_>>();
        if fits.is_empty() {
            return TokenStream::new();
        }

        let check = emit_check();
        let messages = emit_messages();
        let non_zero = self.non_zero_proof();
        return quote! {
            #check

            #messages

            #(#fits)*

            #non_zero
        };
    }

    /// Every value of a wrapper field fits its window: its raw type is at
    /// least as wide, and when wider, its form (or 8-bit table) lies in the
    /// window's range, read as the field's raw type reads it.
    fn fit_proof(
        &self,
        field: &Field,
    ) -> TokenStream {
        let ty = &field.ty;
        let width = field.window.width();
        let w = Self::amount(width);
        let texts = FitTexts::new(self, field);
        let FitTexts {
            head,
            tail,
            wide_head,
            wide_tail,
            budget,
            steps,
            ..
        } = &texts;
        let raw_type = self.integral.get(IntegralName::WithinRaw);
        let form = self.integral.get(IntegralName::WithinForm);
        let half = ones(width - 1) + 1;
        let (signed_min, signed_max) = (
            Literal::u128_suffixed(half),
            Literal::u128_suffixed(half - 1),
        );
        let unsigned_max = Literal::u128_suffixed(ones(width));
        let opaque = self.fit_table(field, &texts);
        return quote! {
            const _: () = {
                let (signed, bits): (bool, u32) = <#ty>::#raw_type;
                if bits < #w {
                    __typekin_within_refute(#wide_head, (false, bits as u128), #wide_tail);
                }
                if bits > #w {
                    let (min, max): (__TypekinWithinValue, __TypekinWithinValue) =
                        if signed {
                            ((true, #signed_min), (false, #signed_max))
                        }
                        else {
                            ((false, 0u128), (false, #unsigned_max))
                        };
                    let window: &[__TypekinWithinPiece] = &[(0, min.0, min.1, max.0, max.1)];
                    match <#ty>::#form {
                        ::core::option::Option::Some(source) => {
                            match __typekin_within_subset(source, (1, window), min, max) {
                                __TypekinWithin::Proven => {}
                                __TypekinWithin::Range(negative, magnitude)
                                | __TypekinWithin::Domain(negative, magnitude) => {
                                    __typekin_within_refute(#head, (negative, magnitude), #tail)
                                }
                                __TypekinWithin::Budget(needed) => {
                                    __typekin_within_refute(#budget, (false, needed), #steps)
                                }
                            }
                        }
                        ::core::option::Option::None => #opaque,
                    }
                }
            };
        };
    }

    /// A field type without a form: its 8-bit table, when the window is
    /// narrower than 8 bits, else why it has no form.
    fn fit_table(
        &self,
        field: &Field,
        texts: &FitTexts,
    ) -> TokenStream {
        let ty = &field.ty;
        let width = field.window.width();
        let why = self.integral.get(IntegralName::WithinWhy);
        let FitTexts {
            head, tail, cannot, ..
        } = texts;
        let explain = quote! { __typekin_within_explain(#cannot, <#ty>::#why) };
        if width >= 8 {
            return explain;
        }

        let table = self.integral.get(IntegralName::WithinTable);
        let half = 1i16 << (width - 1);
        let (signed_min, signed_max) = (-half, half - 1);
        let unsigned_max = (1i16 << width) - 1;
        return quote! {
            match <#ty>::#table {
                ::core::option::Option::Some(table) => {
                    let (lo, hi): (i16, i16) = if signed {
                        (#signed_min, #signed_max)
                    }
                    else {
                        (0, #unsigned_max)
                    };
                    let mut index = 0usize;
                    while index < 256 {
                        let value: i16 = if signed {
                            index as i16 - 128
                        }
                        else {
                            index as i16
                        };
                        if table[index] && (value < lo || value > hi) {
                            __typekin_within_refute(
                                #head,
                                (value < 0, value.unsigned_abs() as u128),
                                #tail,
                            );
                        }
                        index += 1;
                    }
                }
                ::core::option::Option::None => #explain,
            }
        };
    }

    /// Some wrapper field excludes zero, so the record's raw value never is:
    /// its form has no piece holding zero, or its table rejects zero.
    fn non_zero_proof(&self) -> Option<TokenStream> {
        if !self.cfg.flags.has(PackedFlag::NonZero) {
            return None;
        }

        let raw_type = self.integral.get(IntegralName::WithinRaw);
        let form = self.integral.get(IntegralName::WithinForm);
        let table = self.integral.get(IntegralName::WithinTable);
        let excludes = self
            .fields
            .iter()
            .filter(|it| matches!(it.shape, Shape::Wrapper))
            .map(|field| {
                let ty = &field.ty;
                return quote! {
                    match <#ty>::#form {
                        ::core::option::Option::Some(form) => ::core::matches!(
                            __typekin_within_subset(
                                (1, &[(0, false, 0, false, 0)]),
                                form,
                                (false, 0),
                                (false, 0),
                            ),
                            __TypekinWithin::Domain(..)
                        ),
                        ::core::option::Option::None => match <#ty>::#table {
                            ::core::option::Option::Some(table) => {
                                !table[if <#ty>::#raw_type.0 { 128 } else { 0 }]
                            }
                            ::core::option::Option::None => false,
                        },
                    }
                };
            });
        let message = format!(
            "cannot prove `non_zero` for `{}`: every field admits zero",
            self.ty.unraw(),
        );
        return Some(quote! {
            const _: () = {
                if !(false #(|| #excludes)*) {
                    ::core::panic!(#message);
                }
            };
        });
    }
}

/// The message parts of one field's fit proof.
struct FitTexts {
    /// "`F` does not fit bits 10..16 of `T`: ", before a counterexample.
    head: String,
    /// " needs more than 6 bits", after it.
    tail: String,
    /// "`F` does not fit bits 10..20 of `T`: the window is wider than
    /// `F`'s ", before the raw type's width.
    wide_head: String,
    /// " bits".
    wide_tail: String,
    /// "cannot prove `F` fits bits 10..16 of `T`: ", before a reason.
    cannot: String,
    /// `cannot`, then "the check needs ", before the steps it needs.
    budget: String,
    /// " steps, above the subdomain proof budget of 65536".
    steps: String,
}

impl FitTexts {
    fn new(
        maker: &Maker,
        field: &Field,
    ) -> Self {
        let ty = field.ty.to_token_stream().to_string().replace(' ', "");
        let place =
            format!("{} of `{}`", field.window.text(), maker.ty.unraw());
        let width = field.window.width();
        let cannot = format!("cannot prove `{ty}` fits {place}: ");
        return Self {
            head: format!("`{ty}` does not fit {place}: "),
            tail: format!(" needs more than {}", bit_count(width)),
            wide_head: format!(
                "`{ty}` does not fit {place}: the window is wider than \
                `{ty}`'s "
            ),
            wide_tail: " bits".to_string(),
            budget: format!("{cannot}the check needs "),
            cannot,
            steps: format!(
                " steps, above the subdomain proof budget of \
                {__TYPEKIN_WITHIN_BUDGET}"
            ),
        };
    }
}

/// The integral core's configuration: `CORE` and the forwarded flags,
/// validated by the generated callback `valid`, and `NonZero` storage that
/// validation also checks, so `try_make`'s unchecked construction stays
/// sound whatever the `non_zero` proof says.
fn integral_cfg(
    cfg: &Cfg,
    valid: &Ident,
) -> syn::Result<Box<integral_cfg::Cfg>> {
    let mut int = integral_cfg::Cfg::without_konst();
    int.validation.callbacks = vec![parse_quote! { #valid }];
    int.resolve()?;
    int.flags = IntegralFlags::of(CORE);
    for (flag, forwarded) in FORWARDED {
        if cfg.flags.has(*flag) {
            for it in *forwarded {
                int.flags.set(*it, true);
            }
        }
    }
    if cfg.flags.has(PackedFlag::NonZero) {
        int.non_zero = NonZeroPolicy::Require;
        int.validation.require_non_zero(true);
    }
    int.embed_konst(cfg.konst, &KonstFlags::default())?;
    int.hide_in = cfg.hide_in.clone();
    return Ok(Box::new(int));
}

// =============================================================================

#[derive(Debug)]
pub(crate) struct Cfg {
    repr: Option<N>,
    konst: Konst,
    pub(crate) hide_in: HideIn,
    /// `values_name = Ident`: the values struct's name.
    values_name: Option<Ident>,
    settings: FlagSettings<PackedFlag>,
    flags: PackedFlags,
}

impl HideCfg for Cfg {
    fn hide_in(&self) -> &HideIn {
        return &self.hide_in;
    }
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self {
            repr: None,
            konst: Konst::default(),
            hide_in: HideIn::default(),
            values_name: None,
            settings: FlagSettings::default(),
            flags: PackedFlags::default(),
        };
        parse_inner(input, |key, rest| this.parse_attr(key, rest))?;
        this.settings.apply(&mut this.flags);
        if let Some((true, span)) = this.settings.flag(PackedFlag::Ord)
            && !this.flags.has(PackedFlag::Eq)
        {
            return span.fail("`ord` needs `eq`");
        }
        return Ok(this);
    }
}

impl Cfg {
    fn parse_attr(
        &mut self,
        key: Key,
        rest: ParseStream,
    ) -> syn::Result<bool> {
        match key {
            Key::Repr => self.repr = Some(parse_repr(rest)?),
            Key::Konst => self.konst = rest.parse()?,
            Key::HideIn => self.hide_in = rest.parse()?,
            Key::ValuesName => self.values_name = Some(rest.parse()?),
            Key::With | Key::Without => {
                reject_integral_flags(rest)?;
                self.settings.parse(rest, key == Key::With, &GRAMMAR)?;
            }
            other => {
                return rest.span().fail(format!(
                    "`packed` does not support `{}`",
                    other.name()
                ));
            }
        }
        return Ok(true);
    }

    /// The required `repr`.
    fn repr(&self) -> syn::Result<N> {
        return match self.repr {
            Some(repr) => Ok(repr),
            None => Span::call_site()
                .fail("`packed` requires `repr = u8 | u16 | u32 | u64 | u128`"),
        };
    }

    /// The span of an explicit setting of `flag`, else the call site.
    fn explicit_span(
        &self,
        flag: PackedFlag,
    ) -> Span {
        return self
            .settings
            .flag(flag)
            .map_or_else(Span::call_site, |(_, span)| span);
    }
}

/// A fixed-width unsigned integer: the layout must not depend on sign or
/// target.
fn parse_repr(input: ParseStream) -> syn::Result<N> {
    let name: Ident = input.parse()?;
    return match N::of(name.to_string()) {
        Some(N::USIZ) => name.fail(
            "`packed` needs a fixed-width `repr`; the width of `usize` \
            depends on the target",
        ),
        Some(repr) if repr.is_signed() => {
            name.fail("`packed` needs an unsigned `repr`")
        }
        Some(repr) => Ok(repr),
        None => name.fail("`repr` takes `u8`, `u16`, `u32`, `u64`, or `u128`"),
    };
}

/// An integral flag or `konst_*` flag `packed` does not take fails by name,
/// before the generic "unknown flag".
fn reject_integral_flags(input: ParseStream) -> syn::Result<()> {
    let fork = input.fork();
    for ident in one_or_list::<Ident>(&fork)? {
        let name = ident.to_string();
        if PackedFlag::of(&name).is_none()
            && (IntegralFlag::of(&name).is_some()
                || Konsts::of_in(&name, Konsts::INTEGRAL).is_some())
        {
            return ident.fail(format!("`packed` does not support `{name}`"));
        }
    }
    return Ok(());
}

// =============================================================================

/// What `packed`'s names derive from: the record and its fields.
pub(crate) struct PackedOwner {
    ty: Ident,
    fields: Vec<Ident>,
}

/// A name `packed` generates.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum PackedName {
    /// `{Type}Values`: the values struct; `values_name` chooses it.
    Values,
    /// `make`: the constructor from the values struct.
    MakeFn,
    /// `fields`: the values struct of a record.
    FieldsFn,
    /// `__typekin_packed_valid`: the validation of raw ingress.
    ValidFn,
    /// `with_{field}`: the setter of field `index`.
    Setter(usize),
}

impl NameKey for PackedName {
    type Owner = PackedOwner;

    fn default_name(
        self,
        owner: &PackedOwner,
    ) -> String {
        return match self {
            Self::Values => format!("{}Values", owner.ty.unraw()),
            Self::MakeFn => "make".to_string(),
            Self::FieldsFn => "fields".to_string(),
            Self::ValidFn => "__typekin_packed_valid".to_string(),
            Self::Setter(index) => {
                format!("with_{}", owner.fields[index].unraw())
            }
        };
    }

    fn default_span(
        self,
        owner: &PackedOwner,
    ) -> Span {
        return match self {
            Self::Values => owner.ty.span(),
            Self::MakeFn | Self::FieldsFn | Self::ValidFn | Self::Setter(_) => {
                Span::call_site()
            }
        };
    }
}

pub(crate) type PackedFlags = Flags<PackedFlag>;

/// A `packed` `with`/`without` flag.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum PackedFlag {
    Debug,
    Eq,
    Hash,
    ValuesStruct,
    NonZero,
    Ord,
    Serde,
}

impl Flag for PackedFlag {
    const ITEMS: &'static [Self] = &[
        Self::Debug,
        Self::Eq,
        Self::Hash,
        Self::ValuesStruct,
        Self::NonZero,
        Self::Ord,
        Self::Serde,
    ];
    const DEFAULT_ON: &'static [Self] = &[
        Self::Debug,
        Self::Eq,
        Self::Hash,
        Self::ValuesStruct,
    ];

    fn name(self) -> &'static str {
        return match self {
            Self::Debug => "debug",
            Self::Eq => "eq",
            Self::Hash => "hash",
            Self::ValuesStruct => "values_struct",
            Self::NonZero => "non_zero",
            Self::Ord => "ord",
            Self::Serde => "serde",
        };
    }

    fn index(self) -> u32 {
        return self as u32;
    }
}

// Every variant is in `ITEMS`, at its own index.
const _: () = {
    let items = <PackedFlag as Flag>::ITEMS;
    assert!(items.len() == PackedFlag::Serde as usize + 1);
    let mut index = 0;
    while index < items.len() {
        assert!(items[index] as usize == index);
        index += 1;
    }
};
