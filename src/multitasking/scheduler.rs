use alloc::collections::BTreeMap;
use multitasking::context::ContextRegisters;
use multitasking::process::{Process, ProcessID};
use spin::RwLock;
use sys;
use x86;

pub struct Scheduler {
    /// Implements context switching for preemptive multitasking
    // tasks: list of currently active tasks
    tasks: BTreeMap<ProcessID, Process>,
    // counter to generate ids for new tasks
    process_id_counter: u32,
}

pub struct CurrentTask {
    /// holds information about current task (thread or process)
    pub id: ProcessID,
}

impl CurrentTask {
    const fn new() -> CurrentTask {
        CurrentTask { id: 0 }
    }
}

fn init_task() -> u64 {
    /// simple function which does nothing and
    /// is being always executing by the kernel
    loop {
        unsafe {
            x86::hlt();
        };
    }
}

impl Scheduler {
    pub fn new() -> Scheduler {
        /// initializes the Scheduler
        let mut sc = Scheduler {
            tasks: BTreeMap::new(),
            process_id_counter: 0,
        };

        let pid = sc.spawn(init_task as *const fn() as u64);
        CURRENT_TASK.write().id = pid;
        sc
    }

    pub fn current_task_id(&self) -> u32 {
        /// returns id of the task which is active now
        CURRENT_TASK.read().id
    }

    pub fn exit_current(&mut self) {
        let current_task_id = CURRENT_TASK.read().id;
        self.tasks.remove(&current_task_id);
        system_log!("[scheduler] task {} exited", current_task_id);
        // todo: proper remove
    }

    pub fn spawn(&mut self, func_ptr: u64) -> ProcessID {
        // creates a new task from a function pointer
        self.process_id_counter += 1;
        let process = Process::new(self.process_id_counter, func_ptr);
        let pid = process.id;
        self.tasks.insert(pid, process);
        system_log!(
            "[scheduler] new task created with pid={} func_ptr=0x{:x}",
            pid,
            func_ptr
        );
        return pid;
    }

    pub fn get_task_mut(&mut self, id: ProcessID) -> Option<&mut Process> {
        /// returns mutable task from task list by it's id
        return self.tasks.get_mut(&id);
    }

    pub fn get_task(&self, id: ProcessID) -> Option<&Process> {
        /// returns task from task list by it's id
        return self.tasks.get(&id);
    }

    pub fn next_id(&self) -> Option<ProcessID> {
        /// returns id of the task which has to be executed next
        //
        // round-robin scheduling
        // first it finds id of the current task,
        // then iterates over all tasks (inc ordering)
        // and returns the first task which id is bigger than the current's
        let current_id = CURRENT_TASK.read().id;
        for (id, task) in self.tasks.iter() {
            if *id > current_id {
                return Some(*id);
            }
        }

        // if current task was the latest task in the list, (no tasks with bigger ids)
        // we will be here, let's return the first task from the list
        for (id, task) in self.tasks.iter() {
            return Some(*id);
        }

        return None;
    }
}

pub static mut SCHEDULER: Option<Scheduler> = None;

lazy_static! {
    pub static ref CURRENT_TASK: RwLock<CurrentTask> = RwLock::new(CurrentTask::new());
}

pub fn init() {
    unsafe {
        SCHEDULER = Some(Scheduler::new());
    }
}

#[naked]
#[no_mangle]
#[inline(always)]
pub unsafe extern "C" fn switch() {
    /// Context switch happens in this function.
    /// Do not call this fulction while you have holding locks.
    system_log!("[scheduler] switch signal received");

    // if there is no tasks to switch to, just return nothing
    if SCHEDULER.as_ref().unwrap().tasks.len() < 2 {
        system_log!("[scheduler] no tasks");
        return;
    }

    let next_task: &Process;
    // if there is no next task - do nothing
    let next_task_id = SCHEDULER.as_ref().unwrap().next_id();
    if next_task_id.is_none() {
        system_log!("[scheduler] no next task id");
        return;
    }

    // get next tasks's context information (registers)
    let next_task_context = SCHEDULER
        .as_ref()
        .unwrap()
        .get_task(next_task_id.unwrap())
        .unwrap()
        .registers;

    let current_id = CURRENT_TASK.read().id;
    system_log!(
        "[scheduler] switching tasks from {} to {} (context: 0x{:x}; rip: 0x{:x})",
        current_id,
        next_task_id.unwrap(),
        &next_task_context as *const _ as u64,
        next_task_context.rip,
    );

    // if next's task id and current's are the same - do nothing
    if current_id == next_task_id.unwrap() {
        return;
    }

    // if current
    let current_task_exists = SCHEDULER.as_ref().unwrap().get_task(current_id).is_some();

    // update current task information with next task's id
    CURRENT_TASK.write().id = next_task_id.unwrap();

    if current_task_exists {
        // get current tasks's context information (registers)
        let mut current_task_context = SCHEDULER
            .as_ref()
            .unwrap()
            .get_task(current_id)
            .unwrap()
            .registers;
        // context switch!
        switch_to(
            (&mut current_task_context) as *mut ContextRegisters,
            &next_task_context,
        );
    } else {
        system_log!("!!!!!!!");
        start_task(&next_task_context);
    }
}

#[naked]
extern "C" {
    #[inline(always)]
    fn switch_to(old_ctx: *mut ContextRegisters, new_ctx: *const ContextRegisters);
    #[inline(always)]
    fn start_task(ctx: *const ContextRegisters);
}

pub fn current_task_id() -> u32 {
    return CURRENT_TASK.read().id;
}

pub fn spawn(func: fn()) {
    unsafe {
        SCHEDULER.as_mut().unwrap().spawn(func as *const () as u64);
    }
}

pub fn exit_current_process() {
    unsafe {
        SCHEDULER.as_mut().unwrap().exit_current();
    }
}

#[naked]
#[inline(always)]
pub fn switch_if_needed() {
    let switch_counter = sys::time::SYSCLOCK.read().switch_counter;
    if switch_counter == 0 {
        unsafe {
            switch();
        }
    };
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
