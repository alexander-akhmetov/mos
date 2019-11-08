mod constants;
mod embedded_commands;
mod msh;
mod utils;

use crate::multitasking::scheduler;

pub fn start() {
    scheduler::spawn(msh::start);
}
