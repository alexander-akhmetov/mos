#![no_std]
#![feature(
    asm,
    custom_inner_attributes,
    custom_attribute,
    alloc,
    extern_crate_item_prelude
)]
#![allow(unused_doc_comments, dead_code, unused_variables)]

extern crate alloc;
pub mod collections;
pub mod std;
pub mod syscall;
