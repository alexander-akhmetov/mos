use crate::memory::PAGE_SIZE;
use crate::x86;

pub mod entry;
pub mod table;

// 3 level paging: each frame address must be found through 3 pages:
// [P4 -> P3 -> P2] points to physical frame
const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
    number: usize,
}

pub fn translate(virtual_address: VirtualAddress) -> PhysicalAddress {
    system_log!(
        "[mem] 0x{:x}: translating to physical address",
        virtual_address
    );
    let offset = virtual_address % PAGE_SIZE;
    // 0o777 == 0b1_1111_1111, lets us to get only lower 9 bits
    let p4_offset = (virtual_address >> 39) & 0o777;
    let p3_offset = (virtual_address >> 30) & 0o777;
    let p2_offset = (virtual_address >> 21) & 0o777;

    system_log!(
        "[mem] 0x{:x}: p4_offset=0x{:x} p3_offset=0x{:x} p2_offset=0x{:x} offset=0x{:x}",
        virtual_address,
        p4_offset,
        p3_offset,
        p2_offset,
        offset,
    );

    unsafe {
        let p4_address = x86::read_cr3();
        // let p4_address = table::PageTable::new_address();
        system_log_debug!("[mem] P4 address from CR3: 0x{:x}", p4_address);
        let p4: *mut table::PageTable = p4_address as *mut table::PageTable;

        let p3_address = (*p4).next_table_address(p4_offset);
        system_log_debug!("[mem] P3 address: 0x{:x}", p3_address);
        let p3: *mut table::PageTable = p3_address as *mut table::PageTable;

        let p2_address = (*p3).next_table_address(p3_offset);
        system_log_debug!("[mem] P2 address: 0x{:x}", p2_address);
        let p2: *mut table::PageTable = p2_address as *mut table::PageTable;

        let addr = (*p2).next_table_address(p2_offset) + offset;
        system_log!(
            "[mem] virtual address=0x{:x}: physical address=0x{:x}",
            virtual_address,
            addr,
        );

        addr
    }
}
