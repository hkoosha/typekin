//! The domain language over `f32`/`f64` subjects.
//!
//! Same syntax as integers. Ranges, rosters, and multiples never hold NaN, and
//! complement is taken within the type, so `!{0.5}` admits NaN. A domain is
//! NaN-free when both NaN probes evaluate to false; that drives
//! `Eq`/`Ord`/`Hash`. The `core` queries (`is_nan`, `is_normal`, …) become
//! constants when the domain admits values of one answer only.
//! Functions beyond stable `core` need a math provider: `math = std`
//! (inherent methods) or `math = nightly` (`::core::fNN::math::f(…)`,
//! no_std).

use std::collections::BTreeMap;
use std::num::FpCategory;

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::parse::{
    Parse,
    ParseStream,
};

use crate::zz_domain::{
    Atom,
    Const,
    ConstKind,
    Domain,
    Lens,
    Node,
    OperandKey,
    Test,
    collapse,
    group_in_order,
    kleene,
    listing,
};
use crate::zz_runner::MkErr;
use crate::zz_values::{
    F,
    FnName,
};

// =============================================================================
// Math provider

/// Where non-`core` float functions come from (`math = …`).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum MathProvider {
    #[default]
    Core,
    Std,
    Nightly,
}

impl Parse for MathProvider {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        return match name.to_string().as_str() {
            "core" => Ok(Self::Core),
            "std" => Ok(Self::Std),
            "nightly" => Ok(Self::Nightly),
            _ => name.span().fail("expected `core`, `std`, or `nightly`"),
        };
    }
}

// =============================================================================
// Untyped NaN probe (parse time; exact for floats)

impl Domain {
    /// Whether some NaN passes the domain. Ranges, rosters, and multiples
    /// reject NaN, their complements accept it, every whitelisted function
    /// maps NaN to NaN, so the answer does not depend on `f32`/`f64`.
    pub(crate) fn admits_nan_untyped(&self) -> bool {
        return [
            false,
            true,
        ]
        .into_iter()
        .any(|negative| nan_node(&self.root, negative));
    }
}

fn nan_node(
    node: &Node,
    negative: bool,
) -> bool {
    return match node {
        Node::Or(items) => items.iter().any(|it| nan_node(it, negative)),
        Node::And(items) => items.iter().all(|it| nan_node(it, negative)),
        Node::Not(inner) => !nan_node(inner, negative),
        Node::Atom(atom) => nan_atom(atom, negative),
    };
}

fn nan_atom(
    atom: &Atom,
    negative: bool,
) -> bool {
    return match &atom.test {
        Test::Range { .. } => false,
        Test::Point { negated, .. } | Test::Divisible { negated, .. } => {
            *negated
        }
        Test::Predicate => {
            match atom.lens.as_ref().and_then(|it| it.function) {
                Some(FnName::IsNan) => true,
                Some(FnName::IsSignNegative) => negative,
                Some(FnName::IsSignPositive) => !negative,
                _ => false,
            }
        }
    };
}

// =============================================================================
// Typing

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Provided {
    /// Stable `core` inherent method.
    Core,
    /// Needs `math = std` or `math = nightly`.
    Provider,
}

#[derive(Clone)]
struct Operand {
    /// `None` for the value itself.
    call: Option<Call>,
    key: OperandKey,
}

impl Operand {
    /// The value itself, not a function application.
    fn is_subject(&self) -> bool {
        return self.key == OperandKey::Subject;
    }
}

#[derive(Clone)]
struct Call {
    name: Ident,
    function: FnName,
    args: Vec<TokenStream>,
    values: Vec<f64>,
    provided: Provided,
}

fn subject() -> Operand {
    return Operand {
        call: None,
        key: OperandKey::Subject,
    };
}

const CORE_FUNCTIONS: &[FnName] = &[
    FnName::Abs,
    FnName::Signum,
    FnName::Copysign,
    FnName::Recip,
    FnName::ToDegrees,
    FnName::ToRadians,
];
const PROVIDER_FUNCTIONS: &[FnName] = &[
    FnName::Floor,
    FnName::Ceil,
    FnName::Round,
    FnName::RoundTiesEven,
    FnName::Trunc,
    FnName::Fract,
    FnName::MulAdd,
    FnName::Sqrt,
    FnName::Cbrt,
    FnName::RemEuclid,
    FnName::DivEuclid,
];
const PREDICATES: &[FnName] = &[
    FnName::IsNan,
    FnName::IsFinite,
    FnName::IsInfinite,
    FnName::IsNormal,
    FnName::IsSubnormal,
    FnName::IsSignPositive,
    FnName::IsSignNegative,
];

fn arity(function: FnName) -> usize {
    return match function {
        FnName::Copysign | FnName::RemEuclid | FnName::DivEuclid => 1,
        FnName::MulAdd => 2,
        _ => 0,
    };
}

fn type_lens(
    lens: &Lens,
    subject: F,
    math: MathProvider,
) -> syn::Result<Operand> {
    let (function, provided) = float_function(lens, subject, math)?;

    let expected = arity(function);
    if lens.args.len() != expected {
        return lens.name.fail(format!(
            "{} takes {expected} argument{}",
            lens.name,
            if expected == 1 { "" } else { "s" }
        ));
    }

    let values = float_args(lens, function, subject)?;
    let args = lens
        .args
        .iter()
        .map(|it| it.tokens.clone())
        .collect::<Vec<_>>();
    return Ok(Operand {
        call: Some(Call {
            name: lens.name.clone(),
            function,
            args,
            values,
            provided,
        }),
        key: OperandKey::of(function, &lens.args),
    });
}

