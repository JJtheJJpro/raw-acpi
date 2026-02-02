use crate::SDTHeader;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Corrected Platform Error Polling Processor Structure
pub struct CorrectedPlatformErrorPollingProcessor {
    /// 0 - Corrected Platform Error Polling Processor structure for APIC/SAPIC based processors
    pub r#type: u8,
    /// 8
    pub length: u8,
    /// Processor ID of destination.
    pub processor_id: u8,
    /// Processor EID of destination.
    pub processor_eid: u8,
    /// Platform-suggested polling interval (in milliseconds).
    pub polling_interval: u32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Corrected Platform Error Polling Table (CPEP)
///
/// Platforms may contain the ability to detect and correct certain operational errors while maintaining platform function.
/// These errors may be logged by the platform for the purpose of retrieval. Depending on the underlying hardware support, the means for retrieving corrected platform error information varies.
/// If the platform hardware supports interrupt-based signaling of corrected platform errors, the MADT Platform Interrupt Source Structure describes the Corrected Platform Error Interrupt (CPEI).
/// Alternatively, OSPM may poll processors for corrected platform error information.
/// Error log information retrieved from a processor may contain information for all processors within an error reporting group.
/// As such, it may not be necessary for OSPM to poll all processors in the system to retrieve complete error information.
/// This optional table provides information that allows OSPM to poll only the processors necessary for a complete report of the platform’s corrected platform error information.
pub struct CorrectedPlatformErrorPolling {
    /// - **Signature** - "CPEP"
    pub header: SDTHeader,
    reserved: u64,
    /// A list of Corrected Platform Error Polling Processor structures for the platform.
    pub cpep_processor_structures: [CorrectedPlatformErrorPollingProcessor; 0],
}
impl CorrectedPlatformErrorPolling {
    pub const fn cpep_processor_structures(&self) -> &[CorrectedPlatformErrorPollingProcessor] {
        // SAFETY: I sure hope the OEM doesn't frick things up...
        unsafe {
            core::slice::from_raw_parts(
                (self as *const _ as *const u8).add(crate::SDT_HEADER_SIZE + 8)
                    as *const CorrectedPlatformErrorPollingProcessor,
                (self.header.length as usize - (crate::SDT_HEADER_SIZE + 8)) / core::mem::size_of::<CorrectedPlatformErrorPollingProcessor>(),
            )
        }
    }
}
