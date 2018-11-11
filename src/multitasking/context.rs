#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct ContextRegisters {
    pub rflags: u64,
    pub rbx: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rbp: u64,
    pub rip: u64,
}

extern "C" {
    pub fn switch_to(old_ctx: *mut ContextRegisters, new_ctx: *const ContextRegisters);
}
