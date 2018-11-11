/// Halts the CPU by executing the `hlt` instruction.
#[inline(always)]
pub unsafe fn hlt_loop() -> ! {
    /// runs `hlt` instruction forever
    loop {
        hlt();
    }
}

pub unsafe fn hlt() {
    asm!("hlt" :::: "volatile")
}

pub unsafe fn call(addr: u64) {
    asm!("call $0"
         :                            // no output
         : "r"(addr)                  // input
         :: "volatile", "intel",      // options
    );
}

pub unsafe fn jmp(addr: u64) {
    asm!("jmp $0"
         :                            // no output
         : "r"(addr)                  // input
         :: "volatile", "intel",      // options
    );
}

pub unsafe fn cr3() -> u64 {
    let mut result: u64 = 0;
    #[cfg(not(test))]
    asm!("mov %cr3, $0" : "=r" (result) :);
    result
}

pub unsafe fn read_rflags() -> u64 {
    let result: u64;
    asm!("pushfq
          pop rax"
         : "=r"(result)           // output
         :                        // no input
         :: "volatile", "intel",  // options
    );
    result
}
