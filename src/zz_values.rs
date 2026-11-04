use std::cmp::Ordering;
use std::fmt::{
    Display,
    Formatter,
};

use proc_macro2::TokenStream;
use quote::quote;

use crate::zz_lib::Konsts;

pub(crate) const POINTER_WIDTHS: [u32; 3] = [
    16,
    32,
    64,
];

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

impl N {
    const ITEMS: [Self; 12] = [
        Self::USIZ,
        Self::ISIZ,
        Self::U008,
        Self::U016,
        Self::U032,
        Self::U064,
        Self::U128,
        Self::I008,
        Self::I016,
        Self::I032,
        Self::I064,
        Self::I128,
    ];
}

impl N {
    pub fn of(name: impl AsRef<str>) -> Option<Self> {
        return match name.as_ref() {
            "isize" => Some(Self::ISIZ),
            "usize" => Some(Self::USIZ),
            "u8" => Some(Self::U008),
            "u16" => Some(Self::U016),
            "u32" => Some(Self::U032),
            "u64" => Some(Self::U064),
            "u128" => Some(Self::U128),
            "i8" => Some(Self::I008),
            "i16" => Some(Self::I016),
            "i32" => Some(Self::I032),
            "i64" => Some(Self::I064),
            "i128" => Some(Self::I128),
            _ => None,
        };
    }

    pub const fn items() -> &'static [Self] {
        return &Self::ITEMS;
    }
}