/// The function `lens` names and where it comes from, if a domain may
/// apply it under `math`.
fn float_function(
    lens: &Lens,
    subject: F,
    math: MathProvider,
) -> syn::Result<(FnName, Provided)> {
    let name = lens.name.to_string();
    let listed = |functions: &[FnName]| {
        return lens.function.filter(|it| functions.contains(it));
    };
    return if let Some(function) = listed(CORE_FUNCTIONS) {
        Ok((function, Provided::Core))
    }
    else if let Some(function) = listed(PROVIDER_FUNCTIONS) {
        if math == MathProvider::Core {
            return lens.name.fail(format!(
                "{name} needs math = std (std) or math = nightly (no_std, \
                nightly core_float_math)"
            ));
        }
        Ok((function, Provided::Provider))
    }
    else if lens.function == Some(FnName::Powi) {
        lens.name.fail(
            "powi is not supported in domains (its rounding depends on the \
            optimizer)",
        )
    }
    else if listed(PREDICATES).is_some() {
        lens.name.fail(format!(
            "{name} is a predicate; use it alone or negated: {name} or !{name}"
        ))
    }
    else {
        lens.name.fail(format!(
            "unknown domain function {name}; available for {}: {}, and with \
            math = std or nightly: {}",
            subject.rust_name(),
            listing(CORE_FUNCTIONS),
            listing(PROVIDER_FUNCTIONS),
        ))
    };
}

/// The argument values, checked for `function`.
fn float_args(
    lens: &Lens,
    function: FnName,
    subject: F,
) -> syn::Result<Vec<f64>> {
    let mut values = Vec::new();
    for arg in &lens.args {
        let value = float_value(arg, subject)?;
        match function {
            FnName::Copysign if value.is_nan() => {
                return arg.span.fail("copysign needs a non-NaN constant");
            }
            FnName::RemEuclid | FnName::DivEuclid => {
                check_divisor(arg, value, subject)?
            }
            FnName::MulAdd if !value.is_finite() => {
                return arg.span.fail("mul_add needs finite constants");
            }
            _ => {}
        }
        values.push(value);
    }
    return Ok(values);
}

/// The constant's value in the subject type, widened to `f64` exactly.
fn float_value(
    constant: &Const,
    subject: F,
) -> syn::Result<f64> {
    let value = match &constant.kind {
        ConstKind::Int { .. } | ConstKind::Bound { .. } => {
            return constant
                .span
                .fail("float domains need float literals; write 1.0");
        }
        ConstKind::Float {
            digits,
            suffix,
            negated,
        } => {
            if !suffix.is_empty() && suffix != subject.rust_name() {
                return constant.span.fail(format!(
                    "{suffix} suffix does not match the {} operand",
                    subject.rust_name()
                ));
            }
            let value = parse_in(digits, subject).ok_or_else(|| {
                constant.span.errorful("invalid float constant")
            })?;
            if value.is_infinite() {
                return constant.span.fail(format!(
                    "{} overflows {} to infinity",
                    constant.tokens,
                    subject.rust_name()
                ));
            }
            if *negated { -value } else { value }
        }
        ConstKind::FloatPath {
            ty,
            name,
            consts,
            negated,
        } => {
            if *ty != subject {
                return constant.span.fail(format!(
                    "{}::… does not match the {} operand; write {}::… or a \
                    literal",
                    ty.rust_name(),
                    subject.rust_name(),
                    subject.rust_name()
                ));
            }
            let value = match consts {
                true => const_value(subject, name),
                false => bound_value(subject, name),
            }
            .ok_or_else(|| {
                constant.span.errorful(format!(
                    "unknown {} constant {name}",
                    subject.rust_name()
                ))
            })?;
            if *negated { -value } else { value }
        }
    };
    return Ok(value);
}

fn parse_in(
    digits: &str,
    subject: F,
) -> Option<f64> {
    return match subject {
        F::F032 => digits.parse::<f32>().ok().map(f64::from),
        F::F064 => digits.parse::<f64>().ok(),
    };
}

fn bound_value(
    subject: F,
    name: &str,
) -> Option<f64> {
    return Some(match (subject, name) {
        (F::F032, "MIN") => f64::from(f32::MIN),
        (F::F032, "MAX") => f64::from(f32::MAX),
        (F::F032, "MIN_POSITIVE") => f64::from(f32::MIN_POSITIVE),
        (F::F032, "EPSILON") => f64::from(f32::EPSILON),
        (F::F064, "MIN") => f64::MIN,
        (F::F064, "MAX") => f64::MAX,
        (F::F064, "MIN_POSITIVE") => f64::MIN_POSITIVE,
        (F::F064, "EPSILON") => f64::EPSILON,
        (_, "INFINITY") => f64::INFINITY,
        (_, "NEG_INFINITY") => f64::NEG_INFINITY,
        _ => return None,
    });
}

