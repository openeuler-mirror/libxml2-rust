#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_unsafe
)]

extern crate libc;
// extern crate f128;

include!("defination.rs");
include!("ffi.rs");
include!("ffi_safe.rs");

include!("parserInternals.rs");
include!("parserInternals_rust.rs");
include!("parser.rs");
include!("parser_part.rs");
include!("parser_rust.rs");
include!("HTMLparser.rs");
include!("HTMLparser_rust.rs");
include!("xpath.rs");
include!("xpath_rust.rs");
