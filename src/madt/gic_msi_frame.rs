#[derive(Copy, Clone)]
/// ## GIC MSI Frame Flags
pub struct GICMSIFrameFlags(u32);
impl GICMSIFrameFlags {
    /// - **0** - The SPI Count and Base fields should be ignored, and the actual values should be queried from the MSI_TYPER register in the associated GIC MSI frame.
    /// - **1** - The SPI Count and Base values override the values specified in the MSI_TYPER register in the associated GIC MSI frame.
    pub const fn spi_count_base_select(&self) -> bool {
        self.0 & 0b1 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## GIC MSI Frame Structure
///
/// Each GICv2m MSI frame consists of a 4k page which includes registers to generate message signaled interrupts to an associated GIC distributor.
/// The frame also includes registers to discover the set of distributor lines which may be signaled by MSIs from that frame.
/// A system may have multiple MSI frames, and separate frames may be defined for secure and non-secure access.
///
/// This structure must only be used to describe non-secure MSI frames.
pub struct GICMSIFrame {
    /// 13 - GIC MSI Frame Structure
    pub r#type: u8,
    /// 24
    pub length: u8,
    reserved: u16,
    /// GIC MSI Frame ID.
    ///
    /// In a system with multiple GIC MSI frames, this value must be unique to each one.
    pub gic_msi_frame_id: u32,
    /// The 64-bit physical address for this MSI Frame
    pub physical_base_address: u64,
    /// GIC MSI Frame Flags.
    pub flags: GICMSIFrameFlags,
    /// SPI Count used by this frame.
    ///
    /// Unless the SPI Count Select flag is set to 1 this, value should match the lower 16 bits of the MSI_TYPER register in the frame.
    pub spi_count: u16,
    /// SPI Base used by this frame.
    ///
    /// Unless the SPI Base Select flag is set to 1 this, value should match the upper 16 bits of the MSI_TYPER register in the frame.
    pub spi_base: u16,
}