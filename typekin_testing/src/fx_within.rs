use crate::*;

#[typekin::integral(domain = ..=100)]
pub(crate) struct Percent(u8);

#[typekin::integral(
    domain = ..=255,
    friend = [
        _(Percent) -> Within,
        _(u8) -> Within,
    ],
)]
pub(crate) struct Byte(u16);

/// `NonZero` storage on both sides.
#[typekin::integral(domain = 1..=1023 | 49_152..)]
pub(crate) struct Port(u16);

#[typekin::integral(domain = 1.., friend = _(Port) -> Within)]
pub(crate) struct Positive16(u16);

pub(crate) const fn is_odd(raw: i8) -> bool {
    return raw % 2 != 0;
}

/// An 8-bit source whose callback only its table captures.
#[typekin::integral(domain = -50..=50, valid = is_odd)]
pub(crate) struct Odd(i8);

pub(crate) const fn is_wide_odd(raw: i32) -> bool {
    return raw % 2 != 0;
}

#[typekin::integral(
    konst = stable,
    domain = -60..=60,
    valid = is_wide_odd,
    friend = _(Odd) -> Within,
)]
pub(crate) struct WideOdd(i32);

#[typekin::integral(domain = mul(6) & ..=60)]
pub(crate) struct Sixes(u32);

#[typekin::integral(domain = mul(2), friend = _(Sixes) -> Within)]
pub(crate) struct Even(u32);

#[typekin::bitflag(integral = [domain = ..=0b111])]
#[repr(u8)]
pub(crate) enum Mode {
    Read = 1,
    Write = 2,
    Exec = 4,
}

#[typekin::integral(domain = ..=7, friend = _(ModeValue) -> Within)]
pub(crate) struct ModeBits(u8);

#[typekin::integral(domain = ..=15)]
pub(crate) struct Mask(u8);

#[typekin::bitflag(integral = [friend = _(Mask) -> Within])]
#[repr(u16)]
pub(crate) enum Lane {
    Low = 1,
    High = 256,
}

pub(crate) struct In;

#[typekin::integral(domain = ..=100)]
pub(crate) struct Tagged<D>(u16);

#[typekin::integral(domain = ..=255, friend = _(Tagged<In>) -> Within)]
pub(crate) struct TaggedByte(u32);

#[typekin::integral(domain = ..=1000, friend = _(Tagged<D>) -> Within)]
pub(crate) struct Carried<D>(u32);

#[typekin::integral(
    domain = ..=200,
    friend = _(Percent) -> [Within, Rel, Math],
)]
pub(crate) struct Score(u16);

#[typekin::integral(
    konst = nightly,
    domain = ..=255,
    friend = _(Percent) -> Within,
)]
pub(crate) struct ConstByte(u16);
