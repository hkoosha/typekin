mod subject {
    #[typekin::integral(konst = false)]
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct MyExample(u32);
}

type Subject = subject::MyExample;

fn main() {
    let lhs = 0b1101u32;
    let rhs = 0b0110u32;

    let lhs = Subject::of(lhs);
    let rhs = Subject::of(rhs);
    println!("{:?}", lhs + rhs);
}
