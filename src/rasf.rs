use crate::{
    pcct::subspace::{
        GenericCommunicationsChannelCommandField, GenericCommunicationsChannelStatusField,
    },
    SDTHeader,
};

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Parameter Block
///
/// The following table describes the Parameter Blocks.
///
/// The structure is used to pass parameters for controlling the corresponding RAS Feature.
///
/// Each RAS Feature is assigned a TYPE number, which is the bit index into the RAS capabilities bitmap.
pub struct ParameterBlock {
    /// 0x0000 - Patrol scrub
    pub r#type: u16,
    /// - **Byte 0** - Minor Version
    /// - **Byte 1** - Major Version
    pub version: u16,
    /// Length in bytes of the entire parameter block structure
    pub length: u16,
    /// - **0x01** - GET_PATROL_PARAMETERS
    /// - **0x02** - START_PATROL_SCRUBBER
    /// - **0x03** - STOP_PATROL_SCRUBBER
    pub patrol_scrub_command: u16,
    /// OSPM Specifies the base of the address range to be patrol scrubbed.
    ///
    /// OSPM sets this parameter for the following commands: GET_PATROL_PARAMETERS and START_PATROL_SCRUBBER
    pub requested_address_start: u64,
    /// OSPM Specifies the size of the address range to be patrol scrubbed.
    ///
    /// OSPM sets this parameter for the following commands: GET_PATROL_PARAMETERS and START_PATROL_SCRUBBER
    pub requested_address_size: u64,
    /// The platform returns this value in response to GET_PATROL_PARAMETERS.
    ///
    /// The platform calculates the nearest patrol scrub boundary address from where it can start.
    /// This range should be a superset of the Requested Address Range. Base of the address
    pub actual_address_start: u64,
    /// The platform returns this value in response to GET_PATROL_PARAMETERS.
    ///
    /// The platform calculates the nearest patrol scrub boundary address from where it can start.
    /// This range should be a superset of the Requested Address Range. Size of the address
    pub actual_address_size: u64,
    /// The platform returns this value in response to GET_PATROL_PARAMETERS:
    /// - **Bit [[0]]** - Will be set if patrol scrubber is already running for address range specified in "Actual Address Range"
    /// - **Bits [[3:1]]** - Current Patrol Speeds, if Bit [[0]] is set:
    ///     - **000b** - Slow
    ///     - **100b** - Medium
    ///     - **111b** - Fast
    ///     - All other combinations are reserved.
    /// - **Bits [[15:4]]** - RESERVED
    pub flags: u16,
    /// The OSPM Sets this field as follows, for the START_PATROL_SCRUBBER command:
    /// - **Bit [[0]]** - Will be set if patrol scrubber is already running for address range specified in "Actual Address Range"
    /// - **Bits [[2:0]]** - Requested Patrol Speeds
    ///     - **000b** - Slow
    ///     - **100b** - Medium
    ///     - **111b** - Fast
    ///     - All other combinations are reserved.
    /// - **Bits [[7:3]]** - RESERVED
    pub requested_speed: u8,
}

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
    pub const fn parameter_blocks(&self) -> &[ParameterBlock] {
        // SAFETY: I sure hope the OEM doesn't frick things up...
        unsafe {
            core::slice::from_raw_parts(
                (self as *const _ as *const u8).add(size_of::<RASFCommunicationChannelSMR>())
                    as *const ParameterBlock,
                (self.rasf_parameter_block_num as usize - size_of::<RASFCommunicationChannelSMR>())
                    / size_of::<ParameterBlock>(),
            )
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## ACPI RAS Feature Table (RASF)
pub struct RASFeatureTable {
    /// - **Signature** - "RASF"
    pub header: SDTHeader,
    /// Identifier of the RASF Platform Communication Channel.
    ///
    /// OSPM should use this value to identify the PCC Sub channel structure in the RASF table
    pub rasf_platform_communication_channel_id: [u8; 12],
}
