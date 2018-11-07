use alloc::string::String;
use alloc::vec::Vec;
use core::slice;
use core::str;
use multiboot2::BootInformation;
use tar;
use x86;

fn get_module(boot_info: &BootInformation, name: &str) -> (u64, u64) {
    let module = boot_info
        .module_tags()
        .find(|m| m.name() == name)
        .expect("Can't find module");

    (module.start_address() as u64, module.end_address() as u64)
}

pub fn init(boot_info: &BootInformation) {
    let (start_address, end_address) = get_module(boot_info, "initrd");
    let bytes = unsafe {
        slice::from_raw_parts(
            start_address as *const u8,
            (end_address - start_address) as usize,
        )
    };

    let archive = tar::Archive::new(bytes);
    let files_iter = archive.files();
    for f in files_iter {
        system_log!("--- initrd file ---");
        system_log!("name: {}", f.name());
        system_log!("size: {}", f.size());
        system_log!("mtime: {}", f.mtime());
        // let s = unsafe { String::from_utf8_unchecked(f.content.to_vec()) };
        // system_log!("content string: '{}'", s);
    }

    system_log!("initrd loaded; size: {}", bytes.len());
}

pub fn hello_asm(boot_info: &BootInformation) {
    let (start_address, end_address) = get_module(boot_info, "hello");
    let bytes = unsafe {
        slice::from_raw_parts(
            start_address as *const u8,
            (end_address - start_address) as usize,
        )
    };

    let vec = bytes.to_vec().clone();
    let ptr = &vec as *const _ as u64;
    unsafe {
        x86::jmp(ptr);
    };

    system_log!("module hello (asm) loaded with size: {}", bytes.len());
    // unsafe { x86::jmp(start_address); };
}
