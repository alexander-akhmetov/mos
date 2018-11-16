use alloc::string::String;
use librust::syscall;

pub fn hello_world() {
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
}
