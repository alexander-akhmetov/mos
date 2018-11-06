#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => (
        $crate::drivers::vga_buffer::print(
            format_args!($($arg)*),
            $crate::drivers::vga_buffer::ColorCode::new(
                $crate::drivers::vga_buffer::Color::LightGray,
                $crate::drivers::vga_buffer::Color::Black,
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
