#![no_std]
#![feature(alloc_error_handler)]
use core::alloc::{GlobalAlloc, Layout};

const PREALLOCATED_MEM_SIZE: usize = 2 * 1024 * 1024; // 2 Mb

struct PreAllocatedMemory {
    buf: [u8; PREALLOCATED_MEM_SIZE],
    index: usize,
}

impl PreAllocatedMemory {
    const fn new() -> PreAllocatedMemory {
        PreAllocatedMemory {
            buf: [0; PREALLOCATED_MEM_SIZE],
            index: 0,
        }
    }
}

static mut PREALLOCATED_MEM: PreAllocatedMemory = PreAllocatedMemory::new();

pub struct DummyAlloc;

unsafe impl GlobalAlloc for DummyAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let new_index = PREALLOCATED_MEM.index + layout.size();
        if new_index > PREALLOCATED_MEM_SIZE {
            panic!("allocator: memory allocation error!")
        }

        let ptr = &mut PREALLOCATED_MEM.buf[PREALLOCATED_MEM.index] as *mut u8;
        PREALLOCATED_MEM.index = new_index;

        return ptr;
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
