//! Proven friendship: `_(Source) -> Within` builds the target without
//! validation, because rustc proves at compile time that every value of the
//! source lies in the target's domain. `of` compiles to a plain cast.

#[typekin::integral(domain = ..=100)]
pub struct Percent(u8);

#[typekin::integral(domain = 1..=1023 | 49_152..)]
pub struct Port(u16);

// Wrappers and primitives alike; `u16` would not compile:
// error: `u16` is not within `Byte`: 256 is outside the domain `..=255`
#[typekin::integral(
    domain = ..=255,
    friend = [
        _(Percent) -> Within,
        _(u8) -> Within,
    ],
)]
pub struct Byte(u16);

// `_(Byte) -> Within` would not compile:
// error: `Byte` is not within `Positive`: 0 is outside the domain of `Positive`
#[typekin::integral(domain = 1.., friend = _(Port) -> Within)]
pub struct Positive(u16);

fn main() {
    let percent = Percent::try_make(42).unwrap();
    let port = Port::try_make(443).unwrap();

    // No validation and no panic path: each is the bare conversion.
    let from_percent = Byte::of(percent);
    let from_u8 = Byte::of(7u8);
    let positive = Positive::of(port);

    println!("{} {}", from_percent.raw(), from_u8.raw());
    println!("{}", positive.raw());
}
