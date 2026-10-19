#[typekin::integral(
    friends = MyFriend::my_conv(MyFriend) -> unsafe,
)]
pub struct MyUnchecked(u32);

#[derive(Copy, Clone)]
pub struct MyFriend(u32);

impl MyFriend {
    const fn my_conv(self) -> u32 {
        return self.0 * 2;
    }
}

fn main() {
    let lhs = 0b1101u32;
    let rhs = 0b0110u32;

    let lhs = MyUnchecked::of(lhs);
    let rhs = MyUnchecked::of(rhs);

    println!("{:?}", lhs + rhs);
}
