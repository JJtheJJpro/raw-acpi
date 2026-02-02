#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## MSI Programmable Interrupt Controller (MSI PIC) Structure
///
/// MSI Programmable Interrupt Controller Structure is defined to support MSI of PCI/PCIE devices in system.
pub struct MSIProgrammableInterruptController {
    /// 21 - Message Programmable Interrupt Controller Structure
    pub r#type: u8,
    /// Length of the Message Programmable Interrupt Controller Structure in bytes.
    ///
    /// **JJ's Note: There doesn't seem to be any variable-sized fields in this struct.  The size is 24 bytes...**
    pub length: u8,
    /// - **0x00** - Invalid
    /// - **0x01** - MSI PIC v1
    ///
    /// Other values are reserved.
    pub version: u8,
    /// The physical address for MSI.
    pub message_address: u64,
    /// The start vector allocated for MSI from global vectors of HT PIC or EIO PIC.
    pub start: u32,
    /// The count of allocated vectors for MSI.
    pub count: u32,
}