#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern crate libc;
#[macro_use]
extern crate rust_ffi;

mod rust_core;
mod rust_method;
