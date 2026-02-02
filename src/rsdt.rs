use crate::SDTHeader;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Root System Description Table structure.
pub struct RootSystemDescriptionTable {
    /// - **Signature** - "RSDT"
    /// - **Revision** - 1
    /// - **OEM Table ID** - For the RSDT, the table ID is the manufacture model ID. This field must match the OEM Table ID in the FADT structure.
    pub header: SDTHeader,
    /// An array of 32-bit physical addresses that point to other System Description Tables.
    /// 
    /// OSPM assumes at least the System Description Table is addressable, and then can further address the table based upon its Length field.
    pub entry: [u32; 0],
}
impl RootSystemDescriptionTable {
    pub const fn entry(&self) -> &[u32] {
        // SAFETY: I sure hope the OEM doesn't frick things up...
        unsafe {
            core::slice::from_raw_parts(
                (self as *const _ as *const u8).add(crate::SDT_HEADER_SIZE) as *const u32,
                (self.header.length as usize - crate::SDT_HEADER_SIZE) / 4,
            )
        }
    }
}