use spin::Mutex;
pub mod collections;
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
const SYSCALL_COUNT: usize = 14;
pub struct SysCallDispatcher {
    handlers: [SysCallHandler; SYSCALL_COUNT],
}

/*
    When an interrupt occurs, if it has 0x80 number,
    SysCallDisapatcher called. It handles information which function can process
    system call with certain number, so it calls the handler.
*/

impl SysCallDispatcher {
    fn new() -> SysCallDispatcher {
        SysCallDispatcher {
            handlers: [
                // todo: ma
                handlers::sys_debug, // must be 102, but for now let it be here
                handlers::none,
                handlers::none,
                handlers::none,
                handlers::none,
                handlers::none,
                handlers::none,
                handlers::none,
                handlers::none,
                handlers::none,
                handlers::none,
                handlers::none,
                handlers::none,
                handlers::sys_time,
            ],
        }
    }

    pub fn process_system_call(&mut self, system_call_number: u64, first_arg: u64) -> u32 {
        /// Executes a handler for syscall `№ system_call_number`
        system_log!("system call received: '{}'", system_call_number);
        let handler = self.handlers[system_call_number as usize];
        if handler == handlers::none {
            // if this handler is unknown for the dispatcher, let's write a message about it
            system_log!("unhandled system call: {}", system_call_number);
        }
        handler(first_arg)
    }
}

lazy_static! {
    pub static ref SYSCALL_DISPATCHER: Mutex<SysCallDispatcher> =
        Mutex::new(SysCallDispatcher::new());
}
