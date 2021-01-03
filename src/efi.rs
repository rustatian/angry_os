use crate::print;

///! https://dox.ipxe.org/annotated.html
use core::{
    sync::atomic::{AtomicPtr, Ordering},
    u32, usize,
};

/// A pointer to the EFI system table which is saved upon the entry of the kernel.
///
/// Used to do input and output to the console.
static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(core::ptr::null_mut());

//
// The EFI memory allocation functions work in units of EFI_PAGEs that are
// 4KB. This should in no way be confused with the page size of the processor.
// An EFI_PAGE is just the quanta of memory in EFI.
//
const EFI_PAGE_SIZE: u64 = 4096;

/// https://dox.ipxe.org/efi__wrap_8c.html#ac42cc329230339dc8ca22883bf0de060
pub fn get_memory_map() {
    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    if st.is_null() {
        return;
    }

    let mut memory_map = [0u8; 8 * 1024];
    let mut free_memory = 0u64;
    unsafe {
        // size of the memory_map
        let mut remaining = core::mem::size_of_val(&memory_map);
        let mut key = 0;
        let mut descriptor_size = 0;
        let mut descriptor_version = 0;

        let ret = ((*(*st).boot_services).get_memory_map)(
            &mut remaining,
            memory_map.as_mut_ptr(),
            &mut key,
            &mut descriptor_size,
            &mut descriptor_version,
        );

        assert!(ret.0 == 0, "{:x?}", ret);

        for off in (0..remaining).step_by(descriptor_size) {
            let desc =
                core::ptr::read_unaligned(memory_map[off..].as_ptr() as *const EfiMemoryDescriptor);
            let typ: EfiMemoryType = desc.r#type.into();

            if typ.avail_post_exit_boot_services() {
                free_memory += desc.number_of_pages * EFI_PAGE_SIZE;
            }

            print!(
                "{:016x} {:08x} {:016x} {:?}\n",
                desc.physical_start,
                desc.number_of_pages * EFI_PAGE_SIZE,
                desc.attribute,
                typ
            );
        }
    }

    print!("Total bytes free {}\n", free_memory);
}

pub fn output_string(string: &str) {
    // get system table
    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    // ugghhh, ok, null is possible
    if st.is_null() {
        return;
    }

    // Get the console stdout pointer
    let out = unsafe { (*st).con_out };

    // Create a tmp buffer capable of holding 31 character + null terminator at once
    //
    // UEFI uses UCS-2 and not utf-16
    let mut tmp = [0u16; 32];
    let mut idx = 0;

    // iterate over all characters
    for chr in string.encode_utf16() {
        if chr == b'\n' as u16 {
            tmp[idx] = b'\r' as u16;
            idx += 1;
        }

        tmp[idx] = chr;
        idx += 1;
        // full without null terminator
        if idx == (tmp.len() - 2) {
            tmp[idx] = 0; // null terminator
        }

        // write to stdout
        unsafe {
            ((*out).output_string)(out, tmp.as_ptr());
        }
        // clear
        idx = 0;
    }

    if idx > 0 {
        // null terminator
        tmp[idx] = 0;
        // write to stdout
        unsafe {
            ((*out).output_string)(out, tmp.as_ptr());
        }
    }
}

