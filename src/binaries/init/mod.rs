use librust;

pub fn hello_world() {
    kprintln!("[init] --- before system call ---");

    // let's ask the mOS current time
    let timestamp = unsafe { librust::syscall::time() };
    // and then send a debug message
    let syslog_call_result = unsafe { librust::syscall::debug("Hello, mOS!") };
    kprintln!("[init] --- after system call ---");

    // now let's print current time
    kprintln!("[init] current time: {}", timestamp);
    kprintln!("[init] syslog call result: {}", syslog_call_result);
}
