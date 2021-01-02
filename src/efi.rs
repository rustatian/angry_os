use core::sync::atomic::{AtomicPtr, Ordering};

pub fn output_string(string: &str) {}

/// A pointer to the EFI system table which is saved upon the entry of the kernel.
///
/// Used to do input and output to the console.
static EFI_SYSTEM_TABLE: AtomicPtr<SystemTable> = AtomicPtr::new(core::ptr::null_mut());

/// Register a system table pointer.
pub unsafe fn register_system_table(system_table: *mut SystemTable) {
    EFI_SYSTEM_TABLE.compare_exchange(
        core::ptr::null_mut(),
        system_table,
        Ordering::SeqCst,
        Ordering::Relaxed,
    );
}

///! UEFI uses the EFI System Table, which contains pointers to the runtime and boot services tables
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SystemTable {
    hdr: EfiTableHeader,
    firmware_vendor: *mut u16,
    firmware_revision: u32,
    console_in_handle: EfiHandle,
    con_in: *mut EfiSimpleTextInputProtocol,
    console_out_handle: EfiHandle,
    con_out: *mut EfiSimpleTextOutputProtocol,
}

///! Data structure that precedes all of the standard EFI table types
#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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
    output_string: unsafe fn(this: *const EfiSimpleTextOutputProtocol, string: u16) -> EfiStatus,
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
    test_string: unsafe fn(this: *const EfiSimpleTextOutputProtocol, string: u16) -> EfiStatus,
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
