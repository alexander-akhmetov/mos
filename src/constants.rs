use logging;

pub const LOGLEVEL: logging::LogLevels = logging::LogLevels::INFO;
pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const KERNEL_SYSNAME: &str = "mos";
pub const KERNEL_MACHINE: &str = "x86_64";
