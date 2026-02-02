#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## GIC Interrupt Translation Service (ITS) Affinity Structure
///
/// The GIC ITS Affinity Structure provides the association between a GIC ITS and a proximity domain.
/// This enables the OSPM to discover the memory that is closest to the ITS, and use that in allocating its management tables and command queue.
/// The ITS is identified using an ID matching a declaration of a GIC ITS in the MADT.
pub struct GICInterruptTranslationServiceAffinity {
    /// 4 - GIC ITS Affinity Structure
    pub r#type: u8,
    /// 12
    pub length: u8,
    /// Integer that represents the proximity domain to which the GIC ITS belongs to.
    pub proximity_domain: u32,
    reserved: u16,
    /// ITS ID matching a GIC ITS entry in the MADT.
    pub its_id: u32,
}