use alloc::collections::BTreeMap;
use multitasking::process::{Process, ProcessID};
use spin::RwLock;
use sys;

#[derive(PartialEq)]
enum SchedulerStatus {
    /// scheduler starts switching between
    /// tasks only in STARTED state
    INIT,
    STARTED,
}

pub struct Scheduler {
    /// Implements context switching for preemptive multitasking
    // tasks: list of currently active tasks
    tasks: BTreeMap<ProcessID, Process>,
    // counter to generate ids for new processes
    process_id_counter: u32,
    status: SchedulerStatus,
}

pub struct CurrentProcess {
    /// holds information about current task (thread or process)
    pub id: ProcessID,
}

impl CurrentProcess {
    const fn new() -> CurrentProcess {
        CurrentProcess { id: 0 }
    }
}

fn init_task() {
    /// simple function which does nothing and
    /// is being always executing by the kernel
    system_log_debug!(">>>    init task started");
    let mut counter: usize = 0;
    loop {
        system_log_debug!(">>>    init task: counter={}", counter);
        // sys::time::sleep(1000);
        sys::time::stupid_sleep();
        counter += 1;
        if counter > 100_000 {
            counter = 0;
        }
    }
}

impl Scheduler {
    pub fn new() -> Scheduler {
        /// initializes the Scheduler
        let mut sc = Scheduler {
            tasks: BTreeMap::new(),
            process_id_counter: 0,
            status: SchedulerStatus::INIT,
        };
        sc.spawn(init_task as *const () as u64);
        sc
    }

    /// changes status of the scheduler to started
    fn start(&mut self) {
        self.status = SchedulerStatus::STARTED;
    }

    pub fn current_process_id(&self) -> u32 {
        /// returns id of the task which is active now
        CURRENT_PROCESS.read().id
    }

    pub fn exit_current(&mut self) {
        let pid = CURRENT_PROCESS.read().id;
        self.exit(pid);
    }

    pub fn exit(&mut self, pid: u32) {
        /// removes process with `pid` from the tasks list
        self.tasks.remove(&pid);
        system_log!("[scheduler] process {} exited", pid);
    }

    pub fn spawn(&mut self, func_ptr: u64) -> ProcessID {
        // creates a new process from a function pointer
        self.process_id_counter += 1;
        let process = Process::new(self.process_id_counter, func_ptr);
        let pid = process.id;
        self.tasks.insert(pid, process);
        system_log!(
            "[scheduler] new process created with id={} func_ptr=0x{:x}",
            pid,
            func_ptr
        );

        pid
    }

    pub fn get_task_mut(&mut self, id: ProcessID) -> Option<&mut Process> {
        /// returns mutable task from task list by it's id
        self.tasks.get_mut(&id)
    }

    pub fn get_task(&self, id: ProcessID) -> Option<&Process> {
        /// returns task from task list by it's id
        self.tasks.get(&id)
    }

    pub fn get_active_process(&self) -> Option<&Process> {
        /// returns active process
        self.get_task(CURRENT_PROCESS.read().id)
    }

    pub fn get_active_process_mut(&mut self) -> Option<&mut Process> {
        /// returns active process (mutable)
        self.get_task_mut(CURRENT_PROCESS.read().id)
    }

    pub fn next_id(&self) -> Option<ProcessID> {
        /// returns id of the task which has to be executed next
        //
        // round-robin scheduling
        // first it finds id of the current task,
        // then iterates over all tasks (inc ordering)
        // and returns the first task which id is bigger than the current's
        let current_id = CURRENT_PROCESS.read().id;
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

        None
    }
}

pub static mut SCHEDULER: Option<Scheduler> = None;

lazy_static! {
    pub static ref CURRENT_PROCESS: RwLock<CurrentProcess> = RwLock::new(CurrentProcess::new());
}

pub fn init() {
    /// creates a scheduler instance in the INIT (not started) state
    /// but after this function it's already possible to spawn new processes
    unsafe {
        SCHEDULER = Some(Scheduler::new());
    }
    system_log_ok!("[scheduler] initiated");
}

pub fn start() {
    /// starts the scheduler: it begins context switching process
    unsafe {
        SCHEDULER.as_mut().unwrap().start();
    }
    system_log_ok!("[scheduler] started");
}

