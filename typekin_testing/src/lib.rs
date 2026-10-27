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
#![feature(step_trait)]

pub use crate::{
    alloc::*,
    asserts::*,
    dir::*,
    ops::*,
};

#[cfg(feature = "test_slow_only")]
pub use crate::{
    asm::*,
    proc::*,
};

#[cfg(feature = "test_slow_only")]
pub mod asm;

#[cfg(feature = "test_slow_only")]
pub mod proc;

pub mod alloc;
pub mod asserts;
pub mod dir;
pub mod ops;

mod test_bitflag;
mod test_compile_fail;
mod test_friendship;
mod test_integral;
mod test_integral_asm;
mod test_integral_generic;
mod test_integral_generic_compile;
mod test_marker_tree;
mod test_no_std;
mod test_pointer_width_compile;
mod test_text;

#[global_allocator]
static ALLOCATOR: ObservedAllocator = ObservedAllocator;
