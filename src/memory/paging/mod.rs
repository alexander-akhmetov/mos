use memory::PAGE_SIZE;
pub mod entry;
pub mod table;

const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
    number: usize,
}

pub fn translate(p4_address: u64, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
    let offset = virtual_address % PAGE_SIZE;
    None

    // let p4 =
}

/*

создать структуру процесса
хранить там адрес p4
сделать функцию получения физического адреса из виртуального
для процесса
*/
