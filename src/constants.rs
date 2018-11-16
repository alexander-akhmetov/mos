use logging;

pub const LOGLEVEL: logging::LogLevels = logging::LogLevels::INFO;
pub const KERNEL_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const KERNEL_SYSNAME: &'static str = "mos";
pub const KERNEL_MACHINE: &'static str = "x86_64";
