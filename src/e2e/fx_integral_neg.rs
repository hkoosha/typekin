use crate::*;

#[typekin::integral(konst = none)]
pub struct Signed(i32);

#[typekin::integral(konst = none, profile = sequential)]
pub struct Checked(i32);

#[typekin::integral(konst = none, profile = circular)]
pub struct Turn(i8);

#[typekin::integral(konst = none, domain = 0..=10)]
pub struct NonNegative(i32);

#[typekin::integral(konst = none, domain = -10..=10)]
pub struct Symmetric(i32);

#[typekin::integral(konst = none, domain = !{0})]
pub struct NotZero(i32);

#[typekin::integral(konst = none, domain = -10..=5)]
pub struct Lopsided(i32);

#[typekin::integral(konst = none)]
pub struct Wide(u64);

#[typekin::integral(konst = none)]
pub struct Huge(u128);

#[typekin::integral(konst = none)]
pub struct Count(u32);

#[typekin::integral(konst = none)]
pub struct Offset(i64);

#[typekin::integral(konst = none)]
pub struct Index(usize);
