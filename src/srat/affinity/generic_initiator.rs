use crate::srat::{DeviceHandle, affinity::GenericInitiatorPortAffinityFlags};

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Generic Initiator Affinity Structure
///
/// The Generic Initiator Affinity Structure provides the association between a generic initiator and the proximity domain to which the initiator belongs.
///
/// Support of Generic Initiator Affinity Structures by OSPM is optional, and the platform may query whether the OS supports it via the _OSC method.
pub struct GenericInitiatorAffinity {
    /// 5 - Generic Initiator Structure
    pub r#type: u8,
    /// 32
    pub length: u8,
    reserved0: u8,
    /// Device Handle Type:
    ///
    /// - **0** - ACPI Device Handle
    /// - **1** - PCI Device Handle
    ///
    /// The rest of the values are reserved.
    pub device_handle_type: u8,
    /// The proximity domain to which the generic initiator belongs.
    pub proximity_domain: u32,
    /// Device Handle of the Generic Initiator
    pub device_handle: DeviceHandle,
    /// Flags - Generic Initiator/Port Affinity Structure.
    pub flags: GenericInitiatorPortAffinityFlags,
    reserved1: u32,
}