#![feature(panic_implementation)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate alloy;

#[macro_use]
extern crate lazy_static;
extern crate x86_64;

use core::panic::PanicInfo;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    debug!("2 + 2 = {} and 65.0 / 9.0 = {}", 2 + 2, 65.0 / 9.0);
    error!("Errors look like this");
    serial_println!("Hello, host! 1 + 2 = {}", 1 + 2);

    alloy::gdt::init();
    init_idt();

    // Invoke a breakpoint exception
    x86_64::instructions::int3();

    debug!("Initializing PIC");
    alloy::pic::init();

    debug!("Enabling interrupts");
    x86_64::instructions::interrupts::enable();

    debug!("Looping forever...");
    loop {}

    //fn stack_overflow() {
    //    stack_overflow();
    //}
    //stack_overflow();

    info!("Shutting down QEMU...");
    unsafe { alloy::exit_qemu(); }

    loop {}
}

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    loop {}
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(handle_breakpoint);
        idt.segment_not_present.set_handler_fn(handle_segment_not_present);

        unsafe {
            idt.double_fault.set_handler_fn(handle_double_fault)
                            .set_stack_index(alloy::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        for i in 32..48 {
            idt[i].set_handler_fn(handle_int_other);
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn handle_breakpoint(stack_frame: &mut ExceptionStackFrame) {
    debug!("EXCEPTION: Breakpoint\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn handle_double_fault(
    stack_frame: &mut ExceptionStackFrame, _error_code: u64)
{
    error!("EXCEPTION: Double fault\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn handle_segment_not_present(
    stack_frame: &mut ExceptionStackFrame, _error_code: u64)
{
    // Usually, this occurs because of an empty entry for an interrupt.
    error!("EXCEPTION: Segment not present\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn handle_int_other(_stack_frame: &mut ExceptionStackFrame) {
    debug!("Interrupt occurred");
}
