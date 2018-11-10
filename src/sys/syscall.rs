use sys::SysCallArgument;

pub unsafe fn sys_time() -> u32 {
    // get timestamp
    _system_call(13)
}

pub unsafe fn sys_debug(msg: &str) -> u32 {
    // for now msg MUST be null-terminated, for example: b"hello\0"
    let ptr = msg.as_ptr();
    let arg = SysCallArgument {
        length: msg.len() as u64,
        address: ptr as u64,
    };
    _system_call_with_args(0, &arg as *const _ as u64)
}

unsafe fn _system_call(number: u32) -> u32 {
    /// Executes system call with number: number,
    /// reads the response from eax register and returns it
    asm!("int 0x80;"
         :                          // no output
         : "{eax}"(number)          // input
         : "memory"                 // clobbers
         : "volatile", "intel",     // options
    );
    // now we can read returned code
    _read_eax()
}

unsafe fn _system_call_with_args(number: u32, first_arg: u64) -> u32 {
    /// Executes system call with number: number, and first_arg (pointer as u64)
    /// reads the response from eax register and returns it
    asm!("int 0x80;"
         :                                          // no output
         : "{rsi}"(first_arg), "{eax}"(number)      // input
         : "memory"                                 // clobbers
         : "volatile", "intel",                     // options
    );
    // now we can read returned code
    _read_eax()
}

unsafe fn _read_eax() -> u32 {
    let result: u32;
    asm!("mov $0, eax"
         : "=r"(result)           // output
         :                        // no input
         :: "volatile", "intel",  // options
    );
    result
}
