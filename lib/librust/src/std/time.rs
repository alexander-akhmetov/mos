use super::super::syscall;
use super::super::x86_64;

pub fn sleep() {
    for _in in 0..3 {
        unsafe { x86_64::hlt() };
    }
}

struct DateTime {
    // seconds: usize,
    // minutes: usize,
    // hours: usize,
    // day: usize,
    // month: usize,
    // year: usize,
    timestamp: u64,
}

impl DateTime {
    pub fn new(timestamp: u64) -> DateTime {
        DateTime {
            timestamp: timestamp,
        }
    }

    pub fn seconds_ago(&self) -> u64 {
        let current = unsafe { syscall::time() };
        return current - self.timestamp;
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_seconds_ago() {
//         let dt = DateTime::new(1541625081);
//         assert_eq!(dt.seconds_ago(), )
//     }
// }
