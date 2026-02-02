use crate::{
    SDTHeader,
    pcct::subspace::{
        GenericCommunicationsChannelCommandField, GenericCommunicationsChannelStatusField,
    },
};

//#[derive(Copy, Clone)]
//#[repr(C, packed)]
//pub struct ParameterBlock {
//    pub r#type: u16,
//    pub version: u16,
//    pub length: u16,
//    pub patrol_scrub_command: u16,
//    pub requested_address_range: u128,
//    pub actual_address_range: u128,
//    pub flags: u16,
//    pub requested_speed: u8,
//}

#[derive(Copy, Clone)]
/// ## Platform RAS Capabilities Bitmap
pub struct RASCapabilities(u128);
impl RASCapabilities {
    /// Indicates that the platform supports hardware based patrol scrub of DRAM memory
    pub const fn hardware_based_patrol_scrub_support(&self) -> bool {
        self.0 & 0b01 != 0
    }
    /// Indicates that the platform supports hardware based patrol scrub of DRAM memory and platform exposes this capability to software using this RASF mechanism
    pub const fn hardware_based_patrol_scrub_support_and_exposed_to_software(&self) -> bool {
        self.0 & 0b10 != 0
    }
    // JJ here, the rest of the values are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## RASF Platform Communication Channel Shared Memory Region
pub struct RASFCommunicationChannelSMR {
    /// The PCC Signature of 0x52415346 (corresponds to ASCII signature of RASF)
    pub signature: u32,
    /// PCC command field.
    ///
    /// See the Platform Communications Channel (PCC).
    /// 
    /// Command value 0x01 will execute RASF command.  The rest of the values are reserved.
    pub command: GenericCommunicationsChannelCommandField,
    /// PCC status field.
    ///
    /// See Platform Communications Channel (PCC).
    pub status: GenericCommunicationsChannelStatusField,
    /// - **Byte 0** - Minor Version
    /// - **Byte 1** - Major Version
    pub version: u16,
    /// Bit Map describing the platform RAS capabilities as shown in Platform RAS Capabilities.
    ///
    /// The Platform populates this field. The OSPM uses this field to determine the RAS capabilities of the platform.
    pub ras_capabilites: RASCapabilities,
    /// Bit Map of the RAS features for which the OSPM is invoking the command.
    ///
    /// The Bit Map is described in Section 5.2.20.4. OSPM sets the bit corresponding to a RAS capability to invoke a command on that capability.
    /// The bitmap implementation allows OSPM to invoke a command on each RAS feature supported by the platform at the same time.
    pub set_ras_capabilities: RASCapabilities,
    /// The Number of parameter blocks will depend on how many RAS Capabilities the Platform Supports.
    ///
    /// Typically, there will be one Parameter Block per RAS Feature, using which that feature can be managed by OSPM.
    pub rasf_parameter_block_num: u16,
    /// - **0b0000** - Success
    /// - **0b0001** - Not Valid
    /// - **0b0010** - Not Supported
    /// - **0b0011** - Busy
    /// - **0b0100** - FailedF
    /// - **0b0101** - Aborted
    /// - **0b0110** - Invalid Data
    pub set_ras_capabilities_status: u32,
}
impl RASFCommunicationChannelSMR {
    /// Start of the parameter blocks, the structure of which is shown in the Parameter Block Structure for PATROL_SCRUB.
    /// 
    /// These parameter blocks are used as communication mailbox between the OSPM and the platform, and there is 1 parameter block for each RAS feature.
    /// 
    /// NOTE: There can be only on parameter block per type.
    pub const fn parameter_blocks(&self) -> ! {
        todo!()
    }
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## ACPI RAS Feature Table (RASF)
pub struct RASF {
    /// - **Signature** - "RASF"
    pub header: SDTHeader,
    /// Identifier of the RASF Platform Communication Channel.
    ///
    /// OSPM should use this value to identify the PCC Sub channel structure in the RASF table
    pub rasf_platform_communication_channel_id: [u8; 12],
}