fn const_value(
    subject: F,
    name: &str,
) -> Option<f64> {
    return Some(match (subject, name) {
        (F::F032, "PI") => f64::from(core::f32::consts::PI),
        (F::F032, "TAU") => f64::from(core::f32::consts::TAU),
        (F::F032, "E") => f64::from(core::f32::consts::E),
        (F::F032, "SQRT_2") => f64::from(core::f32::consts::SQRT_2),
        (F::F032, "FRAC_1_SQRT_2") => {
            f64::from(core::f32::consts::FRAC_1_SQRT_2)
        }
        (F::F032, "FRAC_PI_2") => f64::from(core::f32::consts::FRAC_PI_2),
        (F::F032, "FRAC_PI_3") => f64::from(core::f32::consts::FRAC_PI_3),
        (F::F032, "FRAC_PI_4") => f64::from(core::f32::consts::FRAC_PI_4),
        (F::F032, "FRAC_PI_6") => f64::from(core::f32::consts::FRAC_PI_6),
        (F::F032, "FRAC_PI_8") => f64::from(core::f32::consts::FRAC_PI_8),
        (F::F032, "FRAC_1_PI") => f64::from(core::f32::consts::FRAC_1_PI),
        (F::F032, "FRAC_2_PI") => f64::from(core::f32::consts::FRAC_2_PI),
        (F::F032, "FRAC_2_SQRT_PI") => {
            f64::from(core::f32::consts::FRAC_2_SQRT_PI)
        }
        (F::F032, "LN_2") => f64::from(core::f32::consts::LN_2),
        (F::F032, "LN_10") => f64::from(core::f32::consts::LN_10),
        (F::F032, "LOG2_E") => f64::from(core::f32::consts::LOG2_E),
        (F::F032, "LOG10_E") => f64::from(core::f32::consts::LOG10_E),
        (F::F032, "LOG2_10") => f64::from(core::f32::consts::LOG2_10),
        (F::F032, "LOG10_2") => f64::from(core::f32::consts::LOG10_2),
        (F::F064, "PI") => core::f64::consts::PI,
        (F::F064, "TAU") => core::f64::consts::TAU,
        (F::F064, "E") => core::f64::consts::E,
        (F::F064, "SQRT_2") => core::f64::consts::SQRT_2,
        (F::F064, "FRAC_1_SQRT_2") => core::f64::consts::FRAC_1_SQRT_2,
        (F::F064, "FRAC_PI_2") => core::f64::consts::FRAC_PI_2,
        (F::F064, "FRAC_PI_3") => core::f64::consts::FRAC_PI_3,
        (F::F064, "FRAC_PI_4") => core::f64::consts::FRAC_PI_4,
        (F::F064, "FRAC_PI_6") => core::f64::consts::FRAC_PI_6,
        (F::F064, "FRAC_PI_8") => core::f64::consts::FRAC_PI_8,
        (F::F064, "FRAC_1_PI") => core::f64::consts::FRAC_1_PI,
        (F::F064, "FRAC_2_PI") => core::f64::consts::FRAC_2_PI,
        (F::F064, "FRAC_2_SQRT_PI") => core::f64::consts::FRAC_2_SQRT_PI,
        (F::F064, "LN_2") => core::f64::consts::LN_2,
        (F::F064, "LN_10") => core::f64::consts::LN_10,
        (F::F064, "LOG2_E") => core::f64::consts::LOG2_E,
        (F::F064, "LOG10_E") => core::f64::consts::LOG10_E,
        (F::F064, "LOG2_10") => core::f64::consts::LOG2_10,
        (F::F064, "LOG10_2") => core::f64::consts::LOG10_2,
        _ => return None,
    });
}

/// `mul(c)`, `rem_euclid(c)`, `div_euclid(c)`: finite, positive, and exactly
/// the written decimal (D13): `mul(0.1)` would test a different divisor than
/// written.
fn check_divisor(
    divisor: &Const,
    value: f64,
    subject: F,
) -> syn::Result<()> {
    if value == 0.0 {
        return divisor.span.fail("division by zero");
    }
    if !value.is_finite() {
        return divisor.span.fail("float divisors must be finite");
    }
    if value < 0.0 {
        return divisor.span.fail("negative divisors are not supported");
    }
    if let ConstKind::Float { digits, .. } = &divisor.kind
        && canonical_decimal(digits)
            != canonical_decimal(&format!("{value:.1100}"))
    {
        return divisor.span.fail(format!(
            "{digits} is not exactly representable as {} (it is {}); use a \
            divisor such as 0.125",
            subject.rust_name(),
            exact_decimal(value),
        ));
    }
    return Ok(());
}

/// Significant digits and the decimal exponent of a decimal string such as
/// `1e-3`, `0.125`, or `12.50`.
fn canonical_decimal(text: &str) -> (String, i64) {
    let text = text.replace('_', "");
    let (mantissa, exponent) = match text.find([
        'e',
        'E',
    ]) {
        Some(index) => (
            &text[..index],
            text[index + 1..].parse::<i64>().unwrap_or_default(),
        ),
        None => (text.as_str(), 0),
    };
    let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let digits = format!("{whole}{fraction}");
    let exponent = exponent + whole.len() as i64;
    let leading = digits.len() - digits.trim_start_matches('0').len();
    let digits = digits.trim_start_matches('0').trim_end_matches('0');
    return (digits.to_owned(), exponent - leading as i64);
}

fn exact_decimal(value: f64) -> String {
    let text = format!("{value:.1100}");
    let text = text.trim_end_matches('0');
    return text.trim_end_matches('.').to_owned();
}

// =============================================================================
// Resolved form

#[derive(Clone)]
struct Bound {
    value: f64,
    tokens: TokenStream,
    inclusive: bool,
}

