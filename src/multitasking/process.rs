use core::mem;
use multitasking::context::ContextRegisters;
use x86;

pub type ProcessID = u32;

pub struct Process {
    pub registers: ContextRegisters,
    pub id: ProcessID,
}

impl Process {
    pub fn new(id: ProcessID, func_ptr: u64) -> Process {
        let mut registers = ContextRegisters::new();
        let process_stack = vec![0; 65536].into_boxed_slice();
        let offset = process_stack.len() - mem::size_of::<usize>();
        registers.cr3 = unsafe { x86::cr3() };
        registers.rsp = func_ptr;
        Process {
            id: id,
            registers: registers,
        }
    }
}
