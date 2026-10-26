#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_clone)]
#![feature(const_destruct)]
#![feature(derive_const)]

#[typekin::integral(
    konst = nightly,
    friend = MyFriend::my_conv(MyFriend) -> [],
)]
pub struct MyU32(u32);

#[derive(Copy)]
#[derive_const(Clone)]
pub struct MyFriend(u32);

impl MyFriend {
    const fn my_conv(self) -> u32 {
        return self.0 * 2;
    }
}

fn main() {
    let lhs = 0b1101u32;
    let rhs = 0b0110u32;

    let lhs = MyU32::of(lhs);
    let rhs = MyU32::of(rhs);

    println!("{:?}", lhs + rhs);
}
