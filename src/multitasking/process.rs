use compiler_builtins::mem::memset;
use core::mem::size_of;
use multitasking::context::ContextRegisters;
use multitasking::scheduler;
use x86;

pub type ProcessID = u32;

#[derive(PartialEq)]
#[repr(C)]
pub enum ProcessState {
    NEW,
    RUNNING,
}

#[derive(Copy, Clone)]
#[repr(align(64))]
#[repr(C)]
pub struct Stack {
    buffer: [u8; 4096],
}

impl Stack {
    pub const fn new() -> Stack {
        Stack { buffer: [0; 4096] }
    }

    pub fn top(&self) -> u64 {
        (&(self.buffer[4096 - 8]) as *const _) as u64
    }

    pub fn bottom(&self) -> u64 {
        (&(self.buffer[0]) as *const _) as u64
    }
}

pub struct Process {
    pub registers: ContextRegisters,
    pub id: ProcessID,
    pub state: ProcessState,
}

impl Process {
    pub fn new(id: ProcessID, func_ptr: u64) -> Process {
        let stack = &Stack::new();
        unsafe {
            // 0xCD: clean memory
            memset((*stack).bottom() as *mut u8, 0xCD, stack.buffer.len());
        };
        let stack_top: *mut u64 = ((*stack).top()) as *mut u64;
        let mut stack_ptr = (stack_top as u64 - (size_of::<u64>() as u64)) as *mut u64;
        let base_stack_pointer = stack_ptr as u64;

        unsafe {
            let context_registers_size = size_of::<ContextRegisters>();
            stack_ptr = (stack_ptr as usize - context_registers_size) as *mut u64;
            // clean structure
            let context_registers: *mut ContextRegisters = stack_ptr as *mut ContextRegisters;
            memset(context_registers as *mut u8, 0x00, context_registers_size);

            (*context_registers).rflags = 0b1000000010;
            (*context_registers).rip = func_ptr;
            (*context_registers).complete = (task_finished as *const ()) as u64;

            (*context_registers).rbp = context_registers as u64 + context_registers_size as u64;

            system_log!(
                "Created new process with entry_point_func={}, id={}, context={}",
                func_ptr,
                id,
                &(*context_registers) as *const _ as u64,
            );

            Process {
                id: id,
                registers: (*context_registers),
                state: ProcessState::NEW,
            }
        }
    }
}

#[naked]
fn task_finished() {
    let current_task = scheduler::current_task_id();
    scheduler::exit_current_process();
    system_log!("finished task {}", current_task);
}
