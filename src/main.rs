#![no_std]
#![no_main]
#![feature(asm)]

//RESUME https://os.phil-opp.com/minimal-rust-kernel/#printing-to-screen

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// Called on panic
#[panic_handler]
fn panic(_into: &PanicInfo) -> ! {
    loop {}
}