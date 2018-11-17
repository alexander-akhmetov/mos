// http://www-numi.fnal.gov/offline_software/srt_public_context/WebDocs/Errors/unix_system_errors.html

pub const EOF: u64 = 1000; // End Of File
pub const SUCCESS: u64 = 0; // success
pub const ENOENT: u64 = 2; // No such file or directory
pub const EINTR: u64 = 4; // Interrupted system call
pub const ENOSYS: u64 = 38; // no such system call
