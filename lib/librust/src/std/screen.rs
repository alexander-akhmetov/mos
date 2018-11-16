use super::super::syscall;

pub fn clear() {
    let b: [u8; 25] = ['\n' as u8; 25];
    unsafe {
        syscall::write(syscall::STDOUT, &b, 25);
    }
}

pub fn print(s: &str) {
    unsafe {
        syscall::write(syscall::STDOUT, &s.as_bytes(), s.len() as u64);
    };
}

pub fn printb(s: &[u8]) {
    unsafe {
        syscall::write(syscall::STDOUT, s, s.len() as u64);
    };
}
