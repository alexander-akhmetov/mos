use super::super::x86_64;

pub fn sleep() {
    for _in in 0..3 {
        unsafe { x86_64::hlt() };
    }
}
