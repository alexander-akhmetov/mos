use cmos;
use core;
use librust::syscall::SysCallArgument;
use multitasking;
use sys;

pub fn none(_: u64) -> u64 {
    sys::errno::EINTR
}

pub fn sys_time(_: u64) -> u64 {
    /// handles "time" syscall and returns current timestamp
    system_log!("syscall received: 'time'");
    cmos::get_timestamp()
}

pub fn sys_exit(_: u64) -> u64 {
    /// handles "exit" syscall
    system_log!("syscall received: 'exit'");
    sys::errno::SUCCESS
}

pub fn sys_debug(arg_pointer: u64) -> u64 {
    /// handles "debug" syscall and prints passed debug message
    system_log!(
        "debug syscall received with message: '{}'",
        read_str(arg_pointer),
    );
    sys::errno::SUCCESS
}

fn read_str(arg_pointer: u64) -> &'static str {
    /// reads string from arg_pointer which has to be a pointer to librust::syscall::SysCallArgument structure
    let arg_ptr: *const SysCallArgument = arg_pointer as *const SysCallArgument;
    let length: usize = unsafe { (*arg_ptr).length as usize };
    let arg_beginning_ptr = unsafe { (*arg_ptr).address as *const u8 };

    let bytes_buf: &[u8] = unsafe { core::slice::from_raw_parts(arg_beginning_ptr, length) };

    unsafe { core::str::from_utf8_unchecked(bytes_buf) }
}
