#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_destruct)]
#![feature(derive_const)]

mod subject {
    #[typekin::integral(
        konst = true,
        friends = MyFriend::my_conv(MyFriend) -> [],
    )]
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    pub struct MyU32(u32);

    #[repr(transparent)]
    #[derive(Copy)]
    #[derive_const(Clone)]
    pub struct MyFriend(u32);

    impl MyFriend {
        const fn my_conv(self) -> u32 {
            return self.0 * 2;
        }
    }

    impl std::fmt::Display for MyU32 {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "MyU16({})", self.raw())
        }
    }
}

use subject::MyU32 as Subject;

fn main() {
    let lhs = 0b1101u32;
    let rhs = 0b0110u32;

    let lhs = Subject::of(lhs);
    let rhs = Subject::of(rhs);

    println!("{:?}", lhs + rhs);
}
