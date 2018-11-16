use super::super::syscall;

pub fn clear() {
    let b: [u8; 1] = ['\n' as u8];
    for _i in 0..80 {
        unsafe {
            syscall::write(syscall::STDOUT, &b, 1);
        };
    }
}
