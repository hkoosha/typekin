use std::ops::BitXor;

#[derive(Debug, Clone, Copy)]
struct Thing0(u128);

fn convert_for_make(it: Thing0) -> u128 {
    return it.0 / 4;
}

fn convert_for_bit(it: Thing0) -> u128 {
    return it.0.bitxor(0b01);
}

#[typekin::friends(
    relation = u128,
    friends = [
        convert_for_make(Thing0) -> [Make],
        convert_for_bit(Thing0) -> [Bit],
    ],
    scope = things,
)]
#[derive(Debug, Clone, Copy)]
pub struct Thingy(u128);

impl Thingy {
    fn of_parts(it: u128) -> Self {
        return Self(it);
    }
}

fn requires_bit<T: things::Bit>(it: T) -> u128 {
    return it.to_u128();
}

fn main() {
    let vv = Thing0(7);
    let it = Thingy::of(vv);
    assert_eq!(it.0, vv.0 / 4);

    let vv = Thing0(0b01101);
    let bit = requires_bit(vv);
    assert_eq!(bit, 0b01100);
}
