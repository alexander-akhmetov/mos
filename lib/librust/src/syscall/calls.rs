pub unsafe fn _system_call(number: u32) -> u64 {
    /// Executes system call with number: number,
    /// reads the response from rax register and returns it
    asm!("int 0x80;"
         :                          // no output
         : "{rax}"(number)          // input
         : "rax", "rbx", "rcx", "rdx", "rsi", "rdi"  // clobbers
         : "volatile", "intel",     // options
    );
    // now we can read returned code
    read_rax()
}

pub unsafe fn _system_call_1(number: u32, arg_1: u64) -> u64 {
    /// Executes system call with one arg
    /// reads the response from rax register and returns it
    asm!("int 0x80;"
         :                                          // no output
         : "{rax}"(number), "{rbx}"(arg_1)          // input
         : "rax", "rbx", "rcx", "rdx", "rsi", "rdi" // clobbers
         : "volatile", "intel",                     // options
    );
    // now we can read returned code
    read_rax()
}

pub unsafe fn _system_call_2(number: u32, arg_1: u64, arg_2: u64) -> u64 {
    /// Executes system call with two args
    /// reads the response from rax register and returns it
    asm!("int 0x80;"
         :
         : "{rax}"(number), "{rbx}"(arg_1), "{rcx}"(arg_2)
         : "rax", "rbx", "rcx", "rdx", "rsi", "rdi"
         : "volatile", "intel",
    );
    // now we can read returned code
    read_rax()
}

pub unsafe fn _system_call_3(number: u32, arg_1: u64, arg_2: u64, arg_3: u64) -> u64 {
    /// Executes system call with three args
    /// reads the response from rax register and returns it
    asm!("int 0x80;"
         :
         : "{rax}"(number), "{rbx}"(arg_1), "{rcx}"(arg_2), "{rdx}"(arg_3)
         : "rax", "rbx", "rcx", "rdx", "rsi", "rdi"
         : "volatile", "intel",
    );
    // now we can read returned code
    read_rax()
}

pub unsafe fn _system_call_4(number: u32, arg_1: u64, arg_2: u64, arg_3: u64, arg_4: u64) -> u64 {
    /// Executes system call with four args
    /// reads the response from rax register and returns it
    asm!("int 0x80;"
         :
         : "{rax}"(number), "{rbx}"(arg_1), "{rcx}"(arg_2), "{rdx}"(arg_3), "{rsi}"(arg_4)
         : "rax", "rbx", "rcx", "rdx", "rsi", "rdi"
         : "volatile", "intel",
    );
    // now we can read returned code
    read_rax()
}

pub unsafe fn _system_call_5(
    number: u32,
    arg_1: u64,
    arg_2: u64,
    arg_3: u64,
    arg_4: u64,
    arg_5: u64,
) -> u64 {
    /// Executes system call with four args
    /// reads the response from rax register and returns it
    asm!("int 0x80;"
         :
         : "{rax}"(number), "{rbx}"(arg_1), "{rcx}"(arg_2), "{rdx}"(arg_3), "{rsi}"(arg_4), "{rdi}"(arg_5)
         : "rax", "rbx", "rcx", "rdx", "rsi", "rdi"
         : "volatile", "intel",
    );
    // now we can read returned code
    read_rax()
}

pub unsafe fn read_rax() -> u64 {
    /// returns RAX register's value
    let result: u64;
    asm!("mov $0, rax"
         : "=r"(result)           // output
         :                        // no input
         :: "volatile", "intel",  // options
    );
    result
}
