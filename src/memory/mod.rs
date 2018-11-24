pub use self::simple_frame_allocator::SimpleFrameAllocator;
use multiboot2::MemoryAreaIter;

pub mod allocator;
pub mod paging;
mod simple_frame_allocator;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    index: usize,
}

impl Frame {
    pub fn get_for_address(address: paging::PhysicalAddress) -> Frame {
        // returns frame for a given physical memory address
        Frame {
            index: address / PAGE_SIZE,
        }
    }

    pub fn start_address(&self) -> paging::PhysicalAddress {
        self.index * PAGE_SIZE
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self);
}

pub static mut FRAME_ALLOCATOR: Option<SimpleFrameAllocator> = None;

pub fn init_frame_allocator(
    kernel_start: usize,
    kernel_end: usize,
    multiboot_start: usize,
    multiboot_end: usize,
    initrd_start: usize,
    initrd_end: usize,
    memory_areas: MemoryAreaIter,
) {
    unsafe {
        FRAME_ALLOCATOR = Some(SimpleFrameAllocator::new(
            kernel_start,
            kernel_end,
            multiboot_start,
            multiboot_end,
            initrd_start,
            initrd_end,
            memory_areas,
        ));
    }
    system_log_ok!("[frame allocator] initiated");
}
