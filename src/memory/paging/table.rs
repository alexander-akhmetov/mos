use core::ops::{Index, IndexMut};
use memory::paging::entry::{Entry, EntryFlags};
use memory::paging::{PhysicalAddress, ENTRY_COUNT};
use memory::{FrameAllocator, FRAME_ALLOCATOR};

pub struct PageTable {
    entries: [Entry; ENTRY_COUNT],
}

impl PageTable {
    pub fn new_address() -> PhysicalAddress {
        unsafe {
            let frame = FRAME_ALLOCATOR.as_mut().unwrap().allocate_frame();
            let frame_address = frame.unwrap().start_address();
            let page: *mut PageTable = frame_address as *mut PageTable;
            (*page).zero();
            return frame_address;
        }
    }

    fn address(&self) -> u64 {
        return (self as *const _) as u64;
    }

    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }

    pub fn next_table_address(&mut self, index: usize) -> usize {
        let entry_flags = self[index].flags();

        if !entry_flags.contains(EntryFlags::PRESENT) {
            system_log_debug!("[page table]: creating a new page");
            unsafe {
                let frame = FRAME_ALLOCATOR.as_mut().unwrap().allocate_frame();
                let frame_address = frame.unwrap().start_address();
                let page: *mut PageTable = frame_address as *mut PageTable;
                (*page).zero();
                self[index].set(
                    (*page).address() as usize,
                    EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER_ACCESSIBLE,
                )
            }
        } else {
            system_log_debug!("[page table]: page already exists");
        }
        return self[index].pointed_frame().unwrap().start_address();
    }
}

impl Index<usize> for PageTable {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}
