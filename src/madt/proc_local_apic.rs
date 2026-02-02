use crate::madt::LocalAPICFlags;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Processor Local APIC Structure
///
/// When using the APIC interrupt model, each processor in the system is required
/// to have a Processor Local APIC record in the MADT, and a processor device object in the DSDT.
/// OSPM does not expect the information provided in this table to be updated
/// if the processor information changes during the lifespan of an OS boot.
/// While in the sleeping state, processors are not allowed to be added, removed, nor can their APIC ID or Flags change.
/// When a processor is not present, the Processor Local APIC information is either not reported or flagged as disabled.
pub struct ProcessorLocalAPIC {
    /// 0 - Processor Local APIC structure
    pub r#type: u8,
    /// 8
    pub length: u8,
    /// The OS associates this Local APIC Structure with a processor object in the namespace when the _UID child object
    /// of the processor's device object (or the ProcessorId listed in the Processor declaration operator)
    /// evaluates to a numeric value that matches the numeric value in this field.
    ///
    /// Note that the use of the Processor declaration operator is deprecated. See the documentation for `MADT`.
    pub acpi_processor_uid: u8,
    /// The processor's local APIC ID.
    pub acpi_id: u8,
    /// Local APIC flags.
    pub flags: LocalAPICFlags,
}