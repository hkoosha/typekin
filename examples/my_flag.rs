#![allow(dead_code)]
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_iter)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]

mod subject {
    #[derive_const(Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[derive(Copy, Debug, Hash)]
    #[repr(u128)]
    #[typekin::bitflag(konst = true, suffix = "s", with = [display])]
    pub enum MyFlag {
        Z = 0,
        A = 10,
        B,
        C = 40,
    }
}

type Subject = subject::MyFlag;
type Value = subject::MyFlags;

fn main() {
    let lhs = Subject::A;
    let rhs = Subject::B.into_value();

    assert_eq!(lhs.to_string(), "A");
    assert_eq!(lhs.into_value().to_string(), "10");
    println!("{:?}", (lhs & rhs) == (lhs & rhs));
    println!("{}", rhs & lhs);
    println!("{}", lhs & rhs);
    println!("{}", lhs);

    for x in Subject::items() {
        println!("{}", x.name());
    }
}
