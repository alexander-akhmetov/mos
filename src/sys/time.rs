use cmos::get_timestamp;
use drivers;
use multitasking::scheduler::switch;
use spin::RwLock;
use sys::constants;
use x86;

const TIMER_HERTZ: u32 = 100;
const TIME_BETWEEN_TICKS: f32 = 1.0 / (TIMER_HERTZ as f32);

pub struct SystemClock {
    base_timestamp: u64,
    counter: u64,
    fraction: f32,
    pub switch_counter: u32,
}

impl SystemClock {
    const fn new() -> SystemClock {
        SystemClock {
            base_timestamp: 0,
            counter: 0,
            fraction: 0.0,
            switch_counter: 0,
        }
    }

    pub fn tick(&mut self) {
        self.fraction += TIME_BETWEEN_TICKS;
        self.switch_counter += 1;

        if self.fraction > 1.0 {
            self.fraction -= 1.0;
            self.counter += 1;
            system_log!("{} system clock: +1 sec", self.timestamp());
        }
        if self.switch_counter == constants::SCHEDULER_TICKS_TO_SWITCH {
            self.switch_counter = 0;
            // seems like can't do this here, because clocks will be blocked forever
            //     unsafe { switch() };
        }
    }

    pub fn timestamp(&self) -> u64 {
        return self.base_timestamp + self.counter;
    }

    pub fn milliseconds(&self) -> u64 {
        return ((self.base_timestamp * 1000) + (self.fraction * 1000.0) as u64) as u64;
    }
}

pub static SYSCLOCK: RwLock<SystemClock> = RwLock::new(SystemClock::new());

pub fn init() {
    drivers::pit::init(TIMER_HERTZ);
    match SYSCLOCK.try_write() {
        Some(mut clock) => clock.base_timestamp = get_timestamp(),
        None => panic!("can't lock clock"),
    }
    system_log!("System clock initialized");
}

pub fn sleep(milliseconds: u64) {
    let mut awake_at = 0;

    while awake_at == 0 {
        let read_sysclock = SYSCLOCK.try_read();
        if !read_sysclock.is_none() {
            awake_at = read_sysclock.unwrap().milliseconds() + milliseconds;
        };
    }

    loop {
        {
            let read_sysclock = SYSCLOCK.try_read();
            if !read_sysclock.is_none() {
                let current_msec = read_sysclock.unwrap().milliseconds();
                if current_msec >= awake_at {
                    break;
                }
            }
        }
        unsafe { x86::hlt() };
    }
}
