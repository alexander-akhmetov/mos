use cmos;
use sys;
use cstr_core::CStr;

pub fn none_handler(_: u64) -> u32 {
    sys::errno::EINTR
}

pub fn sys_time(_: u64) -> u32 {
    kprintln!("system 'time' call received!");
    // let current_time: u32 = 648864000; // 25.07.1990
    let current_time = cmos::get_timestamp();
    current_time
}


pub fn sys_syslog(msg_pointer: u64) -> u32 {
    let str_pointer = msg_pointer as *const i8;
    let string = unsafe {
        CStr::from_ptr(str_pointer).to_str()
    };
    if let Some(string) = string.ok() {
        system_log!("[DEBUG] syscall: {}", string);
    }
    sys::errno::SUCCESS
}
