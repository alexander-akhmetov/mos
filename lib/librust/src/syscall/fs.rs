pub use self::calls::{_system_call_2, read_rax};
use super::calls;

pub unsafe fn chdir(buf: &mut [u8; 255], length: u64) -> u64 {
    /// change current working dir
    _system_call_2(12, buf.as_mut_ptr() as u64, length);
    read_rax()
}

pub unsafe fn getcwd(buf: &mut [u8; 255], length: u64) -> u64 {
    /// get current working dir
    _system_call_2(183, buf.as_mut_ptr() as u64, length);
    read_rax()
}
