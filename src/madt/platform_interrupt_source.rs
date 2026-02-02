use crate::madt::interrupt_source_override::MPSINTIFlags;

#[derive(Copy, Clone)]
pub struct PlatformInterruptSourceFlags(u32);
impl PlatformInterruptSourceFlags {
    /// When set, indicates that retrieval of error information is allowed from any processor and OSPM is to use the information provided by the processor ID,
    /// EID fields of the Platform Interrupt Source Structure as a target processor hint.
    pub const fn cpei_processor_override(&self) -> bool {
        self.0 & 0b1 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Platform Interrupt Source Structure
///
/// The Platform Interrupt Source structure is used to communicate which I/O SAPIC interrupt inputs are connected to the platform interrupt sources.
///
/// Platform Management Interrupts (PMIs) are used to invoke platform firmware to handle various events (similar to SMI in IA-32).
/// The Intel® ItaniumTM architecture permits the I/O SAPIC to send a vector value in the interrupt message of the PMI type.
/// This value is specified in the I/O SAPIC Vector field of the Platform Interrupt Sources Structure.
///
/// INIT messages cause processors to soft reset.
///
/// If a platform can generate an interrupt after correcting platform errors (e.g., single bit error correction),
/// the interrupt input line used to signal such corrected errors is specified by the Global System Interrupt field in the following table.
/// Some systems may restrict the retrieval of corrected platform error information to a specific processor.
/// In such cases, the firmware indicates the processor that can retrieve the corrected platform error information through the Processor ID and EID fields in the structure below.
/// OSPM is required to program the I/O SAPIC redirection table entries with the Processor ID, EID values specified by the ACPI system firmware.
/// On platforms where the retrieval of corrected platform error information can be performed on any processor,
/// the firmware indicates this capability by setting the CPEI Processor Override flag in the Platform Interrupt Source Flags field of the structure below.
/// If the CPEI Processor Override Flag is set, OSPM uses the processor specified by Processor ID,
/// and EID fields of the structure below only as a target processor hint and the error retrieval can be performed on any processor in the system.
/// However, firmware is required to specify valid values in Processor ID, EID fields to ensure backward compatibility.
///
/// If the CPEI Processor Override flag is clear, OSPM may reject a ejection request for the processor that is targeted for the corrected platform error interrupt.<br>
/// If the CPEI Processor Override flag is set, OSPM can retarget the corrected platform error interrupt to a different processor when the target processor is ejected.
///
/// Note that the _MAT object can return a buffer containing Platform Interrupt Source Structure entries.
/// It is allowed for such an entry to refer to a Global System Interrupt that is already specified by a Platform Interrupt Source Structure provided through the static MADT table,
/// provided the value of platform interrupt source flags are identical.
///
/// Refer to the ItaniumTM Processor Family System Abstraction Layer (SAL) Specification for details on handling the Corrected Platform Error Interrupt.
pub struct PlatformInterruptSource {
    /// 8 - Platform Interrupt Source Structure
    pub r#type: u8,
    /// 16
    pub length: u8,
    /// MPS INTI flags.
    pub flags: MPSINTIFlags,
    /// - **0x01** - PMI
    /// - **0x02** - INIT
    /// - **0x03** - Corrected Platform Error Interrupt
    ///
    /// All other values are reserved.
    pub interrupt_type: u8,
    /// Processor ID of destination.
    pub processor_id: u8,
    /// Processor EID of destination.
    pub processor_eid: u8,
    /// Value that OSPM must use to program the vector field of the I/O SAPIC redirection table entry for entries with the PMI interrupt type.
    pub io_sapic_vector: u8,
    /// The Global System Interrupt that this platform interrupt will signal.
    pub global_system_interrupt: u32,
    /// Platform Interrupt Source Flags.
    pub platform_interrupt_source_flags: PlatformInterruptSourceFlags,
}