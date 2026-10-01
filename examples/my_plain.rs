#![allow(dead_code)]
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]

mod subject {
    #[repr(transparent)]
    #[typekin::integral(konst = true)]
    #[derive_const(Clone)]
    #[derive(Copy)]
    pub struct MyPlain(i64);
}

type Subject = subject::MyPlain;

fn main() {
    println!("{:?}", Subject::make(123));
}
