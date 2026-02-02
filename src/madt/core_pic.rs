#[derive(Copy, Clone)]
/// ## CORE PIC Flags
pub struct COREPICFlags(u32);
impl COREPICFlags {
    /// If Physical Processor ID is invalid, OSPM shall ignore this field, and OSPM shall ignore Core Programmable Interrupt Controller Structure.<br>
    /// If Physical Processor ID is valid and if this Enabled bit is clear, this processor will be unusable on booting, and can be online during OS runtime.<br>
    /// If Physical Processor ID is valid and if this Enabled bit is set, this processor is ready for using.
    pub const fn enabled(&self) -> bool {
        self.0 & 0b1 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Core Programmable Interrupt Controller (CORE PIC) Structure
///
/// Each processor in Loongarch system has a Core Programmable Interrupt Controller record in the MADT, and a processor device object in the DSDT.
pub struct CoreProgrammableInterruptController {
    /// 17 - Core Programmable Interrupt Controller Structure
    pub r#type: u8,
    /// Length of the Core Programmable Interrupt Controller Structure in bytes.
    ///
    /// **JJ's Note: There doesn't seem to be any variable-sized fields in this struct.  The size is 15 bytes...**
    pub length: u8,
    /// - **0x00** - Invalid
    /// - **0x01** - CORE PIC v1
    ///
    /// Other values are reserved.
    pub version: u8,
    /// The OS associates this CORE PIC Structure with a processor device object in the namespace
    /// when the _UID child object of the processor device evaluates to a numeric value that matches the numeric value in this field.
    pub acpi_processor_id: u32,
    /// The processor core physical id.
    ///
    /// 0xFFFFFFFF is invalid value.
    ///
    /// If invalid, this processor is unusable, and OSPM shall ignore Core Interrupt Controller Structure.
    pub phsyical_processor_id: u32,
    /// CORE PIC flags.
    pub flags: COREPICFlags,
}