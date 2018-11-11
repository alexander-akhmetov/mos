pub use self::simple_frame_allocator::SimpleFrameAllocator;

pub mod allocator;
pub mod paging;
mod simple_frame_allocator;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    index: usize,
}

impl Frame {
    fn get_for_address(address: paging::PhysicalAddress) -> Frame {
        // returns frame for a given physical memory address
        Frame {
            index: address / PAGE_SIZE,
        }
    }

    fn start_address(&self) -> paging::PhysicalAddress {
        self.index * PAGE_SIZE
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self);
}
