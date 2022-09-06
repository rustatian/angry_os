#![no_std]
#![no_main]

use core::panic::PanicInfo;

/*
1. C calling convention: https://en.wikipedia.org/wiki/Calling_convention
2. https://en.wikipedia.org/wiki/Crt0
 */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

/*
The eh_personality language item marks a function that is used for implementing stack unwinding.

1. https://en.wikipedia.org/wiki/Runtime_system
2. Rust init: https://github.com/rust-lang/rust/blob/master/library/std/src/rt.rs
3.
*/

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
