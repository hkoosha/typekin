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
    PanicAt,
    Translation,
};
use crate::zz_forward::{
    CoreFwd,
    Panics,
    Ret,
};
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
    /// An operator's row of `OPS`, for the right-hand side `slot`. `konst`:
    /// the body must be const-callable, which the operators of `Wrapping`
    /// are not.
    Operator {
        spec: &'a OpSpec,
        slot: &'a Slot,
        konst: bool,
    },
    /// A forward's row of `flag`'s table: the call of the primitive
    /// function of its name, on the forward's receiver and parameters.
    Forward {
        row: &'a CoreFwd,
        flag: IntegralFlag,
    },
    /// `Sum` or `Product` as `core` folds the raw type: the `core::iter`
    /// method of this name on `items`, an iterator of raw values.
    Fold {
        method: &'a Ident,
        items: &'a TokenStream,
    },
    /// One step of that fold: `spec` (`+` or `*`) on the raw values of
    /// `folded`, the fold so far, and of `item`, both wrapper values named
    /// outside an impl of the target.
    FoldStep {
        spec: &'a OpSpec,
        item: &'a TokenStream,
    },
    /// The `core::iter::Step` method of this name: the raw `start` stepped
    /// by `count`.
    Step(&'a Ident),
}

/// An operation on the raw values.
pub(crate) struct Body {
    /// The raw result, an expression of the raw operands (`lhs` and `rhs`;
    /// a step's `start` and `count`; those its `OpRef` names). Below
    /// `panic_at = anything` an operator's is an `Option` of that result:
    /// `None` where there is none.
    pub(crate) tokens: TokenStream,
    /// Whether the result can fail: it may be invalid, or there may be none.
    pub(crate) fails: bool,
    /// Whether the computation itself can panic: rustc's overflow and
    /// shift checks, division, `sequential`'s overflow panic, the
    /// primitive function a forward calls.
    pub(crate) panics: bool,
}

