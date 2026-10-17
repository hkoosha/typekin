#[typekin::integral(
    konst = stable,
    friends = MyFriend::my_conv(MyFriend) -> [],
)]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MyU32(u32);

#[repr(transparent)]
#[derive(Clone, Copy)]
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

    println!("{}", (lhs + rhs).raw());
}
