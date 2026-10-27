#[typekin::integral(
    konst = none,
    friend = MyFriend::my_conv(MyFriend) -> [],
)]
pub struct MyU32(u32);

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

    let lhs = MyU32::of(lhs);
    let rhs = MyU32::of(rhs);
    let largest = MyU32::of(u32::MAX);
    #[cfg(target_pointer_width = "64")]
    assert_eq!(largest.into_isize(), u32::MAX as isize);
    #[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
    assert_eq!(largest.try_into_isize(), Err(()));

    println!("{:?}", lhs + rhs);
}
