use std::cmp::Ordering;
use std::fmt::{
    Display,
    Formatter,
};

use proc_macro2::TokenStream;
use quote::quote;

use crate::zz_domain_closure::ClosureOp;
use crate::zz_lib::{
    Konsts,
    Named,
};

/// A supported pointer width. Per-width data is a `[T; Width::COUNT]` in
/// `ALL` order, indexed by `index()`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Width {
    W16,
    W32,
    W64,
}

impl Width {
    pub(crate) const ALL: [Self; 3] = [
        Self::W16,
        Self::W32,
        Self::W64,
    ];
    pub(crate) const COUNT: usize = Self::ALL.len();

    pub(crate) const fn index(self) -> usize {
        return self as usize;
    }

    pub(crate) const fn bits(self) -> u32 {
        return match self {
            Self::W16 => 16,
            Self::W32 => 32,
            Self::W64 => 64,
        };
    }
}

/// The width in bits, as `target_pointer_width` spells it.
impl Display for Width {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return write!(f, "{}", self.bits());
    }
}

/// The `cfg` attribute selecting `widths`: none for every width, `any(...)`
/// for several.
fn width_cfg(widths: &[Width]) -> TokenStream {
    let widths = widths.iter().map(Width::to_string).collect::<Vec<_>>();
    return match widths.as_slice() {
        [width] => quote! { #[cfg(target_pointer_width = #width)] },
        _ if widths.len() == Width::COUNT => TokenStream::new(),
        _ => quote! { #[cfg(any(#(target_pointer_width = #widths),*))] },
    };
}

/// Per-width tokens with the `cfg` attribute to emit each under: one entry
/// without a `cfg` when every width has the same tokens, otherwise one entry
/// per width.
pub(crate) fn by_width(
    values: [TokenStream; Width::COUNT]
) -> Vec<(TokenStream, TokenStream)> {
    let first = values[0].to_string();
    if values.iter().all(|it| it.to_string() == first) {
        return vec![(TokenStream::new(), values[0].clone())];
    }
    return Width::ALL
        .into_iter()
        .zip(values)
        .map(|(width, value)| (width_cfg(&[width]), value))
        .collect();
}

/// Each distinct per-width value once, in order of its first width, with the
/// `cfg` attribute of the widths that have it (see `width_cfg`).
pub(crate) fn by_width_grouped<T: PartialEq>(
    values: [T; Width::COUNT]
) -> Vec<(TokenStream, T)> {
    let mut groups = Vec::<(Vec<Width>, T)>::new();
    for (width, value) in Width::ALL.into_iter().zip(values) {
        match groups.iter_mut().find(|(_, it)| *it == value) {
            Some((widths, _)) => widths.push(width),
            None => groups.push((vec![width], value)),
        }
    }
    return groups
        .into_iter()
        .map(|(widths, value)| (width_cfg(&widths), value))
        .collect();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum N {
    USIZ,
    ISIZ,
    U008,
    U016,
    U032,
    U064,
    U128,
    I008,
    I016,
    I032,
    I064,
    I128,
}

impl Named for N {
    const TABLE: &'static [(Self, &'static str)] = &[
        (Self::USIZ, "usize"),
        (Self::ISIZ, "isize"),
        (Self::U008, "u8"),
        (Self::U016, "u16"),
        (Self::U032, "u32"),
        (Self::U064, "u64"),
        (Self::U128, "u128"),
        (Self::I008, "i8"),
        (Self::I016, "i16"),
        (Self::I032, "i32"),
        (Self::I064, "i64"),
        (Self::I128, "i128"),
    ];

    fn index(self) -> usize {
        return self as usize;
    }
}

// Every row of `TABLE` is at its variant's index.
const _: () = {
    let table = N::TABLE;
    let mut index = 0;
    while index < table.len() {
        assert!(table[index].0 as usize == index);
        index += 1;
    }
};

impl N {
    /// The unsigned type of the same width; itself when unsigned.
    pub(crate) fn unsigned(self) -> N {
        return match self {
            N::USIZ | N::ISIZ => N::USIZ,
            N::U008 | N::I008 => N::U008,
            N::U016 | N::I016 => N::U016,
            N::U032 | N::I032 => N::U032,
            N::U064 | N::I064 => N::U064,
            N::U128 | N::I128 => N::U128,
        };
    }

    /// The signed type of the same width; itself when signed.
    pub(crate) fn signed(self) -> N {
        return match self {
            N::USIZ | N::ISIZ => N::ISIZ,
            N::U008 | N::I008 => N::I008,
            N::U016 | N::I016 => N::I016,
            N::U032 | N::I032 => N::I032,
            N::U064 | N::I064 => N::I064,
            N::U128 | N::I128 => N::I128,
        };
    }

    pub(crate) fn non_zero_rust_name(&self) -> &'static str {
        return match self {
            N::USIZ => "NonZeroUsize",
            N::ISIZ => "NonZeroIsize",
            N::U008 => "NonZeroU8",
            N::U016 => "NonZeroU16",
            N::U032 => "NonZeroU32",
            N::U064 => "NonZeroU64",
            N::U128 => "NonZeroU128",
            N::I008 => "NonZeroI8",
            N::I016 => "NonZeroI16",
            N::I032 => "NonZeroI32",
            N::I064 => "NonZeroI64",
            N::I128 => "NonZeroI128",
        };
    }

    pub(crate) fn bits_for_pointer_width(
        self,
        width: Width,
    ) -> usize {
        return match self {
            N::USIZ | N::ISIZ => width.bits(),
            N::U008 | N::I008 => 8,
            N::U016 | N::I016 => 16,
            N::U032 | N::I032 => 32,
            N::U064 | N::I064 => 64,
            N::U128 | N::I128 => 128,
        } as usize;
    }

    pub fn is_signed(&self) -> bool {
        return match self {
            N::USIZ => false,
            N::ISIZ => true,
            N::U008 => false,
            N::U016 => false,
            N::U032 => false,
            N::U064 => false,
            N::U128 => false,
            N::I008 => true,
            N::I016 => true,
            N::I032 => true,
            N::I064 => true,
            N::I128 => true,
        };
    }
}

