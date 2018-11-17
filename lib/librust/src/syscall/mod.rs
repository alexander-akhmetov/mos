mod calls;
mod fs;
mod io;
mod uname;

pub use self::calls::{_system_call, _system_call_1, _system_call_2, read_rax};
pub use self::fs::{chdir, getcwd};
pub use self::io::{read, write, STDIN, STDOUT};
pub use self::uname::{uname, UtsName};

pub unsafe fn exit() -> u64 {
    /// sends system call "exit"
    _system_call(1)
}

pub unsafe fn time() -> u64 {
    /// sends system call "time" and returns current timestamp
    _system_call(13)
}

pub unsafe fn getpid() -> u64 {
    /// sends system call "getpid" and returns pid of the current process
    _system_call(20)
}

pub unsafe fn debug(msg: &str) -> u64 {
    /// sends system call "debug" with msg string
    _system_call_2(0, msg.as_ptr() as u64, msg.len() as u64)
}
