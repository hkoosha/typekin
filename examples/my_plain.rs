#![allow(dead_code)]
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]

#[typekin::integral(konst = nightly)]
pub struct MyPlain(i64);

fn main() {
    println!("{:?}", MyPlain::make(123));
}
