#![allow(unused, dead_code)]
#![feature(const_clone)]
#![feature(const_closures)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_iter)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(derive_const)]
#![feature(negative_impls)]

#[cfg(test)]
pub mod alloc;
#[cfg(test)]
pub mod asm;
#[cfg(test)]
pub mod asserts;
#[cfg(test)]
pub mod dir;
#[cfg(test)]
pub mod ops;
#[cfg(test)]
pub mod proc;

#[cfg(test)]
mod test_bitflag;
#[cfg(test)]
mod test_compile_fail;
#[cfg(test)]
mod test_friendship;
#[cfg(test)]
mod test_integral;
#[cfg(test)]
mod test_integral_asm;
#[cfg(test)]
mod test_integral_generic;
#[cfg(test)]
mod test_integral_generic_compile;
#[cfg(test)]
mod test_no_std;
#[cfg(test)]
mod test_text;

#[cfg(test)]
pub use crate::{
    alloc::*,
    asm::*,
    asserts::*,
    dir::*,
    ops::*,
    proc::*,
};

#[cfg(test)]
#[global_allocator]
static ALLOCATOR: ObservedAllocator = ObservedAllocator;
