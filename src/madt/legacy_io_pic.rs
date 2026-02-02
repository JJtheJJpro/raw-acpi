#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Legacy I/O Programmable Interrupt Controller (LIO PIC) Structure
///
/// In early Loongson CPUs, Legacy I/O Programmable Interrupt Controller (LIO PIC) routes interrupts from HT PIC to CORE PIC.
pub struct LegacyIOProgrammableInterruptController {
    /// 18 - Legacy I/O Programmable Interrupt Controller Structure
    pub r#type: u8,
    /// Length of the Legacy I/O Programmable Interrupt Controller Structure in bytes.
    ///
    /// **JJ's Note: There doesn't seem to be any variable-sized fields in this struct.  The size is 23 bytes...**
    pub length: u8,
    /// - **0x00** - Invalid
    /// - **0x01** - LIO PIC v1
    ///
    /// Other values are reserved.
    pub version: u8,
    /// The base address of LIO PIC registers.
    pub base_address: u64,
    /// The register space size of LIO PIC.
    pub size: u16,
    /// This field described routed vectors on CORE PIC from LIO PIC vectors.
    pub cascade_vector: u16,
    /// This field described the vectors of LIO PIC routed to the related vector of parent specified by Cascade vector field.
    pub cascade_vector_mapping: u64,
}