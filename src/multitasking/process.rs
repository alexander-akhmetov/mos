use alloc::string::String;
use compiler_builtins::mem::memset;
use core::mem::size_of;
use multitasking::context::ContextRegisters;
use multitasking::scheduler;
use sys;
use x86;

pub type ProcessID = u32;
const PROCESS_STACK_SIZE: usize = 32; // number of u64 elements (512 * 8)
const RFLAGS: u64 = 0b1000000010;

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
    buffer: [u64; PROCESS_STACK_SIZE],
}

impl Stack {
    pub const fn new() -> Stack {
        Stack {
            buffer: [0; PROCESS_STACK_SIZE],
        }
    }

    pub fn top(&self) -> u64 {
        (&(self.buffer[PROCESS_STACK_SIZE - 1]) as *const _) as u64
    }

    pub fn bottom(&self) -> u64 {
        (&(self.buffer[0]) as *const _) as u64
    }
}

pub struct Process {
    pub registers: ContextRegisters,
    pub id: ProcessID,
    pub state: ProcessState,
    stack: Stack,
}

impl Process {
    pub fn new(id: ProcessID, func_ptr: u64) -> Process {
        let stack = &Stack::new();
        unsafe {
            // 0xCD: clean memory
            memset((*stack).bottom() as *mut u8, 0xCD, stack.buffer.len());
        };
        let mut stack_ptr: *mut u64 = ((*stack).top()) as *mut u64;
        let context_registers_size = size_of::<ContextRegisters>();

        unsafe {
            *stack_ptr = 0xDEADBEEF;
            stack_ptr = (stack_ptr as usize - context_registers_size) as *mut u64;

            let context_registers: *mut ContextRegisters = stack_ptr as *mut ContextRegisters;

            (*context_registers).rflags = RFLAGS;
            (*context_registers).rip = func_ptr;
            (*context_registers).complete = (task_finished as *const ()) as u64;
            (*context_registers).rbp = (*stack).top();

            system_log!(
                "Created new process with entry_point_func=0x{:x} id={} context=0x{:x} finish_func=0x{:x}",
                func_ptr,
                id,
                context_registers as u64,
                (*context_registers).complete,
            );

            let pt = Process {
                id: id,
                registers: (*context_registers),
                state: ProcessState::NEW,
                stack: *stack,
            };
            pt.print_stack();
            pt
        }
    }

    pub fn print_stack(&self) {
        if sys::constants::DEBUG {
            system_log!("Process {} stack:", self.id);
            for element in self.stack.buffer.iter() {
                system_log!("   0x{:x}    0x{:x}", element as *const u64 as u64, element);
            }
            system_log!("---");
        }
    }
}

#[naked]
fn task_finished() {
    let current_task = scheduler::current_task_id();
    scheduler::exit_current_process();
    system_log!("finished task {}", current_task);
}
