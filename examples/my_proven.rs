//! Closure proofs: operations that cannot leave the domain skip their check.
//! They are on by default for `u8`/`i8`; wider integers opt in with
//! `with = [proven]`, and `prove` turns a proof into a compile-time contract.

// `math_add` in `prove` would not compile:
// error: `Percent + Percent` is not closed: 1 + 100 = 101 is outside `..=100`
#[typekin::integral(domain = ..=100, prove = [math_div, math_rem, midpoint])]
pub struct Percent(u8);

#[typekin::integral(with = [proven], domain = 1..=1023 | 49_152..=65_535)]
pub struct Port(u16);

fn main() {
    let half = Percent::try_make(50).unwrap();
    let third = Percent::try_make(33).unwrap();

    // Proven: these compile to the bare `u8` operations.
    let ratio = half / third;
    let middle = half.midpoint(third);

    let ssh = Port::try_make(22).unwrap();
    let dynamic = Port::try_make(50_000).unwrap();

    // `Port | Port` is proven; `Port | u16` still validates.
    let combined = ssh | dynamic;

    println!("{} {}", ratio.raw(), middle.raw());
    println!("{}", combined.raw());
}
