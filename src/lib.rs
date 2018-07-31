#![no_std]
#![feature(crate_visibility_modifier)]

extern crate bootloader_precompiled;
#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate volatile;
extern crate uart_16550;
extern crate x86_64;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

pub mod io;
pub mod log;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;
    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}
