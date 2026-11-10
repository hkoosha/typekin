use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};

use crate::integral::Maker;
use crate::integral_cfg::{
    IntegralFlag,
    Kind,
    Translation,
    translate,
};
use crate::integral_compute::{
    Body,
    OpRef,
};
use crate::zz_domain_float::MathProvider;
use crate::zz_forward::{
    Arg,
    CORE_FLOAT,
    CORE_INT,
    CoreFwd,
    FLOAT_MATH,
    FwdKonst,
    Gate,
    Plain,
    Recv,
    Ret,
    RetEl,
    Sign,
    When,
};
use crate::zz_lib::{
    Check,
    FnAttrs,
    Forward,
    Konst,
    Konsts,
    Named as _,
    OnInvalid,
    Output,
};
use crate::zz_values::{
    FnName,
    Num,
};

/// The forward tables, each with the flag that generates its rows.
const FORWARDS: [(IntegralFlag, &[CoreFwd]); 3] = [
    (IntegralFlag::CoreInt, CORE_INT),
    (IntegralFlag::CoreFloat, CORE_FLOAT),
    (IntegralFlag::FloatMath, FLOAT_MATH),
];

impl Maker {
    /// The inherent items the `core` forwards rely on: the validating
    /// result helpers.
    pub(crate) fn forward_items(&self) -> TokenStream {
        let mut stream = self.make_result_helpers();
        stream.extend(self.make_option_helpers());
        return stream;
    }

    /// The constness of the `core_int` or `core_float` forwards (by kind)
    /// and of their result helpers.
    pub(crate) fn core_konst(&self) -> Option<TokenStream> {
        let family = match self.ctx.cfg.kind {
            Kind::Int => Konsts::CoreInt,
            Kind::Float => Konsts::CoreFloat,
        };
        return Konst::konst(self.ctx.cfg.konst_for(&[
            family,
            Konsts::Value,
        ]));
    }

    /// `core_int` items that are not functions: `BITS`.
    pub(crate) fn make_core_int(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::CoreInt) {
            return TokenStream::new();
        }
        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        let el = &self.el;

