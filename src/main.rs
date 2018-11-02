#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#![feature(asm)]
#![no_std]  // disable rust std library
#![cfg_attr(not(test), no_main)]  // disable rust entry point

#[macro_use]
extern crate mos;

use core::panic::PanicInfo;


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

    // initialize IDT
    mos::interrupts::init();

    // divide_by_zero();
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    println!("It did not crash!");

    loop {}
}


fn divide_by_zero() {
    unsafe {
        asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
    }
}
