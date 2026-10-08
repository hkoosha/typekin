#[typekin::integral(konst = false)]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct MyNonConstU32(u32);

pub struct Verified(u32);

impl Verified {
    pub fn try_make(raw: u32) -> Option<Self> {
        return (1..=100).contains(&raw).then_some(Self(raw));
    }

    fn into_raw(self) -> u32 {
        return self.0;
    }
}

#[typekin::integral(
    konst = false,
    in = 1..=100,
    friends = Verified::into_raw(Verified) -> [Make, Trust],
)]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Limited(u32);

fn main() {
    let lhs = 0b1101u32;
    let rhs = 0b0110u32;

    let lhs = MyNonConstU32::of(lhs);
    let rhs = MyNonConstU32::of(rhs);
    println!("{:?}", lhs + rhs);

    let verified = Verified::try_make(42).unwrap();
    // The non-Copy friend moves its already-validated value into Limited.
    let limited = Limited::of(verified);
    assert_eq!(limited.raw(), 42);
    assert_eq!(Limited::try_make(0), Err(0));
}
