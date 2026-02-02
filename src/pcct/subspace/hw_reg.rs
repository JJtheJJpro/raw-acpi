use crate::{GenericAddressStructure, pcct::subspace::ReducedPCCSubspaceSMR};

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## HW Registers based Communications Subspace Structure (Type 5)
pub struct HWRegistersBasedCommunications {
    /// 5
    pub r#type: u8,
    /// Length (includes vendor defined area)
    /// 
    /// **JJ's Note: The vendor-defined area is just found after this defined struct.
    /// The base length is 96, but OEMs can add to this.**
    pub length: u8,
    /// 0x0001 (Version 1 of this PCC definition)
    pub version: u16,
    /// Base Address of the shared memory range.
    pub base_address: u64,
    /// Length of the shared memory range. If this length is zero then based address is ignored.
    pub shared_memory_range_length: u64,
    /// Contains the processor relative address, represented in Generic Address Structure format, of the PCC doorbell.
    /// 
    /// Note: Only System I/O space and System Memory space are valid for values for `address_space_id`.
    pub doorbell_register: GenericAddressStructure,
    /// Contains a mask of bits to preserve when writing the doorbell register.
    pub doorbell_preserve: u64,
    /// Contains a mask of bits to set when writing the doorbell register. This is used to send specific commands to the Platform.
    pub doorbell_write: u64,
    /// Contains the processor relative address, represented in Generic Address Structure format, of Command complete Check register.
    /// 
    /// Note: Only System I/O space and System Memory space are valid for values for `address_space_id`.
    pub command_complete_check_register: GenericAddressStructure,
    /// Contains a mask of bits to query completion status of the previously issued command from the Command Complete Status Register.
    /// 
    /// OS shall do an AND operation with the Command Complete Check Register value.
    /// The OS must check the completion status before writing to doorbell register for the next command.
    /// If calculated value is 0 then previous operation has been completed.
    /// If completion status is not implemented then this mask should be 0.
    /// In this case OS shall only wait for Minimum Request Turnaround Time.
    /// 
    /// Note: Command complete check needs be done before writing to doorbell register to avoid any race condition with a previous use of the doorbell register.
    /// In addition, command complete check needs to be done after writing to doorbell register and before reading Status from Error Status Register.
    pub command_complete_check_mask: u64,
    /// Contains the processor relative address, represented in Generic Address Structure format, of Error Status register.
    /// 
    /// Note: Only System I/O space and System Memory space are valid for values for `address_space_id`.
    pub error_status_register: GenericAddressStructure,
    /// Contains a mask of bits to get error status of the previous command request from Error Status Register.
    /// 
    /// OS shall do an AND operation with Error Status register value.
    /// If this mask value is 0 then Error Status register is ignored.
    /// Error Status needs to be checked after completion status indicates issued command has been completed.
    /// If Command Complete Check is not implemented (means Command Complete Check mask is 0) then wait for Minimum Request Turnaround Time.
    /// If the calculated value is zero, then it indicates success. Any other value indicates failure.
    pub error_status_mask: u64,
    /// Expected latency to process a command, in microseconds.
    pub nominal_latency: u32,
    /// The minimum amount of time that OSPM must wait after the completion of a command before issuing the next command, in microseconds.
    pub min_request_turnaround_time: u32,
}
impl HWRegistersBasedCommunications {
    pub const fn smr(&self) -> &ReducedPCCSubspaceSMR {
        unsafe { &*(self.base_address as *const ReducedPCCSubspaceSMR) }
    }
    /// **JJ's Note: This field is found in the ReducedPCCSubspaceSMR structure.<br>
    /// The SMR structure can't get the length without the subspace structure, hence why I'm giving the communication_subspace impl here instead.**
    pub const fn communication_subspace(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self.base_address + 4) as *const u8,
                self.shared_memory_range_length as usize - 4,
            )
        }
    }
}
