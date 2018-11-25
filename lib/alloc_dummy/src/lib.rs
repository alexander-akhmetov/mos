#![no_std]
#![feature(alloc_error_handler)]
use core::alloc::{GlobalAlloc, Layout};

#[macro_use]
extern crate lazy_static;

use librust;

const PAGE_SIZE: u64 = 4096;
const PREALLOCATED_SIZE: usize = 1024; // * 4096 (page size) = 4 mb

#[repr(C)]
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

impl DummyAlloc {
    unsafe fn init(&self) {
        let current_addr = librust::syscall::mmap(PAGE_SIZE);
        let mut max_addr = current_addr;
        for _i in 0..PREALLOCATED_SIZE {
            max_addr = librust::syscall::mmap(PAGE_SIZE);
        }
        PREALLOCATED_MEM.current_addr = current_addr;
        PREALLOCATED_MEM.max_addr = max_addr;
    }
}

unsafe impl GlobalAlloc for DummyAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if PREALLOCATED_MEM.current_addr == 0 {
            self.init();
        }

        let new_current_addr = PREALLOCATED_MEM.current_addr + layout.size() as u64;
        if new_current_addr > PREALLOCATED_MEM.max_addr {
            panic!("allocator: memory allocation error!")
        }

        PREALLOCATED_MEM.current_addr = new_current_addr;

        return new_current_addr as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {}
}

#[alloc_error_handler]
#[no_mangle]
pub fn rust_oom(layout: Layout) -> ! {
    panic!(
        "!!! OOM: memory allocation of {} bytes failed",
        layout.size(),
    );
}
