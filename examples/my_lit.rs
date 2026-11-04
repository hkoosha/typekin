//! Literal macros: every validated wrapper gets a macro named after its
//! snake-cased type, which checks a constant at compile time.

#[typekin::integral(domain = 1..=1023 | 49_152..=65_535)]
pub struct Port(u16);

#[typekin::integral(
    tree = Direction {
        In,
        Out,
    },
    domain = 1..,
)]
pub struct Packet<D: Direction>(u32);

// `port!(0)` would not compile:
// error[E0080]: evaluation panicked: 0 is outside the domain of Port
const HTTPS: Port = port!(443);

fn first<D: Direction>() -> Packet<D> {
    // Inside generic code, the inline form infers the arguments.
    return packet!(const 1);
}

fn main() {
    let ssh = port!(22);
    let inbound: Packet<In> = first();
    let outbound = packet!([Out] 7);

    println!("{} {}", HTTPS.raw(), ssh.raw());
    println!("{} {}", inbound.raw(), outbound.raw());
}
