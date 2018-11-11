use spin::Mutex;
pub mod collections;
pub mod elf;
mod errno;
mod handlers;
pub mod interrupts;
pub mod syscall;

/*
    This structure is used by sys::syscall library
    to send arguments with syscalls. Basically, it wraps
    bytes array into this structure and sends a pointer to it.
    Handlers know this structure format and can unwrap it to get data back.
*/

pub struct SyscallArgs {
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
}

struct SysCallArgument {
    length: u64,
    address: u64,
}

type SysCallHandler = fn(first_arg: u64) -> u64;
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
        system_log!("system call received: '{}'", syscall_args.rax);
        let handler = self.get_handler(syscall_args.rax);
        let result = handler(syscall_args.rbx);
        unsafe { syscall::save_rax(result) };
        return result;
    }

    pub fn get_handler(&self, system_call_number: u64) -> SysCallHandler {
        match system_call_number {
            0 => handlers::sys_debug,
            1 => handlers::sys_exit,
            13 => handlers::sys_time,
            60 => handlers::sys_exit,
            1000 => handlers::sys_switch,
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
