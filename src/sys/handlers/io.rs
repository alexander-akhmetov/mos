use alloc::vec::Vec;
use multitasking::scheduler;
use sys;

pub fn sys_read(args: &sys::SyscallArgs) -> u64 {
    // ssize_t read(int fd, void *buf, size_t count);
    // http://man7.org/linux/man-pages/man2/read.2.html
    let current_pid = scheduler::current_process_id();
    system_log_debug!(
        "syscall read: fd={} pid={} count={}",
        args.arg_1,
        current_pid,
        args.arg_2
    );
    if args.arg_1 != sys::constants::STDIN_FD {
        return sys::errno::ENOENT;
    }
    unsafe {
        let process = scheduler::SCHEDULER
            .as_mut()
            .unwrap()
            .get_task_mut(current_pid);
        if process.is_some() {
            let c = process
                .unwrap()
                .file_descriptors
                .get_mut(&args.arg_1)
                .as_mut()
                .unwrap()
                .readc();
            if c.is_some() {
                let mut buf = Vec::from_raw_parts(
                    args.arg_3 as *mut u8,
                    args.arg_2 as usize,
                    args.arg_2 as usize,
                );
                buf[0] = c.unwrap() as u8;
                return sys::errno::SUCCESS;
            } else {
                return sys::errno::EOF;
            }
        } else {
            return sys::errno::ENOENT;
        }
    }
}

pub fn sys_write(args: &sys::SyscallArgs) -> u64 {
    // ssize_t write(int fd, const void *buf, size_t count);
    // http://man7.org/linux/man-pages/man2/write.2.html
    let current_pid = scheduler::current_process_id();
    system_log_debug!(
        "syscall write: fd={} pid={} msg='{}'",
        args.arg_1,
        current_pid,
        sys::handlers::utils::read_str(args.arg_3, args.arg_2) // TODO: arg_2&arg_3 перепутано местами ???
    );
    if args.arg_1 != sys::constants::STDOUT_FD {
        return sys::errno::ENOENT;
    }
    unsafe {
        let process = scheduler::SCHEDULER
            .as_mut()
            .unwrap()
            .get_task_mut(current_pid);
        if process.is_some() {
            let data = Vec::from_raw_parts(
                args.arg_3 as *mut u8,
                args.arg_2 as usize,
                args.arg_2 as usize,
            );
            process
                .unwrap()
                .file_descriptors
                .get_mut(&args.arg_1)
                .as_mut()
                .unwrap()
                .write(data);
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
