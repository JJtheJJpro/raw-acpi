use crate::SDTHeader;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Differentiated System Description Table
///
/// The Differentiated System Description Table (DSDT) is part of the system fixed description.
/// The DSDT is comprised of a system description table header followed by data in Definition Block format. See Section 5.2.11 for a description of Definition Blocks.
/// During initialization, OSPM finds the pointer to the DSDT in the Fixed ACPI Description Table (using the FADT's `dsdt` or `x_dsdt` fields) and then loads the DSDT to create the ACPI Namespace.
pub struct DifferentiatedSystemDescriptionTable {
    /// - **Signature** - "DSDT"
    /// - **Revision** - This field also sets the global integer width for the AML interpreter.
    /// Values less than two will cause the interpreter to use 32-bit integers and math. Values of two and greater will cause the interpreter to use full 64-bit integers and math.
    pub header: SDTHeader,
    /// The bytes of AML code.
    pub def_block: [u8; 0],
}
impl DifferentiatedSystemDescriptionTable {
    pub const fn def_block(&self) -> &[u8] {
        // SAFETY: I sure hope the OEM doesn't frick things up...
        unsafe {
            core::slice::from_raw_parts(
                (self as *const _ as *const u8).add(crate::SDT_HEADER_SIZE),
                self.header.length as usize - crate::SDT_HEADER_SIZE,
            )
        }
    }
}
