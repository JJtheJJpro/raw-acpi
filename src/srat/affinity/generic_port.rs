use crate::srat::{DeviceHandle, affinity::GenericInitiatorPortAffinityFlags};

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Generic Port Affinity Structure
///
/// The Generic Port Affinity Structure provides an association between a proximity domain number and a device handle representing
/// a Generic Port (e.g. CXL Host Bridge, or similar device that hosts a dynamic topology of memory ranges and/or initiators).
///
/// Support of Generic Port Affinity Structures by an OSPM is optional.
pub struct GenericPortAffinity {
    /// 6 - Generic Port Structure
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
    /// The proximity domain to identify the performance of this port in the HMAT.
    pub proximity_domain: u32,
    /// Device Handle of the Generic Initiator.
    pub device_handle: DeviceHandle,
    /// Flags - Generic Initiator/Port Affinity Structure.
    pub flags: GenericInitiatorPortAffinityFlags,
    reserved1: u32,
}