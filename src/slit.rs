use crate::{SDT_HEADER_SIZE, SDTHeader};

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## System Locality Information Table (SLIT)
///
/// This optional table provides a matrix that describes the relative distance (memory latency) between all System Localities, which are also referred to as Proximity Domains.
/// Systems employing a Non Uniform Memory Access (NUMA) architecture contain collections of hardware resources including for example, processors, memory, and I/O buses, that comprise what is known as a "NUMA node".
/// Processor accesses to memory or I/O resources within the local NUMA node is generally faster than processor accesses to memory or I/O resources outside of the local NUMA node.
///
/// The value of each Entry[i,j] in the SLIT table, where i represents a row of a matrix and j represents a column of a matrix,
/// indicates the relative distances from System Locality / Proximity Domain i to every other System Locality j in the system (including itself).
///
/// The i,j row and column values correlate to Proximity Domain values in the System Resource Affinity Table (SRAT), and to values returned by _PXM objects in the ACPI namespace.
///
/// The entry value is a one-byte unsigned integer. The relative distance from System Locality i to System Locality j is the i*N + j entry in the matrix, where N is the number of System Localities.
/// Except for the relative distance from a System Locality to itself, each relative distance is stored twice in the matrix.
/// This provides the capability to describe the scenario where the relative distances for the two directions between System Localities is different.
///
/// The diagonal elements of the matrix, the relative distances from a System Locality to itself are normalized to a value of 10. The relative distances for the non-diagonal elements are scaled to be relative to 10.
/// For example, if the relative distance from System Locality i to System Locality j is 2.4, a value of 24 is stored in table entry i*N+ j and in j*N+ i, where N is the number of System Localities.
///
/// If one locality is unreachable from another, a value of 255 (0xFF) is stored in that table entry. Distance values of 0-9 are reserved and have no meaning.
/// 
/// **JJ's Note: Even though the entry field in specs say it's a 2-D array, having any kernel use it as a 2-D array destroys performance.<br>
/// The best way to use the entry field is to use it as a flat slice.  This is the reason why you don't see the entry field in the struct itself.**
pub struct SystemLocalityInformationTable {
    /// - **Signature** - "SLIT"
    pub header: SDTHeader,
    /// Indicates the number of System Localities in the system.
    pub system_localities_num: u64,
}
impl SystemLocalityInformationTable {
    pub const fn entries_flat(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self as *const _ as *const u8).add(SDT_HEADER_SIZE + 8),
                self.system_localities_num as usize * self.system_localities_num as usize,
            )
        }
    }
}
