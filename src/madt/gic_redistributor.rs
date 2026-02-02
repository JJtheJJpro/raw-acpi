#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## GIC Redistributor (GICR) Structure
///
/// This structure enables the discovery of GIC Redistributor base addresses by providing the Physical Base Address of a page range containing the GIC Redistributors.
/// More than one GICR Structure may be presented in the MADT.
/// GICR structures should only be used when describing GIC implementations which conform to version 3 or higher of the GIC architecture and which place all Redistributors in the always-on power domain.
/// When a GICR structure is presented, the OSPM must ignore the GICR Base Address field of the GICC structures (see the following table).
pub struct GICRedistributor {
    /// 14 - GICR Structure
    pub r#type: u8,
    /// 16
    pub length: u8,
    reserved: u16,
    /// The 64-bit physical address of a page range containing all GIC Redistributors.
    pub discovery_range_base_address: u64,
    /// Length of the GIC Redistributor Discovery page range.
    pub discovery_range_length: u32,
}
