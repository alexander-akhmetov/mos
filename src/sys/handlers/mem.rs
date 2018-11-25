use memory::paging::translate;
use multitasking::scheduler;
use sys;

pub fn sys_mmap(args: &sys::SyscallArgs) -> u64 {
    let pid = scheduler::current_process_id();

    system_log!(
        "syscall received: 'mmap' with size={} pid={}",
        args.arg_1,
        pid
    );

    if args.arg_1 % 4096 != 0 {
        // length must be page aligned for now
        system_log!("mmap error: length is not page aligned");
        return sys::errno::EINVAL;
    }

    unsafe {
        let process = scheduler::SCHEDULER
            .as_mut()
            .unwrap()
            .get_task_mut(pid)
            .unwrap();
        let old_brk_addr = process.brk_addr;
        for i in 0..args.arg_1 {
            translate((old_brk_addr + i) as usize);
        }
        process.brk_addr += args.arg_1;
        return old_brk_addr as u64;
    }
}

pub fn sys_munmap(args: &sys::SyscallArgs) -> u64 {
    system_log!(
        "syscall received: 'munmap' with addr=0x{:x} size={}",
        args.arg_1,
        args.arg_2
    );
    return sys::errno::ENOSYS;
}
