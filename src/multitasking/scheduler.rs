use alloc::collections::BTreeMap;
use multitasking::context::{switch_to, ContextRegisters};
use multitasking::process::{Process, ProcessID};
use spin::RwLock;
use x86;

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

extern "C" fn init() -> u64 {
    loop {
        unsafe { switch() };
    }
}

impl Scheduler {
    pub fn new() -> Scheduler {
        let mut sc = Scheduler {
            tasks: BTreeMap::new(),
            process_id_counter: 0,
        };

        let pid = sc.spawn(init as *const () as u64);
        CURRENT_TASK.write().id = pid;
        sc
    }

    pub fn current_task_id(&self) -> u32 {
        CURRENT_TASK.read().id
    }

    pub fn spawn(&mut self, func_ptr: u64) -> ProcessID {
        self.process_id_counter += 1;
        let process = Process::new(self.process_id_counter, func_ptr);
        let pid = process.id;
        self.tasks.insert(pid, process);
        system_log!(
            "[scheduler] new task created: {}, rsp 0x{:x}",
            pid,
            func_ptr
        );
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

// #[naked]
// pub unsafe fn switch_registers(current: &mut Process, next: ContextRegisters) {
//     // asm!("mov $0, cr3" : "=r"(current.registers.cr3) : : "memory" : "intel", "volatile");
//     // if next.cr3 == 0 {
//     //     panic!("Attempted to switch to a task with an invalid page table!");
//     // } else if next.cr3 != current.registers.cr3 {
//     //     asm!("mov cr3, $0" : : "r"(next.cr3) : "memory" : "intel", "volatile");
//     // }

//     asm!("pushfq ; pop $0" : "=r"(current.registers.rflags) : : "memory" : "intel", "volatile");
//     asm!("push $0 ; popfq" : : "r"(next.rflags) : "memory" : "intel", "volatile");

//     asm!("mov $0, rax" : "=r"(current.registers.rax) : : "memory" : "intel", "volatile");
//     asm!("mov rax, $0" : : "r"(next.rax) : "memory" : "intel", "volatile");

//     asm!("mov $0, rbx" : "=r"(current.registers.rbx) : : "memory" : "intel", "volatile");
//     asm!("mov rbx, $0" : : "r"(next.rbx) : "memory" : "intel", "volatile");

//     asm!("mov $0, rcx" : "=r"(current.registers.rcx) : : "memory" : "intel", "volatile");
//     asm!("mov rcx, $0" : : "r"(next.rcx) : "memory" : "intel", "volatile");

//     asm!("mov $0, rdx" : "=r"(current.registers.rdx) : : "memory" : "intel", "volatile");
//     asm!("mov rdx, $0" : : "r"(next.rdx) : "memory" : "intel", "volatile");

//     // asm!("mov $0, rsi" : "=r"(current.registers.rsi) : : "memory" : "intel", "volatile");
//     // asm!("mov rsi, $0" : : "r"(next.rsi) : "memory" : "intel", "volatile");

//     asm!("mov $0, rdi" : "=r"(current.registers.rdi) : : "memory" : "intel", "volatile");
//     asm!("mov rdi, $0" : : "r"(next.rdi) : "memory" : "intel", "volatile");

//     asm!("mov $0, r12" : "=r"(current.registers.r12) : : "memory" : "intel", "volatile");
//     asm!("mov r12, $0" : : "r"(next.r12) : "memory" : "intel", "volatile");

//     asm!("mov $0, r13" : "=r"(current.registers.r13) : : "memory" : "intel", "volatile");
//     asm!("mov r13, $0" : : "r"(next.r13) : "memory" : "intel", "volatile");

//     asm!("mov $0, r14" : "=r"(current.registers.r14) : : "memory" : "intel", "volatile");
//     asm!("mov r14, $0" : : "r"(next.r14) : "memory" : "intel", "volatile");

//     asm!("mov $0, r15" : "=r"(current.registers.r15) : : "memory" : "intel", "volatile");
//     asm!("mov r15, $0" : : "r"(next.r15) : "memory" : "intel", "volatile");

//     asm!("mov $0, rbp" : "=r"(current.registers.rbp) : : "memory" : "intel", "volatile");
//     asm!("mov rbp, $0" : : "r"(next.rbp) : "memory" : "intel", "volatile");

//     asm!("mov $0, rsp" : "=r"(current.registers.rsp) : : "memory" : "intel", "volatile");
//     asm!("mov rsp, $0" : : "r"(next.rsp) : "memory" : "intel", "volatile");

//     // current.registers.rip = x86::read_rip();
//     x86::jmp(next.rsp);
// }

pub unsafe fn switch() {
    system_log!("[scheduler] switch signal received");

    let scheduler_lock = SCHEDULER.read();

    let current_id = CURRENT_TASK.read().id;
    let next_task: &Process;

    let next_task_id = scheduler_lock.next_id();
    if next_task_id.is_none() {
        system_log!("[scheduler] no next task id");
        return;
    }

    let next_task_context = scheduler_lock
        .get_task(next_task_id.unwrap())
        .unwrap()
        .registers;

    system_log!(
        "[scheduler] switching tasks from {} to {} (0x{:x})",
        current_id,
        next_task_id.unwrap(),
        next_task_context.rip,
    );

    let current_task = scheduler_lock.get_task(current_id);
    if current_task.is_none() {
        system_log!("[scheduler] no current task id");
        return;
    };

    CURRENT_TASK.write().id = next_task_id.unwrap();

    switch_to(&current_task.unwrap().registers, &next_task_context);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process_creation() {
        let mut sc = Scheduler::new();

        // we had to create init process
        assert_eq!(sc.tasks.len(), 1);

        sc.spawn(1);

        assert_eq!(sc.process_id_counter, 2);
        assert_eq!(sc.tasks.len(), 2);

        assert_eq!(sc.tasks[&1].id, 1);
        assert_eq!(sc.tasks[&2].id, 2);
    }

    #[test]
    fn test_next_process() {
        let mut sc = Scheduler::new();

        // we had to create init process
        assert_eq!(sc.tasks.len(), 1);

        sc.spawn(1);

        assert_eq!(CURRENT_TASK.read().id, 1);

        let next_id = sc.next_id().unwrap();
        assert_eq!(next_id, 2);
        assert_eq!(sc.get_task_mut(next_id).unwrap().id, 2);
    }

    #[test]
    fn test_get_next_when_current_is_latest() {
        let mut sc = Scheduler::new();

        // we had to create init process
        assert_eq!(sc.tasks.len(), 1);

        let current_id = CURRENT_TASK.read().id;
        assert_eq!(current_id, 1);
        assert_eq!(sc.get_task_mut(current_id).unwrap().id, 1);

        let next_id = sc.next_id().unwrap();
        assert_eq!(next_id, 1);
        assert_eq!(sc.get_task_mut(next_id).unwrap().id, 1);
    }

    #[test]
    fn test_get_current_process() {
        let mut sc = Scheduler::new();

        // we had to create init process
        assert_eq!(sc.tasks.len(), 1);

        let current_id = CURRENT_TASK.read().id;
        assert_eq!(current_id, 1);
        assert_eq!(sc.get_task_mut(current_id).unwrap().id, 1);
    }
}
