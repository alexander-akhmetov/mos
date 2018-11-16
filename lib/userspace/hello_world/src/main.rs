#![no_std]
#![feature(start, alloc)]

extern crate alloc_dummy;
extern crate librust;
use core::panic::PanicInfo;
extern crate alloc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static GLOBAL: alloc_dummy::DummyAlloc = alloc_dummy::DummyAlloc;

#[start]
#[no_mangle]
fn _start(_argc: isize, _args: *const *const u8) -> isize {
    unsafe {
        // doesn't work yet... :-(
        // let s = alloc::string::String::from("tst");
        librust::syscall::debug(">$     hello_world > Hello mOS!");
        // librust::syscall::debug(&s);
        librust::syscall::getpid();
    };
    return 0;
}
