use cmos;
use sys;
use core::str::{self, Utf8Error};

pub fn none_handler(_: u64) -> u32 {
    sys::errno::EINTR
}

pub fn sys_time(_: u64) -> u32 {
    system_log!("syscall received: 'time'");
    cmos::get_timestamp()
}

/*
This mechanism looks ugly, but I don't have a
memory allocator yet, so this is a way to pass arguments to
system calls: wrap them into sys::SysCallArgument
and unwrap here as a byte array if needed
*/

static mut ARG_BYTES_BUF: [u8; 255] = [b'\0'; 255];

pub fn sys_syslog(arg_pointer: u64) -> u32 {
    system_log!("debug syscall received with message: '{}'", read_str(arg_pointer));
    clean_buffer();
    sys::errno::SUCCESS
}

fn read_str(arg_pointer: u64) -> &'static str {
    let arg_ptr: *const sys::SysCallArgument = arg_pointer as *const sys::SysCallArgument;

    let length = unsafe { (*arg_ptr).length };
    for i in 0..length {
        unsafe {
            ARG_BYTES_BUF[i as usize] = *((*arg_ptr).address as *const u8).offset(i as isize) as u8;
        }
    }

    unsafe {
        str::from_utf8(&ARG_BYTES_BUF[0..((*arg_ptr).length) as usize]).unwrap()
    }
}

fn clean_buffer() {
    // not necessary, but clean this buffer to remove all old data
    // this will be removed when mOS will have memory allocator
    for i in 0..255 {
        unsafe {
            ARG_BYTES_BUF[i as usize] = b'\0';
        }
    }
}
