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
    Panics,
    Plain,
    Recv,
    Ret,
    RetEl,
    Sign,
    When,
};
use crate::zz_lib::{
    Check,
    Forward,
    Konst,
    Konsts,
    Named as _,
    OnInvalid,
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

    /// `core_int` is on, `name`'s row applies to this wrapper, and no
    /// `without = fn_{name}` removes it.
    pub(crate) fn forward_generated(
        &self,
        name: FnName,
    ) -> bool {
        return self.ctx.cfg.flags.has(IntegralFlag::CoreInt)
            && self
                .core_forward(name)
                .is_some_and(|it| self.core_fwd_kept(it))
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

    /// Below `panic_at = anything`, a forward stays only when its
    /// signature cannot panic: a raw result, unless the primitive itself
    /// panics (`ilog*`) and `panic_at` checks such inputs, and an
    /// `Option`. A validated `Self` would have to change its return type
    /// to report failure, so those forwards are left out.
    fn core_fwd_kept(
        &self,
        row: &CoreFwd,
    ) -> bool {
        let level = self.ctx.cfg.wide.panic_at;
        if !level.fallible() {
            return true;
        }
        return match row.ret {
            Ret::Raw(_) => {
                row.panics == Panics::Never || !level.checks_division()
            }
            ret => !ret.panics(),
        };
    }

    /// One forward, named `name`, from a row of `flag`'s table: the
    /// signature and the attributes around `core_fwd_body`.
    fn core_fwd_tokens(
        &self,
        row: &CoreFwd,
        flag: IntegralFlag,
        name: &Ident,
    ) -> TokenStream {
        let receiver = match row.recv {
            Recv::Value => Some(quote! { self }),
            Recv::Ref => Some(quote! { &self }),
            Recv::Assoc => None,
        };
        let (declared, passed): (Vec<_>, Vec<_>) =
            row.args.iter().map(|it| self.core_fwd_arg(*it)).unzip();
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
        let (doc, body) = self.core_fwd_body(row, flag, name, &passed);
        // By family and return shape: an integer forward carries neither
        // attribute; a float's is `must_use` when called on a value and
        // `track_caller` when its result holds a validated `Self`.
        let float = flag != IntegralFlag::CoreInt;
        let must_use =
            (float && row.recv != Recv::Assoc).then(|| quote! { #[must_use] });
        let track_caller =
            (float && row.ret.panics()).then(|| quote! { #[track_caller] });

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

    /// A parameter's declaration and the argument the forward passes on.
    fn core_fwd_arg(
        &self,
        arg: Arg,
    ) -> (TokenStream, TokenStream) {
        let get_raw = &self.ctx.get_raw;
        return match arg {
            Arg::Wrapped(name) => {
                let name = format_ident!("{}", name);
                (quote! { #name: Self }, quote! { #get_raw(#name) })
            }
            Arg::Ref(name) => {
                let name = format_ident!("{}", name);
                (quote! { #name: &Self }, quote! { &#get_raw(*#name) })
            }
            Arg::Plain(name, ty) => {
                let name = format_ident!("{}", name);
                let ty = self.core_plain(ty);
                (quote! { #name: #ty }, quote! { #name })
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

    /// The call of the primitive function. `float_math` follows the
    /// `math` provider: `std` has inherent methods, `nightly` the free
    /// functions of `::core::fNN::math`.
    fn core_fwd_call(
        &self,
        row: &CoreFwd,
        flag: IntegralFlag,
        name: &Ident,
        passed: &[TokenStream],
    ) -> TokenStream {
        let el = &self.el;
        let get_raw = &self.ctx.get_raw;
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
    fn core_fwd_closed(
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

    /// A forward's doc line and its body: the primitive call, and the
    /// construction of each validated `Self` in its result.
    fn core_fwd_body(
        &self,
        row: &CoreFwd,
        flag: IntegralFlag,
        name: &Ident,
        passed: &[TokenStream],
    ) -> (TokenStream, TokenStream) {
        if let Some(decided) = self.decided_query(row) {
            return decided;
        }
        let call = self.core_fwd_call(row, flag, name, passed);
        let closed = self.core_fwd_closed(row);
        // The doc line belongs to the rows closure proofs cover.
        let doc = self.proof_doc(closed && row.proof.is_some());
        let constructor =
            self.proven_constructor(closed, quote! { Self::_result_or_panic });
        // A validated `Self` has two spellings: a structural row and the
        // `core_float` methods bind the raw result to `it` first, the
        // other rows pass the call to the constructor.
        let binds = row.structural.is_some()
            || (flag == IntegralFlag::CoreFloat && row.recv != Recv::Assoc);
        let body = match row.ret {
            Ret::Raw(_) => quote! { return #call; },
            Ret::Wrap if binds => {
                let result = self.closed_or_validated(closed);
                quote! {
                    let it = #call;
                    return #result;
                }
            }
            Ret::Wrap => quote! { return #constructor(#call); },
            Ret::Opt => {
                let constructor = match closed {
                    true => quote! { Self::_make_proven },
                    false => quote! { Self::_make },
                };
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
                        (quote! { #name }, quote! { #constructor(#name) })
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
        return (doc, body);
    }

    /// The forwarded `core` functions that `without = fn_{name}` keeps, in
    /// table order.
    pub(crate) fn make_forwards(&self) -> syn::Result<TokenStream> {
        let mut forwards = Vec::new();
        for (flag, rows) in FORWARDS {
            if !self.ctx.cfg.flags.has(flag) {
                continue;
            }
            let rows = rows.iter().filter(|it| {
                return self.core_fwd_exists(it.when) && self.core_fwd_kept(it);
            });
            forwards.extend(rows.map(|it| {
                return Forward::new(it.name, &self.ctx.names, |name| {
                    return self.core_fwd_tokens(it, flag, name);
                });
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
    /// Operations that report failure call none of those.
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
        if !used || self.ctx.failure.is_some() {
            return TokenStream::new();
        }
        let el = &self.el;
        let konst = self.core_konst();
        let body = self.construction(
            quote! { value },
            Check::Validate,
            OnInvalid::Panic("operation produced an invalid value"),
        );

        return quote! {
            #[inline(always)]
            #[track_caller]
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

        let mut stream = quote! {
            #[inline(always)]
            pub(self) #konst fn _make(
                value: #el,
            ) -> ::core::option::Option<Self> {
                #body
            }
        };
        if self.proofs_available() {
            stream.extend(quote! {
                #[inline(always)]
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
