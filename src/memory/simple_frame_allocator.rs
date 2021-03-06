use crate::memory::{Frame, FrameAllocator};
use multiboot2::{MemoryArea, MemoryAreaIter};

pub struct SimpleFrameAllocator {
    next_free_frame: Frame,
    current_area: Option<&'static MemoryArea>,
    areas: MemoryAreaIter,
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
    initrd_start: Frame,
    initrd_end: Frame,
}

impl FrameAllocator for SimpleFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        system_log_debug!("[FrameAllocator] got allocation request");
        if let Some(area) = self.current_area {
            // at first we create a frame structure
            let frame = Frame {
                index: self.next_free_frame.index,
            };
            let current_area_last_frame = {
                let address = area.base_addr + area.length - 1;
                Frame::get_for_address(address as usize)
            };

            if frame > current_area_last_frame {
                self.switch_to_next_memory_area();
            } else if frame <= self.kernel_end {
                system_log_debug!("[FrameAllocator] found kernel memory area");
                // memory is used by kernel
                self.next_free_frame = Frame {
                    index: self.kernel_end.index + 1,
                }
            } else if frame <= self.multiboot_end {
                system_log_debug!("[FrameAllocator] found multiboot memory area");
                // memory is used by multiboot information structure
                self.next_free_frame = Frame {
                    index: self.multiboot_end.index + 1,
                }
            } else if frame <= self.initrd_end {
                system_log_debug!("[FrameAllocator] found initrd memory area");
                // memory is used by multiboot information structure
                self.next_free_frame = Frame {
                    index: self.initrd_end.index + 1,
                }
            } else {
                system_log_debug!("[FrameAllocator] memory area has been allocated");
                self.next_free_frame.index += 1;
                return Some(frame);
            }

            // if we are here, that means we can' allocate current frame,
            // so let's try again with the new self.next_free_frame.index (we had to do += 1 before!)
            system_log_debug!("[FrameAllocator] trying next frame...");
            self.allocate_frame()
        } else {
            None // no frames left
        }
    }

    fn deallocate_frame(&mut self) {
        unimplemented!()
    }
}

impl SimpleFrameAllocator {
    pub fn new(
        kernel_start: usize,
        kernel_end: usize,
        multiboot_start: usize,
        multiboot_end: usize,
        initrd_start: usize,
        initrd_end: usize,
        memory_areas: MemoryAreaIter,
    ) -> SimpleFrameAllocator {
        let mut allocator = SimpleFrameAllocator {
            next_free_frame: Frame::get_for_address(initrd_end),
            current_area: None,
            areas: memory_areas,
            kernel_start: Frame::get_for_address(kernel_start),
            kernel_end: Frame::get_for_address(kernel_end),
            multiboot_start: Frame::get_for_address(multiboot_start),
            multiboot_end: Frame::get_for_address(multiboot_end),
            initrd_start: Frame::get_for_address(initrd_start),
            initrd_end: Frame::get_for_address(initrd_end),
        };

        allocator.switch_to_next_memory_area();
        allocator
    }

    fn switch_to_next_memory_area(&mut self) {
        system_log_debug!("[FrameAllocator] switching to the next area...");
        self.current_area = self
            .areas
            .clone()
            .filter(|area| {
                let address = area.base_addr + area.length - 1;
                Frame::get_for_address(address as usize) >= self.next_free_frame
            })
            .min_by_key(|area| area.base_addr);

        if let Some(area) = self.current_area {
            let start_frame = Frame::get_for_address(area.base_addr as usize);
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}
