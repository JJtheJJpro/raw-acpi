#[derive(Copy, Clone)]
/// ## Flags - RINTC Affinity Structure
pub struct RINTCAffinityFlags(u32);
impl RINTCAffinityFlags {
    /// If clear, the OSPM ignores the contents of the RINTC Affinity Structure.
    /// This allows system firmware to populate the SRAT with a static number of structures but only enable them as necessary.
    pub const fn enabled(&self) -> bool {
        self.0 & 0b1 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## RINTC Affinity Structure
///
/// The RINTC Affinity Structure provides the association between the ACPI Processor UID of a RISC-V processor and the proximity domain to which the processor belongs.
pub struct RINTCAffinity {
    /// 7 RINTC Affinity Structure
    pub r#type: u8,
    /// 20
    pub length: u8,
    reserved: u16,
    /// The proximity domain to which the logical processor belongs.
    pub proximity_domain: u32,
    /// The ACPI Processor UID of the associated RINTC.
    pub acpi_processor_uid: u32,
    /// Flags - RINTC Affinity Structure.
    pub flags: RINTCAffinityFlags,
    /// The clock domain to which the logical processor belongs. See _CDM (Clock Domain).
    pub clock_domain: u32,
}