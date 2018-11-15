#![no_std]
#![feature(alloc_error_handler)]
use core::alloc::GlobalAlloc;
use core::alloc::Layout;

const PREALLOCATED_HEAP_SIZE: usize = 4 * 1024 * 1024; // 4 Mb

#[repr(C)]
struct PreAllocatedMemory {
    heap: [u8; PREALLOCATED_HEAP_SIZE],
    index: usize,
}

impl PreAllocatedMemory {
    const fn new() -> PreAllocatedMemory {
        PreAllocatedMemory {
            heap: [0; PREALLOCATED_HEAP_SIZE],
            index: 0,
        }
    }
}

static mut PREALLOCATED_MEM: PreAllocatedMemory = PreAllocatedMemory::new();

pub struct DummyAlloc;

unsafe impl<'a> GlobalAlloc for &'a DummyAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let new_index = PREALLOCATED_MEM.index + layout.size();
        if new_index > PREALLOCATED_HEAP_SIZE {
            panic!("allocator: memory allocation error!")
        }

        let ptr = &mut PREALLOCATED_MEM.heap[PREALLOCATED_MEM.index] as *mut u8;
        PREALLOCATED_MEM.index = new_index;

        return ptr;
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
