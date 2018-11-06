/// Halts the CPU by executing the `hlt` instruction.
#[inline(always)]
pub unsafe fn hlt_loop() -> ! {
    /// runs `hlt` instruction forever
    loop {
        asm!("hlt" :::: "volatile");
    }
}
