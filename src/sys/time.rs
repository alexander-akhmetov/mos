use cmos::get_timestamp;
use drivers;
use spin::RwLock;
use sys::constants;
use x86;

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
        // Does a system clock "tick"
        // increases switch_counter which can be used by system scheduler
        // to switch tasks.
        //
        // Also it increases base_timestamp if a second has passed
        self.fraction += constants::TIME_BETWEEN_TICKS;
        self.switch_counter += 1;

        if self.fraction > 1.0 {
            self.fraction -= 1.0;
            self.counter += 1;
            if self.counter % 10 == 0 {
                system_log!("{} system clock: +10 sec", self.timestamp());
            }
        }
        if self.switch_counter == constants::SCHEDULER_TICKS_TO_SWITCH {
            self.switch_counter = 0;
        }
    }

    pub fn timestamp(&self) -> u64 {
        // timestamp returns current timestamp in seconds
        self.base_timestamp + self.counter
    }

    pub fn milliseconds(&self) -> u64 {
        // returns current timestamp in milliseconds
        ((self.base_timestamp * 1000) + (self.fraction * 1000.0) as u64)
    }
}

pub static SYSCLOCK: RwLock<SystemClock> = RwLock::new(SystemClock::new());

pub fn timestamp() -> u64 {
    SYSCLOCK.read().timestamp()
}

pub fn init() {
    /// initializes system clock
    // at first it inits PIT and sets proper frequency
    drivers::pit::init(constants::TIMER_HERTZ);

    // sets current timestamp from CMOS
    match SYSCLOCK.try_write() {
        Some(mut clock) => clock.base_timestamp = get_timestamp(),
        None => panic!("can't lock clock"),
    }

    system_log_ok!("[system clock] started");
}

pub fn sleep(milliseconds: u64) {
    /// sleeps at least N milliseconds
    // first let's calculate when it needs to wake up
    let mut wake_up_at = 0;

    while wake_up_at == 0 {
        let msecs = read_milliseconds_or_none();
        // system clock can be locked by another thread
        if msecs.is_some() {
            wake_up_at = msecs.unwrap() + milliseconds;
        };
    }

    loop {
        // wait...
        // trying to read current milliseconds
        // if system clock is not locked by some else thread,
        // if locked - just does nothing
        let msecs = read_milliseconds_or_none();
        if msecs.is_some() && msecs.unwrap() > wake_up_at {
            // time to wake up!
            break;
        }
        unsafe { x86::hlt() };
    }
}

fn read_milliseconds_or_none() -> Option<u64> {
    /// returns current timestamp in milliseconds
    /// if system clock is not blocked by another thread
    /// if it is, returns none
    let read_sysclock = SYSCLOCK.try_read();

    if read_sysclock.is_some() {
        Some(read_sysclock.unwrap().milliseconds())
    } else {
        None
    }
}

pub fn stupid_sleep() {
    for _in in 0..10 {
        unsafe { x86::hlt() };
    }
}
