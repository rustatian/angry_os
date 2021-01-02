pub fn output_string(string: &str) {}

///! Data structure that precedes all of the standard EFI table types
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct TableHeader {
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
pub struct Handle(usize);

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Status(pub usize);
