use librust;
use librust::std::screen::{clear, print, printb};
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

    print("--- mShell ---\n");
    print("~/ _");
    printb(&['\n' as u8; 23]);

    loop {
        stupid_sleep();
    }
}
