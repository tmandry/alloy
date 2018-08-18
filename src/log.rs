use core::fmt;
use io::Color;

#[cfg(not(test))]
use io::{WriteColor, serial, vga_buffer};
#[cfg(not(test))]
use core::fmt::Write;

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::log_with_header("DEBUG", $crate::io::Color::DarkGray, format_log!($($arg)*));
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::log_with_header("INFO ", $crate::io::Color::White, format_log!($($arg)*));
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::log_with_header("ERROR", $crate::io::Color::Red, format_log!($($arg)*));
    }
}

#[macro_export]
macro_rules! format_log {
    ($fmt:expr) => (format_args!(concat!(": ", $fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (format_args!(concat!(": ", $fmt, "\n"), $($arg)*));
}

#[cfg(not(test))]
pub fn log_with_header(header: &'static str, color: Color, args: fmt::Arguments) {
    log_to_console(header, color, args);
    if cfg!(feature = "log_serial") {
        log_to_serial(header, color, args);
    }
}

#[cfg(not(test))]
fn log_to_console(header: &'static str, color: Color, args: fmt::Arguments) {
    let mut w = vga_buffer::WRITER.lock();
    w.write_str_with_foreground(color, header).unwrap();
    w.write_fmt(args).unwrap();
}

#[cfg(not(test))]
fn log_to_serial(header: &'static str, color: Color, args: fmt::Arguments) {
    let mut w = serial::SERIAL1.lock();
    w.write_str_with_foreground(color, header).unwrap();
    w.write_fmt(args).unwrap();
}

#[cfg(test)]
use std::io::Write;

#[cfg(test)]
pub fn log_with_header(header: &'static str, _color: Color, args: fmt::Arguments) {
    print!("{}", header);
    print!("{}", args);
}
