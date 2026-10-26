#[typekin::integral(
    konst = none,
    friend = self(u16) -> Math,
)]
pub struct MyU16(u16);

fn main() {
    let lhs = 0b1101u16;
    let rhs = 0b0110u16;

    let lhs = MyU16::of(lhs);

    println!("{:?}", lhs + rhs);
    println!("{:?}", lhs | rhs);
    println!("{:?}", lhs.lo8());
}
