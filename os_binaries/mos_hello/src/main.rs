#![no_std]
#![no_main]
#![feature(asm, start)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { asm!("mov eax, 13; int 0x80;" :::: "volatile", "intel"); };
    loop {}
}
