#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![feature(panic_implementation, start)]

extern crate bootloader_precompiled;
#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate volatile;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;
#[macro_use]
mod log;

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    debug!("2 + 2 = {} and 65.0 / 9.0 = {}", 2 + 2, 65.0 / 9.0);
    error!("Errors look like this");
    info!("Beginning infinite loop...");
    loop {}
}
