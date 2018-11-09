use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::slice;
use core::str;
use fs;
use multiboot2::BootInformation;
use sys;
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
    init_filesystem(boot_info);
    run_hello_bin();
}

fn init_filesystem(boot_info: &BootInformation) {
    let (start_address, end_address) = get_module(boot_info, "initrd");
    let bytes = unsafe {
        slice::from_raw_parts(
            start_address as *const u8,
            (end_address - start_address) as usize,
        )
    };

    let archive = tar::Archive::new(bytes);
    let tfs = fs::tarfs::TarFS::new(archive);
    fs::vfs::VFS.lock().mount("/initrd", Box::new(tfs));

    system_log!(
        "initrd loaded; files in '/initrd': {:?}",
        fs::vfs::VFS.lock().list_dir("/initrd")
    );
}

fn run_hello_bin() {
    let f = fs::vfs::VFS.lock().get_file("/initrd/asm_hello.bin");
    if let Some(f) = f {
        system_log!("asm_hello.bin: {:?}", f);
        let b = f.read();
        unsafe {
            sys::elf::exec(b.as_ptr());
        };
    }
}
