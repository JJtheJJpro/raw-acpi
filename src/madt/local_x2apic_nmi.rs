use crate::madt::interrupt_source_override::MPSINTIFlags;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Local x2APIC NMI Structure
///
/// The Local APIC NMI and Local x2APIC NMI structures describe the interrupt input (LINTn) that NMI is connected to for each of the logical processors in the system where such a connection exists.
/// Each NMI connection to a processor requires a separate NMI structure. This information is needed by OSPM to enable the appropriate APIC entry.
///
/// NMI connection to a logical processor with local x2APIC ID 255 and greater requires an X2APIC NMI structure.
/// NMI connection to a logical processor with an x2APIC ID less than 255 requires a Local APIC NMI structure.
/// For example, if the platform contains 8 logical processors with x2APIC IDs 0-3 and 256-259 and NMI is connected LINT1
/// for processor 3, 2, 256 and 257 then two Local APIC NMI entries and two X2APIC NMI entries must be provided in the MADT.
///
/// The Local APIC NMI structure is used to specify global LINTx for all processors if all logical processors have x2APIC ID less than 255.
/// If the platform contains any logical processors with an x2APIC ID of 255 or greater then the Local X2APIC NMI structure must be used to specify global LINTx for ALL logical processors.
pub struct Localx2APICNMI {
    /// 10 - Local x2APIC NMI Structure
    pub r#type: u8,
    /// 12
    pub length: u8,
    /// Same as MPS INTI Flags.
    pub flags: MPSINTIFlags,
    /// UID corresponding to the ID listed in the processor Device object. A value of 0xFFFFFFFF signifies that this applies to all processors in the machine.
    pub acpi_processor_uid: u32,
    /// Local x2APIC interrupt input LINTn to which NMI is connected.
    pub local_x2apic_lint_num: u8,
    reserved: [u8; 3],
}