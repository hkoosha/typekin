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

use std::fmt::Debug;
use std::hash::Hash;
use std::path::Path;
use std::process::Command;

pub mod alloc;
pub mod asm;
pub mod asserts;
pub mod dir;
pub mod ops;
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
mod test_no_std;
#[cfg(test)]
mod test_text;

pub use crate::alloc::*;
pub use crate::asm::*;
pub use crate::asserts::*;
pub use crate::dir::*;
pub use crate::ops::*;
pub use crate::proc::*;

#[global_allocator]
static ALLOCATOR: ObservedAllocator = ObservedAllocator;
