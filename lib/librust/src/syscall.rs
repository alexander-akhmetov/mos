pub struct SysCallArgument {
    /*
        This structure is used by librust::syscall library
        to send arguments with syscalls. Basically, it wraps
        bytes array into this structure and sends a pointer to it.
        Handlers know this structure format and can unwrap it to get data back.
    */
    pub length: u64,
    pub address: u64,
}

pub unsafe fn exit() -> u64 {
    /// sends system call "exit"
    _system_call(1)
}

pub unsafe fn time() -> u64 {
    /// sends system call "time" and returns current timestamp
    _system_call(13)
}

pub unsafe fn getpid() -> u64 {
    /// sends system call "getpid" and returns pid of the current process
    _system_call(20)
}

pub unsafe fn debug(msg: &str) -> u64 {
    /// sends system call "debug" with msg string
    // for now msg MUST be null-terminated, for example: b"hello\0"
    let ptr = msg.as_ptr();
    let arg = SysCallArgument {
        length: msg.len() as u64,
        address: ptr as u64,
    };
    _system_call_with_args(0, &arg as *const _ as u64)
}

unsafe fn _system_call(number: u32) -> u64 {
    /// Executes system call with number: number,
    /// reads the response from eax register and returns it
    asm!("int 0x80;"
         :                          // no output
         : "{rax}"(number)          // input
         : "memory"                 // clobbers
         : "volatile", "intel",     // options
    );
    // now we can read returned code
    read_rax()
}

unsafe fn _system_call_with_args(number: u32, first_arg: u64) -> u64 {
    /// Executes system call with number: number, and first_arg (pointer as u64)
    /// reads the response from eax register and returns it
    asm!("int 0x80;"
         :                                          // no output
         : "{rax}"(number), "{rbx}"(first_arg)      // input
         : "memory"                                 // clobbers
         : "volatile", "intel",                     // options
    );
    // now we can read returned code
    read_rax()
}

unsafe fn read_rax() -> u64 {
    /// returns RAX register's value
    let result: u64;
    asm!("mov $0, rax"
         : "=r"(result)           // output
         :                        // no input
         :: "volatile", "intel",  // options
    );
    result
}
