use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};

use crate::integral::Maker;
use crate::integral_cfg::PanicAt;
use crate::zz_lib::Konsts;
use crate::zz_profile_integral::{
    Arithmetic,
    Slot,
};
use crate::zz_values::{
    Num,
    Op,
    OpKind,
    OpSpec,
};

/// The operation `Maker::compute` writes.
#[derive(Clone, Copy)]
pub(crate) enum OpRef<'a> {
    /// An operator's row of `OPS`. `konst`: the body must be const-callable,
    /// which the operators of `Wrapping` are not.
    Operator { spec: &'a OpSpec, konst: bool },
    /// The `core::iter::Step` method of this name: the raw `start` stepped
    /// by `count`.
    Step(&'a Ident),
}

/// An operation on the raw values.
pub(crate) struct Body {
    /// The raw result, an expression of the raw operands (`lhs` and `rhs`;
    /// a step's `start` and `count`). Below `panic_at = anything` an
    /// operator's is an `Option` of that result: `None` where there is none.
    pub(crate) tokens: TokenStream,
    /// Whether the result can fail: it may be invalid, or there may be none.
    pub(crate) fails: bool,
}

impl Maker {
    /// What `row` computes for the right-hand side `slot`, and whether that
    /// can fail. An operator follows the slot's arithmetic. At `panic_at =
    /// anything` it panics as that arithmetic does, and its result fails
    /// unless the type is unvalidated or a proof shows the result valid.
    /// Below it every operator reports through `_or_err`, so each counts
    /// as failing. A step is the raw type's own, whatever the arithmetic;
    /// only validation could reject it.
    pub(crate) fn compute(
        &self,
        row: OpRef,
        slot: &Slot,
    ) -> Body {
        let validated = self.ctx.cfg.validation.has_validation();
        return match row {
            OpRef::Operator { spec, konst } => {
                let level = self.ctx.cfg.wide.panic_at;
                let proven = self.slot_proven(spec, slot);
                Body {
                    tokens: self.operator(spec, slot.arithmetic, konst, level),
                    fails: level.fallible() || (validated && !proven),
                }
            }
            OpRef::Step(method) => {
                let el = &self.el;
                let start = self.storage.extract(quote! { start.0 });
                Body {
                    tokens: quote! {
                        <#el as ::core::iter::Step>::#method(#start, count)
                    },
                    fails: validated,
                }
            }
        };
    }

    /// `spec` on the raw `lhs` (and `rhs`). Bit operators cannot overflow
    /// and floats never panic: both are the raw operator at every level. A
    /// unary operator and a const body use the primitive's `wrapping_*`
    /// methods where the arithmetic is `Wrapping`'s.
    fn operator(
        &self,
        spec: &OpSpec,
        arithmetic: Arithmetic,
        konst: bool,
        level: PanicAt,
    ) -> TokenStream {
        let plain =
            spec.family == Konsts::Bit || matches!(self.num, Num::Float(_));
        let arithmetic = match (plain, konst || spec.kind == OpKind::Unary) {
            (true, _) => Arithmetic::Primitive,
            (false, true) => arithmetic.konst(),
            (false, false) => arithmetic,
        };
        return match (level.fallible(), plain) {
            (false, _) => panicking(arithmetic, spec),
            (true, true) => some(panicking(arithmetic, spec)),
            (true, false) => reported(arithmetic, spec, level),
        };
    }
}

fn some(value: TokenStream) -> TokenStream {
    return quote! { ::core::option::Option::Some(#value) };
}

/// The right-hand argument of the primitive's `wrapping_*` and `checked_*`
/// methods: a shift amount is a `u32` there, and a unary operator takes
/// none.
fn method_rhs(spec: &OpSpec) -> TokenStream {
    return match spec.kind {
        OpKind::Shift => quote! { rhs as u32 },
        OpKind::Unary => TokenStream::new(),
        OpKind::Arith | OpKind::Bitwise => quote! { rhs },
    };
}

/// `spec` on the raw operands under `arithmetic`, panicking where that
/// arithmetic does: the raw operator as Rust defines it, a wrapping one, or
/// `sequential`'s checked one.
fn panicking(
    arithmetic: Arithmetic,
    spec: &OpSpec,
) -> TokenStream {
    let symbol = spec.token();
    return match arithmetic {
        Arithmetic::Primitive => match spec.kind == OpKind::Unary {
            true => quote! { #symbol lhs },
            false => quote! { lhs #symbol rhs },
        },
        Arithmetic::PrimitiveWrapping => {
            let method = format_ident!("wrapping_{}", spec.method);
            let rhs = method_rhs(spec);
            quote! { lhs.#method(#rhs) }
        }
        Arithmetic::WrapperWrapping => {
            let rhs = match spec.kind == OpKind::Shift {
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
            let method = format_ident!("checked_{}", spec.method);
            let rhs = method_rhs(spec);
            quote! {
                match lhs.#method(#rhs) {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panic!("operation overflowed");
                    }
                }
            }
        }
    };
}

/// `spec` on the raw operands as an `Option` of the raw result: `None`
/// where `level` reports a failure instead of panicking. `sequential`
/// always checks; `int` checks division (zero and `MIN / -1`) from
/// `overflow` and everything at `nothing`; the wrapping profiles only have
/// division by zero to check.
fn reported(
    arithmetic: Arithmetic,
    spec: &OpSpec,
    level: PanicAt,
) -> TokenStream {
    let division = matches!(spec.op, Op::Div | Op::Rem);
    let is_checked = match arithmetic {
        Arithmetic::Checked => true,
        Arithmetic::Primitive => match division {
            true => level.checks_division(),
            false => level.checks_overflow(),
        },
        Arithmetic::PrimitiveWrapping | Arithmetic::WrapperWrapping => false,
    };
    if is_checked {
        return checked(spec);
    }
    let value = some(panicking(arithmetic, spec));
    if division && level.checks_division() {
        return quote! {
            if rhs == 0 {
                ::core::option::Option::None
            }
            else {
                #value
            }
        };
    }
    return value;
}

/// `checked_{method}` of the raw operands; a shift amount past `u32` is
/// `None` rather than truncated.
fn checked(spec: &OpSpec) -> TokenStream {
    let method = format_ident!("checked_{}", spec.method);
    let rhs = method_rhs(spec);
    let call = quote! { lhs.#method(#rhs) };
    return match spec.kind == OpKind::Shift {
        true => quote! {
            if rhs > u32::MAX as usize {
                ::core::option::Option::None
            }
            else {
                #call
            }
        },
        false => call,
    };
}
