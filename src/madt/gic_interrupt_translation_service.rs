#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## GIC Interrupt Translation Service (ITS) Structure
///
/// The GIC ITS is optionally supported in GICv3/v4 implementations.
pub struct GICInterruptTranslationService {
    /// 15 - GIC ITS Structure
    pub r#type: u8,
    /// 20
    pub length: u8,
    reserved0: u16,
    /// GIC ITS ID.
    ///
    /// In a system with multiple GIC ITS units, this value must be unique to each one.
    pub gic_its_id: u32,
    /// The 64-bit physical address for the Interrupt Translation Service.
    pub physical_base_address: u64,
    reserved1: u32,
}