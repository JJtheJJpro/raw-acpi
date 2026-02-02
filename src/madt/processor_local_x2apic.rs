use crate::madt::LocalAPICFlags;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Processor Local x2APIC Structure
///
/// The Processor X2APIC structure is very similar to the processor local APIC structure.
/// When using the X2APIC interrupt model, logical processors are required to have a processor device object in the DSDT
/// and must convey the processor's APIC information to OSPM using the Processor Local X2APIC structure.
///
/// **Compatibility note**: On some legacy OSes, Logical processors with APIC ID values less than 255 (whether in XAPIC or X2APIC mode)
/// must use the Processor Local APIC structure to convey their APIC information to OSPM, and those processors must be declared in the DSDT using the Processor() keyword.
/// Logical processors with APIC ID values 255 and greater must use the Processor Local x2APIC structure and be declared using the Device() keyword.
///
/// OSPM does not expect the information provided in this table to be updated if the processor information changes during the lifespan of an OS boot.
/// While in the sleeping state, logical processors must not be added or removed, nor can their X2APIC ID or x2APIC Flags change.
/// When a logical processor is not present, the processor local X2APIC information is either not reported or flagged as disabled.
pub struct ProcessorLocalx2APIC {
    /// 9 - Processor Local x2APIC Structure
    pub r#type: u8,
    /// 16
    pub length: u8,
    reserved: u16,
    /// The processor's local x2APIC ID.
    pub x2apic_id: u32,
    /// Same as Local APIC flags.
    pub flags: LocalAPICFlags,
    /// OSPM associates the X2APIC Structure with a processor object declared in the namespace using the Device statement,
    /// when the _UID child object of the processor device evaluates to a numeric value, by matching the numeric value with this field.
    pub acpi_processor_uid: u32,
}