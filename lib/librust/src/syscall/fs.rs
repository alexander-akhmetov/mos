pub use self::calls::{_system_call_2, _system_call_3, read_rax};
use super::calls;

pub unsafe fn chdir(buf: &[u8]) -> u64 {
    /// change current working dir
    _system_call_2(12, buf.as_ptr() as u64, buf.len() as u64);
    read_rax()
}

pub unsafe fn getcwd(buf: &mut [u8; 255]) -> u64 {
    /// get current working dir
    _system_call_2(183, buf.as_mut_ptr() as u64, buf.len() as u64);
    read_rax()
}

pub unsafe fn readdir(buf: &mut [u8; 255], offset: u64) -> u64 {
    _system_call_3(89, buf.as_mut_ptr() as u64, buf.len() as u64, offset);
    read_rax()
}
