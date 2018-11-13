// SCHEDULER_TICKS_TO_SWITCH: after this number of ticks scheduler's switch will be called
pub const SCHEDULER_TICKS_TO_SWITCH: u32 = 100;

// TIMER_HERTZ: configures PIT to send interrupt signal with this frequency
pub const TIMER_HERTZ: u32 = 100;

// TIME_BETWEEN_TICKS: calculated value based on the TIMER_HERTZ (in seconds)
pub const TIME_BETWEEN_TICKS: f32 = 1.0 / (TIMER_HERTZ as f32);

pub const DEBUG: bool = false;
