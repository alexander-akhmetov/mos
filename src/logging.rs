#[macro_use]
use vga_buffer;
#[macro_use]
use serial;



#[macro_export]
macro_rules! system_log {
    () => (
        serial_kprint!("\n");
        kprintln!();
    );
    ($fmt:expr) => (
        serial_kprintln!($fmt);
        kprintln!($fmt);
    );
    ($fmt:expr, $($arg:tt)*) => (
        serial_kprintln!($fmt, $($arg)*);
        kprintln!($fmt, $($arg)*);
    );
}
