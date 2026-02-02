#[derive(Copy, Clone)]
/// ## GIC CPU Interface Flags
pub struct GICCPUInterfaceFlags(u32);
impl GICCPUInterfaceFlags {
    /// If this bit is set, the processor is ready for use. If this bit is clear and the Online Capable bit is set, the system supports enabling this processor during OS runtime.<br>
    /// If this bit is clear and the Online Capable bit is also clear, this processor is unusable, and the operating system support will not attempt to use it.
    pub const fn enabled(&self) -> bool {
        self.0 & 0b0001 != 0
    }
    /// - 0 - Level-triggered
    /// - 1 - Edge-triggered
    pub const fn performance_interrupt_mode(&self) -> bool {
        self.0 & 0b0010 != 0
    }
    /// - 0 - Level-triggered
    /// - 1 - Edge-triggered
    pub const fn vgic_maintenance_interrupt_mode_flags(&self) -> bool {
        self.0 & 0b0100 != 0
    }
    /// The information conveyed by this bit depends on the value of the Enabled bit.
    ///
    /// If the Enabled bit is set, this bit is reserved and must be zero.
    /// Otherwise, if this bit is set, the system supports enabling this processor later during OS runtime.
    pub const fn online_capable(&self) -> bool {
        self.0 & 0b1000 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## GIC CPU Interface (GICC) Structure
///
/// In the GIC interrupt model, logical processors are required to have a Processor Device object in the DSDT,
/// and must convey each processor's GIC information to the OS using the GICC structure.
pub struct GICCPUInterface {
    /// 11 - GICC Structure
    pub r#type: u8,
    /// 82
    pub length: u8,
    reserved0: u16,
    /// GIC's CPU Interface Number.
    ///
    /// In GICv1/v2 implementations, this value matches the bit index of the associated processor in the GIC distributor's GICD_ITARGETSR register.<br>
    /// For GICv3/4 implementations, this field must be provided by the platform, if compatibility mode is supported.
    ///
    /// If it is not supported by the implementation, then this field must be zero.
    pub cpu_interface_number: u32,
    /// The OS associates this GICC Structure with a processor device object in the namespace
    /// when the _UID child object of the processor device evaluates to a numeric value that matches the numeric value in this field.
    pub acpi_processor_uid: u32,
    /// GIC CPU Interface Flags.
    pub flags: GICCPUInterfaceFlags,
    /// Version of the ARM-Processor Parking Protocol implemented.
    ///
    /// See http://uefi.org/acpi.  The document link is listed under "Multiprocessor Startup for ARM Platforms."
    ///
    /// For systems that support PSCI exclusively and do not support the parking protocol, this field must be set to 0.
    pub parking_protocol_version: u32,
    /// The GSIV used for Performance Monitoring Interrupts.
    pub performance_interrupt_gsiv: u32,
    /// The 64-bit physical address of the processor's Parking Protocol mailbox.
    pub parked_address: u64,
    /// On GICv1/v2 systems and GICv3/4 systems in GICv2 compatibility mode,
    /// this field holds the 64-bit physical address at which the processor can access this GIC CPU Interface.
    ///
    /// If provided here, the "Local Interrupt Controller Address" field in the MADT must be ignored by the OSPM.
    pub physical_base_address: u64,
    /// Address of the GIC virtual CPU interface registers.
    ///
    /// If the platform is not presenting a GICv2 with virtualization extensions this field can be 0.
    pub gicv: u64,
    /// Address of the GIC virtual interface control block registers.
    ///
    /// If the platform is not presenting a GICv2 with virtualization extensions this field can be 0.
    pub gich: u64,
    /// GSIV for Virtual GIC maintenance interrupt.
    pub vgic_maintenance_interrupt: u32,
    /// On systems supporting GICv3 and above, this field holds the 64-bit physical address of the associated Redistributor.
    ///
    /// If all of the GIC Redistributors are in the always-on power domain, GICR structures should be used to describe the Redistributors instead, and this field must be set to 0.
    ///
    /// If a GICR structure is present in the MADT then this field must be ignored by the OSPM.
    pub gicr_base_address: u64,
    /// This fields follows the MPIDR formatting of ARM architecture. If ARMv7 architecture is used then the format must be as follows:
    ///
    /// - **Bits [[63:24]]** - Must be zero.
    /// - **Bits [[23:16]] Aff2** - Match Aff2 of target processor MPIDR.
    /// - **Bits [[15:08]] Aff1** - Match Aff1 of target processor MPIDR.
    /// - **Bits [[07:00]] Aff0** - Match Aff0 of target processor MPIDR.
    ///
    /// For platforms implementing ARMv8 the format must be:
    ///
    /// - **Bits [[63:40]]** - Must be zero
    /// - **Bits [[39:32]] Aff3** - Match Aff3 of target processor MPIDR
    /// - **Bits [[31:24]]** - Must be zero
    /// - **Bits [[23:16]] Aff2** - Match Aff2 of target processor MPIDR
    /// - **Bits [[15:08]] Aff1** - Match Aff1 of target processor MPIDR
    /// - **Bits [[07:00]] Aff0** - Match Aff0 of target processor MPIDR
    pub mpidr: u64,
    /// Describes the relative power efficiency of the associated processor.
    ///
    /// Lower efficiency class numbers are more efficient than higher ones (e.g. efficiency class 0 should be treated as more efficient than efficiency class 1).
    /// However, absolute values of this number have no meaning: 2 isn't necessarily half as efficient as 1.
    pub processor_power_efficiency_class: u8,
    reserved1: u8,
    /// Statistical Profiling Extension buffer overflow GSIV.
    ///
    /// This interrupt is a level triggered PPI. Zero if SPE is not supported by this processor.
    pub spe_overflow_interrupt: u16,
    /// Trace Buffer Extension interrupt GSIV.
    ///
    /// This interrupt is a level triggered PPI. Zero if TRBE feature is not supported by this processor.
    ///
    /// NOTE: This field was introduced in ACPI Specification version 6.5.
    ///
    /// **JJ's Note: The previous note is EXTREMELY important.  This structure will be 2 bytes less (80 bytes total) in memory with ACPI 6.4 or lower.**
    pub trbe_interrupt: u16,
}