#[derive(Clone)]
enum FNode {
    Const(bool),
    Or(Vec<FNode>),
    And(Vec<FNode>),
    Not(Box<FNode>),
    /// Non-NaN values between the bounds; no bounds means `!is_nan`.
    Interval {
        operand: Operand,
        lo: Option<Bound>,
        hi: Option<Bound>,
    },
    Ne {
        operand: Operand,
        value: f64,
        tokens: TokenStream,
    },
    Divisible {
        operand: Operand,
        divisor: f64,
        tokens: TokenStream,
        negated: bool,
    },
    Predicate {
        name: Ident,
        function: FnName,
    },
}

/// A float domain resolved for one subject type.
#[derive(Clone)]
pub(crate) struct ResolvedFloatDomain {
    subject: F,
    math: MathProvider,
    root: FNode,
}

impl Domain {
    pub(crate) fn resolve_float(
        &self,
        subject: F,
        math: MathProvider,
    ) -> syn::Result<ResolvedFloatDomain> {
        let root = normalize(type_node(&self.root, subject, math)?);
        if matches!(root, FNode::Const(false)) {
            return self.span.fail(format!(
                "domain admits no {} value",
                subject.rust_name()
            ));
        }
        if matches!(root, FNode::Const(true)) {
            return self.span.fail(format!(
                "domain admits every {} value; remove the domain",
                subject.rust_name()
            ));
        }
        return Ok(ResolvedFloatDomain {
            subject,
            math,
            root,
        });
    }
}

fn type_node(
    node: &Node,
    subject: F,
    math: MathProvider,
) -> syn::Result<FNode> {
    return Ok(match node {
        Node::Or(items) => FNode::Or(
            items
                .iter()
                .map(|it| type_node(it, subject, math))
                .collect::<syn::Result<_>>()?,
        ),
        Node::And(items) => FNode::And(
            items
                .iter()
                .map(|it| type_node(it, subject, math))
                .collect::<syn::Result<_>>()?,
        ),
        Node::Not(inner) => {
            FNode::Not(Box::new(type_node(inner, subject, math)?))
        }
        Node::Atom(atom) => type_atom(atom, subject, math)?,
    });
}

fn type_atom(
    atom: &Atom,
    subject: F,
    math: MathProvider,
) -> syn::Result<FNode> {
    if let Test::Predicate = atom.test {
        let lens = atom.lens.as_ref().expect("predicates have a name");
        return type_predicate(lens, subject);
    }

    let operand = match &atom.lens {
        None => self::subject(),
        Some(lens) => type_lens(lens, subject, math)?,
    };
    let bound = |constant: &Const, inclusive: bool| -> syn::Result<Bound> {
        return Ok(Bound {
            value: float_value(constant, subject)?,
            tokens: constant.tokens.clone(),
            inclusive,
        });
    };

    return Ok(match &atom.test {
        Test::Predicate => unreachable!("handled above"),
        Test::Range { lo, hi, inclusive } => {
            let lo = lo.as_ref().map(|it| bound(it, true)).transpose()?;
            let hi = hi.as_ref().map(|it| bound(it, *inclusive)).transpose()?;
            check_empty_range(atom, &lo, &hi)?;
            clamp(FNode::Interval { operand, lo, hi })
        }
        Test::Point { value, negated } => {
            let it = bound(value, true)?;
            match negated {
                false => clamp(FNode::Interval {
                    operand,
                    lo: Some(it.clone()),
                    hi: Some(it),
                }),
                true => FNode::Ne {
                    operand,
                    value: it.value,
                    tokens: it.tokens,
                },
            }
        }
        Test::Divisible { divisor, negated } => {
            let value = float_value(divisor, subject)?;
            check_divisor(divisor, value, subject)?;
            FNode::Divisible {
                operand,
                divisor: value,
                tokens: divisor.tokens.clone(),
                negated: *negated,
            }
        }
    });
}

fn type_predicate(
    lens: &Lens,
    subject: F,
) -> syn::Result<FNode> {
    let name = lens.name.to_string();
    if !lens.args.is_empty() {
        return lens.name.fail(format!("{name} takes no arguments"));
    }
    let Some(function) = lens.function.filter(|it| PREDICATES.contains(it))
    else {
        return lens.name.fail(format!(
            "unknown domain predicate {name}; available for {}: {}",
            subject.rust_name(),
            listing(PREDICATES)
        ));
    };
    return Ok(FNode::Predicate {
        name: lens.name.clone(),
        function,
    });
}

fn check_empty_range(
    atom: &Atom,
    lo: &Option<Bound>,
    hi: &Option<Bound>,
) -> syn::Result<()> {
    if let (Some(lo), Some(hi)) = (lo, hi)
        && (lo.value > hi.value || (lo.value == hi.value && !hi.inclusive))
    {
        return atom.span.fail(format!(
            "this range is empty; its upper bound {} is not above its lower \
            bound",
            hi.tokens
        ));
    }
    return Ok(());
}

/// Infinite bounds that include infinity add nothing beyond `!is_nan`.
fn clamp(node: FNode) -> FNode {
    let FNode::Interval { operand, lo, hi } = node
    else {
        return node;
    };
    let lo = lo.filter(|it| !(it.value == f64::NEG_INFINITY && it.inclusive));
    let hi = hi.filter(|it| !(it.value == f64::INFINITY && it.inclusive));
    if let (Some(lo), Some(hi)) = (&lo, &hi)
        && (lo.value > hi.value
            || (lo.value == hi.value && !(lo.inclusive && hi.inclusive)))
    {
        return FNode::Const(false);
    }
    return FNode::Interval { operand, lo, hi };
}

