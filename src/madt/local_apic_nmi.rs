use crate::madt::interrupt_source_override::MPSINTIFlags;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Local APIC NMI Structure
///
/// This structure describes the Local APIC interrupt input (LINTn) that NMI is connected to for each of the processors in the system where such a connection exists.
/// This information is needed by OSPM to enable the appropriate local APIC entry.
///
/// Each Local APIC NMI connection requires a separate Local APIC NMI structure.
/// For example, if the platform has 4 processors with ID 0-3 and NMI is connected LINT1 for processor 3 and 2, two Local APIC NMI entries would be needed in the MADT.
pub struct LocalAPICNMI {
    /// 4 - Local APIC NMI Structure
    pub r#type: u8,
    /// 6
    pub length: u8,
    /// Value corresponding to the _UID listed in the processor's device object, or the Processor ID corresponding to the ID listed in the processor object.
    ///
    /// A value of 0xFF signifies that this applies to all processors in the machine. Note that the use of the Processor declaration operator is deprecated.
    /// See the compatibility note in Processor Local x2APIC Structure, and see Processor (Declare Processor).
    pub acpi_processor_uid: u8,
    /// MPS INTI flags.
    pub flags: MPSINTIFlags,
    /// Local APIC interrupt input LINTn to which NMI is connected.
    pub local_apic_lint_num: u8,
}