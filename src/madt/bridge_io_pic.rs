#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Bridge I/O Programmable Interrupt Controller (BIO PIC) Structure
///
/// BIO PIC (Bridge I/O Programmable Interrupt Controller) manages legacy IRQs of chipset devices, and routed them to HT PIC or EIO PIC.
pub struct BridgeIOProgrammableInterruptController {
    /// 22 - Bridge I/O Programmable Interrupt Controller Structure
    pub r#type: u8,
    /// Length of the Bridge I/O Programmable Interrupt Controller Structure in bytes.
    ///
    /// **JJ's Note: There doesn't seem to be any variable-sized fields in this struct.  The size is 17 bytes...**
    pub length: u8,
    /// - **0x00** - Invalid
    /// - **0x01** - BIO PIC v1
    ///
    /// Other values are reserved.
    pub version: u8,
    /// The base address of BIO PIC registers.
    pub base_address: u64,
    /// The register space size of BIO PIC.
    pub size: u16,
    /// The hardware ID of BIO PIC.
    pub hardware_id: u16,
    /// The global system interrupt number from which this BIO PIC’s interrupt inputs start.
    ///
    /// For GSI of each interrupt input, GSI = GSI base + interrupt input vector of BIO PIC.
    pub gsi_base: u16,
}