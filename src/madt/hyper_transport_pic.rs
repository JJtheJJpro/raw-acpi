#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## HyperTransport Programmable Interrupt Controller (HT PIC) Structure
///
/// In early Loongson CPUs, HT Programmable Interrupt Controller (HT PIC) routes interrupts from BIO PIC and MSI PIC to LIO PIC.
pub struct HyperTransportProgrammableInterruptController {
    /// 19 - HT Programmable Interrupt Controller Structure
    pub r#type: u8,
    /// Length of the HT Programmable Interrupt Controller Structure in bytes.
    ///
    /// **JJ's Note: There doesn't seem to be any variable-sized fields in this struct.  The size is 21 bytes...**
    pub length: u8,
    /// - **0x00** - Invalid
    /// - **0x01** - HT PIC v1
    ///
    /// Other values are reserved.
    pub version: u8,
    /// The base address of HT PIC registers.
    pub base_address: u64,
    /// The register space size of HT PIC.
    pub size: u16,
    /// This field described routed vector on LIO PIC from HT PIC vectors.
    pub cascade_vector: u64,
}