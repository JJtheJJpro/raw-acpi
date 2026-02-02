use crate::{GenericAddressStructure, pcct::subspace::ReducedPCCSubspaceSMR};

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## HW-Reduced Communications Subspace Structure (type 1)
/// 
/// This is intended for use on HW-Reduced ACPI Platforms, which do not support the SCI.
/// Aside from the interrupt change, and the allowed use of the Functional Fixed HW address space for the Doorbell Register,
/// this subspace is identical to the Generic Communications Subspace.
pub struct HWReducedCommunicationsType1 {
    /// 1 - HW-Reduced Communications Subspace
    pub r#type: u8,
    /// 62
    pub length: u8,
    /// GSIV of the interrupt used for the PCC platform interrupt for this Subspace.
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
    reserved: u8,
    /// Base Address of the shared memory range.
    pub base_address: u64,
    /// Length of the memory range. Must be > 8.
    pub memory_length: u64,
    /// Contains the processor relative address, represented in Generic Address Structure format, of the PCC doorbell.
    /// 
    /// Note: Only System I/O space and System Memory space are valid for values for `address_space_id`.
    pub doorbell_register: GenericAddressStructure,
    /// Contains a mask of bits to preserve when writing the doorbell register.
    pub doorbell_preserve: u64,
    /// Contains a mask of bits to set when writing the doorbell register.
    pub doorbell_write: u64,
    /// Expected latency to process a command, in microseconds.
    pub nominal_latency: u32,
    /// The maximum number of periodic requests that the subspace channel can support, reported in commands per minute.
    /// 
    /// 0 indicates no limitation.
    pub max_periodic_access_rate: u32,
    /// The minimum amount of time that OSPM must wait after the completion of a command before issuing the next command, in microseconds.
    pub min_request_turnaround_time: u16,
}
impl HWReducedCommunicationsType1 {
    pub const fn smr(&self) -> &ReducedPCCSubspaceSMR {
        unsafe { &*(self.base_address as *const ReducedPCCSubspaceSMR) }
    }
    /// **JJ's Note: This field is found in the ReducedPCCSubspaceSMR structure.<br>
    /// The SMR structure can't get the length without the subspace structure, hence why I'm giving the communication_subspace impl here instead.**
    pub const fn communication_subspace(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self.base_address + 4) as *const u8,
                self.memory_length as usize - 4,
            )
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## HW-Reduced Communications Subspace Structure (type 2)
/// 
/// This is intended for use on HW-Reduced ACPI Platforms, which require read-modify-write sequence to acknowledge Platform Interrupt.
/// Aside from three Platform Ack fields, this subspace is identical to HW-Reduced Communications Subspace Structure (type 1).
pub struct HWReducedCommunicationsType2 {
    /// 2 - HW-Reduced Communications Subspace
    pub r#type: u8,
    /// 90
    pub length: u8,
    /// GSIV of the interrupt used for the PCC platform interrupt for this Subspace.
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
    reserved: u8,
    /// Base Address of the shared memory range.
    pub base_address: u64,
    /// Length of the memory range. Must be > 8.
    pub memory_length: u64,
    /// Contains the processor relative address, represented in Generic Address Structure format, of the PCC doorbell.
    /// 
    /// Note: Only System I/O space and System Memory space are valid for values for `address_space_id`.
    pub doorbell_register: GenericAddressStructure,
    /// Contains a mask of bits to preserve when writing the doorbell register.
    pub doorbell_preserve: u64,
    /// Contains a mask of bits to set when writing the doorbell register.
    pub doorbell_write: u64,
    /// Expected latency to process a command, in microseconds.
    pub nominal_latency: u32,
    /// The maximum number of periodic requests that the subspace channel can support, reported in commands per minute.
    /// 
    /// 0 indicates no limitation.
    pub max_periodic_access_rate: u32,
    /// The minimum amount of time that OSPM must wait after the completion of a command before issuing the next command, in microseconds.
    pub min_request_turnaround_time: u16,
    /// Contains the processor relative address, represented in Generic Address Structure format, of the platform interrupt ack register.
    /// 
    /// Note: Only the System I/O, System Memory, and Functional Fixed Hardware spaces are valid for values for `address_space_id`.
    pub platform_interrupt_ack_register: GenericAddressStructure,
    /// Contains a mask of bits to preserve when writing the platform interrupt ack register.
    pub platform_interrupt_ack_preserve: u64,
    /// Contains a mask of bits to set when writing the platform interrupt ack register.
    pub platform_interrupt_ack_write: u64,
}
impl HWReducedCommunicationsType2 {
    pub const fn smr(&self) -> &ReducedPCCSubspaceSMR {
        unsafe { &*(self.base_address as *const ReducedPCCSubspaceSMR) }
    }
    /// **JJ's Note: This field is found in the ReducedPCCSubspaceSMR structure.<br>
    /// The SMR structure can't get the length without the subspace structure, hence why I'm giving the communication_subspace impl here instead.**
    pub const fn communication_subspace(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self.base_address + 4) as *const u8,
                self.memory_length as usize - 4,
            )
        }
    }
}
