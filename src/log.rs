use core::fmt;
use vga_buffer::{Color, WRITER};

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::log_with_header("DEBUG", $crate::vga_buffer::Color::DarkGray, format_log!($($arg)*));
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::log_with_header("INFO ", $crate::vga_buffer::Color::White, format_log!($($arg)*));
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::log_with_header("ERROR", $crate::vga_buffer::Color::Red, format_log!($($arg)*));
    }
}

#[macro_export]
macro_rules! format_log {
    ($fmt:expr) => (format_args!(concat!(": ", $fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (format_args!(concat!(": ", $fmt, "\n"), $($arg)*));
}

pub fn log_with_header(header: &'static str, color: Color, args: fmt::Arguments) {
    use core::fmt::Write;
    let mut w = WRITER.lock();
    w.write_str_with_foreground(color, header).unwrap();
    w.write_fmt(args).unwrap();
}
