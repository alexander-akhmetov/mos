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
        unsafe {
            // x86::hlt();
            switch()
        };
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

    pub fn exit_current(&mut self) {
        let current_task_id = CURRENT_TASK.read().id;
        self.tasks.remove(&current_task_id);
        system_log!("[scheduler] task {} exited", current_task_id);
        // todo: proper remove
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

pub unsafe fn switch() {
    system_log!("[scheduler] switch signal received");

    let current_id = CURRENT_TASK.read().id;
    let next_task: &Process;

    let next_task_id = SCHEDULER.read().next_id();
    if next_task_id.is_none() {
        system_log!("[scheduler] no next task id");
        return;
    }

    let next_task_context = SCHEDULER
        .read()
        .get_task(next_task_id.unwrap())
        .unwrap()
        .registers;

    system_log!(
        "[scheduler] switching tasks from {} to {} (0x{:x})",
        current_id,
        next_task_id.unwrap(),
        next_task_context.rip,
    );

    if current_id == next_task_id.unwrap() {
        return;
    }

    let current_task_context = SCHEDULER
        .read()
        .get_task(current_id)
        .unwrap_or(&Process::new(0, 0)) // if there is no current process, just give mock to switch func
        .registers;

    CURRENT_TASK.write().id = next_task_id.unwrap();

    switch_to(&current_task_context, &next_task_context);
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
