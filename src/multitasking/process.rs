use compiler_builtins::mem::memset;
use core::mem::size_of;
use multitasking::context::ContextRegisters;
use multitasking::scheduler::CURRENT_TASK;
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
    buffer: [u64; 4096],
}

impl Stack {
    pub const fn new() -> Stack {
        Stack { buffer: [0; 4096] }
    }

    pub fn top(&self) -> u64 {
        (&(self.buffer[4095]) as *const _) as u64
    }

    pub fn bottom(&self) -> u64 {
        (&(self.buffer[0]) as *const _) as u64
    }
}

pub struct Process {
    pub registers: ContextRegisters,
    pub func_ptr: u64,
    pub id: ProcessID,
    pub state: ProcessState,
}

impl Process {
    pub fn new(id: ProcessID, func_ptr: u64) -> Process {
        let stack = &Stack::new();
        let stack_top: *mut u64 = ((*stack).top()) as *mut u64;
        let mut stack_ptr = (stack_top as u64 - (size_of::<u64>() as u64)) as *mut u64;
        let base_stack_pointer = stack_ptr as u64;

        unsafe {
            // debugging...
            *stack_ptr = 0xDEADBEEF;
            // going 1 u64 down
            stack_ptr = (stack_ptr as usize - size_of::<u64>()) as *mut u64;

            // set "task_finished" function addr to the top of the stack
            *stack_ptr = (task_finished as *const ()) as u64;

            // new addr in the stack for registers
            let context_size = size_of::<ContextRegisters>();
            stack_ptr = (stack_ptr as usize - context_size) as *mut u64;
            //
            let context: *mut ContextRegisters = stack_ptr as *mut ContextRegisters;
            memset(context as *mut u8, 0x00, context_size);

            (*context).rbp = base_stack_pointer;

            (*context).rip = func_ptr;
            (*context).rflags = 0x1002u64;

            Process {
                id: id,
                registers: *context,
                state: ProcessState::NEW,
                func_ptr: func_ptr,
            }
        }
    }
}

#[naked]
#[no_mangle]
extern "C" fn task_finished() -> ! {
    system_log!("finished task {}", CURRENT_TASK.read().id);
    loop {}
}
