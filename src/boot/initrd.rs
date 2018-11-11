use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use boot::multiboot::get_module;
use core::slice;
use core::str;
use fs;
use multiboot2::BootInformation;
use multitasking::scheduler::{switch, SCHEDULER};
use sys;
use tar;
use x86;

pub fn init(boot_info: &BootInformation) {
    init_filesystem(boot_info);
    // run_hello_bin();
    system_log!("[init] ###### testing scheduler ######");
    unsafe {
        let rflags = x86::read_rflags();
        system_log!("[init] !!! RFLAGS: 0b{:b}", rflags);
    };
    test_scheduler();
    system_log!("initrd end");
    unsafe {
        switch();
    };
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
    let f = fs::vfs::VFS.lock().get_file("/initrd/asm_hello.bin");
    if let Some(f) = f {
        system_log!("asm_hello.bin: {:?}", f);
        let b = f.read();
        unsafe {
            sys::elf::exec(b.as_ptr());
        };
    }
}

fn test_scheduler() {
    for _i in 0..3 {
        SCHEDULER.write().spawn(foo as *const () as u64);
    }
}

extern "C" fn foo() {
    let mut counter = 0;
    system_log!(">> task_{} started", SCHEDULER.read().current_task_id());
    for _i in 0..5 {
        counter += 1;
        system_log!(
            ">> task_{}: hello {}",
            SCHEDULER.read().current_task_id(),
            counter,
        );
        for _j in 0..1000000 {}
        unsafe {
            let rflags = x86::read_rflags();
            system_log!(
                ">> task_{} RFLAGS: 0b{:b}",
                rflags,
                SCHEDULER.read().current_task_id()
            );
            switch();
        };
    }
    system_log!(
        ">> task_{}: completed, stopping...",
        SCHEDULER.read().current_task_id()
    );
}
