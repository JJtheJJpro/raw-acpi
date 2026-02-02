use crate::{GenericAddressStructure, SDTHeader};

#[derive(Copy, Clone)]
pub enum FADTPersistentCPUCacheFeature {
    NotReported,
    NotPersistent,
    Persistent,
}

#[derive(Copy, Clone)]
/// ## Fixed ACPI Description Table Fixed Feature Flags
pub struct FADTFixedFeatureFlags(u32);
impl FADTFixedFeatureFlags {
    /// Processor properly implements a functional equivalent to the WBINVD IA-32 instruction.
    ///
    /// If set, signifies that the WBINVD instruction correctly flushes the processor caches, maintains memory coherency,
    /// and upon completion of the instruction, all caches for the current processor contain no cached data other than what OSPM references and allows to be cached.
    ///
    /// If this flag is not set, the ACPI OS is responsible for disabling all ACPI features that need this function.
    /// This field is maintained for ACPI 1.0 processor compatibility on existing systems. Processors in new ACPI-compatible systems are required to support this function and indicate this to OSPM by setting this field.
    pub const fn wbinvd(&self) -> bool {
        self.0 & 0b00000000000000000000000000000001 != 0
    }
    /// If set, indicates that the hardware flushes all caches on the WBINVD instruction and maintains memory coherency, but does not guarantee the caches are invalidated.
    /// This provides the complete semantics of the WBINVD instruction, and provides enough to support the system sleeping states.
    ///
    /// If neither of the WBINVD flags is set, the system will require FLUSH_SIZE and FLUSH_STRIDE to support sleeping states.
    /// If the FLUSH parameters are also not supported, the machine cannot support sleeping states S1, S2, or S3.
    pub const fn wbinvd_flush(&self) -> bool {
        self.0 & 0b00000000000000000000000000000010 != 0
    }
    /// A one indicates that the C1 power state is supported on all processors.
    pub const fn proc_c1(&self) -> bool {
        self.0 & 0b00000000000000000000000000000100 != 0
    }
    /// A zero indicates that the C2 power state is configured to only work on a uniprocessor (UP) system.<br>
    /// A one indicates that the C2 power state is configured to work on a UP or multiprocessor (MP) system.
    pub const fn p_lvl2_up(&self) -> bool {
        self.0 & 0b00000000000000000000000000001000 != 0
    }
    /// A zero indicates the power button is handled as a fixed feature programming model.<br>
    /// A one indicates the power button is handled as a control method device.
    ///
    /// If the system does not have a power button, this value would be "1" and no power button device would be present.
    /// Independent of the value of this field, the presence of a power button device in the namespace indicates to OSPM that the power button is handled as a control method device.
    pub const fn pwr_button(&self) -> bool {
        self.0 & 0b00000000000000000000000000010000 != 0
    }
    /// A zero indicates the sleep button is handled as a fixed feature programming model.<br>
    /// A one indicates the sleep button is handled as a control method device.
    ///
    /// If the system does not have a sleep button, this value would be "1" and no sleep button device would be present.
    /// Independent of the value of this field, the presence of a sleep button device in the namespace indicates to OSPM that the sleep button is handled as a control method device.
    pub const fn slp_button(&self) -> bool {
        self.0 & 0b00000000000000000000000000100000 != 0
    }
    /// A zero indicates the RTC wake status is supported in fixed register space.<br>
    /// A one indicates the RTC wake status is not supported in fixed register space.
    pub const fn fix_rtc(&self) -> bool {
        self.0 & 0b00000000000000000000000001000000 != 0
    }
    /// Indicates whether the RTC alarm function can wake the system from the S4 state.
    ///
    /// The RTC must be able to wake the system from an S1, S2, or S3 sleep state.
    /// The RTC alarm can optionally support waking the system from the S4 state, as indicated by this value.
    pub const fn rtc_s4(&self) -> bool {
        self.0 & 0b00000000000000000000000010000000 != 0
    }
    /// A zero indicates TMR_VAL is implemented as a 24-bit value.<br>
    /// A one indicates TMR_VAL is implemented as a 32-bit value.
    ///
    /// The TMR_STS bit is set when the most significant bit of the TMR_VAL toggles.
    pub const fn tmr_val_ext(&self) -> bool {
        self.0 & 0b00000000000000000000000100000000 != 0
    }
    /// A zero indicates that the system cannot support docking.<br>
    /// A one indicates that the system can support docking.
    ///
    /// Notice that this flag does not indicate whether or not a docking station is currently present; it only indicates that the system is capable of docking.
    pub const fn dck_cap(&self) -> bool {
        self.0 & 0b00000000000000000000001000000000 != 0
    }
    /// If set, indicates the system supports system reset via the FADT RESET_REG as described in Section 4.8.3.6.
    pub const fn reset_reg_sup(&self) -> bool {
        self.0 & 0b00000000000000000000010000000000 != 0
    }
    /// System Type Attribute.
    ///
    /// If set, indicates that the system has no internal expansion capabilities and the case is sealed.
    pub const fn sealed_case(&self) -> bool {
        self.0 & 0b00000000000000000000100000000000 != 0
    }
    /// System Type Attribute.
    ///
    /// If set, indicates the system cannot detect the monitor or keyboard/mouse devices.
    pub const fn headless(&self) -> bool {
        self.0 & 0b00000000000000000001000000000000 != 0
    }
    /// If set, indicates to OSPM that a processor native instruction must be executed after writing the SLP_TYPx register.
    pub const fn cpu_sw_slp(&self) -> bool {
        self.0 & 0b00000000000000000010000000000000 != 0
    }
    /// If set, indicates the platform supports the PCIEXP_WAKE_STS bit in the PM1 Status register and the PCIEXP_WAKE_EN bit in the PM1 Enable register.
    ///
    /// This bit must be set on platforms containing chipsets that implement PCI Express and supports PM1 PCIEXP_WAK bits.
    pub const fn pci_exp_wak(&self) -> bool {
        self.0 & 0b00000000000000000100000000000000 != 0
    }
    /// A value of one indicates that OSPM should use a platform provided timer to drive any monotonically non-decreasing counters, such as OSPM performance counter services.
    /// Which particular platform timer will be used is OSPM specific; however, it is recommended that the timer used is based on the following algorithm:
    ///
    /// - If the HPET is exposed to OSPM, OSPM should use the HPET.
    /// - Otherwise, OSPM will use the ACPI power management timer.
    ///
    /// A value of one indicates that the platform is known to have a correctly implemented ACPI power management timer.
    /// A platform may choose to set this flag if a internal processor clock (or clocks in a multi-processor configuration) cannot provide consistent monotonically non-decreasing counters.
    ///
    /// Note: If a value of zero is present, OSPM may arbitrarily choose to use an internal processor clock or a platform timer clock for these operations.
    /// That is, a zero does not imply that OSPM will necessarily use the internal processor clock to generate a monotonically non-decreasing counter to the system.
    pub const fn use_platform_clock(&self) -> bool {
        self.0 & 0b00000000000000001000000000000000 != 0
    }
    /// A one indicates that the contents of the RTC_STS flag is valid when waking the system from S4. See Table 4.11 for more information.
    ///
    /// Some existing systems do not reliably set this input today, and this bit allows OSPM to differentiate correctly functioning platforms from platforms with this errata.
    pub const fn s4_rtc_sts_valid(&self) -> bool {
        self.0 & 0b00000000000000010000000000000000 != 0
    }
    /// A one indicates that the platform is compatible with remote power-on. That is, the platform supports OSPM leaving GPE wake events armed prior to an S5 transition.
    ///
    /// Some existing platforms do not reliably transition to S5 with wake events enabled (for example, the platform may immediately generate a spurious wake event after completing the S5 transition).
    /// This flag allows OSPM to differentiate correctly functioning platforms from platforms with this type of errata.
    pub const fn remote_power_on_capable(&self) -> bool {
        self.0 & 0b00000000000000100000000000000000 != 0
    }
    /// A one indicates that all local APICs must be configured for the cluster destination model when delivering interrupts in logical mode.
    ///
    /// If this bit is set, then logical mode interrupt delivery operation may be undefined until OSPM has moved all local APICs to the cluster model.
    ///
    /// Note that the cluster destination model doesn't apply to Itanium™ Processor Family (IPF) local SAPICs.
    /// This bit is intended for xAPIC based machines that require the cluster destination model even when 8 or fewer local APICs are present in the machine.
    pub const fn force_apic_cluster_model(&self) -> bool {
        self.0 & 0b00000000000001000000000000000000 != 0
    }
    /// A one indicates that all local xAPICs must be configured for physical destination mode.
    ///
    /// If this bit is set, interrupt delivery operation in logical destination mode is undefined.
    /// On machines that contain fewer than 8 local xAPICs or that do not use the xAPIC architecture, this bit is ignored.
    pub const fn force_apic_physical_destination_mode(&self) -> bool {
        self.0 & 0b00000000000010000000000000000000 != 0
    }
    /// A one indicates that the Hardware-Reduced ACPI (section 4.1) is implemented, therefore software-only alternatives are used for supported fixed-features defined in chapter 4.
    pub const fn hw_reduced_acpi(&self) -> bool {
        self.0 & 0b00000000000100000000000000000000 != 0
    }
    /// A one informs OSPM that the platform is able to achieve power savings in S0 similar to or better than those typically achieved in S3.
    ///
    /// In effect, when this bit is set it indicates that the system will achieve no power benefit by making a sleep transition to S3.
    pub const fn low_power_s0_idle_capable(&self) -> bool {
        self.0 & 0b00000000001000000000000000000000 != 0
    }
    /// The following values describe whether cpu caches and any other caches that are coherent with them, are considered by the platform to be persistent.
    /// The platform evaluates the configuration present at system startup to determine this value. System configuration changes after system startup may invalidate this.
    /// - 00b - Not reported by the platform. Software should reference the NFIT Platform Capabilities
    /// - 01b - Cpu caches and any other caches that are coherent with them, are not persistent. Software is responsible for flushing data from cpu caches to make stores persistent. Supersedes NFIT Platform Capabilities.
    /// - 10b - Cpu caches and any other caches that are coherent with them, are persistent. Supersedes NFIT Platform Capabilities.
    /// When reporting this state, the platform shall provide enough stored energy for ALL of the following:
    ///   - Time to flush cpu caches and any other caches that are coherent with them
    ///   - Time of all targets of those flushes to complete flushing stored data
    ///   - If supporting hot plug, the worst case CXL device topology that can be hot plugged
    /// - 11b - Reserved
    ///
    /// **JJ's note: I made a rust enum just for this 2-bit flag to avoid any sort of confusion when working with this.**
    pub const fn persistent_cpu_caches(&self) -> FADTPersistentCPUCacheFeature {
        match (self.0 & 0b00000000110000000000000000000000) >> 22 {
            0b00 => FADTPersistentCPUCacheFeature::NotReported,
            0b01 => FADTPersistentCPUCacheFeature::NotPersistent,
            0b10 => FADTPersistentCPUCacheFeature::Persistent,
            0b11 => panic!(
                "FADT Persistent CPU Cache 2-bit flag set to 0b11, which is a reserved value."
            ),
            _ => unreachable!(),
        }
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
/// ## IA-PC Boot Architecture Flags
///
/// This set of flags is used by an OS to guide the assumptions it can make in initializing hardware on IA-PC platforms.
/// These flags are used by an OS at boot time (before the OS is capable of providing an operating environment suitable for parsing the ACPI namespace) to determine the code paths to take during boot.
///
/// In IA-PC platforms with reduced legacy hardware, the OS can skip code paths for legacy devices if none are present.
/// For example, if there are no ISA devices, an OS could skip code that assumes the presence of these devices and their associated resources.
/// These flags are used independently of the ACPI namespace. The presence of other devices must be described in the ACPI namespace as specified in Section 6 These flags pertain only to IA-PC platforms.
///
/// On other system architectures, the entire field should be set to 0.
pub struct FADTIAPCBootArch(u16);
impl FADTIAPCBootArch {
    /// If set, indicates that the motherboard supports user-visible devices on the LPC or ISA bus. User-visible devices are devices that have end-user accessible connectors (for example, LPT port), or devices for which the OS must load a device driver so that an end-user application can use a device.
    ///
    /// If clear, the OS may assume there are no such devices and that all devices in the system can be detected exclusively via industry standard device enumeration mechanisms (including the ACPI namespace).
    pub const fn legacy_devices(&self) -> bool {
        self.0 & 0b0000000000000001 != 0
    }
    /// If set, indicates that the motherboard contains support for a port 60 and 64 based keyboard controller, usually implemented as an 8042 or equivalent micro-controller.
    pub const fn _8042(&self) -> bool {
        self.0 & 0b0000000000000010 != 0
    }
    /// If set, indicates to OSPM that it must not blindly probe the VGA hardware (that responds to MMIO addresses A0000h-BFFFFh and IO ports 3B0h-3BBh and 3C0h-3DFh) that may cause machine check on this system.
    ///
    /// If clear, indicates to OSPM that it is safe to probe the VGA hardware.
    pub const fn vga_not_present(&self) -> bool {
        self.0 & 0b0000000000000100 != 0
    }
    /// If set, indicates to OSPM that it must not enable Message Signaled Interrupts (MSI) on this platform.
    pub const fn msi_not_supported(&self) -> bool {
        self.0 & 0b0000000000001000 != 0
    }
    /// If set, indicates to OSPM that it must not enable OSPM ASPM control on this platform.
    pub const fn pcie_aspm_controls(&self) -> bool {
        self.0 & 0b0000000000010000 != 0
    }
    /// If set, indicates that the CMOS RTC is either not implemented, or does not exist at the legacy addresses.
    /// OSPM uses the Control Method Time and Alarm Namespace device instead.
    pub const fn cmos_rtc_not_present(&self) -> bool {
        self.0 & 0b0000000000100000 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
/// ## ARM Architecture Boot Flags
///
/// These flags are used by an OS at boot time (before the OS is capable of providing an operating environment suitable for parsing the ACPI namespace) to determine the code paths to take during boot.
///
/// For the PSCI flags, specifically, the flags describe if the platform is compliant with the PSCI specification.
///
/// A link to the PSCI specification can be found at "Links to ACPI-Related Documents" at http://uefi.org/acpi.
pub struct FADTARMBootArch(u16);
impl FADTARMBootArch {
    /// 1 if PSCI is implemented.
    pub const fn psci_compliant(&self) -> bool {
        self.0 & 0b01 != 0
    }
    /// 1 if HVC must be used as the PSCI conduit.instead of SMC.
    pub const fn psci_use_hvc(&self) -> bool {
        self.0 & 0b01 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Fixed ACPI Description Table
///
/// The Fixed ACPI Description Table (FADT) defines various fixed hardware ACPI information vital to an ACPI-compatible OS.
pub struct FixedACPIDescriptionTable {
    /// - **Signature** - "FACP"
    /// - **Revision** - FADT Major Version (see the minor version field of this structure).
    /// - **OEM Table ID** - For the FADT, the table ID is the manufacture model ID. This field must match the OEM Table ID in the RSDT.
    pub header: SDTHeader,

    /// Physical memory address of the FACS, where OSPM and Firmware exchange control information.
    ///
    /// See <a href="https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#firmware-acpi-control-structure-facs">Section 5.2.10</a> for more information about the FACS.
    ///
    /// If the `x_firmware_ctrl` field contains a non zero value which can be used by the OSPM, then this field must be ignored by the OSPM.<br>
    /// If the HARDWARE_REDUCED_ACPI flag is set, and both this field and the `x_firmware_ctrl` field are zero, there is no FACS available.
    pub firmware_ctrl: u32,
    /// Physical memory address of the DSDT. If the `x_dsdt` field contains a non-zero value which can be used by the OSPM, then this field must be ignored by the OSPM.
    pub dsdt: u32,
    /// ACPI 1.0 defined this offset as a field named INT_MODEL, which was eliminated in ACPI 2.0. Platforms should set this field to zero but field values of one are also allowed to maintain compatibility with ACPI 1.0.
    pub int_model: u8,
    /// This field is set by the OEM to convey the preferred power management profile to OSPM. OSPM can use this field to set default power management policy parameters during OS installation. Field Values:
    ///
    /// - 0 - Unspecified
    /// - 1 - Desktop
    /// - 2 - Mobile
    /// - 3 - Workstation
    /// - 4 - Enterprise Server
    /// - 5 - SOHO Server
    /// - 6 - Appliance PC
    /// - 7 - Performance Server
    /// - 8 - Tablet
    ///
    /// Other values are reserved.
    pub preferred_pm_profile: u8,

    /// System vector the SCI interrupt is wired to in 8259 mode.
    ///
    /// On systems that do not contain the 8259, this field contains the Global System interrupt number of the SCI interrupt.
    /// OSPM is required to treat the ACPI SCI interrupt as a shareable, level, active low interrupt.
    pub sci_int: u16,
    /// System port address of the SMI Command Port.
    ///
    /// During ACPI OS initialization, OSPM can determine that the ACPI hardware registers are owned by SMI (by way of the SCI_EN bit), in which case the ACPI OS issues the ACPI_ENABLE command to the SMI_CMD port.
    /// The SCI_EN bit effectively tracks the ownership of the ACPI hardware registers. OSPM issues commands to the SMI_CMD port synchronously from the boot processor.
    /// This field is reserved and must be zero on systems that does not support System Management mode.
    pub smi_cmd: u32,
    /// The value to write to SMI_CMD to disable SMI ownership of the ACPI hardware registers.
    ///
    /// The last action SMI does to relinquish ownership is to set the SCI_EN bit.
    /// During the OS initialization process, OSPM will synchronously wait for the ntransfer of SMI ownership to complete, so the ACPI system releases SMI ownership as quickly as possible.
    /// This field is reserved and must be zero on systems that do not support Legacy Mode.
    pub acpi_enable: u8,
    /// The value to write to SMI_CMD to re-enable SMI ownership of the ACPI hardware registers.
    ///
    /// This can only be done when ownership was originally acquired from SMI by OSPM using ACPI_ENABLE.
    /// An OS can hand ownership back to SMI by relinquishing use to the ACPI hardware registers, masking off all SCI interrupts, clearing the SCI_EN bit and then writing ACPI_DISABLE to the SMI_CMD port from the boot processor.
    /// This field is reserved and must be zero on systems that do not support Legacy Mode.
    pub acpi_disable: u8,
    /// The value to write to SMI_CMD to enter the S4BIOS state.
    ///
    /// The S4BIOS state provides an alternate way to enter the S4 state where the firmware saves and restores the memory context.
    /// A value of zero in S4BIOS_F indicates S4BIOS_REQ is not supported.
    pub s4bios_req: u8,
    /// If non-zero, this field contains the value OSPM writes to the SMI_CMD register to assume processor performance state control responsibility.
    pub pstate_cnt: u8,

    /// System port address of the PM1a Event Register Block. See Section 4.8.3.1 for a hardware description layout of this register block.
    ///
    /// This is a required field. If the `x_pm1a_evt_blk` field contains a non zero value which can be used by the OSPM, then this field must be ignored by the OSPM.
    pub pm1a_evt_blk: u32,
    /// System port address of the PM1b Event Register Block. See Section 4.8.3.1 for a hardware description layout of this register block.
    ///
    /// This field is optional; if this register block is not supported, this field contains zero.
    /// If the `x_pm1b_evt_blk` field contains a non zero value which can be used by the OSPM, then this field must be ignored by the OSPM.
    pub pm1b_evt_blk: u32,
    /// System port address of the PM1a Control Register Block. See Section 4.8.3.1 for a hardware description layout of this register block.
    ///
    /// This is a required field. If the `x_pm1a_cnt_blk` field contains a non zero value which can be used by the OSPM, then this field must be ignored by the OSPM.
    pub pm1a_cnt_blk: u32,
    /// System port address of the PM1b Control Register Block. See Section 4.8.3.1 for a hardware description layout of this register block.
    ///
    /// This field is optional; if this register block is not supported, this field contains zero.
    /// If the `x_pm1b_cnt_blk` field contains a non zero value which can be used by the OSPM, then this field must be ignored by the OSPM.
    pub pm1b_cnt_blk: u32,
    /// System port address of the PM2 Control Register Block. See Table 4.4 for a hardware description layout of this register block.
    ///
    /// This field is optional; if this register block is not supported, this field contains zero.
    /// If the `x_pm2_cnt_blk` field contains a non zero value which can be used by the OSPM, then this field must be ignored by the OSPM.
    pub pm2_cnt_blk: u32,
    /// System port address of the Power Management Timer Control Register Block. See the Section 4.8.3.3 for a hardware description layout of this register block.
    ///
    /// This is an optional field; if this register block is not supported, this field contains zero.
    /// If the `x_pm_tmr_blk` field contains a non-zero value which can be used by the OSPM, then this field must be ignored by the OSPM.
    pub pm_tmr_blk: u32,
    /// System port address of General-Purpose Event 0 Register Block. See Section 4.8.4.1 for more information.
    ///
    /// If this register block is not supported, this field contains zero.
    /// If the `x_gpe0_blk` field contains a nonzero value which can be used by the OSPM, then this field must be ignored by the OSPM.
    pub gpe0_blk: u32,
    /// System port address of General-Purpose Event 1 Register Block. See Section 4.8.4.1 for more information.
    ///
    /// This is an optional field; if this register block is not supported, this field contains zero.
    /// If the `x_gpe1_blk` field contains a nonzero value which can be used by the OSPM, then this field must be ignored by the OSPM.
    pub gpe1_blk: u32,

    /// Number of bytes decoded by `pm1a_evt_blk` and, if supported, `pm1b_evt_blk`. This value is &ge;4.
    pub pm1_evt_len: u8,
    /// Number of bytes decoded by `pm1a_cnt_blk` and, if supported, `pm1b_cnt_blk`. This value is &ge;2.
    pub pm1_cnt_len: u8,
    /// Number of bytes decoded by `pm2_cnt_blk`. Support for the PM2 register block is optional. If supported, this value is &ge;1. If not supported, this field contains zero.
    pub pm2_cnt_len: u8,
    /// Number of bytes decoded by `pm_tmr_blk`. If the PM Timer is supported, this field's value must be 4. If not supported, this field contains zero.
    pub pm_tmr_len: u8,
    /// The length of the register whose address is given by `x_gpe0_blk` (if nonzero) or by `gpe0_blk` (otherwise) in bytes. The value is a non-negative multiple of 2.
    pub gpe0_blk_len: u8,
    /// The length of the register whose address is given by `x_gpe1_blk` (if nonzero) or by `gpe1_blk` (otherwise) in bytes. The value is a non-negative multiple of 2.
    pub gpe1_blk_len: u8,
    /// Offset within the ACPI general-purpose event model where GPE1 based events start.
    pub gpe1_base: u8,
    /// If non-zero, this field contains the value OSPM writes to the SMI_CMD register to indicate OS support for the _CST object and C States Changed notification.
    pub cst_cnt: u8,

    /// The worst-case hardware latency, in microseconds, to enter and exit a C2 state. A value > 100 indicates the system does not support a C2 state.
    pub p_lvl2_lat: u16,
    /// The worst-case hardware latency, in microseconds, to enter and exit a C3 state. A value > 1000 indicates the system does not support a C3 state.
    pub p_lvl3_lat: u16,
    /// - If WBINVD=0, the value of this field is the number of flush strides that need to be read (using cacheable addresses) to completely flush dirty lines from any processor's memory caches.
    ///   - Notice that the value in `flush_stride` is typically the smallest cache line width on any of the processor's caches (for more information, see the `flush_stride` field definition).
    ///
    /// - If the system does not support a method for flushing the processor's caches, then `flush_size` and WBINVD are set to zero.
    ///   - Notice that this method of flushing the processor caches has limitations, and WBINVD=1 is the preferred way to flush the processors caches.
    ///
    /// - This value is typically at least 2 times the cache size.
    ///   - The maximum allowed value for `flush_size` multiplied by `flush_stride` is 2 MB for a typical maximum supported cache size of 1 MB.
    ///   - Larger cache sizes are supported using WBINVD=1.
    ///
    /// - This value is ignored if WBINVD=1.
    ///
    /// This field is maintained for ACPI 1.0 processor compatibility on existing systems.
    /// Processors in new ACPI-compatible systems are required to support the WBINVD function and indicate this to OSPM by setting the WBINVD field = 1.
    pub flush_size: u16,
    /// - If WBINVD=0, the value of this field is the cache line width, in bytes, of the processor's memory caches.
    /// - This value is typically the smallest cache line width on any of the processor's caches. For more information, see the description of the `flush_size` field.
    /// - This value is ignored if WBINVD=1.
    ///
    /// This field is maintained for ACPI 1.0 processor compatibility on existing systems.
    /// Processors in new ACPI-compatible systems are required to support the WBINVD function and indicate this to OSPM by setting the WBINVD field = 1.
    pub flush_stride: u16,

    /// The zero-based index of where the processor's duty cycle setting is within the processor's P_CNT register.
    pub duty_offset: u8,
    /// The bit width of the processor's duty cycle setting value in the P_CNT register.
    ///
    /// Each processor's duty cycle setting allows the software to select a nominal processor frequency below its absolute frequency as defined by:
    ///
    /// THTL_EN = 1 ⇒ BF * DC / (2 ^ `duty_width`)
    ///
    /// Where:
    /// - BF-Base frequency
    /// - DC-Duty cycle setting
    ///
    /// When THTL_EN is 0, the processor runs at its absolute BF.
    /// A `duty_width` value of 0 indicates that processor duty cycle is not supported and the processor continuously runs at its base frequency.
    pub duty_width: u8,
    /// The RTC CMOS RAM index to the day-of-month alarm value.
    ///
    /// If this field contains a zero, then the RTC day of the month alarm feature is not supported.
    /// If this field has a non-zero value, then this field contains an index into RTC RAM space that OSPM can use to program the day of the month alarm.
    /// See Section 4.8.2.4 for a description of how this hardware works.
    pub day_alrm: u8,
    /// The RTC CMOS RAM index to the month of year alarm value.
    ///
    /// If this field contains a zero, then the RTC month of the year alarm feature is not supported.
    /// If this field has a non-zero value, then this field contains an index into RTC RAM space that OSPM can use to program the month of the year alarm.
    /// If this feature is supported, then the DAY_ALRM feature must be supported also.
    pub mon_alrm: u8,
    /// The RTC CMOS RAM index to the century of data value (hundred and thousand year decimals).
    ///
    /// If this field contains a zero, then the RTC centenary feature is not supported.
    /// If this field has a non-zero value, then this field contains an index into RTC RAM space that OSPM can use to program the centenary field.
    pub century: u8,

    /// IA-PC Boot Architecture Flags. See Section 5.2.9.3 for a description of this field.
    pub iapc_boot_arch: FADTIAPCBootArch,
    /// Must be 0.
    reserved: u8,
    /// Fixed feature flags.
    pub flags: FADTFixedFeatureFlags,
    /// The address of the reset register represented in Generic Address Structure format (See Section 4.8.3.6 for a description of the reset mechanism).
    /// Note: Only System I/O space, System Memory space and PCI Configuration space (bus #0) are valid for values for `address_space_id`.
    /// Also, `reg_bit_width` must be 8 and `reg_bit_offset` must be 0.
    pub reset_reg: GenericAddressStructure,
    /// Indicates the value to write to the RESET_REG port to reset the system (See Section 4.8.3.6 for a description of the reset mechanism).
    pub reset_value: u8,
    /// ARM Boot Architecture Flags. See Table 5.12 for a description of this field.
    pub arm_boot_arch: FADTARMBootArch,
    /// Minor Version of this FADT structure, in "Major.Minor" form, where 'Major' is the value in the Major Version Field (Byte offset 8 in this table).
    /// - Bits 0-3 - The low order bits correspond to the minor version of the specification version. For instance, ACPI 6.3 has a major version of 6, and a minor version of 3.
    /// - Bits 4-7 - The high order bits correspond to the version of the ACPI Specification errata this table complies with. A value of 0 means that it complies with the base version of the current specification.
    /// A value of 1 means this is compatible with Errata A, 2 would be compatible with Errata B, and so on.
    pub minor_version: u8,

    /// Extended physical address of the FACS.
    ///
    /// If this field contains a nonzero value which can be used by the OSPM, then the `firmware_ctrl` field must be ignored by the OSPM.
    /// If the HARDWARE_REDUCED_ACPI flag is set, and both this field and the `firmware_ctrl` field are zero, there is no FACS available.
    pub x_firmware_ctrl: u64,
    /// Extended physical address of the DSDT.
    ///
    /// If this field contains a nonzero value which can be used by the OSPM, then the `dsdt` field must be ignored by the OSPM.
    pub x_dsdt: u64,
    /// Extended address of the PM1a Event Register Block, represented in Generic Address Structure format. See Section 4.8.3.1 for a hardware description layout of this register block.
    ///
    /// This is a required field. If this field contains a nonzero value which can be used by the OSPM, then the `pm1a_evt_blk` field must be ignored by the OSPM.
    pub x_pm1a_evt_blk: GenericAddressStructure,
    /// Extended address of the PM1b Event Register Block, represented in Generic Address Structure format. See Section 4.8.3.1 for a hardware description layout of this register block.
    ///
    /// This field is optional; if this register block is not supported, this field contains zero. If this field contains a nonzero value which can be used by the OSPM, then the `pm1b_evt_blk` field must be ignored by the OSPM.
    pub x_pm1b_evt_blk: GenericAddressStructure,
    /// Extended address of the PM1a Control Register Block, represented in Generic Address Structure format. See Section 4.8.3.2 for a hardware description layout of this register block.
    ///
    /// This is a required field. If this field contains a nonzero value which can be used by the OSPM, then the `pm1a_cnt_blk` field must be ignored by the OSPM.
    pub x_pm1a_cnt_blk: GenericAddressStructure,
    /// Extended address of the PM1b Control Register Block, represented in Generic Address Structure format. See Section 4.8.3.2 for a hardware description layout of this register block.
    ///
    /// This field is optional; if this register block is not supported, this field contains zero. If this field contains a nonzero value which can be used by the OSPM, then the `pm1b_cnt_blk` field must be ignored by the OSPM.
    pub x_pm1b_cnt_blk: GenericAddressStructure,
    /// Extended address of the PM2 Control Register Block, represented in Generic Address Structure format. See PM2 Control (PM2_CNT) for a hardware description layout of this register block.
    ///
    /// This field is optional; if this register block is not supported, this field contains zero. If this field contains a nonzero value which can be used by the OSPM, then the `pm2_cnt_blk` field must be ignored by the OSPM.
    pub x_pm2_cnt_blk: GenericAddressStructure,
    /// Extended address of the Power Management Timer Control Register Block, represented in Generic Address Structure format. See Section 4.8.3.3 for a hardware description layout of this register block.
    ///
    /// This field is optional; if this register block is not supported, this field contains zero. If this field contains a nonzero value which can be used by the OSPM, then the `pm_tmr_blk` field must be ignored by the OSPM.
    pub x_pm_tmr_blk: GenericAddressStructure,
    /// Extended address of the General-Purpose Event 0 Register Block, represented in Generic Address Structure format. See Section 4.8.4.1 for more information.
    ///
    /// This is an optional field; if this register block is not supported, this field contains zero.
    /// If this field contains a nonzero value which can be used by the OSPM, then the `gpe0_blk` field must be ignored by the OSPM.
    ///
    /// Note: Only System I/O space and System Memory space are valid for `address_space_id` values, and the OSPM ignores `reg_bit_width`, `reg_bit_offset` and `access_size`.
    pub x_gpe0_blk: GenericAddressStructure,
    /// Extended address of the General-Purpose Event 1 Register Block, represented in Generic Address Structure format. See Section 4.8.4.1 for more information.
    ///
    /// This is an optional field; if this register block is not supported, this field contains zero.
    /// If this field contains a nonzero value which can be used by the OSPM, then the `gpe1_blk` field must be ignored by the OSPM.
    ///
    /// Note: Only System I/O space and System Memory space are valid for `address_space_id` values, and the OSPM ignores `reg_bit_width`, `reg_bit_offset` and `access_size`.
    pub x_gpe1_blk: GenericAddressStructure,

    /// The address of the Sleep register, represented in Generic Address Structure format (see Section 4.8.3.7 for a description of the sleep mechanism).
    ///
    /// Note: Only System I/O space, System Memory space and PCI Configuration space (bus #0) are valid for values for `address_space_id`. Also, `reg_bit_width` must be 8 and `reg_bit_offset` must be 0.
    pub sleep_control_reg: GenericAddressStructure,
    /// The address of the Sleep status register, represented in Generic Address Structure format (see Section 4.8.3.7 for a description of the sleep mechanism).
    ///
    /// Note: Only System I/O space, System Memory space and PCI Configuration space (bus #0) are valid for values for `address_space_id`. Also, `reg_bit_width` must be 8 and `reg_bit_offset` must be 0.
    pub sleep_status_reg: GenericAddressStructure,

    /// 64-bit identifier of hypervisor vendor.
    ///
    /// All bytes in this field are considered part of the vendor identity.
    /// These identifiers are defined independently by the vendors themselves, usually following the name of the hypervisor product.<br>
    /// Version information should NOT be included in this field - this shall simply denote the vendor's name or identifier.
    /// Version information can be communicated through a supplemental vendor-specific hypervisor API.<br>
    /// Firmware implementers would place zero bytes into this field, denoting that no hypervisor is present in the actual firmware.
    pub hypervisor_vendor_identity: u64,
}
