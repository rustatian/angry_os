#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {

    }
}

/// Called on panic
#[panic_handler]
fn panic(_into: &PanicInfo) -> ! {
    loop {

    }
}