use crate::SDTHeader;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## RAS2 Platform Communication Channel Descriptor
///
/// RAS2 supports multiple PCC channels, where a channel is dedicated to a given component instance.
/// The RAS2 PCC descriptor specifies the PCC sub-space associated with a specific RAS feature.
/// The RAS feature type specifies the RAS feature.
pub struct RAS2PCCDescriptor {
    /// Identifier of the RAS2 Platform Communication Channel.
    ///
    /// OSPM should use this value as an index into the subspace array within the PCCT table
    pub pcc_id: u8,
    reserved: [u8; 2],
    /// RAS feature type.
    ///
    /// - **0x00** - RAS features related to memory
    /// - **0x01-0x7F** - Reserved for future standard RAS feature types defined by this specification.
    /// - **0x80-0xFF** - Vendor-defined RAS feature types.
    pub feature_type: u8,
    /// Identifier for the system component instance that this RAS feature is associated with.
    pub instance: u32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## ACPI RAS2 Feature Table (RAS2)
///
/// The RAS2 table provides interfaces for platform RAS features.
/// RAS2 offers the same services as RASF, but is more scalable than the latter.
/// In particular, RAS2 supports independent RAS controls and capabilities for
/// a given RAS feature for multiple instances of the same component in a given system.
///
/// Platform firmware can publish RAS2 and RASF table but OSPM should use only one.
pub struct RAS2 {
    /// - **Signature** - "RAS2"
    pub header: SDTHeader,
    reserved: [u8; 2],
    /// Number of PCC descriptors.
    pub num_pcc_descriptors: u16,
}
impl RAS2 {
    pub fn ras2_pcc_desc_list(&self) -> &[RAS2PCCDescriptor] {
        
    }
}
