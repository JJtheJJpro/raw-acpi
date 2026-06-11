#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## RISC-V Platform Level Interrupt Controller (PLIC) Structure
/// 
/// The RISC-V Platform-Level Interrupt Controller Specification defines
/// a platform level interrupt controller (PLIC) for handling wired interrupts in a RISC-V platform.
/// A PLIC signals an interrupt to a hart through a dedicated connection, usually a wire, for each privilege level that the hart may receive interrupts.
/// A system may contain multiple PLICs with each PLIC handling interrupts from a different subset of devices and signaling a different subset of harts.
/// Every PLIC exposed to OSPM must have a matching MADT PLIC structure defined.
pub struct RISCVPlatformLevelInterruptController {
    /// 27 - PLIC Structure
    pub r#type: u8,
    /// 36
    pub length: u8,
    pub version: u8,
    /// ID of this PLIC, should be a unique value across all PLICs.
    pub plic_id: u8,
    /// A valid ACPI ID in the form "NNNN####"" where N is an uppercase letter or a digit ('0'-'9') and # is a hex digit.
    ///
    /// This field is used by the OSPM for any implementation-specific behaviors and quirks.
    pub hardware_id: u64,
    /// Number of external interrupts supported in this PLIC.
    ///
    /// - Minimum: 1
    /// - Maximum: 1023.
    pub total_external_interrupt_sources_supported: u16,
    /// Maximum interrupt priority
    pub max_priority: u16,
    /// RISC-V PLIC Flags
    ///
    /// **JJ's Note: all bits are reserved (must be zero).  Until that changes, the flags field will stay inaccessible.**
    flags: u32,
    /// Length of the PLIC MMIO space.
    pub plic_size: u32,
    /// The 64-bit physical address to access this PLIC.
    /// 
    /// Each PLIC resides at a unique address.
    pub plic_address: u64,
    /// The GSI where this PLIC’s interrupt inputs start.
    pub global_system_interrupt_base: u32,
}
