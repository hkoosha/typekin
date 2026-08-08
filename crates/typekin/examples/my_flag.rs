#![allow(dead_code)]
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_iter)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]

use typekin_testing::demo_u128;

mod subject {
    use std::fmt::{Display, Formatter};

    #[derive_const(Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[derive(Copy, Debug, Hash)]
    #[repr(u128)]
    #[typekin::bitflag(
        integral = [
            friends = [u128] // Only for test utils.
        ],
    )]
    pub enum MyFlag {
        Z = 0,
        A = 10,
        B,
        C = 40,
    }

    impl MyFlag {
        #[inline(always)]
        #[must_use]
        pub const fn empty() -> Self {
            return Self::Z;
        }
    }

    impl Display for MyFlagValue {
        fn fmt(
            &self,
            f: &mut Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "MyFlagValue({})", self.raw())
        }
    }
}

type Subject = subject::MyFlag;
type Value = subject::MyFlagValue;

fn main() {
    let lhs = 0b11000;
    let rhs = 0b10100;

    let demo = demo_u128(Value::of(lhs), rhs, |it| it.raw());
    println!("{}", demo.print());

    for x in Subject::items() {
        println!("{}", x.name());
    }
}
