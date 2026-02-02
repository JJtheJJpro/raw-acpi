#[derive(Copy, Clone)]
#[repr(C, align(4))]
/// ## Multiprocessor Wakeup Mailbox Structure
pub struct MultiprocessorWakeupMailbox {
    /// - **0x0000** - Noop (no operation).
    /// - **0x0001** - Wakeup (jump to the wakeup vector).
    ///
    /// All other values are reserved.
    pub command: u16,
    reserved: u16,
    /// The processor's local APIC ID.
    ///
    /// The application processor shall check if the ApicId field matches its own APIC ID.
    /// The application processor shall ignore the command in case of APIC ID mismatch.
    pub apic_id: u32,
    /// The wakeup address for application processor(s).
    ///
    /// For Intel processors, the execution environment is:
    /// - Interrupts must be disabled.
    /// - RFLAGES.IF set to 0.
    /// - Long mode enabled.
    /// - Paging mode is enabled and physical memory for waking vector is identity mapped (virtual address equals physical address)
    /// - Waking vector must be contained within one physical page.
    /// - Selectors are set to flat and otherwise not used.
    pub wakeup_vector: u64,
    /// Reserved for OS use.
    pub reserved_os: [u8; 2032],
    /// Reserved for firmware use.
    pub reserved_firmware: [u8; 2048],
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Multiprocessor Wakeup Structure
///
/// The platform firmware publishes a multiprocessor wakeup structure to let the bootstrap processor wake up application processors with a mailbox.
/// The mailbox is memory that the firmware reserves so that each processor can have the OS send a message to them.
///
/// During system boot, the firmware puts the application processors in a state to check the mailbox.
/// The shared mailbox is a 4K-aligned 4K-size memory block allocated by the firmware in the ACPINvs memory.
/// The firmware is not allowed to modify the mailbox location when the firmware transfer the control to an OS loader.
/// The mailbox is broken down into two 2KB sections: an OS section and a firmware section.
///
/// The OS section can only be written by OS and read by the firmware, except the command field.
/// The application processor need clear the command to Noop(0) as the acknowledgement that the command is received.
/// The firmware must cache the content in the mailbox which might be used later before clear the command such as WakeupVector.
/// Only after the command is changed to Noop(0), the OS can send the next command.
/// The firmware section must be considered read-only to the OS and is only to be written to by the firmware.
/// All data communication between the OS and FW must be in little endian format.
///
/// The OS section contains command, flags, APIC ID, and a wakeup address.
/// After the OS detects the processor number from the MADT table, the OS may prepare the wakeup routine,
/// fill the wakeup address field in the mailbox, indicate which processor need to be wakeup in the APID ID field, and send wakeup command.
/// Once an application processor detects the wakeup command and its own APIC ID, the application processor will jump to the OS-provided wakeup address.
/// The application processor will ignore the command if the APIC ID does not match its own.
///
/// For each application processor, the mailbox can be used only once for the wakeup command.
/// After the application process takes the action according to the command, this mailbox will no longer be checked by this application processor.
/// Other processors can continue using the mailbox for the next command.
pub struct MultiprocessorWakeup {
    /// 16 - Multiprocessor Wakeup Structure
    pub r#type: u8,
    /// 16
    pub length: u8,
    /// Version of the mailbox.
    pub mailbox_version: u16,
    reserved: u32,
    /// Physical address of the mailbox.
    pub mailbox_address: u64,
}
impl MultiprocessorWakeup {
    /// JJ here, just so 32- and 64-bit CPUs don't get confused, the mailbox_address field must stay 64-bit.
    /// This method is here for ease of use in runtime.
    pub const fn mailbox_address(&self) -> *const MultiprocessorWakeupMailbox {
        self.mailbox_address as *const _
    }
}