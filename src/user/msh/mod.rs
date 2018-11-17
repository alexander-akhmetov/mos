mod constants;
mod embedded_commands;
mod msh;

use multitasking::scheduler;

pub fn start() {
    scheduler::spawn(msh::start);
}
