const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

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

pub struct Console {
    line: usize,
    column: usize,
}

impl Console {
    pub fn new() -> Self {
        clear_screen();
        Self { line: 0, column: 0 }
    }

    pub fn write_line(&mut self, string: &str) {
        for byte in string.chars().map(|c| c as u8) {
            if byte == b'\n' || self.column >= 80 {
                self.new_line();
                
                continue;
            }

            write_to_screen(byte, self.line % 25, self.column % 80, Color::White);
            self.column += 1;
        }
        
        self.new_line();
    }

    pub fn write_line_colored(&mut self, string: &str, color: Color) {
        for byte in string.chars().map(|c| c as u8) {
            if byte == b'\n' || self.column >= 80 {
                self.new_line();
                continue;
            }
            
            write_to_screen(byte, self.line, self.column, color);
            self.column += 1;
        }
        
        self.new_line();
    }
    
    pub fn new_line(&mut self) {
        self.column = 0;
        self.line += 1;

        if self.line >= 25 {
            self.clear()
        }
    }
    
    pub fn clear(&mut self) {
        clear_screen();
        
        self.line = 0;
        self.column = 0;
    }
}

pub fn write_to_screen(byte: u8, line: usize, column: usize, color: Color) {
    unsafe {
        let offset: isize = (line * 160 + column * 2) as isize;
        
        let byte = match byte {
            0x20..=0x7e => byte,
            _ => 0xfe,
        };

        *VGA_BUFFER.offset(offset) = byte;
        *VGA_BUFFER.offset(offset + 1) = color as u8;
    }
}

pub fn clear_screen() {
    for i in 0..80 {
        for j in 0..25 {
            write_to_screen(b' ', j, i, Color::White);
        }
    }
}