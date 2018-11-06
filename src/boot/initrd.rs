use alloc::string::String;
use core::slice;
use core::str;
use multiboot2::BootInformation;

pub fn init(boot_info: &BootInformation) {
    let initrd = boot_info
        .module_tags()
        .find(|m| m.name() == "initrd")
        .expect("can't find initrd!");

    let bytes = unsafe {
        slice::from_raw_parts(
            initrd.start_address() as *const u8,
            (initrd.end_address() - initrd.start_address()) as usize,
        )
    };
    system_log!("initrd loaded");
    print_as_str(bytes);
}

fn print_as_str(bytes: &[u8]) {
    let string = unsafe { str::from_utf8_unchecked(bytes) };
    system_log!("initrd: {:?}", string);
}
