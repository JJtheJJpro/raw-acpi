#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## GIC Distributor (GICD) Structure
///
/// ACPI represents all wired interrupts as "flat" values known as global system interrupts (GSIVs).
///
/// On ARM-based systems the Generic Interrupt Controller (GIC) manages interrupts on the system. Each interrupt is identified in the GIC by an interrupt identifier (INTID).
/// ACPI GSIVs map one to one to GIC INTIDs for peripheral interrupts, whether shared (SPI) or private (PPI).
/// The GIC distributor structure describes the GIC distributor to the OS. One, and only one, GIC distributor structure must be present in the MADT for an ARM based system.
pub struct GICDistributor {
    /// 12 - GICD Structure
    pub r#type: u8,
    /// 24
    pub length: u8,
    reserved0: u16,
    /// This GIC Distributor's hardware ID.
    pub gic_id: u32,
    /// The 64-bit physical address for this Distributor.
    pub physical_base_address: u64,
    /// Reserved - Must be zero.
    ///
    /// **JJ's Note: Turns out, this field in practice was shut down quickly due to the nature of ARM interrupts.  This field is not public because it has no use anymore.**
    system_vector_base: u32,
    /// - **0x00** - No GIC version is specified, fall back to hardware discovery for GIC version
    /// - **0x01** - GICv1
    /// - **0x02** - GICv2
    /// - **0x03** - GICv3
    /// - **0x04** - GICv4
    /// - **0x05-0xFF** -  Reserved for future use.
    pub gic_version: u8,
    reserved1: [u8; 3],
}