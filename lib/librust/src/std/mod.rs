use core::fmt;
#[macro_use]
pub mod screen;
pub mod time;
use super::syscall;

pub const EOF: u32 = 1000; // end of file

pub fn getchar() -> char {
    let mut buf: [u8; 1] = [0];
    loop {
        unsafe {
            let result = syscall::read(syscall::STDIN, &mut buf, 1);
            if result == 0 {
                return buf[0] as char;
            }
        };

        time::sleep();
    }
}
