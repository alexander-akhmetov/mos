pub mod constants;
pub mod elf;
mod errno;
mod handlers;
pub mod interrupts;
pub mod time;
use spin::Mutex;
use x86;

#[derive(Debug)]
pub struct SyscallArgs {
    pub rax: u64,
    pub arg_1: u64,
    pub arg_2: u64,
    pub arg_3: u64,
    pub arg_4: u64,
    pub arg_5: u64,
    pub arg_6: u64,
}

/// SysCallHandler is a function template which handles system calls from apps
type SysCallHandler = fn(args: &SyscallArgs) -> u64;

// SysCallDispatcher implements a dispatcher to all system calls
pub struct SysCallDispatcher {}

/*
    When an interrupt occurs, if it has 0x80 number,
    SysCallDisapatcher called. It handles information which function can process
    system call with certain number, so it calls the handler.
*/

impl SysCallDispatcher {
    fn new() -> SysCallDispatcher {
        SysCallDispatcher {}
    }

    pub fn process_system_call(&mut self, syscall_args: &SyscallArgs) -> u64 {
        /// Executes a handler for syscall `№ system_call_number`
        system_log_debug!(
            "system call received: '{}', args: {:?}",
            syscall_args.rax,
            syscall_args,
        );
        let handler = self.get_handler(syscall_args.rax);
        let result = handler(syscall_args);
        unsafe { x86::save_rax(result) };
        return result;
    }

    pub fn get_handler(&self, system_call_number: u64) -> SysCallHandler {
        /// returns a handler for system call `№ system_call_number`
        match system_call_number {
            0 => handlers::sys_debug,
            1 => handlers::sys_exit,
            3 => handlers::sys_read,
            4 => handlers::sys_write,
            11 => handlers::sys_execve,
            13 => handlers::sys_time,
            20 => handlers::sys_getpid,
            60 => handlers::sys_exit,
            109 => handlers::sys_uname,
            _ => {
                system_log!("unhandled system call: {}", system_call_number);
                handlers::none
            }
        }
    }
}

lazy_static! {
    pub static ref SYSCALL_DISPATCHER: Mutex<SysCallDispatcher> =
        Mutex::new(SysCallDispatcher::new());
}
