#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## RISC-V Incoming MSI Controller (IMSIC) Structure
/// 
/// The RISC-V advanced interrupt architecture (AIA) defines a per-processor incoming MSI controller (IMSIC) for handling MSIs in a RISC-V platform.
/// 
/// The IMSIC is a per-processor (or per-hart) device with a separate interrupt file for each privilege level (machine or supervisor).
/// The configuration of an IMSIC interrupt file is done using AIA CSRs, and it also has a 4KB MMIO space to receive MSIs from devices.
/// Each IMSIC interrupt file supports a fixed number of interrupt identities (to distinguish MSIs from devices)
/// which is the same for a given privilege level across processors (or harts).
/// 
/// Even though IMSIC is a per-processor, a system with IMSICs must have only one IMSIC structure present in the MADT to provide information common across processors.
/// The RINTC structures will provide the per-processor IMSIC information. The format of the IMSIC structure is listed in the table below.
pub struct RISCVIncomingMSIController {
    /// 25 - IMSIC Structure
    pub r#type: u8,
    /// 16
    pub length: u8,
    pub version: u8,
    reserved: u8,
    /// IMSIC Flags
    /// 
    /// **JJ's Note: all bits are reserved (must be zero).  Until that changes, the flags field will stay inaccessible.**
    flags: u32,
    /// Specifies how many interrupt identities are supported by the IMSIC supervisor interrupt files.
    /// 
    /// - Minimum: 63
    /// - Maximum: 2047 (One less than a multiple of 64).
    pub num_supervisor_interrupt_identities: u16,
    /// Specifies how many interrupt identities are supported by IMSIC guest interrupt files.
    /// 
    /// - Minimum: 63
    /// - Maximum: 2047 (One less than a multiple of 64).
    /// 
    /// This field is zero if no guest interrupt files are implemented.
    pub num_guest_interrupt_identities: u16,
    /// Specifies the number of guest index bits in the MSI target address.
    /// 
    /// This is the least significant bit of the hart index bits in an MSI target address, minus 12.
    /// Values can be in the range of 0 - 7.
    pub guest_index_bits: u8,
    /// Specifies the number of hart index bits in the MSI target address.
    /// 
    /// Values can be in the range of 0 - 15.
    pub hart_index_bits: u8,
    /// Specifies the number of group index bits in the MSI target address.
    /// 
    /// Values can be in the range of 0 - 7.
    pub group_index_bits: u8,
    /// Specifies the least significant bit of the group index bits in the MSI target address.
    /// 
    /// Values can be in the range of 0 - 55.
    /// If there is an APLIC, value can be in the range of 24 - 55.
    pub group_index_shift: u8,
}
