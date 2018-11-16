pub use self::calls::{_system_call_3, read_rax};
use super::calls;

pub const STDIN: u64 = 0;
pub const STDOUT: u64 = 1;

pub unsafe fn read(fd: u64, buf: &mut [u8], count: u64) -> u64 {
    /// attempts to read up to count bytes from file descriptor fd
    /// into the buffer starting at buf
    _system_call_3(3, fd, buf.as_mut_ptr() as u64, count);
    read_rax()
}

pub unsafe fn write(fd: u64, buf: &[u8], count: u64) -> u64 {
    /// writes up to count bytes from the buffer starting at buf to
    /// the file referred to by the file descriptor fd
    _system_call_3(4, fd, buf.as_ptr() as u64, count);
    read_rax()
}
