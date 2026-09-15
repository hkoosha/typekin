#![allow(dead_code)]
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_destruct)]
#![feature(derive_const)]

mod subject {
    #[typekin::integral(konst = true, friends = [i128(conv = self, level = [Full])])]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    pub struct MyU32(i128);
}

type Subject = subject::MyU32;

fn main() {
    let lhs = 0b1101i128;
    let rhs = 0b0110i128;

    let lhs = Subject::of(lhs);
    let rhs = Subject::of(rhs);
    println!("{:?}", lhs + rhs);
}
