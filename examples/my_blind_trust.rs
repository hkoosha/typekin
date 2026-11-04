#[typekin::integral(
    friend = MyFriend::my_conv(MyFriend) -> unsafe,
)]
pub struct MyBlindTrust(u32);

#[derive(Copy, Clone)]
pub struct MyFriend(u32);

impl MyFriend {
    const fn my_conv(self) -> u32 {
        return self.0 * 2;
    }
}

fn main() {
    let lhs = MyFriend(0b1101u32);
    let rhs = MyFriend(0b0110u32);

    let lhs = MyBlindTrust::of(lhs);
    let rhs = MyBlindTrust::of(rhs);

    println!("{:?}", lhs + rhs);
}
