use constants;
use core::fmt;
use drivers::serial;
use drivers::vga_buffer::colors;
use multitasking::focus::get_focused_pid;

/// prints output to the screen and serial port 1
/// so output will be available in the qemu running
/// with `-serial mon:stdio` paramete

#[derive(PartialEq, PartialOrd, Eq)]
pub enum LogLevels {
    DEBUG = 1,
    INFO,
    OK,
    WARNING,
    ERROR,
}

impl fmt::Display for LogLevels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogLevels::DEBUG => write!(f, "DEBUG"),
            LogLevels::OK => write!(f, "OK"),
            LogLevels::INFO => write!(f, "INFO"),
            LogLevels::WARNING => write!(f, "WARNING"),
            LogLevels::ERROR => write!(f, "ERROR"),
        }
    }
}

#[macro_export]
macro_rules! system_log_debug {
    () => (
        if $crate::constants::LOGLEVEL <= $crate::logging::LogLevels::DEBUG {
            let level = $crate::logging::LogLevels::DEBUG;
            _system_log!(level);
        };
    );

    ($fmt:expr) => (
        if $crate::constants::LOGLEVEL <= $crate::logging::LogLevels::DEBUG {
            let level = $crate::logging::LogLevels::DEBUG;
            _system_log!(level, $fmt);
        }
    );

    ($fmt:expr, $($arg:tt)*) => (
        if $crate::constants::LOGLEVEL <= $crate::logging::LogLevels::DEBUG {
            let level = $crate::logging::LogLevels::DEBUG;
            _system_log!(level, $fmt, $($arg)*);
        };
    );
}

#[macro_export]
macro_rules! system_log_ok {
    () => (
        _system_log!($crate::logging::LogLevels::OK);
    );

    ($fmt:expr) => (
        _system_log!($crate::logging::LogLevels::OK, $fmt);
    );

    ($fmt:expr, $($arg:tt)*) => (
        _system_log!($crate::logging::LogLevels::OK, $fmt, $($arg)*);
    );
}

#[macro_export]
macro_rules! system_log_warn {
    () => (
        if $crate::constants::LOGLEVEL <= $crate::logging::LogLevels::WARNING {
            let level = $crate::logging::LogLevels::WARNING;
            _system_log!(level)
        }
    );

    ($fmt:expr) => (
        if $crate::constants::LOGLEVEL <= $crate::logging::LogLevels::WARNING {
            let level = $crate::logging::LogLevels::WARNING;
            _system_log!(level, $fmt)
        }
    );

    ($fmt:expr, $($arg:tt)*) => (
        if $crate::constants::LOGLEVEL <= $crate::logging::LogLevels::WARNING {
            let level = $crate::logging::LogLevels::WARNING;
            _system_log!(level, $fmt, $($arg)*);
        }
    );
}

#[macro_export]
macro_rules! system_log {
    () => (
        let level = $crate::logging::LogLevels::INFO;
        _system_log!(level)
    );

    ($fmt:expr) => (
        let level = $crate::logging::LogLevels::INFO;
        _system_log!(level, $fmt)
    );

    ($fmt:expr, $($arg:tt)*) => (
        let level = $crate::logging::LogLevels::INFO;
        _system_log!(level, $fmt, $($arg)*)
    );
}

#[macro_export]
macro_rules! _system_log {
    ($level:expr) => (
        let color = $crate::drivers::vga_buffer::colors::WHITE;
        $crate::logging::print_prefix($level);
        serial_kprintln!("\n");
        if $crate::multitasking::focus::get_focused_pid() == 0 {
            kprintln!();
        }
    );

    ($level:expr, $fmt:expr) => (
        $crate::logging::print_prefix($level);
        serial_kprintln!($fmt);
        if $crate::multitasking::focus::get_focused_pid() == 0 {
            kprintln!($fmt);
        }
    );

    ($level:expr, $fmt:expr, $($arg:tt)*) => (
        $crate::logging::print_prefix($level);
        serial_kprintln!($fmt, $($arg)*);
        if $crate::multitasking::focus::get_focused_pid() == 0 {
            kprintln!($fmt, $($arg)*);
        }
    );
}

pub fn print_prefix(level: LogLevels) {
    let color: colors::ColorCode = match level {
        LogLevels::DEBUG => colors::LIGHT_GRAY,
        LogLevels::INFO => colors::WHITE,
        LogLevels::OK => colors::GREEN,
        LogLevels::WARNING => colors::YELLOW,
        LogLevels::ERROR => colors::RED,
    };

    #[cfg(not(test))]
    serial_kprint!("[kernel] {}: ", level);
    if get_focused_pid() == 0 {
        #[cfg(not(test))]
        kprint_color!(color, "[kernel] {}: ", level);
    }
}
