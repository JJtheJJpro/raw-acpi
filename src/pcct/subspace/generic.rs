use crate::{GenericAddressStructure, pcct::subspace::GenericCommunicationsChannelSMR};

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Generic Communications Subspace Structure (type 0)
pub struct GenericCommunications {
    /// 0 - Generic Communications Subspace
    pub r#type: u8,
    /// 62
    pub length: u8,
    reserved: [u8; 6],
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
impl GenericCommunications {
    pub const fn smr(&self) -> &GenericCommunicationsChannelSMR {
        unsafe { &*(self.base_address as *const GenericCommunicationsChannelSMR) }
    }
    /// **JJ's Note: This field is found in the GenericCommunicationsChannelSMR structure.<br>
    /// The SMR structure can't get the length without the subspace structure, hence why I'm giving the communication_subspace impl here instead.**
    pub const fn comm_space(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self.base_address + 8) as *const u8,
                self.memory_length as usize - 8,
            )
        }
    }
}
