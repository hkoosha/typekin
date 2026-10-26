use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::fmt::{
    Debug,
    Formatter,
};
use std::str::FromStr;

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
use syn::spanned::Spanned as _;
use syn::visit_mut::VisitMut as _;
use syn::{
    Fields,
    GenericParam,
    Generics,
    ItemStruct,
    Path,
    Type,
    Visibility,
    parse_quote,
};

use crate::friendship::cfg::Friend;
use crate::friendship::{
    Construction,
    Protocol,
    ProtocolFriend,
};
use crate::zz::{
    Konst,
    Konsts,
    ValidationCfg,
};
use crate::zz_conv::{
    OperatorOutput,
    SelfConverter,
};
use crate::zz_profile::{
    Arithmetic,
    DefaultFacetCfg,
    FacetCfg,
    FacetOwner,
    FacetSlot,
    FacetSlotKey,
    Profiles,
};
use crate::zz_runner::{
    Merged,
    MkErr,
    ToCollection,
    mk_flags,
};
use crate::zz_value_type::{
    ArithmeticOp,
    N,
    Operator,
};
use crate::{
    zz,
    zz_runner,
};

#[derive(Clone)]
pub(crate) struct IntegralTarget {
    ident: Ident,
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
        let occupied_names = generics
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
        let mut this = Self {
            ident,
            generics,
            phantom_types: Vec::new(),
            const_markers: Vec::new(),
            occupied_names,
        };

