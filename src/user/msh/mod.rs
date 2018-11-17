mod constants;
mod embedded_commands;
mod msh;
mod utils;

use multitasking::scheduler;

pub fn start() {
    scheduler::spawn(msh::start);
}
