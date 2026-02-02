#[derive(Copy, Clone)]
/// ## Firmware Control Structure Feature Flags
pub struct FACSFlags(u32);
impl FACSFlags {
    /// Indicates whether the platform supports S4BIOS_REQ.
    ///
    /// If S4BIOS_REQ is not supported, OSPM must be able to save and restore the memory state in order to use the S4 state.
    pub const fn s4bios_f(&self) -> bool {
        self.0 & 0b01 != 0
    }
    /// Indicates that the platform firmware supports a 64 bit execution environment for the waking vector.
    ///
    /// When set and the OSPM additionally set 64BIT_WAKE_F, the platform firmware will create a 64 bit execution environment before transferring control to the X_Firmware_Waking_Vector.
    pub const fn _64bit_wake_supported_f(&self) -> bool {
        self.0 & 0b10 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
/// ## OSPM Enabled Firmware Control Structure Feature Flags
pub struct OEFACSFlags(u32);
impl OEFACSFlags {
    /// OSPM sets this bit to indicate to platform firmware that the X_Firmware_Waking_Vector requires a 64 bit execution environment.
    ///
    /// This flag can only be set if platform firmware sets 64BIT_WAKE_SUPPORTED_F in the FACS flags field.
    /// This bit field has no affect on ItaniumTM Processor Family (IPF) -based platforms, which require a 64 bit execution environment.
    pub const fn _64bit_wake_f(&self) -> bool {
        self.0 & 0b1 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
/// ## Global Lock Structure within the FACS
///
/// **JJ's Note: As of right now, this is unused.  Once I'm done with the ACPI stuff, I'll learn more about this.**
pub struct FACSGlobalLock(u32);
impl FACSGlobalLock {
    /// Non-zero indicates that a request for ownership of the Global Lock is pending.
    pub const fn pending(&self) -> bool {
        self.0 & 0b01 != 0
    }
    /// Non-zero indicates that the Global Lock is Owned.
    pub const fn owned(&self) -> bool {
        self.0 & 0b10 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// The Firmware ACPI Control Structure (FACS) is a structure in read/write memory that the platform boot firmware reserves for ACPI usage.
///
/// This structure is optional if and only if the HARDWARE_REDUCED_ACPI flag in the FADT is set. The FACS is passed to an ACPI-compatible OS using the FADT.
///
/// The platform boot firmware aligns the FACS on a 64-byte boundary anywhere within the system's memory address space.
/// The memory where the FACS structure resides must not be reported as system AddressRangeMemory in the system address map.
/// For example, the E820 address map reporting interface would report the region as AddressRangeReserved.
///
/// **JJ's Note: Due of the nature of the structure, this struct pertaining to this library will not be using the SDTHeader struct.**
pub struct FirmwareACPIControl {
    /// "FACS"
    pub signature: [u8; 4],
    /// Length, in bytes, of the entire Firmware ACPI Control Structure. This value is 64 bytes or larger.
    pub length: u32,
    /// The value of the system's "hardware signature at current boot." The only thing that determines the hardware signature is the ACPI tables.
    /// If any content or structure of the ACPI tables has changed, including adding or removing of tables, then the hardware signature must change.
    pub hardware_signature: [u8; 4],
    /// The 32-bit address field where OSPM puts its waking vector.
    ///
    /// This field is superseded by the `x_firmware_waking_vector` field.
    ///
    /// Before transitioning the system into a global sleeping state, OSPM fills in this field with the physical memory address of an OS-specific wake function.
    /// During POST, the platform firmware first checks if the value of the `x_firmware_waking_vector` field is non-zero and if so transfers control to OSPM as outlined in the `x_firmware_waking_vector` field description below.
    /// If the `x_firmware_waking_vector` field is zero then the platform firmware checks the value of this field and if it is non-zero, transfers control to the specified address.
    ///
    /// On PCs, the wake function address is in memory below 1 MB and the control is transferred while in real mode. OSPM's wake function restores the processors' context.
    ///
    /// For IA-PC platforms, the following example shows the relationship between the physical address in the Firmware Waking Vector and the real mode address the BIOS jumps to:<br>
    /// If, for example, the physical address is 0x12345, then the BIOS must jump to real mode address 0x1234:0x0005.<br>
    /// In general this relationship is Real-mode address = Physical address >> 4 : Physical address and 0x000F.
    ///
    /// Notice that on IA-PC platforms, A20 must be enabled when the BIOS jumps to the real mode address derived from the physical address stored in the Firmware Waking Vector.
    pub firmware_waking_vector: u32,
    /// This field contains the Global Lock used to synchronize access to shared hardware resources between the OSPM environment and an external controller environment (for example, the SMI environment).
    ///
    /// This lock is owned exclusively by either OSPM or the firmware at any one time.
    /// When ownership of the lock is attempted, it might be busy, in which case the requesting environment exits and waits for the signal that the lock has been released.
    ///
    /// For example, the Global Lock can be used to protect an embedded controller interface such that only OSPM or the firmware will access the embedded controller interface at any one time.
    ///
    /// See Section 5.2.10.1 for more information on acquiring and releasing the Global Lock.
    pub global_lock: u32,
    pub flags: FACSFlags,
    /// 64-bit physical address of OSPM's Waking Vector.
    ///
    /// Before transitioning the system into a global sleeping state, OSPM fills in this field and the OSPM Flags field to describe the waking vector.
    /// OSPM populates this field with the physical memory address of an OS-specific wake function.
    ///
    /// During POST, the platform firmware checks if the value of this field is non-zero and if so transfers control to OSPM by jumping to this address after creating the appropriate execution environment, which must be configured as follows:
    ///
    /// - For 64-bit Itanium™ Processor Family (IPF) -based platforms: Interrupts must be disabled The processor must have psr.i set to 0.
    ///   - Memory address translation must be disabled The processor must have psr.it, psr.dt, and psr.rt set to 0.
    ///   - See the Intel® ItaniumTM Architecture Software Developer's Manual for more information.
    /// - For IA 32 and x64 platforms, platform firmware is required to support a 32 bit execution environment.
    ///   - Platform firmware can additionally support a 64 bit execution environment.
    ///   - If platform firmware supports a 64 bit execution environment, firmware inspects the OSPM Flags during POST.
    ///   - If the 64BIT_WAKE_F flag is set, the platform firmware creates a 64 bit execution environment. Otherwise, the platform firmware creates a 32 bit execution environment.
    /// - For 64 bit execution environment:
    ///   - Interrupts must be disabled
    ///   - EFLAGS.IF set to 0
    ///   - Long mode enabled
    ///   - Paging mode is enabled
    ///   - Physical memory for waking vector is identity mapped (virtual address equals physical address)
    ///   - Waking vector must be contained within one physical page
    ///   - Selectors are set to be flat and are otherwise not used For 32 bit execution environment
    ///   - Memory address translation / paging must be disabled
    ///   - 4 GB flat address space for all segment registers
    pub x_firmware_waking_vector: u64,
    /// 3-Version of this table.
    pub version: u8,
    reserved1: [u8; 3],
    /// OSPM enabled firmware control structure flags. Platform firmware must initialize this field to zero.
    pub ospem_flags: u32,
    reserved2: [u8; 24],
}
impl FirmwareACPIControl {
    /// **JJ's Note: This is just for now, once I get the ACPI stuff done, I'll learn more about this.**
    pub const fn atomic_global_lock(&self) -> &core::sync::atomic::AtomicU32 {
        unsafe {
            &*((self as *const Self).add(0x10) as *const u32
                as *const core::sync::atomic::AtomicU32)
        }
    }
}