/// An integer type at one pointer width.
#[derive(Clone, Copy)]
pub(crate) struct IntTy {
    pub(crate) signed: bool,
    pub(crate) bits: u32,
    pub(crate) min: Int,
    pub(crate) max: Int,
}

impl IntTy {
    pub(crate) fn contains(
        self,
        value: Int,
    ) -> bool {
        return self.min <= value && value <= self.max;
    }

    /// Every bit of the type set.
    pub(crate) fn mask(self) -> u128 {
        return match self.bits {
            u128::BITS => u128::MAX,
            bits => (1u128 << bits) - 1,
        };
    }

    /// The value of a two's-complement bit pattern reduced to this type.
    pub(crate) fn wrap(
        self,
        pattern: u128,
    ) -> Int {
        let masked = pattern & self.mask();
        if !self.signed || (masked >> (self.bits - 1)) & 1 == 0 {
            return Int::NonNegative(masked);
        }
        return Int::Negative((!masked & self.mask()) + 1);
    }
}

impl N {
    pub(crate) fn at(
        self,
        width: Width,
    ) -> IntTy {
        let signed = self.is_signed();
        let bits = self.bits_for_pointer_width(width) as u32;
        // `2^(bits - 1)`: the magnitude of a signed minimum.
        let half = 1u128 << (bits - 1);
        let (min, max) = match signed {
            true => (Int::Negative(half), Int::NonNegative(half - 1)),
            false => (Int::NonNegative(0), Int::NonNegative(half - 1 + half)),
        };
        return IntTy {
            signed,
            bits,
            min,
            max,
        };
    }

    pub(crate) fn bounds_for_pointer_width(
        self,
        width: Width,
    ) -> (Int, Int) {
        let ty = self.at(width);
        return (ty.min, ty.max);
    }

    pub(crate) fn can_safe_cast_to(
        self,
        to: Self,
        width: Width,
    ) -> bool {
        let (source, target) = (self.at(width), to.at(width));
        return target.min <= source.min && source.max <= target.max;
    }
}

/// Magnitude bits of an integer type: every value fits a float whose
/// mantissa has at least this many digits.
impl N {
    pub(crate) fn magnitude_bits_for_pointer_width(
        self,
        width: Width,
    ) -> u32 {
        let bits = self.bits_for_pointer_width(width) as u32;
        return match self.is_signed() {
            true => bits - 1,
            false => bits,
        };
    }
}

// =============================================================================

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum F {
    F032,
    F064,
}

impl Named for F {
    const TABLE: &'static [(Self, &'static str)] = &[
        (Self::F032, "f32"),
        (Self::F064, "f64"),
    ];

    fn index(self) -> usize {
        return self as usize;
    }
}

// Every row of `TABLE` is at its variant's index.
const _: () = {
    let table = F::TABLE;
    let mut index = 0;
    while index < table.len() {
        assert!(table[index].0 as usize == index);
        index += 1;
    }
};

impl F {
    pub fn bits(&self) -> u32 {
        return match self {
            Self::F032 => 32,
            Self::F064 => 64,
        };
    }

    /// `MANTISSA_DIGITS`: significand bits including the implicit one.
    pub fn mantissa_digits(&self) -> u32 {
        return match self {
            Self::F032 => f32::MANTISSA_DIGITS,
            Self::F064 => f64::MANTISSA_DIGITS,
        };
    }

    pub(crate) fn can_safe_cast_to(
        self,
        to: Self,
    ) -> bool {
        return self.bits() <= to.bits();
    }

    /// Whether every value of the integer type converts to this float
    /// exactly, at the given pointer width.
    pub(crate) fn represents_every(
        self,
        int: N,
        width: Width,
    ) -> bool {
        return int.magnitude_bits_for_pointer_width(width)
            <= self.mantissa_digits();
    }
}

/// Exact decimal float literal of `2^exponent`, e.g. `2^3` is `"8.0"`.
///
/// Every power of two in range is exactly representable in `f32` and `f64`,
/// so the literal is exact and needs no rounding by rustc.
pub(crate) fn pow2_literal(exponent: u32) -> String {
    // Little-endian decimal digits, doubled `exponent` times.
    let mut digits = vec![1u8];
    for _ in 0..exponent {
        let mut carry = 0;
        for digit in &mut digits {
            let doubled = *digit * 2 + carry;
            *digit = doubled % 10;
            carry = doubled / 10;
        }
        if carry > 0 {
            digits.push(carry);
        }
    }
    let mut result = digits
        .iter()
        .rev()
        .map(|it| char::from(b'0' + it))
        .collect::<String>();
    result.push_str(".0");
    return result;
}

