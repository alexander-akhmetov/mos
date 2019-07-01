use memory::paging;
use memory::Frame;

bitflags! {
    pub struct EntryFlags: u64 {
        const PRESENT =         1 << 0;  // the page is currently in memory
        const WRITABLE =        1 << 1;  // it's allowed to write to this address
        const USER_ACCESSIBLE = 1 << 2;  // it's accessible from userspace
        const WRITE_THROUGH =   1 << 3;  // writes go directly to memory ?
        const NO_CACHE =        1 << 4;  // disable cache
        const ACCESSED =        1 << 5;  // CPU sets this bit when page is used
        const DIRTY =           1 << 6;  // CPU sets this bit when write occurs
        const HUGE_PAGE =       1 << 7;  // must be 0 in P1 and P4, creates a 1GiB page in P3, creates a 2MiB page in P2
        const GLOBAL =          1 << 8;  // page isn't flushed from caches on address space switch (PGE bit of CR4 register must be set)
        // 9-11 can be used by OS
        // 12-51 physical address (40 bits)
        // 52-62 can be used by OS
        const NO_EXECUTE =      1 << 63; // forbid executing code on this page
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Entry(u64);

impl Entry {
    pub fn new() -> Entry {
        Entry { 0: 0 }
    }

    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }

    pub fn pointed_frame(&self) -> Option<Frame> {
        // extracts physical address from the entry (12-51 bits)
        // and returns the Frame
        // if the page is present, else returns None
        if self.flags().contains(EntryFlags::PRESENT) {
            Some(Frame::get_for_address(
                self.address()
            ))
        } else {
            None
        }
    }

    pub fn address(&self) -> usize {
        self.0 as usize & 0x000fffff_fffff000
    }

    pub fn set(&mut self, frame_address: paging::PhysicalAddress, flags: EntryFlags) {
        // the start address of a frame should be page aligned and smaller than 2^52
        assert!(frame_address & !0x000fffff_fffff000 == 0);
        self.0 = (frame_address as u64) | flags.bits();
    }
}
