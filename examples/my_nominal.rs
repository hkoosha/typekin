//! Nominal values are only told apart: no operators, ordering, or numeric
//! conversions; construction, `raw()`, `Eq`, and `Hash` remain.

#[typekin::nominal(with = display)]
struct Port(u16);

#[typekin::nominal]
struct EchoId(u16);

/// An IPv4 IHL or TCP data offset: a header length in 32-bit words.
#[typekin::nominal(domain = 5..=15)]
struct HeaderWords(u8);

fn main() {
    let http = Port::make(80);
    let id = EchoId::make(80);
    let words = header_words!(5);

    // Rejected by the types: `http == id`, `http + 1`, `http < http`, and
    // `http.into_u32()`.
    assert_eq!(http.raw(), id.raw());
    assert_eq!(HeaderWords::try_make(4), Err(4));
    println!("port {http}, header {} bytes", usize::from(words.raw()) * 4);
}
