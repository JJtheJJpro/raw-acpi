pub mod extended;
pub mod generic;
pub mod hw_reduced;
pub mod hw_reg;

// JJ here, I most likely will make this into an enum of references, but just so alignment doesn't get hurt, I will not include this.
/*
#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Platform Communications Channel Subspace Structures
///
/// PCC Subspaces are described by the PCC Subspace structure in the PCCT table.
/// The subspace ID of a PCC subspace is its index in the array of subspace structures, starting with subspace 0.
pub struct PCCSubspace {
    /// The type of subspace.
    pub r#type: u8,
    /// Length of the subspace structure, in bytes.
    ///
    /// The next subspace structure begins length bytes after the start of this one.
    pub length: u8,
    /// See specific subspace types for more details.
    pub type_specific_fields: [u8; 0],
}
*/

#[derive(Copy, Clone)]
/// ## Generic Communications Channel Command Field
/// 
/// For channels of type 0 to 2, this 16-bit field is used to select one of the defined commands for the platform to perform.
/// OSPM is responsible for populating this field before each command invocation.
pub struct GenericCommunicationsChannelCommandField(u16);
impl GenericCommunicationsChannelCommandField {
    /// Command code to execute.
    /// 
    /// Command codes are application specific and defined by the consumer of this interface.
    pub const fn command(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }
    /// If set, the platform should generate a Doorbell interrupt at the completion of this command.
    /// The interrupt is an SCI for a Type 0 subspace structure, or as described by the Doorbell Interrupt field for Type 1 and Type 2 subspace structures.
    /// If the Doorbell bit is not set in the PCC global flags, this bit must be cleared.
    pub const fn notify_on_completion(&self) -> bool {
        self.0 & 0x8000 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement them.
}

#[derive(Copy, Clone)]
/// ## Generic Communications Channel Status Field
pub struct GenericCommunicationsChannelStatusField(u16);
impl GenericCommunicationsChannelStatusField {
    /// If set, the platform has completed processing the last command.
    pub const fn command_complete(&self) -> bool {
        self.0 & 0b0001 != 0
    }
    /// If set, the platform has issued a Platform Interrupt to this subspace.
    /// 
    /// OSPM must check the Command Complete and Platform Notification fields to determine the cause of the Interrupt.
    pub const fn platform_interrupt(&self) -> bool {
        self.0 & 0b0010 != 0
    }
    /// If set, an error occurred executing the last command.
    pub const fn error(&self) -> bool {
        self.0 & 0b0100 != 0
    }
    /// If set, indicates the platform is issuing an asynchronous notification to OSPM.
    pub const fn platform_notification(&self) -> bool {
        self.0 & 0b1000 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement them.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Generic Communications Channel Shared Memory Region
pub struct GenericCommunicationsChannelSMR {
    /// The PCC signature.
    /// 
    /// The signature of a subspace is computed by a bitwise-or of the value 0x50434300 with the subspace ID. For example, subspace 3 has the signature 0x50434303.
    pub signature: u32,
    /// PCC command field.
    pub command: GenericCommunicationsChannelCommandField,
    /// PCC status field.
    pub status: GenericCommunicationsChannelStatusField,
    /// Memory region for reading/writing PCC data.
    /// 
    /// The size of this region is 8 bytes smaller than the size of the shared memory region (specified in the Generic Communications Subspace structure).
    /// 
    /// The first byte of this field represents PCC address 0.
    pub communication_subspace: [u8; 0],
}

#[derive(Copy, Clone)]
/// ## Master Slave Communications Channel Flags
pub struct MasterSlaveCommunicationsChannelFlags(u32);
impl MasterSlaveCommunicationsChannelFlags {
    /// For master subspaces this field indicates to the platform that it must generate an interrupt when the command has completed.
    /// - Setting this bit to 1 when sending a command, requests that completion of the command is signaled via the platform interrupt.
    /// - Setting it to 0 when sending a command, requests that no interrupt is asserted when the command is completed.
    /// 
    /// For slave subspaces, if the doorbell field of the slave subspace is non zero, and this flag is set, the OSPM must access the doorbell once it has processed the notification.
    /// This bit is ignored by the platform if the Platform Interrupt field of the PCC flags (Platform Communications Channel Global Flags) is set to zero.
    pub const fn notify_on_completion(&self) -> bool {
        self.0 & 0b1 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement them.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Extended PCC Subspace Shared Memory Region
/// 
/// **JJ's Note: Same thing as Master Slave Communications Channel Shared Memory Region.**
pub struct MasterSlaveCommunicationsChannelSMR {
    /// The PCC signature.
    /// 
    /// The signature of a subspace is computed by a bitwise-or of the value 0x50434300 with the subspace ID.
    /// For example, subspace 3 has the signature 0x50434303.
    pub signature: u32,
    /// Master Slave Communications Channel Flags.
    pub flags: MasterSlaveCommunicationsChannelFlags,
    /// Length of payload being transmitted including command field.
    pub length: u32,
    /// Command being sent over the subspace.
    pub command: u32,
    /// Memory region for reading/writing PCC data.
    /// 
    /// The maximum size of this region is 16 bytes smaller than the size of the shared memory region (specified in the Master Slave Communications Subspace structure).
    /// When a command is sent to or received from the platform, the size of the data in this space will be Length (expressed above) minus the 4 bytes taken up by the command.
    pub communication_subspace: [u8; 0]
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Reduced PCC Subspace Shared Memory Region
pub struct ReducedPCCSubspaceSMR {
    /// The PCC signature.
    /// 
    /// The signature of a subspace is computed by a bitwise-or of the value 0x50434300 with the subspace ID.
    /// For example, subspace 3 has the signature 0x50434303.
    pub signature: u32,
    /// Memory region for reading/writing PCC data.
    /// 
    /// The maximum size of this region is 4 bytes smaller than the size of the shared memory region (specified in the Type 5 PCC Subspace structure).
    /// When a command is sent to or received from the platform, the size of the data in this space will be Length (expressed above) minus the 4 bytes taken up by the Signature.
    pub communication_subspace: [u8; 0]
}
