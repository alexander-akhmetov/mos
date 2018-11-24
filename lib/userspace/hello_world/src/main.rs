#![no_std]
#![feature(start, alloc, panic_info_message)]
use core::alloc::GlobalAlloc;
use core::alloc::Layout;

extern crate alloc_dummy;
use core::panic::PanicInfo;
#[macro_use]
extern crate librust;

#[macro_use]
extern crate alloc;

/*
    Just a simple program:
    prints "Hello, world!" and exits.
*/

#[start]
#[no_mangle]
fn _start(_argc: isize, _args: *const *const u8) -> isize {
    unsafe {
        let pid = librust::syscall::getpid();
        let time = librust::syscall::time();
        let m = alloc::string::String::new();
        librust::std::screen::print("Hello, world!");
    }
    return 0;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        librust::syscall::debug("ERROR: panic");
    };
    loop {}
}

#[global_allocator]
static GLOBAL: alloc_dummy::DummyAlloc = alloc_dummy::DummyAlloc;
