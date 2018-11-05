use sys::SysCallArgument;

pub unsafe fn system_call_test() {
    system_log!("--- before system call ---");
    let result = sys_time();
    sys_syslog("<hello mos>");
    system_log!("--- after system call ---");
    system_log!("current time: {}", result);
}

unsafe fn sys_time() -> u32 {
    // get timestamp
    _system_call(13)
}

unsafe fn sys_syslog(msg: &str) -> u32 {
    // for now msg MUST be null-terminated, for example: b"hello\0"
    let ptr = msg.as_ptr();
    let arg = SysCallArgument {
        length: msg.len() as u64,
        address: ptr as u64,
    };
    _system_call_with_args(0, &arg as *const _ as u64)
}

unsafe fn _system_call(number: u32) -> u32 {
    asm!("mov eax, $0; int 0x80;"
         :                          // no output
         : "r"(number)              // input
         : "eax"                    // clobbers
         : "volatile", "intel",     // options
    );
    // now we can read returned code
    let eax: u32 = _read_eax();
    eax
}

unsafe fn _system_call_with_args(number: u32, first_arg: u64) -> u32 {
    asm!("mov rsi, $0; mov eax, $1; int 0x80;"
         :                                          // no output
         : "r"(first_arg), "r"(number)              // input
         : "eax"                                    // clobbers
         : "volatile", "intel",                     // options
    );
    // now we can read returned code
    let eax: u32 = _read_eax();
    eax
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
