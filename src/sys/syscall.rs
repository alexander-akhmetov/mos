use sys::SysCallArgument;

pub unsafe fn sys_exit() -> u64 {
    _system_call(1)
}

pub unsafe fn sys_time() -> u64 {
    // get timestamp
    _system_call(13)
}

pub unsafe fn sys_switch() -> u64 {
    _system_call(1000)
}

pub unsafe fn sys_debug(msg: &str) -> u64 {
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
    let result: u64;
    asm!("mov $0, rax"
         : "=r"(result)           // output
         :                        // no input
         :: "volatile", "intel",  // options
    );
    result
}

pub unsafe fn save_rax(value: u64) {
    asm!("mov rax, $0" :: "m"(value) :: "intel")
}
