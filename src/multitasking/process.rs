use crate::fs;
use crate::multitasking::context::Context;
use crate::multitasking::{constants, scheduler};
use crate::sys::time;
use crate::x86;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use compiler_builtins::mem::memset;
use core::mem::size_of;

use super::stdio;

pub type ProcessID = u32;

#[derive(PartialEq)]
pub enum ProcessState {
    WAITING,
    RUNNING,
}

pub struct Process {
    pub id: ProcessID,
    pub state: ProcessState,
    pub name: String,
    pub stack: Vec<u64>,
    pub rsp: u64,
    pub file_descriptors: BTreeMap<u64, Box<dyn fs::FileDescriptor>>,
    pub started_at: u64, // timestamp
    pub brk_addr: u64,
    workdir: String,
}

impl Process {
    pub fn new(id: ProcessID, func_ptr: u64) -> Process {
        /// creates a new process and stack for it
        //
        //  Creating the stack:
        //      To start a new process it must have properly configured stack.
        //      In the stack we should put data for all registers to restore before the start
        //      (see switch_to.asm), entrypoint function and finish function.
        //      Entrypoint function is an address where the new process will jump after restoring
        //      all registers.
        //      Finish function must be placed right after entrypoint_func and will be executed
        //      when task finished. It cleans it: removes the task from the scheduler and should deallocate
        //      process' memory.
        let mut stack: Vec<u64> = vec![0; constants::PROCESS_STACK_SIZE];
        let stack_bottom = stack.as_mut_ptr();
        let stack_top = unsafe { stack_bottom.add(constants::PROCESS_STACK_SIZE - 1) };

        unsafe {
            // 0xCD: clean memory
            memset(stack_bottom as *mut u8, 0xCD, stack.len());

            let mut stack_ptr: *mut u64 = stack_top as *mut u64;

            // for debugging, real code should not ever get this value from stack
            *stack_ptr = 0xDEAD_BEEF;

            // right now pointer looks at the top of the stack:
            //      +----------+  top (rsp must point to this address): highest memory address
            //      |0xDEADBEEF|<------
            //      |          |<------ here will be finish_func        [Context struct ends  ]
            //      |          |<------ here will be entrypoint_func
            //      |          |<------ hereinafter: registers
            //      |..........|                                        [Context struct begins]
            //      +----------+ bottom (lowest memory address)
            //
            //
            // So, we need to subtract size of the Context structure to get pointer to the bottom
            // it will be rsp: Pointer Stack and it's value must be placed into rsp register
            stack_ptr = (stack_ptr as usize - size_of::<Context>()) as *mut u64;

            // now we have address of the Context struct beginning
            let context_registers: *mut Context = stack_ptr as *mut Context;
            // fill it with register's data
            (*context_registers).rflags = constants::RFLAGS;
            (*context_registers).cr3 = x86::read_cr3();
            // and function's addresses
            (*context_registers).start_func = start_task as *const () as u64;
            (*context_registers).entrypoint_func = func_ptr;
            (*context_registers).finish_func = finish_task as *const () as u64;
            // Base Pointer register (points to the top)
            (*context_registers).rbp = stack_top as u64;
            // and finally the stack pointer, rsp
            let rsp = stack_ptr as u64;

            system_log!(
                "[scheduler] created new process id={} entrypoint=0x{:x}",
                id,
                (*context_registers).entrypoint_func,
            );

            let mut pt = Process {
                id,
                state: ProcessState::RUNNING,
                stack,
                rsp,
                file_descriptors: BTreeMap::new(),
                workdir: String::from("/"),
                name: format!("process {}", id),
                started_at: time::timestamp(),
                brk_addr: 0x0031_0000, // todo: change me
            };

            let stdout = stdio::StdOut::new(id);
            let stdin = stdio::StdIn::new(id);
            pt.file_descriptors.insert(0, Box::new(stdin));
            pt.file_descriptors.insert(1, Box::new(stdout));
            pt.print_stack(); // if debug, prints new process' stack
            pt
        }
    }

    pub fn get_workdir(&self) -> String {
        self.workdir.clone()
    }

    pub fn set_workdir(&mut self, workdir: &str) -> bool {
        let wdir = fs::utils::normalize(workdir);
        let exists = fs::vfs::VFS.lock().list_dir(&wdir);

        if exists.is_some() {
            self.workdir = wdir;
            true
        } else {
            false
        }
    }

    pub fn print_stack(&self) {
        /// prints stack of the process, if constants::LOGLEVEL is DEBUG
        system_log_debug!(
            "Process (ctx: 0x{:x} rsp: 0x{:x}) {} stack:",
            self.stack.as_ptr() as u64,
            self.rsp,
            self.id
        );
        for index in 0..self.stack.len() {
            let ptr = unsafe { self.stack.as_ptr().add(index) };
            system_log_debug!("   0x{:x}    0x{:x}", ptr as u64, unsafe { *ptr });
        }
        system_log_debug!("---");
    }
}

fn start_task() {
    let pid = scheduler::current_process_id();
    system_log!("[scheduler] process {}: starting...", pid);
}

fn finish_task() -> ! {
    /// finishes the task and then goes to infinite hlt loop
    /// it must not return anything, because stack is empty and there is no
    /// place to return.
    /// So, this function just executes the switch and waits for the next context switch
    let pid = scheduler::current_process_id();
    system_log!(
        "[scheduler] process {}: completed (rax: {}), finishing...",
        pid,
        unsafe { x86::read_rax() }
    );
    scheduler::exit(pid);
    unsafe {
        scheduler::switch();
        x86::hlt_loop();
    }
}
