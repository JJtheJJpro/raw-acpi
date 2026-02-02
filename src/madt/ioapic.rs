#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## I/O APIC Structure
///
/// In an APIC implementation, there are one or more I/O APICs.
/// Each I/O APIC has a series of interrupt inputs, referred to as INTIn,
/// where the value of n is from 0 to the number of the last interrupt input on the I/O APIC.
/// The I/O APIC structure declares which global system interrupts are uniquely associated with the I/O APIC interrupt inputs.
/// There is one I/O APIC structure for each I/O APIC in the system.
/// For more information on global system interrupts see Global System Interrupts.
pub struct IOAPIC {
    /// 1 - I/O APIC structure
    pub r#type: u8,
    /// 12
    pub length: u8,
    /// The I/O APIC's ID.
    pub io_apic_id: u8,
    reserved: u8,
    /// The 32-bit physical address to access this I/O APIC. Each I/O APIC resides at a unique address.
    pub io_apic_address: u32,
    /// The global system interrupt number where this I/O APIC's interrupt inputs start.
    /// 
    /// The number of interrupt inputs is determined by the I/O APIC's Max Redir Entry register.
    pub global_system_interrupt_base: u32,
}