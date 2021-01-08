//! Memory management routines

/// A strongly typed physical address
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub u64);

/// Read `T` from physical memory address `paddr`
#[inline]
pub unsafe fn read_phys<T>(paddr: PhysAddr) -> T {
    core::ptr::read_volatile(paddr.0 as *const T)
}