impl N {
    pub fn rust_name(&self) -> &'static str {
        return match self {
            N::USIZ => "usize",
            N::ISIZ => "isize",
            N::U008 => "u8",
            N::U016 => "u16",
            N::U032 => "u32",
            N::U064 => "u64",
            N::U128 => "u128",
            N::I008 => "i8",
            N::I016 => "i16",
            N::I032 => "i32",
            N::I064 => "i64",
            N::I128 => "i128",
        };
    }

    pub fn unsigned_rust_name(&self) -> &'static str {
        return match self {
            N::USIZ => "usize",
            N::ISIZ => "usize",
            N::U008 => "u8",
            N::U016 => "u16",
            N::U032 => "u32",
            N::U064 => "u64",
            N::U128 => "u128",
            N::I008 => "u8",
            N::I016 => "u16",
            N::I032 => "u32",
            N::I064 => "u64",
            N::I128 => "u128",
        };
    }

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

    pub fn signed_rust_name(&self) -> &'static str {
        return match self {
            N::USIZ | N::ISIZ => "isize",
            N::U008 | N::I008 => "i8",
            N::U016 | N::I016 => "i16",
            N::U032 | N::I032 => "i32",
            N::U064 | N::I064 => "i64",
            N::U128 | N::I128 => "i128",
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
        pointer_width: u32,
    ) -> usize {
        return match self {
            N::USIZ | N::ISIZ => pointer_width,
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

impl N {
    pub(crate) fn bounds_for_pointer_width(
        self,
        pointer_width: u32,
    ) -> (Int, Int) {
        assert!(POINTER_WIDTHS.contains(&pointer_width));
        return match self {
            Self::USIZ => (
                Int::NonNegative(0),
                Int::NonNegative((1u128 << pointer_width) - 1),
            ),
            Self::ISIZ => (
                Int::Negative(1u128 << (pointer_width - 1)),
                Int::NonNegative((1u128 << (pointer_width - 1)) - 1),
            ),
            Self::U008 => (
                Int::NonNegative(u8::MIN as u128),
                Int::NonNegative(u8::MAX as u128),
            ),
            Self::U016 => (
                Int::NonNegative(u16::MIN as u128),
                Int::NonNegative(u16::MAX as u128),
            ),
            Self::U032 => (
                Int::NonNegative(u32::MIN as u128),
                Int::NonNegative(u32::MAX as u128),
            ),
            Self::U064 => (
                Int::NonNegative(u64::MIN as u128),
                Int::NonNegative(u64::MAX as u128),
            ),
            Self::U128 => {
                (Int::NonNegative(u128::MIN), Int::NonNegative(u128::MAX))
            }
            Self::I008 => (
                Int::Negative((i8::MIN as i128).unsigned_abs()),
                Int::NonNegative(i8::MAX as u128),
            ),
            Self::I016 => (
                Int::Negative((i16::MIN as i128).unsigned_abs()),
                Int::NonNegative(i16::MAX as u128),
            ),
            Self::I032 => (
                Int::Negative((i32::MIN as i128).unsigned_abs()),
                Int::NonNegative(i32::MAX as u128),
            ),
            Self::I064 => (
                Int::Negative((i64::MIN as i128).unsigned_abs()),
                Int::NonNegative(i64::MAX as u128),
            ),
            Self::I128 => (
                Int::Negative(i128::MIN.unsigned_abs()),
                Int::NonNegative(i128::MAX as u128),
            ),
        };
    }

    pub(crate) fn can_safe_cast_to(
        self,
        to: Self,
        pointer_width: u32,
    ) -> bool {
        let (source_min, source_max) =
            self.bounds_for_pointer_width(pointer_width);
        let (target_min, target_max) =
            to.bounds_for_pointer_width(pointer_width);
        return target_min <= source_min && source_max <= target_max;
    }
}

/// Magnitude bits of an integer type: every value fits a float whose
/// mantissa has at least this many digits.
impl N {
    pub(crate) fn magnitude_bits_for_pointer_width(
        self,
        pointer_width: u32,
    ) -> u32 {
        let bits = self.bits_for_pointer_width(pointer_width) as u32;
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

impl F {
    const ITEMS: [Self; 2] = [
        Self::F032,
        Self::F064,
    ];

    pub fn of(name: impl AsRef<str>) -> Option<Self> {
        return match name.as_ref() {
            "f32" => Some(Self::F032),
            "f64" => Some(Self::F064),
            _ => None,
        };
    }

    pub const fn items() -> &'static [Self] {
        return &Self::ITEMS;
    }

    pub fn rust_name(&self) -> &'static str {
        return match self {
            Self::F032 => "f32",
            Self::F064 => "f64",
        };
    }

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
        pointer_width: u32,
    ) -> bool {
        return int.magnitude_bits_for_pointer_width(pointer_width)
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
        return N::items()
            .iter()
            .copied()
            .map(Self::Int)
            .chain(F::items().iter().copied().map(Self::Float));
    }

    pub fn rust_name(&self) -> &'static str {
        return match self {
            Self::Int(it) => it.rust_name(),
            Self::Float(it) => it.rust_name(),
        };
    }

    pub fn int(&self) -> Option<N> {
        return match self {
            Self::Int(it) => Some(*it),
            Self::Float(_) => None,
        };
    }

    pub fn float(&self) -> Option<F> {
        return match self {
            Self::Int(_) => None,
            Self::Float(it) => Some(*it),
        };
    }

    /// Lossless `as` conversion at the given pointer width.
    pub(crate) fn can_safe_cast_to(
        self,
        to: Self,
        pointer_width: u32,
    ) -> bool {
        return match (self, to) {
            (Self::Int(from), Self::Int(to)) => {
                from.can_safe_cast_to(to, pointer_width)
            }
            (Self::Int(from), Self::Float(to)) => {
                to.represents_every(from, pointer_width)
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

// =============================================================================

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Shl,
    Shr,
}

impl ArithmeticOp {
    pub(crate) fn name(self) -> &'static str {
        return match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Mul => "mul",
            Self::Div => "div",
            Self::Rem => "rem",
            Self::Shl => "shl",
            Self::Shr => "shr",
        };
    }

    /// The operator token: `+`, `-`, `*`, `/`, `%`, `<<`, `>>`.
    pub(crate) fn symbol(self) -> TokenStream {
        return match self {
            Self::Add => quote! { + },
            Self::Sub => quote! { - },
            Self::Mul => quote! { * },
            Self::Div => quote! { / },
            Self::Rem => quote! { % },
            Self::Shl => quote! { << },
            Self::Shr => quote! { >> },
        };
    }

    /// The binary operator behind `self`; the shifts are none.
    pub(crate) fn operator(self) -> Option<Operator> {
        return match self {
            Self::Add => Some(Operator::Add),
            Self::Sub => Some(Operator::Sub),
            Self::Mul => Some(Operator::Mul),
            Self::Div => Some(Operator::Div),
            Self::Rem => Some(Operator::Rem),
            Self::Shl | Self::Shr => None,
        };
    }
}

// =============================================================================

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
}

