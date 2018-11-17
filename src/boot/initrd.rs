use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use boot::multiboot::get_module;
use core::slice;
use core::str;
use fs;
use multiboot2::BootInformation;
use multitasking::scheduler;
use sys;
use tar;
use x86;

pub fn init(boot_info: &BootInformation) {
    init_filesystem(boot_info);
    run_hello_bin();
    system_log_ok!("[initrd] loaded");
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
        "initrd loaded; files in '/initrd/': {:?}",
        fs::vfs::VFS.lock().list_dir("/initrd/")
    );
}

fn run_hello_bin() {
    let f = fs::vfs::VFS.lock().get_file("/initrd/hello_world.bin");
    if let Some(mut f) = f {
        unsafe {
            // todo: change me
            // memory are allocated forever, but when deallocate is implemented
            // this will stop work
            sys::elf::exec(f.read().as_ptr());
        };
    }
}
