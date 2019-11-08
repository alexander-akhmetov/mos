#![no_std]
#![feature(alloc_error_handler)]
use core::alloc::{GlobalAlloc, Layout};

#[macro_use]
extern crate librust;

extern crate compiler_builtins;

// const PAGE_SIZE: u64 = 4096;
const PAGE_SIZE: u64 = 1024 * 1024 * 2;

struct PreAllocatedMemory {
    current_addr: u64,
    max_addr: u64,
}

impl PreAllocatedMemory {
    const fn new() -> PreAllocatedMemory {
        PreAllocatedMemory {
            current_addr: 0,
            max_addr: 0,
        }
    }
}

static mut PREALLOCATED_MEM: PreAllocatedMemory = PreAllocatedMemory::new();

pub struct DummyAlloc;

unsafe impl GlobalAlloc for DummyAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        println!("[dummy_alloc] got alloc request");

        let new_current_addr = PREALLOCATED_MEM.current_addr + (layout.size() as u64);
        for _i in 0..layout.size() {}
        while new_current_addr >= PREALLOCATED_MEM.max_addr {
            println!("[dummy_alloc] requesting more memory from the OS...");
            // librust::syscall::debug_int(new_current_addr);
            PREALLOCATED_MEM.max_addr = librust::syscall::mmap(PAGE_SIZE);
        }

        PREALLOCATED_MEM.current_addr = new_current_addr;

        println!("[dummy_alloc] memory allocated");
        return new_current_addr as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        println!("[dummy_alloc] got dealloc request");
    }
}

#[alloc_error_handler]
#[no_mangle]
pub fn rust_oom(layout: Layout) -> ! {
    panic!(
        "!!! OOM: memory allocation of {} bytes failed",
        layout.size(),
    );
}
