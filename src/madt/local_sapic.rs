use crate::madt::LocalAPICFlags;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Local SAPIC Structure
///
/// The Processor local SAPIC structure is very similar to the processor local APIC structure.
/// When using the SAPIC interrupt model, each processor in the system is required to have a Processor Local SAPIC record in the MADT, and a processor device object in the DSDT.
/// OSPM does not expect the information provided in this table to be updated if the processor information changes during the lifespan of an OS boot.
/// While in the sleeping state, processors are not allowed to be added, removed, nor can their SAPIC ID or Flags change.
/// When a processor is not present, the Processor Local SAPIC information is either not reported or flagged as disabled.
pub struct LocalSAPIC {
    /// 7 - Processor Local SAPIC Structure
    pub r#type: u8,
    /// Length of the Local SAPIC Structure in bytes.
    pub length: u8,
    /// OSPM associates the Local SAPIC Structure with a processor object declared in the namespace using the Processor statement by matching the processor object's ProcessorID value with this field.
    ///
    /// The use of the Processor statement is deprecated. See the compatibility note in Processor Local x2APIC Structure, and Processor (Declare Processor).
    pub acpi_processor_id: u8,
    /// The processor's local SAPIC ID
    pub local_sapic_id: u8,
    /// The processor's local SAPIC EID
    pub local_sapic_eid: u8,
    reserved: [u8; 3],
    /// Local SAPIC flags.
    pub flags: LocalAPICFlags,
    /// OSPM associates the Local SAPIC Structure with a processor object declared in the namespace using the Device statement,
    /// when the _UID child object of the processor device evaluates to a numeric value, by matching the numeric value with this field.
    pub acpi_processor_uid_value: u32,
    /// OSPM associates the Local SAPIC Structure with a processor object declared in the namespace using the Device statement,
    /// when the _UID child object of the processor device evaluates to a string, by matching the string with this field. This value is stored as a null-terminated ASCII string.
    pub acpi_processor_uid_string: [u8; 0],
}