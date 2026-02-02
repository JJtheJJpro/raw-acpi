pub mod generic_initiator;
pub mod generic_port;
pub mod gic_interrupt_translation_service;
pub mod gicc;
pub mod memory;
pub mod proc_local_apic;
pub mod proc_local_x2apic;

#[derive(Copy, Clone)]
/// ## Flags - Processor Local APIC/SAPIC Affinity Structure
pub struct ProcessorLocalAPICAffinityFlags(u32);
impl ProcessorLocalAPICAffinityFlags {
    /// If clear, the OSPM ignores the contents of the Processor Local APIC/SAPIC Affinity Structure.
    /// This allows system firmware to populate the SRAT with a static number of structures but only enable them as necessary.
    pub const fn enabled(&self) -> bool {
        self.0 & 0b1 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
/// ## Flags - Generic Initiator/Port Affinity Structure
pub struct GenericInitiatorPortAffinityFlags(u32);
impl GenericInitiatorPortAffinityFlags {
    /// If clear, the OSPM ignores the contents of the Generic Initiator/Port Affinity Structure.
    /// This allows system firmware to populate the SRAT with a static number of structures, but only enable them as necessary.
    pub const fn enabled(&self) -> bool {
        self.0 & 0b01 != 0
    }
    /// If set, indicates that the Generic Initiator/Port can initiate all transactions at the same architectural level as the host (e.g. full atomicOps, cache coherency, virtual memory, etc.)
    pub const fn architectural_transactions(&self) -> bool {
        self.0 & 0b10 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}
