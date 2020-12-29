/// https://software.intel.com/sites/default/files/managed/9e/bc/64-ia-32-architectures-optimization-manual.pdf
/// https://stackoverflow.com/questions/43343231/enhanced-rep-movsb-for-memcpy
/// libc `memcpy` implementation in Rust
///
/// #Parameters
///
/// * `dest` - Pointer to memory to copy to
/// * `src`  - Pointer to memory to copy from
/// * `n`    - Number of bytes to copy
///
/// #Returns
///
/// Pointer to `dest`
///
#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    asm!("rep movsb",
    inout("rcx") n => _,
    inout("rdi") dest => _,
    inout("rsi") src => _);

    dest
}

#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    asm!("rep stosb",
    inout("rcx") n => _,
    inout("rdi") s => _,
    in("eax") c as u32);

    s
}

#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut ii = 0;

    while ii < n {
        let a = *s1.add(ii);
        let b = *s2.add(ii);
        if a != b {
            return (a as i32).wrapping_sub(b as i32);
        }
        ii = ii.wrapping_add(1);
    }
    0
}

#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, mut n: usize) -> *mut u8 {
    if (dest as usize) > (src as usize) && (src as usize).wrapping_add(n) > (dest as usize) {
        let overhang = dest as usize - src as usize;
        if overhang < 64 {
            // 8 byte align the dest with one byte copies

            while n != 0 && (dest as usize).wrapping_add(n) & 0x7 != 0 {
                n = n.wrapping_sub(1);
                *dest.offset(n as isize) = *src.offset(n as isize);
            }

            // Do a rev copy 8-bytes at a time
            while n >= 8 {
                n = n.wrapping_sub(8);
                // Read the value to copy
                let val = core::ptr::read_unaligned(src.offset(n as isize) as *const u64);

                // Write value
                core::ptr::write(dest.offset(n as isize) as *mut u64, val);
            }

            // copy the reminder
            while n != 0 {
                n = n.wrapping_sub(1);
                *dest.offset(n as isize) = *src.offset(n as isize);
            }

            return dest;
        }

        // copy the non-overlapping tail part
        while n >= overhang {
            // update the length reminder
            n = n.wrapping_sub(overhang);

            // copy the remaining parts
            let src = src.offset(n as isize);
            let dst = dest.offset(n as isize);
            memcpy(dest, src, overhang);
        }

        // check if we copied everything
        if n == 0 {
            return dest;
        }
    }

    // just copy forwards
    memcpy(dest, src, n);

    dest
}
