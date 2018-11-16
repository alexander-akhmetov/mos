use librust;

pub fn hello_world() {
    system_log!("[init] --- before system call ---");

    // let's ask the mOS current time
    let timestamp = unsafe { librust::syscall::time() };
    // and then send a debug message
    let syslog_call_result = unsafe { librust::syscall::debug("Hello, mOS!") };
    let uname_info = librust::syscall::UtsName::new();
    unsafe { librust::syscall::uname(&uname_info) };
    system_log!("[init] --- after system call ---");

    system_log!("[init] uname: {}", uname_info);
    // now let's print current time
    system_log!("[init] current time: {}", timestamp);
    system_log!("[init] syslog call result: {}", syslog_call_result);
}
