#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

pub const LIGHT_BLUE: ColorCode = ColorCode::new(Color::LightBlue, Color::Black);
pub const BLUE: ColorCode = ColorCode::new(Color::Blue, Color::Black);

pub const RED: ColorCode = ColorCode::new(Color::Red, Color::Black);
pub const LIGHT_RED: ColorCode = ColorCode::new(Color::LightRed, Color::Black);

pub const GREEN: ColorCode = ColorCode::new(Color::Green, Color::Black);
pub const LIGHT_GREEN: ColorCode = ColorCode::new(Color::LightGreen, Color::Black);

pub const CYAN: ColorCode = ColorCode::new(Color::Cyan, Color::Black);
pub const LIGHT_CYAN: ColorCode = ColorCode::new(Color::LightCyan, Color::Black);

pub const LIGHT_GRAY: ColorCode = ColorCode::new(Color::LightGray, Color::Black);
pub const WHITE: ColorCode = ColorCode::new(Color::White, Color::Black);

pub const YELLOW: ColorCode = ColorCode::new(Color::Yellow, Color::Black);
pub const MAGENTA: ColorCode = ColorCode::new(Color::Magenta, Color::Black);
