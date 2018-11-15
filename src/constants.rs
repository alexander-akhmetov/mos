#[derive(PartialEq)]
pub enum LogLevels {
    DEBUG,
    INFO,
}

pub const LOGLEVEL: LogLevels = LogLevels::INFO;
