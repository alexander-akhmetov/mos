use drivers::serial;

/// prints output to the screen and serial port 1
/// so output will be available in the qemu running
/// with `-serial mon:stdio` parameter
#[macro_export]
macro_rules! system_log {
    () => (
        serial_kprintln!("\n");
        kprintln_color!();
    );
    ($fmt:expr) => (
        serial_kprintln!(concat!("[kernel] ", $fmt));
        kprintln_color!(
            $crate::drivers::vga_buffer::ColorCode::new($crate::drivers::vga_buffer::Color::White, $crate::drivers::vga_buffer::Color::Black),
            concat!("[kernel] ", $fmt),
        );
    );
    ($fmt:expr, $($arg:tt)*) => (
        serial_kprintln!(concat!("[kernel] ", $fmt), $($arg)*);
        kprintln_color!(
            $crate::drivers::vga_buffer::ColorCode::new($crate::drivers::vga_buffer::Color::White, $crate::drivers::vga_buffer::Color::Black),
            concat!("[kernel] ", $fmt),
            $($arg)*
        );
    );
}

#[macro_export]
macro_rules! system_log_without_prefix {
    () => (

    );
    ($fmt:expr) => (
        serial_kprintln!($fmt);
        kprintln_color!(
            $crate::drivers::vga_buffer::ColorCode::new($crate::drivers::vga_buffer::Color::White, $crate::drivers::vga_buffer::Color::Black),
            $fmt,
        );
    );
    ($fmt:expr, $($arg:tt)*) => (
        serial_kprintln!($fmt, $($arg)*);
        kprintln_color!(
            $crate::drivers::vga_buffer::ColorCode::new($crate::drivers::vga_buffer::Color::White, $crate::drivers::vga_buffer::Color::Black),
            $fmt,
            $($arg)*
        );
    );
}
