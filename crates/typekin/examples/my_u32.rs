#![allow(dead_code)]
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_destruct)]
#![feature(derive_const)]

use typekin_testing::demo_u128;

mod subject {
    use std::fmt::Formatter;

    #[typekin::integral(friends = [u32(conv = self, level = [Full])])]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    pub struct MyU32(u32);

    impl std::fmt::Display for MyU32 {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "MyU32({})", self.raw())
        }
    }
}

type Subject = subject::MyU32;

fn main() {
    let lhs = 0b1101u32;
    let rhs = 0b0110u32;

    let demo = demo_u128(Subject::of(lhs), rhs, |it| it.raw() as u128);
    println!("{}", demo.print());
}
