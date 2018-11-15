#![no_std]
#![feature(lang_items, start)]

use core::panic::PanicInfo;
extern crate librust;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
#[no_mangle]
fn _start(argc: isize, args: *const *const u8) -> isize {
    unsafe { librust::syscall::debug("hello_world > Hello mOS!") };
    return 0;
}
