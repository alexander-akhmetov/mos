pub use self::simple_allocator::SimpleFrameAllocator;

mod simple_allocator;

pub const PAGE_SIZE: usize = 4096;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    index: usize,
}

impl Frame {
    fn get_for_address(address: usize) -> Frame {
        // returns frame for a given physical memory address
        Frame {index: address / PAGE_SIZE}
    }
}


pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self);
}