impl Maker {
    /// What `row` computes, whether that can fail and whether it can
    /// panic. An operator follows its slot's arithmetic. At `panic_at =
    /// anything` it panics as that arithmetic does, and its result fails
    /// unless the type is unvalidated or a proof shows the result valid.
    /// Below it every operator reports through `_or_err`, so each counts
    /// as failing. A forward's failure is `forward_fails`, and it panics
    /// as its row says. A fold is a float's: it never panics and no proof
    /// covers it. A step is the raw type's own checked or overflowing one,
    /// whatever the arithmetic; only validation could reject it.
    pub(crate) fn compute(
        &self,
        row: OpRef,
    ) -> Body {
        let validated = self.ctx.cfg.validation.has_validation();
        let level = self.ctx.cfg.wide.panic_at;
        let el = &self.el;
        return match row {
            OpRef::Operator { spec, slot, konst } => {
                let proven = self.slot_proven(spec, slot);
                let (tokens, panics) =
                    self.operator(spec, slot.arithmetic, konst, level);
                Body {
                    tokens,
                    fails: level.fallible() || (validated && !proven),
                    panics,
                }
            }
            OpRef::Forward { row, flag } => Body {
                tokens: self.core_fwd_call(row, flag),
                fails: self.forward_fails(row, level),
                panics: row.panics != Panics::Never,
            },
            OpRef::Fold { method, items } => Body {
                tokens: quote! { #items.#method::<#el>() },
                fails: level.fallible() || validated,
                panics: false,
            },
            OpRef::FoldStep { spec, item } => {
                // `Self` is not the target where a fold takes steps.
                let get_raw = self.target.qualify_path(&self.ctx.get_raw);
                let symbol = spec.token();
                Body {
                    tokens: quote! {
                        #get_raw(folded) #symbol #get_raw(#item)
                    },
                    fails: level.fallible() || validated,
                    panics: false,
                }
            }
            OpRef::Step(method) => {
                let start = self.storage.extract(quote! { start.0 });
                Body {
                    tokens: quote! {
                        <#el as ::core::iter::Step>::#method(#start, count)
                    },
                    fails: validated,
                    panics: false,
                }
            }
        };
    }

    /// Whether a form can panic that computes a body which `computes` says
    /// can and builds `Self` from the result as `translation` says: in the
    /// computation, on an invalid value (`Unwrap`), or in the storage's
    /// test for zero. A proof's `debug_assert!` does not count.
    pub(crate) fn form_panics(
        &self,
        computes: bool,
        translation: Translation,
    ) -> bool {
        return computes
            || matches!(translation, Translation::Unwrap(_))
            || self.storage.write_panics();
    }

    /// Whether a forward's result can fail. A raw result is never invalid;
    /// there is none where the primitive function panics (`ilog*`) and
    /// `level` checks such input. A validated `Self` (also in an `Option`
    /// or a tuple) fails like an operator's result: below `panic_at =
    /// anything` always, else unless the type is unvalidated or the row is
    /// closed.
    fn forward_fails(
        &self,
        row: &CoreFwd,
        level: PanicAt,
    ) -> bool {
        return match row.ret {
            Ret::Raw(_) => {
                row.panics == Panics::Arithmetic && level.checks_division()
            }
            Ret::Wrap | Ret::Opt | Ret::Pair(..) => {
                level.fallible()
                    || (self.ctx.cfg.validation.has_validation()
                        && !self.core_fwd_closed(row))
            }
        };
    }

    /// `spec` on the raw `lhs` (and `rhs`), and whether that can panic. Bit
    /// operators cannot overflow and floats never panic: both are the raw
    /// operator at every level. A unary operator and a const body use the
    /// primitive's `wrapping_*` methods where the arithmetic is
    /// `Wrapping`'s.
    fn operator(
        &self,
        spec: &OpSpec,
        arithmetic: Arithmetic,
        konst: bool,
        level: PanicAt,
    ) -> (TokenStream, bool) {
        let plain =
            spec.family == Konsts::Bit || matches!(self.num, Num::Float(_));
        let arithmetic = match (plain, konst || spec.kind == OpKind::Unary) {
            (true, _) => Arithmetic::Primitive,
            (false, true) => arithmetic.konst(),
            (false, false) => arithmetic,
        };
        return match (level.fallible(), plain) {
            (false, true) => (panicking(arithmetic, spec), false),
            (false, false) => (
                panicking(arithmetic, spec),
                arithmetic_panics(arithmetic, spec),
            ),
            (true, true) => (some(panicking(arithmetic, spec)), false),
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

/// Whether `panicking`'s expression can panic for an integer operator that
/// is not a bit operator: rustc checks the raw operator (overflow, a zero
/// divisor, the shift amount), the wrapping arithmetics still divide by
/// zero, and `sequential` panics on overflow itself.
pub(crate) fn arithmetic_panics(
    arithmetic: Arithmetic,
    spec: &OpSpec,
) -> bool {
    return match arithmetic {
        Arithmetic::Primitive | Arithmetic::Checked => true,
        Arithmetic::PrimitiveWrapping | Arithmetic::WrapperWrapping => {
            matches!(spec.op, Op::Div | Op::Rem)
        }
    };
}

/// `spec` on the raw operands as an `Option` of the raw result: `None`
/// where `level` reports a failure instead of panicking; and whether the
/// expression can still panic. `sequential` always checks; `int` checks
/// division (zero and `MIN / -1`) from `overflow` and everything at
/// `nothing`; the wrapping profiles only have division by zero to check.
fn reported(
    arithmetic: Arithmetic,
    spec: &OpSpec,
    level: PanicAt,
) -> (TokenStream, bool) {
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
        return (checked(spec), false);
    }
    let value = some(panicking(arithmetic, spec));
    if division && level.checks_division() {
        let guarded = quote! {
            if rhs == 0 {
                ::core::option::Option::None
            }
            else {
                #value
            }
        };
        return (guarded, false);
    }
    return (value, arithmetic_panics(arithmetic, spec));
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
