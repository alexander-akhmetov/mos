#![no_std]

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate uart_16550;
extern crate x86_64;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

pub mod vga_buffer;
pub mod serial;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}
