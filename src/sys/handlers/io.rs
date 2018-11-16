use super::utils;
use alloc::vec::Vec;
use multitasking::scheduler;
use sys;

pub fn sys_read(args: &sys::SyscallArgs) -> u64 {
    // ssize_t read(int fd, void *buf, size_t count);
    // http://man7.org/linux/man-pages/man2/read.2.html
    let current_pid = scheduler::current_task_id();
    system_log!(
        "syscall read: fd={} pid={} count={}",
        args.arg_1,
        current_pid,
        args.arg_3
    );
    return sys::errno::SUCCESS;
}

pub fn sys_write(args: &sys::SyscallArgs) -> u64 {
    // ssize_t write(int fd, const void *buf, size_t count);
    // http://man7.org/linux/man-pages/man2/write.2.html
    let current_pid = scheduler::current_task_id();
    system_log!(
        "syscall write: fd={} pid={} msg='{}'",
        args.arg_1,
        current_pid,
        utils::read_str(args.arg_3, args.arg_2)
    );
    if args.arg_1 != 1 {
        return sys::errno::ENOENT;
    }
    unsafe {
        let process = scheduler::SCHEDULER.as_ref().unwrap().get_task(current_pid);
        if process.is_some() {
            let data = Vec::from_raw_parts(
                args.arg_2 as *mut u8,
                args.arg_3 as usize,
                args.arg_3 as usize,
            );
            process.unwrap().file_descriptors[&args.arg_1].write(data);
        } else {
            system_log_warn!(
                "syscall write error: fd={} pid={}: no such process!",
                args.arg_1,
                current_pid
            );
        }
    };
    return sys::errno::SUCCESS;
}
