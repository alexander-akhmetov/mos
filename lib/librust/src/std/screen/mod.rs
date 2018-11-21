use super::super::syscall;
use core::fmt;
pub mod style;

pub fn clear() {
    let b: [u8; 25] = ['\n' as u8; 25];
    unsafe {
        syscall::write(syscall::STDOUT, &b, 25);
    }
}

pub fn _print(s: &str) {
    unsafe {
        syscall::write(syscall::STDOUT, &s.as_bytes(), s.len() as u64);
    };
}

pub fn printb(s: &[u8]) {
    unsafe {
        syscall::write(syscall::STDOUT, s, s.len() as u64);
    };
}

#[macro_export]
macro_rules! printf {
    () => ($crate::std::screen::_print(&""));
    ($fmt:expr) => ($crate::std::screen::_print(&$fmt));
    ($fmt:expr, $($arg:tt)*) => (
        $crate::std::screen::_print(&format!($fmt, $($arg)*));
    );
}

#[macro_export]
macro_rules! println {
    () => (printf!("\n"));
    ($fmt:expr) => (printf!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (printf!(concat!($fmt, "\n"), $($arg)*));
}
