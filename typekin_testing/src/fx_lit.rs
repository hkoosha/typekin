use self::net::{
    Port,
    port,
};
use crate::*;

pub mod net {
    #[typekin::integral(domain = 1..=1023 | 49_152..=65_535)]
    pub struct Port(u16);

    #[typekin::integral(domain = -5..=5)]
    pub struct Offset(i8);

    #[typekin::integral(domain = ..=100, lit_path = crate::fx_lit::net::Percent)]
    pub struct Percent(u8);

    #[typekin::integral(domain = 1..=9, hide_in = none)]
    pub(crate) struct Digit(u8);
}

pub mod other {
    #[typekin::integral(domain = 1..)]
    pub struct Port(u16);
}

#[typekin::integral(domain = 1.., lit_name = line_no)]
pub struct Line(u32);

#[typekin::integral(domain = 1..=5)]
struct Grade(u8);

/// Builds a `Grade` through its literal macro's path; `Grade` is private, so
/// only this module can name either.
pub(crate) fn grade_by_path() -> u8 {
    return self::grade!(3).raw();
}

#[typekin::integral(with = lit_macro)]
pub struct Count(u16);

// Each `macro_rules!` below takes its type's literal macro name, which fails to
// compile if typekin also generated that macro.
#[typekin::integral]
pub struct Plain(u16);

macro_rules! plain {
    () => {
        "caller's plain!"
    };
}

pub(crate) use plain;

#[typekin::integral(domain = 1.., without = lit_macro)]
pub struct Unlisted(u16);

macro_rules! unlisted {
    () => {
        "caller's unlisted!"
    };
}

pub(crate) use unlisted;

#[typekin::integral(domain = 1.., konst = none)]
pub struct Runtime(u16);

macro_rules! runtime {
    () => {
        "caller's runtime!"
    };
}

pub(crate) use runtime;

#[typekin::float(domain = 0.0..=1.0)]
pub struct Unit(f64);

#[typekin::circular(domain = 1..)]
pub struct Sequence(u32);

#[typekin::bitflag(repr = u8, integral = [domain = ..=3])]
pub enum Access {
    Read = 0b01,
    Write = 0b10,
}

#[typekin::integral(
    tree = Direction {
        In,
        Out,
    },
    domain = 1..,
)]
pub struct Tagged<D: Direction>(u32);

#[typekin::integral(domain = 1..)]
pub struct Labeled<'a, T: ?Sized = (), const SPACE: u8 = 7>(u32)
where
    T: 'a;

pub(crate) fn inferred<D: Direction>() -> Tagged<D> {
    return tagged!(const 3);
}

pub(crate) fn explicit_inline<D: Direction>() -> Tagged<D> {
    return tagged!(const [D] 4);
}

pub(crate) fn other_port() -> other::Port {
    use other::{
        Port,
        port,
    };

    return port!(8080);
}

pub(crate) const HTTPS: Port = port!(443);