impl Operator {
    const ITEMS: [Self; 8] = [
        Self::Add,
        Self::Sub,
        Self::Mul,
        Self::Div,
        Self::Rem,
        Self::BitAnd,
        Self::BitOr,
        Self::BitXor,
    ];

    const MATH: [Self; 5] = [
        Self::Add,
        Self::Sub,
        Self::Mul,
        Self::Div,
        Self::Rem,
    ];

    const BIT: [Self; 3] = [
        Self::BitAnd,
        Self::BitOr,
        Self::BitXor,
    ];

    pub(crate) fn name(self) -> &'static str {
        return match self {
            Self::Add => "Add",
            Self::Sub => "Sub",
            Self::Mul => "Mul",
            Self::Div => "Div",
            Self::Rem => "Rem",
            Self::BitAnd => "BitAnd",
            Self::BitOr => "BitOr",
            Self::BitXor => "BitXor",
        };
    }

    pub(crate) fn method_name(self) -> &'static str {
        return match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Mul => "mul",
            Self::Div => "div",
            Self::Rem => "rem",
            Self::BitAnd => "bitand",
            Self::BitOr => "bitor",
            Self::BitXor => "bitxor",
        };
    }

    /// The operator token: `+`, `-`, `*`, `/`, `%`, `&`, `|`, `^`.
    pub(crate) fn symbol(self) -> TokenStream {
        return match self {
            Self::Add => quote! { + },
            Self::Sub => quote! { - },
            Self::Mul => quote! { * },
            Self::Div => quote! { / },
            Self::Rem => quote! { % },
            Self::BitAnd => quote! { & },
            Self::BitOr => quote! { | },
            Self::BitXor => quote! { ^ },
        };
    }

    /// The `konst_*` flag of the assignment operator: `AddAssign`, …
    pub(crate) fn assign_konst(self) -> Konsts {
        return match self {
            Self::Add => Konsts::AddAssign,
            Self::Sub => Konsts::SubAssign,
            Self::Mul => Konsts::MulAssign,
            Self::Div => Konsts::DivAssign,
            Self::Rem => Konsts::RemAssign,
            Self::BitAnd => Konsts::AndAssign,
            Self::BitOr => Konsts::OrAssign,
            Self::BitXor => Konsts::XorAssign,
        };
    }

    pub(crate) fn arithmetic_op(self) -> Option<ArithmeticOp> {
        return match self {
            Self::Add => Some(ArithmeticOp::Add),
            Self::Sub => Some(ArithmeticOp::Sub),
            Self::Mul => Some(ArithmeticOp::Mul),
            Self::Div => Some(ArithmeticOp::Div),
            Self::Rem => Some(ArithmeticOp::Rem),
            Self::BitAnd | Self::BitOr | Self::BitXor => None,
        };
    }

    pub(crate) fn items() -> &'static [Self] {
        return &Self::ITEMS;
    }

    pub(crate) fn bit_items() -> &'static [Self] {
        return &Self::BIT;
    }

    pub(crate) fn math_items() -> &'static [Self] {
        return &Self::MATH;
    }

    pub(crate) fn is_konst_flags(&self) -> [Konsts; 2] {
        return match self {
            Self::Add => [
                Konsts::Add,
                Konsts::Math,
            ],
            Self::Sub => [
                Konsts::Sub,
                Konsts::Math,
            ],
            Self::Mul => [
                Konsts::Mul,
                Konsts::Math,
            ],
            Self::Div => [
                Konsts::Div,
                Konsts::Math,
            ],
            Self::Rem => [
                Konsts::Rem,
                Konsts::Math,
            ],
            Self::BitAnd => [
                Konsts::And,
                Konsts::Bit,
            ],
            Self::BitOr => [
                Konsts::Or,
                Konsts::Bit,
            ],
            Self::BitXor => [
                Konsts::Xor,
                Konsts::Bit,
            ],
        };
    }
}

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
    Pow,
    Powf,
    Powi,
    Recip,
    RemEuclid,
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
    TryClear,
    TryInsert,
    TryInsertStr,
    TryPop,
    TryPush,
    TryPushStr,
    TryRemove,
    TryReplaceRange,
    TryRetain,
    TryTruncate,
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

