pub mod subspace;

use crate::SDTHeader;

#[derive(Copy, Clone)]
pub struct PCCGlobalFlags(u32);
impl PCCGlobalFlags {
    /// If set, the platform is capable of generating an interrupt to indicate completion of a command.
    pub const fn platform_interrupt(&self) -> bool {
        self.0 & 0b1 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Platform Communications Channel (PCC)
///
/// The platform communication channel (PCC) is a generic mechanism for OSPM to communicate with an entity in the platform (e.g. a platform controller, or a Baseboard Management Controller (BMC)).
/// Neither the entity that OSPM communicates with, nor any aspects of the information passed back and forth is defined in this section.
/// That information is defined by the actual interface that that employs PCC register address space as the communication channel.
///
/// PCC defines a new address space type (PCC Space, 0xA), which is implemented as one or more independent communications channels, or subspaces.
pub struct PlatformCommunicationsChannel {
    /// - **Signature** - "PCCT"
    pub header: SDTHeader,
    /// Platform Communications Channel Global flags.
    pub flags: PCCGlobalFlags,
    reserved: u64,
    /// A list of Platform Communications Channel Subspace structures for this platform. At most 256 subspaces are supported.
    pub pcc_subspace_structure: [u8; 0],
}
