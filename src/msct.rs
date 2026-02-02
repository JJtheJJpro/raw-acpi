use crate::SDTHeader;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Maximum Proximity Domain Information Structure
///
/// The Maximum Proximity Domain Information Structure is used to report system maximum characteristics.
/// It is likely that these characteristics may be the same for many proximity domains, but they can vary from one proximity domain to another.
/// This structure optimizes to cover the former case, while allowing the flexibility for the latter as well.
/// These structures must be organized in ascending order of the proximity domain enumerations.
/// All proximity domains within the Maximum Number of Proximity Domains reported in the MSCT must be covered by one of these structures.
pub struct MaximumProximityDomainInformation {
    pub revision: u8,
    /// 22
    pub length: u8,
    /// The starting proximity domain for the proximity domain range that this structure is providing information.
    pub proximity_domain_range_low: u32,
    /// The ending proximity domain for the proximity domain range that this structure is providing information.
    pub proximity_domain_range_high: u32,
    /// The Maximum Processor Capacity of each of the Proximity Domains specified in the range.
    ///
    /// A value of 0 means that the proximity domains do not contain processors.
    /// This field must be >= the number of processor entries for the domain in the SRAT.
    pub max_processor_capacity: u32,
    /// The Maximum Memory Capacity (size in bytes) of the Proximity Domains specified in the range.
    ///
    /// A value of 0 means that the proximity domains do not contain memory.
    pub max_memory_capacity: u64,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Maximum System Characteristics Table (MSCT)
///
/// This section describes the format of the Maximum System Characteristic Table (MSCT), which provides OSPM with information characteristics of a system’s maximum topology capabilities.
/// If the system maximum topology is not known up front at boot time, then this table is not present.
/// OSPM will use information provided by the MSCT only when the System Resource Affinity Table (SRAT) exists. The MSCT must contain all proximity and clock domains defined in the SRAT.
pub struct MaximumSystemCharacteristicsTable {
    /// - **Signature** - "MSCT"
    pub header: SDTHeader,
    /// Offset in bytes to the Proximity Domain Information Structure table entry.
    pub offset_prox_dom_info: u32,
    /// Indicates the maximum number of Proximity Domains ever possible in the system.
    ///
    /// The number reported in this field is (maximum domains - 1). For example if there are 0x10000 possible domains in the system, this field would report 0xFFFF.
    pub max_number_of_proximity_domains: u32,
    /// Indicates the maximum number of Clock Domains ever possible in the system.
    ///
    /// The number reported in this field is (maximum domains - 1).
    pub max_number_of_clock_domains: u32,
    /// Indicates the maximum Physical Address ever possible in the system.
    ///
    /// Note: this is the top of the reachable physical address.
    pub max_physical_address: u64,
}
impl MaximumSystemCharacteristicsTable {
    /// **JJ's Note: The specs say that this field is located somewhere depending on the offset field in the structure (not aligned with the structure at all), which is why I provided this.**
    pub const fn proximity_domain_information(&self) -> &[MaximumProximityDomainInformation] {
        // SAFETY: I sure hope the OEM doesn't frick things up...
        unsafe {
            core::slice::from_raw_parts(
                (self as *const _ as *const u8).add(self.offset_prox_dom_info as usize)
                    as *const MaximumProximityDomainInformation,
                (self.header.length as usize)
                    .checked_sub(self.offset_prox_dom_info as usize)
                    .expect("INCORRECT MATH: MSCT impl --- slice reference parsing (offset beyond table length)")
                    / core::mem::size_of::<MaximumProximityDomainInformation>(),
            )
        }
    }
}
