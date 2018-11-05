use sys;


pub unsafe fn hello_world() {
    kprintln!("\n--- before system call ---");

    // let's ask the mOS current time
    let timestamp = sys::lib::sys_time();
    // and then send a debug message
    let syslog_call_result = sys::lib::sys_syslog("Hello, mOS!");
    kprintln!("--- after system call ---\n");

    // now let's print current time
    kprintln!("current time: {}", timestamp);
    kprintln!("syslog call result: {}", syslog_call_result);
}
