use drivers::serial;

/// prints output to the screen and serial port 1
/// so output will be available in the qemu running
/// with `-serial mon:stdio` parameter

#[macro_export]
macro_rules! system_log {
    () => (
        #[cfg(not(test))]
        serial_kprintln!("\n");
        #[cfg(not(test))]
        kprintln_color!();
    );
    ($fmt:expr) => (
        #[cfg(not(test))]
        serial_kprintln!(concat!("[kernel] ", $fmt));
        #[cfg(not(test))]
        kprintln_color!(
            $crate::drivers::vga_buffer::ColorCode::new($crate::drivers::vga_buffer::Color::White, $crate::drivers::vga_buffer::Color::Black),
            concat!("[kernel] ", $fmt),
        );
    );
    ($fmt:expr, $($arg:tt)*) => (
        #[cfg(not(test))]
        serial_kprintln!(concat!("[kernel] ", $fmt), $($arg)*);
        #[cfg(not(test))]
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
        #[cfg(not(test))]
        serial_kprintln!($fmt);
        #[cfg(not(test))]
        kprintln_color!(
            $crate::drivers::vga_buffer::ColorCode::new($crate::drivers::vga_buffer::Color::White, $crate::drivers::vga_buffer::Color::Black),
            $fmt,
        );
    );
    ($fmt:expr, $($arg:tt)*) => (
        #[cfg(not(test))]
        serial_kprintln!($fmt, $($arg)*);
        #[cfg(not(test))]
        kprintln_color!(
            $crate::drivers::vga_buffer::ColorCode::new($crate::drivers::vga_buffer::Color::White, $crate::drivers::vga_buffer::Color::Black),
            $fmt,
            $($arg)*
        );
    );
}
