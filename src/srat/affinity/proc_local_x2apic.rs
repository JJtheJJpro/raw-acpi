use crate::srat::affinity::ProcessorLocalAPICAffinityFlags;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Processor Local x2APIC Affinity Structure
///
/// The Processor Local x2APIC Affinity structure provides the association between the local x2APIC ID of a processor and the proximity domain to which the processor belongs.
pub struct ProcessorLocalx2APICAffinity {
    /// 2 - Processor Local x2APIC Affinity Structure
    pub r#type: u8,
    /// 24
    pub length: u8,
    reserved0: u16,
    /// The proximity domain to which the logical processor belongs.
    pub proximity_domain: u32,
    /// The processor local x2APIC ID.
    pub x2apic_id: u32,
    /// Same as Processor Local APIC/SAPIC Affinity Structure flags.
    pub flags: ProcessorLocalAPICAffinityFlags,
    /// The clock domain to which the logical processor belongs. See _CDM (Clock Domain).
    pub clock_domain: u32,
    reserved1: u32,
}