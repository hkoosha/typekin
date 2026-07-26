use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    pub(crate) fn fits_in(
        &self,
        other: &N,
    ) -> bool {
        return if self.is_signed() == other.is_signed() {
            // Same signedness
            self.bits() <= other.bits()
        }
        else {
            // Otherwise, only unsigned may go to bigger signed
            !self.is_signed() && self.bits() < other.bits()
        };
    }
}

const N_ITEMS: [N; 12] = [
    N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    N::U032,
    N::U064,
    N::U128,
    N::I008,
    N::I016,
    N::I032,
    N::I064,
    N::I128,
];

const N_RUST_NAMES: [&'static str; 12] = [
    "usize", "isize", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32",
    "i64", "i128",
];

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
        return &N_ITEMS;
    }

    pub const fn rust_names() -> &'static [&'static str] {
        return &N_RUST_NAMES;
    }

    #[allow(dead_code)]
    pub const fn len() -> usize {
        return Self::items().len();
    }
}

impl N {
    pub fn name(&self) -> &'static str {
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

    #[allow(dead_code)]
    pub fn bits(&self) -> usize {
        return match self {
            N::USIZ => usize::BITS,
            N::ISIZ => usize::BITS,
            N::U008 => u8::BITS,
            N::U016 => u16::BITS,
            N::U032 => u32::BITS,
            N::U064 => u64::BITS,
            N::U128 => u128::BITS,
            N::I008 => i8::BITS,
            N::I016 => i16::BITS,
            N::I032 => i32::BITS,
            N::I064 => i64::BITS,
            N::I128 => i128::BITS,
        } as usize;
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn safe_casts(&self) -> &[Self] {
        match self {
            N::USIZ => &SAFE_USIZ,
            N::ISIZ => &SAFE_ISIZ,

            N::U008 => &SAFE_U008,
            N::U016 => &SAFE_U016,
            N::U032 => &SAFE_U032,
            N::U064 => &SAFE_U064,
            N::U128 => &SAFE_U128,

            N::I008 => &SAFE_I008,
            N::I016 => &SAFE_I016,
            N::I032 => &SAFE_I032,
            N::I064 => &SAFE_I064,
            N::I128 => &SAFE_I128,
        }
    }

    #[allow(dead_code)]
    pub fn chck_casts(&self) -> &[Self] {
        match self {
            N::USIZ => &CHCK_USIZ,
            N::ISIZ => &CHCK_ISIZ,

            N::U008 => &CHCK_U008,
            N::U016 => &CHCK_U016,
            N::U032 => &CHCK_U032,
            N::U064 => &CHCK_U064,
            N::U128 => &CHCK_U128,

            N::I008 => &CHCK_I008,
            N::I016 => &CHCK_I016,
            N::I032 => &CHCK_I032,
            N::I064 => &CHCK_I064,
            N::I128 => &CHCK_I128,
        }
    }
}

impl N {
    pub(crate) fn can_safe_cast_to(
        &self,
        to: Self,
    ) -> bool {
        return self.safe_casts().contains(&to);
    }
}

impl Display for N {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// =============================================================================

#[allow(dead_code)]
const CHCK_U008: [N; 1] = [
    // N::USIZ,
    // N::ISIZ,
    // N::U008,
    // N::U016,
    // N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    // N::I016,
    // N::I032,
    // N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_U008: [N; 11] = [
    N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    N::U032,
    N::U064,
    N::U128,
    // N::I008,
    N::I016,
    N::I032,
    N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_U016_16: [N; 4] = [
    // N::USIZ,
    N::ISIZ,
    N::U008,
    // N::U016,
    // N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    N::I016,
    // N::I032,
    // N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_U016_16: [N; 8] = [
    N::USIZ,
    // N::ISIZ,
    // N::U008,
    N::U016,
    N::U032,
    N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    N::I032,
    N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_U016_32: [N; 3] = [
    // N::USIZ,
    // N::ISIZ,
    N::U008,
    // N::U016,
    // N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    N::I016,
    // N::I032,
    // N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_U016_32: [N; 9] = [
    N::USIZ,
    N::ISIZ,
    // N::U008,
    N::U016,
    N::U032,
    N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    N::I032,
    N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_U016_64: [N; 3] = CHCK_U016_32;

#[allow(dead_code)]
const SAFE_U016_64: [N; 9] = SAFE_U016_32;

#[allow(dead_code)]
const CHCK_U032_16: [N; 7] = [
    N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    // N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    N::I016,
    N::I032,
    // N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_U032_16: [N; 5] = [
    // N::USIZ,
    // N::ISIZ,
    // N::U008,
    // N::U016,
    N::U032,
    N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    // N::I032,
    N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_U032_32: [N; 6] = [
    // N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    // N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    N::I016,
    N::I032,
    // N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_U032_32: [N; 6] = [
    N::USIZ,
    // N::ISIZ,
    // N::U008,
    // N::U016,
    N::U032,
    N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    // N::I032,
    N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_U032_64: [N; 6] = CHCK_U032_32;
#[allow(dead_code)]
const SAFE_U032_64: [N; 6] = SAFE_U032_32;

#[allow(dead_code)]
const CHCK_U064_16: [N; 9] = [
    N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    N::I016,
    N::I032,
    N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_U064_16: [N; 3] = [
    // N::USIZ,
    // N::ISIZ,
    // N::U008,
    // N::U016,
    // N::U032,
    N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    // N::I032,
    // N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_U064_32: [N; 9] = CHCK_U064_16;
#[allow(dead_code)]
const SAFE_U064_32: [N; 3] = SAFE_U064_16;

#[allow(dead_code)]
const CHCK_U064_64: [N; 8] = [
    // N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    N::I016,
    N::I032,
    N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_U064_64: [N; 4] = [
    N::USIZ,
    // N::ISIZ,
    // N::U008,
    // N::U016,
    // N::U032,
    N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    // N::I032,
    // N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_U128: [N; 11] = [
    N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    N::U032,
    N::U064,
    // N::U128,
    N::I008,
    N::I016,
    N::I032,
    N::I064,
    N::I128,
];

#[allow(dead_code)]
const SAFE_U128: [N; 1] = [
    // N::USIZ,
    // N::ISIZ,
    // N::U008,
    // N::U016,
    // N::U032,
    // N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    // N::I032,
    // N::I064,
    // N::I128,
];

// =============================================================================

#[allow(dead_code)]
const CHCK_I008: [N; 1] = [
    // N::USIZ,
    // N::ISIZ,
    // N::U008,
    // N::U016,
    // N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    // N::I016,
    // N::I032,
    // N::I064,
    // N::I128,
];
#[allow(dead_code)]
const SAFE_I008: [N; 11] = [
    N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    N::U032,
    N::U064,
    N::U128,
    // N::I008,
    N::I016,
    N::I032,
    N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_I016_16: [N; 4] = [
    // N::USIZ,
    N::ISIZ,
    N::U008,
    // N::U016,
    // N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    N::I016,
    // N::I032,
    // N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_I016_16: [N; 8] = [
    N::USIZ,
    // N::ISIZ,
    // N::U008,
    N::U016,
    N::U032,
    N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    N::I032,
    N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_I016_32: [N; 3] = [
    // N::USIZ,
    // N::ISIZ,
    N::U008,
    // N::U016,
    // N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    N::I016,
    // N::I032,
    // N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_I016_32: [N; 9] = [
    N::USIZ,
    N::ISIZ,
    // N::U008,
    N::U016,
    N::U032,
    N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    N::I032,
    N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_I016_64: [N; 3] = CHCK_I016_32;

#[allow(dead_code)]
const SAFE_I016_64: [N; 9] = SAFE_I016_32;

#[allow(dead_code)]
const CHCK_I032_16: [N; 7] = [
    N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    // N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    N::I016,
    N::I032,
    // N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_I032_16: [N; 5] = [
    // N::USIZ,
    // N::ISIZ,
    // N::U008,
    // N::U016,
    N::U032,
    N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    // N::I032,
    N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_I032_32: [N; 6] = [
    // N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    // N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    N::I016,
    N::I032,
    // N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_I032_32: [N; 6] = [
    N::USIZ,
    // N::ISIZ,
    // N::U008,
    // N::U016,
    N::U032,
    N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    // N::I032,
    N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_I032_64: [N; 6] = CHCK_I032_32;
#[allow(dead_code)]
const SAFE_I032_64: [N; 6] = SAFE_I032_32;

#[allow(dead_code)]
const CHCK_I064_16: [N; 9] = [
    N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    N::I016,
    N::I032,
    N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_I064_16: [N; 3] = [
    // N::USIZ,
    // N::ISIZ,
    // N::U008,
    // N::U016,
    // N::U032,
    N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    // N::I032,
    // N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_I064_32: [N; 9] = CHCK_I064_16;
#[allow(dead_code)]
const SAFE_I064_32: [N; 3] = SAFE_I064_16;

#[allow(dead_code)]
const CHCK_I064_64: [N; 8] = [
    // N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    N::U032,
    // N::U064,
    // N::U128,
    N::I008,
    N::I016,
    N::I032,
    N::I064,
    // N::I128,
];

#[allow(dead_code)]
const SAFE_I064_64: [N; 4] = [
    N::USIZ,
    // N::ISIZ,
    // N::U008,
    // N::U016,
    // N::U032,
    N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    // N::I032,
    // N::I064,
    N::I128,
];

#[allow(dead_code)]
const CHCK_I128: [N; 11] = [
    N::USIZ,
    N::ISIZ,
    N::U008,
    N::U016,
    N::U032,
    N::U064,
    // N::U128,
    N::I008,
    N::I016,
    N::I032,
    N::I064,
    N::I128,
];

#[allow(dead_code)]
const SAFE_I128: [N; 1] = [
    // N::USIZ,
    // N::ISIZ,
    // N::U008,
    // N::U016,
    // N::U032,
    // N::U064,
    N::U128,
    // N::I008,
    // N::I016,
    // N::I032,
    // N::I064,
    // N::I128,
];

// =============================================================================

#[allow(dead_code)]
const CHCK_USIZ_16: [N; 4] = CHCK_U016_16;
#[allow(dead_code)]
const CHCK_USIZ_32: [N; 6] = CHCK_U032_32;
#[allow(dead_code)]
const CHCK_USIZ_64: [N; 8] = CHCK_U064_64;
#[allow(dead_code)]
const SAFE_USIZ_16: [N; 8] = SAFE_U016_16;
#[allow(dead_code)]
const SAFE_USIZ_32: [N; 6] = SAFE_U032_32;
#[allow(dead_code)]
const SAFE_USIZ_64: [N; 4] = SAFE_U064_64;

#[allow(dead_code)]
const CHCK_ISIZ_16: [N; 4] = CHCK_I016_16;
#[allow(dead_code)]
const CHCK_ISIZ_32: [N; 6] = CHCK_I032_32;
#[allow(dead_code)]
const CHCK_ISIZ_64: [N; 8] = CHCK_I064_64;
#[allow(dead_code)]
const SAFE_ISIZ_16: [N; 8] = SAFE_I016_16;
#[allow(dead_code)]
const SAFE_ISIZ_32: [N; 6] = SAFE_I032_32;
#[allow(dead_code)]
const SAFE_ISIZ_64: [N; 4] = SAFE_I064_64;

// =============================================================================

#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const SAFE_I016: [N; 8] = SAFE_I016_16;
#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const CHCK_I016: [N; 4] = CHCK_I016_16;

#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const SAFE_I016: [N; 9] = SAFE_I016_32;
#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const CHCK_I016: [N; 3] = CHCK_I016_32;

#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const SAFE_I016: [N; 9] = SAFE_I016_64;
#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const CHCK_I016: [N; 3] = CHCK_I016_64;

// ----

#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const SAFE_I032: [N; 5] = SAFE_I032_16;
#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const CHCK_I032: [N; 7] = CHCK_I032_16;

#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const SAFE_I032: [N; 6] = SAFE_I032_32;
#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const CHCK_I032: [N; 6] = CHCK_I032_32;

#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const SAFE_I032: [N; 6] = SAFE_I032_64;
#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const CHCK_I032: [N; 6] = CHCK_I032_64;

// ----

#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const SAFE_I064: [N; 3] = SAFE_I064_16;
#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const CHCK_I064: [N; 9] = CHCK_I064_16;

#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const SAFE_I064: [N; 3] = SAFE_I064_32;
#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const CHCK_I064: [N; 9] = CHCK_I064_32;

#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const SAFE_I064: [N; 4] = SAFE_I064_64;
#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const CHCK_I064: [N; 8] = CHCK_I064_64;

// ----

#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const SAFE_ISIZ: [N; 3] = SAFE_ISIZ_16;
#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const CHCK_ISIZ: [N; 9] = CHCK_ISIZ_16;

#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const SAFE_ISIZ: [N; 3] = SAFE_ISIZ_32;
#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const CHCK_ISIZ: [N; 9] = CHCK_ISIZ_32;

#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const SAFE_ISIZ: [N; 4] = SAFE_ISIZ_64;
#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const CHCK_ISIZ: [N; 8] = CHCK_ISIZ_64;

// ----

#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const SAFE_USIZ: [N; 3] = SAFE_USIZ_16;
#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const CHCK_USIZ: [N; 9] = CHCK_USIZ_16;

#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const SAFE_USIZ: [N; 3] = SAFE_USIZ_32;
#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const CHCK_USIZ: [N; 9] = CHCK_USIZ_32;

#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const SAFE_USIZ: [N; 4] = SAFE_USIZ_64;
#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const CHCK_USIZ: [N; 8] = CHCK_USIZ_64;

// ----

#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const SAFE_U016: [N; 8] = SAFE_U016_16;
#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const CHCK_U016: [N; 4] = CHCK_U016_16;

#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const SAFE_U016: [N; 9] = SAFE_U016_32;
#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const CHCK_U016: [N; 3] = CHCK_U016_32;

#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const SAFE_U016: [N; 9] = SAFE_U016_64;
#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const CHCK_U016: [N; 3] = CHCK_U016_64;

// ----

#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const SAFE_U032: [N; 5] = SAFE_U032_16;
#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const CHCK_U032: [N; 7] = CHCK_U032_16;

#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const SAFE_U032: [N; 6] = SAFE_U032_32;
#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const CHCK_U032: [N; 6] = CHCK_U032_32;

#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const SAFE_U032: [N; 6] = SAFE_U032_64;
#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const CHCK_U032: [N; 6] = CHCK_U032_64;

// ----

#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const SAFE_U064: [N; 3] = SAFE_U064_16;
#[cfg(target_pointer_width = "16")]
#[allow(dead_code)]
const CHCK_U064: [N; 9] = CHCK_U064_16;

#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const SAFE_U064: [N; 3] = SAFE_U064_32;
#[cfg(target_pointer_width = "32")]
#[allow(dead_code)]
const CHCK_U064: [N; 9] = CHCK_U064_32;

#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const SAFE_U064: [N; 4] = SAFE_U064_64;
#[cfg(target_pointer_width = "64")]
#[allow(dead_code)]
const CHCK_U064: [N; 8] = CHCK_U064_64;
