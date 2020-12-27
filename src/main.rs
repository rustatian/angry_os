#![feature(asm)]
#![no_std]
#![no_main]

mod core_requirements;

// panic handler
use core::panic::PanicInfo;

/// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn efi_main() {
    unsafe {
        core::ptr::write_volatile(0x414141444414 as *mut u64, 0);
    }
}

// pub extern fn efi_main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status
