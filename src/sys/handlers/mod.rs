use alloc::string::String;
use cmos;
use constants;
use multitasking::scheduler;
use sys;

mod fs;
mod io;
mod utils;

pub use self::fs::*;
pub use self::io::{sys_read, sys_write};
use self::utils::read_str;

pub fn none(args: &sys::SyscallArgs) -> u64 {
    sys::errno::ENOSYS
}

pub fn sys_time(args: &sys::SyscallArgs) -> u64 {
    /// handles "time" syscall and returns current timestamp
    system_log!("syscall received: 'time'");
    cmos::get_timestamp()
}

pub fn sys_getpid(args: &sys::SyscallArgs) -> u64 {
    let pid = scheduler::current_task_id();
    system_log!("syscall received: 'getpid', pid: {}", pid);
    return pid as u64;
}

pub fn sys_exit(args: &sys::SyscallArgs) -> u64 {
    /// handles "exit" syscall
    system_log!("syscall received: 'exit'");
    scheduler::exit_current();
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

pub fn sys_execve(args: &sys::SyscallArgs) -> u64 {
    sys::errno::ENOSYS
}

pub fn sys_waitpid(args: &sys::SyscallArgs) -> u64 {
    sys::errno::ENOSYS
}

pub struct UtsName {
    pub sysname: String,
    pub version: String,
}

pub fn sys_uname(args: &sys::SyscallArgs) -> u64 {
    // get name and information about current kernel
    // http://man7.org/linux/man-pages/man2/uname.2.html
    unsafe {
        let info_struct = args.arg_1 as *mut UtsName;
        (*info_struct).sysname = String::from(constants::KERNEL_SYSNAME);
        (*info_struct).version = String::from(constants::KERNEL_VERSION);
    };

    return sys::errno::SUCCESS;
}
