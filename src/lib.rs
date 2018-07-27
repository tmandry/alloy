#![no_std]

extern crate bootloader_precompiled;
#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate volatile;
extern crate uart_16550;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

pub mod log;
pub mod serial;
pub mod vga_buffer;
