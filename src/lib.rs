#![no_std]
#![feature(
    asm,
    const_fn,
    core_intrinsics,
    rustc_private,
    abi_x86_interrupt,
    naked_functions
)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![allow(dead_code, unused_imports)]

extern crate bootloader_precompiled;
extern crate multiboot2;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate uart_16550;
extern crate x86_64;
extern crate cstr_core;

#[macro_use]
mod vga_buffer;
#[macro_use]
mod logging;
#[macro_use]
mod serial;
mod cmos;
mod cpuio;
mod interrupts;
mod keyboard;
mod memory;
mod pic8259;
mod sys;

use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    system_log!("[KERNEL PANIC] {}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main(multiboot_information_address: usize) -> ! {
    // kernel entrypoint
    vga_buffer::clear_screen();
    kprintln!("Hello, world!");

    interrupts::init();
    unsafe {
        pic8259::PICS.lock().initialize();
    }
    interrupts::enable();

    // unsafe {
    //     asm!("int3");
    // }
    // divide_by_zero();

    unsafe {
        sys::lib::system_call_test();
    }

    kprintln!("It did not crash!");
    print_kernel_info(multiboot_information_address);

    unsafe {
        hlt_loop();
    }
}

fn print_kernel_info(multiboot_information_address: usize) {
    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    // let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf-sections tag required");
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();

    let kernel_end = elf_sections_tag
        .sections()
        .map(|s| s.addr + s.size)
        .max()
        .unwrap();
    system_log!(
        "kernel_start: 0x{:x}, kernel_end: 0x{:x}, bytes: {}",
        kernel_start,
        kernel_end,
        kernel_end - kernel_start
    );

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    system_log!(
        "multiboot_start: 0x{:x}, multiboot_end: 0x{:x}, bytes: {}",
        multiboot_start,
        multiboot_end,
        multiboot_end - multiboot_start
    );

    // let mut frame_allocator = memory::SimpleFrameAllocator::new(
    //     kernel_start as usize,
    //     kernel_end as usize,
    //     multiboot_start,
    //     multiboot_end,
    //     memory_map_tag.memory_areas(),
    // );
    // for i in 0.. {
    //     if let None = frame_allocator.allocate_frame() {
    //         system_log!("allocated {} frames", i);
    //         break;
    //     }
    // }
}

fn divide_by_zero() {
    unsafe { asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel") }
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
