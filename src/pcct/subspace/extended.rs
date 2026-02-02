use crate::{GenericAddressStructure, pcct::subspace::MasterSlaveCommunicationsChannelSMR};

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Extended PCC subspaces (types 3 and 4)
/// 
/// Extended PCC communication subspaces are of two types:
/// 
/// - Type 3 Master subspace: used by the OSPM to communicate with the platform.
/// - Type 4 Slave subspace: Used by the platform to send asynchronous notifications to the OSPM.
/// 
/// Master subspaces are not substantially different to type 0, 1, or 2 subspaces, the most notable difference is that a type 3 master subspace does not support asynchronous notifications.
/// Slave subspaces, type 4, provide those notifications, and cannot be used by the OSPM to send messages to the platform. Together a master and slave pair create a bidirectional interface between the OSPM and the platform.
pub struct ExtendedPCC {
    /// - **3** - Master subspace
    /// - **4** - Slave subspace
    pub r#type: u8,
    /// 164
    pub length: u8,
    /// GSIV of an interrupt triggered by the platform:
    /// - For master subspaces (type 3) this is raised when a command is completed on this subspace.
    /// - For slave subspaces (type 4) this is raised when platform sends a notification.
    /// 
    /// For a master subspace, this field is ignored if the platform interrupt flag in Table 14.2 is set to zero.<br>
    /// If a slave-subspace is present in the PCCT, then the platform interrupt flag must be set to 1.
    /// 
    /// Note that if interrupts are edge triggered, then each subspace must have its own unique interrupt.<br>
    /// If interrupts are level, a GSIV may be shared by multiple subspaces, but each one must have unique Platform interrupt Ack preserve and Ack Set masks.
    pub platform_interrupt: u32,
    /// - **Bit 0** - Platform interrupt polarity
    ///   - 1: Interrupt is Active low
    ///   - 0: Interrupt is Active high
    /// - **Bit 1** - Platform interrupt mode
    ///   - 1: Interrupt is Edge triggered
    ///   - 0: Interrupt is Level triggered
    /// 
    /// The rest of the bits are reserved.
    pub platform_interrupt_flags: u8,
    reserved0: u8,
    /// Base Address of the shared memory range.
    pub base_address: u64,
    /// Length of the memory range. Must be >= 16.
    pub memory_length: u32,
    /// Contains the processor relative address, represented in Generic Address Structure (GAS) format, of the PCC doorbell.
    /// 
    /// Note: Only the System I/O, System Memory, and Functional Fixed Hardware spaces are valid values for `address_space_id`.
    /// 
    /// For slave subspaces this field is optional, if not present the field should just contain zeros.
    pub doorbell_register: GenericAddressStructure,
    /// Contains a mask of bits to preserve when writing the doorbell register.
    pub doorbell_preserve: u64,
    /// Contains a mask of bits to set when writing the doorbell register.
    pub doorbell_write: u64,
    /// Expected latency to process a command, in microseconds.
    /// 
    /// This field is only relevant for master subspaces.
    pub nominal_latency: u32,
    /// The maximum number of periodic requests that the subspace subspace can support, reported in commands per minute.
    /// 
    /// 0 indicates no limitation.
    /// 
    /// This field is only relevant for master subspaces.
    pub max_periodic_access_rate: u32,
    /// The minimum amount of time that OSPM must wait after the completion of a command before issuing the next command, in microseconds.
    /// 
    /// This field is only relevant for master subspaces.
    pub min_request_turnaround_time: u32,
    /// Contains the processor relative address, represented in Generic Address Structure (GAS) format, of the platform interrupt acknowledge register.
    /// 
    /// Note: Only the System I/O, System Memory, and Functional Fixed Hardware spaces are valid for values for `address_space_id`.
    /// 
    /// If the subspace does not support interrupts or the interrupt is edge driven the register may be omitted.
    /// A value of 0 on all 12 bytes of the GAS structure indicates the register is not present.
    /// If the subspace does support interrupts, and these are level, this register must be supplied and is used to clear the interrupt by using a read, modify, write sequence.
    pub platform_interrupt_ack_register: GenericAddressStructure,
    /// Contains a mask of bits to preserve when writing the platform interrupt ack register.
    pub platform_interrupt_ack_preserve: u64,
    /// Contains a mask of bits to set when writing the platform interrupt ack register.
    pub platform_interrupt_ack_set: u64,
    reserved1: u64,
    /// Contains the processor relative address, represented in Generic Address Structure (GAS) format, of the Command complete check register.
    /// 
    /// Note: Only the System I/O, System Memory, and Functional Fixed Hardware spaces are valid for values for `address_space_id`.
    pub command_complete_check_register_address: GenericAddressStructure,
    /// Mask to determine whether a command is complete, using the command complete check register.
    /// 
    /// A command is complete if the value of the register when combined through a logical AND with this mask, yields a non-zero value.
    pub command_complete_check_mask: u64,
    /// Contains the processor relative address, represented in Generic Address Structure (GAS) format, of the command complete update register.
    /// 
    /// Note: Only the System I/O, System Memory, and Functional Fixed Hardware spaces are valid for values for `address_space_id`.
    pub command_complete_update_register: GenericAddressStructure,
    /// Mask of bits to preserve in the command complete update register, when updating command complete in this subspace.
    pub command_complete_update_preserve_mask: u64,
    /// Mask of bits to set in the command complete update register, when updating command complete in this subspace.
    /// 
    /// For master subspaces the mask must indicate how to clear the command complete bit. For slave subspaces, the mask must indicate how to set the command complete bit.
    pub command_complete_update_set_mask: u64,
    /// Contains the processor relative address, represented in Generic Address Structure (GAS) format, of the Error status register. This field is ignored by the OSPM on slave channels.
    /// 
    /// Note: Only the System I/O, System Memory, and Functional Fixed Hardware spaces are valid for values for `address_space_id`.
    /// 
    /// Note: this register can be the same as the command complete check register.
    pub error_status_register: GenericAddressStructure,
    /// The mask contained here can be combined through a logical AND with content of the Error status register to ascertain whether an error occurred in the transmission of the command through the subspace.
    /// The logical NOT of this mask is be used to clear the error.
    /// The inverted mask is combined through a logical AND with the content of the Error status register, and the result is written back into said register. This field is ignored for slave channels.
    pub error_status_mask: u64,
}
impl ExtendedPCC {
    pub const fn smr(&self) -> &MasterSlaveCommunicationsChannelSMR {
        unsafe { &*(self.base_address as *const MasterSlaveCommunicationsChannelSMR) }
    }
    /// **JJ's Note: This field is found in the MasterSlaveCommunicationsChannelSMR structure.<br>
    /// The SMR structure can't get the length without the subspace structure, hence why I'm giving the communication_subspace impl here instead.**
    pub const fn communication_subspace(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self.base_address + 16) as *const u8,
                self.memory_length as usize - 16,
            )
        }
    }
}
