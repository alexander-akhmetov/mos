use multitasking::scheduler::{spawn_internal, switch, SCHEDULER};
use sys;

pub fn hello_world() {
    kprintln!("[init] --- before system call ---");

    // let's ask the mOS current time
    let timestamp = unsafe { sys::syscall::sys_time() };
    // and then send a debug message
    let syslog_call_result = unsafe { sys::syscall::sys_debug("Hello, mOS!") };
    kprintln!("[init] --- after system call ---");

    // now let's print current time
    kprintln!("[init] current time: {}", timestamp);
    kprintln!("[init] syslog call result: {}", syslog_call_result);

    system_log!("[init] ###### testing scheduler ######");
    test_scheduler();
}

fn test_scheduler() {
    for _i in 0..2 {
        spawn_internal(foo);
    }
    unsafe { switch() };
}

extern "C" fn foo() {
    for _i in 0..5 {
        system_log!("hello from task {}", SCHEDULER.read().current_task_id());
        unsafe { switch() };
    }
}
