//! `Within` friends of integral targets, and the constants every integral
//! source publishes for them (`zz_domain_within`).
//!
//! A source publishes its raw type, its form per pointer width (or why it
//! has none), and for 8-bit raw types the table of its validation. A target
//! proves each wrapper friend in an associated constant that rustc
//! evaluates: the table against the target's own validation when both are
//! const-callable, else the source's form against the target's with the
//! shared check. Primitive sources are decided here, at expansion.

use proc_macro2::{
    Literal,
    TokenStream,
};
use quote::{
    ToTokens,
    quote,
};
use syn::ext::IdentExt as _;
use syn::parse_quote;

use crate::integral::Maker;
use crate::integral_cfg::{
    IntegralFlag,
    IntegralName,
};
use crate::zz_domain_within::{
    Form,
    Opaque,
    emit_check,
    emit_messages,
    subset,
};
use crate::zz_domain_within_const::{
    __TYPEKIN_WITHIN_BUDGET,
    __TypekinWithin,
    __TypekinWithinText,
};
use crate::zz_friend::{
    Capability,
    Friend,
};
use crate::zz_lib::{
    Check,
    Konst,
    Konsts,
    Named as _,
    OnInvalid,
};
use crate::zz_runner::MkErr;
use crate::zz_values::{
    Int,
    N,
    Num,
    Width,
    by_width,
};

/// How a `Within` friend is proven.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WithinProof {
    /// Typekin proved a primitive source at expansion.
    Expanded,
    /// rustc evaluates the friend's proof constant at compile time, from an
    /// 8-bit primitive's table or from a wrapper's published constants.
    Compiled { index: usize, primitive: Option<N> },
}

/// One pointer width's verdict on a primitive source.
#[derive(Clone, Eq, PartialEq)]
enum Verdict {
    Proven,
    Refuted(String),
    Unproven(String),
}

impl Maker {
    // -------------------------------------------------------------------------
    // Source side

    /// The constants a `Within` target reads, and the raw accessor its
    /// conversion calls.
    pub(crate) fn make_within_source(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::WithinSource) {
            return TokenStream::new();
        }

