use core::fmt;
use lazy_static::lazy_static;
use volatile::Volatile;
use spin::Mutex;

#[allow(dead_code)] // ignore unused color enums
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // make copy-, print- and comparable
#[repr(u8)]
pub enum Color {
    // all VGA colors
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)] // make copy-, print- and comparable
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] // make copy-, print- and comparable
#[repr(C)] // make sure memory is in defined layout
struct ScreenChar {
    // char element in VGA buffer
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    // screen VGA buffer
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT], // Set volatile so compiler doesn't optimize it away, it doesnt know its in VGA buffer, thinks it's never read again
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // if byte is newline, move cursor to next line
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    // if last char of line is reached, move cursor to next line
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    // Write to volitile memory
                    // set char under cursor to input char
                    ascii_char: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            // move every char 1 line up (terminal style)
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1); // clear last row
        self.column_position = 0; // set cursor at beginning
    }

    fn clear_row(&mut self, row: usize) {
        // Set every char in row to ' ' (space)
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte), // If char is ascii char (between 0x20 and 0x7e) of byte => write the byte
                _ => self.write_byte(0xfe), // Else (eg. UTF-8 char) write square char
            }
        }
    }
}

// Write macro
impl fmt::Write for Writer {
    // make it so you can use the write! and writeln! macros
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Define macros
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

// Static mutable WRITER that is initialized on compile time
lazy_static! {
pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }, // Pointer to vga buffer location // unsafe because don't know if valid pointer (but it is though)
});
}
