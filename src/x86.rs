#[inline(always)]
pub unsafe fn hlt_loop() -> ! {
    /// Halts the CPU by executing the `hlt` instruction.
    /// runs "hlt" instruction forever
    loop {
        hlt();
    }
}

#[inline(always)]
pub unsafe fn hlt() {
    /// calls "hlt" instruction
    asm!("hlt" :::: "volatile")
}

#[inline(always)]
pub unsafe fn call(addr: u64) {
    /// calls "call" instruction with a given addr
    asm!("call $0"
         :                            // no output
         : "r"(addr)                  // input
         :: "volatile", "intel",      // options
    );
}

#[inline(always)]
pub unsafe fn jmp(addr: u64) {
    /// calls jmp instruction to a given addr
    asm!("jmp $0"
         :                            // no output
         : "r"(addr)                  // input
         :: "volatile", "intel",      // options
    );
}

pub unsafe fn cr3() -> u64 {
    /// returns CR3 value
    let result: u64;
    asm!("mov %cr3, $0" : "=r" (result) :);
    result
}

pub unsafe fn read_rflags() -> u64 {
    /// returns RFLAGS calue
    let result: u64;
    asm!("pushfq
          pop rax"
         : "=r"(result)           // output
         :                        // no input
         :: "volatile", "intel",  // options
    );
    result
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

pub unsafe fn read_cr3() -> u64 {
    /// returns CR3 register's value
    let result: u64;
    asm!("mov $0, cr3"
         : "=r"(result)           // output
         :                        // no input
         :: "volatile", "intel",  // options
    );
    result
}

pub unsafe fn save_rax(value: u64) {
    /// saves value to the RAX register
    asm!("mov rax, $0" :: "m"(value) :: "intel")
}

pub unsafe fn read_cr2() -> u64 {
    /// returns CR2 register's value
    let result: u64;
    asm!("mov $0, cr2"
         : "=r"(result)           // output
         :                        // no input
         :: "volatile", "intel",  // options
    );
    result
}
