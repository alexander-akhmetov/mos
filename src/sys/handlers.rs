use cmos;
use core;
use multitasking;
use sys;

pub fn none(args: &sys::SyscallArgs) -> u64 {
    sys::errno::EINTR
}

pub fn sys_time(args: &sys::SyscallArgs) -> u64 {
    /// handles "time" syscall and returns current timestamp
    system_log!("syscall received: 'time'");
    cmos::get_timestamp()
}

pub fn sys_getpid(args: &sys::SyscallArgs) -> u64 {
    let pid = multitasking::scheduler::current_task_id();
    system_log!("syscall received: 'getpid', pid: {}", pid);
    return pid as u64;
}

pub fn sys_exit(args: &sys::SyscallArgs) -> u64 {
    /// handles "exit" syscall
    system_log!("syscall received: 'exit'");
    sys::errno::SUCCESS
}

pub fn sys_debug(args: &sys::SyscallArgs) -> u64 {
    /// handles "debug" syscall and prints passed debug message
    system_log!(
        "debug syscall received with message: '{}'",
        read_str(args.arg_1, args.arg_2),
    );
    sys::errno::SUCCESS
}

pub fn read(arg_pointer: u64) -> u64 {
    0
}

pub fn write(arg_pointer: u64) -> u64 {
    0
}

fn read_str(arg_ptr: u64, length: u64) -> &'static str {
    let bytes_buf: &[u8] =
        unsafe { core::slice::from_raw_parts(arg_ptr as *const u8, length as usize) };
    unsafe { core::str::from_utf8_unchecked(bytes_buf) }
}
