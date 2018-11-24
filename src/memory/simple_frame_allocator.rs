use memory::{Frame, FrameAllocator};
use multiboot2::{MemoryArea, MemoryAreaIter};

// #[derive(Debug)]
// #[repr(C)]
// pub struct MemoryArea {
//     // multiboot2 memory entry structure
//     // http://nongnu.askapache.com/grub/phcoder/multiboot.pdf : "memory map"
//     length: u64,
//     base_addr: u64,

//     // type field description
//     //      1 - available RAM
//     //      3 - usable memory holding ACPI information
//     //      4 - reserved memory which needs to be preserved on hibernation
//     //      * other values currently indicated a reserved area
//     _type: u32,

//     _reserved: u32,  // set to 0 by the bootloader and must be ignored by OS
// }

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
        system_log!("[FrameAllocator] got allocation request");
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
                system_log!("[FrameAllocator] found kernel memory area");
                // memory is used by kernel
                self.next_free_frame = Frame {
                    index: self.kernel_end.index + 1,
                }
            } else if frame <= self.multiboot_end {
                system_log!("[FrameAllocator] found multiboot memory area");
                // memory is used by multiboot information structure
                self.next_free_frame = Frame {
                    index: self.multiboot_end.index + 1,
                }
            } else if frame <= self.initrd_end {
                system_log!("[FrameAllocator] found initrd memory area");
                // memory is used by multiboot information structure
                self.next_free_frame = Frame {
                    index: self.multiboot_end.index + 1,
                }
            } else {
                system_log!("[FrameAllocator] memory area has been allocated");
                self.next_free_frame.index += 1;
                return Some(frame);
            }

            // if we are here, that means we can' allocate current frame,
            // so let's try again with the new self.next_free_frame.index (we had to do += 1 before!)
            system_log!("[FrameAllocator] trying next frame...");
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
            next_free_frame: Frame::get_for_address(0),
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
        system_log!("[FrameAllocator] switching to the next area...");
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
