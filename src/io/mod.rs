pub mod serial;
pub mod vga_buffer;

use core::fmt;

/// The standard color palette in VGA text mode and terminals.
///
/// Really, this is the least common denominator, VGA text mode and ANSI terminals
/// support slightly different sets of colors.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    //Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    //Pink,
    Yellow,
    White,
}

crate trait WriteColor {
    /// Prints the given string to the device with the given foreground color.
    ///
    /// This function restores the original color after it finishes.
    fn write_str_with_foreground(&mut self, foreground: Color, s: &str) -> fmt::Result;
}
