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
    buffer: [u8; 4096],
}

impl Stack {
    pub const fn new() -> Stack {
        Stack { buffer: [0; 4096] }
    }

    pub fn top(&self) -> u64 {
        (&(self.buffer[4096 - 16]) as *const _) as u64
    }

    pub fn bottom(&self) -> u64 {
        (&(self.buffer[0]) as *const _) as u64
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct NewProcessStack {
    registers: ContextRegisters,
    entry_point_func: u64,
    finished_func: u64,
    debugging: u64,
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
            let process_stack_size = size_of::<NewProcessStack>();
            stack_ptr = (stack_ptr as usize - process_stack_size) as *mut u64;
            // clean structure
            let process_stack: *mut NewProcessStack = stack_ptr as *mut NewProcessStack;
            memset(process_stack as *mut u8, 0x00, process_stack_size);

            (*process_stack).debugging = 0xDEADBEEF;
            (*process_stack).finished_func = (task_finished as *const ()) as u64;
            (*process_stack).entry_point_func = func_ptr;
            (*process_stack).registers.rflags = 0b1000000010;
            (*process_stack).registers.rip = func_ptr;

            let context_size = size_of::<ContextRegisters>() as u64;
            (*process_stack).registers.rbp = process_stack as u64 + context_size;

            // debugging...
            // *stack_ptr = 0xDEADBEEF;
            // // going 1 u64 down
            // stack_ptr = (stack_ptr as usize - size_of::<u64>()) as *mut u64;

            // // set "task_finished" function addr to the top of the stack
            // *stack_ptr = (task_finished as *const ()) as u64;

            // stack_ptr = (stack_ptr as usize - size_of::<u64>()) as *mut u64;
            // *stack_ptr = func_ptr;

            // // new addr in the stack for registers
            // let context_size = size_of::<ContextRegisters>();
            // stack_ptr = (stack_ptr as usize - context_size) as *mut u64;
            // //
            // let context: *mut ContextRegisters = stack_ptr as *mut ContextRegisters;
            // memset(context as *mut u8, 0x00, context_size);

            // (*context).rbp = stack_ptr as u64;
            // (*context).rflags = 0b1000000010;

            system_log!(
                "Created new process with entry_point_func={}, id={}, context={}",
                func_ptr,
                id,
                &((*process_stack).registers) as *const _ as u64,
            );

            Process {
                id: id,
                registers: (*process_stack).registers,
                state: ProcessState::NEW,
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
