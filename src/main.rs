#![no_std]
#![no_main]

// panic handler
use core::panic::PanicInfo;


/// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn efi_main() -> ! {
    loop {}
}