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

struct SysCallArgument {
    length: u64,
    address: u64,
}

type SysCallHandler = fn(first_arg: u64) -> u32;
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

    pub fn process_system_call(&mut self, system_call_number: u64, first_arg: u64) -> u32 {
        /// Executes a handler for syscall `№ system_call_number`
        system_log!("system call received: '{}'", system_call_number);
        let handler = self.get_handler(system_call_number);
        return handler(first_arg);
    }

    pub fn get_handler(&self, system_call_number: u64) -> SysCallHandler {
        match system_call_number {
            0 => handlers::sys_debug,
            13 => handlers::sys_time,
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