// =============================================================================

/// A primitive numeric value-field type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum Num {
    Int(N),
    Float(F),
}

impl From<N> for Num {
    fn from(it: N) -> Self {
        return Self::Int(it);
    }
}

impl From<F> for Num {
    fn from(it: F) -> Self {
        return Self::Float(it);
    }
}

impl Num {
    pub fn of(name: impl AsRef<str>) -> Option<Self> {
        let name = name.as_ref();
        return N::of(name)
            .map(Self::Int)
            .or_else(|| F::of(name).map(Self::Float));
    }

    pub fn items() -> impl Iterator<Item = Self> {
        return N::items().map(Self::Int).chain(F::items().map(Self::Float));
    }

    pub fn name(&self) -> &'static str {
        return match self {
            Self::Int(it) => it.name(),
            Self::Float(it) => it.name(),
        };
    }

    pub fn int(&self) -> Option<N> {
        return match self {
            Self::Int(it) => Some(*it),
            Self::Float(_) => None,
        };
    }

    /// Lossless `as` conversion at the given pointer width.
    pub(crate) fn can_safe_cast_to(
        self,
        to: Self,
        width: Width,
    ) -> bool {
        return match (self, to) {
            (Self::Int(from), Self::Int(to)) => {
                from.can_safe_cast_to(to, width)
            }
            (Self::Int(from), Self::Float(to)) => {
                to.represents_every(from, width)
            }
            (Self::Float(_), Self::Int(_)) => false,
            (Self::Float(from), Self::Float(to)) => from.can_safe_cast_to(to),
        };
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Int {
    Negative(u128),
    NonNegative(u128),
}

impl Ord for Int {
    fn cmp(
        &self,
        that: &Self,
    ) -> Ordering {
        return match (self, that) {
            (Self::Negative(_), Self::NonNegative(_)) => Ordering::Less,
            (Self::NonNegative(_), Self::Negative(_)) => Ordering::Greater,
            (Self::NonNegative(l), Self::NonNegative(r)) => l.cmp(r),
            (Self::Negative(l), Self::Negative(r)) => r.cmp(l),
        };
    }
}

impl PartialOrd for Int {
    fn partial_cmp(
        &self,
        that: &Self,
    ) -> Option<Ordering> {
        return Some(self.cmp(that));
    }
}

impl Display for Int {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return match self {
            Self::Negative(value) => write!(f, "-{value}"),
            Self::NonNegative(value) => write!(f, "{value}"),
        };
    }
}

impl std::fmt::Debug for Int {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        return Display::fmt(self, f);
    }
}

impl Int {
    pub(crate) fn negate(self) -> Self {
        return match self {
            Self::Negative(value) => Self::NonNegative(value),
            Self::NonNegative(0) => Self::NonNegative(0),
            Self::NonNegative(value) => Self::Negative(value),
        };
    }

    pub(crate) fn predecessor(self) -> Option<Self> {
        return match self {
            Self::Negative(u128::MAX) => None,
            Self::Negative(value) => Some(Self::Negative(value + 1)),
            Self::NonNegative(0) => Some(Self::Negative(1)),
            Self::NonNegative(value) => Some(Self::NonNegative(value - 1)),
        };
    }

    pub(crate) fn successor(self) -> Option<Self> {
        return match self {
            Self::Negative(1) => Some(Self::NonNegative(0)),
            Self::Negative(value) => Some(Self::Negative(value - 1)),
            Self::NonNegative(u128::MAX) => None,
            Self::NonNegative(value) => Some(Self::NonNegative(value + 1)),
        };
    }
}

// Exact arithmetic; results beyond the `u128` magnitude return `None`.
impl Int {
    /// Canonical value: zero is never `Negative`.
    pub(crate) fn from_sign_magnitude(
        negative: bool,
        magnitude: u128,
    ) -> Self {
        return match negative && magnitude != 0 {
            true => Self::Negative(magnitude),
            false => Self::NonNegative(magnitude),
        };
    }

    pub(crate) fn magnitude(self) -> u128 {
        return match self {
            Self::Negative(value) | Self::NonNegative(value) => value,
        };
    }

    pub(crate) fn is_negative(self) -> bool {
        return matches!(self, Self::Negative(_));
    }

    pub(crate) fn checked_add(
        self,
        rhs: Self,
    ) -> Option<Self> {
        let (left, right) = (self.magnitude(), rhs.magnitude());
        if self.is_negative() == rhs.is_negative() {
            return Some(Self::from_sign_magnitude(
                self.is_negative(),
                left.checked_add(right)?,
            ));
        }
        return Some(match left >= right {
            true => Self::from_sign_magnitude(self.is_negative(), left - right),
            false => Self::from_sign_magnitude(rhs.is_negative(), right - left),
        });
    }

    pub(crate) fn checked_sub(
        self,
        rhs: Self,
    ) -> Option<Self> {
        return self.checked_add(rhs.negate());
    }

    pub(crate) fn checked_mul(
        self,
        rhs: Self,
    ) -> Option<Self> {
        return Some(Self::from_sign_magnitude(
            self.is_negative() != rhs.is_negative(),
            self.magnitude().checked_mul(rhs.magnitude())?,
        ));
    }

