#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## I/O SAPIC Structure
///
/// The I/O SAPIC structure is very similar to the I/O APIC structure. If both I/O APIC and I/O SAPIC structures exist for a specific APIC ID, the information in the I/O SAPIC structure must be used.
///
/// The I/O SAPIC structure uses the I/O APIC ID field as defined in the I/O APIC table. The Global System Interrupt Base field remains unchanged but has been moved.
/// The I/O APIC Address field has been deleted. A new address and reserved field have been added.
///
/// If defined, OSPM must use the information contained in the I/O SAPIC structure instead of the information from the I/O APIC structure.
///
/// If both I/O APIC and an I/O SAPIC structures exist in an MADT, the OEM/platform firmware writer must prevent "mixing" I/O APIC and I/O SAPIC addresses.
/// This is done by ensuring that there are at least as many I/O SAPIC structures as I/O APIC structures and that every I/O APIC structure has a corresponding I/O SAPIC structure (same APIC ID).
pub struct IOSAPIC {
    /// 6 - I/O SAPIC Structure
    pub r#type: u8,
    /// 16
    pub length: u8,
    /// I/O SAPIC ID
    pub io_apic_id: u8,
    reserved: u8,
    /// The global system interrupt number where this I/O SAPIC's interrupt inputs start. The number of interrupt inputs is determined by the I/O SAPIC's Max Redir Entry register.
    pub global_system_interrupt_base: u32,
    /// The 64-bit physical address to access this I/O SAPIC. Each I/O SAPIC resides at a unique address.
    pub io_sapic_address: u64,
}