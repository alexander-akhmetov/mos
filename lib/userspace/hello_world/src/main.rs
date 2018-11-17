#![no_std]
#![feature(start)]

extern crate alloc_dummy;
use core::panic::PanicInfo;
#[macro_use]
extern crate librust;

/*
    Just a simple program which prints "Hello, world!"
    and exits.
*/

#[start]
#[no_mangle]
fn _start(_argc: isize, _args: *const *const u8) -> isize {
    println!("Hello, world!");
    return 0;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static GLOBAL: alloc_dummy::DummyAlloc = alloc_dummy::DummyAlloc;
