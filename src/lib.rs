#![no_std]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![feature(rustc_private)]

#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate uart_16550;
extern crate x86_64;

#[macro_use]
mod vga_buffer;
#[macro_use]
mod serial;
mod interrupts;
mod cpuio;
mod keyboard;
mod pic8259;

use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(test))] // only compile when the test flag is not set
fn panic(info: &PanicInfo) -> ! {
    kprintln!("[KERNEL PANIC] {}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern fn main() -> ! {
    // kernel entrypoint
    vga_buffer::clear_screen();
    kprintln!("Hello, world!");

    interrupts::init();
    unsafe { pic8259::PICS.lock().initialize(); }
    x86_64::instructions::interrupts::enable();

    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    let mut keyboard = keyboard::polling::PollingKeyboard::new(print_char);
    loop {
        keyboard.update();
    }
}


fn print_char(c: char) {
    kprint!("{}", c);
}

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}
