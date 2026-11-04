#![allow(unused, dead_code)]
#![feature(adt_const_params)]
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

pub use machinery::alloc::*;
pub use machinery::asm::*;
pub use machinery::asserts::*;
pub use machinery::fixture::*;
pub use machinery::ops::*;
pub use machinery::proc::*;

pub mod machinery;

mod asm_case_compile_time;
mod asm_case_domain_int;
mod asm_case_my_f32;
mod asm_case_my_f64;
mod asm_case_my_i32;
mod asm_case_my_sequence;
mod asm_case_my_u32;
mod asm_case_my_u64_const;
mod asm_case_my_unit;
mod asm_case_packed;
mod asm_case_within;

mod fx_bitflag;
mod fx_compile_fail;
mod fx_compile_pass;
mod fx_cross_asm;
mod fx_domain;
mod fx_float;
mod fx_friendship;
mod fx_hide;
mod fx_integral;
mod fx_integral_generic;
mod fx_integral_neg;
mod fx_lit;
mod fx_marker_tree;
mod fx_no_std;
mod fx_packed;
mod fx_pointer_width_compile;
mod fx_text;
mod fx_within;

mod test_bitflag;
mod test_domain;
mod test_float;
mod test_friendship;
mod test_hide;
mod test_integral;
mod test_integral_generic;
mod test_integral_neg;
mod test_lit;
mod test_marker_tree;
mod test_packed;
mod test_text;
mod test_within;

#[cfg(feature = "default")]
mod test_compile_fail;
#[cfg(feature = "default")]
mod test_compile_pass;
#[cfg(feature = "default")]
mod test_compile_time_asm;
#[cfg(feature = "default")]
mod test_cross_asm;
#[cfg(feature = "default")]
mod test_domain_asm;
#[cfg(feature = "default")]
mod test_float_asm;
#[cfg(feature = "default")]
mod test_integral_asm;
#[cfg(feature = "default")]
mod test_no_std;
#[cfg(feature = "default")]
mod test_packed_asm;
#[cfg(feature = "default")]
mod test_pointer_width_compile;
#[cfg(feature = "default")]
mod test_within_asm;
