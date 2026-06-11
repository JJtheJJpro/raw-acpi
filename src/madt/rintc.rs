use crate::madt::LocalAPICFlags;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## RISC-V Interrupt Controller (RINTC) Structure
/// 
/// The RISC-V platforms need to have a simple, per-hart (hardware thread or logical processor) interrupt controller available to supervisor mode.
/// Each hart in the system is required to have a RINTC record in the MADT, and a processor device object in the DSDT.
/// 
/// All the RINTCs should be probed by the OSPM before any other interrupt controllers.
/// 
/// For RISC-V platforms, the "Local Interrupt Controller Address" field in the MADT must be ignored by the OSPM.
pub struct RISCVHartLocalInterruptController {
    /// 24 - RISC-V INTC Structure
    pub r#type: u8,
    /// 36
    pub length: u8,
    pub version: u8,
    reserved: u8,
    /// RISC-V INTC Flags
    pub flags: LocalAPICFlags,
    /// Hart ID (mhartid) of the hart this interrupt controller belongs to.
    pub hard_id: u64,
    /// The OS associates this RINTC structure with a processor device object in the namespace when the
    /// _UID child object of the processor device evaluates to a numeric value that matches the numeric value in this field.
    pub acpi_processor_uid: u32,
    /// The unique ID of the external interrupts connected to this hart.
    /// 
    /// This field is valid only when either PLIC or APLIC is the external interrupt controller of this hart and present in the MADT.
    /// 
    /// For APLIC, the format is as follows:
    /// - **Bits [[31:24]]** - APLIC ID
    /// - **Bits [[23:16]]** - Must be zero
    /// - **Bits [[15:00]]** - APLIC IDC ID - This is the index of the Interrupt Delivery Control (IDC) structure
    /// 
    /// For PLIC, the format is as follows:
    /// - **Bits [[31:24]]** - PLIC ID
    /// - **Bits [[23:16]]** - Must be zero
    /// - **Bits [[15:00]]** - PLIC S-Mode Context ID for this hart
    pub external_interrupt_controller_id: u32,
    /// Physical base address of the Incoming MSI Controller (IMSIC) MMIO region of this hart.
    /// 
    /// This field must be ignored by the OSPM when the IMSIC structure is not present in the MADT.
    pub imsic_base_address: u64,
    /// Size in bytes of the IMSIC MMIO region of this hart.
    /// 
    /// This field must be ignored by the OSPM when the IMSIC structure is not present in the MADT.
    /// The size should include supervisor-level and guest-level interrupt files of the hart.
    pub imsic_size: u32
}
