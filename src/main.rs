#![no_std]  // disable rust std library
#![no_main]  // disable rust entry point

use core::panic::PanicInfo;

extern crate bootloader_precompiled;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, World!";

fn print_hello_world() {
    // 0xb8000 is the VGA buffer's address
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // kernel entrypoint

    print_hello_world();
    loop {}
}
