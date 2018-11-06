use cpuio;
use spin::Mutex;

mod codes;

lazy_static! {
    pub static ref KEYBOARD: Mutex<cpuio::Port<u8>> = Mutex::new(unsafe { cpuio::Port::new(0x60) });
}

pub fn read_character() -> Option<char> {
    let code = KEYBOARD.lock().read();
    return codes::char_from_scancode(code as usize);
}
