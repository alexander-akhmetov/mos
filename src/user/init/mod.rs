use alloc::string::String;
use fs;
use librust::syscall;
use sys;

pub fn hello_world() {
    /// test function, makes a few system calls and prints the results
    system_log!("[init] --- before system call ---");

    unsafe {
        let uname_info = syscall::UtsName::new();

        let timestamp = syscall::time();
        let syslog_call_result = syscall::debug("Hello, mOS!");
        syscall::uname(&uname_info);

        let s = String::from(">> hello, stdout!");
        syscall::write(syscall::STDOUT, s.as_bytes(), s.len() as u64);

        system_log!("[init] --- after system call ---");
        system_log!("[init] uname: {}", uname_info);
        system_log!("[init] current time: {}", timestamp);
        system_log!("[init] syslog call result: {}", syslog_call_result);
    }

    run_hello_bin();
}

fn run_hello_bin() {
    /// starts initrd/hello_world binary (executes elf file)
    let f = fs::vfs::VFS.lock().get_file("/initrd/hello_world");
    if let Some(mut f) = f {
        unsafe {
            // todo: change me
            // memory are allocated forever, but when deallocate will be implemented
            // this stops working
            sys::elf::exec(f.read().as_ptr());
        };
    }
}
