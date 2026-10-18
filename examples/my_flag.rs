#![cfg_attr(
    feature = "nightly",
    feature(
        const_clone,
        const_cmp,
        const_convert,
        const_destruct,
        const_iter,
        const_ops,
        const_trait_impl,
        derive_const,
    )
)]

#[repr(u128)]
#[typekin::bitflag(
    suffix = "s",
    with = [display],
)]
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
    assert_eq!(lhs.into_value().to_string(), "A");
    assert_eq!(
        "A | 0x1".parse::<MyFlags>().unwrap().bits(),
        MyFlag::B.raw(),
    );
    println!("{:?}", (lhs & rhs) == (lhs & rhs));
    println!("{}", rhs & lhs);
    println!("{}", lhs & rhs);
    println!("{}", lhs);

    for x in MyFlag::items() {
        println!("{}", x.name());
    }
}
