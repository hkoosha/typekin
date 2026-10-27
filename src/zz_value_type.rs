#![allow(unused, dead_code)]

use std::cmp::Ordering;
use std::fmt::{
    Display,
    Formatter,
};
use std::str::FromStr;

use crate::zz_lib::Konsts;

pub(crate) const POINTER_WIDTHS: [u32; 3] = [
    16, 32, 64,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    const N_RUST_NAMES: [&str; 12] = [
        "usize", "isize", "u8", "u16", "u32", "u64", "u128", "i8", "i16",
        "i32", "i64", "i128",
    ];

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

    pub const fn rust_names() -> &'static [&'static str] {
        return &Self::N_RUST_NAMES;
    }

    pub const fn len() -> usize {
        return Self::items().len();
    }
}

impl N {
    pub fn typekin_name(&self) -> &'static str {
        return match self {
            N::USIZ => "USIZ",
            N::ISIZ => "ISIZ",
            N::U008 => "U008",
            N::U016 => "U016",
            N::U032 => "U032",
            N::U064 => "U064",
            N::U128 => "U128",
            N::I008 => "I008",
            N::I016 => "I016",
            N::I032 => "I032",
            N::I064 => "I064",
            N::I128 => "I128",
        };
    }

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

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Int {
    Negative(u128),
    NonNegative(u128),
}

impl Ord for Int {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        return match (self, other) {
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
        other: &Self,
    ) -> Option<Ordering> {
        return Some(self.cmp(other));
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

    pub(crate) fn parse_non_negative(
        digits: &str,
        suffix: &str,
    ) -> Result<Int, ()> {
        if !suffix.is_empty() && N::rust_names().contains(&suffix) {
            return Err(());
        }

        let digits = digits.replace('_', "");

        let value = if let Some(digits) = digits.strip_prefix("0x") {
            u128::from_str_radix(digits, 16)
        }
        else if let Some(digits) = digits.strip_prefix("0o") {
            u128::from_str_radix(digits, 8)
        }
        else if let Some(digits) = digits.strip_prefix("0b") {
            u128::from_str_radix(digits, 2)
        }
        else {
            digits.parse()
        };

        return value.map(Int::NonNegative).map_err(|_| ());
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum RangeEnding {
    Empty,
    Value(Int),
    Unbounded,
}

impl Ord for RangeEnding {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        return match (self, other) {
            (Self::Value(left), Self::Value(right)) => left.cmp(right),

            (Self::Empty, Self::Empty) => Ordering::Equal,
            (Self::Unbounded, Self::Unbounded) => Ordering::Equal,

            (Self::Empty, _) => Ordering::Less,
            (_, Self::Unbounded) => Ordering::Less,

            (Self::Unbounded, _) => Ordering::Greater,
            (_, Self::Empty) => Ordering::Greater,
        };
    }
}

impl PartialOrd for RangeEnding {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        return Some(self.cmp(other));
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

impl Display for Operator {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Operator {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Self::items()
            .iter()
            .find(|it| it.name() == s)
            .cloned()
            .ok_or("unknown operator");
    }
}