// -----------------------------------------------------------------------------
// Normalization

fn normalize(node: FNode) -> FNode {
    return match node {
        FNode::Or(items) => normalize_or(items),
        FNode::And(items) => normalize_and(items),
        FNode::Not(inner) => match normalize(*inner) {
            FNode::Const(value) => FNode::Const(!value),
            other => FNode::Not(Box::new(other)),
        },
        other => other,
    };
}

fn flatten(
    items: Vec<FNode>,
    or: bool,
) -> Vec<FNode> {
    let mut result = Vec::with_capacity(items.len());
    for item in items {
        match (normalize(item), or) {
            (FNode::Or(inner), true) | (FNode::And(inner), false) => {
                result.extend(inner);
            }
            (other, _) => result.push(other),
        }
    }
    return result;
}

fn cmp_lo(
    left: &Option<Bound>,
    right: &Option<Bound>,
) -> std::cmp::Ordering {
    return match (left, right) {
        (None, None) => std::cmp::Ordering::Equal,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (Some(_), None) => std::cmp::Ordering::Greater,
        (Some(left), Some(right)) => left
            .value
            .partial_cmp(&right.value)
            .expect("bounds are never NaN")
            .then(right.inclusive.cmp(&left.inclusive)),
    };
}

/// The grouping key of an interval; other nodes are not grouped.
fn interval_key(node: &FNode) -> Option<&OperandKey> {
    return match node {
        FNode::Interval { operand, .. } => Some(&operand.key),
        _ => None,
    };
}

fn normalize_or(items: Vec<FNode>) -> FNode {
    let items = flatten(items, true);
    if items.iter().any(|it| matches!(it, FNode::Const(true))) {
        return FNode::Const(true);
    }
    let items = items
        .into_iter()
        .filter(|it| !matches!(it, FNode::Const(false)))
        .collect::<Vec<_>>();

    // N3: per operand, sort and merge overlapping or touching intervals.
    let (order, mut groups) = group_in_order(items, interval_key);

    let mut result = Vec::new();
    for entry in order {
        match entry {
            Err(item) => result.push(item),
            Ok(key) => result.extend(merge_union(groups.remove(&key).unwrap())),
        }
    }

    return collapse(result, FNode::Const(false), FNode::Or);
}

/// One operand's intersected interval: the operand and its two ends.
type Ends = (Operand, Option<Bound>, Option<Bound>);

fn parts(node: FNode) -> Ends {
    let FNode::Interval { operand, lo, hi } = node
    else {
        unreachable!("only intervals are grouped");
    };
    return (operand, lo, hi);
}

fn merge_union(intervals: Vec<FNode>) -> Vec<FNode> {
    let mut parts = intervals.into_iter().map(parts).collect::<Vec<_>>();
    parts.sort_by(|left, right| cmp_lo(&left.1, &right.1));

    let mut merged = Vec::<(Operand, Option<Bound>, Option<Bound>)>::new();
    for next in parts {
        let Some(current) = merged.last_mut()
        else {
            merged.push(next);
            continue;
        };
        let connected = match (&current.2, &next.1) {
            (None, _) | (_, None) => true,
            (Some(hi), Some(lo)) => {
                lo.value < hi.value
                    || (lo.value == hi.value && (lo.inclusive || hi.inclusive))
            }
        };
        if !connected {
            merged.push(next);
            continue;
        }
        current.2 = match (current.2.take(), next.2) {
            (None, _) | (_, None) => None,
            (Some(left), Some(right)) => Some(
                match right.value > left.value
                    || (right.value == left.value && right.inclusive)
                {
                    true => right,
                    false => left,
                },
            ),
        };
    }

    return merged
        .into_iter()
        .map(|(operand, lo, hi)| clamp(FNode::Interval { operand, lo, hi }))
        .collect();
}

fn normalize_and(items: Vec<FNode>) -> FNode {
    let items = flatten(items, false);
    if items.iter().any(|it| matches!(it, FNode::Const(false))) {
        return FNode::Const(false);
    }
    let items = items
        .into_iter()
        .filter(|it| !matches!(it, FNode::Const(true)))
        .collect::<Vec<_>>();

    // N4: per operand, intersect intervals at the first one's position.
    let (order, groups) = group_in_order(items, interval_key);
    let mut ends = BTreeMap::<OperandKey, Ends>::new();
    for (key, group) in groups {
        let mut group = group.into_iter().map(parts);
        let mut current = group.next().expect("a group has an interval");
        for (_, lo, hi) in group {
            intersect_ends(&mut current, lo, hi);
        }
        ends.insert(key, current);
    }

    exclusive_ends(&order, &mut ends);
    let finite_subject = finite_subject(&ends);

    let mut result = Vec::new();
    for entry in order {
        match entry {
            Err(item) => result.push(item),
            Ok(key) => {
                let (operand, lo, hi) = ends[&key].clone();
                match clamp(FNode::Interval { operand, lo, hi }) {
                    FNode::Const(false) => return FNode::Const(false),
                    other => result.push(other),
                }
            }
        }
    }

    let result = result
        .into_iter()
        .filter(|item| !redundant(item, &ends, finite_subject))
        .collect::<Vec<_>>();

    return collapse(result, FNode::Const(true), FNode::And);
}

