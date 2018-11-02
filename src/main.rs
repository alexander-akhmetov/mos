#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#![no_std]  // disable rust std library
#![cfg_attr(not(test), no_main)]  // disable rust entry point

use core::panic::PanicInfo;

extern crate bootloader_precompiled;
extern crate volatile;
extern crate spin;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod vga_buffer;


#[panic_handler]
#[cfg(not(test))] // only compile when the test flag is not set
fn panic(info: &PanicInfo) -> ! {
    println!("[KERNEL PANIC] {}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // kernel entrypoint
    print!("Hello, world!\n");
    loop {}
}
