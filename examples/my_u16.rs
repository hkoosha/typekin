mod subject {
    #[typekin::integral(konst = false)]
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct MyU16(u16);
}

use subject::MyU16 as Subject;

fn main() {
    let lhs = 0b1101u16;
    let rhs = 0b0110u16;

    let lhs = Subject::of(lhs);
    let rhs = Subject::of(rhs);

    println!("{:?}", lhs + rhs);
    println!("{:?}", lhs.lo8());
}