impl FnName {
    const ITEMS: [Self; 185] = [
        Self::Abs,
        Self::AbsDiff,
        Self::Acos,
        Self::Acosh,
        Self::AsBytes,
        Self::AsStr,
        Self::Asin,
        Self::Asinh,
        Self::Atan,
        Self::Atan2,
        Self::Atanh,
        Self::BitWidth,
        Self::BorrowingSub,
        Self::Capacity,
        Self::CarryingAdd,
        Self::CarryingMul,
        Self::CarryingMulAdd,
        Self::CastSigned,
        Self::CastUnsigned,
        Self::Cbrt,
        Self::Ceil,
        Self::CheckedAbs,
        Self::CheckedAdd,
        Self::CheckedDiv,
        Self::CheckedDivEuclid,
        Self::CheckedIlog,
        Self::CheckedIlog10,
        Self::CheckedIlog2,
        Self::CheckedIsqrt,
        Self::CheckedMul,
        Self::CheckedNeg,
        Self::CheckedNextMultipleOf,
        Self::CheckedNextPowerOfTwo,
        Self::CheckedPow,
        Self::CheckedRem,
        Self::CheckedRemEuclid,
        Self::CheckedShl,
        Self::CheckedShr,
        Self::CheckedSub,
        Self::Clamp,
        Self::Classify,
        Self::Copysign,
        Self::Cos,
        Self::Cosh,
        Self::CountOnes,
        Self::CountZeros,
        Self::DivCeil,
        Self::DivEuclid,
        Self::Exp,
        Self::Exp2,
        Self::ExpM1,
        Self::Floor,
        Self::Fract,
        Self::FromBe,
        Self::FromBeBytes,
        Self::FromBits,
        Self::FromLe,
        Self::FromLeBytes,
        Self::FromNeBytes,
        Self::FunnelShl,
        Self::FunnelShr,
        Self::HighestOne,
        Self::Hypot,
        Self::Ilog,
        Self::Ilog10,
        Self::Ilog2,
        Self::IntoBytes,
        Self::IsEmpty,
        Self::IsFinite,
        Self::IsInfinite,
        Self::IsMultipleOf,
        Self::IsNan,
        Self::IsNegative,
        Self::IsNormal,
        Self::IsPositive,
        Self::IsPowerOfTwo,
        Self::IsSignNegative,
        Self::IsSignPositive,
        Self::IsSubnormal,
        Self::IsolateHighestOne,
        Self::IsolateLowestOne,
        Self::Isqrt,
        Self::LeadingOnes,
        Self::LeadingZeros,
        Self::Len,
        Self::Ln,
        Self::Ln1p,
        Self::Log,
        Self::Log10,
        Self::Log2,
        Self::LowestOne,
        Self::Max,
        Self::Midpoint,
        Self::Min,
        Self::MulAdd,
        Self::NextDown,
        Self::NextMultipleOf,
        Self::NextPowerOfTwo,
        Self::NextUp,
        Self::OverflowingAbs,
        Self::OverflowingAdd,
        Self::OverflowingDiv,
        Self::OverflowingDivEuclid,
        Self::OverflowingMul,
        Self::OverflowingNeg,
        Self::OverflowingPow,
        Self::OverflowingRem,
        Self::OverflowingRemEuclid,
        Self::OverflowingShl,
        Self::OverflowingShr,
        Self::OverflowingSub,
        Self::Pow,
        Self::Powf,
        Self::Powi,
        Self::Recip,
        Self::RemEuclid,
        Self::ReverseBits,
        Self::RotateLeft,
        Self::RotateRight,
        Self::Round,
        Self::RoundTiesEven,
        Self::SaturatingAbs,
        Self::SaturatingAdd,
        Self::SaturatingDiv,
        Self::SaturatingMul,
        Self::SaturatingNeg,
        Self::SaturatingPow,
        Self::SaturatingSub,
        Self::Signum,
        Self::Sin,
        Self::SinCos,
        Self::Sinh,
        Self::Sqrt,
        Self::StrictAbs,
        Self::StrictAdd,
        Self::StrictDiv,
        Self::StrictDivEuclid,
        Self::StrictMul,
        Self::StrictNeg,
        Self::StrictPow,
        Self::StrictRem,
        Self::StrictRemEuclid,
        Self::StrictShl,
        Self::StrictShr,
        Self::StrictSub,
        Self::SwapBytes,
        Self::Tan,
        Self::Tanh,
        Self::ToBe,
        Self::ToBeBytes,
        Self::ToBits,
        Self::ToDegrees,
        Self::ToLe,
        Self::ToLeBytes,
        Self::ToNeBytes,
        Self::ToRadians,
        Self::TotalCmp,
        Self::TrailingOnes,
        Self::TrailingZeros,
        Self::Trunc,
        Self::TryClear,
        Self::TryInsert,
        Self::TryInsertStr,
        Self::TryPop,
        Self::TryPush,
        Self::TryPushStr,
        Self::TryRemove,
        Self::TryReplaceRange,
        Self::TryRetain,
        Self::TryTruncate,
        Self::UnboundedShl,
        Self::UnboundedShr,
        Self::UnsignedAbs,
        Self::WrappingAbs,
        Self::WrappingAdd,
        Self::WrappingDiv,
        Self::WrappingDivEuclid,
        Self::WrappingMul,
        Self::WrappingNeg,
        Self::WrappingPow,
        Self::WrappingRem,
        Self::WrappingRemEuclid,
        Self::WrappingShl,
        Self::WrappingShr,
        Self::WrappingSub,
    ];