        for (index, parameter) in this.generics.params.iter().enumerate() {
            let phantom = match parameter {
                GenericParam::Lifetime(parameter) => {
                    let lifetime = &parameter.lifetime;
                    quote! { &#lifetime () }
                }
                GenericParam::Type(parameter) => {
                    let name = &parameter.ident;
                    quote! { #name }
                }
                GenericParam::Const(parameter) => {
                    let name = &parameter.ident;
                    let type_name = this.ident.to_string();
                    let type_name = type_name.trim_start_matches("r#");
                    let marker = this.fresh_ident(&format!(
                        "__Typekin{type_name}Const{index}",
                    ));
                    let ty = &parameter.ty;
                    if this.uses_generic_names(ty.to_token_stream()) {
                        // A nightly const type may itself depend on another
                        // parameter. Keep its entire declaration scope, with
                        // zero-sized fields for otherwise-unused parameters.
                        let generics = this.external_generics();
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
                        this.const_markers.push(quote! {
                            #[doc(hidden)]
                            struct #marker #parameters(
                                #(::core::marker::PhantomData<#fields>),*
                            ) #where_clause;
                        });
                        quote! { #marker #arguments }
                    }
                    else {
                        let value = this.fresh_ident("VALUE");
                        this.const_markers.push(quote! {
                            #[doc(hidden)]
                            struct #marker<const #value: #ty>;
                        });
                        quote! { #marker<#name> }
                    }
                }
            };
            this.phantom_types.push(phantom);
        }

        return this;
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

    fn has_generics(&self) -> bool {
        return !self.generics.params.is_empty();
    }

    fn ty(&self) -> TokenStream {
        let ident = &self.ident;
        let (_, arguments, _) = self.generics.split_for_impl();
        return quote! { #ident #arguments };
    }

    fn constructor(&self) -> TokenStream {
        let ident = &self.ident;
        let (_, arguments, _) = self.generics.split_for_impl();
        let arguments = arguments.as_turbofish();
        return quote! { #ident #arguments };
    }

    fn arguments(&self) -> TokenStream {
        let (_, arguments, _) = self.generics.split_for_impl();
        return quote! { #arguments };
    }

    fn impl_header(
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

    fn external_generics(&self) -> Generics {
        let mut generics = self.generics.clone();
        TargetSelf(self.ty()).visit_generics_mut(&mut generics);
        return generics;
    }

    fn qualify_path(
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

    fn external_type(
        &self,
        ty: &Type,
    ) -> TokenStream {
        let mut ty = ty.clone();
        TargetSelf(self.ty()).visit_type_mut(&mut ty);
        return quote! { #ty };
    }

    fn operator_output<'a>(
        &self,
        configuration: &'a SelfConverter,
        external: bool,
    ) -> OperatorOutput<'a> {
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
        return OperatorOutput::of(configuration, ty, conversion);
    }

    fn is_target(
        &self,
        path: &Path,
    ) -> bool {
        return path.is_ident("Self")
            || path.to_token_stream().to_string() == self.ty().to_string();
    }

    fn fresh_ident(
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

    fn fresh_lifetime(
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

    fn layout_assertion(&self) -> Ident {
        return self.fresh_ident("__TYPEKIN_LAYOUT_ASSERT");
    }

    fn construct(
        &self,
        storage: &StoragePlan,
        constructor: TokenStream,
        raw: TokenStream,
        assert_layout: bool,
    ) -> TokenStream {
        return self.construct_storage(
            constructor,
            storage.storage_expression(raw),
            assert_layout,
        );
    }

    fn construct_storage(
        &self,
        constructor: TokenStream,
        storage: TokenStream,
        assert_layout: bool,
    ) -> TokenStream {
        if !self.has_generics() {
            return quote! { #constructor(#storage) };
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
            #constructor(#storage, #(#markers),*)
        }};
    }
}

struct TargetSelf(TokenStream);

impl syn::visit_mut::VisitMut for TargetSelf {
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StoragePlan {
    pub(crate) raw: N,
    pub(crate) non_zero: bool,
    pub(crate) wrapping: bool,
}

impl StoragePlan {
    pub(crate) fn primitive(raw: N) -> Self {
        return Self {
            raw,
            non_zero: false,
            wrapping: false,
        };
    }

    pub(crate) fn new(
        raw: N,
        non_zero: bool,
        wrapping: bool,
    ) -> Self {
        return Self {
            raw,
            non_zero,
            wrapping,
        };
    }

    pub(crate) fn rewrites_field(&self) -> bool {
        return self.non_zero || self.wrapping;
    }

    pub(crate) fn field_type(&self) -> TokenStream {
        let raw = format_ident!("{}", self.raw.rust_name());
        let storage = if self.non_zero {
            let non_zero = format_ident!("{}", self.raw.non_zero_rust_name());
            quote! { ::core::num::#non_zero }
        }
        else {
            quote! { #raw }
        };

        return if self.wrapping {
            quote! { ::core::num::Wrapping<#storage> }
        }
        else {
            storage
        };
    }

    fn storage_expression(
        &self,
        raw: TokenStream,
    ) -> TokenStream {
        let storage = if self.non_zero {
            let non_zero = format_ident!("{}", self.raw.non_zero_rust_name());
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
        else {
            raw
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

        return if self.non_zero {
            quote! { (#storage).get() }
        }
        else {
            storage
        };
    }

    pub(crate) fn extract_ref(
        &self,
        storage: TokenStream,
    ) -> TokenStream {
        if !self.rewrites_field() {
            return quote! { &#storage };
        }

        let raw = format_ident!("{}", self.raw.rust_name());
        return quote! {
            unsafe {
                // SAFETY: NonZero integers and Wrapping are transparent
                // over their inner values. Storage construction also ensures
                // that every NonZero value contains a valid primitive value.
                &*(
                    ::core::ptr::addr_of!(#storage)
                        .cast::<#raw>()
                )
            }
        };
    }

    pub(crate) fn layout_assertions(
        &self,
        target: TokenStream,
    ) -> TokenStream {
        let raw = format_ident!("{}", self.raw.rust_name());
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

pub(crate) fn integral(
    attr: Box<Cfg>,
    item: ItemStruct,
) -> proc_macro::TokenStream {
    return zz_runner::catching(move || ekran(attr, item));
}

pub(crate) fn ekran(
    attr: Box<Cfg>,
    mut item: ItemStruct,
) -> syn::Result<TokenStream> {
    let target = IntegralTarget::new(item.ident.clone(), item.generics.clone());
    let repr: Option<TokenStream> =
        attr.flags.repr.then(|| quote! { #[repr(transparent)] });
    let derive_copy: Option<TokenStream> = (attr.flags.copy
        && !target.has_generics())
    .then(|| quote! { #[derive(Copy)] });
    let derive_clone: Option<TokenStream> = (attr.flags.clone
        && !target.has_generics())
    .then(|| match attr.konst {
        Konst::Nightly => quote! { #[derive_const(Clone)] },
        Konst::None | Konst::Stable => quote! { #[derive(Clone)] },
    });

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
    let field = fields.unnamed.first().unwrap();
    let el = match &field.ty {
        Type::Path(path) if path.qself.is_none() => path
            .path
            .get_ident()
            .filter(|id| N::rust_names().contains(&id.to_string().as_str())),
        _ => None,
    }
    .ok_or_else(|| {
        syn::Error::new(
            field.ty.span(),
            "integral value field must be a concrete primitive integer",
        )
    })?
    .clone();
    let field_visibility = &field.vis;

    let raw = N::of(el.to_string())
        .ok_or_else(|| syn::Error::new(el.span(), "unknown integral type"))?;
    let storage = StoragePlan::new(
        raw,
        attr.requires_non_zero_storage(),
        attr.requires_wrapping_storage(),
    );
    let private = match &field_visibility {
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

    let (impl_generics, where_clause) = target.impl_header(&[], &[]);
    let target_ty = target.ty();
    let manual_copy = (attr.flags.copy && target.has_generics()).then(|| {
        quote! {
            impl #impl_generics ::core::marker::Copy for #target_ty
            #where_clause
            {}
        }
    });
    let manual_clone = (attr.flags.clone && target.has_generics()).then(|| {
        let konst = zz_runner::konst(matches!(attr.konst, Konst::Nightly));
        let value = target.construct_storage(
            quote! { Self },
            quote! { self.0 },
            attr.flags.assertions,
        );
        quote! {
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
    let markers = &target.const_markers;
    let const_markers = quote! { #(#markers)* };

    let it = Maker::new_target(target, el, attr, storage)?.ekran()?;

    let stream = quote::quote! {
        #const_markers

        #repr
        #derive_copy
        #derive_clone
        #item

        #manual_copy
        #manual_clone

        #[allow(dead_code)]
        #[allow(unused_qualifications)]
        #[allow(clippy::unnecessary_cast)]
        const _: () = {
            #it
        };
    };

    return Ok(stream);
}

pub(crate) struct Maker {
    repr: N,
    storage: StoragePlan,
    el: Ident,
    target: IntegralTarget,

    fn_conv: Ident,
    fp_get_raw: Path,
    fp_unchecked: Path,

    trait_seal: Ident,
    trait_friend_make: Ident,
    trait_friend_math: Ident,
    trait_friend_bit: Ident,
    trait_friend_rel: Ident,

    cfg: Box<Cfg>,
}

impl Maker {
    pub(crate) fn new(
        ty: Ident,
        el: Ident,
        cfg: Box<Cfg>,
        storage: StoragePlan,
    ) -> syn::Result<Self> {
        return Self::new_target(
            IntegralTarget::new(ty, Generics::default()),
            el,
            cfg,
            storage,
        );
    }

    fn new_target(
        target: IntegralTarget,
        el: Ident,
        cfg: Box<Cfg>,
        storage: StoragePlan,
    ) -> syn::Result<Self> {
        let ty = &target.ident;
        let cfg = Self::preprocess_cfg(cfg);
        let (unchecked_flag, emits_unchecked) =
            if cfg.validation.has_validation() {
                ("fn_make_unchecked", cfg.flags.fn_make_unchecked)
            }
            else {
                ("fn_make_checked", cfg.flags.fn_make_checked)
            };
        if storage.non_zero && !emits_unchecked {
            let span = cfg
                .explicit_flag(unchecked_flag)
                .map_or_else(Span::call_site, |(_, span)| span);
            return span.fail(
                "nonzero storage does not support custom unchecked constructors",
            );
        }

        let mut this = Self {
            trait_seal: target.fresh_ident("Seal"),
            trait_friend_bit: target.fresh_ident("Bit"),
            trait_friend_make: target.fresh_ident("Make"),
            trait_friend_math: target.fresh_ident("Math"),
            trait_friend_rel: target.fresh_ident("Rel"),

            fp_unchecked: syn::parse_str(&cfg.flags.fp_unchecked)?,
            fp_get_raw: cfg
                .get_raw
                .clone()
                .unwrap_or_else(|| parse_quote! { Self::raw }),
            fn_conv: format_ident!(
                "conv_{}",
                zz_runner::snake_case_of(&ty.to_string())
            ),

            repr: N::of(el.to_string()).ok_or_else(|| {
                syn::Error::new(el.span(), "unknown integral type")
            })?,
            storage,
            el,
            target,
            cfg,
        };

        this.fix_friendship()?;

        return Ok(this);
    }

    fn preprocess_cfg(mut cfg: Box<Cfg>) -> Box<Cfg> {
        if cfg.flags.auto_of_raw && cfg.validation.has_validation() {
            cfg.flags.auto_of_raw = false;
        }

        return cfg;
    }

    fn target_qualified_path(
        &self,
        path: &Path,
    ) -> TokenStream {
        return self.target.qualify_path(path);
    }

    fn arithmetic_method_name(
        &self,
        operation: ArithmeticOp,
        is_konst: bool,
    ) -> Ident {
        let name = operation.name();
        return if !is_konst
            && self.cfg.profile.arithmetic() == Arithmetic::WrapperWrapping
        {
            format_ident!("_{}_wrapping", name)
        }
        else {
            format_ident!("_{}", name)
        };
    }

    fn ekran_arithmetic_helper(
        &self,
        operation: ArithmeticOp,
        rhs_type: TokenStream,
    ) -> TokenStream {
        let fp_get_raw = &self.fp_get_raw;
        let fp_unchecked = &self.fp_unchecked;
        let arithmetic = self.cfg.profile.arithmetic();
        let method = self.arithmetic_method_name(operation, true);
        let expression = arithmetic.make_const_expr(operation);
        let wrapping = (arithmetic == Arithmetic::WrapperWrapping).then(|| {
            let method = self.arithmetic_method_name(operation, false);
            let expression = arithmetic.make_expr(operation);
            quote! {
                #[inline(always)]
                pub(self) fn #method(
                    &self,
                    rhs: #rhs_type,
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = #expression;
                    return #fp_unchecked(it);
                }
            }
        });

        return quote! {
            #[inline(always)]
            pub(self) const fn #method(
                &self,
                rhs: #rhs_type,
            ) -> Self {
                let lhs = #fp_get_raw(*self);
                let it = #expression;
                return #fp_unchecked(it);
            }

            #wrapping
        };
    }

    fn is_operator_capability(capability: &Ident) -> bool {
        return matches!(capability.to_string().as_str(), "Math" | "Bit");
    }

    fn friend_conversion_expression(
        &self,
        ty: &Path,
        conversion: Option<&Path>,
        value: TokenStream,
    ) -> TokenStream {
        if self.target.is_target(ty) {
            let fp_get_raw = self.target_qualified_path(&self.fp_get_raw);
            return quote! { #fp_get_raw(#value) };
        }

        let relation = &self.el;
        if ty
            .get_ident()
            .is_some_and(|ident| N::of(ident.to_string()).is_some())
        {
            return quote! { #value as #relation };
        }

        if conversion.is_some_and(|conversion| conversion.is_ident("self")) {
            return quote! {
                {
                    let relation: #relation = #value;
                    relation
                }
            };
        }

        return match conversion {
            Some(conversion) => {
                let conversion = self.target_qualified_path(conversion);
                quote! { #conversion(#value) }
            }
            None => quote! {
                {
                    let relation: #relation = #value.into();
                    relation
                }
            },
        };
    }

    fn ekran_facet_dispatch(&self) -> TokenStream {
        if !self.cfg.facets_present() {
            return TokenStream::new();
        }

        let target = self.target.ty();
        let generics = self.target.external_generics();
        let (parameters, arguments, where_clause) = generics.split_for_impl();
        let operators = self
            .cfg
            .facet_slots
            .keys()
            .map(|slot| slot.operator)
            .collect::<BTreeSet<_>>();
        let traits = operators.iter().copied().map(|operator| {
            let trait_name =
                self.target.fresh_ident(&format!("{}Rhs", operator.name()));
            let method = format_ident!("{}", operator.method_name());
            let is_konst = self.cfg.konst_for(&operator.is_konst_flags());
            let konst = zz_runner::konst(is_konst);

            return quote! {
                #[doc(hidden)]
                #[allow(private_interfaces)]
                pub #konst trait #trait_name #parameters #where_clause {
                    type Output;

                    fn #method(
                        lhs: #target,
                        rhs: Self,
                    ) -> Self::Output;
                }
            };
        });
        let implementations = self.cfg.facet_slots.iter().map(|(key, slot)| {
            let operator = key.operator;
            let trait_name =
                self.target.fresh_ident(&format!("{}Rhs", operator.name()));
            let method = format_ident!("{}", operator.method_name());
            let rhs_ty = slot.rhs.as_ref().map_or_else(
                || target.clone(),
                |rhs| self.target.external_type(&parse_quote! { #rhs }),
            );
            let rhs = match &slot.rhs {
                Some(rhs) => self.friend_conversion_expression(
                    rhs,
                    slot.conversion.as_ref(),
                    quote! { rhs },
                ),
                None => {
                    let fp_get_raw =
                        self.target_qualified_path(&self.fp_get_raw);
                    quote! { #fp_get_raw(rhs) }
                }
            };
            let lhs = self.target_qualified_path(&self.fp_get_raw);
            let unchecked = self.target_qualified_path(&self.fp_unchecked);
            let output = self.target.operator_output(&slot.ty_out, true);
            let output_ty = output.ty();
            let result = output.convert(quote! { value });
            let is_konst = self.cfg.konst_for(&operator.is_konst_flags());
            let operation =
                slot.arithmetic.make_operator_expr(operator, is_konst);
            let konst = zz_runner::konst(is_konst);

            return quote! {
                #konst impl #parameters #trait_name #arguments for #rhs_ty
                #where_clause {
                    type Output = #output_ty;

                    #[inline(always)]
                    fn #method(
                        lhs: #target,
                        rhs: #rhs_ty,
                    ) -> Self::Output {
                        let lhs = #lhs(lhs);
                        let rhs = #rhs;
                        let value = #operation;
                        let value = #unchecked(value);
                        return #result;
                    }
                }
            };
        });
        let operator_impls = operators.iter().copied().map(|operator| {
            let trait_name = self
                .target
                .fresh_ident(&format!("{}Rhs", operator.name()));
            let method = format_ident!("{}", operator.method_name());
            let operation = format_ident!("{}", operator.name());
            let rhs = self.target.fresh_ident("T");
            let is_konst = self.cfg.konst_for(&operator.is_konst_flags());
            let konst = zz_runner::konst(is_konst);
            let bonst = zz_runner::bonst(is_konst);
            let destruct = zz_runner::destruct(is_konst);
            let (impl_generics, where_clause) = self.target.impl_header(
                &[parse_quote! { #rhs }],
                &[parse_quote! {
                    #rhs: #bonst #trait_name #arguments #destruct
                }],
            );

            return quote! {
                #[allow(private_bounds)]
                #konst impl #impl_generics ::core::ops::#operation<#rhs>
                    for #target
                #where_clause {
                    type Output = <#rhs as #trait_name #arguments>::Output;

                    #[inline(always)]
                    fn #method(
                        self,
                        rhs: #rhs,
                    ) -> Self::Output {
                        return <#rhs as #trait_name #arguments>::#method(self, rhs);
                    }
                }
            };
        });

        return quote! {
            #(#traits)*
            #(#implementations)*
            #(#operator_impls)*
        };
    }

    fn fix_friendship(&mut self) -> syn::Result<()> {
        let target: Path = syn::parse2(self.target.ty())
            .expect("integral targets emit valid type paths");
        let relation = &self.el;
        let self1 = &self.cfg;
        let validated = self1.validation.has_validation();
        let has_make = self.cfg.flags.self_friend_make;
        let target_friend = self
            .cfg
            .friends
            .iter()
            .find(|friend| self.target.is_target(&friend.ty))
            .cloned()
            .unwrap_or_else(|| Friend::from(&target));

        if let Some(it) = self.cfg.friends.get(&target_friend) {
            if it.capabilities.contains(&format_ident!("Math")) {
                return it.ty.fail(
                    "self math operations use `self_math`, not `friend`",
                );
            }

            if it.capabilities.contains(&format_ident!("Bit")) {
                return it
                    .ty
                    .fail("self bit operations use `self_bit`, not `friend`");
            }
        }

        for (f_def, conv, is_validated) in [
            (target_friend, Some(parse_quote! { Self::raw }), false),
            (
                Friend::from(relation),
                Some(parse_quote! { self }),
                validated,
            ),
        ] {
            let mut it = self
                .cfg
                .friends
                .take(&f_def)
                .unwrap_or_else(|| f_def.clone());

            if it.conv.is_none() {
                it.conv = conv;
            }

            if !is_validated && has_make {
                it.capabilities.insert(format_ident!("Make"));
            }

            if !self.target.is_target(&f_def.ty)
                && self.cfg.flags.self_friend_math_ops
            {
                it.capabilities.insert(format_ident!("Math"));
            }

            if self.target.is_target(&f_def.ty)
                || self.cfg.flags.self_friend_rel
            {
                it.capabilities.insert(format_ident!("Rel"));
            }

            if !self.target.is_target(&f_def.ty)
                && self.cfg.flags.self_friend_math_bit
            {
                it.capabilities.insert(format_ident!("Bit"));
            }

            // Generic Self operations share the RHS protocol: separate Self
            // impls would overlap when downstream parameters satisfy it.
            if self.target.has_generics() && self.target.is_target(&f_def.ty) {
                if self.cfg.flags.self_math {
                    it.capabilities.insert(format_ident!("Math"));
                }
                if self.cfg.flags.self_bit {
                    it.capabilities.insert(format_ident!("Bit"));
                }
            }

            assert!(self.cfg.friends.insert(it));
        }

        if !validated && has_make {
            N::items()
                .iter()
                .filter(|it| {
                    **it != self.repr && it.can_safe_cast_to(self.repr)
                })
                .map(|it| format_ident!("{}", it.rust_name()))
                .map(|ty| Friend::from(&ty))
                .for_each(|friend| {
                    let mut friend =
                        self.cfg.friends.take(&friend).unwrap_or(friend);
                    friend.capabilities.insert(format_ident!("Make"));
                    assert!(self.cfg.friends.insert(friend));
                });
        }

        return Ok(());
    }

    fn protocol_capability(
        &self,
        capability: &Ident,
    ) -> Ident {
        return match capability.to_string().as_str() {
            "Make" => self.trait_friend_make.clone(),
            "Math" => self.trait_friend_math.clone(),
            "Bit" => self.trait_friend_bit.clone(),
            "Rel" => self.trait_friend_rel.clone(),
            _ => self.target.fresh_ident(&capability.to_string()),
        };
    }

    fn ekran_friendship(&self) -> TokenStream {
        let faceted = self.cfg.facets_present();
        let target = self.target.ty();
        let relation_ident = &self.el;
        let relation: Type = parse_quote! { #relation_ident };
        let seal = self.trait_seal.clone();
        let conversion = self.fn_conv.clone();
        let fp_get_raw = &self.fp_get_raw;
        let target_conversion = self.cfg.flags.friend_seal.then(|| {
            quote! {
                return #fp_get_raw(self);
            }
        });
        let capabilities = [
            (self.cfg.flags.friendzone_friend_make, format_ident!("Make")),
            (
                self.cfg.flags.friendzone_friend_math_ops,
                format_ident!("Math"),
            ),
            (
                self.cfg.flags.friendzone_friend_math_bit,
                format_ident!("Bit"),
            ),
            (self.cfg.flags.friendzone_friend_rel, format_ident!("Rel")),
            (true, crate::friendship::cfg::trust_capability()),
        ]
        .into_iter()
        .filter_map(|(enabled, capability)| enabled.then_some(capability))
        .filter(|capability| {
            return !faceted || !Self::is_operator_capability(capability);
        })
        .map(|capability| self.protocol_capability(&capability))
        .vec();
        let friends = match self.cfg.flags.friend {
            false => vec![],
            true => self
                .cfg
                .friends
                .iter()
                .filter_map(|friend| {
                    let capabilities = friend
                        .capabilities
                        .iter()
                        .filter(|capability| {
                            return !faceted
                                || !Self::is_operator_capability(capability);
                        })
                        .map(|capability| self.protocol_capability(capability))
                        .collect::<BTreeSet<_>>();
                    if capabilities.is_empty() {
                        return None;
                    }

                    let conversion = if self.target.is_target(&friend.ty) {
                        None
                    }
                    else {
                        let value = self.friend_conversion_expression(
                            &friend.ty,
                            friend.conv.as_ref(),
                            quote! { self },
                        );
                        Some(quote! { return #value; })
                    };

                    let friend_ty = &friend.ty;
                    return Some(ProtocolFriend {
                        ty: syn::parse2(
                            self.target
                                .external_type(&parse_quote! { #friend_ty }),
                        )
                        .expect(
                            "friend types remain valid after target lowering",
                        ),
                        capabilities,
                        conversion,
                    });
                })
                .vec(),
        };
        let checked = self.target_qualified_path(&self.fp_unchecked);
        let trusted = self.target.construct(
            &self.storage,
            self.target.constructor(),
            quote! { raw },
            self.cfg.flags.assertions,
        );

        let protocol = crate::friendship::emit_protocol(Protocol {
            target,
            generics: self.target.external_generics(),
            relation,
            seal,
            conversion,
            capabilities,
            friends,
            emit_seal: self.cfg.flags.friendzone_seal,
            target_conversion,
            construction: Some(Construction {
                make: self.trait_friend_make.clone(),
                trusted: self.protocol_capability(
                    &crate::friendship::cfg::trust_capability(),
                ),
                trusted_constructor: trusted,
                checked: quote! { #checked },
            }),
            konst: self.cfg.konst_friendship(),
        });
        let dispatch = self.ekran_facet_dispatch();
        return quote! {
            #protocol
            #dispatch
        };
    }

    pub(crate) fn make_range(&self) -> TokenStream {
        unimplemented!("range");
    }

    pub(crate) fn ekran(&self) -> syn::Result<TokenStream> {
        let impls = self.ekran_impls()?;
        let items = self.ekran_items();
        let bit_access = self.ekran_bit_access_items();

        let ty = self.target.ty();
        let (impl_generics, where_clause) = self.target.impl_header(&[], &[]);
        return Ok(quote::quote! {
            #impls

            impl #impl_generics #ty #where_clause {
                #items
            }

            #[allow(clippy::unnecessary_cast)]
            impl #impl_generics #ty #where_clause {
                #bit_access
            }
        });
    }

    pub(crate) fn ekran_items(&self) -> TokenStream {
        let ty = self.target.ty();
        let el = &self.el;
        let fp_unchecked = &self.fp_unchecked;
        let fp_get_raw = &self.fp_get_raw;
        let raw = self.storage.extract(quote! { self.0 });
        let arguments = self.target.arguments();

        let mut stream = TokenStream::new();

        if self.cfg.flags.fn_conv_of {
            let is_konst = self.cfg.konst_for(&[
                Konsts::Of,
                Konsts::Conversion,
            ]);
            let konst = zz_runner::konst(is_konst);
            let bonst = zz_runner::bonst(is_konst);
            let destruct = zz_runner::destruct(is_konst);
            let what = &self.trait_friend_make;
            let parameter = self.target.fresh_ident("T");

            let fn_of = format_ident!("of");

            let it = quote! {
                #[inline(always)]
                #[allow(private_bounds)]
                pub #konst fn #fn_of<#parameter>(
                    it: #parameter,
                ) -> #ty
                where
                    #parameter: #bonst #what #arguments #destruct,
                {
                    return <#parameter as #what #arguments>::make(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.auto_of_raw {
            let konst = zz_runner::konst(self.cfg.konst_for(&[
                Konsts::Of,
                Konsts::Conversion,
            ]));
            let fn_make = format_ident!("{}", self.cfg.flags.fn_make);
            let fn_of = self.target_qualified_path(&parse_quote! { Self::of });
            let it = quote! {
                #[inline(always)]
                pub #konst fn #fn_make(it: #el) -> #ty {
                    return #fn_of(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_conv_raw {
            let get_raw = &fp_get_raw.segments.last().unwrap().ident;

            let it = quote! {
                #[must_use]
                #[inline(always)]
                pub const fn #get_raw(self) -> #el {
                    return #raw;
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_conv_into {
            let it = N::items()
                .iter()
                .filter(|it| self.repr.can_safe_cast_to(**it))
                .map(|it| {
                    let target = format_ident!("{}", it.rust_name());
                    let conv = format_ident!("into_{}", it.rust_name());
                    return quote! {
                        #[must_use]
                        #[inline(always)]
                        #[allow(clippy::unnecessary_cast)]
                        pub const fn #conv(self) -> #target {
                            let it = #fp_get_raw(self);
                            return it as #target;
                        }
                    };
                })
                .merged();
            stream.extend(it);
        }

        if self.cfg.flags.map {
            let is_konst = self.cfg.konst_for(&[Konsts::Map])
                || self.cfg.flags.with_const_map_fn;
            let konst = zz_runner::konst(is_konst);
            let bonst = zz_runner::bonst(is_konst);
            let destruct = zz_runner::destruct(is_konst);
            let callback = self.target.fresh_ident("F");
            let value = quote! { value };
            let validation = self.cfg.validation.emit(&value);
            let constructed = self.target.construct(
                &self.storage,
                quote! { Self },
                value.clone(),
                self.cfg.flags.assertions,
            );
            let it = quote! {
                #[inline(always)]
                pub #konst fn try_map<#callback>(
                    self,
                    function: #callback,
                ) -> ::core::result::Result<Self, #el>
                where
                    #callback: #bonst ::core::ops::FnOnce(#el) -> #el #destruct,
                {
                    let value = function(#fp_get_raw(self));
                    return if #validation {
                        ::core::result::Result::Ok(#constructed)
                    }
                    else {
                        ::core::result::Result::Err(value)
                    };
                }

                #[inline(always)]
                #[track_caller]
                pub #konst fn map<#callback>(
                    self,
                    function: #callback,
                ) -> Self
                where
                    #callback: #bonst ::core::ops::FnOnce(#el) -> #el #destruct,
                {
                    return match self.try_map(function) {
                        ::core::result::Result::Ok(value) => value,
                        ::core::result::Result::Err(_) => {
                            ::core::panic!(
                                "mapping operation produced an invalid value"
                            )
                        }
                    };
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_conv_try_into_unchecked {
            let it = N::items()
                .iter()
                .filter(|it| self.repr.can_safe_cast_to(**it))
                .map(|it| {
                    let target = format_ident!("{}", it.rust_name());
                    let conv = format_ident!("try_into_{}", it.rust_name());
                    return quote! {
                        #[inline(always)]
                        #[allow(clippy::unnecessary_cast)]
                        pub const fn #conv(self) -> ::core::result::Result<#target, ()> {
                            return ::core::result::Result::Ok(
                                #fp_get_raw(self) as #target
                            );
                        }
                    };
                })
                .merged();
            stream.extend(it);
        }

        if self.cfg.flags.fn_conv_try_into_checked {
            let source = format_ident!("{}", self.repr.rust_name());
            let it = N::items()
                .iter()
                .filter(|it| !self.repr.can_safe_cast_to(**it))
                .map(|it| {
                    let target = format_ident!("{}", it.rust_name());
                    let name = format_ident!("try_into_{}", it.rust_name());
                    let sign_check = match (self.repr.is_signed(), it.is_signed()) {
                        (false, true) => quote! { t >= 0 },
                        (true, false) => quote! { r >= 0 },
                        _ => quote! { true },
                    };
                    return quote! {
                    #[inline(always)]
                    #[allow(clippy::unnecessary_cast)]
                    pub const fn #name(self) -> ::core::result::Result<#target, ()> {
                        let r = #fp_get_raw(self);
                        let t = r as #target;
                        let s = t as #source;

                        return if s == r && #sign_check {
                            ::core::result::Result::Ok(t)
                        }
                        else {
                            ::core::result::Result::Err(())
                        };
                    }
                };
                })
                .merged();
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_add {
            stream.extend(
                self.ekran_arithmetic_helper(ArithmeticOp::Add, quote! { #el }),
            );
        }

        if self.cfg.flags.fn_math_sub {
            stream.extend(
                self.ekran_arithmetic_helper(ArithmeticOp::Sub, quote! { #el }),
            );
        }

        if self.cfg.flags.fn_math_mul {
            stream.extend(
                self.ekran_arithmetic_helper(ArithmeticOp::Mul, quote! { #el }),
            );
        }

        if self.cfg.flags.fn_math_div {
            stream.extend(
                self.ekran_arithmetic_helper(ArithmeticOp::Div, quote! { #el }),
            );
        }

        if self.cfg.flags.fn_math_rem {
            stream.extend(
                self.ekran_arithmetic_helper(ArithmeticOp::Rem, quote! { #el }),
            );
        }

        if self.cfg.flags.fn_math_xor {
            let it = quote! {
                #[inline(always)]
                pub(self) const fn _bitxor(
                    &self,
                    rhs: #el
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs ^ rhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_and {
            let it = quote! {
                #[inline(always)]
                pub(self) const fn _bitand(
                    &self,
                    rhs: #el
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs & rhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_or {
            let it = quote! {
                #[inline(always)]
                pub(self) const fn _bitor(
                    &self,
                    rhs: #el
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = lhs | rhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_math_shr {
            stream.extend(
                self.ekran_arithmetic_helper(
                    ArithmeticOp::Shr,
                    quote! { usize },
                ),
            );
        }

        if self.cfg.flags.fn_math_shl {
            stream.extend(
                self.ekran_arithmetic_helper(
                    ArithmeticOp::Shl,
                    quote! { usize },
                ),
            );
        }

        if self.cfg.flags.fn_math_not {
            let it = quote! {
                #[inline(always)]
                pub(self) const fn _not(
                    &self,
                ) -> Self {
                    let lhs = #fp_get_raw(*self);
                    let it = !lhs;
                    return #fp_unchecked(it);
                }
            };
            stream.extend(it);
        }

        if self.cfg.profile.emits_precedes() {
            let unsigned = format_ident!("{}", self.repr.unsigned_rust_name());
            let konst = zz_runner::konst(self.cfg.konst_for(&[Konsts::Value]));
            let it = quote! {
                #[must_use]
                #[inline(always)]
                pub #konst fn precedes(self, other: Self) -> bool {
                    let lhs = #fp_get_raw(self) as #unsigned;
                    let rhs = #fp_get_raw(other) as #unsigned;
                    let difference = rhs.wrapping_sub(lhs);
                    let half = (1 as #unsigned) << (#unsigned::BITS - 1);
                    return difference != 0 && difference < half;
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_op_eq {
            let it = quote! {
                #[must_use]
                #[inline(always)]
                #[doc(hidden)]
                pub(self) const fn _eq(
                    self,
                    rhs: #el,
                ) -> bool {
                    let lhs = #fp_get_raw(self);
                    return lhs == rhs;
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fn_op_cmp {
            let it = quote! {
                #[must_use]
                #[inline(always)]
                #[doc(hidden)]
                pub(self) fn _cmp(
                    self,
                    rhs: #el,
                ) -> ::core::cmp::Ordering {
                    let lhs = #fp_get_raw(self);
                    return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs)
                        .unwrap();
                }
            };
            stream.extend(it);
        }

        let self1 = &self.cfg;
        let self2 = &self.cfg;
        if self.cfg.flags.fn_make_unchecked_try
            && self2.validation.has_validation()
        {
            let konst =
                zz_runner::konst(self.cfg.konst_for(&[Konsts::TryMake]));
            let value = quote! { it };
            let validation = self.cfg.validation.emit(&value);
            let constructed = self.target.construct(
                &self.storage,
                quote! { Self },
                value.clone(),
                self.cfg.flags.assertions,
            );
            let it = quote! {
                #[inline(always)]
                pub #konst fn try_make(it: #el) -> Result<Self, #el> {
                    return if #validation {
                        return ::core::result::Result::Ok(#constructed);
                    }
                    else {
                        ::core::result::Result::Err(it)
                    };
                }
            };
            stream.extend(it);
        }
        else if self.cfg.flags.fn_make_checked_try
            && !self1.validation.has_validation()
        {
            let it = quote! {
                #[inline(always)]
                pub const fn try_make(it: #el) -> Result<Self, #el> {
                    return Ok(#fp_unchecked(it));
                }
            };
            stream.extend(it);
        }

        let self1 = &self.cfg;
        let self2 = &self.cfg;
        if self.cfg.flags.fn_make_unchecked && self2.validation.has_validation()
        {
            let value = quote! { it };
            let validation = self.cfg.validation.emit(&value);
            let constructed = self.target.construct(
                &self.storage,
                quote! { Self },
                value.clone(),
                self.cfg.flags.assertions,
            );
            let it = quote! {
                #[must_use]
                #[inline(always)]
                pub(self) const fn _unchecked(it: #el) -> Self {
                    if #validation {
                        return #constructed;
                    }
                    else {
                        ::core::panic!("invalid value");
                    };
                }
            };
            stream.extend(it);
        }
        else if self.cfg.flags.fn_make_checked
            && !self1.validation.has_validation()
        {
            let constructed = self.target.construct(
                &self.storage,
                quote! { Self },
                quote! { it },
                self.cfg.flags.assertions,
            );
            let it = quote! {
                #[must_use]
                #[inline(always)]
                pub(self) const fn _unchecked(it: #el) -> Self {
                    return #constructed;
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.core_int {
            let konst = zz_runner::konst(self.cfg.konst_for(&[
                Konsts::CoreInt,
                Konsts::Value,
            ]));
            let value = quote! { value };
            let validation = self.cfg.validation.emit(&value);
            let constructed = self.target.construct(
                &self.storage,
                quote! { Self },
                value.clone(),
                self.cfg.flags.assertions,
            );
            let it = quote! {
                #[inline(always)]
                #[track_caller]
                pub(self) #konst fn _core_int(value: #el) -> Self {
                    if #validation {
                        return #constructed;
                    }
                    else {
                        ::core::panic!("operation produced an invalid value");
                    }
                }

                #[inline(always)]
                pub(self) #konst fn _core_int_checked(
                    value: #el,
                ) -> ::core::option::Option<Self> {
                    return if #validation {
                        ::core::option::Option::Some(#constructed)
                    }
                    else {
                        ::core::option::Option::None
                    };
                }
            };
            stream.extend(it);
        }

        return stream;
    }

    pub(crate) fn ekran_impls(&self) -> syn::Result<TokenStream> {
        let ty = self.target.ty();
        let (impl_generics, where_clause) = self.target.impl_header(&[], &[]);
        let arguments = self.target.arguments();
        let rhs_ty = self.target.fresh_ident("T");
        let (rhs_impl_generics, _) =
            self.target.impl_header(&[parse_quote! { #rhs_ty }], &[]);
        let rhs_where_clause = |bounds: TokenStream| {
            return self
                .target
                .impl_header(
                    &[parse_quote! { #rhs_ty }],
                    &[parse_quote! { #rhs_ty: #bounds }],
                )
                .1;
        };
        let el = &self.el;
        let value = quote! { value };
        let validation = self.cfg.validation.emit(&value);

        let bit = &self.trait_friend_bit;
        let math = &self.trait_friend_math;
        let rel = &self.trait_friend_rel;
        let seal = &self.trait_seal;
        let trait_friend_bit = quote! { #bit #arguments };
        let trait_friend_math = quote! { #math #arguments };
        let trait_friend_rel = quote! { #rel #arguments };
        let trait_seal = quote! { #seal #arguments };

        let fn_conv = &self.fn_conv;
        let fp_friend_conv = if self.target.has_generics() {
            quote! { <#rhs_ty as #trait_seal>::#fn_conv }
        }
        else {
            quote! { #trait_seal::#fn_conv }
        };
        let fp_get_raw = &self.fp_get_raw;
        let rhs_raw = self.storage.extract(quote! { rhs.0 });
        let self_raw = self.storage.extract(quote! { self.0 });
        let self_raw_ref = self.storage.extract_ref(quote! { self.0 });

        let global_operators = !self.cfg.facets_present();
        let with_self_math = self.cfg.flags.self_math
            && global_operators
            && !self.target.has_generics();
        let with_self_bit = self.cfg.flags.self_bit
            && global_operators
            && !self.target.has_generics();
        let output = self.target.operator_output(&self.cfg.ty_out, false);

        let bounds_for = |flags: &[Konsts]| {
            let is_konst = self.cfg.konst_for(flags);
            let bonst = zz_runner::bonst(is_konst);
            let destruct = zz_runner::destruct(is_konst);
            return quote! { #bonst #trait_seal #destruct };
        };

        let mut stream = TokenStream::new();
        stream.extend(self.ekran_friendship());

        if self.cfg.flags.as_ref_ {
            let it = quote! {
                impl #impl_generics ::core::convert::AsRef<#el> for #ty
                #where_clause {
                    #[inline(always)]
                    fn as_ref(&self) -> &#el {
                        return #self_raw_ref;
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.from_str {
            let constructed = self.target.construct(
                &self.storage,
                quote! { Self },
                quote! { value },
                self.cfg.flags.assertions,
            );
            stream.extend(quote! {
                impl #impl_generics ::core::str::FromStr for #ty
                #where_clause {
                    type Err = ();

                    #[inline(always)]
                    fn from_str(
                        source: &str,
                    ) -> ::core::result::Result<Self, Self::Err> {
                        let value =
                            match <#el as ::core::str::FromStr>::from_str(source) {
                                ::core::result::Result::Ok(value) => value,
                                ::core::result::Result::Err(_) => {
                                    return ::core::result::Result::Err(());
                                }
                            };

                        return if #validation {
                            ::core::result::Result::Ok(#constructed)
                        }
                        else {
                            ::core::result::Result::Err(())
                        };
                    }
                }
            });
        }

        if self.cfg.flags.serde {
            let constructed = self.target.construct(
                &self.storage,
                quote! { Self },
                quote! { value },
                self.cfg.flags.assertions,
            );
            let serializer = self.target.fresh_ident("S");
            let deserializer = self.target.fresh_ident("D");
            let de = self.target.fresh_lifetime("de");
            let (de_generics, de_where_clause) =
                self.target.impl_header(&[parse_quote! { #de }], &[]);
            stream.extend(quote! {
                impl #impl_generics ::serde::Serialize for #ty
                #where_clause {
                    #[inline(always)]
                    fn serialize<#serializer>(
                        &self,
                        serializer: #serializer,
                    ) -> ::core::result::Result<#serializer::Ok, #serializer::Error>
                    where
                        #serializer: ::serde::Serializer,
                    {
                        let value = #fp_get_raw(*self);
                        return <#el as ::serde::Serialize>::serialize(
                            &value,
                            serializer,
                        );
                    }
                }

                impl #de_generics ::serde::Deserialize<#de> for #ty
                #de_where_clause {
                    #[inline(always)]
                    fn deserialize<#deserializer>(
                        deserializer: #deserializer,
                    ) -> ::core::result::Result<Self, #deserializer::Error>
                    where
                        #deserializer: ::serde::Deserializer<#de>,
                    {
                        let value =
                            <#el as ::serde::Deserialize<#de>>::deserialize(
                                deserializer,
                            )?;

                        return if #validation {
                            ::core::result::Result::Ok(#constructed)
                        }
                        else {
                            ::core::result::Result::Err(
                                <#deserializer::Error as ::serde::de::Error>::custom(
                                    "invalid value",
                                ),
                            )
                        };
                    }
                }
            });
        }

        if self.cfg.flags.core_int {
            let it = quote! {
                impl #impl_generics #ty #where_clause {
                    /// The number of bits in the wrapped primitive integer.
                    pub const BITS: u32 = #el::BITS;

                    #[inline(always)]
                    pub const fn count_ones(self) -> u32 {
                        return #fp_get_raw(self).count_ones();
                    }

                    #[inline(always)]
                    pub const fn count_zeros(self) -> u32 {
                        return #fp_get_raw(self).count_zeros();
                    }

                    #[inline(always)]
                    pub const fn leading_zeros(self) -> u32 {
                        return #fp_get_raw(self).leading_zeros();
                    }

                    #[inline(always)]
                    pub const fn trailing_zeros(self) -> u32 {
                        return #fp_get_raw(self).trailing_zeros();
                    }

                    #[inline(always)]
                    pub const fn leading_ones(self) -> u32 {
                        return #fp_get_raw(self).leading_ones();
                    }

                    #[inline(always)]
                    pub const fn trailing_ones(self) -> u32 {
                        return #fp_get_raw(self).trailing_ones();
                    }

                    #[inline(always)]
                    pub const fn highest_one(self) -> ::core::option::Option<u32> {
                        return #fp_get_raw(self).highest_one();
                    }

                    #[inline(always)]
                    pub const fn lowest_one(self) -> ::core::option::Option<u32> {
                        return #fp_get_raw(self).lowest_one();
                    }

                    #[inline(always)]
                    pub const fn ilog(self, base: Self) -> u32 {
                        return #fp_get_raw(self).ilog(#fp_get_raw(base));
                    }

                    #[inline(always)]
                    pub const fn ilog2(self) -> u32 {
                        return #fp_get_raw(self).ilog2();
                    }

                    #[inline(always)]
                    pub const fn ilog10(self) -> u32 {
                        return #fp_get_raw(self).ilog10();
                    }

                    #[inline(always)]
                    pub const fn checked_ilog(
                        self,
                        base: Self,
                    ) -> ::core::option::Option<u32> {
                        return #fp_get_raw(self).checked_ilog(#fp_get_raw(base));
                    }

                    #[inline(always)]
                    pub const fn checked_ilog2(self) -> ::core::option::Option<u32> {
                        return #fp_get_raw(self).checked_ilog2();
                    }

                    #[inline(always)]
                    pub const fn checked_ilog10(self) -> ::core::option::Option<u32> {
                        return #fp_get_raw(self).checked_ilog10();
                    }

                    #[inline(always)]
                    pub const fn to_be_bytes(
                        self,
                    ) -> [u8; ::core::mem::size_of::<#el>()] {
                        return #fp_get_raw(self).to_be_bytes();
                    }

                    #[inline(always)]
                    pub const fn to_le_bytes(
                        self,
                    ) -> [u8; ::core::mem::size_of::<#el>()] {
                        return #fp_get_raw(self).to_le_bytes();
                    }

                    #[inline(always)]
                    pub const fn to_ne_bytes(
                        self,
                    ) -> [u8; ::core::mem::size_of::<#el>()] {
                        return #fp_get_raw(self).to_ne_bytes();
                    }
                }
            };
            stream.extend(it);

            let konst = zz_runner::konst(self.cfg.konst_for(&[
                Konsts::CoreInt,
                Konsts::Value,
            ]));
            let it = quote! {
                impl #impl_generics #ty #where_clause {
                    #[inline(always)]
                    pub #konst fn rotate_left(self, n: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).rotate_left(n));
                    }

                    #[inline(always)]
                    pub #konst fn rotate_right(self, n: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).rotate_right(n));
                    }

                    #[inline(always)]
                    pub #konst fn swap_bytes(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).swap_bytes());
                    }

                    #[inline(always)]
                    pub #konst fn reverse_bits(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).reverse_bits());
                    }

                    #[inline(always)]
                    pub #konst fn isolate_highest_one(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).isolate_highest_one());
                    }

                    #[inline(always)]
                    pub #konst fn isolate_lowest_one(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).isolate_lowest_one());
                    }

                    #[inline(always)]
                    pub #konst fn to_be(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).to_be());
                    }

                    #[inline(always)]
                    pub #konst fn to_le(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).to_le());
                    }

                    #[inline(always)]
                    pub #konst fn from_be(value: Self) -> Self {
                        return Self::_core_int(#el::from_be(#fp_get_raw(value)));
                    }

                    #[inline(always)]
                    pub #konst fn from_le(value: Self) -> Self {
                        return Self::_core_int(#el::from_le(#fp_get_raw(value)));
                    }

                    #[inline(always)]
                    pub #konst fn from_ne_bytes(
                        bytes: [u8; ::core::mem::size_of::<#el>()],
                    ) -> Self {
                        return Self::_core_int(#el::from_ne_bytes(bytes));
                    }

                    #[inline(always)]
                    pub #konst fn from_be_bytes(
                        bytes: [u8; ::core::mem::size_of::<#el>()],
                    ) -> Self {
                        return Self::_core_int(#el::from_be_bytes(bytes));
                    }

                    #[inline(always)]
                    pub #konst fn from_le_bytes(
                        bytes: [u8; ::core::mem::size_of::<#el>()],
                    ) -> Self {
                        return Self::_core_int(#el::from_le_bytes(bytes));
                    }

                    #[inline(always)]
                    pub #konst fn checked_add(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_add(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_sub(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_sub(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_neg(
                        self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_neg() {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_mul(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_mul(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_div(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_div(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_rem(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_rem(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_div_euclid(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_div_euclid(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_rem_euclid(
                        self,
                        rhs: Self,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_rem_euclid(#fp_get_raw(rhs)) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_shl(
                        self,
                        rhs: u32,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_shl(rhs) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_shr(
                        self,
                        rhs: u32,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_shr(rhs) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                    #[inline(always)]
                    pub #konst fn checked_pow(
                        self,
                        exp: u32,
                    ) -> ::core::option::Option<Self> {
                        return match #fp_get_raw(self).checked_pow(exp) {
                            ::core::option::Option::Some(value) => {
                                Self::_core_int_checked(value)
                            }
                            ::core::option::Option::None => {
                                ::core::option::Option::None
                            }
                        };
                    }

                }
            };
            stream.extend(it);

            let it = quote! {
                impl #impl_generics #ty #where_clause {
                    #[inline(always)]
                    pub #konst fn midpoint(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).midpoint(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn unbounded_shl(self, rhs: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).unbounded_shl(rhs));
                    }

                    #[inline(always)]
                    pub #konst fn unbounded_shr(self, rhs: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).unbounded_shr(rhs));
                    }

                    #[inline(always)]
                    pub #konst fn saturating_add(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).saturating_add(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn saturating_sub(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).saturating_sub(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn saturating_mul(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).saturating_mul(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn saturating_div(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).saturating_div(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn saturating_pow(self, exp: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).saturating_pow(exp));
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_add(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_add(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_sub(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_sub(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_neg(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).wrapping_neg());
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_mul(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_mul(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_div(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_div(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_rem(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_rem(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_div_euclid(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_div_euclid(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).wrapping_rem_euclid(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_shl(self, rhs: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).wrapping_shl(rhs));
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_shr(self, rhs: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).wrapping_shr(rhs));
                    }

                    #[inline(always)]
                    pub #konst fn wrapping_pow(self, exp: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).wrapping_pow(exp));
                    }

                    #[inline(always)]
                    pub #konst fn pow(self, exp: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).pow(exp));
                    }

                    #[inline(always)]
                    pub #konst fn isqrt(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).isqrt());
                    }

                    #[inline(always)]
                    pub #konst fn div_euclid(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).div_euclid(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn rem_euclid(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).rem_euclid(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_add(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_add(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_sub(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_sub(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_neg(self) -> Self {
                        return Self::_core_int(#fp_get_raw(self).strict_neg());
                    }

                    #[inline(always)]
                    pub #konst fn strict_mul(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_mul(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_div(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_div(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_rem(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_rem(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_div_euclid(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_div_euclid(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_rem_euclid(self, rhs: Self) -> Self {
                        return Self::_core_int(
                            #fp_get_raw(self).strict_rem_euclid(#fp_get_raw(rhs))
                        );
                    }

                    #[inline(always)]
                    pub #konst fn strict_shl(self, rhs: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).strict_shl(rhs));
                    }

                    #[inline(always)]
                    pub #konst fn strict_shr(self, rhs: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).strict_shr(rhs));
                    }

                    #[inline(always)]
                    pub #konst fn strict_pow(self, exp: u32) -> Self {
                        return Self::_core_int(#fp_get_raw(self).strict_pow(exp));
                    }
                }
            };
            stream.extend(it);

            let it = quote! {
                impl #impl_generics #ty #where_clause {
                    #[inline(always)]
                    pub #konst fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_add(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_sub(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_neg(self) -> (Self, bool) {
                        let (value, overflowed) = #fp_get_raw(self).overflowing_neg();
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_mul(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_div(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_rem(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_div_euclid(
                        self,
                        rhs: Self,
                    ) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_div_euclid(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_rem_euclid(
                        self,
                        rhs: Self,
                    ) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_rem_euclid(#fp_get_raw(rhs));
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_shl(rhs);
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_shr(rhs);
                        return (Self::_core_int(value), overflowed);
                    }

                    #[inline(always)]
                    pub #konst fn overflowing_pow(
                        self,
                        exp: u32,
                    ) -> (Self, bool) {
                        let (value, overflowed) =
                            #fp_get_raw(self).overflowing_pow(exp);
                        return (Self::_core_int(value), overflowed);
                    }
                }
            };
            stream.extend(it);

            if self.repr.is_signed() {
                let unsigned =
                    format_ident!("{}", self.repr.unsigned_rust_name());
                let it = quote! {
                    impl #impl_generics #ty #where_clause {
                        #[inline(always)]
                        pub #konst fn cast_unsigned(self) -> #unsigned {
                            return #fp_get_raw(self).cast_unsigned();
                        }

                        #[inline(always)]
                        pub #konst fn abs(self) -> Self {
                            return Self::_core_int(#fp_get_raw(self).abs());
                        }

                        #[inline(always)]
                        pub #konst fn checked_abs(
                            self,
                        ) -> ::core::option::Option<Self> {
                            return match #fp_get_raw(self).checked_abs() {
                                ::core::option::Option::Some(value) => {
                                    Self::_core_int_checked(value)
                                }
                                ::core::option::Option::None => {
                                    ::core::option::Option::None
                                }
                            };
                        }

                        #[inline(always)]
                        pub #konst fn checked_isqrt(
                            self,
                        ) -> ::core::option::Option<Self> {
                            return match #fp_get_raw(self).checked_isqrt() {
                                ::core::option::Option::Some(value) => {
                                    Self::_core_int_checked(value)
                                }
                                ::core::option::Option::None => {
                                    ::core::option::Option::None
                                }
                            };
                        }

                        #[inline(always)]
                        pub #konst fn strict_abs(self) -> Self {
                            return Self::_core_int(#fp_get_raw(self).strict_abs());
                        }

                        #[inline(always)]
                        pub #konst fn saturating_neg(self) -> Self {
                            return Self::_core_int(#fp_get_raw(self).saturating_neg());
                        }

                        #[inline(always)]
                        pub #konst fn saturating_abs(self) -> Self {
                            return Self::_core_int(#fp_get_raw(self).saturating_abs());
                        }

                        #[inline(always)]
                        pub #konst fn wrapping_abs(self) -> Self {
                            return Self::_core_int(#fp_get_raw(self).wrapping_abs());
                        }

                        #[inline(always)]
                        pub #konst fn overflowing_abs(self) -> (Self, bool) {
                            let (value, overflowed) =
                                #fp_get_raw(self).overflowing_abs();
                            return (Self::_core_int(value), overflowed);
                        }

                        #[inline(always)]
                        pub #konst fn unsigned_abs(self) -> #unsigned {
                            return #fp_get_raw(self).unsigned_abs();
                        }

                        #[inline(always)]
                        pub #konst fn abs_diff(self, rhs: Self) -> #unsigned {
                            return #fp_get_raw(self).abs_diff(#fp_get_raw(rhs));
                        }

                        #[inline(always)]
                        pub #konst fn signum(self) -> Self {
                            return Self::_core_int(#fp_get_raw(self).signum());
                        }

                        #[inline(always)]
                        pub #konst fn is_positive(self) -> bool {
                            return #fp_get_raw(self).is_positive();
                        }

                        #[inline(always)]
                        pub #konst fn is_negative(self) -> bool {
                            return #fp_get_raw(self).is_negative();
                        }
                    }
                };
                stream.extend(it);
            }
            else {
                let signed = format_ident!(
                    "{}",
                    match self.repr {
                        N::USIZ => "isize",
                        N::U008 => "i8",
                        N::U016 => "i16",
                        N::U032 => "i32",
                        N::U064 => "i64",
                        N::U128 => "i128",
                        _ => unreachable!("unsigned representation"),
                    }
                );
                let funnel =
                    matches!(self.cfg.konst, Konst::Nightly).then(|| {
                        quote! {
                            #[inline(always)]
                            pub #konst fn funnel_shl(
                                self,
                                right: Self,
                                n: u32,
                            ) -> Self {
                                return Self::_core_int(
                                    #fp_get_raw(self).funnel_shl(
                                        #fp_get_raw(right),
                                        n,
                                    )
                                );
                            }

                            #[inline(always)]
                            pub #konst fn funnel_shr(
                                self,
                                right: Self,
                                n: u32,
                            ) -> Self {
                                return Self::_core_int(
                                    #fp_get_raw(self).funnel_shr(
                                        #fp_get_raw(right),
                                        n,
                                    )
                                );
                            }
                        }
                    });
                let it = quote! {
                    impl #impl_generics #ty #where_clause {
                        #[inline(always)]
                        pub #konst fn cast_signed(self) -> #signed {
                            return #fp_get_raw(self).cast_signed();
                        }

                        #[inline(always)]
                        pub #konst fn bit_width(self) -> u32 {
                            return #fp_get_raw(self).bit_width();
                        }

                        #funnel

                        #[inline(always)]
                        pub #konst fn abs_diff(self, rhs: Self) -> #el {
                            return #fp_get_raw(self).abs_diff(#fp_get_raw(rhs));
                        }

                        #[inline(always)]
                        pub #konst fn is_multiple_of(self, rhs: Self) -> bool {
                            return #fp_get_raw(self).is_multiple_of(#fp_get_raw(rhs));
                        }

                        #[inline(always)]
                        pub #konst fn is_power_of_two(self) -> bool {
                            return #fp_get_raw(self).is_power_of_two();
                        }

                        #[inline(always)]
                        pub #konst fn next_power_of_two(self) -> Self {
                            return Self::_core_int(
                                #fp_get_raw(self).next_power_of_two()
                            );
                        }

                        #[inline(always)]
                        pub #konst fn checked_next_power_of_two(
                            self,
                        ) -> ::core::option::Option<Self> {
                            return match #fp_get_raw(self).checked_next_power_of_two() {
                                ::core::option::Option::Some(value) => {
                                    Self::_core_int_checked(value)
                                }
                                ::core::option::Option::None => {
                                    ::core::option::Option::None
                                }
                            };
                        }

                        #[inline(always)]
                        pub #konst fn div_ceil(self, rhs: Self) -> Self {
                            return Self::_core_int(
                                #fp_get_raw(self).div_ceil(#fp_get_raw(rhs))
                            );
                        }

                        #[inline(always)]
                        pub #konst fn next_multiple_of(self, rhs: Self) -> Self {
                            return Self::_core_int(
                                #fp_get_raw(self).next_multiple_of(#fp_get_raw(rhs))
                            );
                        }

                        #[inline(always)]
                        pub #konst fn checked_next_multiple_of(
                            self,
                            rhs: Self,
                        ) -> ::core::option::Option<Self> {
                            return match #fp_get_raw(self).checked_next_multiple_of(
                                #fp_get_raw(rhs),
                            ) {
                                ::core::option::Option::Some(value) => {
                                    Self::_core_int_checked(value)
                                }
                                ::core::option::Option::None => {
                                    ::core::option::Option::None
                                }
                            };
                        }

                        #[inline(always)]
                        pub fn carrying_add(
                            self,
                            rhs: Self,
                            carry: bool,
                        ) -> (Self, bool) {
                            let (value, carry) = #fp_get_raw(self).carrying_add(
                                #fp_get_raw(rhs),
                                carry,
                            );
                            return (Self::_core_int(value), carry);
                        }

                        #[inline(always)]
                        pub fn borrowing_sub(
                            self,
                            rhs: Self,
                            borrow: bool,
                        ) -> (Self, bool) {
                            let (value, borrow) = #fp_get_raw(self).borrowing_sub(
                                #fp_get_raw(rhs),
                                borrow,
                            );
                            return (Self::_core_int(value), borrow);
                        }

                        #[inline(always)]
                        pub fn carrying_mul(
                            self,
                            rhs: Self,
                            carry: Self,
                        ) -> (Self, Self) {
                            let (low, high) = #fp_get_raw(self).carrying_mul(
                                #fp_get_raw(rhs),
                                #fp_get_raw(carry),
                            );
                            return (
                                Self::_core_int(low),
                                Self::_core_int(high),
                            );
                        }

                        #[inline(always)]
                        pub fn carrying_mul_add(
                            self,
                            rhs: Self,
                            carry: Self,
                            add: Self,
                        ) -> (Self, Self) {
                            let (low, high) =
                                #fp_get_raw(self).carrying_mul_add(
                                    #fp_get_raw(rhs),
                                    #fp_get_raw(carry),
                                    #fp_get_raw(add),
                                );
                            return (
                                Self::_core_int(low),
                                Self::_core_int(high),
                            );
                        }
                    }
                };
                stream.extend(it);
            }
        }

        if self.cfg.flags.into {
            let konst = zz_runner::konst(self.cfg.konst_for(&[
                Konsts::Into,
                Konsts::Conversion,
            ]));
            let it = N::items()
                .iter()
                .filter(|it| self.repr.can_safe_cast_to(**it))
                .map(|it| {
                    let target = format_ident!("{}", it.rust_name());
                    let conv = format_ident!("into_{}", it.rust_name());
                    return quote! {
                       #konst impl #impl_generics ::core::convert::Into<#target> for #ty
                       #where_clause {
                           #[inline(always)]
                           fn into(self) -> #target {
                                return Self::#conv(self);
                           }
                       }
                    };
                })
                .merged();
            stream.extend(it);
        }

        if self.cfg.flags.try_into {
            let konst = zz_runner::konst(self.cfg.konst_for(&[
                Konsts::TryInto,
                Konsts::Conversion,
            ]));
            let it = N::items()
                .iter()
                .filter(|it| !self.repr.can_safe_cast_to(**it))
                .map(|it| {
                    let target = format_ident!("{}", it.rust_name());
                    let conv = format_ident!("try_into_{}", it.rust_name());
                    return quote! {
                       #konst impl #impl_generics ::core::convert::TryInto<#target> for #ty
                       #where_clause {
                           type Error = ();

                           #[inline(always)]
                           fn try_into(self) -> ::core::result::Result<#target, Self::Error> {
                                return Self::#conv(self);
                           }

                       }
                    };
                })
                .merged();
            stream.extend(it);
        }

        if self.cfg.flags.math_shr {
            let is_konst = self.cfg.konst_for(&[
                Konsts::Shr,
                Konsts::Math,
            ]);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Shr, is_konst);
            let output_ty = output.ty();
            let result = output.convert(quote! { self.#method(rhs) });
            let it = quote! {
                #konst impl #impl_generics ::core::ops::Shr<usize> for #ty
                #where_clause {
                    type Output = #output_ty;

                    #[inline(always)]
                    fn shr(
                        self,
                        rhs: usize,
                    ) -> Self::Output {
                        return #result;
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.math_shl {
            let is_konst = self.cfg.konst_for(&[
                Konsts::Shl,
                Konsts::Math,
            ]);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Shl, is_konst);
            let output_ty = output.ty();
            let result = output.convert(quote! { self.#method(rhs) });
            let it = quote::quote! {
                #konst impl #impl_generics ::core::ops::Shl<usize> for #ty
                #where_clause {
                    type Output = #output_ty;

                    #[inline(always)]
                    fn shl(
                        self,
                        rhs: usize,
                    ) -> Self::Output {
                        return #result;
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.assign_shr {
            let is_konst = self.cfg.konst_for(&[
                Konsts::ShrAssign,
                Konsts::Shr,
                Konsts::Assign,
                Konsts::Math,
            ]);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Shr, is_konst);
            let it = quote! {
                #konst impl #impl_generics ::core::ops::ShrAssign<usize> for #ty
                #where_clause {
                    #[inline(always)]
                    fn shr_assign(
                        &mut self,
                        other: usize,
                    ) {
                        *self = self.#method(other);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.assign_shl {
            let is_konst = self.cfg.konst_for(&[
                Konsts::ShlAssign,
                Konsts::Shl,
                Konsts::Assign,
                Konsts::Math,
            ]);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Shl, is_konst);
            let it = quote! {
                #konst impl #impl_generics ::core::ops::ShlAssign<usize> for #ty
                #where_clause {
                    #[inline(always)]
                    fn shl_assign(
                        &mut self,
                        other: usize,
                    ) {
                        *self = self.#method(other);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.assign_and && global_operators {
            let flags = [
                Konsts::AndAssign,
                Konsts::And,
                Konsts::Assign,
                Konsts::Bit,
            ];
            let konst = zz_runner::konst(self.cfg.konst_for(&flags));
            let bounds = bounds_for(&flags);
            if with_self_bit {
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::BitAndAssign for #ty
                    #where_clause {
                        #[inline(always)]
                        fn bitand_assign(
                            &mut self,
                            rhs: Self,
                        ) {
                            *self = self._bitand(#rhs_raw);
                        }
                    }
                });
            }

            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_bit + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::BitAndAssign<#rhs_ty> for #ty
                #rhs_where
                {
                    #[inline(always)]
                    fn bitand_assign(
                        &mut self,
                        rhs: #rhs_ty,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self._bitand(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.assign_add && global_operators {
            let flags = [
                Konsts::AddAssign,
                Konsts::Add,
                Konsts::Assign,
                Konsts::Math,
            ];
            let is_konst = self.cfg.konst_for(&flags);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Add, is_konst);
            let bounds = bounds_for(&flags);
            if with_self_math {
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::AddAssign for #ty
                    #where_clause {
                        #[inline(always)]
                        fn add_assign(
                            &mut self,
                            rhs: Self,
                        ) {
                            *self = self.#method(#rhs_raw);
                        }
                    }
                });
            }

            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_math + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::AddAssign<#rhs_ty> for #ty
                #rhs_where
                {
                    #[inline(always)]
                    fn add_assign(
                        &mut self,
                        rhs: #rhs_ty,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self.#method(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.assign_sub && global_operators {
            let flags = [
                Konsts::SubAssign,
                Konsts::Sub,
                Konsts::Assign,
                Konsts::Math,
            ];
            let is_konst = self.cfg.konst_for(&flags);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Sub, is_konst);
            let bounds = bounds_for(&flags);
            if with_self_math {
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::SubAssign for #ty
                    #where_clause {
                        #[inline(always)]
                        fn sub_assign(
                            &mut self,
                            rhs: Self,
                        ) {
                            *self = self.#method(#rhs_raw);
                        }
                    }
                });
            }

            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_math + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::SubAssign<#rhs_ty> for #ty
                #rhs_where
                {
                    #[inline(always)]
                    fn sub_assign(
                        &mut self,
                        rhs: #rhs_ty,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self.#method(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.assign_mul && global_operators {
            let flags = [
                Konsts::MulAssign,
                Konsts::Mul,
                Konsts::Assign,
                Konsts::Math,
            ];
            let is_konst = self.cfg.konst_for(&flags);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Mul, is_konst);
            let bounds = bounds_for(&flags);
            if with_self_math {
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::MulAssign for #ty
                    #where_clause {
                        #[inline(always)]
                        fn mul_assign(
                            &mut self,
                            rhs: Self,
                        ) {
                            *self = self.#method(#rhs_raw);
                        }
                    }
                });
            }

            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_math + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::MulAssign<#rhs_ty> for #ty
                #rhs_where
                {
                    #[inline(always)]
                    fn mul_assign(
                        &mut self,
                        rhs: #rhs_ty,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self.#method(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.assign_div && global_operators {
            let flags = [
                Konsts::DivAssign,
                Konsts::Div,
                Konsts::Assign,
                Konsts::Math,
            ];
            let is_konst = self.cfg.konst_for(&flags);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Div, is_konst);
            let bounds = bounds_for(&flags);
            if with_self_math {
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::DivAssign for #ty
                    #where_clause {
                        #[inline(always)]
                        fn div_assign(
                            &mut self,
                            rhs: Self,
                        ) {
                            *self = self.#method(#rhs_raw);
                        }
                    }
                });
            }

            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_math + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::DivAssign<#rhs_ty> for #ty
                #rhs_where
                {
                    #[inline(always)]
                    fn div_assign(
                        &mut self,
                        rhs: #rhs_ty,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self.#method(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.assign_rem && global_operators {
            let flags = [
                Konsts::RemAssign,
                Konsts::Rem,
                Konsts::Assign,
                Konsts::Math,
            ];
            let is_konst = self.cfg.konst_for(&flags);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Rem, is_konst);
            let bounds = bounds_for(&flags);
            if with_self_math {
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::RemAssign for #ty
                    #where_clause {
                        #[inline(always)]
                        fn rem_assign(
                            &mut self,
                            rhs: Self,
                        ) {
                            *self = self.#method(#rhs_raw);
                        }
                    }
                });
            }

            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_math + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::RemAssign<#rhs_ty> for #ty
                #rhs_where
                {
                    #[inline(always)]
                    fn rem_assign(
                        &mut self,
                        rhs: #rhs_ty,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self.#method(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.assign_or && global_operators {
            let flags = [
                Konsts::OrAssign,
                Konsts::Or,
                Konsts::Assign,
                Konsts::Bit,
            ];
            let konst = zz_runner::konst(self.cfg.konst_for(&flags));
            let bounds = bounds_for(&flags);
            if with_self_bit {
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::BitOrAssign for #ty
                    #where_clause {
                        #[inline(always)]
                        fn bitor_assign(
                            &mut self,
                            rhs: Self,
                        ) {
                            *self = self._bitor(#rhs_raw);
                        }
                    }
                });
            }

            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_bit + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::BitOrAssign<#rhs_ty> for #ty
                #rhs_where
                {
                    #[inline(always)]
                    fn bitor_assign(
                        &mut self,
                        rhs: #rhs_ty,
                    ) {
                        let it = #fp_friend_conv(rhs);
                        *self = self._bitor(it);
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.range {
            stream.extend(self.make_range());
        }

        if self.cfg.flags.debug {
            let name = self.target.ident.to_string();
            let it = quote! {
                impl #impl_generics ::core::fmt::Debug for #ty #where_clause {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        return f
                            .debug_tuple(#name)
                            .field(&#self_raw)
                            .finish();
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.hash {
            let hasher = self.target.fresh_ident("H");
            let it = quote! {
                impl #impl_generics ::core::hash::Hash for #ty #where_clause {
                    #[inline(always)]
                    fn hash<#hasher>(
                        &self,
                        state: &mut #hasher,
                    )
                    where
                        #hasher: ::core::hash::Hasher,
                    {
                        return ::core::hash::Hash::hash(&#self_raw, state);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.eq {
            let konst = zz_runner::konst(self.cfg.konst_for(&[
                Konsts::Eq,
                Konsts::Comparison,
            ]));
            // TODO why can't we do derive?
            let it = quote! {
                #konst impl #impl_generics ::core::cmp::Eq for #ty #where_clause {}
            };
            stream.extend(it);
        }

        if self.cfg.flags.ord {
            let konst = zz_runner::konst(self.cfg.konst_for(&[
                Konsts::Ord,
                Konsts::Comparison,
            ]));
            let it = quote! {
                #konst impl #impl_generics ::core::cmp::Ord for #ty #where_clause {
                    #[inline(always)]
                    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                        return self.partial_cmp(other).unwrap();
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.assertions {
            let assertions = self.storage.layout_assertions(quote! { #ty });
            if self.target.has_generics() {
                let name = self.target.layout_assertion();
                stream.extend(quote! {
                    impl #impl_generics #ty #where_clause {
                        const #name: () = {
                            #assertions
                        };
                    }
                });
            }
            else {
                stream.extend(assertions);
            }
        }

        if self.cfg.flags.math_add && global_operators {
            let flags = [
                Konsts::Add,
                Konsts::Math,
            ];
            let is_konst = self.cfg.konst_for(&flags);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Add, is_konst);
            let bounds = bounds_for(&flags);
            if with_self_math {
                let output_ty = output.ty();
                let result = output.convert(quote! { self.#method(#rhs_raw) });
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::Add for #ty #where_clause {
                        type Output = #output_ty;

                        #[inline(always)]
                        fn add(
                            self,
                            rhs: Self,
                        ) -> Self::Output {
                            return #result;
                        }
                    }
                });
            }

            let output_ty = output.ty();
            let result = output.convert(quote! { self.#method(it) });
            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_math + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::Add<#rhs_ty> for #ty
                #rhs_where
                {
                    type Output = #output_ty;

                    #[inline(always)]
                    fn add(
                        self,
                        rhs: #rhs_ty,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return #result;
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.math_sub && global_operators {
            let flags = [
                Konsts::Sub,
                Konsts::Math,
            ];
            let is_konst = self.cfg.konst_for(&flags);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Sub, is_konst);
            let bounds = bounds_for(&flags);
            if with_self_math {
                let output_ty = output.ty();
                let result = output.convert(quote! { self.#method(#rhs_raw) });
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::Sub for #ty #where_clause {
                        type Output = #output_ty;

                        #[inline(always)]
                        fn sub(
                            self,
                            rhs: Self,
                        ) -> Self::Output {
                            return #result;
                        }
                    }
                });
            }

            let output_ty = output.ty();
            let result = output.convert(quote! { self.#method(it) });
            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_math + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::Sub<#rhs_ty> for #ty
                #rhs_where
                {
                    type Output = #output_ty;

                    #[inline(always)]
                    fn sub(
                        self,
                        rhs: #rhs_ty,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return #result;
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.math_mul && global_operators {
            let flags = [
                Konsts::Mul,
                Konsts::Math,
            ];
            let is_konst = self.cfg.konst_for(&flags);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Mul, is_konst);
            let bounds = bounds_for(&flags);
            if with_self_math {
                let output_ty = output.ty();
                let result = output.convert(quote! { self.#method(#rhs_raw) });
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::Mul for #ty #where_clause {
                        type Output = #output_ty;

                        #[inline(always)]
                        fn mul(
                            self,
                            rhs: Self,
                        ) -> Self::Output {
                            return #result;
                        }
                    }
                });
            }

            let output_ty = output.ty();
            let result = output.convert(quote! { self.#method(it) });
            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_math + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::Mul<#rhs_ty> for #ty
                #rhs_where
                {
                    type Output = #output_ty;

                    #[inline(always)]
                    fn mul(
                        self,
                        rhs: #rhs_ty,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return #result;
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.math_div && global_operators {
            let flags = [
                Konsts::Div,
                Konsts::Math,
            ];
            let is_konst = self.cfg.konst_for(&flags);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Div, is_konst);
            let bounds = bounds_for(&flags);
            if with_self_math {
                let output_ty = output.ty();
                let result = output.convert(quote! { self.#method(#rhs_raw) });
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::Div for #ty #where_clause {
                        type Output = #output_ty;

                        #[inline(always)]
                        fn div(
                            self,
                            rhs: Self,
                        ) -> Self::Output {
                            return #result;
                        }
                    }
                });
            }

            let output_ty = output.ty();
            let result = output.convert(quote! { self.#method(it) });
            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_math + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::Div<#rhs_ty> for #ty
                #rhs_where
                {
                    type Output = #output_ty;

                    #[inline(always)]
                    fn div(
                        self,
                        rhs: #rhs_ty,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return #result;
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.math_rem && global_operators {
            let flags = [
                Konsts::Rem,
                Konsts::Math,
            ];
            let is_konst = self.cfg.konst_for(&flags);
            let konst = zz_runner::konst(is_konst);
            let method =
                self.arithmetic_method_name(ArithmeticOp::Rem, is_konst);
            let bounds = bounds_for(&flags);
            if with_self_math {
                let output_ty = output.ty();
                let result = output.convert(quote! { self.#method(#rhs_raw) });
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::Rem for #ty #where_clause {
                        type Output = #output_ty;

                        #[inline(always)]
                        fn rem(
                            self,
                            rhs: Self,
                        ) -> Self::Output {
                            return #result;
                        }
                    }
                });
            }

            let output_ty = output.ty();
            let result = output.convert(quote! { self.#method(it) });
            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_math + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::Rem<#rhs_ty> for #ty
                #rhs_where
                {
                    type Output = #output_ty;

                    #[inline(always)]
                    fn rem(
                        self,
                        rhs: #rhs_ty,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return #result;
                    }
                }
            };

            stream.extend(it);
        }

        if self.cfg.flags.math_and && global_operators {
            let flags = [
                Konsts::And,
                Konsts::Bit,
            ];
            let konst = zz_runner::konst(self.cfg.konst_for(&flags));
            let bounds = bounds_for(&flags);
            if with_self_bit {
                let output_ty = output.ty();
                let result = output.convert(quote! { self._bitand(#rhs_raw) });
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::BitAnd for #ty #where_clause {
                        type Output = #output_ty;

                        #[inline(always)]
                        fn bitand(
                            self,
                            rhs: Self,
                        ) -> Self::Output {
                            return #result;
                        }
                    }
                });
            }

            let output_ty = output.ty();
            let result = output.convert(quote! { self._bitand(it) });
            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_bit + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::BitAnd<#rhs_ty> for #ty
                #rhs_where
                {
                    type Output = #output_ty;

                    #[inline(always)]
                    fn bitand(
                        self,
                        rhs: #rhs_ty,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return #result;
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.math_or && global_operators {
            let flags = [
                Konsts::Or,
                Konsts::Bit,
            ];
            let konst = zz_runner::konst(self.cfg.konst_for(&flags));
            let bounds = bounds_for(&flags);
            if with_self_bit {
                let output_ty = output.ty();
                let result = output.convert(quote! { self._bitor(#rhs_raw) });
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::BitOr for #ty #where_clause {
                        type Output = #output_ty;

                        #[inline(always)]
                        fn bitor(
                            self,
                            rhs: Self,
                        ) -> Self::Output {
                            return #result;
                        }
                    }
                });
            }

            let output_ty = output.ty();
            let result = output.convert(quote! { self._bitor(it) });
            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_bit + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::BitOr<#rhs_ty> for #ty
                #rhs_where
                {
                    type Output = #output_ty;

                    #[inline(always)]
                    fn bitor(
                        self,
                        rhs: #rhs_ty,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return #result;
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.math_xor && global_operators {
            let flags = [
                Konsts::Xor,
                Konsts::Bit,
            ];
            let konst = zz_runner::konst(self.cfg.konst_for(&flags));
            let bounds = bounds_for(&flags);
            let assign_flags = [
                Konsts::XorAssign,
                Konsts::Xor,
                Konsts::Assign,
                Konsts::Bit,
            ];
            let konst_assign =
                zz_runner::konst(self.cfg.konst_for(&assign_flags));
            let assign_bounds = bounds_for(&assign_flags);
            if with_self_bit {
                let output_ty = output.ty();
                let result = output.convert(quote! { self._bitxor(#rhs_raw) });
                stream.extend(quote! {
                    #konst impl #impl_generics ::core::ops::BitXor for #ty #where_clause {
                        type Output = #output_ty;

                        #[inline(always)]
                        fn bitxor(
                            self,
                            rhs: Self,
                        ) -> Self::Output {
                            return #result;
                        }
                    }
                });

                if self.cfg.ty_out.is_self() {
                    stream.extend(quote! {
                        #konst_assign impl #impl_generics ::core::ops::BitXorAssign for #ty
                        #where_clause {
                            #[inline(always)]
                            fn bitxor_assign(
                                &mut self,
                                rhs: Self,
                            ) {
                                *self = self._bitxor(#rhs_raw);
                            }
                        }
                    });
                }
            }

            let output_ty = output.ty();
            let result = output.convert(quote! { self._bitxor(it) });
            let rhs_where =
                rhs_where_clause(quote! { #trait_friend_bit + #bounds });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::ops::BitXor<#rhs_ty> for #ty
                #rhs_where
                {
                    type Output = #output_ty;

                    #[inline(always)]
                    fn bitxor(
                        self,
                        rhs: #rhs_ty,
                    ) -> Self::Output {
                        let it = #fp_friend_conv(rhs);
                        return #result;
                    }
                }

            };
            stream.extend(it);

            if self.cfg.ty_out.is_self() {
                let rhs_where = rhs_where_clause(
                    quote! { #trait_friend_bit + #assign_bounds },
                );
                let it = quote! {
                    #konst_assign impl #rhs_impl_generics ::core::ops::BitXorAssign<#rhs_ty> for #ty
                    #rhs_where
                    {
                        #[inline(always)]
                        fn bitxor_assign(
                            &mut self,
                            rhs: #rhs_ty,
                        ) {
                            let it = #fp_friend_conv(rhs);
                            *self = self._bitxor(it);
                        }
                    }
                };
                stream.extend(it);
            }
        }

        if self.cfg.flags.math_not {
            let konst = zz_runner::konst(self.cfg.konst_for(&[
                Konsts::Not,
                Konsts::Bit,
            ]));
            let output_ty = output.ty();
            let result = output.convert(quote! { self._not() });
            let it = quote! {
                #konst impl #impl_generics ::core::ops::Not for #ty #where_clause {
                    type Output = #output_ty;

                    #[inline(always)]
                    fn not(self) -> Self::Output {
                        return #result;
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.display {
            stream.extend(quote! {
                impl #impl_generics ::core::fmt::Display for #ty #where_clause {
                    #[inline(always)]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let raw = #fp_get_raw(*self);
                        return ::core::fmt::Display::fmt(&raw, f);
                    }
                }
            });
        }

        if self.cfg.flags.fmt_binary {
            let it = quote! {
                impl #impl_generics ::core::fmt::Binary for #ty #where_clause {
                    #[inline(always)]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let raw = #fp_get_raw(*self);
                        return ::core::fmt::Binary::fmt(&raw, f);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fmt_octal {
            let it = quote! {
                impl #impl_generics ::core::fmt::Octal for #ty #where_clause {
                    #[inline(always)]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let raw = #fp_get_raw(*self);
                        return ::core::fmt::Octal::fmt(&raw, f);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fmt_hex_lower {
            let it = quote! {
                impl #impl_generics ::core::fmt::LowerHex for #ty #where_clause {
                    #[inline(always)]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let raw = #fp_get_raw(*self);
                        return ::core::fmt::LowerHex::fmt(&raw, f);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.fmt_hex_upper {
            let it = quote! {
                impl #impl_generics ::core::fmt::UpperHex for #ty #where_clause {
                    #[inline(always)]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        let raw = #fp_get_raw(*self);
                        return ::core::fmt::UpperHex::fmt(&raw, f);
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.partial_eq {
            let flags = [
                Konsts::PartialEq,
                Konsts::Comparison,
            ];
            let konst = zz_runner::konst(self.cfg.konst_for(&flags));
            let bounds = bounds_for(&flags);
            let rhs_where = rhs_where_clause(quote! {
                #trait_friend_rel + #bounds + ::core::marker::Copy
            });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::cmp::PartialEq<#rhs_ty> for #ty
                #rhs_where
                {
                    #[inline(always)]
                    fn eq(
                        &self,
                        rhs: &#rhs_ty,
                    ) -> bool {
                        let lhs = #fp_get_raw(*self);
                        let rhs = #fp_friend_conv(*rhs);
                        return lhs == rhs;
                    }
                }
            };
            stream.extend(it);
        }

        if self.cfg.flags.partial_ord {
            let flags = [
                Konsts::PartialOrd,
                Konsts::Comparison,
            ];
            let konst = zz_runner::konst(self.cfg.konst_for(&flags));
            let bounds = bounds_for(&flags);
            let rhs_where = rhs_where_clause(quote! {
                #trait_friend_rel + #bounds + ::core::marker::Copy
            });
            let it = quote! {
                #konst impl #rhs_impl_generics ::core::cmp::PartialOrd<#rhs_ty> for #ty
                #rhs_where
                {
                    #[inline(always)]
                    fn partial_cmp(
                        &self,
                        rhs: &#rhs_ty,
                    ) -> ::core::option::Option<::core::cmp::Ordering> {
                        let lhs = #fp_get_raw(*self);
                        let rhs = #fp_friend_conv(*rhs);
                        return ::core::cmp::PartialOrd::partial_cmp(&lhs, &rhs);
                    }
                }
            };
            stream.extend(it);
        }

        return Ok(stream);
    }

    pub(crate) fn ekran_bit_access_items(&self) -> TokenStream {
        let fp_get_raw = &self.fp_get_raw;
        let el = &self.el;

        if !self.cfg.flags.bit_access || self.repr.is_signed() {
            return TokenStream::new();
        }

        let bytes = (0..self.repr.bytes())
            .into_iter()
            .map(|i| {
                let name = format_ident!("byte{}", i);
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn #name(self) -> u8 {
                        return ((#fp_get_raw(self) >> (8 * #i)) & (0xFF as #el)) as u8;
                    }
                };
            })
            .merged();

        let words = (0..self.repr.words())
            .into_iter()
            .map(|i| {
                let name = format_ident!("word{}", i);
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn #name(self) -> u16 {
                        return ((#fp_get_raw(self) >> (16 * #i)) & (0xFFFF as #el)) as u16;
                    }
                };
            })
            .merged();

        let dwords = (0..self.repr.dwords())
            .into_iter()
            .map(|i| {
                let name = format_ident!("dword{}", i);
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn #name(self) -> u32 {
                        return ((#fp_get_raw(self) >> (32 * #i)) & (0xFFFFFFFF as #el)) as u32;
                    }
                };
            })
            .merged();

        let qwords = (0..self.repr.qwords())
            .into_iter()
            .map(|i| {
                let name = format_ident!("qword{}", i);
                return quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn #name(self) -> u64 {
                        return ((#fp_get_raw(self) >> (64 * #i)) & (0xFFFFFFFFFFFFFFFF as #el)) as u64;
                    }
                };
            })
            .merged();

        let halved = match self.repr {
            N::U016 => {
                quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn lo8(self) -> u8 {
                        return self.byte0();
                    }

                    #[must_use]
                    #[inline(always)]
                    pub const fn hi8(self) -> u8 {
                        return self.byte1();
                    }
                }
            }

            N::U032 => {
                quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn lo16(self) -> u16 {
                        return self.word0();
                    }

                    #[must_use]
                    #[inline(always)]
                    pub const fn hi16(self) -> u16 {
                        return self.word1();
                    }
                }
            }

            N::U064 => {
                quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn lo32(self) -> u32 {
                        return self.dword0();
                    }

                    #[must_use]
                    #[inline(always)]
                    pub const fn hi32(self) -> u32 {
                        return self.dword1();
                    }
                }
            }

            N::U128 => {
                quote! {
                    #[must_use]
                    #[inline(always)]
                    pub const fn lo64(self) -> u64 {
                        return self.qword0();
                    }

                    #[must_use]
                    #[inline(always)]
                    pub const fn hi64(self) -> u64 {
                        return self.qword1();
                    }
                }
            }

            // N::USIZ => {}
            _ => TokenStream::new(),
        };

        return quote! {
            #halved

            #bytes

            #words

            #dwords

            #qwords
        };
    }
}

// =============================================================================

#[derive(Clone, Default)]
pub(crate) struct IntSettings {
    pub(crate) flags: BTreeMap<String, (bool, Span)>,
    konst_flags: BTreeMap<Konsts, (bool, Span)>,
    seen: BTreeMap<String, bool>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum NonZeroPolicy {
    #[default]
    Infer,
    Require,
    Suppress,
}

#[derive(Default)]
pub(crate) struct Cfg {
    pub(crate) konst: Konst,
    konst_flags: BTreeMap<Konsts, (bool, Span)>,
    root_konst_flags: BTreeMap<Konsts, (bool, Span)>,
    root_flags: BTreeMap<String, (bool, Span)>,
    root_seen_flags: BTreeMap<String, bool>,
    pub(crate) get_raw: Option<Path>,
    pub(crate) validation: ValidationCfg,
    pub(crate) friends: BTreeSet<Friend>,
    pub(crate) flags: Box<IntegralFlags>,
    pub(crate) non_zero: NonZeroPolicy,
    wrapping_storage: bool,
    pub(crate) ty_out: SelfConverter,
    pub(crate) profile: Profiles,
    pub(crate) facets: Vec<FacetCfg>,
    pub(crate) default_facet: Option<DefaultFacetCfg>,
    pub(crate) facet_slots: BTreeMap<FacetSlotKey, FacetSlot>,
    facets_present: bool,
}

impl Cfg {
    #[allow(dead_code)]
    pub(crate) fn requires_non_zero_storage(&self) -> bool {
        return match self.non_zero {
            NonZeroPolicy::Infer => self.validation.domain_excludes_zero(),
            NonZeroPolicy::Require => true,
            NonZeroPolicy::Suppress => false,
        };
    }

    pub(crate) fn requires_wrapping_storage(&self) -> bool {
        return self.wrapping_storage;
    }

    pub(crate) fn facets_present(&self) -> bool {
        return self.facets_present;
    }

    pub(crate) fn parse_flags_into(
        input: ParseStream,
        value_if_seen: bool,
        settings: &mut IntSettings,
    ) -> syn::Result<()> {
        let attrs = zz::one_or_list::<Ident>(input)?;
        let mut duplicated = BTreeMap::new();

        for attr in attrs {
            let name = attr.to_string();
            let span = attr.span();

            if duplicated.insert(name.clone(), span).is_some() {
                return span.fail("duplicated flags");
            }

            if settings
                .seen
                .get(&name)
                .is_some_and(|value| *value != value_if_seen)
            {
                return span
                    .fail("flag specified in both `with` and `without`");
            }

            if name.starts_with("konst_") {
                let Some(flag) = Konsts::from_str(&name).ok()
                else {
                    return span.fail("unknown flag");
                };

                settings.konst_flags.insert(flag, (value_if_seen, span));
            }
            else if IntegralFlags::default()
                .set_bool(&name, value_if_seen)
                .is_err()
            {
                return span.fail("unknown flag");
            }
            else {
                settings.flags.insert(name.clone(), (value_if_seen, span));
            }

            settings.seen.insert(name.clone(), value_if_seen);
        }

        return Ok(());
    }

    fn parse_flags(
        &mut self,
        input: ParseStream,
        value_if_seen: bool,
    ) -> syn::Result<()> {
        let mut settings = IntSettings {
            flags: std::mem::take(&mut self.root_flags),
            konst_flags: std::mem::take(&mut self.root_konst_flags),
            seen: std::mem::take(&mut self.root_seen_flags),
        };
        let result =
            Self::parse_flags_into(input, value_if_seen, &mut settings);

        self.root_flags = settings.flags;
        self.root_konst_flags = settings.konst_flags;
        self.root_seen_flags = settings.seen;

        return result;
    }

    fn apply_flag_settings(
        flags: &mut IntegralFlags,
        settings: &BTreeMap<String, (bool, Span)>,
    ) {
        for (name, (value, _)) in settings {
            flags
                .set_bool(name, *value)
                .expect("integral configuration contains an unknown flag");
        }
    }

    fn validate_facet(it: &FacetCfg) -> syn::Result<()> {
        if let Some(span) = it.precedes_span {
            return span.fail(
                "`precedes` is wrapper-wide and cannot be configured inside a facet",
            );
        }

        for profile in it.profile.iter() {
            let settings = profile.int_settings();

            if let Some((_, span)) = settings.konst_flags.values().next() {
                return span.fail(
                    "constness is wrapper-wide and cannot be configured inside a facet",
                );
            }

            for (name, (enabled, span)) in &settings.flags {
                if name == "non_zero" {
                    return span.fail(
                        "`non_zero` is wrapper-wide and cannot be configured inside a facet",
                    );
                }

                if name.starts_with("assign_") {
                    if *enabled {
                        return span.fail(
                            "facets do not support assignment operators",
                        );
                    }
                    continue;
                }

                if !matches!(
                    name.as_str(),
                    "math_add"
                        | "math_sub"
                        | "math_mul"
                        | "math_div"
                        | "math_rem"
                        | "math_and"
                        | "math_or"
                        | "math_xor"
                        | "self_math"
                        | "self_bit"
                ) {
                    return span.fail(
                        "attribute is wrapper-wide or has no operand \
                        slot and cannot be configured inside a facet",
                    );
                }
            }
        }

        for friend in &it.friends {
            for capability in &friend.capabilities {
                if !matches!(capability.to_string().as_str(), "Math" | "Bit") {
                    return capability.fail(
                        "facet friends only support `Math` and `Bit` capabilities",
                    );
                }
            }
        }

        return Ok(());
    }

    fn resolved_facet_flags(
        baseline: &IntegralFlags,
        facet: &FacetCfg,
    ) -> IntegralFlags {
        let mut flags = baseline.clone();
        for profile in facet.profile.iter() {
            profile.apply(&mut flags);
            Self::apply_flag_settings(
                &mut flags,
                &profile.int_settings().flags,
            );
        }
        return flags;
    }

    fn insert_explicit_slot(
        slots: &mut BTreeMap<FacetSlotKey, FacetSlot>,
        key: FacetSlotKey,
        slot: FacetSlot,
    ) -> syn::Result<()> {
        if let Some(previous) = slots.get(&key) {
            return slot.fail_combi(
                "duplicate explicit facet slot",
                previous.span,
                "the slot was first claimed here",
            );
        }

        slots.insert(key, slot);
        return Ok(());
    }

    #[allow(clippy::too_many_arguments)]
    fn insert_slots(
        slots: &mut BTreeMap<FacetSlotKey, FacetSlot>,
        operations: &[Operator],
        flags: &IntegralFlags,
        rhs: Option<&Path>,
        conversion: Option<&Path>,
        arithmetic: Arithmetic,
        ty_out: &SelfConverter,
        owner: FacetOwner,
        span: Span,
    ) -> syn::Result<()> {
        let identity = rhs.map_or_else(
            || "Self".to_string(),
            |rhs| rhs.to_token_stream().to_string(),
        );

        for operator in operations
            .iter()
            .copied()
            .filter(|operator| flags.enabled(*operator))
        {
            let key = FacetSlotKey {
                operator,
                rhs: identity.clone(),
            };
            let slot = FacetSlot {
                rhs: rhs.cloned(),
                conversion: conversion.cloned(),
                arithmetic,
                ty_out: ty_out.clone(),
                owner,
                span,
            };

            if matches!(owner, FacetOwner::Explicit(_)) {
                Self::insert_explicit_slot(slots, key, slot)?;
            }
            else {
                slots.entry(key).or_insert(slot);
            }
        }

        return Ok(());
    }

    fn resolve_facets(&mut self) -> syn::Result<()> {
        self.facet_slots.clear();
        if !self.facets_present() {
            return Ok(());
        }

        let default = self
            .default_facet
            .as_ref()
            .expect("facets require a resolved default facet");
        let mut slots = BTreeMap::new();

        for (index, facet) in self.facets.iter().enumerate() {
            Self::validate_facet(facet)?;

            let flags = Self::resolved_facet_flags(&default.flags, facet);
            let arithmetic =
                facet.profile.arithmetic_or(default.profile.arithmetic());
            let ty_out = facet.ty_out.as_ref().unwrap_or(&default.ty_out);
            let owner = FacetOwner::Explicit(index);

            for friend in &facet.friends {
                let span = friend.ty.span();
                if friend.capabilities.contains(&format_ident!("Math")) {
                    Self::insert_slots(
                        &mut slots,
                        Operator::math_items(),
                        &flags,
                        Some(&friend.ty),
                        friend.conv.as_ref(),
                        arithmetic,
                        ty_out,
                        owner,
                        span,
                    )?;
                }
                if friend.capabilities.contains(&format_ident!("Bit")) {
                    Self::insert_slots(
                        &mut slots,
                        Operator::bit_items(),
                        &flags,
                        Some(&friend.ty),
                        friend.conv.as_ref(),
                        arithmetic,
                        ty_out,
                        owner,
                        span,
                    )?;
                }
            }

            if let Some((true, span)) = facet.explicit_flag("self_math") {
                let conversion: Path = parse_quote! { Self::raw };
                Self::insert_slots(
                    &mut slots,
                    Operator::math_items(),
                    &flags,
                    None,
                    Some(&conversion),
                    arithmetic,
                    ty_out,
                    owner,
                    span,
                )?;
            }

            if let Some((true, span)) = facet.explicit_flag("self_bit") {
                let conversion: Path = parse_quote! { Self::raw };
                Self::insert_slots(
                    &mut slots,
                    Operator::bit_items(),
                    &flags,
                    None,
                    Some(&conversion),
                    arithmetic,
                    ty_out,
                    owner,
                    span,
                )?;
            }
        }

        let arithmetic = default.profile.arithmetic();
        for friend in &default.friends {
            let span = friend.ty.span();
            if friend.capabilities.contains(&format_ident!("Math")) {
                Self::insert_slots(
                    &mut slots,
                    Operator::math_items(),
                    &default.flags,
                    Some(&friend.ty),
                    friend.conv.as_ref(),
                    arithmetic,
                    &default.ty_out,
                    FacetOwner::Default,
                    span,
                )?;
            }
            if friend.capabilities.contains(&format_ident!("Bit")) {
                Self::insert_slots(
                    &mut slots,
                    Operator::bit_items(),
                    &default.flags,
                    Some(&friend.ty),
                    friend.conv.as_ref(),
                    arithmetic,
                    &default.ty_out,
                    FacetOwner::Default,
                    span,
                )?;
            }
        }

        let conversion: Path = parse_quote! { Self::raw };
        if default.flags.self_math {
            Self::insert_slots(
                &mut slots,
                Operator::math_items(),
                &default.flags,
                None,
                Some(&conversion),
                arithmetic,
                &default.ty_out,
                FacetOwner::Default,
                Span::call_site(),
            )?;
        }
        if default.flags.self_bit {
            Self::insert_slots(
                &mut slots,
                Operator::bit_items(),
                &default.flags,
                None,
                Some(&conversion),
                arithmetic,
                &default.ty_out,
                FacetOwner::Default,
                Span::call_site(),
            )?;
        }

        self.facet_slots = slots;
        return Ok(());
    }

    fn explicit_flag(
        &self,
        name: &str,
    ) -> Option<(bool, Span)> {
        if let Some(setting) = self.root_flags.get(name) {
            return Some(*setting);
        }

        let mut setting = None;
        for profile in self.profile.iter() {
            if profile.resets_flag(name) {
                setting = None;
            }
            if let Some(value) = profile.int_settings().flags.get(name) {
                setting = Some(*value);
            }
        }

        return setting;
    }

    pub(crate) fn resolve(&mut self) -> syn::Result<()> {
        self.non_zero = match self.explicit_flag("non_zero") {
            Some((true, _)) => NonZeroPolicy::Require,
            Some((false, _)) => NonZeroPolicy::Suppress,
            None => NonZeroPolicy::Infer,
        };
        self.validation
            .require_non_zero(self.non_zero == NonZeroPolicy::Require);

        *self.flags = IntegralFlags::default();
        self.konst_flags.clear();
        for profile in self.profile.iter() {
            profile.apply(&mut self.flags);
            Self::apply_flag_settings(
                &mut self.flags,
                &profile.int_settings().flags,
            );
            self.konst_flags
                .extend(profile.int_settings().konst_flags.clone());
        }
        Self::apply_flag_settings(&mut self.flags, &self.root_flags);
        self.konst_flags.extend(self.root_konst_flags.clone());

        if self.profile.is_circular() {
            for name in [
                "ord",
                "partial_ord",
            ] {
                if let Some((true, span)) = self.explicit_flag(name) {
                    return span.fail(
                        "circular profiles do not support linear ordering",
                    );
                }
            }
            self.flags.ord = false;
            self.flags.partial_ord = false;
        }

        if !self.ty_out.is_self() {
            for name in [
                "assign_add",
                "assign_and",
                "assign_div",
                "assign_mul",
                "assign_or",
                "assign_rem",
                "assign_shl",
                "assign_shr",
                "assign_sub",
            ] {
                if let Some((true, span)) = self.explicit_flag(name) {
                    return span
                        .fail("out_type cannot generate assignment operators");
                }
                self.flags
                    .set_bool(name, false)
                    .expect("integral configuration contains an unknown flag");
            }
        }

        self.default_facet = self.facets_present().then(|| DefaultFacetCfg {
            profile: self.profile.clone(),
            friends: self.friends.clone(),
            flags: self.flags.clone(),
            ty_out: self.ty_out.clone(),
        });
        self.resolve_facets()?;
        self.wrapping_storage = self.profile.requires_wrapping_storage()
            || self.facets.iter().any(FacetCfg::requires_wrapping_storage);

        return Ok(());
    }

    pub(crate) fn konst_for(
        &self,
        flags: &[Konsts],
    ) -> bool {
        for it in flags {
            if let Some((value, _)) = self.konst_flags.get(it) {
                return *value;
            }
        }

        return self.konst.enables(flags);
    }

    pub(crate) fn explicit_konst_flag(
        &self,
        flag: Konsts,
    ) -> Option<(bool, Span)> {
        return self.konst_flags.get(&flag).copied();
    }

    pub(crate) fn set_konst_flag(
        &mut self,
        flag: Konsts,
        value: bool,
    ) {
        self.konst_flags.insert(flag, (value, Span::call_site()));
    }

    pub(crate) fn konst_friendship(&self) -> bool {
        if self.konst_for(&[Konsts::Friendship]) {
            return true;
        }

        return Konsts::INTEGRAL
            .iter()
            .filter(|flag| **flag != Konsts::Friendship && !flag.is_stable())
            .any(|flag| self.konst_for(&[*flag]));
    }

    pub(crate) fn parse_friends_unexpanded(
        input: ParseStream
    ) -> syn::Result<BTreeSet<Friend>> {
        let defaults = [format_ident!("Math")].into_iter().collect();
        return Friend::parse_with_valid(
            input,
            &[
                format_ident!("Make"),
                format_ident!("Math"),
                format_ident!("Bit"),
                format_ident!("Rel"),
                format_ident!("UnsafeTrust"),
            ],
            &defaults,
        );
    }

    fn parse_friends(input: ParseStream) -> syn::Result<BTreeSet<Friend>> {
        let defaults = [format_ident!("Math")].into_iter().collect();
        return Friend::parse_with_expansion(
            input,
            &[format_ident!("Make"),
                format_ident!("Math"),
                format_ident!("Bit"),
                format_ident!("Rel"),
                format_ident!("UnsafeTrust")],
            &defaults,
            &([
                (
                    format_ident!("Math"),
                    vec![
                        format_ident!("Make"),
                        format_ident!("Bit"),
                    ],
                ),
            ]
            .into_iter()
            .collect()),
        );
    }

    pub(crate) fn add_friend(
        &mut self,
        ty: &Ident,
        capabilities: impl IntoIterator<Item = Ident>,
        conv: Path,
    ) {
        let req = Friend::from(ty);
        let mut req = self.friends.take(&req).unwrap_or(req);
        req.capabilities.extend(capabilities);
        req.conv = Some(conv);
        assert!(self.friends.insert(req));
    }

    fn parse_attrs(
        &mut self,
        input: ParseStream,
        allow_konst: bool,
    ) -> syn::Result<()> {
        return zz::parse_inner(input, |attr, rest| {
            if self.validation.parse_attr(attr, rest)? {
                return Ok(true);
            }

            match attr {
                "konst" if allow_konst => self.konst = rest.parse()?,
                "konst" => {
                    return attr
                        .fail("constness is specified in multiple places");
                }
                "error" => zz::parse_unit_error(rest)?,
                "with" => self.parse_flags(rest, true)?,
                "without" => self.parse_flags(rest, false)?,
                "friend" => self.friends = Self::parse_friends(rest)?,
                "facet" => {
                    self.facets = zz::one_or_list::<FacetCfg>(rest)?.collect();
                    self.facets_present = true;
                }
                "get_raw" => self.get_raw = Some(rest.parse()?),
                "out_type" => self.ty_out = rest.parse()?,
                "profile" => self.profile = rest.parse()?,
                _ => return Ok(false),
            };

            return Ok(true);
        });
    }

    pub(crate) fn parse_with_konst_unresolved(
        input: ParseStream
    ) -> syn::Result<Self> {
        let mut this = Self::without_konst();
        this.parse_attrs(input, false)?;
        return Ok(this);
    }

    pub(crate) fn without_konst() -> Self {
        let mut this = Self::default();
        this.konst = Konst::None;
        return this;
    }
}

impl Debug for Cfg {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return f
            .debug_struct("IntegralCfg")
            .field("konst", &self.konst)
            .field("get_raw", &self.get_raw.to_token_stream().to_string())
            .field("validation", &self.validation)
            .field("friends", &self.friends)
            .field("non_zero", &self.non_zero)
            .field("ty_out", &!self.ty_out.is_self())
            .field("profile", &self.profile.arithmetic())
            .field("facet_slots", &self.facet_slots)
            .field("flags", &self.flags)
            .finish_non_exhaustive();
    }
}

impl Parse for Cfg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();
        this.parse_attrs(input, true)?;
        this.resolve()?;
        return Ok(this);
    }
}

// =============================================================================

impl IntegralFlags {
    const RESTRICTED_MATH_FLAGS: [&str; 22] = [
        "self_friend_math_ops",
        "self_friend_math_bit",
        "self_friend_rel",
        "self_bit",
        "bit_access",
        "core_int",
        "math_and",
        "math_div",
        "math_mul",
        "math_not",
        "math_or",
        "math_rem",
        "math_shl",
        "math_shr",
        "math_xor",
        "assign_and",
        "assign_div",
        "assign_mul",
        "assign_or",
        "assign_rem",
        "assign_shl",
        "assign_shr",
    ];

    pub(crate) fn restricted_math_flags() -> &'static [&'static str] {
        return &Self::RESTRICTED_MATH_FLAGS;
    }

    pub(crate) fn enabled(
        &self,
        it: Operator,
    ) -> bool {
        return match it {
            Operator::Add => self.math_add,
            Operator::Sub => self.math_sub,
            Operator::Mul => self.math_mul,
            Operator::Div => self.math_div,
            Operator::Rem => self.math_rem,
            Operator::BitAnd => self.math_and,
            Operator::BitOr => self.math_or,
            Operator::BitXor => self.math_xor,
        };
    }
}

mk_flags! {
    #[flag_default(bool=true, str="")]
    #[derive(Debug, Clone)]
    pub(crate) struct IntegralFlags {
        pub range: bool = false,
        pub as_ref_: bool,

        pub auto_of_raw: bool,
        pub assertions: bool,
        pub bit_access: bool,
        pub clone: bool,
        pub copy: bool,

        pub debug: bool,
        pub display: bool = false,
        pub hash: bool,
        pub eq: bool,
        pub fmt_binary: bool,
        pub fmt_hex_lower: bool,
        pub fmt_hex_upper: bool,
        pub fmt_octal: bool,
        pub from_str: bool,
        pub core_int: bool,
        pub into: bool,
        pub map: bool,
        pub ord: bool,
        pub partial_eq: bool,
        pub partial_ord: bool,
        pub repr: bool,
        pub serde: bool = false,
        pub try_into: bool,
        pub with_const_map_fn: bool = false,
        pub non_zero: bool = false,

        pub assign_add: bool,
        pub assign_and: bool,
        pub assign_div: bool,
        pub assign_mul: bool,
        pub assign_or: bool,
        pub assign_rem: bool,
        pub assign_shl: bool,
        pub assign_shr: bool,
        pub assign_sub: bool,
        pub math_add: bool,
        pub math_and: bool,
        pub math_div: bool,
        pub math_mul: bool,
        pub math_not: bool,
        pub math_or: bool,
        pub math_rem: bool,
        pub math_shl: bool,
        pub math_shr: bool,
        pub math_sub: bool,
        pub math_xor: bool,

        pub friend: bool,
        pub self_friend_make: bool,
        pub self_friend_math_bit: bool,
        pub self_friend_math_ops: bool,
        pub self_friend_rel: bool,
        pub friend_seal: bool,
        pub friendzone_friend_make: bool,
        pub friendzone_friend_math_bit: bool,
        pub friendzone_friend_math_ops: bool,
        pub friendzone_friend_rel: bool,
        pub friendzone_seal: bool,

        pub self_math: bool,
        pub self_bit: bool,

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

        pub fn_make: String = "make",

        pub fp_unchecked: String = "Self::_unchecked",
    }
}
