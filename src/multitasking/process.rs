use alloc::string::String;
use alloc::vec::Vec;
use compiler_builtins::mem::memset;
use core::mem::size_of;
use multitasking::context::ContextRegisters;
use multitasking::scheduler;
use sys;
use x86;

pub type ProcessID = u32;
const PROCESS_STACK_SIZE: usize = 1024; // number of u64 elements (512 * 8)
const RFLAGS: u64 = 0b1000000010;

#[derive(PartialEq)]
#[repr(C)]
pub enum ProcessState {
    NEW,
    RUNNING,
}

pub struct Process {
    // pub registers: ContextRegisters,
    pub id: ProcessID,
    pub state: ProcessState,
    pub stack: Vec<u64>,
    pub rsp: u64,
}

impl Process {
    pub fn new(id: ProcessID, func_ptr: u64) -> Process {
        let mut stack: Vec<u64> = vec![0; PROCESS_STACK_SIZE];
        let stack_bottom = stack.as_mut_ptr();
        let stack_top = unsafe { stack_bottom.add(PROCESS_STACK_SIZE - 1) };

        unsafe {
            // 0xCD: clean memory
            memset(stack_bottom as *mut u8, 0xCD, stack.len());
        };
        let mut stack_ptr: *mut u64 = stack_top as *mut u64;
        let context_registers_size = size_of::<ContextRegisters>();

        unsafe {
            *stack_ptr = 0xDEADBEEF;
            stack_ptr = (stack_ptr as usize - context_registers_size) as *mut u64;

            let context_registers: *mut ContextRegisters = stack_ptr as *mut ContextRegisters;

            (*context_registers).rflags = RFLAGS;
            (*context_registers).rip = func_ptr;
            (*context_registers).complete = (task_finished as *const ()) as u64;
            (*context_registers).rbp = stack_top as u64;
            // let rsp = stack_top.sub(3) as u64;
            let rsp = stack_ptr as u64;

            system_log!(
                "Created new process with entry_point_func=0x{:x} id={} context=0x{:x} finish_func=0x{:x} rsp=0x{:x}",
                func_ptr,
                id,
                context_registers as u64,
                (*context_registers).complete,
                rsp,
            );

            let pt = Process {
                id: id,
                // registers: (*context_registers),
                state: ProcessState::NEW,
                stack: stack,
                rsp: rsp,
            };
            pt.print_stack();
            pt
        }
    }

    pub fn print_stack(&self) {
        if sys::constants::DEBUG {
            system_log!(
                "Process (ctx: 0x{:x} rsp: 0x{:x}) {} stack:",
                self.stack.as_ptr() as u64,
                self.rsp,
                self.id
            );
            for index in 0..self.stack.len() {
                let ptr = unsafe { self.stack.as_ptr().add(index) };
                system_log!("   0x{:x}    0x{:x}", ptr as u64, unsafe { *ptr });
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