    pub(crate) fn name(self) -> &'static str {
        return match self {
            Self::Abs => "abs",
            Self::AbsDiff => "abs_diff",
            Self::Acos => "acos",
            Self::Acosh => "acosh",
            Self::AsBytes => "as_bytes",
            Self::AsStr => "as_str",
            Self::Asin => "asin",
            Self::Asinh => "asinh",
            Self::Atan => "atan",
            Self::Atan2 => "atan2",
            Self::Atanh => "atanh",
            Self::BitWidth => "bit_width",
            Self::BorrowingSub => "borrowing_sub",
            Self::Capacity => "capacity",
            Self::CarryingAdd => "carrying_add",
            Self::CarryingMul => "carrying_mul",
            Self::CarryingMulAdd => "carrying_mul_add",
            Self::CastSigned => "cast_signed",
            Self::CastUnsigned => "cast_unsigned",
            Self::Cbrt => "cbrt",
            Self::Ceil => "ceil",
            Self::CheckedAbs => "checked_abs",
            Self::CheckedAdd => "checked_add",
            Self::CheckedDiv => "checked_div",
            Self::CheckedDivEuclid => "checked_div_euclid",
            Self::CheckedIlog => "checked_ilog",
            Self::CheckedIlog10 => "checked_ilog10",
            Self::CheckedIlog2 => "checked_ilog2",
            Self::CheckedIsqrt => "checked_isqrt",
            Self::CheckedMul => "checked_mul",
            Self::CheckedNeg => "checked_neg",
            Self::CheckedNextMultipleOf => "checked_next_multiple_of",
            Self::CheckedNextPowerOfTwo => "checked_next_power_of_two",
            Self::CheckedPow => "checked_pow",
            Self::CheckedRem => "checked_rem",
            Self::CheckedRemEuclid => "checked_rem_euclid",
            Self::CheckedShl => "checked_shl",
            Self::CheckedShr => "checked_shr",
            Self::CheckedSub => "checked_sub",
            Self::Clamp => "clamp",
            Self::Classify => "classify",
            Self::Copysign => "copysign",
            Self::Cos => "cos",
            Self::Cosh => "cosh",
            Self::CountOnes => "count_ones",
            Self::CountZeros => "count_zeros",
            Self::DivCeil => "div_ceil",
            Self::DivEuclid => "div_euclid",
            Self::Exp => "exp",
            Self::Exp2 => "exp2",
            Self::ExpM1 => "exp_m1",
            Self::Floor => "floor",
            Self::Fract => "fract",
            Self::FromBe => "from_be",
            Self::FromBeBytes => "from_be_bytes",
            Self::FromBits => "from_bits",
            Self::FromLe => "from_le",
            Self::FromLeBytes => "from_le_bytes",
            Self::FromNeBytes => "from_ne_bytes",
            Self::FunnelShl => "funnel_shl",
            Self::FunnelShr => "funnel_shr",
            Self::HighestOne => "highest_one",
            Self::Hypot => "hypot",
            Self::Ilog => "ilog",
            Self::Ilog10 => "ilog10",
            Self::Ilog2 => "ilog2",
            Self::IntoBytes => "into_bytes",
            Self::IsEmpty => "is_empty",
            Self::IsFinite => "is_finite",
            Self::IsInfinite => "is_infinite",
            Self::IsMultipleOf => "is_multiple_of",
            Self::IsNan => "is_nan",
            Self::IsNegative => "is_negative",
            Self::IsNormal => "is_normal",
            Self::IsPositive => "is_positive",
            Self::IsPowerOfTwo => "is_power_of_two",
            Self::IsSignNegative => "is_sign_negative",
            Self::IsSignPositive => "is_sign_positive",
            Self::IsSubnormal => "is_subnormal",
            Self::IsolateHighestOne => "isolate_highest_one",
            Self::IsolateLowestOne => "isolate_lowest_one",
            Self::Isqrt => "isqrt",
            Self::LeadingOnes => "leading_ones",
            Self::LeadingZeros => "leading_zeros",
            Self::Len => "len",
            Self::Ln => "ln",
            Self::Ln1p => "ln_1p",
            Self::Log => "log",
            Self::Log10 => "log10",
            Self::Log2 => "log2",
            Self::LowestOne => "lowest_one",
            Self::Max => "max",
            Self::Midpoint => "midpoint",
            Self::Min => "min",
            Self::MulAdd => "mul_add",
            Self::NextDown => "next_down",
            Self::NextMultipleOf => "next_multiple_of",
            Self::NextPowerOfTwo => "next_power_of_two",
            Self::NextUp => "next_up",
            Self::OverflowingAbs => "overflowing_abs",
            Self::OverflowingAdd => "overflowing_add",
            Self::OverflowingDiv => "overflowing_div",
            Self::OverflowingDivEuclid => "overflowing_div_euclid",
            Self::OverflowingMul => "overflowing_mul",
            Self::OverflowingNeg => "overflowing_neg",
            Self::OverflowingPow => "overflowing_pow",
            Self::OverflowingRem => "overflowing_rem",
            Self::OverflowingRemEuclid => "overflowing_rem_euclid",
            Self::OverflowingShl => "overflowing_shl",
            Self::OverflowingShr => "overflowing_shr",
            Self::OverflowingSub => "overflowing_sub",
            Self::Pow => "pow",
            Self::Powf => "powf",
            Self::Powi => "powi",
            Self::Recip => "recip",
            Self::RemEuclid => "rem_euclid",
            Self::ReverseBits => "reverse_bits",
            Self::RotateLeft => "rotate_left",
            Self::RotateRight => "rotate_right",
            Self::Round => "round",
            Self::RoundTiesEven => "round_ties_even",
            Self::SaturatingAbs => "saturating_abs",
            Self::SaturatingAdd => "saturating_add",
            Self::SaturatingDiv => "saturating_div",
            Self::SaturatingMul => "saturating_mul",
            Self::SaturatingNeg => "saturating_neg",
            Self::SaturatingPow => "saturating_pow",
            Self::SaturatingSub => "saturating_sub",
            Self::Signum => "signum",
            Self::Sin => "sin",
            Self::SinCos => "sin_cos",
            Self::Sinh => "sinh",
            Self::Sqrt => "sqrt",
            Self::StrictAbs => "strict_abs",
            Self::StrictAdd => "strict_add",
            Self::StrictDiv => "strict_div",
            Self::StrictDivEuclid => "strict_div_euclid",
            Self::StrictMul => "strict_mul",
            Self::StrictNeg => "strict_neg",
            Self::StrictPow => "strict_pow",
            Self::StrictRem => "strict_rem",
            Self::StrictRemEuclid => "strict_rem_euclid",
            Self::StrictShl => "strict_shl",
            Self::StrictShr => "strict_shr",
            Self::StrictSub => "strict_sub",
            Self::SwapBytes => "swap_bytes",
            Self::Tan => "tan",
            Self::Tanh => "tanh",
            Self::ToBe => "to_be",
            Self::ToBeBytes => "to_be_bytes",
            Self::ToBits => "to_bits",
            Self::ToDegrees => "to_degrees",
            Self::ToLe => "to_le",
            Self::ToLeBytes => "to_le_bytes",
            Self::ToNeBytes => "to_ne_bytes",
            Self::ToRadians => "to_radians",
            Self::TotalCmp => "total_cmp",
            Self::TrailingOnes => "trailing_ones",
            Self::TrailingZeros => "trailing_zeros",
            Self::Trunc => "trunc",
            Self::TryClear => "try_clear",
            Self::TryInsert => "try_insert",
            Self::TryInsertStr => "try_insert_str",
            Self::TryPop => "try_pop",
            Self::TryPush => "try_push",
            Self::TryPushStr => "try_push_str",
            Self::TryRemove => "try_remove",
            Self::TryReplaceRange => "try_replace_range",
            Self::TryRetain => "try_retain",
            Self::TryTruncate => "try_truncate",
            Self::UnboundedShl => "unbounded_shl",
            Self::UnboundedShr => "unbounded_shr",
            Self::UnsignedAbs => "unsigned_abs",
            Self::WrappingAbs => "wrapping_abs",
            Self::WrappingAdd => "wrapping_add",
            Self::WrappingDiv => "wrapping_div",
            Self::WrappingDivEuclid => "wrapping_div_euclid",
            Self::WrappingMul => "wrapping_mul",
            Self::WrappingNeg => "wrapping_neg",
            Self::WrappingPow => "wrapping_pow",
            Self::WrappingRem => "wrapping_rem",
            Self::WrappingRemEuclid => "wrapping_rem_euclid",
            Self::WrappingShl => "wrapping_shl",
            Self::WrappingShr => "wrapping_shr",
            Self::WrappingSub => "wrapping_sub",
        };
    }

    pub(crate) fn of(name: &str) -> Option<Self> {
        return Self::ITEMS.iter().copied().find(|it| it.name() == name);
    }
}
