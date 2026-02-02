#[derive(Copy, Clone)]
/// ## Flags - Memory Affinity Structure
pub struct MemoryAffinityStructureFlags(u32);
impl MemoryAffinityStructureFlags {
    /// If clear, the OSPM ignores the contents of the Memory Affinity Structure.
    /// This allows system firmware to populate the SRAT with a static number of structures but only enable then as necessary.
    pub const fn enabled(&self) -> bool {
        self.0 & 0b001 != 0
    }
    /// The information conveyed by this bit depends on the value of the Enabled bit.
    ///
    /// If the Enabled bit is set and the Hot Pluggable bit is also set.
    /// The system hardware supports hot-add and hot-remove of this memory region If the Enabled bit is set and the Hot Pluggable bit is clear,
    /// the system hardware does not support hot-add or hot-remove of this memory region.
    ///
    /// If the Enabled bit is clear, the OSPM will ignore the contents of the Memory Affinity Structure
    pub const fn hot_pluggable(&self) -> bool {
        self.0 & 0b010 != 0
    }
    /// If set, the memory region represents Non-Volatile memory
    pub const fn non_volatile(&self) -> bool {
        self.0 & 0b100 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Memory Affinity Structure
///
/// The Memory Affinity structure provides the following topology information statically to the operating system:
///
/// - The association between a memory range and the proximity domain to which it belongs
/// - Information about whether the memory range can be hot-plugged.
pub struct MemoryAffinity {
    /// 1 - Memory Affinity Structure
    pub r#type: u8,
    /// 40
    pub length: u8,
    /// Integer that represents the proximity domain to which the memory range belongs.
    pub proximity_domain: u32,
    reserved0: u16,
    /// Low 32 Bits of the Base Address of the memory range.
    pub base_address_low: u32,
    /// High 32 Bits of the Base Address of the memory range.
    pub base_address_high: u32,
    /// Low 32 Bits of the length of the memory range.
    pub length_low: u32,
    /// High 32 Bits of the length of the memory range.
    pub length_high: u32,
    reserved1: u32,
    /// Flags - Memory Affinity Structure.
    ///
    /// Indicates whether the region of memory is enabled and can be hot plugged.
    pub flags: MemoryAffinityStructureFlags,
    reserved2: u64,
}