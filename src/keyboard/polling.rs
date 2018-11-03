use keyboard;
use keyboard::codes;

pub struct PollingKeyboard {
    callback: fn(char),
    last_value: u8
}

impl PollingKeyboard {
    pub fn new(callback: fn(char)) -> PollingKeyboard {
        PollingKeyboard {
            callback: callback,
            last_value: 250,
        }
    }

    pub fn update(&mut self) {
        let code = keyboard::KEYBOARD.lock().read();
        if code != self.last_value {
            self.last_value = code;
            let character = keyboard::codes::char_from_scancode(code as usize);
            match character {
                Some(c) => (self.callback)(c),
                None => ()
            }
        }
    }
}
