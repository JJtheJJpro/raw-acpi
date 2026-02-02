use crate::SDTHeader;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Secondary System Description Table
///
/// Secondary System Description Tables (SSDT) are a continuation of the DSDT.
/// The SSDT is comprised of a system description table header followed by data in Definition Block format. There can be multiple SSDTs present.
/// After OSPM loads the DSDT to create the ACPI Namespace, each secondary system description table listed in the RSDT/XSDT with a unique OEM Table ID is loaded in the order presented in the RSDT/XSDT.
///
/// Additional tables can only add data; they cannot overwrite data from previous tables.
///
/// This allows the OEM to provide the base support in one table and add smaller system options in other tables.
/// For example, the OEM might put dynamic object definitions into a secondary table such that the firmware can construct the dynamic information at boot without needing to edit the static DSDT.
/// A SSDT can only rely on the DSDT being loaded prior to it.
pub struct SecondarySystemDescriptionTable {
    /// - **Signature** - "SSDT"
    pub header: SDTHeader,
    /// The bytes of AML code.
    pub def_block: [u8; 0],
}
impl SecondarySystemDescriptionTable {
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
