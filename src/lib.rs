#![no_std]
#![feature(crate_visibility_modifier)]
#![feature(crate_in_paths)]

extern crate bootloader_precompiled;
#[macro_use]
extern crate lazy_static;
extern crate pic8259_simple;
extern crate spin;
extern crate uart_16550;
extern crate volatile;
extern crate x86_64;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
pub mod log;

pub mod frame;
pub mod gdt;
pub mod io;
pub mod pic;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;
    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}
