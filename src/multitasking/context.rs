#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct ContextRegisters {
    /// ContextRegisters stores all information about paused thread
    /// in the memory
    /// To restore the thread it must be putted to the thread's stack before switching
    pub rflags: u64,
    pub rbx: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rbp: u64,
    pub rip: u64,
    pub complete: u64,
}