pub unsafe fn switch() {
    // Context switch happens in this function.
    // Do not call this fulction while you have holding locks.

    if SCHEDULER.as_ref().unwrap().status != SchedulerStatus::STARTED {
        // if scheduler is not started yet, do nothing
        return;
    }

    // disable interrupts, to avoid problems when interrupt
    // is being raised in the middle of switch
    sys::interrupts::disable();
    system_log_debug!("[scheduler] switch signal received");
    let read_scheduler = SCHEDULER.as_ref().unwrap();

    if read_scheduler.tasks.len() < 1 {
        // if there is no tasks to switch to, just return nothing
        system_log_debug!("[scheduler] no tasks");
        return;
    }

    let next_task: &Process;
    let next_task_id = read_scheduler.next_id();
    if next_task_id.is_none() {
        // if there is no next task - do nothing
        system_log_debug!("[scheduler] no next task id");
        return;
    }

    // get next tasks's context information (registers)
    let next_task_context = SCHEDULER
        .as_ref()
        .unwrap()
        .get_task(next_task_id.unwrap())
        .unwrap()
        .rsp;

    let current_id = CURRENT_PROCESS.read().id;
    system_log_debug!(
        "[scheduler] switching tasks from {} to {} (next task rsp addr: 0x{:x})",
        current_id,
        next_task_id.unwrap(),
        next_task_context,
    );

    if current_id == next_task_id.unwrap() {
        // if next's task id and current's are the same - do nothing
        return;
    }

    // if current
    let current_process_exists = read_scheduler.get_task(current_id).is_some();

    // debugging print current and the next's processes stacks
    if current_process_exists {
        print_current_process_stack();
    };
    read_scheduler
        .get_task(next_task_id.unwrap())
        .unwrap()
        .print_stack();

    // update current task information with next task's id
    CURRENT_PROCESS.write().id = next_task_id.unwrap();

    if current_process_exists {
        // get current tasks's context information (registers)
        // context switch!
        let current_process_context = &mut SCHEDULER
            .as_mut()
            .unwrap()
            .get_task_mut(current_id)
            .unwrap()
            .rsp as *mut u64;
        system_log_debug!("[scheduler] switch");
        #[cfg(not(test))]
        switch_to(current_process_context, next_task_context);
    } else {
        // if there is no process with pid CURRENT_PROCESS.id,
        // we don't have to save context of the current non-existing process,
        // so, just call "start_task" function, which does almost the same as switch_to
        // without saving current CPU state and stack
        system_log_debug!("[scheduler] switch (start)");
        #[cfg(not(test))]
        start_task(next_task_context);
    }
}

#[cfg(not(test))]
#[naked]
extern "C" {
    // from switch_to.asm
    #[inline(always)]
    fn switch_to(old_rsp: *mut u64, new_rsp: u64);
    #[inline(always)]
    fn start_task(rsp: u64);
}

pub fn current_process_id() -> u32 {
    /// returns current task's id
    CURRENT_PROCESS.read().id
}

pub fn spawn(func: fn()) {
    /// spawns a new process
    unsafe {
        SCHEDULER.as_mut().unwrap().spawn(func as *const () as u64);
    }
}

pub fn spawn_addr(func_ptr: u64) {
    /// spawns a new process
    unsafe {
        SCHEDULER.as_mut().unwrap().spawn(func_ptr);
    }
}

pub fn exit(pid: u32) {
    // removes process with id = pid from scheduler's task list
    unsafe {
        SCHEDULER.as_mut().unwrap().exit(pid);
    }
}

pub fn exit_current() {
    let pid = current_process_id();
    unsafe {
        SCHEDULER.as_mut().unwrap().exit(pid);
    }
}

pub fn print_current_process_stack() {
    /// prints current process' stack (only if logging level is DEBUG)
    unsafe {
        let pid = CURRENT_PROCESS.read().id;
        let process = SCHEDULER
            .as_ref()
            .unwrap()
            .get_task(pid)
            .unwrap()
            .print_stack();
    }
}

#[naked]
pub extern "C" fn switch_if_needed() {
    /// executes "switch" function if there is time to switch
    ///     (time to switch is defined in constants::SCHEDULER_TICKS_TO_SWITCH)
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

        assert_eq!(CURRENT_PROCESS.read().id, 0);

        let next_id = sc.next_id().unwrap();
        assert_eq!(next_id, 1);
        assert_eq!(sc.get_task_mut(next_id).unwrap().id, 1);
    }

    #[test]
    fn test_get_next_when_current_is_latest() {
        let mut sc = Scheduler::new();

        // we had to create init process
        assert_eq!(sc.tasks.len(), 1);

        let current_id = CURRENT_PROCESS.read().id;
        assert_eq!(current_id, 0);
        assert_eq!(sc.get_task_mut(1).unwrap().id, 1);

        let next_id = sc.next_id().unwrap();
        assert_eq!(next_id, 1);
        assert_eq!(sc.get_task_mut(next_id).unwrap().id, 1);
    }

    #[test]
    fn test_get_current_process() {
        let mut sc = Scheduler::new();

        // we had to create init process
        assert_eq!(sc.tasks.len(), 1);

        let current_id = CURRENT_PROCESS.read().id;
        assert_eq!(current_id, 0);
        assert_eq!(sc.get_task_mut(1).unwrap().id, 1);
    }
}
