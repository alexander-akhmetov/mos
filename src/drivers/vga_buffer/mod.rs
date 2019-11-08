use alloc::string::String;
use core::fmt;
use spin::Mutex;
use volatile::Volatile;

#[macro_use]
pub mod macroses;
pub mod colors;
use self::colors::{Color, ColorCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn set_color(&mut self, color: ColorCode) {
        self.color_code = color;
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            0x20..=0x7e => self.add_byte(byte),
            b'\n' => self.new_line(),
            _ => self.add_byte(0xfe), // ■
        }
    }

    fn add_byte(&mut self, byte: u8) {
        if self.column_position >= BUFFER_WIDTH {
            self.new_line();
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        self.buffer.chars[row][col].write(ScreenChar {
            ascii_character: byte,
            color_code: self.color_code,
        });
        self.column_position += 1;
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let symbol = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(symbol);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let empty_symbol = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(empty_symbol);
        }
    }

    fn clear_screen(&mut self) {
        for _row in 0..BUFFER_HEIGHT * BUFFER_WIDTH {
            self.write_byte(b' ');
        }
    }

    pub fn print_colored(&mut self, msg: &str) {
        /// parses ansi control chars and prints colored text
        // todo: refactor me
        let old_color_code = self.color_code;
        let msg_len = msg.len();
        let msg_bytes = msg.as_bytes();

        let mut cur_value = String::new();

        for (index, c) in msg_bytes.iter().enumerate() {
            if *c == '\x1b' as u8 {
                self.write_string(&cur_value);
                cur_value.clear();
            }
            cur_value.push(*c as char);

            if cur_value.chars().last().unwrap_or('\0') == 'm' {
                match cur_value.as_str() {
                    "\x1b[31m" => self.color_code = colors::RED,
                    "\x1b[91m" => self.color_code = colors::LIGHT_RED,
                    "\x1b[32m" => self.color_code = colors::GREEN,
                    "\x1b[92m" => self.color_code = colors::LIGHT_GREEN,
                    "\x1b[33m" => self.color_code = colors::YELLOW,
                    "\x1b[93m" => self.color_code = colors::YELLOW,
                    "\x1b[34m" => self.color_code = colors::BLUE,
                    "\x1b[94m" => self.color_code = colors::LIGHT_BLUE,
                    "\x1b[35m" => self.color_code = colors::MAGENTA,
                    "\x1b[95m" => self.color_code = colors::MAGENTA,
                    "\x1b[36m" => self.color_code = colors::CYAN,
                    "\x1b[96m" => self.color_code = colors::LIGHT_CYAN,
                    "\x1b[97m" => self.color_code = colors::WHITE,
                    "\x1b[37m" => self.color_code = colors::LIGHT_GRAY,
                    "\x1b[0m" => self.color_code = colors::LIGHT_GRAY,
                    "\x1b[?25l" => self.hide_cursor(),
                    "\x1b[?25h" => self.show_cursor(),
                    _ => self.write_string(&cur_value),
                }
                cur_value.clear();
            }
        }
        self.write_string(&cur_value);
        self.color_code = old_color_code;
    }

    fn hide_cursor(&mut self) {}

    fn show_cursor(&mut self) {}
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_colored(s);
        Ok(())
    }
}

pub fn print(args: fmt::Arguments, color: ColorCode) {
    use core::fmt::Write;
    unsafe { WRITER.force_unlock() };
    let mut writer = WRITER.lock();
    writer.set_color(color);
    writer.write_fmt(args).unwrap();
}

pub fn clear_screen() {
    WRITER.lock().clear_screen();
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGray, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[cfg(test)]
mod test {
    use super::*;

    fn construct_writer() -> Writer {
        use std::boxed::Box;

        let buffer = construct_buffer();
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Blue, Color::Magenta),
            buffer: Box::leak(Box::new(buffer)),
        }
    }

    fn construct_buffer() -> Buffer {
        use array_init::array_init;

        Buffer {
            chars: array_init(|_| array_init(|_| Volatile::new(empty_char()))),
        }
    }

    fn empty_char() -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Green, Color::Brown),
        }
    }

    #[test]
    fn write_byte() {
        let mut writer = construct_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 1 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'X');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 1 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'Y');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }

    #[test]
    fn write_formatted() {
        use core::fmt::Write;

        let mut writer = construct_writer();
        writeln!(&mut writer, "a").unwrap();
        writeln!(&mut writer, "b{}", "c").unwrap();

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 3 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'a');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'b');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'c');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i >= BUFFER_HEIGHT - 2 {
                    assert_eq!(screen_char.ascii_character, b' ');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }
}
