use alloc::boxed::Box;
use boot::multiboot::get_module;
use core::slice;
use fs;
use multiboot2::BootInformation;
use tar;

pub fn init(boot_info: &BootInformation) {
    init_filesystem(boot_info);
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