        return quote! {
            impl #impl_generics #ty #where_clause {
                /// The number of bits in the wrapped primitive integer.
                pub const BITS: u32 = #el::BITS;
            }
        };
    }

    /// The `CORE_INT` row for `name`, when this wrapper has the function.
    pub(crate) fn core_forward(
        &self,
        name: FnName,
    ) -> Option<&'static CoreFwd> {
        return CoreFwd::of(CORE_INT, name)
            .filter(|it| self.core_fwd_exists(it.when));
    }

    /// `core_int` is on, `name`'s row applies to this wrapper and has a
    /// form, and no `without = fn_{name}` removes it.
    pub(crate) fn forward_generated(
        &self,
        name: FnName,
    ) -> bool {
        let flag = IntegralFlag::CoreInt;
        return self.ctx.cfg.flags.has(flag)
            && self.core_forward(name).is_some_and(|row| {
                let body = self.compute(OpRef::Forward { row, flag });
                return self.core_fwd_form(row, body.fails).is_some();
            })
            && !self.ctx.cfg.removals.removes(name);
    }

    /// The row's sign and gate hold for this wrapper.
    fn core_fwd_exists(
        &self,
        when: When,
    ) -> bool {
        let cfg = &self.ctx.cfg;
        let sign = match when.sign {
            Sign::Any => true,
            Sign::Signed => self.repr().is_signed(),
            Sign::Unsigned => !self.repr().is_signed(),
        };
        let gate = match when.gate {
            Gate::Always => true,
            Gate::NightlyKonst => matches!(cfg.wide.konst, Konst::Nightly),
            Gate::Std => cfg.math == MathProvider::Std,
            Gate::Unordered => !cfg.flags.has(IntegralFlag::Ord),
        };
        return sign && gate;
    }

    /// What a forward does with a body that `fails`; `None` where it is
    /// not generated. A function never changes its signature, so a
    /// forward's output is fixed: one that would have to report a failure
    /// is left out, be that an invalid `Self` or a raw result the
    /// primitive function would panic for. An `Option` reports it as
    /// `None`.
    fn core_fwd_form(
        &self,
        row: &CoreFwd,
        fails: bool,
    ) -> Option<Translation> {
        return match row.ret {
            Ret::Opt => Some(Translation::Pass),
            Ret::Raw(_) | Ret::Wrap | Ret::Pair(..) => {
                translate(fails, self.ctx.cfg.wide.panic_at, true)
            }
        };
    }

    /// One forward, named `name`, from a row of `flag`'s table: the
    /// signature and the attributes around `core_fwd_body`. A forward is an
    /// inherent function that returns a value. It can panic where the
    /// primitive function does (the row's `panics`), and one that builds
    /// `Self` also as its form does.
    fn core_fwd_tokens(
        &self,
        row: &CoreFwd,
        flag: IntegralFlag,
        name: &Ident,
        translation: Translation,
        body: &Body,
    ) -> TokenStream {
        let receiver = match row.recv {
            Recv::Value => Some(quote! { self }),
            Recv::Ref => Some(quote! { &self }),
            Recv::Assoc => None,
        };
        let declared = row.args.iter().map(|it| self.core_fwd_parameter(*it));
        let parameters = receiver.into_iter().chain(declared);
        let output = match row.ret {
            Ret::Raw(ty) => self.core_plain(ty),
            Ret::Wrap => quote! { Self },
            Ret::Opt => quote! { ::core::option::Option<Self> },
            Ret::Pair(first, second) => {
                let types = [
                    first,
                    second,
                ]
                .map(|it| match it {
                    RetEl::Wrap(_) => quote! { Self },
                    RetEl::Bool(_) => quote! { bool },
                });
                quote! { (#(#types),*) }
            }
        };
        let konst = match row.konst {
            FwdKonst::Always => Some(quote! { const }),
            FwdKonst::Setting => self.core_konst(),
            FwdKonst::Never => None,
        };
        let panics = match row.ret {
            Ret::Raw(_) => body.panics,
            Ret::Wrap | Ret::Opt | Ret::Pair(..) => {
                self.form_panics(body.panics, translation)
            }
        };
        let FnAttrs {
            must_use,
            track_caller,
        } = FnAttrs::of(panics, Output::Value);
        let (doc, body) = self.core_fwd_body(row, flag, translation, body);

        return quote! {
            #doc
            #must_use
            #[inline(always)]
            #track_caller
            pub #konst fn #name(#(#parameters),*) -> #output {
                #body
            }
        };
    }

    /// A parameter's declaration.
    fn core_fwd_parameter(
        &self,
        arg: Arg,
    ) -> TokenStream {
        return match arg {
            Arg::Wrapped(name) => {
                let name = format_ident!("{}", name);
                quote! { #name: Self }
            }
            Arg::Ref(name) => {
                let name = format_ident!("{}", name);
                quote! { #name: &Self }
            }
            Arg::Plain(name, ty) => {
                let name = format_ident!("{}", name);
                let ty = self.core_plain(ty);
                quote! { #name: #ty }
            }
        };
    }

    /// The argument a forward passes on for a parameter.
    fn core_fwd_passed(
        &self,
        arg: Arg,
    ) -> TokenStream {
        let get_raw = &self.ctx.get_raw;
        return match arg {
            Arg::Wrapped(name) => {
                let name = format_ident!("{}", name);
                quote! { #get_raw(#name) }
            }
            Arg::Ref(name) => {
                let name = format_ident!("{}", name);
                quote! { &#get_raw(*#name) }
            }
            Arg::Plain(name, _) => {
                let name = format_ident!("{}", name);
                quote! { #name }
            }
        };
    }

    /// A type a forward passes through.
    fn core_plain(
        &self,
        ty: Plain,
    ) -> TokenStream {
        let el = &self.el;
        return match ty {
            Plain::Bool => quote! { bool },
            Plain::U32 => quote! { u32 },
            Plain::I32 => quote! { i32 },
            Plain::OptionU32 => quote! { ::core::option::Option<u32> },
            Plain::Bytes => quote! { [u8; ::core::mem::size_of::<#el>()] },
            Plain::Unsigned => {
                let unsigned = match self.num {
                    Num::Float(it) => format_ident!("u{}", it.bits()),
                    Num::Int(it) if it.is_signed() => {
                        format_ident!("{}", it.unsigned().name())
                    }
                    Num::Int(_) => el.clone(),
                };
                quote! { #unsigned }
            }
            Plain::Signed => {
                let signed = format_ident!("{}", self.repr().signed().name());
                quote! { #signed }
            }
            Plain::Category => quote! { ::core::num::FpCategory },
            Plain::Ordering => quote! { ::core::cmp::Ordering },
        };
    }

    /// The call of the primitive function of the row's name. `float_math`
    /// follows the `math` provider: `std` has inherent methods, `nightly`
    /// the free functions of `::core::fNN::math`.
    pub(crate) fn core_fwd_call(
        &self,
        row: &CoreFwd,
        flag: IntegralFlag,
    ) -> TokenStream {
        let el = &self.el;
        let get_raw = &self.ctx.get_raw;
        let name = format_ident!("{}", row.name.name());
        let passed = row.args.iter().map(|it| self.core_fwd_passed(*it));
        if flag == IntegralFlag::FloatMath {
            match self.ctx.cfg.math {
                MathProvider::Std => {}
                MathProvider::Nightly => {
                    return quote! {
                        ::core::#el::math::#name(#get_raw(self) #(, #passed)*)
                    };
                }
                MathProvider::Core => {
                    unreachable!("float_math is rejected without a provider")
                }
            }
        }
        return match row.recv {
            Recv::Value => quote! { #get_raw(self).#name(#(#passed),*) },
            Recv::Ref => quote! { #get_raw(*self).#name(#(#passed),*) },
            Recv::Assoc => quote! { #el::#name(#(#passed),*) },
        };
    }

    /// A row's result needs no validation: by its closure proof, or by
    /// the domain's shape where validation that it may skip is configured
    /// (a domain and the generated `_or_panic`).
    pub(crate) fn core_fwd_closed(
        &self,
        row: &CoreFwd,
    ) -> bool {
        let cfg = &self.ctx.cfg;
        let proven = row.proof.is_some_and(|(op, policy)| {
            return self.proven(op, row.rhs(), policy);
        });
        return proven
            || row.structural.is_some_and(|it| {
                return cfg.validation.has_validation()
                    && cfg.flags.has(IntegralFlag::FnOrPanic)
                    && it.closed(&cfg.validation);
            });
    }

    /// The constructor of an `Option` result: `_make`, whose `None`
    /// reports an invalid value. A result that cannot fail skips validation
    /// through `_make_proven` where the type has `_proven`.
    fn option_constructor(
        &self,
        fails: bool,
    ) -> TokenStream {
        return match !fails && self.proofs_available() {
            true => quote! { Self::_make_proven },
            false => quote! { Self::_make },
        };
    }

    /// A forward's doc line and its body: the primitive call `body`
    /// computes, and the construction `translation` asks for of each
    /// validated `Self` in its result.
    fn core_fwd_body(
        &self,
        row: &CoreFwd,
        flag: IntegralFlag,
        translation: Translation,
        body: &Body,
    ) -> (TokenStream, TokenStream) {
        if let Some(decided) = self.decided_query(row) {
            return decided;
        }
        let call = &body.tokens;
        // The doc line belongs to the rows closure proofs cover, where
        // validation exists and the result cannot fail it.
        let doc = self.proof_doc(
            row.proof.is_some()
                && self.ctx.cfg.validation.has_validation()
                && !body.fails,
        );
        let constructed = |value: TokenStream| {
            return self.result_constructed(body.fails, translation, value);
        };
        // A validated `Self` has two spellings: a structural row and the
        // `core_float` methods bind the raw result to `it` first, the
        // other rows pass the call to the constructor.
        let binds = row.structural.is_some()
            || (flag == IntegralFlag::CoreFloat && row.recv != Recv::Assoc);
        let tokens = match row.ret {
            Ret::Raw(_) => quote! { return #call; },
            Ret::Wrap if binds => {
                let result = constructed(quote! { it });
                quote! {
                    let it = #call;
                    return #result;
                }
            }
            Ret::Wrap => {
                let result = constructed(quote! { #call });
                quote! { return #result; }
            }
            Ret::Opt => {
                let constructor = self.option_constructor(body.fails);
                quote! {
                    return match #call {
                        ::core::option::Option::Some(value) => {
                            #constructor(value)
                        }
                        ::core::option::Option::None => {
                            ::core::option::Option::None
                        }
                    };
                }
            }
            Ret::Pair(first, second) => {
                let (names, results): (Vec<_>, Vec<_>) = [
                    first,
                    second,
                ]
                .into_iter()
                .map(|it| match it {
                    RetEl::Wrap(name) => {
                        let name = format_ident!("{}", name);
                        (quote! { #name }, constructed(quote! { #name }))
                    }
                    RetEl::Bool(name) => {
                        let name = format_ident!("{}", name);
                        (quote! { #name }, quote! { #name })
                    }
                })
                .unzip();
                quote! {
                    let (#(#names),*) = #call;
                    return (#(#results),*);
                }
            }
        };
        return (doc, tokens);
    }

    /// The forwarded `core` functions that `without = fn_{name}` keeps, in
    /// table order: each row that applies to this wrapper and has a form.
    pub(crate) fn make_forwards(&self) -> syn::Result<TokenStream> {
        let mut forwards = Vec::new();
        for (flag, rows) in FORWARDS {
            if !self.ctx.cfg.flags.has(flag) {
                continue;
            }
            let rows = rows.iter().filter(|it| self.core_fwd_exists(it.when));
            forwards.extend(rows.filter_map(|it| {
                let body = self.compute(OpRef::Forward { row: it, flag });
                let translation = self.core_fwd_form(it, body.fails)?;
                return Some(Forward::new(it.name, &self.ctx.names, |name| {
                    return self.core_fwd_tokens(
                        it,
                        flag,
                        name,
                        translation,
                        &body,
                    );
                }));
            }));
        }
        let kept = self.ctx.cfg.removals.keep(forwards)?;
        if kept.is_empty() {
            return Ok(kept);
        }

        let ty = &self.ctx.ty;
        let impl_generics = &self.ctx.impl_generics;
        let where_clause = &self.ctx.where_clause;
        return Ok(quote! {
            impl #impl_generics #ty #where_clause {
                #kept
            }
        });
    }

    /// `_result_or_panic` validates an operation's raw result for the
    /// `core_int`, `core_float`, `float_math`, `Sum` and `Product` items.
    /// Its output is fixed, so it only exists where an invalid value may
    /// panic: operations that report failure call none of those.
    fn make_result_helpers(&self) -> TokenStream {
        let flags = &self.ctx.cfg.flags;
        let used = [
            IntegralFlag::CoreInt,
            IntegralFlag::CoreFloat,
            IntegralFlag::FloatMath,
            IntegralFlag::Sum,
            IntegralFlag::Product,
        ]
        .into_iter()
        .any(|it| flags.has(it));
        if !used || self.fixed_form().is_none() {
            return TokenStream::new();
        }
        let el = &self.el;
        let konst = self.core_konst();
        let body = self.construction(
            quote! { value },
            Check::Validate,
            OnInvalid::Panic("operation produced an invalid value"),
        );

        let FnAttrs {
            must_use,
            track_caller,
        } = FnAttrs::of(self.validation_panics(), Output::Value);
        return quote! {
            #must_use
            #[inline(always)]
            #track_caller
            pub(self) #konst fn _result_or_panic(value: #el) -> Self {
                #body
            }
        };
    }

    /// `_make` is `_result_or_panic`'s `Option` form for the `checked_*`
    /// integer forwards; `_make_proven` builds a proven result.
    fn make_option_helpers(&self) -> TokenStream {
        if !self.ctx.cfg.flags.has(IntegralFlag::CoreInt) {
            return TokenStream::new();
        }
        let el = &self.el;
        let konst = self.core_konst();
        let body = self.construction(
            quote! { value },
            Check::Validate,
            OnInvalid::None,
        );

        let FnAttrs {
            must_use,
            track_caller,
        } = FnAttrs::of(self.storage.write_panics(), Output::Value);
        let mut stream = quote! {
            #must_use
            #[inline(always)]
            #track_caller
            pub(self) #konst fn _make(
                value: #el,
            ) -> ::core::option::Option<Self> {
                #body
            }
        };
        if self.proofs_available() {
            stream.extend(quote! {
                #must_use
                #[inline(always)]
                #track_caller
                pub(self) const fn _make_proven(
                    value: #el,
                ) -> ::core::option::Option<Self> {
                    return ::core::option::Option::Some(
                        Self::_proven(value),
                    );
                }
            });
        }
        return stream;
    }
}
