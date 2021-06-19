use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new({
        use Colour::*;
        let spec = ColourSpec { fg: Cyan, bg: Black, bright: true, flash: false };
        let vga_buffer_pointer = 0xb8000 as *mut Buffer;
        Writer {
            column_pos: 0,
            colour_code: spec.to_code(),
            buffer: unsafe { &mut * vga_buffer_pointer }
        }
    });
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy,Clone)]
enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    White = 7,
}

struct ColourSpec {
    fg: Colour,
    bg: Colour,
    bright: bool,
    flash: bool,
}

#[derive(Copy,Clone)]
#[repr(transparent)]
struct ColourCode (u8);

impl ColourSpec {
    fn to_code(&self) -> ColourCode {
        let ColourSpec{fg,bg,bright,flash} = *self;
        ColourCode(
            (flash as u8) << 7 | (bg as u8) << 4 | (bright as u8) << 3 | fg as u8
        )
    }
}

#[derive(Copy,Clone)]
#[repr(C)]
struct ScreenChar {
    ascii_byte: u8,
    colour_code: ColourCode,
}

const HEIGHT: usize = 25;
const WIDTH: usize = 80;

struct Buffer {
    chars: [[Volatile<ScreenChar>; WIDTH]; HEIGHT]
}

pub struct Writer {
    column_pos: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}

impl Writer {

    pub fn write_byte(&mut self, ascii_byte: u8) {
        if self.column_pos >= WIDTH {
            self.newline()
        }
        let colour_code = self.colour_code;
        let screen_char = ScreenChar { ascii_byte, colour_code };
        let row = HEIGHT - 1;
        let col = self.column_pos;
        self.buffer.chars[row][col].write(screen_char);
        self.column_pos += 1;
    }

    fn newline(&mut self) {
        self.column_pos = 0;
        let a = &mut self.buffer.chars;
        for col in 0..WIDTH {
            for row in 0..HEIGHT-1 {
                a[row][col].write(a[row+1][col].read());
            }
            let blank = ScreenChar {
                colour_code: self.colour_code,
                ascii_byte: b' ',
            };
            a[HEIGHT-1][col].write(blank);
        }
    }

    fn write_square(&mut self) { self.write_byte(0xfe) }

    pub fn write_string(&mut self, s: &str) {
        for b in s.bytes() {
            match b  {
                0x20..=0x7e => self.write_byte(b),
                b'\n' => self.newline(),
                _ => self.write_square()
            }
        }
    }
}