    /// Division rounding towards zero; `None` for a zero divisor.
    pub(crate) fn checked_div(
        self,
        rhs: Self,
    ) -> Option<Self> {
        return Some(Self::from_sign_magnitude(
            self.is_negative() != rhs.is_negative(),
            self.magnitude().checked_div(rhs.magnitude())?,
        ));
    }

    /// Remainder of `checked_div`, with the sign of `self`.
    pub(crate) fn checked_rem(
        self,
        rhs: Self,
    ) -> Option<Self> {
        return Some(Self::from_sign_magnitude(
            self.is_negative(),
            self.magnitude().checked_rem(rhs.magnitude())?,
        ));
    }

    /// Euclidean division: the remainder is never negative.
    pub(crate) fn checked_div_euclid(
        self,
        rhs: Self,
    ) -> Option<Self> {
        let quotient = self.checked_div(rhs)?;
        if !self.checked_rem(rhs)?.is_negative() {
            return Some(quotient);
        }
        return match rhs.is_negative() {
            true => quotient.checked_add(Self::NonNegative(1)),
            false => quotient.checked_sub(Self::NonNegative(1)),
        };
    }

    pub(crate) fn checked_rem_euclid(
        self,
        rhs: Self,
    ) -> Option<Self> {
        let remainder = self.checked_rem(rhs)?;
        return match remainder.is_negative() {
            true => remainder.checked_add(Self::NonNegative(rhs.magnitude())),
            false => Some(remainder),
        };
    }

    /// Bits needed to write a non-negative value; `None` when negative.
    pub(crate) fn bitlen(self) -> Option<u32> {
        return match self {
            Self::Negative(_) => None,
            Self::NonNegative(value) => {
                Some(u128::BITS - value.leading_zeros())
            }
        };
    }

    /// Floor square root; `None` when negative.
    pub(crate) fn isqrt(self) -> Option<Self> {
        return match self {
            Self::Negative(_) => None,
            Self::NonNegative(value) => Some(Self::NonNegative(value.isqrt())),
        };
    }

    /// Two's-complement bit pattern of a value, modulo 2^128.
    pub(crate) fn pattern(self) -> u128 {
        return match self {
            Int::NonNegative(it) => it,
            Int::Negative(it) => it.wrapping_neg(),
        };
    }

    pub(crate) fn to_i128(self) -> i128 {
        return self.pattern() as i128;
    }

    pub(crate) fn from_i128(value: i128) -> Int {
        return Int::from_sign_magnitude(value < 0, value.unsigned_abs());
    }

    /// `floor(value / 2^amount)`: an arithmetic right shift without a width.
    pub(crate) fn shr_floor(
        self,
        amount: u32,
    ) -> Int {
        return match self {
            Int::NonNegative(it) => {
                Int::NonNegative(it.checked_shr(amount).unwrap_or(0))
            }
            // floor(-m / 2^s) = -ceil(m / 2^s) = -(((m - 1) >> s) + 1), m >= 1
            Int::Negative(it) => {
                Int::Negative((it - 1).checked_shr(amount).unwrap_or(0) + 1)
            }
        };
    }

    /// `2^bitlen(value) - 1` for a non-negative value: every bit pattern no
    /// longer than `value`.
    pub(crate) fn mask(self) -> Int {
        let bits = self.bitlen().expect("mask of a non-negative value");
        return Int::NonNegative(match bits {
            u128::BITS => u128::MAX,
            bits => (1u128 << bits) - 1,
        });
    }

    /// `!value = -value - 1`, for a value of any type.
    pub(crate) fn bitwise_not(self) -> Int {
        return self
            .negate()
            .checked_sub(Int::NonNegative(1))
            .expect("`!` of a primitive value fits");
    }
}

/// A set of values: sorted, disjoint, non-adjacent inclusive intervals.
#[derive(Clone)]
pub(crate) struct IntervalSet(Vec<(Int, Int)>);

impl IntervalSet {
    /// The union of `intervals`; one whose ends are out of order is empty.
    pub(crate) fn new(mut intervals: Vec<(Int, Int)>) -> Self {
        intervals.retain(|(lo, hi)| lo <= hi);
        intervals.sort();
        let mut result = Vec::<(Int, Int)>::with_capacity(intervals.len());
        for (lo, hi) in intervals {
            if let Some(last) = result.last_mut()
                && last.1.successor().is_none_or(|it| lo <= it)
            {
                last.1 = last.1.max(hi);
                continue;
            }
            result.push((lo, hi));
        }
        return Self(result);
    }

    pub(crate) fn contains(
        &self,
        value: Int,
    ) -> bool {
        return self.0.iter().any(|(lo, hi)| *lo <= value && value <= *hi);
    }

    /// One interval holds all of `lo..=hi`.
    pub(crate) fn encloses(
        &self,
        (lo, hi): (Int, Int),
    ) -> bool {
        return self.0.iter().any(|(dl, dh)| *dl <= lo && hi <= *dh);
    }
}

/// The intervals, in order.
impl std::ops::Deref for IntervalSet {
    type Target = [(Int, Int)];

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

// =============================================================================

/// An operator typekin generates. The variants are in the canonical
/// operator order: the order of `OPS`, and of everything generated per
/// operator.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Neg,
    Not,
}