/// Keeps the greater lower and the smaller upper end; on equal values the
/// exclusive one, else the left one.
fn intersect_ends(
    ends: &mut Ends,
    lo: Option<Bound>,
    hi: Option<Bound>,
) {
    ends.1 = match (ends.1.take(), lo) {
        (None, other) | (other, None) => other,
        (Some(left), Some(right)) => Some(
            match right.value > left.value
                || (right.value == left.value && !right.inclusive)
            {
                true => right,
                false => left,
            },
        ),
    };
    ends.2 = match (ends.2.take(), hi) {
        (None, other) | (other, None) => other,
        (Some(left), Some(right)) => Some(
            match right.value < left.value
                || (right.value == left.value && !right.inclusive)
            {
                true => right,
                false => left,
            },
        ),
    };
}

/// N8: `!{c}` at an inclusive end of an AND-sibling interval on the same
/// operand makes that end exclusive; N7 then drops the `!{c}`.
fn exclusive_ends(
    order: &[Result<OperandKey, FNode>],
    ends: &mut BTreeMap<OperandKey, Ends>,
) {
    for item in order {
        let Err(FNode::Ne { operand, value, .. }) = item
        else {
            continue;
        };
        let Some((_, lo, hi)) = ends.get_mut(&operand.key)
        else {
            continue;
        };
        for end in [
            lo,
            hi,
        ]
        .into_iter()
        .flatten()
        {
            if end.inclusive && end.value == *value {
                end.inclusive = false;
            }
        }
    }
}

/// N5 probe: the value itself lies in a finite interval.
fn finite_subject(ends: &BTreeMap<OperandKey, Ends>) -> bool {
    return ends.get(&OperandKey::Subject).is_some_and(|(_, lo, hi)| {
        return lo.as_ref().is_some_and(|it| it.value.is_finite())
            && hi.as_ref().is_some_and(|it| it.value.is_finite());
    });
}

/// N7 and N5: an AND item its sibling intervals already imply.
fn redundant(
    item: &FNode,
    ends: &BTreeMap<OperandKey, Ends>,
    finite_subject: bool,
) -> bool {
    return match item {
        // N7: `!= c` outside an AND-sibling interval is redundant.
        FNode::Ne { operand, value, .. } => {
            ends.get(&operand.key).is_some_and(|(_, lo, hi)| {
                lo.as_ref().is_some_and(|lo| {
                    *value < lo.value || (*value == lo.value && !lo.inclusive)
                }) || hi.as_ref().is_some_and(|hi| {
                    *value > hi.value || (*value == hi.value && !hi.inclusive)
                })
            })
        }
        // N5: a finite interval on the value already implies these.
        FNode::Predicate {
            function: FnName::IsFinite,
            ..
        } => finite_subject,
        _ => false,
    };
}

// -----------------------------------------------------------------------------
// Exact evaluation (probes)

fn apply(
    call: &Call,
    value: f64,
) -> f64 {
    let arg = |index: usize| call.values[index];
    return match call.function {
        FnName::Abs => value.abs(),
        FnName::Signum => value.signum(),
        FnName::Copysign => value.copysign(arg(0)),
        FnName::Recip => value.recip(),
        FnName::ToDegrees => value.to_degrees(),
        FnName::ToRadians => value.to_radians(),
        FnName::Floor => value.floor(),
        FnName::Ceil => value.ceil(),
        FnName::Round => value.round(),
        FnName::RoundTiesEven => value.round_ties_even(),
        FnName::Trunc => value.trunc(),
        FnName::Fract => value.fract(),
        FnName::MulAdd => value.mul_add(arg(0), arg(1)),
        FnName::Sqrt => value.sqrt(),
        FnName::Cbrt => value.cbrt(),
        FnName::RemEuclid => value.rem_euclid(arg(0)),
        FnName::DivEuclid => value.div_euclid(arg(0)),
        _ => f64::NAN,
    };
}

fn operand_value(
    operand: &Operand,
    value: f64,
) -> f64 {
    return match &operand.call {
        None => value,
        Some(call) => apply(call, value),
    };
}

fn eval(
    node: &FNode,
    value: f64,
) -> bool {
    return match node {
        FNode::Const(it) => *it,
        FNode::Or(items) => items.iter().any(|it| eval(it, value)),
        FNode::And(items) => items.iter().all(|it| eval(it, value)),
        FNode::Not(inner) => !eval(inner, value),
        FNode::Interval { operand, lo, hi } => {
            let it = operand_value(operand, value);
            !it.is_nan()
                && lo.as_ref().is_none_or(|lo| match lo.inclusive {
                    true => it >= lo.value,
                    false => it > lo.value,
                })
                && hi.as_ref().is_none_or(|hi| match hi.inclusive {
                    true => it <= hi.value,
                    false => it < hi.value,
                })
        }
        FNode::Ne {
            operand,
            value: other,
            ..
        } => operand_value(operand, value) != *other,
        FNode::Divisible {
            operand,
            divisor,
            negated,
            ..
        } => (operand_value(operand, value) % divisor == 0.0) != *negated,
        FNode::Predicate { function, .. } => predicate(*function, value),
    };
}

/// The `core` query `function` on `value`; typing admits only `PREDICATES`.
fn predicate(
    function: FnName,
    value: f64,
) -> bool {
    return match function {
        FnName::IsNan => value.is_nan(),
        FnName::IsFinite => value.is_finite(),
        FnName::IsInfinite => value.is_infinite(),
        FnName::IsNormal => value.is_normal(),
        FnName::IsSubnormal => value.is_subnormal(),
        FnName::IsSignPositive => value.is_sign_positive(),
        FnName::IsSignNegative => value.is_sign_negative(),
        _ => unreachable!("{} is not a float query", function.name()),
    };
}

