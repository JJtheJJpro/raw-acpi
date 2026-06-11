#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## RISC-V Advanced Platform Level Interrupt Controller (APLIC) Structure
///
/// The RISC-V advanced interrupt architecture (AIA) defines an advanced platform level interrupt controller (APLIC) for handling wired interrupts in a RISC-V platform.
/// In a machine without IMSICs, every RISC-V hart accepts interrupts from exactly one APLIC which is the external interrupt controller for that hart.
/// A hart's external interrupt controller (the APLIC) signals an interrupt to the hart through a dedicated connection, usually a wire,
/// for each privilege level that the hart may receive interrupts.
/// RISC-V harts that employ IMSICs as their external interrupt controllers receive external interrupts only in the form of MSIs.
/// In that case, the role of an APLIC is to convert wired interrupts into MSIs for harts and APLICs should be probed by the OSPM only after probing the IMSIC.
///
/// A system may contain multiple APLICs with each APLIC forwarding interrupts from a different subset of devices.
/// Every APLIC exposed to OSPM must have a matching MADT APLIC structure defined.
pub struct RISCVAdvancedPlatformLevelInterruptController {
    /// 26 - APLIC Structure
    pub r#type: u8,
    /// 36
    pub length: u8,
    pub version: u8,
    /// ID of this APLIC, should be a unique value across all APLICs.
    pub aplic_id: u8,
    /// RISC-V APLIC Flags
    ///
    /// **JJ's Note: all bits are reserved (must be zero).  Until that changes, the flags field will stay inaccessible.**
    flags: u32,
    /// A valid ACPI ID in the form "NNNN####"" where N is an uppercase letter or a digit ('0'-'9') and # is a hex digit.
    ///
    /// This field is used by the OSPM for any implementation-specific behaviors and quirks.
    pub hardware_id: u64,
    /// Number of Interrupt Delivery Control (IDC) structures.
    ///
    /// This should be set to 0 when APLIC is used as a "wired-to-MSI" bridge.
    pub num_idcs: u16,
    /// Number of external interrupts supported in this APLIC.
    ///
    /// - Minimum: 1
    /// - Maximum: 1023.
    pub total_external_interrupt_sources_supported: u16,
    /// The Global System Interrupt number where this APLIC's interrupt inputs start.
    pub global_system_interrupt_base: u32,
    /// The 64-bit physical address to access this APLIC.
    ///
    /// Each APLIC resides at a unique address.
    pub aplic_address: u64,
    /// Length of the APLIC MMIO space.
    pub aplic_size: u32,
}
