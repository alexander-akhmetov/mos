#![no_std]
#![feature(
    asm,
    const_fn,
    core_intrinsics,
    rustc_private,
    abi_x86_interrupt,
    naked_functions,
    alloc,
    allocator_api,
    lang_items,
    alloc_error_handler
)]
#![allow(dead_code, unused_imports, unused_doc_comments)]

extern crate multiboot2;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate uart_16550;
extern crate x86_64;

#[macro_use]
extern crate alloc;
extern crate tar;

#[macro_use]
mod drivers;
#[macro_use]
mod logging;
mod boot;
mod cmos;
mod cpuio;
mod init;
mod memory;
mod pic8259;
mod sys;
mod utils;
mod x86;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

use core::panic::PanicInfo;

// this allows to use MGlobalAlloc for dynamic data structures
// Rust will call allocator when it's needed.
#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: &'static memory::allocator::MGlobalAlloc =
    &memory::allocator::MGlobalAlloc;

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    /// this function is called when kernel panic occurs
    // it will print red message with panic information
    let color = drivers::vga_buffer::ColorCode::new(
        drivers::vga_buffer::Color::Red,
        drivers::vga_buffer::Color::Black,
    );

    kprintln_color!(color, "\n-----------\n[KERNEL PANIC] {}\n-----------", info);

    unsafe {
        x86::hlt_loop();
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main(multiboot_information_address: usize) -> ! {
    /// kernel entrypoint
    // This function got "multiboot_information_address" as a parameter,
    // because when a multiboot bootloader loads a kernel, it passes a
    // pointer to a boot information structure in the ebx register.
    // And regarding to C calling convention:
    //      >
    //      > The first six integer or pointer arguments are
    //      > passed in registers RDI, RSI, RDX, RCX, R8, and R9
    //      >
    // So in the boot asm code I set it: "mov edi, ebx;".
    drivers::vga_buffer::clear_screen();
    system_log!("kernel loading...");

    // init IDT table, so the CPU will know about our interrupt handlers
    sys::interrupts::init();
    // init PIC to start getting interrupts from timer, keyboard, etc.
    unsafe {
        pic8259::PICS.lock().initialize();
    }
    // finish enabling interrupts
    sys::interrupts::enable();

    // print multiboot info
    boot::multiboot::print_multiboot_info(multiboot_information_address);

    // load initrd
    let boot_info = boot::multiboot::get_multiboot_info(multiboot_information_address);
    boot::initrd::init(&boot_info);

    // and not the OS is ready
    system_log_without_prefix!("----------------------------");
    system_log!("kernel started");

    // ----------------------- test commands
    // run init "hello world" command
    init::hello_world();
    // allocator_test creates dynamic data structures to check that it works
    // utils::allocator_test();
    // -------------------------------------

    // loop with hlt forever
    unsafe {
        x86::hlt_loop();
    }
}
