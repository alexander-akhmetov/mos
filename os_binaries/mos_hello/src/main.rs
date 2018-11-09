#![feature(asm, start)]
#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
fn main(argc: isize, args: *const *const u8) -> isize {
    return 1;
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
