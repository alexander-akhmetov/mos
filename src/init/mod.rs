use sys;

pub fn hello_world() {
    kprintln!("[init] --- before system call ---");

    // let's ask the mOS current time
    let timestamp = unsafe { sys::syscall::sys_time() };
    // and then send a debug message
    let syslog_call_result = unsafe { sys::syscall::sys_debug("Hello, mOS!") };
    unsafe { sys::syscall::sys_switch() };
    kprintln!("[init] --- after system call ---");

    // now let's print current time
    kprintln!("[init] current time: {}", timestamp);
    kprintln!("[init] syslog call result: {}", syslog_call_result);
}
