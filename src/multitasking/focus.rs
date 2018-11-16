static mut FOCUSED_PID: u32 = 0;

pub fn focus(pid: u32) {
    system_log!("[focus]: new pid={}", pid);
    unsafe {
        FOCUSED_PID = pid;
    }
}

pub fn get_focused_pid() -> u32 {
    unsafe {
        return FOCUSED_PID;
    }
}
