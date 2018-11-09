use alloc::vec::Vec;
use core::slice;
use x86;

#[repr(C, packed)]
pub struct ELFIdent {
    ei_mag: [u8; 4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8; 7],
}

#[repr(C, packed)]
pub struct ELFHeader {
    e_ident: ELFIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

pub fn read_header<'a>(addr: *const u8) -> &'a ELFHeader {
    unsafe {
        let x: *const ELFHeader = addr as *const ELFHeader;
        return &*x;
    }
}

pub unsafe fn exec(addr: *const u8) {
    let header = read_header(addr);
    let call_addr = addr as u64 + header.e_entry as u64;
    system_log!("Executing ELF: e_entry: {:x}", call_addr);
    // x86::jmp(call_addr);
    // system_log!("Executed ELF: e_entry: {:x}", call_addr);
}

#[test]
fn test_read_elf_from_file() {
    use std::fs::File;
    use std::io::Read;
    use std::println;

    let mut f = File::open("initrd/asm_hello.bin").expect("file not found");
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf).unwrap();

    let header = read_header(buf.as_ptr());

    // Check the magic bytes
    println!(
        "header.e_ident.ei_mag: {:?} header.e_entry: {:?}, version: {:?}, ei_osabi: {:?}, e_machine: {:?}",
        header.e_ident.ei_mag, header.e_entry, header.e_version, header.e_ident.ei_osabi, header.e_machine,
    );
    assert!(header.e_ident.ei_mag[1..4] == *"ELF".as_bytes());
}
