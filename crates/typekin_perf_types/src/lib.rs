#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]

#[typekin::integral]
#[repr(transparent)]
#[derive(Copy)]
#[derive_const(Clone)]
pub struct Number(pub u32);
