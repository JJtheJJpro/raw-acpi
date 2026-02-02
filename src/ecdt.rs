use crate::{GenericAddressStructure, SDTHeader};
use core::ffi::CStr;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Embedded Controller Boot Resources Table (ECDT)
///
/// This optional table provides the processor-relative, translated resources of an Embedded Controller.
/// The presence of this table allows OSPM to provide Embedded Controller operation region space access before the namespace has been evaluated.
/// If this table is not provided, the Embedded Controller region space will not be available
/// until the Embedded Controller device in the AML namespace has been discovered and enumerated.
/// The availability of the region space can be detected by providing a _REG method object underneath the Embedded Controller device.
pub struct EmbeddedControllerBootResourcesTable {
    /// - **Signature** - "ECDT"
    pub header: SDTHeader,
    /// Contains the processor relative address, represented in Generic Address Structure format, of the Embedded Controller Command/Status register.
    ///
    /// Note: Only System I/O space and System Memory space are valid for values for `address_space_id`.
    pub ec_control: GenericAddressStructure,
    /// Contains the processor-relative address, represented in Generic Address Structure format, of the Embedded Controller Data register.
    ///
    /// Note: Only System I/O space and System Memory space are valid for values for `address_space_id`.
    pub ec_data: GenericAddressStructure,
    /// Unique ID-Same as the value returned by the _UID under the device in the namespace that represents this embedded controller.
    pub uid: u32,
    /// The bit assignment of the SCI interrupt within the GPEx_STS register of a GPE block described in the FADT that the embedded controller triggers.
    pub gpe_bit: u8,
    /// ASCII, null terminated, string that contains a fully qualified reference to the namespace object that is this embedded controller device (for example, "\_SB.PCI0.ISA.EC").
    /// Quotes are omitted in the data field.
    pub ec_id: [u8; 0],
}
impl EmbeddedControllerBootResourcesTable {
    pub const fn ec_id(&self) -> &CStr {
        unsafe {
            CStr::from_bytes_with_nul_unchecked(core::slice::from_raw_parts(
                (self as *const _ as *const u8).add(0x41),
                self.header.length as usize - 0x41,
            ))
        }
    }
}
