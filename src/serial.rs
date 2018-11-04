use spin::Mutex;
use uart_16550::SerialPort;

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
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_kprint {
    ($($arg:tt)*) => {
        $crate::serial::print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_kprintln {
    () => (serial_kprint!("\n"));
    ($fmt:expr) => (serial_kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial_kprint!(concat!($fmt, "\n"), $($arg)*));
}
