#![no_std]
#![feature(start, alloc, panic_info_message)]

use core::panic::PanicInfo;
#[macro_use]
extern crate librust;

#[macro_use]
extern crate alloc;
extern crate alloc_dummy;

use core::alloc::{GlobalAlloc, Layout};

/*
    Just a simple program:
    prints "Hello, world!" and exits.
*/

#[start]
#[no_mangle]
fn _start(_argc: isize, _args: *const *const u8) -> isize {
    println!("[hello_world] first program is started!");
    let pid = unsafe { librust::syscall::getpid() };
    let time = unsafe { librust::syscall::time() };
    // let m = alloc::string::String::new();
    // println!("time: {}", time);
    // format!("123: {}", "456");
    // let layout = Layout::from_size_align(20, 4096).ok().unwrap();
    // unsafe {
    //     GLOBAL.alloc(layout);
    // }
    unsafe {
        let addr = librust::syscall::mmap(4096);
        for _i in 0..2 {
            librust::syscall::mmap(4096);
        }
        let mut n = addr as *mut u64;
        *n = 50;
    };
    println!("[hello_world] Hello, world!");
    println!("[hello_world] stopping...");
    return 0;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("ERROR: panic");
    loop {}
}

#[global_allocator]
static GLOBAL: alloc_dummy::DummyAlloc = alloc_dummy::DummyAlloc;
