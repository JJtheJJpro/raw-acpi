use crate::srat::affinity::ProcessorLocalAPICAffinityFlags;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Processor Local APIC/SAPIC Affinity Structure
///
/// The Processor Local APIC/SAPIC Affinity structure provides the association between the APIC ID or SAPIC ID/EID of a processor and the proximity domain to which the processor belongs.
pub struct ProcessorLocalAPICAffinity {
    /// 0 - Processor Local APIC/SAPIC Affinity Structure
    pub r#type: u8,
    /// 16
    pub length: u8,
    /// Bit [[7:0]] of the proximity domain to which the processor belongs.
    pub proximity_domain_7_0: u8,
    /// The processor local APIC ID.
    pub apic_id: u8,
    /// Flags for the Processor Local APIC/SAPIC Affinity Structure.
    pub flags: ProcessorLocalAPICAffinityFlags,
    /// The processor local SAPIC EID.
    pub local_sapic_eid: u8,
    /// Bit [[31:8]] of the proximity domain to which the processor belongs.
    pub proximity_domain_31_8: [u8; 3],
    /// The clock domain to which the processor belongs. See _CDM (Clock Domain).
    pub clock_domain: u32,
}