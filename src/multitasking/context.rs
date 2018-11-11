#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct ContextRegisters {
    pub rflags: usize,
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbp: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    // base pointer
    pub rbx: u64,
    // instruction pointer
    pub rip: u64,
}

impl ContextRegisters {
    pub fn new() -> ContextRegisters {
        ContextRegisters {
            rflags: 0,
            rax: 0,
            rcx: 0,
            rdx: 0,
            rbp: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rbx: 0,
            rip: 0,
        }
    }
}

extern "C" {
    pub fn switch_to(old_ctx: *const ContextRegisters, new_ctx: *const ContextRegisters);
    fn isr_return();
}
