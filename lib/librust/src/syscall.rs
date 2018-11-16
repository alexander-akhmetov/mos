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
    _system_call_2(0, msg.as_ptr() as u64, msg.len() as u64)
}

unsafe fn _system_call(number: u32) -> u64 {
    /// Executes system call with number: number,
    /// reads the response from eax register and returns it
    asm!("int 0x80;"
         :                          // no output
         : "{rax}"(number)          // input
         : "rax", "rbx", "rcx", "rdx", "rsi", "rdi"  // clobbers
         : "volatile", "intel",     // options
    );
    // now we can read returned code
    read_rax()
}

unsafe fn _system_call_1(number: u32, arg_1: u64) -> u64 {
    /// Executes system call with one arg
    /// reads the response from eax register and returns it
    asm!("int 0x80;"
         :                                          // no output
         : "{rax}"(number), "{rbx}"(arg_1)          // input
         : "rax", "rbx", "rcx", "rdx", "rsi", "rdi" // clobbers
         : "volatile", "intel",                     // options
    );
    // now we can read returned code
    read_rax()
}

unsafe fn _system_call_2(number: u32, arg_1: u64, arg_2: u64) -> u64 {
    /// Executes system call with two args
    /// reads the response from eax register and returns it
    asm!("int 0x80;"
         :
         : "{rax}"(number), "{rbx}"(arg_1), "{rcx}"(arg_2)
         : "rax", "rbx", "rcx", "rdx", "rsi", "rdi"
         : "volatile", "intel",
    );
    // now we can read returned code
    read_rax()
}

unsafe fn _system_call_3(number: u32, arg_1: u64, arg_2: u64, arg_3: u64) -> u64 {
    /// Executes system call with three args
    /// reads the response from eax register and returns it
    asm!("int 0x80;"
         :
         : "{rax}"(number), "{rbx}"(arg_1), "{rcx}"(arg_2), "{rdx}"(arg_3)
         : "rax", "rbx", "rcx", "rdx", "rsi", "rdi"
         : "volatile", "intel",
    );
    // now we can read returned code
    read_rax()
}

unsafe fn _system_call_4(number: u32, arg_1: u64, arg_2: u64, arg_3: u64, arg_4: u64) -> u64 {
    /// Executes system call with four args
    /// reads the response from eax register and returns it
    asm!("int 0x80;"
         :
         : "{rax}"(number), "{rbx}"(arg_1), "{rcx}"(arg_2), "{rdx}"(arg_3), "{rsi}"(arg_4)
         : "rax", "rbx", "rcx", "rdx", "rsi", "rdi"
         : "volatile", "intel",
    );
    // now we can read returned code
    read_rax()
}

unsafe fn _system_call_5(
    number: u32,
    arg_1: u64,
    arg_2: u64,
    arg_3: u64,
    arg_4: u64,
    arg_5: u64,
) -> u64 {
    /// Executes system call with four args
    /// reads the response from eax register and returns it
    asm!("int 0x80;"
         :
         : "{rax}"(number), "{rbx}"(arg_1), "{rcx}"(arg_2), "{rdx}"(arg_3), "{rsi}"(arg_4), "{rdi}"(arg_5)
         : "rax", "rbx", "rcx", "rdx", "rsi", "rdi"
         : "volatile", "intel",
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
