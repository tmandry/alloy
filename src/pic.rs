use pic8259_simple::ChainedPics;
use spin::Mutex;

// Map PIC interrupts to 0x20 through 0x2f.
static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(0x20, 0x28) });

pub fn init() {
    unsafe { PICS.lock().initialize(); }
}

pub unsafe fn clear(interrupt_id: u8) {
    PICS.lock().notify_end_of_interrupt(interrupt_id);
}
