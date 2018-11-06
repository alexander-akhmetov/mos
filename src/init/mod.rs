use sys;

pub fn hello_world() {
    kprintln!("--- before system call ---");

    // let's ask the mOS current time
    let timestamp = unsafe { sys::syscall::sys_time() };
    // and then send a debug message
    let syslog_call_result = unsafe { sys::syscall::sys_debug("Hello, mOS!") };
    kprintln!("--- after system call ---");

    // now let's print current time
    kprintln!("current time: {}", timestamp);
    kprintln!("syslog call result: {}", syslog_call_result);
}
