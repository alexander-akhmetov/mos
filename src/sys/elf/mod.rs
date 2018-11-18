use multitasking::scheduler;
use x86;

#[derive(Default)]
#[repr(packed)]
pub struct ELFHeader {
    magic: [u8; 4],
    class: u8,
    endianness: u8,
    version: u8,
    os_abi: u8,
    abi_version: u8,
    unused: [u8; 7],
    elf_type: u16,
    machine: u16,
    version2: u32,
    pub entry_point: u64,
    pub phoff: u64, // program Header offset
    shoff: u64,     // pection Header offset
    flags: u32,
    header_size: u16,
    pub phentsize: u16, // Program header entry size
    pub phnum: u16,     // Program header entry count
    shentsize: u16,     // Section header entry size
    shnum: u16,         // Section header entry count
    e_shstrndx: u16,
}

pub fn read_header<'a>(addr: *const u8) -> &'a ELFHeader {
    unsafe {
        let x: *const ELFHeader = addr as *const ELFHeader;
        return &*x;
    }
}

pub unsafe fn exec(addr: *const u8) {
    let call_addr = get_elf_entrypoint(addr);
    system_log!("Executing ELF: entry_point: 0x{:x}", call_addr);
    // scheduler::spawn_addr(call_addr);
    x86::call(call_addr);
    system_log!("Executed ELF: entry_point: 0x{:x}", call_addr);
}

pub unsafe fn get_elf_entrypoint(addr: *const u8) -> u64 {
    let header = read_header(addr);
    system_log!(
        "Parsed ELF file, entry_point: 0x{:x}, addr: 0{:x}",
        header.entry_point,
        addr as u64,
    );
    // how to find offset?
    let call_addr = addr.offset(0x00001000) as u64;
    return call_addr;
}

#[test]
fn test_read_elf_from_file() {
    use alloc::vec::Vec;
    use std::fs::File;
    use std::io::Read;
    use std::println;

    let mut f = File::open("initrd/hello_world").expect("file not found");
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf).unwrap();

    let header = read_header(buf.as_ptr());
    let entry_point = header.entry_point;

    // Check the magic bytes
    println!(
        "-----
        endianness: {:?}
        entry_point: 0x{:x}
        swapped_bytes: 0x{:x}
        ------",
        header.endianness,
        entry_point,
        u64::swap_bytes(entry_point as u64),
    );
    // assert_eq!(entry_point, 0x2010e0);
}