        let el = &self.el;
        let names = &self.ctx.names;
        let raw = names.get(IntegralName::WithinRaw);
        let form = names.get(IntegralName::WithinForm);
        let why = names.get(IntegralName::WithinWhy);
        let table = names.get(IntegralName::WithinTable);
        let signed = self.repr().is_signed();
        let ty = self.target.ident.unraw().to_string();
        let forms = Width::ALL.map(|width| self.source_form(width));
        let form_values = forms.clone().map(|it| {
            return match it {
                Ok(form) => {
                    let form = form.tokens();
                    quote! { ::core::option::Option::Some(#form) }
                }
                Err(_) => quote! { ::core::option::Option::None },
            };
        });
        let why_values = forms.map(|it| {
            let why = it.err().map(|it| it.describe(&ty)).unwrap_or_default();
            return why.into_token_stream();
        });
        let form = per_width_const(
            &form,
            &quote! {
                ::core::option::Option<(
                    u128,
                    &'static [(u128, bool, u128, bool, u128)],
                )>
            },
            form_values,
        );
        let why = per_width_const(&why, &quote! { &'static str }, why_values);
        let table_value = self.within_table();
        let raw_fn = self.within_raw_fn();

        return quote! {
            #[doc(hidden)]
            pub const #raw: (bool, u32) = (#signed, #el::BITS);
            #form
            #why
            #[doc(hidden)]
            pub const #table: ::core::option::Option<[bool; 256]> =
                #table_value;
            #raw_fn
        };
    }

    /// The raw value a target's conversion reads, when the wrapper has a
    /// raw getter.
    fn within_raw_fn(&self) -> Option<TokenStream> {
        let cfg = &self.ctx.cfg;
        if !cfg.flags.has(IntegralFlag::FnConvRaw) && cfg.get_raw.is_none() {
            return None;
        }
        let el = &self.el;
        let raw = &self.ctx.self_raw;
        let name = self.ctx.names.get(IntegralName::WithinRawFn);
        return Some(quote! {
            #[doc(hidden)]
            #[inline(always)]
            pub const fn #name(self) -> #el {
                return #raw;
            }
        });
    }

    /// The values the wrapper can hold at `width`: every raw value when its
    /// value field is visible, else what validation admits.
    fn source_form(
        &self,
        width: Width,
    ) -> Result<Form, Opaque> {
        let repr = self.repr();
        if self.ctx.cfg.visible_value {
            return Ok(Form::full(repr, width));
        }
        return self.ctx.cfg.validation.form(repr, width);
    }

    /// An 8-bit wrapper's validation at every raw value, when it is
    /// const-callable: entry `i` holds raw `i` (`u8`) or `i - 128` (`i8`).
    fn within_table(&self) -> TokenStream {
        let cfg = &self.ctx.cfg;
        let decode = match self.repr() {
            N::U008 => quote! { index as u8 },
            N::I008 => quote! { (index as i16 - 128) as i8 },
            _ => return quote! { ::core::option::Option::None },
        };
        if cfg.visible_value || !cfg.validation.has_validation() {
            return quote! { ::core::option::Option::Some([true; 256]) };
        }
        if !self.validation_konst() {
            return quote! { ::core::option::Option::None };
        }
        let el = &self.el;
        let validation = &self.ctx.validation;
        return quote! {
            {
                let mut table = [false; 256];
                let mut index = 0;
                while index < 256 {
                    let value: #el = #decode;
                    table[index] = #validation;
                    index += 1;
                }
                ::core::option::Option::Some(table)
            }
        };
    }

    /// Whether validation runs in const items: a domain alone always does,
    /// and callbacks do when a generated `const fn` (`try_make`,
    /// `_or_panic`) already runs them, so rustc already requires them to be
    /// const.
    pub(crate) fn validation_konst(&self) -> bool {
        let cfg = &self.ctx.cfg;
        let try_make = cfg.flags.has(IntegralFlag::FnTryMake)
            && cfg.konst_for(&[Konsts::TryMake]);
        let or_panic =
            cfg.flags.has(IntegralFlag::FnOrPanic) && cfg.helper_konst();
        return cfg.validation.callbacks.is_empty() || try_make || or_panic;
    }

    // -------------------------------------------------------------------------
    // Target side: planning

    /// Every `Within` friend and its proof: primitive sources are decided
    /// now, and a refuted or unprovable one fails at its span.
    pub(crate) fn plan_within(
        &self
    ) -> syn::Result<Vec<(Friend, WithinProof)>> {
        let mut plan = Vec::new();
        let mut compiled = 0;
        for friend in &self.ctx.cfg.friends {
            if !friend.has(Capability::Within) {
                continue;
            }
            let path = friend.ty.path();
            if self.target.is_target(path) {
                return path.fail(
                    "a type is always within itself; remove this friend",
                );
            }
            let primitive =
                path.get_ident().and_then(|it| Num::of(it.to_string()));
            let proof = match primitive {
                Some(Num::Float(_)) => {
                    return path.fail("`Within` supports integer sources only");
                }
                Some(Num::Int(source)) => {
                    match self.decide_primitive(friend, source)? {
                        true => WithinProof::Expanded,
                        false => WithinProof::Compiled {
                            index: compiled,
                            primitive: Some(source),
                        },
                    }
                }
                None => WithinProof::Compiled {
                    index: compiled,
                    primitive: None,
                },
            };
            if let WithinProof::Compiled { .. } = proof {
                compiled += 1;
            }
            plan.push((friend.clone(), proof));
        }
        return Ok(plan);
    }

    /// The target's validated values at `width`.
    fn target_form(
        &self,
        width: Width,
    ) -> Result<Form, Opaque> {
        return self.ctx.cfg.validation.form(self.repr(), width);
    }

    /// The target's raw range at `width`.
    fn target_bounds(
        &self,
        width: Width,
    ) -> (Int, Int) {
        return self.repr().bounds_for_pointer_width(width);
    }

    /// A primitive source at every width: `true` when proven, `false` when
    /// rustc proves it from the table (an 8-bit source, a target opaque at
    /// some width), an error when refuted at some width or unprovable.
    fn decide_primitive(
        &self,
        friend: &Friend,
        source: N,
    ) -> syn::Result<bool> {
        let verdicts =
            Width::ALL.map(|width| self.primitive_verdict(source, width));
        if verdicts.iter().all(|it| *it == Verdict::Proven) {
            return Ok(true);
        }
        let refuted = verdicts
            .iter()
            .position(|it| matches!(it, Verdict::Refuted(_)));
        let tabled =
            matches!(source, N::U008 | N::I008) && self.validation_konst();
        let position = match (refuted, tabled) {
            (Some(position), _) => position,
            (None, true) => return Ok(false),
            (None, false) => verdicts
                .iter()
                .position(|it| *it != Verdict::Proven)
                .expect("a verdict that is not proven"),
        };
        let (Verdict::Refuted(text) | Verdict::Unproven(text)) =
            &verdicts[position]
        else {
            unreachable!("a failing verdict");
        };
        let message = match verdicts.iter().all(|it| *it == verdicts[position])
        {
            true => text.clone(),
            false => {
                format!("{text} at {}-bit pointer width", Width::ALL[position])
            }
        };
        return friend.ty.path().fail(message);
    }

    /// Whether every value of primitive `source` lies in the target at
    /// `width`.
    fn primitive_verdict(
        &self,
        source: N,
        width: Width,
    ) -> Verdict {
        let source_name = source.name();
        let ty = self.target.ident.unraw();
        let refuted = |value: Int, tail: String| {
            let head = format!("`{source_name}` is not within `{ty}`: ");
            return Verdict::Refuted(message(&head, value, &tail));
        };
        let cannot = format!("cannot prove `{source_name}` within `{ty}`: ");
        let target = match self.target_form(width) {
            Ok(it) => it,
            Err(reason) => {
                let reason = reason.describe(&ty.to_string());
                return Verdict::Unproven(format!("{cannot}{reason}"));
            }
        };
        let value = Int::from_sign_magnitude;
        let full = Form::full(source, width);
        return match subset(&full, &target, self.target_bounds(width)) {
            __TypekinWithin::Proven => Verdict::Proven,
            __TypekinWithin::Range(negative, magnitude) => refuted(
                value(negative, magnitude),
                format!(" does not fit `{}`", self.el),
            ),
            __TypekinWithin::Domain(negative, magnitude) => refuted(
                value(negative, magnitude),
                format!(
                    " is outside the domain `{}`",
                    self.ctx.cfg.validation.domain_text()
                ),
            ),
            __TypekinWithin::Budget(steps) => Verdict::Unproven(format!(
                "{cannot}the check needs {steps} steps, above the subdomain \
                proof budget of {__TYPEKIN_WITHIN_BUDGET}"
            )),
        };
    }

    /// The proof of `friend`, a `Within` friend.
    pub(crate) fn within_proof(
        &self,
        friend: &Friend,
    ) -> WithinProof {
        return self
            .within
            .iter()
            .find(|(it, _)| it == friend)
            .map(|(_, proof)| *proof)
            .expect("`Within` friends are planned");
    }

    // -------------------------------------------------------------------------
    // Target side: emission

    /// The constructor of proven sources (`_within`): no validation, which a
    /// `debug_assert!` still checks. Construction that reports failure
    /// validates every source instead.
    pub(crate) fn make_within(&self) -> TokenStream {
        if self.within.is_empty() || self.ctx.failure.is_some() {
            return TokenStream::new();
        }

        let el = &self.el;
        let name = self.ctx.names.get(IntegralName::WithinFn);
        let konst = Konst::konst(self.ctx.cfg.helper_konst());
        let violated = "typekin within proof violated";
        let body = self.construction(
            quote! { raw },
            Check::DebugOnly(violated),
            OnInvalid::Panic(violated),
        );
        return quote! {
            /// Builds a value from a `Within` friend, whose every value
            /// lies in the domain (proven at compile time); debug builds
            /// still check it.
            #[doc(hidden)]
            #[must_use]
            #[inline(always)]
            pub(self) #konst fn #name(raw: #el) -> Self {
                #body
            }
        };
    }

    /// The seal conversion of a `Within` friend: its raw value, cast
    /// losslessly once proven. A generic target's proof runs where the
    /// conversion is instantiated.
    pub(crate) fn within_conversion(
        &self,
        friend: &Friend,
    ) -> TokenStream {
        let el = &self.el;
        let path = friend.ty.path();
        let value =
            match path.get_ident().and_then(|it| Num::of(it.to_string())) {
                Some(_) => quote! { self as #el },
                None => {
                    let source =
                        self.target.external_type(&parse_quote! { #path });
                    let raw = self.ctx.names.get(IntegralName::WithinRawFn);
                    quote! { <#source>::#raw(self) as #el }
                }
            };
        let force = match self.within_proof(friend) {
            WithinProof::Compiled { index, .. }
                if self.target.has_generics() =>
            {
                let target = self.target.ty();
                let proof =
                    self.ctx.names.get(IntegralName::WithinProof(index));
                Some(quote! { let () = <#target>::#proof; })
            }
            _ => None,
        };
        return quote! {
            #force
            return #value;
        };
    }

    /// The doc line on a `Within` friend's `make`.
    pub(crate) fn within_doc(
        &self,
        friend: &Friend,
    ) -> TokenStream {
        let source = spelled(friend);
        let ty = self.target.ident.unraw();
        let by = match self.within_proof(friend) {
            WithinProof::Expanded => "typekin at expansion",
            WithinProof::Compiled { .. } => "rustc at compile time",
        };
        let doc = format!(
            " Cannot fail validation: every value of `{source}` lies in the \
            domain of `{ty}` (proven by {by})."
        );
        return quote! { #[doc = #doc] };
    }

    /// The proof constants, one per friend rustc proves.
    pub(crate) fn make_within_proofs(&self) -> TokenStream {
        return self
            .within
            .iter()
            .filter_map(|(friend, proof)| {
                let WithinProof::Compiled { index, primitive } = proof
                else {
                    return None;
                };
                let name =
                    self.ctx.names.get(IntegralName::WithinProof(*index));
                let body = self.within_proof_body(friend, *primitive);
                return Some(quote! {
                    #[doc(hidden)]
                    const #name: () = #body;
                });
            })
            .collect();
    }

    /// The shared check and message helpers, then the constants that force
    /// a non-generic target's proofs during `cargo check`.
    pub(crate) fn ekran_within(&self) -> TokenStream {
        let indexes = self
            .within
            .iter()
            .filter_map(|(_, proof)| match proof {
                WithinProof::Compiled { index, .. } => Some(*index),
                WithinProof::Expanded => None,
            })
            .collect::<Vec<_>>();
        if indexes.is_empty() {
            return TokenStream::new();
        }

        let check = emit_check();
        let forced = (!self.target.has_generics()).then(|| {
            let target = self.target.ty();
            let proofs = indexes.iter().map(|index| {
                let proof =
                    self.ctx.names.get(IntegralName::WithinProof(*index));
                return quote! { const _: () = <#target>::#proof; };
            });
            quote! { #(#proofs)* }
        });
        let messages = emit_messages();
        return quote! {
            #check

            #messages

            #forced
        };
    }

    /// A proof constant's body: the table tier when the source has a table
    /// and the target's validation is const-callable, else the form tier.
    fn within_proof_body(
        &self,
        friend: &Friend,
        primitive: Option<N>,
    ) -> TokenStream {
        let texts = ProofTexts::new(self, friend);
        let (table, form) = match primitive {
            Some(_) => {
                (quote! { ::core::option::Option::Some([true; 256]) }, None)
            }
            None => {
                let path = friend.ty.path();
                let names = &self.ctx.names;
                let table = names.get(IntegralName::WithinTable);
                let form = names.get(IntegralName::WithinForm);
                let why = names.get(IntegralName::WithinWhy);
                (
                    quote! { <#path>::#table },
                    Some(self.within_form_tier(
                        &quote! { <#path>::#form },
                        &quote! { <#path>::#why },
                        &texts,
                    )),
                )
            }
        };
        let signed = match primitive {
            Some(source) => source.is_signed().into_token_stream(),
            None => {
                let path = friend.ty.path();
                let raw = self.ctx.names.get(IntegralName::WithinRaw);
                quote! { <#path>::#raw.0 }
            }
        };
        if !self.validation_konst() {
            let form =
                form.expect("primitive table proofs need const validation");
            return quote! { { #form } };
        }
        let table_tier = self.within_table_tier(&signed, &texts);
        let otherwise = form.map(|form| quote! { else { #form } });
        return quote! {
            {
                if let ::core::option::Option::Some(table) = #table {
                    #table_tier
                }
                #otherwise
            }
        };
    }

    /// Each value the source's table admits must fit the raw type and pass
    /// the target's own validation.
    fn within_table_tier(
        &self,
        signed: &TokenStream,
        texts: &ProofTexts,
    ) -> TokenStream {
        let el = &self.el;
        let validation = &self.ctx.validation;
        let (min, max) = self.target_bounds(Width::W16);
        let clamp = |it: Int| it.to_i128().clamp(-128, 255) as i16;
        let lo = Literal::i16_unsuffixed(clamp(min));
        let hi = Literal::i16_unsuffixed(clamp(max));
        let ProofTexts {
            refuted,
            range,
            domain,
            ..
        } = texts;
        return quote! {
            let signed: bool = #signed;
            let mut index = 0usize;
            while index < 256 {
                if table[index] {
                    let raw: i16 = if signed {
                        index as i16 - 128
                    }
                    else {
                        index as i16
                    };
                    let counterexample = (raw < 0, raw.unsigned_abs() as u128);
                    if raw < #lo || raw > #hi {
                        __typekin_within_refute(#refuted, counterexample, #range);
                    }
                    let value = raw as #el;
                    if !(#validation) {
                        __typekin_within_refute(#refuted, counterexample, #domain);
                    }
                }
                index += 1;
            }
        };
    }

    /// The source's form against the target's, per pointer width, or the
    /// reason one of them has none.
    fn within_form_tier(
        &self,
        source_form: &TokenStream,
        source_why: &TokenStream,
        texts: &ProofTexts,
    ) -> TokenStream {
        let ProofTexts {
            refuted,
            range,
            domain,
            cannot,
            budget,
            steps,
        } = texts;
        let el = &self.el;
        let min = match self.repr().is_signed() {
            true => quote! { (true, (#el::MIN as i128).unsigned_abs()) },
            false => quote! { (false, 0u128) },
        };
        let max = quote! { (false, #el::MAX as u128) };
        let checks = Width::ALL.map(|width| {
            return match self.target_form(width) {
                Ok(target) => {
                    let target = target.tokens();
                    quote! {
                        match __typekin_within_subset(source, #target, #min, #max) {
                            __TypekinWithin::Proven => {}
                            __TypekinWithin::Range(negative, magnitude) => {
                                __typekin_within_refute(#refuted, (negative, magnitude), #range)
                            }
                            __TypekinWithin::Domain(negative, magnitude) => {
                                __typekin_within_refute(#refuted, (negative, magnitude), #domain)
                            }
                            __TypekinWithin::Budget(needed) => {
                                __typekin_within_refute(#budget, (false, needed), #steps)
                            }
                        }
                    }
                }
                Err(reason) => {
                    let ty = self.target.ident.unraw().to_string();
                    let reason = reason.describe(&ty);
                    quote! { __typekin_within_explain(#cannot, #reason) }
                }
            };
        });
        let check = per_width_statement(checks);
        return quote! {
            match #source_form {
                ::core::option::Option::Some(source) => { #check }
                ::core::option::Option::None => {
                    __typekin_within_explain(#cannot, #source_why)
                }
            }
        };
    }
}

/// The message parts of one friend's proof.
struct ProofTexts {
    /// "`S` is not within `T`: ", before a counterexample.
    refuted: String,
    /// " does not fit `u8`", after one outside the raw range.
    range: String,
    /// " is outside the domain of `T`", after one outside the domain.
    domain: String,
    /// "cannot prove `S` within `T`: ", before a reason.
    cannot: String,
    /// `cannot`, then "the check needs ", before the steps it needs.
    budget: String,
    /// " steps, above the subdomain proof budget of 65536".
    steps: String,
}

impl ProofTexts {
    fn new(
        maker: &Maker,
        friend: &Friend,
    ) -> Self {
        let source = spelled(friend);
        let ty = maker.target.ident.unraw();
        let cannot = format!("cannot prove `{source}` within `{ty}`: ");
        return Self {
            refuted: format!("`{source}` is not within `{ty}`: "),
            range: format!(" does not fit `{}`", maker.el),
            domain: format!(" is outside the domain of `{ty}`"),
            budget: format!("{cannot}the check needs "),
            cannot,
            steps: format!(
                " steps, above the subdomain proof budget of \
                {__TYPEKIN_WITHIN_BUDGET}"
            ),
        };
    }
}

/// A friend's source type as written, without token spacing.
fn spelled(friend: &Friend) -> String {
    return friend
        .ty
        .path()
        .to_token_stream()
        .to_string()
        .replace(' ', "");
}

/// `head`, `value` in decimal, and `tail`, as the generated proofs write
/// them.
fn message(
    head: &str,
    value: Int,
    tail: &str,
) -> String {
    let mut text = __TypekinWithinText::new();
    text.push(head);
    text.push_value((value.is_negative(), value.magnitude()));
    text.push(tail);
    return text.as_str().to_string();
}

/// `#[doc(hidden)] pub const #name: #ty = #value;`, once, or once per
/// pointer width when the values differ.
fn per_width_const(
    name: &proc_macro2::Ident,
    ty: &TokenStream,
    values: [TokenStream; Width::COUNT],
) -> TokenStream {
    let items = by_width(values).into_iter().map(|(cfg, value)| {
        return quote! {
            #cfg
            #[doc(hidden)]
            pub const #name: #ty = #value;
        };
    });
    return quote! { #(#items)* };
}

/// One statement, or one per pointer width when they differ.
fn per_width_statement(statements: [TokenStream; Width::COUNT]) -> TokenStream {
    let statements = by_width(statements);
    if let [(_, statement)] = &statements[..] {
        return statement.clone();
    }
    let items = statements.into_iter().map(|(cfg, statement)| {
        return quote! {
            #cfg
            let () = { #statement };
        };
    });
    return quote! { #(#items)* };
}
