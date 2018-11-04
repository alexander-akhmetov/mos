use vga_buffer;
use serial;

#[macro_export]
macro_rules! system_log {
    () => (
        serial_kprint!("\n");
        kprintln!();
    );
    ($fmt:expr) => (
        serial_kprintln!(concat!("[klog] ", $fmt));
        kprintln!(concat!("[klog] ", $fmt));
    );
    ($fmt:expr, $($arg:tt)*) => (
        serial_kprintln!(concat!("[klog] ", $fmt), $($arg)*);
        kprintln!(concat!("[klog] ", $fmt), $($arg)*);
    );
}
