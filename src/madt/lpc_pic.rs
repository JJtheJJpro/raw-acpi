#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## LPC Programmable Interrupt Controller (LPC PIC) Structure
///
/// LPC PIC (Low Pin Count Programmable Interrupt Controller) is responsible for handling ISA IRQs of old legacy devices
/// such as PS/2 mouse, keyboard and UARTs for Loongarch machines.
pub struct LPCProgrammableInterruptController {
    /// 23 - LPC Programmable Interrupt Controller Structure
    pub r#type: u8,
    /// Length of the LPC Programmable Interrupt Controller Structure in bytes.
    ///
    /// **JJ's Note: There doesn't seem to be any variable-sized fields in this struct.  The size is 15 bytes...**
    pub length: u8,
    /// - **0x00** - Invalid
    /// - **0x01** - LPC PIC v1
    ///
    /// Other values are reserved.
    pub version: u8,
    /// The base address of LPC PIC registers.
    pub base_address: u64,
    /// The register space size of LPC PIC.
    pub size: u16,
    /// This field described routed vector on BIO PIC from LPC PIC vectors.
    pub cascade_vector: u16,
}