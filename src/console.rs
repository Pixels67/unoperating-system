use core::fmt;
use core::fmt::Write;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 25;
const VGA_BUFFER: *mut [Volatile<u8>; SCREEN_WIDTH * SCREEN_HEIGHT * 2] =
    0xb8000 as *mut [Volatile<u8>; SCREEN_WIDTH * SCREEN_HEIGHT * 2];

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

impl From<u8> for Color {
    fn from(index: u8) -> Self {
        match index {
            0 => Color::Black,
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::Cyan,
            4 => Color::Red,
            5 => Color::Magenta,
            6 => Color::Brown,
            7 => Color::LightGray,
            8 => Color::DarkGray,
            9 => Color::LightBlue,
            10 => Color::LightGreen,
            11 => Color::LightCyan,
            12 => Color::LightRed,
            13 => Color::Pink,
            14 => Color::Yellow,
            15 => Color::White,
            _ => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode {
    value: u8,
}

impl ColorCode {
    pub fn new(text_color: Color, bg_color: Color) -> ColorCode {
        ColorCode {
            value: (bg_color as u8) << 4 | (text_color as u8),
        }
    }

    pub fn get_text_color(&self) -> Color {
        Color::from((self.value << 4) >> 4)
    }

    pub fn get_bg_color(&self) -> Color {
        Color::from(self.value >> 4)
    }
}

lazy_static! {
    pub static ref CONSOLE: Mutex<Console> = Mutex::new(Console::new());
}

pub struct Console {
    char_count: usize,
    vga_buffer: VGABuffer,
}

impl Console {
    pub fn new() -> Self {
        Self {
            char_count: 0,
            vga_buffer: VGABuffer::new(),
        }
    }

    pub fn write_str(&mut self, str: &str) {
        self.write_str_colored(str, ColorCode::new(Color::White, Color::Black));
    }

    pub fn write_str_colored(&mut self, str: &str, color: ColorCode) {
        for char in str.chars() {
            self.write_char_colored(char, color);
        }
    }

    pub fn write_char(&mut self, char: char) {
        self.write_char_colored(char, ColorCode::new(Color::White, Color::Black));
    }

    pub fn write_char_colored(&mut self, char: char, color: ColorCode) {
        if char == '\n' {
            self.new_line();
            return;
        }

        let byte = match char as u8 {
            0x20..=0x7e => char as u8,
            _ => 0xfe,
        };

        self.vga_buffer
            .write_byte(self.get_column(), self.get_line(), byte, color);
        self.char_count += 1;

        if self.char_count % SCREEN_WIDTH == 0 && self.get_line() >= SCREEN_HEIGHT {
            self.scroll(1);
        }
    }

    pub fn new_line(&mut self) {
        self.char_count += SCREEN_WIDTH - self.get_column();

        if self.get_line() >= SCREEN_HEIGHT {
            self.scroll(1);
        }
    }

    pub fn scroll(&mut self, lines: usize) {
        self.vga_buffer.scroll(lines);
        self.char_count -= SCREEN_WIDTH - self.get_column();
    }

    pub fn clear(&mut self) {
        self.vga_buffer.clear();
        self.char_count = 0;
    }

    pub fn get_column(&self) -> usize {
        self.char_count % SCREEN_WIDTH
    }

    pub fn get_line(&self) -> usize {
        self.char_count / SCREEN_WIDTH
    }
}

impl Write for Console {
    fn write_str(&mut self, str: &str) -> fmt::Result {
        self.write_str(str);
        Ok(())
    }
}

#[repr(transparent)]
pub struct VGABuffer {
    buffer: &'static mut [Volatile<u8>; SCREEN_WIDTH * SCREEN_HEIGHT * 2],
}

impl VGABuffer {
    pub fn new() -> Self {
        let buffer =
            unsafe { &mut *VGA_BUFFER };

        let mut buffer = VGABuffer { buffer };
        buffer.clear();
        buffer
    }

    pub fn write_byte(&mut self, x: usize, y: usize, byte: u8, color: ColorCode) {
        let offset = (y * SCREEN_WIDTH + x) * 2;

        if offset + 1 >= self.buffer.len() {
            return;
        }

        self.buffer[offset].write(byte);
        self.buffer[offset + 1].write(color.value);
    }

    pub fn scroll(&mut self, lines: usize) {
        let line_size = SCREEN_WIDTH * 2;
        let screen_size = SCREEN_HEIGHT * line_size;

        let start = lines * SCREEN_WIDTH;
        let end = SCREEN_WIDTH * SCREEN_HEIGHT;

        for i in start..end {
            let src = i * 2;
            let dst = (i - lines * SCREEN_WIDTH) * 2;

            if src + 1 >= screen_size || dst + 1 >= screen_size {
                continue;
            }

            self.buffer[dst].write(self.buffer[src].read());
            self.buffer[dst + 1].write(self.buffer[src + 1].read());
        }

        let start = (SCREEN_HEIGHT - lines) * SCREEN_WIDTH;

        for i in start..end {
            let offset = i * 2;
            self.buffer[offset].write(0);
            self.buffer[offset + 1].write(0);
        }
    }

    pub fn clear(&mut self) {
        let blank = ColorCode::new(Color::White, Color::Black);
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.write_byte(x, y, 0, blank);
            }
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    CONSOLE.lock().write_fmt(args).unwrap();
}