use librust;
use multitasking::focus::focus;
use multitasking::scheduler;
use sys::time::stupid_sleep;

pub fn start() {
    scheduler::spawn(_start);
}

fn _start() {
    let pid = unsafe { librust::syscall::getpid() };
    system_log!("[msh] started with pid {}", pid);
    focus(pid as u32);
    loop {
        stupid_sleep();
    }
}
