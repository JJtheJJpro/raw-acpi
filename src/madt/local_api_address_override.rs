#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Local APIC Address Override Structure
///
/// This optional structure supports 64-bit systems by providing an override of the physical address of the local APIC in the MADT's table header, which is defined as a 32-bit field.
///
/// If defined, OSPM must use the address specified in this structure for all local APICs (and local SAPICs), rather than the address contained in the MADT's table header.
/// Only one Local APIC Address Override Structure may be defined.
pub struct LocalAPICAddressOverride {
    /// 5 - Local APIC Address Override Structure
    pub r#type: u8,
    /// 12
    pub length: u8,
    reserved: u32,
    /// Physical address of Local APIC. For Itanium™ Processor Family (IPF)-based platforms, this field contains the starting address of the Processor Interrupt Block.
    ///
    /// See the Intel® ItaniumTM Architecture Software Developer's Manual for more information.
    pub local_apic_address: u64,
}