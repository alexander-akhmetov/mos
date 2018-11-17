use alloc::string::String;
use alloc::vec::Vec;

use super::utils;
use multitasking::scheduler;
use sys;

pub fn sys_rename(args: &sys::SyscallArgs) -> u64 {
    return sys::errno::ENOSYS;
}

pub fn sys_mkdir(args: &sys::SyscallArgs) -> u64 {
    return sys::errno::ENOSYS;
}

pub fn sys_rmdir(args: &sys::SyscallArgs) -> u64 {
    return sys::errno::ENOSYS;
}

pub fn sys_chdir(args: &sys::SyscallArgs) -> u64 {
    unsafe {
        let new_workdir = utils::read_str(args.arg_1, args.arg_2); // TODO: arg_2&arg_3 перепутано местами ???
        let workdir = scheduler::SCHEDULER
            .as_mut()
            .unwrap()
            .get_active_process_mut()
            .unwrap()
            .set_workdir(&new_workdir);
    }
    return sys::errno::SUCCESS;
}

pub fn sys_getcwd(args: &sys::SyscallArgs) -> u64 {
    unsafe {
        let workdir = scheduler::SCHEDULER
            .as_ref()
            .unwrap()
            .get_active_process()
            .unwrap()
            .get_workdir();
        let mut data = Vec::from_raw_parts(
            args.arg_1 as *mut u8,
            args.arg_2 as usize,
            args.arg_2 as usize,
        );
        for (index, element) in workdir.into_bytes().iter().enumerate() {
            data[index] = *element;
        }
    }

    return sys::errno::SUCCESS;
}
