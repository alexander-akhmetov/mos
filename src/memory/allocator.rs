use core::alloc::GlobalAlloc;
use core::alloc::Layout;

const PREALLOCATED_HEAP_SIZE: usize = 32 * 1024 * 1024; // 32Mb

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

pub struct MGlobalAlloc;

unsafe impl<'a> GlobalAlloc for &'a MGlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let new_index = PREALLOCATED_MEM.index + layout.size();
        if new_index > PREALLOCATED_HEAP_SIZE {
            panic!("allocator: memory allocation error!")
        }

        let ptr = &mut PREALLOCATED_MEM.heap[PREALLOCATED_MEM.index] as *mut u8;
        PREALLOCATED_MEM.index = new_index;
        // system_log!("[allocator]: current index: {}", new_index);

        // system_log!(
        //     "allocator: alloc called, allocated {} bytes at {:#X}",
        //     layout.size(),
        //     ptr as usize
        // );

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // system_log!(
        //     "allocator: dealloc called: deallocate {} bytes at {:#X}",
        //     layout.size(),
        //     ptr as usize
        // );
    }
}

#[cfg(not(test))]
#[alloc_error_handler]
#[no_mangle]
pub fn rust_oom(layout: Layout) -> ! {
    system_log!("! OOM: memory allocation of {} bytes failed", layout.size());
    loop {}
}