impl Op {
    /// The operator's row of `OPS`.
    pub(crate) fn spec(self) -> &'static OpSpec {
        return &OPS[self as usize];
    }
}

/// What an operator takes on its right-hand side.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OpKind {
    /// A value, combined under the profile's arithmetic: `+ - * / %`.
    Arith,
    /// A value, combined bit by bit: `& | ^`.
    Bitwise,
    /// A `usize` amount: `<< >>`.
    Shift,
    /// Nothing: `-x`, `!x`.
    Unary,
}

impl OpKind {
    /// The right-hand side is a value (same type, raw, or friend): the
    /// operators that facets dispatch by slot.
    pub(crate) fn has_operand(self) -> bool {
        return matches!(self, Self::Arith | Self::Bitwise);
    }
}

/// One operator: its `core::ops` trait and method, its `konst_*` flags, its
/// closure-proof operation and its symbol.
pub(crate) struct OpSpec {
    pub(crate) op: Op,
    /// `Add`; the assignment trait is `AddAssign`.
    pub(crate) trait_name: &'static str,
    /// `add`; the assignment method is `add_assign`.
    pub(crate) method: &'static str,
    pub(crate) kind: OpKind,
    /// The family's `konst_*` flag: `Math` or `Bit`.
    pub(crate) family: Konsts,
    pub(crate) konst: Konsts,
    /// The assignment's own `konst_*` flag; unary operators have no
    /// assignment.
    pub(crate) konst_assign: Option<Konsts>,
    /// What a closure proof asks about; `<<` is never proven.
    pub(crate) closure: Option<ClosureOp>,
    /// `+`: the operator token, also shown by `prove` diagnostics.
    pub(crate) symbol: &'static str,
}

impl OpSpec {
    /// The operator token: `+`, `<<`, `!`, …
    pub(crate) fn token(&self) -> TokenStream {
        return self.symbol.parse().expect("an operator symbol is a token");
    }

    /// The `konst_*` flags of the operator, first decisive: `[X, family]`.
    pub(crate) fn konsts(&self) -> [Konsts; 2] {
        return [
            self.konst,
            self.family,
        ];
    }

    /// The `konst_*` flags of the assignment operator, first decisive:
    /// `[XAssign, X, Assign, family]`.
    pub(crate) fn assign_konsts(&self) -> [Konsts; 4] {
        return [
            self.konst_assign
                .expect("a binary operator has an assignment"),
            self.konst,
            Konsts::Assign,
            self.family,
        ];
    }
}

/// The operators, in the canonical order; every site that generates or
/// checks something per operator iterates this table.
pub(crate) const OPS: &[OpSpec] = &[
    OpSpec {
        op: Op::Add,
        trait_name: "Add",
        method: "add",
        kind: OpKind::Arith,
        family: Konsts::Math,
        konst: Konsts::Add,
        konst_assign: Some(Konsts::AddAssign),
        closure: Some(ClosureOp::Add),
        symbol: "+",
    },
    OpSpec {
        op: Op::Sub,
        trait_name: "Sub",
        method: "sub",
        kind: OpKind::Arith,
        family: Konsts::Math,
        konst: Konsts::Sub,
        konst_assign: Some(Konsts::SubAssign),
        closure: Some(ClosureOp::Sub),
        symbol: "-",
    },
    OpSpec {
        op: Op::Mul,
        trait_name: "Mul",
        method: "mul",
        kind: OpKind::Arith,
        family: Konsts::Math,
        konst: Konsts::Mul,
        konst_assign: Some(Konsts::MulAssign),
        closure: Some(ClosureOp::Mul),
        symbol: "*",
    },
    OpSpec {
        op: Op::Div,
        trait_name: "Div",
        method: "div",
        kind: OpKind::Arith,
        family: Konsts::Math,
        konst: Konsts::Div,
        konst_assign: Some(Konsts::DivAssign),
        closure: Some(ClosureOp::Div),
        symbol: "/",
    },
    OpSpec {
        op: Op::Rem,
        trait_name: "Rem",
        method: "rem",
        kind: OpKind::Arith,
        family: Konsts::Math,
        konst: Konsts::Rem,
        konst_assign: Some(Konsts::RemAssign),
        closure: Some(ClosureOp::Rem),
        symbol: "%",
    },
    OpSpec {
        op: Op::BitAnd,
        trait_name: "BitAnd",
        method: "bitand",
        kind: OpKind::Bitwise,
        family: Konsts::Bit,
        konst: Konsts::And,
        konst_assign: Some(Konsts::AndAssign),
        closure: Some(ClosureOp::BitAnd),
        symbol: "&",
    },
    OpSpec {
        op: Op::BitOr,
        trait_name: "BitOr",
        method: "bitor",
        kind: OpKind::Bitwise,
        family: Konsts::Bit,
        konst: Konsts::Or,
        konst_assign: Some(Konsts::OrAssign),
        closure: Some(ClosureOp::BitOr),
        symbol: "|",
    },
    OpSpec {
        op: Op::BitXor,
        trait_name: "BitXor",
        method: "bitxor",
        kind: OpKind::Bitwise,
        family: Konsts::Bit,
        konst: Konsts::Xor,
        konst_assign: Some(Konsts::XorAssign),
        closure: Some(ClosureOp::BitXor),
        symbol: "^",
    },
    OpSpec {
        op: Op::Shl,
        trait_name: "Shl",
        method: "shl",
        kind: OpKind::Shift,
        family: Konsts::Math,
        konst: Konsts::Shl,
        konst_assign: Some(Konsts::ShlAssign),
        closure: None,
        symbol: "<<",
    },
    OpSpec {
        op: Op::Shr,
        trait_name: "Shr",
        method: "shr",
        kind: OpKind::Shift,
        family: Konsts::Math,
        konst: Konsts::Shr,
        konst_assign: Some(Konsts::ShrAssign),
        closure: Some(ClosureOp::Shr),
        symbol: ">>",
    },
    OpSpec {
        op: Op::Neg,
        trait_name: "Neg",
        method: "neg",
        kind: OpKind::Unary,
        family: Konsts::Math,
        konst: Konsts::Neg,
        konst_assign: None,
        closure: Some(ClosureOp::Neg),
        symbol: "-",
    },
    OpSpec {
        op: Op::Not,
        trait_name: "Not",
        method: "not",
        kind: OpKind::Unary,
        family: Konsts::Bit,
        konst: Konsts::Not,
        konst_assign: None,
        closure: Some(ClosureOp::Not),
        symbol: "!",
    },
];

