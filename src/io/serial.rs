//! Serial port I/O.
//! 
//! Mainly used with qemu for integration tests.

use super::{Color, WriteColor};
use core::fmt::{self, Write};
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = SerialPort::new(0x3F8);
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::io::serial::print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => (serial_print!("\n"));
    ($fmt:expr) => (serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial_print!(concat!($fmt, "\n"), $($arg)*));
}

fn code(color: Color) -> u8 {
    use self::Color::*;
    match color {
        Black => 30,
        Red => 31,
        Green => 32,
        Yellow => 33,
        Blue => 34,
        Magenta => 35,
        Cyan => 36,
        LightGray => 37,
        DarkGray => 90,
        LightRed => 91,
        LightGreen => 92,
        //LightYellow => 93,
        LightBlue => 94,
        //LightMagenta => 95,
        LightCyan => 96,
        White => 97,
    }
}

impl WriteColor for SerialPort {
    /// Prints the given ASCII string to the serial TTY with the given foreground color.
    ///
    /// This function restores the old color after it finishes.
    fn write_str_with_foreground(&mut self, foreground: Color, s: &str) -> fmt::Result {
        self.write_fmt(format_args!("\x1B[{}m", code(foreground)))?;
        self.write_str(s)?;
        self.write_str("\x1B[0m")?;
        Ok(())
    }
}
