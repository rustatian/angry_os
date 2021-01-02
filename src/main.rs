#![feature(asm, panic_info_message)]
#![no_std]
#![no_main]
#![macro_use]
mod core_requirements;
mod efi;
mod print;

// panic handler
use core::panic::PanicInfo;

/// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("!!! -> PANIC <- !!!\n");
    if let Some(location) = info.location() {
        print!(
            "{}:{}:{}\n",
            location.file(),
            location.line(),
            location.column(),
        );
    }

    if let Some(msg) = info.message() {
        print!("{}\n", msg);
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[no_mangle]
extern "C" fn efi_main(
    _image_handle: efi::EfiHandle,
    st: *mut efi::EfiSystemTable,
) -> efi::EfiStatus {
    unsafe {
        efi::register_system_table(st);
    }
    efi::output_string("HELLO EFI!!!!");
    panic!("PANIC");
}
