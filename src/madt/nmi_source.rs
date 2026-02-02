use crate::madt::interrupt_source_override::MPSINTIFlags;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Non-Maskable Interrupt (NMI) Source Structure
///
/// This structure allows a platform designer to specify which I/O (S)APIC interrupt inputs should be enabled as non-maskable.
/// Any source that is non-maskable will not be available for use by devices.
pub struct NMISource {
    /// 3 - NMI Source
    pub r#type: u8,
    /// 8
    pub length: u8,
    /// Same as MPS INTI flags.
    pub flags: MPSINTIFlags,
    /// The Global System Interrupt that this NMI will signal.
    pub global_system_interrupt: u32,
}