// Every operator has a row of `OPS`, at its index.
const _: () = {
    assert!(OPS.len() == Op::Not as usize + 1);
    let mut index = 0;
    while index < OPS.len() {
        assert!(OPS[index].op as usize == index);
        index += 1;
    }
};

// =============================================================================

/// A function typekin generates by forwarding to `core` (text: `String`),
/// or a function of the `domain` language. Variants are in name order, so
/// ordered collections of names keep their textual order.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum FnName {
    Abs,
    AbsDiff,
    Acos,
    Acosh,
    AsBytes,
    AsStr,
    Asin,
    Asinh,
    Atan,
    Atan2,
    Atanh,
    BitWidth,
    BorrowingSub,
    Capacity,
    CarryingAdd,
    CarryingMul,
    CarryingMulAdd,
    CastSigned,
    CastUnsigned,
    Cbrt,
    Ceil,
    CheckedAbs,
    CheckedAdd,
    CheckedDiv,
    CheckedDivEuclid,
    CheckedIlog,
    CheckedIlog10,
    CheckedIlog2,
    CheckedIsqrt,
    CheckedMul,
    CheckedNeg,
    CheckedNextMultipleOf,
    CheckedNextPowerOfTwo,
    CheckedPow,
    CheckedRem,
    CheckedRemEuclid,
    CheckedShl,
    CheckedShr,
    CheckedSub,
    Clamp,
    Classify,
    Clear,
    Copysign,
    Cos,
    Cosh,
    CountOnes,
    CountZeros,
    DivCeil,
    DivEuclid,
    Exp,
    Exp2,
    ExpM1,
    Floor,
    Fract,
    FromBe,
    FromBeBytes,
    FromBits,
    FromLe,
    FromLeBytes,
    FromNeBytes,
    FunnelShl,
    FunnelShr,
    HighestOne,
    Hypot,
    Ilog,
    Ilog10,
    Ilog2,
    Insert,
    InsertStr,
    IntoBytes,
    IsEmpty,
    IsFinite,
    IsInfinite,
    IsMultipleOf,
    IsNan,
    IsNegative,
    IsNormal,
    IsPositive,
    IsPowerOfTwo,
    IsSignNegative,
    IsSignPositive,
    IsSubnormal,
    IsolateHighestOne,
    IsolateLowestOne,
    Isqrt,
    LeadingOnes,
    LeadingZeros,
    Len,
    Ln,
    Ln1p,
    Log,
    Log10,
    Log2,
    LowestOne,
    Max,
    Midpoint,
    Min,
    MulAdd,
    NextDown,
    NextMultipleOf,
    NextPowerOfTwo,
    NextUp,
    OverflowingAbs,
    OverflowingAdd,
    OverflowingDiv,
    OverflowingDivEuclid,
    OverflowingMul,
    OverflowingNeg,
    OverflowingPow,
    OverflowingRem,
    OverflowingRemEuclid,
    OverflowingShl,
    OverflowingShr,
    OverflowingSub,
    Pop,
    Pow,
    Powf,
    Powi,
    Push,
    PushStr,
    Recip,
    RemEuclid,
    Remove,
    ReplaceRange,
    Retain,
    ReverseBits,
    RotateLeft,
    RotateRight,
    Round,
    RoundTiesEven,
    SaturatingAbs,
    SaturatingAdd,
    SaturatingDiv,
    SaturatingMul,
    SaturatingNeg,
    SaturatingPow,
    SaturatingSub,
    Signum,
    Sin,
    SinCos,
    Sinh,
    Sqrt,
    StrictAbs,
    StrictAdd,
    StrictDiv,
    StrictDivEuclid,
    StrictMul,
    StrictNeg,
    StrictPow,
    StrictRem,
    StrictRemEuclid,
    StrictShl,
    StrictShr,
    StrictSub,
    SwapBytes,
    Tan,
    Tanh,
    ToBe,
    ToBeBytes,
    ToBits,
    ToDegrees,
    ToLe,
    ToLeBytes,
    ToNeBytes,
    ToRadians,
    TotalCmp,
    TrailingOnes,
    TrailingZeros,
    Trunc,
    Truncate,
    UnboundedShl,
    UnboundedShr,
    UnsignedAbs,
    WrappingAbs,
    WrappingAdd,
    WrappingDiv,
    WrappingDivEuclid,
    WrappingMul,
    WrappingNeg,
    WrappingPow,
    WrappingRem,
    WrappingRemEuclid,
    WrappingShl,
    WrappingShr,
    WrappingSub,
}

