use spin::Mutex;
mod errno;
mod handlers;
pub mod lib;

struct SysCallArgument {
    length: u64,
    address: u64,
}

const SYSCALL_COUNT: usize = 14;
type SysCallHandler = fn(first_arg: u64) -> u32;

pub struct SysCallDispatcher {
    handlers: [SysCallHandler; SYSCALL_COUNT],
}

impl SysCallDispatcher {
    fn new() -> SysCallDispatcher {
        SysCallDispatcher {
            handlers: [
                // todo: map
                handlers::sys_syslog, // must be 102, but for now let it be here
                handlers::none_handler,
                handlers::none_handler,
                handlers::none_handler,
                handlers::none_handler,
                handlers::none_handler,
                handlers::none_handler,
                handlers::none_handler,
                handlers::none_handler,
                handlers::none_handler,
                handlers::none_handler,
                handlers::none_handler,
                handlers::none_handler,
                handlers::sys_time,
            ],
        }
    }

    pub fn process_system_call(&mut self, system_call_number: u64, first_arg: u64) -> u32 {
        kprintln!("system call received: '{}'", system_call_number);
        let handler = self.get_handler(system_call_number);
        if handler == handlers::none_handler {
            kprintln!("unhandled system call: {}", system_call_number);
        }
        handler(first_arg)
    }

    pub fn get_handler(&mut self, system_call_number: u64) -> SysCallHandler {
        self.handlers[system_call_number as usize]
    }
}

lazy_static! {
    pub static ref SYSCALL_DISPATCHER: Mutex<SysCallDispatcher> =
        Mutex::new(SysCallDispatcher::new());
}
