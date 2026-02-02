#[derive(Copy, Clone)]
/// ## Flags - GICC Affinity Structure
pub struct GICCAffinityFlags(u32);
impl GICCAffinityFlags {
    /// If clear, the OSPM ignores the contents of the GICC Affinity Structure.
    /// This allows system firmware to populate the SRAT with a static number of structures but only enable them as necessary.
    pub const fn enabled(&self) -> bool {
        self.0 & 0b1 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## GICC Affinity Structure
///
/// The GICC Affinity Structure provides the association between the ACPI Processor UID of a processor and the proximity domain to which the processor belongs.
pub struct GICCAffinity {
    /// 3 - GICC Affinity Structure
    pub r#type: u8,
    /// 18
    pub length: u8,
    /// The proximity domain to which the logical processor belongs.
    pub proximity_domain: u32,
    /// The ACPI Processor UID of the associated GICC.
    pub acpi_processor_uid: u32,
    /// Flags - GICC Affinity Structure.
    pub flags: GICCAffinityFlags,
    /// The clock domain to which the logical processor belongs. See _CDM (Clock Domain).
    pub clock_domain: u32,
}