impl Named for FnName {
    const TABLE: &'static [(Self, &'static str)] = &[
        (Self::Abs, "abs"),
        (Self::AbsDiff, "abs_diff"),
        (Self::Acos, "acos"),
        (Self::Acosh, "acosh"),
        (Self::AsBytes, "as_bytes"),
        (Self::AsStr, "as_str"),
        (Self::Asin, "asin"),
        (Self::Asinh, "asinh"),
        (Self::Atan, "atan"),
        (Self::Atan2, "atan2"),
        (Self::Atanh, "atanh"),
        (Self::BitWidth, "bit_width"),
        (Self::BorrowingSub, "borrowing_sub"),
        (Self::Capacity, "capacity"),
        (Self::CarryingAdd, "carrying_add"),
        (Self::CarryingMul, "carrying_mul"),
        (Self::CarryingMulAdd, "carrying_mul_add"),
        (Self::CastSigned, "cast_signed"),
        (Self::CastUnsigned, "cast_unsigned"),
        (Self::Cbrt, "cbrt"),
        (Self::Ceil, "ceil"),
        (Self::CheckedAbs, "checked_abs"),
        (Self::CheckedAdd, "checked_add"),
        (Self::CheckedDiv, "checked_div"),
        (Self::CheckedDivEuclid, "checked_div_euclid"),
        (Self::CheckedIlog, "checked_ilog"),
        (Self::CheckedIlog10, "checked_ilog10"),
        (Self::CheckedIlog2, "checked_ilog2"),
        (Self::CheckedIsqrt, "checked_isqrt"),
        (Self::CheckedMul, "checked_mul"),
        (Self::CheckedNeg, "checked_neg"),
        (Self::CheckedNextMultipleOf, "checked_next_multiple_of"),
        (Self::CheckedNextPowerOfTwo, "checked_next_power_of_two"),
        (Self::CheckedPow, "checked_pow"),
        (Self::CheckedRem, "checked_rem"),
        (Self::CheckedRemEuclid, "checked_rem_euclid"),
        (Self::CheckedShl, "checked_shl"),
        (Self::CheckedShr, "checked_shr"),
        (Self::CheckedSub, "checked_sub"),
        (Self::Clamp, "clamp"),
        (Self::Classify, "classify"),
        (Self::Clear, "clear"),
        (Self::Copysign, "copysign"),
        (Self::Cos, "cos"),
        (Self::Cosh, "cosh"),
        (Self::CountOnes, "count_ones"),
        (Self::CountZeros, "count_zeros"),
        (Self::DivCeil, "div_ceil"),
        (Self::DivEuclid, "div_euclid"),
        (Self::Exp, "exp"),
        (Self::Exp2, "exp2"),
        (Self::ExpM1, "exp_m1"),
        (Self::Floor, "floor"),
        (Self::Fract, "fract"),
        (Self::FromBe, "from_be"),
        (Self::FromBeBytes, "from_be_bytes"),
        (Self::FromBits, "from_bits"),
        (Self::FromLe, "from_le"),
        (Self::FromLeBytes, "from_le_bytes"),
        (Self::FromNeBytes, "from_ne_bytes"),
        (Self::FunnelShl, "funnel_shl"),
        (Self::FunnelShr, "funnel_shr"),
        (Self::HighestOne, "highest_one"),
        (Self::Hypot, "hypot"),
        (Self::Ilog, "ilog"),
        (Self::Ilog10, "ilog10"),
        (Self::Ilog2, "ilog2"),
        (Self::Insert, "insert"),
        (Self::InsertStr, "insert_str"),
        (Self::IntoBytes, "into_bytes"),
        (Self::IsEmpty, "is_empty"),
        (Self::IsFinite, "is_finite"),
        (Self::IsInfinite, "is_infinite"),
        (Self::IsMultipleOf, "is_multiple_of"),
        (Self::IsNan, "is_nan"),
        (Self::IsNegative, "is_negative"),
        (Self::IsNormal, "is_normal"),
        (Self::IsPositive, "is_positive"),
        (Self::IsPowerOfTwo, "is_power_of_two"),
        (Self::IsSignNegative, "is_sign_negative"),
        (Self::IsSignPositive, "is_sign_positive"),
        (Self::IsSubnormal, "is_subnormal"),
        (Self::IsolateHighestOne, "isolate_highest_one"),
        (Self::IsolateLowestOne, "isolate_lowest_one"),
        (Self::Isqrt, "isqrt"),
        (Self::LeadingOnes, "leading_ones"),
        (Self::LeadingZeros, "leading_zeros"),
        (Self::Len, "len"),
        (Self::Ln, "ln"),
        (Self::Ln1p, "ln_1p"),
        (Self::Log, "log"),
        (Self::Log10, "log10"),
        (Self::Log2, "log2"),
        (Self::LowestOne, "lowest_one"),
        (Self::Max, "max"),
        (Self::Midpoint, "midpoint"),
        (Self::Min, "min"),
        (Self::MulAdd, "mul_add"),
        (Self::NextDown, "next_down"),
        (Self::NextMultipleOf, "next_multiple_of"),
        (Self::NextPowerOfTwo, "next_power_of_two"),
        (Self::NextUp, "next_up"),
        (Self::OverflowingAbs, "overflowing_abs"),
        (Self::OverflowingAdd, "overflowing_add"),
        (Self::OverflowingDiv, "overflowing_div"),
        (Self::OverflowingDivEuclid, "overflowing_div_euclid"),
        (Self::OverflowingMul, "overflowing_mul"),
        (Self::OverflowingNeg, "overflowing_neg"),
        (Self::OverflowingPow, "overflowing_pow"),
        (Self::OverflowingRem, "overflowing_rem"),
        (Self::OverflowingRemEuclid, "overflowing_rem_euclid"),
        (Self::OverflowingShl, "overflowing_shl"),
        (Self::OverflowingShr, "overflowing_shr"),
        (Self::OverflowingSub, "overflowing_sub"),
        (Self::Pop, "pop"),
        (Self::Pow, "pow"),
        (Self::Powf, "powf"),
        (Self::Powi, "powi"),
        (Self::Push, "push"),
        (Self::PushStr, "push_str"),
        (Self::Recip, "recip"),
        (Self::RemEuclid, "rem_euclid"),
        (Self::Remove, "remove"),
        (Self::ReplaceRange, "replace_range"),
        (Self::Retain, "retain"),
        (Self::ReverseBits, "reverse_bits"),
        (Self::RotateLeft, "rotate_left"),
        (Self::RotateRight, "rotate_right"),
        (Self::Round, "round"),
        (Self::RoundTiesEven, "round_ties_even"),
        (Self::SaturatingAbs, "saturating_abs"),
        (Self::SaturatingAdd, "saturating_add"),
        (Self::SaturatingDiv, "saturating_div"),
        (Self::SaturatingMul, "saturating_mul"),
        (Self::SaturatingNeg, "saturating_neg"),
        (Self::SaturatingPow, "saturating_pow"),
        (Self::SaturatingSub, "saturating_sub"),
        (Self::Signum, "signum"),
        (Self::Sin, "sin"),
        (Self::SinCos, "sin_cos"),
        (Self::Sinh, "sinh"),
        (Self::Sqrt, "sqrt"),
        (Self::StrictAbs, "strict_abs"),
        (Self::StrictAdd, "strict_add"),
        (Self::StrictDiv, "strict_div"),
        (Self::StrictDivEuclid, "strict_div_euclid"),
        (Self::StrictMul, "strict_mul"),
        (Self::StrictNeg, "strict_neg"),
        (Self::StrictPow, "strict_pow"),
        (Self::StrictRem, "strict_rem"),
        (Self::StrictRemEuclid, "strict_rem_euclid"),
        (Self::StrictShl, "strict_shl"),
        (Self::StrictShr, "strict_shr"),
        (Self::StrictSub, "strict_sub"),
        (Self::SwapBytes, "swap_bytes"),
        (Self::Tan, "tan"),
        (Self::Tanh, "tanh"),
        (Self::ToBe, "to_be"),
        (Self::ToBeBytes, "to_be_bytes"),
        (Self::ToBits, "to_bits"),
        (Self::ToDegrees, "to_degrees"),
        (Self::ToLe, "to_le"),
        (Self::ToLeBytes, "to_le_bytes"),
        (Self::ToNeBytes, "to_ne_bytes"),
        (Self::ToRadians, "to_radians"),
        (Self::TotalCmp, "total_cmp"),
        (Self::TrailingOnes, "trailing_ones"),
        (Self::TrailingZeros, "trailing_zeros"),
        (Self::Trunc, "trunc"),
        (Self::Truncate, "truncate"),
        (Self::UnboundedShl, "unbounded_shl"),
        (Self::UnboundedShr, "unbounded_shr"),
        (Self::UnsignedAbs, "unsigned_abs"),
        (Self::WrappingAbs, "wrapping_abs"),
        (Self::WrappingAdd, "wrapping_add"),
        (Self::WrappingDiv, "wrapping_div"),
        (Self::WrappingDivEuclid, "wrapping_div_euclid"),
        (Self::WrappingMul, "wrapping_mul"),
        (Self::WrappingNeg, "wrapping_neg"),
        (Self::WrappingPow, "wrapping_pow"),
        (Self::WrappingRem, "wrapping_rem"),
        (Self::WrappingRemEuclid, "wrapping_rem_euclid"),
        (Self::WrappingShl, "wrapping_shl"),
        (Self::WrappingShr, "wrapping_shr"),
        (Self::WrappingSub, "wrapping_sub"),
    ];

    fn index(self) -> usize {
        return self as usize;
    }
}

// Every row of `TABLE` is at its variant's index.
const _: () = {
    let table = FnName::TABLE;
    let mut index = 0;
    while index < table.len() {
        assert!(table[index].0 as usize == index);
        index += 1;
    }
};