impl ResolvedFloatDomain {
    /// Membership of `value` (an exact `f32` value when the subject is
    /// `f32`).
    pub(crate) fn contains(
        &self,
        value: f64,
    ) -> bool {
        return eval(&self.root, value);
    }

    /// Some NaN passes.
    #[cfg(test)]
    pub(crate) fn admits_nan(&self) -> bool {
        return self.contains(f64::NAN) || self.contains(-f64::NAN);
    }

    /// Both `+0.0` and `-0.0` pass: hashing must canonicalize the zero sign.
    pub(crate) fn admits_both_zeros(&self) -> bool {
        return self.contains(0.0) && self.contains(-0.0);
    }

    /// Every valid `x` has a valid `-x`, proven per atom; `!`, `&&`, and
    /// `||` keep the property.
    pub(crate) fn neg_closed(&self) -> bool {
        return symmetric(&self.root);
    }

    /// Every valid `x` has a valid `abs(x)`.
    pub(crate) fn abs_closed(&self) -> bool {
        return abs_closed(&self.root);
    }

    /// The answer every valid value gives to the `core` query `function`
    /// (`is_nan`, `is_normal`, …), when the domain decides it.
    pub(crate) fn query(
        &self,
        function: FnName,
    ) -> Option<bool> {
        let mut answers = Class::all()
            .filter(|class| {
                class.verdict(&self.root, self.subject) != Some(false)
            })
            .map(|class| predicate(function, class.witness()));
        let first = answers.next()?;
        return answers.all(|it| it == first).then_some(first);
    }

    pub(crate) fn emit(
        &self,
        value: &TokenStream,
    ) -> TokenStream {
        return emit(&self.root, value, self.subject, self.math);
    }
}

/// `abs` and `copysign` ignore the sign of their input.
fn sign_blind(operand: &Operand) -> bool {
    return operand.call.as_ref().is_some_and(|it| {
        matches!(it.function, FnName::Abs | FnName::Copysign)
    });
}

fn symmetric(node: &FNode) -> bool {
    return match node {
        FNode::Const(_) => true,
        FNode::Or(items) | FNode::And(items) => items.iter().all(symmetric),
        FNode::Not(inner) => symmetric(inner),
        FNode::Interval { operand, .. }
        | FNode::Ne { operand, .. }
        | FNode::Divisible { operand, .. }
            if sign_blind(operand) =>
        {
            true
        }
        FNode::Interval { operand, lo, hi } if operand.is_subject() => {
            match (lo, hi) {
                (None, None) => true,
                (Some(lo), Some(hi)) => {
                    lo.value == -hi.value && lo.inclusive == hi.inclusive
                }
                _ => false,
            }
        }
        FNode::Ne { operand, value, .. } if operand.is_subject() => {
            *value == 0.0
        }
        // `fmod` takes the dividend's sign; `-0.0 == 0.0`.
        FNode::Divisible { operand, .. } => operand.is_subject(),
        FNode::Predicate { function, .. } => {
            !matches!(function, FnName::IsSignPositive | FnName::IsSignNegative)
        }
        _ => false,
    };
}

fn abs_closed(node: &FNode) -> bool {
    if symmetric(node) {
        return true;
    }
    return match node {
        FNode::Or(items) | FNode::And(items) => items.iter().all(abs_closed),
        // `x >= 0` (`-0.0` included) makes `abs(x) == x` as compared.
        FNode::Interval {
            operand,
            lo: Some(lo),
            ..
        } if operand.is_subject() => lo.value >= 0.0,
        FNode::Predicate { function, .. } => {
            *function == FnName::IsSignPositive
        }
        _ => false,
    };
}

// -----------------------------------------------------------------------------
// Query answers

/// The `core` queries tell values apart only by sign and category: each
/// query answers alike on every value of a class.
#[derive(Clone, Copy)]
struct Class {
    negative: bool,
    category: FpCategory,
}

impl Class {
    fn all() -> impl Iterator<Item = Self> {
        return [
            FpCategory::Nan,
            FpCategory::Infinite,
            FpCategory::Zero,
            FpCategory::Subnormal,
            FpCategory::Normal,
        ]
        .into_iter()
        .flat_map(|category| {
            return [
                false,
                true,
            ]
            .map(|negative| Self { negative, category });
        });
    }

    /// An `f64` of this class, answering every query as the class does.
    /// NaN (whatever its payload), the infinity, and the zero also behave
    /// alike in both widths and under every domain function, so they probe
    /// the domain exactly; the other classes span many values.
    fn witness(self) -> f64 {
        let it = match self.category {
            FpCategory::Nan => f64::NAN,
            FpCategory::Infinite => f64::INFINITY,
            FpCategory::Zero => 0.0,
            FpCategory::Subnormal => f64::from_bits(1),
            FpCategory::Normal => 1.0,
        };
        return if self.negative { -it } else { it };
    }

    /// The least and greatest `subject` values of a subnormal or normal
    /// class, widened to `f64` exactly.
    fn span(
        self,
        subject: F,
    ) -> Option<(f64, f64)> {
        let (first, last) = match (self.category, subject) {
            (FpCategory::Subnormal, F::F032) => (
                f64::from(0.0_f32.next_up()),
                f64::from(f32::MIN_POSITIVE.next_down()),
            ),
            (FpCategory::Subnormal, F::F064) => {
                (0.0_f64.next_up(), f64::MIN_POSITIVE.next_down())
            }
            (FpCategory::Normal, F::F032) => {
                (f64::from(f32::MIN_POSITIVE), f64::from(f32::MAX))
            }
            (FpCategory::Normal, F::F064) => (f64::MIN_POSITIVE, f64::MAX),
            _ => return None,
        };
        return Some(match self.negative {
            true => (-last, -first),
            false => (first, last),
        });
    }

