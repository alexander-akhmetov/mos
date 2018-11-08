#![no_std]
#![no_main]
#![feature(asm, start)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
fn main(argc: isize, args: *const *const u8) -> isize {
    return 1;
}
