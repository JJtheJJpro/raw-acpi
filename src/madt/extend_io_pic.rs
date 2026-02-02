#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Extend I/O Programmable Interrupt Controller (EIO PIC) Structure
///
/// In newer generation Loongson CPUs, Extend I/O Programmable Interrupt Controller (EIO PIC) replaces the combination of HT PIC and part of LIO PIC,
/// and routes interrupts from BIO PIC and MSI PIC to CORE PIC directly.
pub struct ExtendIOProgrammableInterruptController {
    /// 20 - Extend I/O Programmable Interrupt Controller Structure
    pub r#type: u8,
    /// Length of the Extend I/O Programmable Interrupt Controller Structure in bytes.
    ///
    /// **JJ's Note: There doesn't seem to be any variable-sized fields in this struct.  The size is 13 bytes...**
    pub length: u8,
    /// - **0x00** - Invalid
    /// - **0x01** - EIO PIC v1
    ///
    /// Other values are reserved.
    pub version: u8,
    /// This field describes routed vector on CORE PIC from EIO PIC vectors.
    pub cascade_vector: u8,
    /// The node ID of the node connected to bridge.
    pub node: u8,
    /// Each bit indicates one node that can receive interrupt routing from the EIO PIC.
    pub node_map: u64,
}