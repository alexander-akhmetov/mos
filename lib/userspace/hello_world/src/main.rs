#![no_std]
#![feature(start, alloc, panic_info_message)]

use core::panic::PanicInfo;
#[macro_use]
extern crate librust;

#[macro_use]
extern crate alloc;
extern crate alloc_dummy;

/*
    Just a simple program:
    prints "Hello, world!" and exits.
*/

#[start]
#[no_mangle]
fn _start(_argc: isize, _args: *const *const u8) -> isize {
    unsafe {
        println!("[hello_world] first program is started!");
        let pid = librust::syscall::getpid();
        let time = librust::syscall::time();
        // let mut vec = alloc::vec::Vec::new();
        // vec.push(1);
        // vec.push(2);
        // println!("time: {}", time);
        let addr = librust::syscall::mmap(4096);
        for _i in 0..2 {
            librust::syscall::mmap(4096);
        }
        let n = addr as *mut u64;
        *n = 50;
        println!("[hello_world] Hello, world!");
        println!("[hello_world] stopping...");
        return 0;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    librust::std::screen::print("PANIC!");
    loop {}
}

#[global_allocator]
static mut ALLOCATOR: alloc_dummy::DummyAlloc = alloc_dummy::DummyAlloc;
