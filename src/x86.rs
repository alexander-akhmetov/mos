/// Halts the CPU by executing the `hlt` instruction.
#[inline(always)]
pub unsafe fn hlt_loop() -> ! {
    /// runs `hlt` instruction forever
    loop {
        asm!("hlt" :::: "volatile");
    }
}

pub unsafe fn jmp(addr: u64) {
    asm!("call $0"
         :                            // no output
         : "r"(addr)                  // input
         :: "volatile", "intel",     // options
    );
}
