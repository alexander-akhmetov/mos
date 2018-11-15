// #[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Context {
    /// Context stores all information about paused thread
    /// in the memory
    /// To restore the thread it must be putted to the thread's stack before switching
    pub rbp: u64,
    pub rflags: u64,
    rbx: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    pub entrypoint_func: u64,
    pub finish_func: u64,
}