/// Register a system table pointer.
pub unsafe fn register_system_table(system_table: *mut EfiSystemTable) -> Result<(), ()> {
    let res = EFI_SYSTEM_TABLE.compare_exchange(
        core::ptr::null_mut(),
        system_table,
        Ordering::SeqCst,
        Ordering::Relaxed,
    );

    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

// https://dox.ipxe.org/UefiMultiPhase_8h.html#a0e2cdd0290e753cca604d3977cbe8bb9
#[derive(Clone, Copy, Debug)]
#[repr(C)]
enum EfiMemoryType {
    ///
    /// Not used.
    ///
    EfiReservedMemoryType,
    ///
    /// The code portions of a loaded application.
    /// (Note that UEFI OS loaders are UEFI applications.)
    ///
    EfiLoaderCode,
    ///
    /// The data portions of a loaded application and the default data allocation
    /// type used by an application to allocate pool memory.
    ///
    EfiLoaderData,
    ///
    /// The code portions of a loaded Boot Services Driver.
    ///
    EfiBootServicesCode,
    ///
    /// The data portions of a loaded Boot Serves Driver, and the default data
    /// allocation type used by a Boot Services Driver to allocate pool memory.
    ///
    EfiBootServicesData,
    ///
    /// The code portions of a loaded Runtime Services Driver.
    ///
    EfiRuntimeServicesCode,
    ///
    /// The data portions of a loaded Runtime Services Driver and the default
    /// data allocation type used by a Runtime Services Driver to allocate pool memory.
    ///
    EfiRuntimeServicesData,
    ///
    /// Free (unallocated) memory.
    ///
    EfiConventionalMemory,
    ///
    /// Memory in which errors have been detected.
    ///
    EfiUnusableMemory,
    ///
    /// Memory that holds the ACPI tables.
    ///
    EfiACPIReclaimMemory,
    ///
    /// Address space reserved for use by the firmware.
    ///
    EfiACPIMemoryNVS,
    ///
    /// Used by system firmware to request that a memory-mapped IO region
    /// be mapped by the OS to a virtual address so it can be accessed by EFI runtime services.
    ///
    EfiMemoryMappedIO,
    ///
    /// System memory-mapped IO region that is used to translate memory
    /// cycles to IO cycles by the processor.
    ///
    EfiMemoryMappedIOPortSpace,
    ///
    /// Address space reserved by the firmware for code that is part of the processor.
    ///
    EfiPalCode,
    ///
    /// A memory region that operates as EfiConventionalMemory,
    /// however it happens to also support byte-addressable non-volatility.
    ///
    EfiPersistentMemory,
    EfiMaxMemoryType,
    Invalid,
}

impl EfiMemoryType {
    /// Returns whether of not this memory type is available for general
    /// purpose use after boot services have boot exited
    fn avail_post_exit_boot_services(&self) -> bool {
        match self {
            EfiMemoryType::EfiBootServicesCode
            | EfiMemoryType::EfiBootServicesData
            | EfiMemoryType::EfiConventionalMemory
            | EfiMemoryType::EfiPersistentMemory => true,
            _ => return false,
        }
    }
}

impl From<u32> for EfiMemoryType {
    fn from(val: u32) -> Self {
        match val {
            0 => EfiMemoryType::EfiReservedMemoryType,
            1 => EfiMemoryType::EfiLoaderCode,
            2 => EfiMemoryType::EfiLoaderData,
            3 => EfiMemoryType::EfiBootServicesCode,
            4 => EfiMemoryType::EfiBootServicesData,
            5 => EfiMemoryType::EfiRuntimeServicesCode,
            6 => EfiMemoryType::EfiRuntimeServicesData,
            7 => EfiMemoryType::EfiConventionalMemory,
            8 => EfiMemoryType::EfiUnusableMemory,
            9 => EfiMemoryType::EfiACPIReclaimMemory,
            10 => EfiMemoryType::EfiACPIMemoryNVS,
            11 => EfiMemoryType::EfiMemoryMappedIO,
            12 => EfiMemoryType::EfiMemoryMappedIOPortSpace,
            13 => EfiMemoryType::EfiPalCode,
            14 => EfiMemoryType::EfiPersistentMemory,
            15 => EfiMemoryType::EfiMaxMemoryType,
            _ => EfiMemoryType::Invalid,
        }
    }
}

///! UEFI uses the EFI System Table, which contains pointers to the runtime and boot services tables
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiSystemTable {
    // The table header for the EFI System Table
    hdr: EfiTableHeader,
    // A pointer to a null terminated string that identifies the vendor that produces the system firmware for the platform
    firmware_vendor: *const u16,
    // A firmware vendor specific value that identifies the revision of the system firmware for the platform
    firmware_revision: u32,

    // The handle for the active console input device
    console_in_handle: EfiHandle,
    // A pointer to the EFI_SIMPLE_TEXT_INPUT_PROTOCOL interface that is associated with ConsoleInHandle
    con_in: *const EfiSimpleTextInputProtocol,

    // The handle for the active console output device
    console_out_handle: EfiHandle,
    // A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is associated with ConsoleOutHandle
    con_out: *const EfiSimpleTextOutputProtocol,

    // The handle for the active standard error console device
    standard_error_handle: EfiHandle,
    // 	A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is associated with StandardErrorHandle
    std_err: *const EfiSimpleTextOutputProtocol,

    _runtime_services: usize,

    // A pointer to the EFI Boot Services Table
    boot_services: *const EfiBootServices,

    number_of_table_entries: usize,

    _configuration_table: usize,
}

///! Data structure that precedes all of the standard EFI table types
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
struct EfiTableHeader {
    /*
    A 64-bit signature that identifies the type of table that follows. Unique signatures
    have been generated for the EFI System Table, the EFI Boot Services Table, and
    the EFI Runtime Services Table.
    */
    signature: u64,
    /*
    The revision of the EFI Specification to which this table conforms. The upper 16
    bits of this field contain the major revision value, and the lower 16 bits contain
    the minor revision value. The minor revision values are binary coded decimals
    and are limited to the range of 00..99.
    When printed or displayed UEFI spec revision is referred as (Major
    revision).(Minor revision upper decimal).(Minor revision lower decimal) or
    (Major revision).(Minor revision upper decimal) in case Minor revision lower
    decimal is set to 0. For example:
    A specification with the revision value ((2<<16) | (30)) would be referred as 2.3;
    A specification with the revision value ((2<<16) | (31)) would be referred as 2.3.1
    */
    revision: u32,
    // The size, in bytes, of the entire table including the EFI_TABLE_HEADER.
    header_size: u32,
    // The 32-bit CRC for the entire table. This value is computed by setting this field to
    // 0, and computing the 32-bit CRC for HeaderSize bytes.
    crc32: u32,
    // Reserved field that must be set to 0.
    reserved: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiHandle(usize);

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct EfiStatus(pub usize);

/// The keystroke information for the key that was pressed.
#[repr(C)]
struct EfiInputKey {
    scan_code: u16,
    unicode_char: u16,
}

/// https://dox.ipxe.org/struct__EFI__SIMPLE__TEXT__INPUT__PROTOCOL.html
/// The handle for the active console input device
#[repr(C)]
struct EfiSimpleTextInputProtocol {
    //    return EFI_SUCCESS          The device was reset.
    //    return EFI_DEVICE_ERROR     The device is not functioning properly and could not be reset.
    reset: unsafe fn(
        this: *const EfiSimpleTextInputProtocol,
        extended_verification: bool,
    ) -> EfiStatus,

    read_key_stroke: unsafe fn(
        this: *const EfiSimpleTextInputProtocol,
        efi_input_key: *mut EfiInputKey,
    ) -> EfiStatus,

    // TODO implement
    _wait_for_key: usize,
}

/// The SIMPLE_TEXT_OUTPUT protocol is used to control text-based output devices
#[repr(C)]
struct EfiSimpleTextOutputProtocol {
    /**
      Reset the text output device hardware and optionally run diagnostics

      @param  This                 The protocol instance pointer.
      @param  ExtendedVerification Driver may perform more exhaustive verification
                                   operation of the device during reset.

      @retval EFI_SUCCESS          The text output device was reset.
      @retval EFI_DEVICE_ERROR     The text output device is not functioning correctly and
                                   could not be reset.
    **/
    reset: unsafe fn(
        this: *const EfiSimpleTextOutputProtocol,
        extended_verification: bool,
    ) -> EfiStatus,
    /**
      Write a string to the output device.

      @param  This   The protocol instance pointer.
      @param  String The NULL-terminated string to be displayed on the output
                     device(s). All output devices must also support the Unicode
                     drawing character codes defined in this file.

      @retval EFI_SUCCESS             The string was output to the device.
      @retval EFI_DEVICE_ERROR        The device reported an error while attempting to output
                                      the text.
      @retval EFI_UNSUPPORTED         The output device's mode is not currently in a
                                      defined text mode.
      @retval EFI_WARN_UNKNOWN_GLYPH  This warning code indicates that some of the
                                      characters in the string could not be
                                      rendered and were skipped.
    **/
    output_string:
        unsafe fn(this: *const EfiSimpleTextOutputProtocol, string: *const u16) -> EfiStatus,
    /**
      Verifies that all characters in a string can be output to the
      target device.

      @param  This   The protocol instance pointer.
      @param  String The NULL-terminated string to be examined for the output
                     device(s).

      @retval EFI_SUCCESS      The device(s) are capable of rendering the output string.
      @retval EFI_UNSUPPORTED  Some of the characters in the string cannot be
                               rendered by one or more of the output devices mapped
                               by the EFI handle.
    **/
    test_string:
        unsafe fn(this: *const EfiSimpleTextOutputProtocol, string: *const u16) -> EfiStatus,
    /**
      Returns information for an available text mode that the output device(s)
      supports.

      @param  This       The protocol instance pointer.
      @param  ModeNumber The mode number to return information on.
      @param  Columns    Returns the geometry of the text output device for the
                         requested ModeNumber.
      @param  Rows       Returns the geometry of the text output device for the
                         requested ModeNumber.

      @retval EFI_SUCCESS      The requested mode information was returned.
      @retval EFI_DEVICE_ERROR The device had an error and could not complete the request.
      @retval EFI_UNSUPPORTED  The mode number was not valid.
    **/
    query_mode: unsafe fn(
        this: *const EfiSimpleTextOutputProtocol,
        mode_number: usize,
        columns: *mut usize,
        rows: *mut usize,
    ) -> EfiStatus,
}

#[repr(C)]
struct EfiMemoryDescriptor {
    // Type of the memory region.
    r#type: u32,
    ///
    /// Physical address of the first byte of the memory region.  Must aligned
    /// on a 4 KB boundary.
    ///
    physical_start: u64,
    ///
    /// Virtual address of the first byte of the memory region.  Must aligned
    /// on a 4 KB boundary.
    ///
    virtual_start: u64,
    ///
    /// Number of 4KB pages in the memory region.
    ///
    number_of_pages: u64,
    ///
    /// Attributes of the memory region that describe the bit mask of capabilities
    /// for that memory region, and not necessarily the current settings for that
    /// memory region.
    ///
    attribute: u64,
}

// https://dox.ipxe.org/structEFI__BOOT__SERVICES.html#ac2694db09258bd684a07e08f5248c421
// EFI Boot Services Table
#[repr(C)]
struct EfiBootServices {
    // The table header for the EFI Boot Services Table
    hdr: EfiTableHeader,
    _raise_tpl: usize,
    _restore_tpl: usize,
    _allocate_pages: usize,
    _free_pages: usize,

    // https://dox.ipxe.org/UefiSpec_8h.html#a6a58fcf17f205e9b4ff45fd9b198829a
    /**
      Returns the current memory map.

      @param[in, out]  MemoryMapSize         A pointer to the size, in bytes, of the MemoryMap buffer.
                                             On input, this is the size of the buffer allocated by the caller.
                                             On output, it is the size of the buffer returned by the firmware if
                                             the buffer was large enough, or the size of the buffer needed to contain
                                             the map if the buffer was too small.
      @param[in, out]  MemoryMap             A pointer to the buffer in which firmware places the current memory
                                             map.
      @param[out]      MapKey                A pointer to the location in which firmware returns the key for the
                                             current memory map.
      @param[out]      DescriptorSize        A pointer to the location in which firmware returns the size, in bytes, of
                                             an individual EFI_MEMORY_DESCRIPTOR.
      @param[out]      DescriptorVersion     A pointer to the location in which firmware returns the version number
                                             associated with the EFI_MEMORY_DESCRIPTOR.

      @retval EFI_SUCCESS           The memory map was returned in the MemoryMap buffer.
      @retval EFI_BUFFER_TOO_SMALL  The MemoryMap buffer was too small. The current buffer size
                                    needed to hold the memory map is returned in MemoryMapSize.
      @retval EFI_INVALID_PARAMETER 1) MemoryMapSize is NULL.
                                    2) The MemoryMap buffer is not too small and MemoryMap is
                                       NULL.
    **/
    get_memory_map: unsafe fn(
        memory_map_size: &mut usize,
        memory_map: *mut u8,
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> EfiStatus,

    _allocate_pool: usize,
    _free_pool: usize,
    _create_event: usize,
    _set_timer: usize,
    _wait_for_event: usize,
    _signal_event: usize,
    _close_event: usize,
    _check_event: usize,
    _install_protocol_interface: usize,
    _reinstall_protocol_interface: usize,
    _uninstall_protocol_interface: usize,
    _handle_protocol: usize,
    _reserved: usize,
    _register_protocol_notify: usize,
    _locate_handle: usize,
    _locate_device_path: usize,
    _install_configuration_table: usize,
    _load_image: usize,
    _start_image: usize,
    _exit: usize,
    _unload_image: usize,

    exit_boot_services: unsafe fn(image_handle: EfiHandle, map_key: usize) -> EfiStatus,
}