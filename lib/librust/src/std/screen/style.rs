use alloc::string::String;

pub static RED: &'static str = "\x1B[31m";
pub static LIGHT_RED: &'static str = "\x1B[91m";

pub static GREEN: &'static str = "\x1B[32m";
pub static LIGHT_GREEN: &'static str = "\x1B[92m";

pub static YELLOW: &'static str = "\x1B[33m";
pub static LIGHT_YELLOW: &'static str = "\x1B[93m";

pub static BLUE: &'static str = "\x1B[34m";
pub static LIGHT_BLUE: &'static str = "\x1B[94m";

pub static MAGENTA: &'static str = "\x1B[35m";
pub static LIGHT_MAGENTA: &'static str = "\x1B[95m";

pub static CYAN: &'static str = "\x1B[36m";
pub static LIGHT_CYAN: &'static str = "\x1B[96m";

pub static GRAY: &'static str = "\x1B[37m";
pub static WHITE: &'static str = "\x1B[97m";

pub static RESET: &'static str = "\x1B[0m";

pub struct Style {}

impl Style {
    pub fn new() -> Style {
        Style {}
    }

    pub fn format_yellow(msg: &str) -> String {
        return Style::format_colored(msg, YELLOW);
    }

    pub fn format_red(msg: &str) -> String {
        return Style::format_colored(msg, RED);
    }

    pub fn format_light_red(msg: &str) -> String {
        return Style::format_colored(msg, LIGHT_RED);
    }

    fn format_colored(msg: &str, color: &'static str) -> String {
        return format!("{}{}{}", color, msg, RESET);
    }
}
