use alloc::string::String;
use alloc::vec::Vec;

use super::utils;
use fs::vfs::VFS;
use fs::FileSystem;
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

pub fn sys_readdir(args: &sys::SyscallArgs) -> u64 {
    // works only for the current dir
    unsafe {
        let workdir = scheduler::SCHEDULER
            .as_ref()
            .unwrap()
            .get_active_process()
            .unwrap()
            .get_workdir();

        let content = VFS.lock().list_dir(workdir.as_str());
        system_log!("LS: {:?}", content);

        if content.is_none() {
            return sys::errno::ENOENT;
        };
        let cnt = content.unwrap();
        if cnt.len() <= args.arg_2 as usize {
            return sys::errno::EOF;
        };
        write_to_buf(
            args.arg_1 as *mut u8,
            args.arg_3 as usize,
            &(cnt[args.arg_2 as usize].name()),
        );
    }
    return sys::errno::SUCCESS;
}

pub fn sys_chdir(args: &sys::SyscallArgs) -> u64 {
    unsafe {
        let new_workdir = utils::read_str(args.arg_1, args.arg_2);
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
        write_to_buf(args.arg_1 as *mut u8, args.arg_2 as usize, &workdir);
    }

    return sys::errno::SUCCESS;
}

fn write_to_buf(ptr: *mut u8, length: usize, s: &str) {
    unsafe {
        let mut data = Vec::from_raw_parts(ptr, length, length);
        for (index, element) in s.as_bytes().iter().enumerate() {
            data[index] = *element;
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_sys_readdir() {
//         let mut buf: [u8; 10] = ['\0' as u8; 10];
//         let length = 10;
//         let args = sys::SyscallArgs {
//             rax: 0,
//             arg_1: buf.as_mut_ptr() as u64,
//             arg_2: length,
//             arg_3: 0,
//             arg_4: 0,
//             arg_5: 0,
//             arg_6: 0,
//         };
//         assert_eq!(dt.timestamp(), 0)
//     }

// }
