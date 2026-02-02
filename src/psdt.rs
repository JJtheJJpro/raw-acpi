use crate::SDTHeader;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Persistent System Description Table
///
/// The table signature, "PSDT" refers to the Persistent System Description Table (PSDT) defined in the ACPI 1.0 specification.
/// The PSDT was judged to provide no specific benefit and as such has been deleted from follow-on versions of the ACPI specification.
/// OSPM will evaluate a table with the "PSDT" signature in like manner to the evaluation of an SSDT as described in Section 5.2.11.2
pub struct PersistentSystemDescriptionTable {
    /// - **Signature** - "PSDT"
    pub header: SDTHeader,
    /// The bytes of AML code.
    pub def_block: [u8; 0],
}
impl PersistentSystemDescriptionTable {
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
