use cmos::get_timestamp;
use drivers;
use spin::Mutex;

const TIMER_HERTZ: u32 = 1000;
const TIME_BETWEEN_TICKS: f32 = 1.0 / (TIMER_HERTZ as f32);

pub struct SystemClock {
    base_timestamp: u64,
    counter: u64,
    fraction: f32,
}

impl SystemClock {
    const fn new() -> SystemClock {
        SystemClock {
            base_timestamp: 0,
            counter: 0,
            fraction: 0.0,
        }
    }

    pub fn tick(&mut self) {
        self.fraction += TIME_BETWEEN_TICKS;
        if self.fraction > 1.0 {
            self.fraction -= 1.0;
            self.counter += 1;
            system_log!("{} system clock: +1 sec", self.timestamp());
        }
    }

    pub fn timestamp(&self) -> u64 {
        return self.base_timestamp + self.counter;
    }
}

pub static SYSCLOCK: Mutex<SystemClock> = Mutex::new(SystemClock::new());

pub fn init() {
    drivers::pit::init(TIMER_HERTZ);
    match SYSCLOCK.try_lock() {
        Some(mut clock) => clock.base_timestamp = get_timestamp(),
        None => panic!("can't lock clock"),
    }
    system_log!("System clock initialized");
}
