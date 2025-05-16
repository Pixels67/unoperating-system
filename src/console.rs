const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 25;

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

    pub fn write_line(&mut self, str: &str) {
        self.write_line_colored(str, ColorCode::new(Color::White, Color::Black));
    }

    pub fn write_line_colored(&mut self, str: &str, color: ColorCode) {
        for char in str.chars() {
            self.write_char_colored(char, color);
        }

        self.new_line();
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

#[repr(transparent)]
pub struct VGABuffer {
    buffer: *mut u8,
}

impl VGABuffer {
    pub fn new() -> Self {
        let mut buffer = VGABuffer { buffer: VGA_BUFFER };
        buffer.clear();
        buffer
    }

    pub fn write_byte(&mut self, x: usize, y: usize, byte: u8, color: ColorCode) {
        // multiplied by 2 because each element is a text byte and a color byte
        let offset: usize = (y * SCREEN_WIDTH + x) * 2;

        if offset >= SCREEN_WIDTH * SCREEN_HEIGHT * 2 {
            return;
        }

        unsafe {
            *self.buffer.offset(offset as isize) = byte;
            *self.buffer.offset((offset + 1) as isize) = color.value;
        }
    }

    pub fn scroll(&mut self, lines: usize) {
        for i in SCREEN_WIDTH * lines..SCREEN_WIDTH * SCREEN_HEIGHT {
            // multiplied by 2 because each element is a text byte and a color byte
            let offset: isize = (i * 2) as isize;

            unsafe {
                *self.buffer.offset(offset - (SCREEN_WIDTH * 2) as isize) =
                    *self.buffer.offset(offset);
                *self.buffer.offset(offset - (SCREEN_WIDTH * 2) as isize + 1) =
                    *self.buffer.offset(offset + 1);
            }
        }

        for i in (SCREEN_HEIGHT - lines) * SCREEN_WIDTH..SCREEN_HEIGHT * SCREEN_WIDTH {
            // multiplied by 2 because each element is a text byte and a color byte
            let offset: isize = (i * 2) as isize;

            unsafe {
                *self.buffer.offset(offset) = 0;
                *self.buffer.offset(offset + 1) = 0;
            }
        }
    }

    pub fn clear(&mut self) {
        for i in 0..SCREEN_HEIGHT {
            for j in 0..SCREEN_WIDTH {
                self.write_byte(i, j, 0, ColorCode::new(Color::White, Color::Black));
            }
        }
    }
}
