#![no_std]
#![feature(start)]

extern crate librust;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
#[no_mangle]
fn _start(_argc: isize, _args: *const *const u8) -> isize {
    unsafe {
        librust::syscall::debug(">$     hello_world > Hello mOS!");
        librust::syscall::getpid();
    };
    return 0;
}