    /// Whether every value of this class passes `node` (`Some(true)`), none
    /// does (`Some(false)`), or the analysis cannot tell.
    fn verdict(
        self,
        node: &FNode,
        subject: F,
    ) -> Option<bool> {
        return match self.span(subject) {
            None => Some(eval(node, self.witness())),
            Some((first, last)) => self.span_verdict(node, first, last),
        };
    }

    /// Sound over the values `first..=last`: interval and `!=` tests on the
    /// value itself are decided, predicates are constant on the class, and
    /// everything else is unknown.
    fn span_verdict(
        self,
        node: &FNode,
        first: f64,
        last: f64,
    ) -> Option<bool> {
        let verdict = |it: &FNode| self.span_verdict(it, first, last);
        return match node {
            FNode::Const(it) => Some(*it),
            FNode::Or(items) => kleene(items.iter().map(verdict), true),
            FNode::And(items) => kleene(items.iter().map(verdict), false),
            FNode::Not(inner) => verdict(inner).map(|it| !it),
            FNode::Interval { operand, lo, hi } if operand.is_subject() => {
                let below = hi.as_ref().is_some_and(|hi| {
                    hi.value < first || (hi.value == first && !hi.inclusive)
                });
                let above = lo.as_ref().is_some_and(|lo| {
                    lo.value > last || (lo.value == last && !lo.inclusive)
                });
                if below || above {
                    Some(false)
                }
                // Intervals are convex.
                else if eval(node, first) && eval(node, last) {
                    Some(true)
                }
                else {
                    None
                }
            }
            FNode::Ne { operand, value, .. } if operand.is_subject() => {
                (*value < first || *value > last).then_some(true)
            }
            FNode::Predicate { function, .. } => {
                Some(predicate(*function, self.witness()))
            }
            _ => None,
        };
    }
}

fn operand_tokens(
    operand: &Operand,
    value: &TokenStream,
    subject: F,
    math: MathProvider,
) -> TokenStream {
    let Some(call) = &operand.call
    else {
        return quote! { #value };
    };
    let name = &call.name;
    let args = call.args.iter().map(|it| quote! { (#it) });
    return match (call.provided, math) {
        (Provided::Provider, MathProvider::Nightly) => {
            let ty = format_ident!("{}", subject.rust_name());
            quote! { ::core::#ty::math::#name(#value #(, #args)*) }
        }
        _ => quote! { (#value).#name(#(#args),*) },
    };
}

fn emit_operand(
    node: &FNode,
    value: &TokenStream,
    subject: F,
    math: MathProvider,
) -> TokenStream {
    let it = emit(node, value, subject, math);
    return match node {
        FNode::Or(_) | FNode::And(_) => quote! { (#it) },
        _ => it,
    };
}

fn emit(
    node: &FNode,
    value: &TokenStream,
    subject: F,
    math: MathProvider,
) -> TokenStream {
    return match node {
        FNode::Const(true) => quote! { true },
        FNode::Const(false) => quote! { false },
        FNode::Or(items) => {
            let items = items
                .iter()
                .map(|it| emit_operand(it, value, subject, math));
            quote! { #(#items)||* }
        }
        FNode::And(items) => {
            let items = items
                .iter()
                .map(|it| emit_operand(it, value, subject, math));
            quote! { #(#items)&&* }
        }
        FNode::Not(inner) => {
            let inner = emit_operand(inner, value, subject, math);
            quote! { (!#inner) }
        }
        FNode::Interval { operand, lo, hi } => {
            let it = operand_tokens(operand, value, subject, math);
            if let (Some(lo), Some(hi)) = (lo, hi)
                && lo.value == hi.value
                && lo.inclusive
                && hi.inclusive
            {
                let tokens = &lo.tokens;
                return quote! { (#it == (#tokens)) };
            }
            let lo = lo.as_ref().map(|lo| {
                let tokens = &lo.tokens;
                match lo.inclusive {
                    true => quote! { (#it >= (#tokens)) },
                    false => quote! { (#it > (#tokens)) },
                }
            });
            let hi = hi.as_ref().map(|hi| {
                let tokens = &hi.tokens;
                match hi.inclusive {
                    true => quote! { (#it <= (#tokens)) },
                    false => quote! { (#it < (#tokens)) },
                }
            });
            match (lo, hi) {
                (Some(lo), Some(hi)) => quote! { (#lo && #hi) },
                (Some(it), None) | (None, Some(it)) => it,
                (None, None) => quote! { (!(#it).is_nan()) },
            }
        }
        FNode::Ne {
            operand, tokens, ..
        } => {
            let it = operand_tokens(operand, value, subject, math);
            quote! { (#it != (#tokens)) }
        }
        FNode::Divisible {
            operand,
            tokens,
            negated,
            ..
        } => {
            let it = operand_tokens(operand, value, subject, math);
            match negated {
                false => quote! { ((#it % (#tokens)) == 0.0) },
                true => quote! { ((#it % (#tokens)) != 0.0) },
            }
        }
        FNode::Predicate { name, .. } => quote! { ((#value).#name()) },
    };
}
