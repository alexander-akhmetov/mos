#[inline(always)]
pub unsafe fn hlt() {
    /// calls "hlt" instruction
    asm!("hlt" :::: "volatile")
}
