use core::ops::{Index, IndexMut};
use memory::paging::entry::{Entry, EntryFlags};
use memory::paging::ENTRY_COUNT;

pub struct PageTable {
    entries: [Entry; ENTRY_COUNT],
}

impl PageTable {
    pub fn new() -> PageTable {
        PageTable {
            entries: [Entry::new(); ENTRY_COUNT],
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

    fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();

        if entry_flags.contains(EntryFlags::PRESENT) {
            let table_address = self as *const _ as usize;
            Some((table_address << 9) | (index << 12))
        } else {
            None
        }
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
