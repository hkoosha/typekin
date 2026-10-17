#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u128)]
#[typekin::bitflag(konst = stable, suffix = "s", with = [display])]
pub enum MyFlag {
    Z = 0,
    A = 10,
    B,
    C = 40,
}

const A_BITS: u128 = MyFlag::A.into_value().bits();

fn main() {
    let lhs = MyFlag::A;
    let rhs = MyFlag::B.into_value();

    assert_eq!(lhs.to_string(), "A");
    assert_eq!(A_BITS, 10);
    assert_eq!(lhs.into_value().to_string(), "10");
    println!("{:?}", (lhs & rhs) == (lhs & rhs));
    println!("{}", rhs & lhs);
    println!("{}", lhs & rhs);
    println!("{}", lhs);

    for x in MyFlag::items() {
        println!("{}", x.name());
    }
}
