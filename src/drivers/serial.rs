use spin::Mutex;
use uart_16550::SerialPort;

/// This module provides macroses to write output
/// to serial port 0x3F8.
/// Useful with qemu's `-serial mon:stdio` parameter

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        // 0x3F8 - standard port number for the first serial interface
        let mut serial_port = SerialPort::new(0x3F8);
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    unsafe { SERIAL1.force_unlock() };
    let s = SERIAL1.try_lock();
    if s.is_none() {
        // sometimes it was locked forever
        // todo: fix
        return;
    };
    s.unwrap()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_kprint {
    ($($arg:tt)*) => {
        $crate::drivers::serial::print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_kprintln {
    () => (serial_kprint!("\n"));
    ($fmt:expr) => (serial_kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial_kprint!(concat!($fmt, "\n"), $($arg)*));
}
