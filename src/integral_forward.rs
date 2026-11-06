use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::Path;

use crate::integral::Maker;
use crate::integral_cfg::{
    IntegralFlag,
    Kind,
};
use crate::zz_domain_closure::{
    ClosureOp,
    Policy,
    Rhs,
};
use crate::zz_lib::{
    Forward,
    Konst,
    Konsts,
};
use crate::zz_values::{
    FnName,
    N,
};

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

    /// The `core_int` methods: each calls the primitive's method of the same
    /// name and validates a wrapper result.
    fn core_int_forwards(&self) -> Vec<Forward> {
        if !self.ctx.cfg.flags.has(IntegralFlag::CoreInt) {
            return Vec::new();
        }
        let konst = self.core_konst();
        return CORE_INT_FORWARDS
            .iter()
            .filter(|it| self.core_forward_applies(it))
            .map(|it| {
                return Forward::new(it.name, &self.ctx.names, |name| {
                    return self.core_forward_tokens(it, &konst, name);
                });
            })
            .collect();
    }

    /// The `CORE_INT_FORWARDS` row for `name` at this wrapper's sign.
    pub(crate) fn core_forward(
        &self,
        name: FnName,
    ) -> Option<&'static CoreForward> {
        let signed = self.repr().is_signed();
        return CORE_INT_FORWARDS
            .iter()
            .find(|it| it.name == name && it.sign.applies(signed));
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
                .is_some_and(|it| self.core_forward_applies(it))
            && !self.ctx.cfg.removals.removes(name);
    }

    /// The row's sign and gate apply to this wrapper, and it panics no more
    /// than `panic_at` allows (`core_forward_kept`).
    fn core_forward_applies(
        &self,
        forward: &CoreForward,
    ) -> bool {
        return forward.sign.applies(self.repr().is_signed())
            && forward.gate.applies(self.ctx.cfg.konst)
            && self.core_forward_kept(forward);
    }

    /// Below `panic_at = anything`, a forward stays only when its
    /// signature cannot panic: reads and queries, unless the primitive
    /// itself panics (`ilog*`) and `panic_at` checks such inputs, and the
    /// `checked_*` methods. A validated `Self` would have to change its
    /// return type to report failure, so those forwards are left out.
    fn core_forward_kept(
        &self,
        forward: &CoreForward,
    ) -> bool {
        let level = self.ctx.cfg.panic_at;
        if !level.fallible() {
            return true;
        }
        return match forward.sig {
            CoreSig::Read(_) | CoreSig::Base(_) | CoreSig::Query(..) => {
                !forward.core_panics || !level.checks_division()
            }
            CoreSig::Method(CoreShape::Checked, _) => true,
            CoreSig::Method(..) | CoreSig::From(_) | CoreSig::Custom(_) => {
                false
            }
        };
    }

    /// One `core_int` forward, named `name`.
    fn core_forward_tokens(
        &self,
        forward: &CoreForward,
        konst: &Option<TokenStream>,
        name: &Ident,
    ) -> TokenStream {
        let get_raw = &self.ctx.get_raw;
        return match forward.sig {
            // `const`, whatever the constness settings.
            CoreSig::Read(out) => {
                let output = self.core_out(out);
                quote! {
                    #[inline(always)]
                    pub const fn #name(self) -> #output {
                        return #get_raw(self).#name();
                    }
                }
            }
            CoreSig::Base(out) => {
                let output = self.core_out(out);
                quote! {
                    #[inline(always)]
                    pub const fn #name(self, base: Self) -> #output {
                        return #get_raw(self).#name(#get_raw(base));
                    }
                }
            }
            CoreSig::Query(rhs, out) => {
                let output = self.core_out(out);
                let (parameters, arguments) = rhs.signature(get_raw);
                quote! {
                    #[inline(always)]
                    pub #konst fn #name(self #(, #parameters)*) -> #output {
                        return #get_raw(self).#name(#(#arguments),*);
                    }
                }
            }
            CoreSig::Method(shape, rhs) => {
                self.core_int_method(konst, shape, rhs, forward.proof, name)
            }
            CoreSig::From(argument) => self.core_from(konst, argument, name),
            CoreSig::Custom(emit) => emit(self, konst, name),
        };
    }

    /// The return type of a `core_int` read or query.
    fn core_out(
        &self,
        out: Out,
    ) -> TokenStream {
        let el = &self.el;
        return match out {
            Out::U32 => quote! { u32 },
            Out::OptionU32 => quote! { ::core::option::Option<u32> },
            Out::Bool => quote! { bool },
            Out::Bytes => quote! { [u8; ::core::mem::size_of::<#el>()] },
            Out::Unsigned => {
                let unsigned =
                    format_ident!("{}", self.repr().unsigned_rust_name());
                quote! { #unsigned }
            }
            Out::Signed => {
                let signed =
                    format_ident!("{}", self.repr().signed_rust_name());
                quote! { #signed }
            }
            Out::El => quote! { #el },
        };
    }

    /// An associated function building a value from the primitive's.
    fn core_from(
        &self,
        konst: &Option<TokenStream>,
        argument: FromArg,
        name: &Ident,
    ) -> TokenStream {
        let el = &self.el;
        let get_raw = &self.ctx.get_raw;
        let (parameter, argument) = match argument {
            FromArg::Value => {
                (quote! { value: Self }, quote! { #get_raw(value) })
            }
            FromArg::Bytes => {
                let bytes = self.core_out(Out::Bytes);
                (quote! { bytes: #bytes }, quote! { bytes })
            }
        };
        return quote! {
            #[inline(always)]
            pub #konst fn #name(#parameter) -> Self {
                return Self::_result_or_panic(#el::#name(#argument));
            }
        };
    }

    /// One `core_int` method forwarding to the primitive method of the same
    /// name; a proven result skips validation.
    fn core_int_method(
        &self,
        konst: &Option<TokenStream>,
        shape: CoreShape,
        rhs: CoreRhs,
        proof: Option<(ClosureOp, Policy)>,
        name: &Ident,
    ) -> TokenStream {
        let get_raw = &self.ctx.get_raw;
        let (parameters, arguments) = rhs.signature(get_raw);
        let proven = proof.is_some_and(|(op, policy)| {
            return self.core_int_proven(Some(op), rhs, policy);
        });
        let doc = self.proof_doc(proven);
        let call = quote! { #get_raw(self).#name(#(#arguments),*) };

        return match shape {
            CoreShape::Checked => {
                let constructor = match proven {
                    true => quote! { Self::_make_proven },
                    false => quote! { Self::_make },
                };
                quote! {
                    #doc
                    #[inline(always)]
                    pub #konst fn #name(
                        self #(, #parameters)*
                    ) -> ::core::option::Option<Self> {
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
            }
            CoreShape::Result => {
                let constructor = self.proven_constructor(
                    proven,
                    quote! { Self::_result_or_panic },
                );
                quote! {
                    #doc
                    #[inline(always)]
                    pub #konst fn #name(self #(, #parameters)*) -> Self {
                        return #constructor(#call);
                    }
                }
            }
            CoreShape::Overflowing => {
                let constructor = self.proven_constructor(
                    proven,
                    quote! { Self::_result_or_panic },
                );
                quote! {
                    #doc
                    #[inline(always)]
                    pub #konst fn #name(
                        self #(, #parameters)*
                    ) -> (Self, bool) {
                        let (value, overflowed) = #call;
                        return (#constructor(value), overflowed);
                    }
                }
            }
        };
    }

    /// `abs` panics on `MIN` with overflow checks and wraps without; a
    /// domain closed under `abs` skips validation.
    fn core_abs(
        &self,
        konst: &Option<TokenStream>,
        name: &Ident,
    ) -> TokenStream {
        let get_raw = &self.ctx.get_raw;
        let (op, policy) = ABS_PROOF;
        let closed = self.validated_closed(
            self.ctx.cfg.validation.abs_closed(),
            op,
            policy,
        );
        let doc = self.proof_doc(closed);
        let result = self.closed_or_validated(closed);
        return quote! {
            #doc
            #[inline(always)]
            pub #konst fn #name(self) -> Self {
                let it = #get_raw(self).#name();
                return #result;
            }
        };
    }

    /// `funnel_shl`/`funnel_shr` (nightly only).
    fn core_funnel(
        &self,
        konst: &Option<TokenStream>,
        name: &Ident,
    ) -> TokenStream {
        let get_raw = &self.ctx.get_raw;
        return quote! {
            #[inline(always)]
            pub #konst fn #name(
                self,
                right: Self,
                n: u32,
            ) -> Self {
                return Self::_result_or_panic(
                    #get_raw(self).#name(#get_raw(right), n)
                );
            }
        };
    }

    fn core_carrying_add(
        &self,
        _konst: &Option<TokenStream>,
        name: &Ident,
    ) -> TokenStream {
        return self.core_with_flag(name, "carry");
    }

    fn core_borrowing_sub(
        &self,
        _konst: &Option<TokenStream>,
        name: &Ident,
    ) -> TokenStream {
        return self.core_with_flag(name, "borrow");
    }

    /// `carrying_add`/`borrowing_sub`: a value and the carry or borrow
    /// flag, passed through.
    fn core_with_flag(
        &self,
        name: &Ident,
        flag: &str,
    ) -> TokenStream {
        let get_raw = &self.ctx.get_raw;
        let flag = format_ident!("{}", flag);
        return quote! {
            #[inline(always)]
            pub fn #name(
                self,
                rhs: Self,
                #flag: bool,
            ) -> (Self, bool) {
                let (value, #flag) =
                    #get_raw(self).#name(#get_raw(rhs), #flag);
                return (Self::_result_or_panic(value), #flag);
            }
        };
    }

    fn core_carrying_mul(
        &self,
        _konst: &Option<TokenStream>,
        name: &Ident,
    ) -> TokenStream {
        let get_raw = &self.ctx.get_raw;
        return quote! {
            #[inline(always)]
            pub fn #name(self, rhs: Self, carry: Self) -> (Self, Self) {
                let (low, high) =
                    #get_raw(self).#name(#get_raw(rhs), #get_raw(carry));
                return (
                    Self::_result_or_panic(low),
                    Self::_result_or_panic(high),
                );
            }
        };
    }

    fn core_carrying_mul_add(
        &self,
        _konst: &Option<TokenStream>,
        name: &Ident,
    ) -> TokenStream {
        let get_raw = &self.ctx.get_raw;
        return quote! {
            #[inline(always)]
            pub fn #name(
                self,
                rhs: Self,
                carry: Self,
                add: Self,
            ) -> (Self, Self) {
                let (low, high) = #get_raw(self).#name(
                    #get_raw(rhs),
                    #get_raw(carry),
                    #get_raw(add),
                );
                return (
                    Self::_result_or_panic(low),
                    Self::_result_or_panic(high),
                );
            }
        };
    }

    /// The forwarded `core` functions that `without = fn_{name}` keeps.
    pub(crate) fn make_forwards(&self) -> syn::Result<TokenStream> {
        let mut forwards = self.core_int_forwards();
        forwards.extend(self.core_float_forwards());
        forwards.extend(self.float_math_forwards());
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
        let validation = &self.ctx.validation;
        let constructed = self.construct_self(quote! { value });

        return quote! {
            #[inline(always)]
            #[track_caller]
            pub(self) #konst fn _result_or_panic(value: #el) -> Self {
                if #validation {
                    return #constructed;
                }
                else {
                    ::core::panic!("operation produced an invalid value");
                }
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
        let validation = &self.ctx.validation;
        let constructed = self.construct_self(quote! { value });

        let mut stream = quote! {
            #[inline(always)]
            pub(self) #konst fn _make(
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

/// How a generated `core_int` method returns its result.
#[derive(Clone, Copy)]
enum CoreShape {
    /// `Option<Self>`: overflow gives `None`.
    Checked,
    /// `Self`, under the entry's policy.
    Result,
    /// `(Self, bool)`: the value wraps.
    Overflowing,
}

/// What a `core_int` read or query returns.
#[derive(Clone, Copy)]
enum Out {
    U32,
    OptionU32,
    Bool,
    /// `[u8; size_of::<el>()]`.
    Bytes,
    /// The unsigned primitive of the same width.
    Unsigned,
    /// The signed primitive of the same width.
    Signed,
    /// The wrapped primitive.
    El,
}

/// The parameter of a `from_*` associated function.
#[derive(Clone, Copy)]
enum FromArg {
    /// `value: Self`, passed on as its raw value.
    Value,
    /// `bytes: [u8; size_of::<el>()]`.
    Bytes,
}

/// A hand-written forward: the maker, the constness, the function's name.
type CoreEmit = fn(&Maker, &Option<TokenStream>, &Ident) -> TokenStream;

/// How a `core_int` forward's function is built.
#[derive(Clone, Copy)]
enum CoreSig {
    /// `pub const fn name(self) -> out`, whatever the constness settings.
    Read(Out),
    /// `pub const fn name(self, base: Self) -> out`.
    Base(Out),
    /// `pub #konst fn name(self[, rhs]) -> out`.
    Query(CoreRhs, Out),
    /// A validated wrapper result, proven when the row's proof holds.
    Method(CoreShape, CoreRhs),
    /// `pub #konst fn name(value | bytes) -> Self`.
    From(FromArg),
    /// `abs`, `funnel_*`, `carrying_*`, `borrowing_sub`.
    Custom(CoreEmit),
}

/// The integer signs a forward exists for.
#[derive(Clone, Copy)]
enum Sign {
    Any,
    Signed,
    Unsigned,
}

impl Sign {
    fn applies(
        self,
        signed: bool,
    ) -> bool {
        return match self {
            Self::Any => true,
            Self::Signed => signed,
            Self::Unsigned => !signed,
        };
    }
}

/// When a forward is generated, beyond `core_int` and its sign.
#[derive(Clone, Copy)]
enum Gate {
    Always,
    /// Only with `konst = nightly` (not `konst_core_int`): the primitive
    /// function is unstable.
    NightlyKonst,
}

impl Gate {
    fn applies(
        self,
        konst: Konst,
    ) -> bool {
        return match self {
            Self::Always => true,
            Self::NightlyKonst => matches!(konst, Konst::Nightly),
        };
    }
}

/// A generated `core_int` function forwarding to the primitive function of
/// the same name.
pub(crate) struct CoreForward {
    name: FnName,
    sig: CoreSig,
    sign: Sign,
    gate: Gate,
    /// The closure-proof question; `None`: never proven.
    pub(crate) proof: Option<(ClosureOp, Policy)>,
    /// The primitive function itself panics on some inputs, as `ilog2` on
    /// a non-positive value does.
    core_panics: bool,
}

impl CoreForward {
    /// The right-hand side of the row's operation; `abs` is unary.
    pub(crate) fn rhs(&self) -> CoreRhs {
        return match self.sig {
            CoreSig::Method(_, rhs) | CoreSig::Query(rhs, _) => rhs,
            _ => CoreRhs::Unary,
        };
    }
}

/// `abs` panics on `MIN` with overflow checks and wraps without: wrapped
/// results cover both.
const ABS_PROOF: (ClosureOp, Policy) = (ClosureOp::Abs, Policy::Wrapping);

const fn row(
    name: FnName,
    sig: CoreSig,
) -> CoreForward {
    return CoreForward {
        name,
        sig,
        sign: Sign::Any,
        gate: Gate::Always,
        proof: None,
        core_panics: false,
    };
}

const fn read(
    name: FnName,
    out: Out,
) -> CoreForward {
    return row(name, CoreSig::Read(out));
}

const fn base(
    name: FnName,
    out: Out,
) -> CoreForward {
    return row(name, CoreSig::Base(out));
}

const fn query(
    name: FnName,
    rhs: CoreRhs,
    out: Out,
) -> CoreForward {
    return row(name, CoreSig::Query(rhs, out));
}

/// A validated method that is never proven.
const fn method(
    name: FnName,
    shape: CoreShape,
    rhs: CoreRhs,
) -> CoreForward {
    return row(name, CoreSig::Method(shape, rhs));
}

/// A validated method, proven when `op` under `policy` is closed.
const fn proven(
    name: FnName,
    shape: CoreShape,
    rhs: CoreRhs,
    op: ClosureOp,
    policy: Policy,
) -> CoreForward {
    return CoreForward {
        proof: Some((op, policy)),
        ..method(name, shape, rhs)
    };
}

const fn from_value(name: FnName) -> CoreForward {
    return row(name, CoreSig::From(FromArg::Value));
}

const fn from_bytes(name: FnName) -> CoreForward {
    return row(name, CoreSig::From(FromArg::Bytes));
}

const fn custom(
    name: FnName,
    emit: CoreEmit,
) -> CoreForward {
    return row(name, CoreSig::Custom(emit));
}

const fn signed(it: CoreForward) -> CoreForward {
    return CoreForward {
        sign: Sign::Signed,
        ..it
    };
}

const fn unsigned(it: CoreForward) -> CoreForward {
    return CoreForward {
        sign: Sign::Unsigned,
        ..it
    };
}

const fn nightly(it: CoreForward) -> CoreForward {
    return CoreForward {
        gate: Gate::NightlyKonst,
        ..it
    };
}

/// The primitive function panics on some inputs.
const fn core_panics(it: CoreForward) -> CoreForward {
    return CoreForward {
        core_panics: true,
        ..it
    };
}

/// The `core_int` forwards, in emission order: the common rows, then the
/// signed or the unsigned ones. `abs_diff` has a row per sign.
const CORE_INT_FORWARDS: &[CoreForward] = {
    use ClosureOp::*;
    use CoreRhs::{
        Same,
        Unary,
    };
    use CoreShape::{
        Checked,
        Overflowing,
        Result,
    };
    use Out::{
        Bool,
        Bytes,
        El,
        OptionU32,
        U32,
    };
    const ROTATE: CoreRhs = CoreRhs::Amount("n");
    const RHS: CoreRhs = CoreRhs::Amount("rhs");
    const EXP: CoreRhs = CoreRhs::Amount("exp");
    const C: Policy = Policy::Checked;
    const S: Policy = Policy::Saturating;
    // Overflow panics with overflow checks and wraps without: wrapped
    // results cover both.
    const W: Policy = Policy::Wrapping;
    &[
        read(FnName::CountOnes, U32),
        read(FnName::CountZeros, U32),
        read(FnName::LeadingZeros, U32),
        read(FnName::TrailingZeros, U32),
        read(FnName::LeadingOnes, U32),
        read(FnName::TrailingOnes, U32),
        read(FnName::HighestOne, OptionU32),
        read(FnName::LowestOne, OptionU32),
        core_panics(base(FnName::Ilog, U32)),
        core_panics(read(FnName::Ilog2, U32)),
        core_panics(read(FnName::Ilog10, U32)),
        base(FnName::CheckedIlog, OptionU32),
        read(FnName::CheckedIlog2, OptionU32),
        read(FnName::CheckedIlog10, OptionU32),
        read(FnName::ToBeBytes, Bytes),
        read(FnName::ToLeBytes, Bytes),
        read(FnName::ToNeBytes, Bytes),
        method(FnName::RotateLeft, Result, ROTATE),
        method(FnName::RotateRight, Result, ROTATE),
        method(FnName::SwapBytes, Result, Unary),
        method(FnName::ReverseBits, Result, Unary),
        method(FnName::IsolateHighestOne, Result, Unary),
        method(FnName::IsolateLowestOne, Result, Unary),
        method(FnName::ToBe, Result, Unary),
        method(FnName::ToLe, Result, Unary),
        from_value(FnName::FromBe),
        from_value(FnName::FromLe),
        from_bytes(FnName::FromNeBytes),
        from_bytes(FnName::FromBeBytes),
        from_bytes(FnName::FromLeBytes),
        proven(FnName::CheckedAdd, Checked, Same, Add, C),
        proven(FnName::CheckedSub, Checked, Same, Sub, C),
        proven(FnName::CheckedNeg, Checked, Unary, Neg, C),
        proven(FnName::CheckedMul, Checked, Same, Mul, C),
        proven(FnName::CheckedDiv, Checked, Same, Div, C),
        proven(FnName::CheckedRem, Checked, Same, Rem, C),
        proven(FnName::CheckedDivEuclid, Checked, Same, DivEuclid, C),
        proven(FnName::CheckedRemEuclid, Checked, Same, RemEuclid, C),
        method(FnName::CheckedShl, Checked, RHS),
        proven(FnName::CheckedShr, Checked, RHS, Shr, C),
        method(FnName::CheckedPow, Checked, EXP),
        proven(FnName::Midpoint, Result, Same, Midpoint, C),
        method(FnName::UnboundedShl, Result, RHS),
        proven(FnName::UnboundedShr, Result, RHS, UnboundedShr, C),
        proven(FnName::SaturatingAdd, Result, Same, Add, S),
        proven(FnName::SaturatingSub, Result, Same, Sub, S),
        proven(FnName::SaturatingMul, Result, Same, Mul, S),
        proven(FnName::SaturatingDiv, Result, Same, Div, S),
        method(FnName::SaturatingPow, Result, EXP),
        proven(FnName::WrappingAdd, Result, Same, Add, W),
        proven(FnName::WrappingSub, Result, Same, Sub, W),
        proven(FnName::WrappingNeg, Result, Unary, Neg, W),
        proven(FnName::WrappingMul, Result, Same, Mul, W),
        proven(FnName::WrappingDiv, Result, Same, Div, W),
        proven(FnName::WrappingRem, Result, Same, Rem, W),
        proven(FnName::WrappingDivEuclid, Result, Same, DivEuclid, W),
        proven(FnName::WrappingRemEuclid, Result, Same, RemEuclid, W),
        method(FnName::WrappingShl, Result, RHS),
        proven(FnName::WrappingShr, Result, RHS, Shr, W),
        method(FnName::WrappingPow, Result, EXP),
        method(FnName::Pow, Result, EXP),
        proven(FnName::Isqrt, Result, Unary, Isqrt, C),
        proven(FnName::DivEuclid, Result, Same, DivEuclid, W),
        proven(FnName::RemEuclid, Result, Same, RemEuclid, W),
        proven(FnName::StrictAdd, Result, Same, Add, C),
        proven(FnName::StrictSub, Result, Same, Sub, C),
        proven(FnName::StrictNeg, Result, Unary, Neg, C),
        proven(FnName::StrictMul, Result, Same, Mul, C),
        proven(FnName::StrictDiv, Result, Same, Div, C),
        proven(FnName::StrictRem, Result, Same, Rem, C),
        proven(FnName::StrictDivEuclid, Result, Same, DivEuclid, C),
        proven(FnName::StrictRemEuclid, Result, Same, RemEuclid, C),
        method(FnName::StrictShl, Result, RHS),
        proven(FnName::StrictShr, Result, RHS, Shr, C),
        method(FnName::StrictPow, Result, EXP),
        proven(FnName::OverflowingAdd, Overflowing, Same, Add, W),
        proven(FnName::OverflowingSub, Overflowing, Same, Sub, W),
        proven(FnName::OverflowingNeg, Overflowing, Unary, Neg, W),
        proven(FnName::OverflowingMul, Overflowing, Same, Mul, W),
        proven(FnName::OverflowingDiv, Overflowing, Same, Div, W),
        proven(FnName::OverflowingRem, Overflowing, Same, Rem, W),
        proven(
            FnName::OverflowingDivEuclid,
            Overflowing,
            Same,
            DivEuclid,
            W,
        ),
        proven(
            FnName::OverflowingRemEuclid,
            Overflowing,
            Same,
            RemEuclid,
            W,
        ),
        method(FnName::OverflowingShl, Overflowing, RHS),
        proven(FnName::OverflowingShr, Overflowing, RHS, Shr, W),
        method(FnName::OverflowingPow, Overflowing, EXP),
        signed(query(FnName::CastUnsigned, Unary, Out::Unsigned)),
        signed(CoreForward {
            proof: Some(ABS_PROOF),
            ..custom(FnName::Abs, Maker::core_abs)
        }),
        signed(proven(FnName::CheckedAbs, Checked, Unary, Abs, C)),
        signed(proven(FnName::CheckedIsqrt, Checked, Unary, Isqrt, C)),
        signed(proven(FnName::StrictAbs, Result, Unary, Abs, C)),
        signed(proven(FnName::SaturatingNeg, Result, Unary, Neg, S)),
        signed(proven(FnName::SaturatingAbs, Result, Unary, Abs, S)),
        signed(proven(FnName::WrappingAbs, Result, Unary, Abs, W)),
        signed(proven(FnName::OverflowingAbs, Overflowing, Unary, Abs, W)),
        signed(query(FnName::UnsignedAbs, Unary, Out::Unsigned)),
        signed(query(FnName::AbsDiff, Same, Out::Unsigned)),
        signed(method(FnName::Signum, Result, Unary)),
        signed(query(FnName::IsPositive, Unary, Bool)),
        signed(query(FnName::IsNegative, Unary, Bool)),
        unsigned(query(FnName::CastSigned, Unary, Out::Signed)),
        unsigned(query(FnName::BitWidth, Unary, U32)),
        unsigned(nightly(custom(FnName::FunnelShl, Maker::core_funnel))),
        unsigned(nightly(custom(FnName::FunnelShr, Maker::core_funnel))),
        unsigned(query(FnName::AbsDiff, Same, El)),
        unsigned(query(FnName::IsMultipleOf, Same, Bool)),
        unsigned(query(FnName::IsPowerOfTwo, Unary, Bool)),
        unsigned(method(FnName::NextPowerOfTwo, Result, Unary)),
        unsigned(method(FnName::CheckedNextPowerOfTwo, Checked, Unary)),
        unsigned(method(FnName::DivCeil, Result, Same)),
        unsigned(method(FnName::NextMultipleOf, Result, Same)),
        unsigned(method(FnName::CheckedNextMultipleOf, Checked, Same)),
        unsigned(custom(FnName::CarryingAdd, Maker::core_carrying_add)),
        unsigned(custom(FnName::BorrowingSub, Maker::core_borrowing_sub)),
        unsigned(custom(FnName::CarryingMul, Maker::core_carrying_mul)),
        unsigned(custom(FnName::CarryingMulAdd, Maker::core_carrying_mul_add)),
    ]
};

/// The right-hand side of a generated `core_int` method.
#[derive(Clone, Copy)]
pub(crate) enum CoreRhs {
    Unary,
    /// `rhs: Self`.
    Same,
    /// `name: u32`: a shift amount or an exponent.
    Amount(&'static str),
}

impl CoreRhs {
    pub(crate) fn closure(self) -> Rhs {
        return match self {
            Self::Unary => Rhs::Unary,
            Self::Same => Rhs::SameDomain,
            Self::Amount(_) => Rhs::Shift,
        };
    }

    /// The parameters after `self` and the primitive call's arguments.
    fn signature(
        self,
        get_raw: &Path,
    ) -> (Vec<TokenStream>, Vec<TokenStream>) {
        return match self {
            Self::Unary => params(&[], get_raw),
            Self::Same => params(&[Param::Wrapped("rhs")], get_raw),
            Self::Amount(name) => {
                params(&[Param::Plain(name, N::U032)], get_raw)
            }
        };
    }
}

/// A parameter of a forwarded function, after `self`.
#[derive(Clone, Copy)]
pub(crate) enum Param {
    /// `name: Self`, passed on as `#get_raw(name)`.
    Wrapped(&'static str),
    /// `name: ty`, passed on as is.
    Plain(&'static str, N),
}

/// Each parameter's declaration and the argument the forward passes on.
pub(crate) fn params(
    params: &[Param],
    get_raw: &Path,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    return params
        .iter()
        .map(|it| {
            return match *it {
                Param::Wrapped(name) => {
                    let name = format_ident!("{}", name);
                    (quote! { #name: Self }, quote! { #get_raw(#name) })
                }
                Param::Plain(name, ty) => {
                    let name = format_ident!("{}", name);
                    let ty = format_ident!("{}", ty.rust_name());
                    (quote! { #name: #ty }, quote! { #name })
                }
            };
        })
        .unzip();
}
