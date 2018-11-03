use spin::Mutex;
use cpuio;

pub mod polling;
pub mod codes;


lazy_static! {
    pub static ref KEYBOARD: Mutex<cpuio::Port<u8>> = Mutex::new(cpuio::Port::new(0x60));
}
