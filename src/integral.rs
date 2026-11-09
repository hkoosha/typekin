use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::fmt::Debug;

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
use syn::ext::IdentExt as _;
use syn::visit_mut::VisitMut as _;
use syn::{
    ConstParam,
    Field,
    Fields,
    GenericParam,
    Generics,
    ItemStruct,
    Path,
    Type,
    Visibility,
    parse_quote,
};

use crate::integral_cfg::{
    Cfg,
    IntegralFlag,
    IntegralName,
    Kind,
    Translation,
    translate,
};
use crate::integral_proof::ProofMemo;
use crate::integral_within::WithinProof;
use crate::zz_conv::{
    OperatorOutput,
    SelfConverter,
};
use crate::zz_friend::{
    Capability,
    Friend,
};
use crate::zz_hide::Envelope;
use crate::zz_lib::{
    Check,
    Konst,
    NUMERIC_LINTS,
    OnInvalid,
    lint_scope,
    repr_transparent,
};
use crate::zz_names::{
    Names,
    ResolvedNames,
};
use crate::zz_profile_integral::{
    RhsKind,
    Slot,
};
use crate::zz_runner::MkErr;
use crate::zz_values::{
    N,
    Num,
    OpKind,
    OpSpec,
    Width,
    by_width_grouped,
};

// An empty token stream means the item is valid on every supported target.
pub(crate) fn pointer_width_cfg(
    enabled: impl FnMut(Width) -> bool
) -> Option<TokenStream> {
    return by_width_grouped(Width::ALL.map(enabled))
        .into_iter()
        .find_map(|(cfg, enabled)| enabled.then_some(cfg));
}

/// Float types typekin names but does not support (diagnostic only).
const UNSUPPORTED_FLOATS: [&str; 2] = [
    "f16",
    "f128",
];

