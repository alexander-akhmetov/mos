#![no_std]
#![feature(asm, const_fn, core_intrinsics, rustc_private, abi_x86_interrupt, no_std)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
extern crate multiboot2;
#[macro_use]
extern crate lazy_static;
extern crate uart_16550;
extern crate x86_64;

#[macro_use]
mod vga_buffer;
#[macro_use]
mod logging;
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
    system_log!("[KERNEL PANIC] {}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern fn main(multiboot_information_address: usize) -> ! {
    // kernel entrypoint
    vga_buffer::clear_screen();
    kprintln!("Hello, world!");

    interrupts::init();
    unsafe { pic8259::PICS.lock().initialize(); }
    interrupts::enable();

    // unsafe {
    //     asm!("int3");
    // }
    // divide_by_zero();

    kprintln!("It did not crash!");
    print_kernel_info(multiboot_information_address);

    unsafe { hlt_loop(); }
}


fn print_kernel_info(multiboot_information_address: usize) {
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    system_log!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        system_log!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required");
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();
    system_log!("kernel_start: 0x{:x}, kernel_end: 0x{:x}, bytes: {}",
                kernel_start, kernel_end, kernel_end - kernel_start);

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    system_log!("multiboot_start: 0x{:x}, multiboot_end: 0x{:x}, bytes: {}",
                multiboot_start, multiboot_end, multiboot_end - multiboot_start);
}


fn divide_by_zero() {
    unsafe {
        asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
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


/// Halts the CPU by executing the `hlt` instruction.
#[inline(always)]
pub unsafe fn hlt_loop() -> ! {
    loop {
        asm!("hlt" :::: "volatile");
    }
}
