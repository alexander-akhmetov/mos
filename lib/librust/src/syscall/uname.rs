pub use self::calls::{_system_call_1, read_rax};
use super::calls;
use alloc::string::String;
use core::fmt;

pub struct UtsName {
    pub sysname: String,
    pub version: String,
}

impl UtsName {
    pub fn new() -> UtsName {
        UtsName {
            sysname: String::from("unknown"),
            version: String::from("unknown"),
        }
    }
}

impl fmt::Display for UtsName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.sysname, self.version)
    }
}

pub unsafe fn uname(result: &UtsName) -> u64 {
    /// get name and information about current kernel
    _system_call_1(109, result as *const _ as u64);
    read_rax()
}