pub(crate) fn ekran(
    mut attr: Box<Cfg>,
    mut item: ItemStruct,
) -> syn::Result<TokenStream> {
    let target = IntegralTarget::new(item.ident.clone(), item.generics.clone());
    let attributes = declaration_attributes(&attr, &target);
    let field = value_field(&item)?;
    let (raw, el) = value_type(attr.kind, field)?;
    let field_visibility = field.vis.clone();
    attr.settle(raw, target.has_generics())?;
    let storage = StoragePlan::new(
        raw,
        attr.requires_non_zero_storage(),
        attr.requires_wrapping_storage(),
    );
    value_privacy(&mut attr, &target, &storage, &field_visibility)?;

    let mut envelope = Envelope::new(&attr.hide_in, &item, &item.vis);
    if envelope.is_hidden() {
        hidden_access_checks(&attr)?;
    }
    envelope.prepare(&mut item, attr.tree.as_ref())?;
    rewrite_value_field(&mut item, &target, &storage);

    let (impl_generics, where_clause) = target.impl_header(&[], &[]);
    let target_ty = target.ty();
    let manual_copy_clone = manual_copy_clone(&attr, &target);
    let markers = &target.const_markers;
    let const_markers = quote! { #(#markers)* };

    let lit = match attr.lit_macro()? {
        true => Some(crate::zz_lit::LitMacro::new(
            &item.ident,
            &item.generics,
            envelope.declared(),
            attr.lit_name.as_ref(),
            attr.lit_path.as_ref(),
        )?),
        false => None,
    };
    let lit_guard = lit
        .as_ref()
        .map(|lit| lit.guard(&impl_generics, &target_ty, &where_clause));
    let lit_definition = lit.as_ref().map(|lit| envelope.lit_definition(lit));

    let it = Maker::new_target(target, el, attr, storage)?.ekran()?;
    let scope = lint_scope(NUMERIC_LINTS, it);

    let implementation = quote::quote! {
        #const_markers

        #lit_definition

        #manual_copy_clone
        #lit_guard

        #scope
    };

    return Ok(envelope.finish(attributes, item, implementation));
}

/// `repr(transparent)`, `derive(Copy)` and the (possibly const) `Clone`
/// derive of a non-generic declaration.
fn declaration_attributes(
    cfg: &Cfg,
    target: &IntegralTarget,
) -> TokenStream {
    let repr = repr_transparent(cfg.flags.has(IntegralFlag::Repr));
    let derive_copy: Option<TokenStream> = (cfg.flags.has(IntegralFlag::Copy)
        && !target.has_generics())
    .then(|| quote! { #[derive(Copy)] });
    let derive_clone: Option<TokenStream> =
        (cfg.flags.has(IntegralFlag::Clone) && !target.has_generics())
            .then(|| cfg.wide.konst.derive(quote! { Clone }));
    return quote! { #repr #derive_copy #derive_clone };
}

/// The single unnamed value field of an integral declaration.
fn value_field(item: &ItemStruct) -> syn::Result<&Field> {
    let Fields::Unnamed(fields) = &item.fields
    else {
        return item.ident.fail(
            "expected a tuple struct with exactly one field \
            of primitive integral type",
        );
    };
    if fields.unnamed.len() != 1 {
        let span = fields
            .unnamed
            .iter()
            .nth(1)
            .map_or_else(|| item.ident.span(), syn::spanned::Spanned::span);
        return span.fail(
            "integral source declarations require exactly one primitive \
            value field; phantom fields are generated automatically",
        );
    }
    return Ok(fields.unnamed.first().unwrap());
}

/// The primitive type of the value field and its identifier.
fn value_type(
    kind: Kind,
    field: &Field,
) -> syn::Result<(Num, Ident)> {
    let field_name = match &field.ty {
        Type::Path(path) if path.qself.is_none() => path.path.get_ident(),
        _ => None,
    };
    let raw = field_name.and_then(|it| Num::of(it.to_string()));
    let raw = match (kind, raw) {
        (Kind::Int, Some(raw @ Num::Int(_)))
        | (Kind::Float, Some(raw @ Num::Float(_))) => raw,
        (Kind::Int, Some(Num::Float(_))) => {
            return field.ty.fail(
                "integral value field must be a concrete primitive integer; \
                use #[typekin::float] for f32/f64",
            );
        }
        (Kind::Int, None) => {
            return field.ty.fail(
                "integral value field must be a concrete primitive integer",
            );
        }
        (Kind::Float, _)
            if field_name.is_some_and(|it| {
                UNSUPPORTED_FLOATS.iter().any(|name| it == name)
            }) =>
        {
            return field.ty.fail("f16 and f128 are not supported");
        }
        (Kind::Float, _) => {
            return field.ty.fail("float value field must be f32 or f64");
        }
    };
    let el = field_name.expect("a typed field has a name").clone();
    return Ok((raw, el));
}

/// Rejects a visible value field where generics, storage rewriting or float
/// totality need it private; records the visibility on `cfg`.
fn value_privacy(
    cfg: &mut Cfg,
    target: &IntegralTarget,
    storage: &StoragePlan,
    field_visibility: &Visibility,
) -> syn::Result<()> {
    let private = match field_visibility {
        Visibility::Inherited => true,
        Visibility::Restricted(visibility) => visibility.path.is_ident("self"),
        Visibility::Public(_) => false,
    };
    if target.has_generics() && !private {
        return field_visibility
            .fail("generic integral types require a private value field");
    }
    if storage.rewrites_field() && !private {
        return field_visibility
            .fail("integral storage can only rewrite a private tuple field");
    }
    cfg.visible_value = !private;
    if cfg.kind == Kind::Float && !private {
        cfg.require_private_totality()?;
    }
    return Ok(());
}

/// Features that need the value field, which `hide_in` hides.
fn hidden_access_checks(cfg: &Cfg) -> syn::Result<()> {
    if let Some(get_raw) = &cfg.get_raw {
        return get_raw.fail(
            "get_raw needs access to the value field, which hide_in hides; \
            set hide_in = none",
        );
    }
    if !cfg.flags.has(IntegralFlag::FnOrPanic) {
        return cfg.explicit_span(IntegralFlag::FnOrPanic).fail(
            "a custom or_panic constructor needs access to the value \
                field, which hide_in hides; set hide_in = none",
        );
    }
    return Ok(());
}

/// Rewrites the value field to its storage type and appends the phantom
/// fields of the generic parameters.
fn rewrite_value_field(
    item: &mut ItemStruct,
    target: &IntegralTarget,
    storage: &StoragePlan,
) {
    let Fields::Unnamed(fields) = &mut item.fields
    else {
        unreachable!("the integral shape was validated above");
    };
    let field = fields
        .unnamed
        .first_mut()
        .expect("the integral shape was validated above");
    if storage.rewrites_field() {
        field.ty = syn::parse2(storage.field_type())
            .expect("storage plans emit valid field types");
    }
    for phantom_type in &target.phantom_types {
        fields.unnamed.push(parse_quote! {
            ::core::marker::PhantomData<#phantom_type>
        });
    }
}

/// `Copy` and `Clone` impls of a generic target, which cannot derive them.
fn manual_copy_clone(
    cfg: &Cfg,
    target: &IntegralTarget,
) -> TokenStream {
    let (impl_generics, where_clause) = target.impl_header(&[], &[]);
    let target_ty = target.ty();
    let manual_copy = (cfg.flags.has(IntegralFlag::Copy)
        && target.has_generics())
    .then(|| {
        quote! {
            impl #impl_generics ::core::marker::Copy for #target_ty
            #where_clause
            {}
        }
    });
    let manual_clone = (cfg.flags.has(IntegralFlag::Clone)
        && target.has_generics())
    .then(|| {
        let konst = Konst::konst(matches!(cfg.wide.konst, Konst::Nightly));
        let value = target.construct_storage(
            quote! { self.0 },
            cfg.flags.has(IntegralFlag::Assertions),
        );
        // A derive would bound the generic parameters by `Clone`; clippy
        // still asks for one whenever they are `Copy`.
        quote! {
            #[allow(clippy::expl_impl_clone_on_copy)]
            #konst impl #impl_generics ::core::clone::Clone for #target_ty
            #where_clause
            {
                #[inline(always)]
                fn clone(&self) -> Self {
                    return #value;
                }
            }
        }
    });
    return quote! { #manual_copy #manual_clone };
}

pub(crate) fn ekran_with(
    ty: Ident,
    el: Ident,
    mut cfg: Box<Cfg>,
    storage: StoragePlan,
) -> syn::Result<TokenStream> {
    cfg.settle(storage.raw, false)?;
    return Maker::new_target(
        IntegralTarget::new(ty, Generics::default()),
        el,
        cfg,
        storage,
    )?
    .ekran();
}

pub(crate) struct IntegralTarget {
    pub(crate) ident: Ident,
    generics: Generics,
    phantom_types: Vec<TokenStream>,
    const_markers: Vec<TokenStream>,
    occupied_names: BTreeSet<String>,
}

impl IntegralTarget {
    fn new(
        ident: Ident,
        generics: Generics,
    ) -> Self {
        let mut this = Self {
            ident,
            occupied_names: Self::generic_names(&generics),
            generics,
            phantom_types: Vec::new(),
            const_markers: Vec::new(),
        };
        let parameters = this.generics.params.clone();
        for (index, parameter) in parameters.iter().enumerate() {
            let phantom = this.phantom_for(index, parameter);
            this.phantom_types.push(phantom);
        }
        return this;
    }

    /// The user's generic parameter identifiers, avoided by fresh names.
    fn generic_names(generics: &Generics) -> BTreeSet<String> {
        return generics
            .params
            .iter()
            .map(|parameter| match parameter {
                GenericParam::Lifetime(parameter) => {
                    parameter.lifetime.ident.to_string()
                }
                GenericParam::Type(parameter) => parameter.ident.to_string(),
                GenericParam::Const(parameter) => parameter.ident.to_string(),
            })
            .collect();
    }

    /// The `PhantomData` argument that uses generic parameter `index`.
    fn phantom_for(
        &mut self,
        index: usize,
        parameter: &GenericParam,
    ) -> TokenStream {
        return match parameter {
            GenericParam::Lifetime(parameter) => {
                let lifetime = &parameter.lifetime;
                quote! { &#lifetime () }
            }
            GenericParam::Type(parameter) => {
                let name = &parameter.ident;
                quote! { #name }
            }
            GenericParam::Const(parameter) => {
                self.const_marker(index, parameter)
            }
        };
    }

    /// Declares a marker type carrying const parameter `index` and returns
    /// its use.
    fn const_marker(
        &mut self,
        index: usize,
        parameter: &ConstParam,
    ) -> TokenStream {
        let name = &parameter.ident;
        let type_name = self.ident.unraw();
        let marker =
            self.fresh_ident(&format!("__Typekin{type_name}Const{index}"));
        let ty = &parameter.ty;
        if self.uses_generic_names(ty.to_token_stream()) {
            // A nightly const type may itself depend on another
            // parameter. Keep its entire declaration scope, with
            // zero-sized fields for otherwise-unused parameters.
            let generics = self.external_generics();
            let (parameters, arguments, where_clause) =
                generics.split_for_impl();
            let fields = generics.params.iter().filter_map(|it| {
                return match it {
                    GenericParam::Lifetime(it) => {
                        let lifetime = &it.lifetime;
                        Some(quote! { &#lifetime () })
                    }
                    GenericParam::Type(it) => {
                        let name = &it.ident;
                        Some(quote! { #name })
                    }
                    GenericParam::Const(_) => None,
                };
            });
            self.const_markers.push(quote! {
                #[doc(hidden)]
                struct #marker #parameters(
                    #(::core::marker::PhantomData<#fields>),*
                ) #where_clause;
            });
            return quote! { #marker #arguments };
        }
        let value = self.fresh_ident("VALUE");
        self.const_markers.push(quote! {
            #[doc(hidden)]
            struct #marker<const #value: #ty>;
        });
        return quote! { #marker<#name> };
    }

    fn uses_generic_names(
        &self,
        tokens: TokenStream,
    ) -> bool {
        return tokens.into_iter().any(|token| match token {
            proc_macro2::TokenTree::Ident(ident) => {
                ident == "Self"
                    || self.occupied_names.contains(&ident.to_string())
            }
            proc_macro2::TokenTree::Group(group) => {
                self.uses_generic_names(group.stream())
            }
            _ => false,
        });
    }

    pub(crate) fn has_generics(&self) -> bool {
        return !self.generics.params.is_empty();
    }

    pub(crate) fn ty(&self) -> TokenStream {
        let ident = &self.ident;
        let (_, arguments, _) = self.generics.split_for_impl();
        return quote! { #ident #arguments };
    }

    fn arguments(&self) -> TokenStream {
        let (_, arguments, _) = self.generics.split_for_impl();
        return quote! { #arguments };
    }

    pub(crate) fn impl_header(
        &self,
        extra_parameters: &[GenericParam],
        extra_predicates: &[syn::WherePredicate],
    ) -> (TokenStream, TokenStream) {
        let mut generics = self.generics.clone();
        for parameter in extra_parameters {
            if matches!(parameter, GenericParam::Lifetime(_)) {
                let index = generics
                    .params
                    .iter()
                    .take_while(|it| matches!(it, GenericParam::Lifetime(_)))
                    .count();
                generics.params.insert(index, parameter.clone());
            }
            else {
                generics.params.push(parameter.clone());
            }
        }
        if !extra_predicates.is_empty() {
            generics
                .make_where_clause()
                .predicates
                .extend(extra_predicates.iter().cloned());
        }
        let (parameters, _, where_clause) = generics.split_for_impl();
        return (quote! { #parameters }, quote! { #where_clause });
    }

    pub(crate) fn external_generics(&self) -> Generics {
        let mut generics = self.generics.clone();
        TargetSelf(self.ty()).visit_generics_mut(&mut generics);
        return generics;
    }

    pub(crate) fn qualify_path(
        &self,
        path: &Path,
    ) -> TokenStream {
        if !self.has_generics() {
            let mut path = path.clone();
            if path.leading_colon.is_none()
                && let Some(segment) = path.segments.first_mut()
                && segment.ident == "Self"
            {
                segment.ident = self.ident.clone();
            }
            return quote! { #path };
        }
        let mut path: syn::ExprPath = parse_quote! { #path };
        TargetSelf(self.ty()).visit_expr_path_mut(&mut path);
        return quote! { #path };
    }

    pub(crate) fn external_type(
        &self,
        ty: &Type,
    ) -> TokenStream {
        let mut ty = ty.clone();
        TargetSelf(self.ty()).visit_type_mut(&mut ty);
        return quote! { #ty };
    }

    pub(crate) fn operator_output(
        &self,
        configuration: &SelfConverter,
        external: bool,
        failure: Option<&TokenStream>,
    ) -> OperatorOutput {
        let ty = if configuration.is_self() {
            if external {
                self.ty()
            }
            else {
                quote! { Self }
            }
        }
        else if external {
            self.external_type(&configuration.target)
        }
        else {
            let ty = &configuration.target;
            quote! { #ty }
        };
        let conversion = if configuration.is_self() {
            TokenStream::new()
        }
        else {
            self.qualify_path(&configuration.conversion)
        };
        return OperatorOutput::of(
            configuration,
            ty,
            conversion,
            failure.cloned(),
        );
    }

    pub(crate) fn is_target(
        &self,
        path: &Path,
    ) -> bool {
        return path.is_ident("Self")
            || path.to_token_stream().to_string() == self.ty().to_string();
    }

    pub(crate) fn fresh_ident(
        &self,
        preferred: &str,
    ) -> Ident {
        let mut name = preferred.to_owned();
        if self.occupied_names.contains(&name) {
            name = format!("__Typekin{preferred}");
            while self.occupied_names.contains(&name) {
                name.push('_');
            }
        }
        return Ident::new(&name, Span::mixed_site());
    }

    pub(crate) fn fresh_lifetime(
        &self,
        preferred: &str,
    ) -> syn::Lifetime {
        let mut name = preferred.to_owned();
        if self.occupied_names.contains(&name) {
            name = format!("__typekin_{preferred}");
            while self.occupied_names.contains(&name) {
                name.push('_');
            }
        }
        return syn::Lifetime::new(&format!("'{name}"), Span::mixed_site());
    }

    pub(crate) fn layout_assertion(&self) -> Ident {
        return self.fresh_ident("__TYPEKIN_LAYOUT_ASSERT");
    }

    /// `Self` built from `raw`, which validation accepted, a proof showed
    /// valid, or the caller vouches for (`_blind_trust`). `NonZero` storage
    /// still rejects zero with a panic.
    pub(crate) fn construct(
        &self,
        storage: &StoragePlan,
        raw: TokenStream,
        assert_layout: bool,
    ) -> TokenStream {
        return self
            .construct_storage(storage.storage_expression(raw), assert_layout);
    }

    /// `Self` holding `storage`, an expression of the field's type.
    fn construct_storage(
        &self,
        storage: TokenStream,
        assert_layout: bool,
    ) -> TokenStream {
        if !self.has_generics() {
            return quote! { Self(#storage) };
        }
        let markers = self
            .phantom_types
            .iter()
            .map(|_| quote! { ::core::marker::PhantomData });
        let assertion = assert_layout.then(|| {
            let target = self.ty();
            let name = self.layout_assertion();
            quote! { let () = <#target>::#name; }
        });
        return quote! {{
            #assertion
            Self(#storage, #(#markers),*)
        }};
    }
}

struct TargetSelf(TokenStream);

impl syn::visit_mut::VisitMut for TargetSelf {
    fn visit_expr_path_mut(
        &mut self,
        path: &mut syn::ExprPath,
    ) {
        syn::visit_mut::visit_expr_path_mut(self, path);
        if path.qself.is_none()
            && path.path.leading_colon.is_none()
            && path
                .path
                .segments
                .first()
                .is_some_and(|it| it.ident == "Self")
        {
            if path.path.is_ident("Self") {
                return;
            }
            let target = &self.0;
            let tail = Path {
                leading_colon: None,
                segments: path.path.segments.iter().skip(1).cloned().collect(),
            };
            *path = parse_quote! { <#target>::#tail };
        }
    }

    fn visit_type_path_mut(
        &mut self,
        path: &mut syn::TypePath,
    ) {
        syn::visit_mut::visit_type_path_mut(self, path);
        if path.qself.is_none()
            && path.path.leading_colon.is_none()
            && path
                .path
                .segments
                .first()
                .is_some_and(|it| it.ident == "Self")
        {
            let target = &self.0;
            path.path.segments =
                path.path.segments.iter().skip(1).cloned().collect();
            *path = if path.path.segments.is_empty() {
                parse_quote! { #target }
            }
            else {
                let tail = &path.path;
                parse_quote! { <#target>::#tail }
            };
        }
    }
}

/// What holds the raw value, inside `Wrapping` or not.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Base {
    /// The raw type itself.
    Plain,
    /// `::core::num::NonZero*`: zero cannot be stored.
    NonZero,
}

/// How a wrapper stores its raw value. Every read of the field goes through
/// `extract`, every write through `storage_expression`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StoragePlan {
    pub(crate) raw: Num,
    pub(crate) base: Base,
    pub(crate) wrapping: bool,
}

impl StoragePlan {
    pub(crate) fn primitive(raw: impl Into<Num>) -> Self {
        return Self::new(raw, false, false);
    }

    pub(crate) fn new(
        raw: impl Into<Num>,
        non_zero: bool,
        wrapping: bool,
    ) -> Self {
        return Self {
            raw: raw.into(),
            base: match non_zero {
                true => Base::NonZero,
                false => Base::Plain,
            },
            wrapping,
        };
    }

    fn rewrites_field(&self) -> bool {
        return self.base != Base::Plain || self.wrapping;
    }

    fn non_zero_name(&self) -> &'static str {
        return self
            .raw
            .int()
            .expect("NonZero storage is integer-only")
            .non_zero_rust_name();
    }

    pub(crate) fn field_type(&self) -> TokenStream {
        let raw = format_ident!("{}", self.raw.name());
        let storage = match self.base {
            Base::NonZero => {
                let non_zero = format_ident!("{}", self.non_zero_name());
                quote! { ::core::num::#non_zero }
            }
            Base::Plain => quote! { #raw },
        };

        return if self.wrapping {
            quote! { ::core::num::Wrapping<#storage> }
        }
        else {
            storage
        };
    }

    /// The storage holding `raw`. `NonZero` storage checks for zero with the
    /// safe constructor and panics on it, whether or not validation already
    /// rejected zero: typekin calls no `unsafe` code.
    pub(crate) fn storage_expression(
        &self,
        raw: TokenStream,
    ) -> TokenStream {
        let storage = match self.base {
            Base::NonZero => {
                let non_zero = format_ident!("{}", self.non_zero_name());
                quote! {
                    match ::core::num::#non_zero::new(#raw) {
                        ::core::option::Option::Some(value) => value,
                        ::core::option::Option::None => {
                            ::core::panic!(
                                "zero cannot be stored by this integral type"
                            )
                        }
                    }
                }
            }
            Base::Plain => raw,
        };

        return if self.wrapping {
            quote! { ::core::num::Wrapping(#storage) }
        }
        else {
            storage
        };
    }

    pub(crate) fn extract(
        &self,
        storage: TokenStream,
    ) -> TokenStream {
        let storage = if self.wrapping {
            quote! { (#storage).0 }
        }
        else {
            storage
        };

        return match self.base {
            Base::NonZero => quote! { (#storage).get() },
            Base::Plain => storage,
        };
    }

    /// A borrow of the raw value in `storage`. `NonZero` cannot lend one
    /// without `unsafe`, so `Cfg::settle` turns `AsRef` off for it.
    pub(crate) fn extract_ref(
        &self,
        storage: TokenStream,
    ) -> TokenStream {
        assert!(
            self.base != Base::NonZero,
            "NonZero storage cannot lend its raw value"
        );
        let raw = self.extract(storage);
        return quote! { &#raw };
    }

    pub(crate) fn layout_assertions(
        &self,
        target: TokenStream,
    ) -> TokenStream {
        let raw = format_ident!("{}", self.raw.name());
        return quote! {
            if !(::core::mem::size_of::<#target>()
                == ::core::mem::size_of::<#raw>())
            {
                ::core::panic!(
                    "invalid integral storage size"
                );
            };
            if !(::core::mem::align_of::<#target>()
                == ::core::mem::align_of::<#raw>())
            {
                ::core::panic!(
                    "invalid integral storage alignment"
                );
            };
        };
    }
}

/// The integral friend protocol's trait names and conversion, from the names
/// table.
pub(crate) struct FriendTraits {
    seal: Ident,
    make: Ident,
    math: Ident,
    bit: Ident,
    rel: Ident,
    unsafe_: Ident,
    conversion: Ident,
    arguments: TokenStream,
    /// `<T as Seal…>::conv_…` or `Seal…::conv_…`.
    conversion_path: TokenStream,
}

impl FriendTraits {
    fn new(
        names: &ResolvedNames<IntegralName>,
        arguments: TokenStream,
        rhs_ty: &Ident,
        has_generics: bool,
    ) -> Self {
        let seal = names.get(IntegralName::SealTrait);
        let conversion = names.get(IntegralName::Conversion);
        let conversion_path = match has_generics {
            true => quote! { <#rhs_ty as #seal #arguments>::#conversion },
            false => quote! { #seal #arguments::#conversion },
        };
        return Self {
            make: names.get(IntegralName::MakeTrait),
            math: names.get(IntegralName::MathTrait),
            bit: names.get(IntegralName::BitTrait),
            rel: names.get(IntegralName::RelTrait),
            unsafe_: names.get(IntegralName::UnsafeTrait),
            seal,
            conversion,
            arguments,
            conversion_path,
        };
    }

    pub(crate) fn seal(&self) -> &Ident {
        return &self.seal;
    }

    /// `#seal #arguments`.
    pub(crate) fn seal_with_arguments(&self) -> TokenStream {
        let seal = &self.seal;
        let arguments = &self.arguments;
        return quote! { #seal #arguments };
    }

    /// The seal's conversion method.
    pub(crate) fn conversion(&self) -> &Ident {
        return &self.conversion;
    }

    /// The path that converts a friend: `<T as Seal…>::conv_…` or
    /// `Seal…::conv_…`.
    pub(crate) fn conversion_path(&self) -> &TokenStream {
        return &self.conversion_path;
    }

    /// The protocol trait of `capability`; `Cmp` and `Within` have none.
    pub(crate) fn of(
        &self,
        capability: Capability,
    ) -> Option<&Ident> {
        return match capability {
            Capability::Make => Some(&self.make),
            Capability::Math => Some(&self.math),
            Capability::Bit => Some(&self.bit),
            Capability::Rel => Some(&self.rel),
            Capability::Unsafe => Some(&self.unsafe_),
            Capability::Cmp | Capability::Within => None,
        };
    }

    /// `#trait #arguments` for `capability`'s protocol trait.
    pub(crate) fn bound(
        &self,
        capability: Capability,
    ) -> TokenStream {
        let name = self
            .of(capability)
            .expect("integral capabilities have protocol traits");
        let arguments = &self.arguments;
        return quote! { #name #arguments };
    }
}

pub(crate) struct Ctx {
    pub(crate) cfg: Box<Cfg>,
    pub(crate) names: ResolvedNames<IntegralName>,
    pub(crate) friends: FriendTraits,
    pub(crate) ty: TokenStream,
    pub(crate) impl_generics: TokenStream,
    pub(crate) where_clause: TokenStream,
    pub(crate) arguments: TokenStream,
    pub(crate) rhs_ty: Ident,
    pub(crate) validation: TokenStream,
    pub(crate) rhs_raw: TokenStream,
    pub(crate) self_raw: TokenStream,
    pub(crate) get_raw: Path,
    pub(crate) fp_or_panic: Path,
    /// `Option<Raw>`, the error a failing operator passes on in its
    /// `Result` where it may not panic (`panic_at` below `anything`).
    pub(crate) failure: Option<TokenStream>,
}

impl Ctx {
    fn new(
        target: &IntegralTarget,
        cfg: Box<Cfg>,
        storage: &StoragePlan,
        el: &Ident,
    ) -> Self {
        let names = Names::<IntegralName>::default()
            .resolve(target.ident.clone(), target.occupied_names.clone());
        let ty = target.ty();
        let (impl_generics, where_clause) = target.impl_header(&[], &[]);
        let arguments = target.arguments();
        let or_panic = names.get(IntegralName::OrPanic);
        let fp_or_panic = parse_quote! { Self::#or_panic };
        let get_raw = cfg
            .get_raw
            .clone()
            .unwrap_or_else(|| parse_quote! { Self::raw });
        let rhs_ty = target.fresh_ident("T");
        let friends = FriendTraits::new(
            &names,
            arguments.clone(),
            &rhs_ty,
            target.has_generics(),
        );
        let validation =
            cfg.validation.emit(&quote! { value }).to_token_stream();
        let rhs_raw = storage.extract(quote! { rhs.0 });
        let self_raw = storage.extract(quote! { self.0 });
        let failure = match translate(true, cfg.wide.panic_at, false) {
            Some(Translation::Pass) => {
                Some(quote! { ::core::option::Option<#el> })
            }
            Some(Translation::Unwrap(_)) | None => None,
        };

        return Self {
            cfg,
            names,
            friends,
            ty,
            impl_generics,
            where_clause,
            arguments,
            rhs_ty,
            validation,
            rhs_raw,
            self_raw,
            get_raw,
            fp_or_panic,
            failure,
        };
    }
}

pub(crate) struct Maker {
    pub(crate) num: Num,
    pub(crate) storage: StoragePlan,
    pub(crate) el: Ident,
    pub(crate) target: IntegralTarget,
    pub(crate) ctx: Ctx,
    pub(crate) conditional_make: BTreeMap<Num, TokenStream>,
    empty_friend_checks: TokenStream,
    /// The `Within` friends and how each is proven.
    pub(crate) within: Vec<(Friend, WithinProof)>,
    /// The closure proofs already run.
    pub(crate) proofs: ProofMemo,
}

impl Maker {
    fn new_target(
        target: IntegralTarget,
        el: Ident,
        cfg: Box<Cfg>,
        storage: StoragePlan,
    ) -> syn::Result<Self> {
        if storage.base == Base::NonZero
            && !cfg.flags.has(IntegralFlag::FnOrPanic)
        {
            return cfg
                .explicit_span(IntegralFlag::FnOrPanic)
                .fail("nonzero storage does not support a custom or_panic constructor");
        }

        let ctx = Ctx::new(&target, cfg, &storage, &el);
        let mut this = Self {
            num: storage.raw,
            storage,
            el,
            target,
            ctx,
            conditional_make: BTreeMap::new(),
            empty_friend_checks: TokenStream::new(),
            within: Vec::new(),
            proofs: ProofMemo::default(),
        };

        // Built-in friendships may add capabilities to an explicit empty list.
        this.empty_friend_checks = this.make_empty_friend_checks();
        this.fix_friendship()?;
        this.within = this.plan_within()?;

        return Ok(this);
    }

    /// The translation of a form whose output is fixed and whose value may
    /// be invalid (a friend's, a closure's): `None` where that may not
    /// panic, so the form is not generated.
    pub(crate) fn fixed_form(&self) -> Option<Translation> {
        return translate(true, self.ctx.cfg.wide.panic_at, true);
    }

    /// The error in a form's output: `Option<Raw>` when its body `fails`
    /// and `translation` passes the `Result` on.
    pub(crate) fn passed_failure(
        &self,
        fails: bool,
        translation: Translation,
    ) -> Option<&TokenStream> {
        return match (translation, fails) {
            (Translation::Pass, true) => self.ctx.failure.as_ref(),
            (Translation::Pass, false) | (Translation::Unwrap(_), _) => None,
        };
    }

    /// What `slot`'s operator outputs: its produced type, in a `Result`
    /// when the body's failure is passed on. `external`: named outside an
    /// impl of the target, where `Self` is another type.
    pub(crate) fn output_of(
        &self,
        slot: &Slot,
        external: bool,
        fails: bool,
        translation: Translation,
    ) -> OperatorOutput {
        return self.target.operator_output(
            &slot.produce,
            external,
            self.passed_failure(fails, translation),
        );
    }

    /// The constructor a computed raw result goes through. A failure that
    /// panics does so in `_or_panic`; one that is passed on comes from
    /// `_or_err`. A result that cannot fail skips validation through
    /// `_proven` where the type has it; `None` then says to build `Self`
    /// directly (the shape of a float's domain proves it). An unvalidated
    /// type has only `_or_panic`, which just constructs.
    pub(crate) fn constructor(
        &self,
        fails: bool,
        translation: Translation,
    ) -> Option<Path> {
        return match (translation, fails) {
            (Translation::Unwrap(_), _) => Some(self.ctx.fp_or_panic.clone()),
            (Translation::Pass, true) => {
                let or_err = self.ctx.names.get(IntegralName::OrErr);
                Some(parse_quote! { Self::#or_err })
            }
            (Translation::Pass, false) => {
                if !self.ctx.cfg.validation.has_validation() {
                    Some(self.ctx.fp_or_panic.clone())
                }
                else if self.proofs_available() {
                    Some(parse_quote! { Self::_proven })
                }
                else {
                    None
                }
            }
        };
    }

    /// `value`, a computed raw result, as `Self` (or as `_or_err`'s
    /// `Result`): through `constructor`, or built directly.
    pub(crate) fn constructed(
        &self,
        fails: bool,
        translation: Translation,
        value: TokenStream,
    ) -> TokenStream {
        return match self.constructor(fails, translation) {
            Some(constructor) => quote! { #constructor(#value) },
            None => self.construct_self(value),
        };
    }

    /// `#konst impl<..> #path for Ty where .. { #body }`, `const` under
    /// `is_konst`.
    pub(crate) fn impl_trait(
        &self,
        is_konst: bool,
        path: TokenStream,
        body: TokenStream,
    ) -> TokenStream {
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let konst = Konst::konst(is_konst);
        return quote! {
            #konst impl #impl_generics #path for #ty #where_clause {
                #body
            }
        };
    }

    /// `impl_trait` over the target's generics extended with `parameters`
    /// (lifetimes join the lifetimes, the rest go last) and `predicates`.
    pub(crate) fn impl_trait_with(
        &self,
        is_konst: bool,
        parameters: &[GenericParam],
        predicates: &[syn::WherePredicate],
        path: TokenStream,
        body: TokenStream,
    ) -> TokenStream {
        let ty = &self.ctx.ty;
        let (impl_generics, where_clause) =
            self.target.impl_header(parameters, predicates);
        let konst = Konst::konst(is_konst);
        return quote! {
            #konst impl #impl_generics #path for #ty #where_clause {
                #body
            }
        };
    }

    /// `Self` built from a validated `raw` value.
    pub(crate) fn construct_self(
        &self,
        raw: TokenStream,
    ) -> TokenStream {
        return self.target.construct(
            &self.storage,
            raw,
            self.ctx.cfg.flags.has(IntegralFlag::Assertions),
        );
    }

    /// The statements ending a constructor's body: the raw value named
    /// `raw` checked as `check` says, then `Self` built from it, else
    /// `on_invalid`.
    pub(crate) fn construction(
        &self,
        raw: TokenStream,
        check: Check,
        on_invalid: OnInvalid,
    ) -> TokenStream {
        return self.ctx.cfg.validation.emit(&raw).construct(
            check,
            on_invalid,
            self.construct_self(raw),
        );
    }

    /// The integer value type, for integer-only emitters (their flags are
    /// off for floats).
    pub(crate) fn repr(&self) -> N {
        return self
            .num
            .int()
            .expect("integer-only emitter on a float type");
    }

    /// Primitive conversion targets: lossless ones with `safe`, lossy ones
    /// without.
    pub(crate) fn cast_targets(
        &self,
        safe: bool,
    ) -> impl Iterator<Item = (Num, TokenStream)> + '_ {
        return Num::items().filter_map(move |target| {
            let cfg = pointer_width_cfg(|width| {
                self.num.can_safe_cast_to(target, width) == safe
            })?;
            Some((target, cfg))
        });
    }

    /// The validating helper `slot`'s impls of a binary operator call:
    /// `_add`, `_bitor`, `_shr`. A non-const impl calls the `_wrapping`
    /// twin where there is one, and the wrapper's own right-hand side the
    /// `_proven` sibling when only same-type operands are proven.
    pub(crate) fn helper_name(
        &self,
        operator: &OpSpec,
        slot: &Slot,
        is_konst: bool,
    ) -> Ident {
        let wrapping = !is_konst && self.has_wrapping_helper(operator, slot);
        let proof = self.operator_proof(operator, slot.arithmetic);
        let same_only =
            matches!(slot.rhs, RhsKind::Same) && proof.same && !proof.full;
        return format_ident!(
            "_{}{}{}",
            operator.method,
            if wrapping { "_wrapping" } else { "" },
            if same_only { "_proven" } else { "" },
        );
    }

    /// Whether `operator`'s helper has a non-const `_wrapping` twin for
    /// `slot`: the arithmetic and shifts of `Wrapping`, whose own operators
    /// are not const.
    pub(crate) fn has_wrapping_helper(
        &self,
        operator: &OpSpec,
        slot: &Slot,
    ) -> bool {
        return operator.kind != OpKind::Bitwise
            && slot.arithmetic != slot.arithmetic.konst();
    }

    /// The checks and constants that are neither a trait nor an impl of
    /// one: the friend conversion checks, the layout assertions, and the
    /// `Within` proof machinery.
    fn ekran_declarations(&self) -> TokenStream {
        return [
            self.empty_friend_checks.clone(),
            self.make_assertions(),
            self.ekran_within(),
        ]
        .into_iter()
        .collect();
    }

    /// The inherent items: the `core` constants and forwards in their impl
    /// blocks, the block of everything else, and the bit-access block.
    fn ekran_items(&self) -> syn::Result<TokenStream> {
        let core_int = self.make_core_int();
        let core_float = self.make_core_float();
        let forwards = self.make_forwards()?;
        let items = [
            self.make_fn_conv_of(),
            self.make_auto_of_raw(),
            self.make_fn_conv_raw(),
            self.make_fn_conv_into(),
            self.make_map(),
            self.make_fn_conv_try_into_lossless(),
            self.make_fn_conv_try_into_lossy(),
            self.make_fn_math_helpers(),
            self.make_precedes(),
            self.make_fn_op_eq(),
            self.make_fn_op_cmp(),
            self.make_fn_make_try(),
            self.make_or_panic(),
            self.make_or_err(),
            self.make_proven(),
            self.make_blind_trust(),
            self.make_within(),
            self.make_within_proofs(),
            self.make_within_source(),
            self.forward_items(),
        ];
        let bit_access = self.ekran_bit_access_items();

        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        return Ok(quote! {
            #core_int
            #core_float
            #forwards

            impl #impl_generics #ty #where_clause {
                #(#items)*
            }

            #[allow(clippy::unnecessary_cast)]
            impl #impl_generics #ty #where_clause {
                #bit_access
            }
        });
    }

    /// The trait impls. The operators follow `OPS`, each row's slots and
    /// each slot's forms.
    fn ekran_impls(&self) -> TokenStream {
        return [
            self.make_as_ref(),
            self.make_from_str(),
            self.make_serde(),
            self.make_cast_impls(true),
            self.make_cast_impls(false),
            self.make_range(),
            self.make_debug(),
            self.make_hash(),
            self.make_eq(),
            self.make_ord(),
            self.make_operators(),
            self.make_fmt(),
            self.make_partial_eq(),
            self.make_partial_ord(),
            self.make_fmt_exp(),
            self.make_iterator_folds(),
        ]
        .into_iter()
        .collect();
    }

    /// Everything generated after the annotated item, by section:
    /// declarations, protocol traits, inherent items, trait impls.
    fn ekran(&self) -> syn::Result<TokenStream> {
        self.check_prove()?;
        let declarations = self.ekran_declarations();
        let protocol = self.ekran_friendship();
        let items = self.ekran_items()?;
        let impls = self.ekran_impls();

        return Ok(quote! {
            #declarations

            #protocol

            #items

            #impls
        });
    }
}
