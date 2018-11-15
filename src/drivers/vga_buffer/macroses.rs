#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => (
        #[cfg(not(test))]
        $crate::drivers::vga_buffer::print(
            format_args!($($arg)*),
            $crate::drivers::vga_buffer::colors::ColorCode::new(
                $crate::drivers::vga_buffer::colors::Color::LightGray,
                $crate::drivers::vga_buffer::colors::Color::Black,
            ),
        )
    );
}

#[macro_export]
macro_rules! kprint_color {
    ($color:expr, $($arg:tt)*) => (
        $crate::drivers::vga_buffer::print(format_args!($($arg)*), $color)
    );
}

#[macro_export]
macro_rules! kprintln {
    () => (kprint!("\n"));
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! kprintln_color {
    () => (kprint!("\n"));
    ($color:expr, $fmt:expr) => (kprint_color!($color, concat!($fmt, "\n")));
    ($color:expr, $fmt:expr, $($arg:tt)*) => (kprint_color!($color, concat!($fmt, "\n"), $($arg)*));
}
