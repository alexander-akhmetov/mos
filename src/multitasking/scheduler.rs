use alloc::collections::BTreeMap;
use multitasking::process::{Process, ProcessID};
use spin::RwLock;

pub struct Scheduler {
    tasks: BTreeMap<ProcessID, Process>,
    process_id_counter: u32,
}

pub struct CurrentTask {
    pub id: ProcessID,
}

impl CurrentTask {
    const fn new() -> CurrentTask {
        CurrentTask { id: 0 }
    }
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            tasks: BTreeMap::new(),
            process_id_counter: 0,
        }
    }

    pub fn current_task_id(&self) -> u32 {
        CURRENT_TASK.read().id
    }

    pub fn spawn(&mut self, func_ptr: u64) -> ProcessID {
        self.process_id_counter += 1;
        let process = Process::new(self.process_id_counter, func_ptr);
        let pid = process.id;
        self.tasks.insert(pid, process);
        system_log!("[scheduler] new task created: {}", pid);
        return pid;
    }

    pub fn get_task_mut(&mut self, id: ProcessID) -> Option<&mut Process> {
        return self.tasks.get_mut(&id);
    }

    pub fn get_task(&self, id: ProcessID) -> Option<&Process> {
        return self.tasks.get(&id);
    }

    pub fn next_id(&self) -> Option<ProcessID> {
        let current_id = CURRENT_TASK.read().id;
        for (id, task) in self.tasks.iter() {
            if *id > current_id {
                return Some(*id);
            }
        }

        // if it was the latest task - return the first one
        for (id, task) in self.tasks.iter() {
            return Some(*id);
        }

        return None;
    }
}

lazy_static! {
    pub static ref SCHEDULER: RwLock<Scheduler> = RwLock::new(Scheduler::new());
    pub static ref CURRENT_TASK: RwLock<CurrentTask> = RwLock::new(CurrentTask::new());
}

#[naked]
pub unsafe fn switch_registers(current: &mut Process, next: &Process) {
    asm!("mov $0, cr3" : "=r"(current.registers.cr3) : : "memory" : "intel", "volatile");
    if next.registers.cr3 == 0 {
        panic!("Attempted to switch to a task with an invalid page table!");
    } else if next.registers.cr3 != current.registers.cr3 {
        asm!("mov cr3, $0" : : "r"(next.registers.cr3) : "memory" : "intel", "volatile");
    }

    asm!("pushfq ; pop $0" : "=r"(current.registers.rflags) : : "memory" : "intel", "volatile");
    asm!("push $0 ; popfq" : : "r"(next.registers.rflags) : "memory" : "intel", "volatile");

    asm!("mov $0, rax" : "=r"(current.registers.rax) : : "memory" : "intel", "volatile");
    asm!("mov rax, $0" : : "r"(next.registers.rax) : "memory" : "intel", "volatile");

    asm!("mov $0, rbx" : "=r"(current.registers.rbx) : : "memory" : "intel", "volatile");
    asm!("mov rbx, $0" : : "r"(next.registers.rbx) : "memory" : "intel", "volatile");

    asm!("mov $0, rcx" : "=r"(current.registers.rcx) : : "memory" : "intel", "volatile");
    asm!("mov rcx, $0" : : "r"(next.registers.rcx) : "memory" : "intel", "volatile");

    asm!("mov $0, rdx" : "=r"(current.registers.rdx) : : "memory" : "intel", "volatile");
    asm!("mov rdx, $0" : : "r"(next.registers.rdx) : "memory" : "intel", "volatile");

    asm!("mov $0, rsi" : "=r"(current.registers.rsi) : : "memory" : "intel", "volatile");
    asm!("mov rsi, $0" : : "r"(next.registers.rsi) : "memory" : "intel", "volatile");

    asm!("mov $0, rdi" : "=r"(current.registers.rdi) : : "memory" : "intel", "volatile");
    asm!("mov rdi, $0" : : "r"(next.registers.rdi) : "memory" : "intel", "volatile");

    asm!("mov $0, r12" : "=r"(current.registers.r12) : : "memory" : "intel", "volatile");
    asm!("mov r12, $0" : : "r"(next.registers.r12) : "memory" : "intel", "volatile");

    asm!("mov $0, r13" : "=r"(current.registers.r13) : : "memory" : "intel", "volatile");
    asm!("mov r13, $0" : : "r"(next.registers.r13) : "memory" : "intel", "volatile");

    asm!("mov $0, r14" : "=r"(current.registers.r14) : : "memory" : "intel", "volatile");
    asm!("mov r14, $0" : : "r"(next.registers.r14) : "memory" : "intel", "volatile");

    asm!("mov $0, r15" : "=r"(current.registers.r15) : : "memory" : "intel", "volatile");
    asm!("mov r15, $0" : : "r"(next.registers.r15) : "memory" : "intel", "volatile");

    asm!("mov $0, rbp" : "=r"(current.registers.rbp) : : "memory" : "intel", "volatile");
    asm!("mov rbp, $0" : : "r"(next.registers.rbp) : "memory" : "intel", "volatile");

    asm!("mov $0, rsp" : "=r"(current.registers.rsp) : : "memory" : "intel", "volatile");
    asm!("mov rsp, $0" : : "r"(next.registers.rsp) : "memory" : "intel", "volatile");
}

pub unsafe fn switch() {
    system_log!("[scheduler] switch signal received");

    let current_id = CURRENT_TASK.read().id;

    let scheduler_lock_read = SCHEDULER.read();
    let next_task_id = scheduler_lock_read.next_id();
    if next_task_id.is_none() {
        system_log!("[scheduler] no next task id");
        return;
    }

    let next_task = scheduler_lock_read.get_task(next_task_id.unwrap()).unwrap();

    system_log!(
        "[scheduler] switching tasks from {} to {}",
        current_id,
        next_task.id,
    );

    system_log!("trying to get scheduler...");
    let mut scheduler_lock = SCHEDULER.write();
    system_log!("scheduler locked!");
    let current_task = scheduler_lock.get_task_mut(current_id);
    if current_task.is_none() {
        system_log!("[scheduler] no current task id");
        return;
    }
    switch_registers(current_task.unwrap(), next_task);

    CURRENT_TASK.write().id = next_task.id;
    system_log!("[scheduler] switch completed");
}

pub fn spawn_internal(func: extern "C" fn()) {
    let func_ptr = (func as *const ()) as u64;
    SCHEDULER.write().spawn(func_ptr);
    unsafe { switch() };
}
