pub mod aplic;
pub mod bridge_io_pic;
pub mod core_pic;
pub mod extend_io_pic;
pub mod gic_cpu_interface;
pub mod gic_distributor;
pub mod gic_its;
pub mod gic_msi_frame;
pub mod gic_redistributor;
pub mod hyper_transport_pic;
pub mod imsic;
pub mod interrupt_source_override;
pub mod ioapic;
pub mod iosapic;
pub mod legacy_io_pic;
pub mod local_apic_address_override;
pub mod local_apic_nmi;
pub mod local_sapic;
pub mod local_x2apic_nmi;
pub mod lpc_pic;
pub mod msi_pic;
pub mod multiprocessor_wakeup;
pub mod nmi_source;
pub mod platform_interrupt_source;
pub mod plic;
pub mod processor_local_apic;
pub mod processor_local_x2apic;
pub mod rintc;

use crate::SDTHeader;

#[derive(Copy, Clone)]
pub enum ICSTypes {
    ProcessorLocalAPIC,
    IOAPIC,
    InterruptSourceOverride,
    NMISource,
    LocalAPICNMI,
    LocalAPICAddressOverride,
    IOSAPIC,
    LocalSAPIC,
    PlatformInterruptSource,
    ProcessorLocalx2APIC,
    Localx2APICNMI,
    GICCPUInterface,
    GICDistributor,
    GICMSIFrame,
    GICRedistributor,
    GICInterruptTranslationService,
    MultiprocessorWakeup,
    CoreProgrammableInterruptController,
    LegacyIOProgrammableInterruptController,
    HyperTransportProgrammableInterruptController,
    ExtendIOProgrammableInterruptController,
    MSIProgrammableInterruptController,
    BridgeIOProgrammableInterruptController,
    LPCProgrammableInterruptController,
    RISCVHartLocalInterruptController,
    RISCVIncomingMSIController,
    RISCVAdvancedPlatformLevelInterruptController,
    RISCVPlatformLevelInterruptController,
}
impl ICSTypes {
    pub fn from(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Self::ProcessorLocalAPIC),
            0x01 => Some(Self::IOAPIC),
            0x02 => Some(Self::InterruptSourceOverride),
            0x03 => Some(Self::NMISource),
            0x04 => Some(Self::LocalAPICNMI),
            0x05 => Some(Self::LocalAPICAddressOverride),
            0x06 => Some(Self::IOSAPIC),
            0x07 => Some(Self::LocalSAPIC),
            0x08 => Some(Self::PlatformInterruptSource),
            0x09 => Some(Self::ProcessorLocalx2APIC),
            0x0A => Some(Self::Localx2APICNMI),
            0x0B => Some(Self::GICCPUInterface),
            0x0C => Some(Self::GICDistributor),
            0x0D => Some(Self::GICMSIFrame),
            0x0E => Some(Self::GICRedistributor),
            0x0F => Some(Self::GICInterruptTranslationService),
            0x10 => Some(Self::MultiprocessorWakeup),
            0x11 => Some(Self::CoreProgrammableInterruptController),
            0x12 => Some(Self::LegacyIOProgrammableInterruptController),
            0x13 => Some(Self::HyperTransportProgrammableInterruptController),
            0x14 => Some(Self::ExtendIOProgrammableInterruptController),
            0x15 => Some(Self::MSIProgrammableInterruptController),
            0x16 => Some(Self::BridgeIOProgrammableInterruptController),
            0x17 => Some(Self::LPCProgrammableInterruptController),
            0x18 => Some(Self::RISCVHartLocalInterruptController),
            0x19 => Some(Self::RISCVIncomingMSIController),
            0x1A => Some(Self::RISCVAdvancedPlatformLevelInterruptController),
            0x1B => Some(Self::RISCVPlatformLevelInterruptController),

            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TypeLength {
    r#type: u8,
    length: u8,
}
impl TypeLength {
    pub fn raw_type(&self) -> u8 {
        self.r#type
    }
    /// Returns None if value is reserved for OEM use.
    pub fn r#type(&self) -> Option<ICSTypes> {
        ICSTypes::from(self.r#type)
    }
    pub fn length(&self) -> u8 {
        self.length
    }
}

#[derive(Copy, Clone)]
/// ## Local (S)APIC Flags
pub struct LocalAPICFlags(u32);
impl LocalAPICFlags {
    /// If this bit is set the processor is ready for use. If this bit is clear and the Online Capable bit is set,
    /// system hardware supports enabling this processor during OS runtime.<br>
    /// If this bit is clear and the Online Capable bit is also clear, this processor is unusable,
    /// and OSPM shall ignore the contents of the given structure.
    pub const fn enabled(&self) -> bool {
        self.0 & 0b01 != 0
    }
    /// The information conveyed by this bit depends on the value of the Enabled bit.
    /// If the Enabled bit is set, this bit is reserved and must be zero.
    /// Otherwise, if this this bit is set, system hardware supports enabling this processor during OS runtime.
    pub const fn online_capable(&self) -> bool {
        self.0 & 0b10 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
/// ## Multiple APIC Flags
pub struct MADTFlags(u32);
impl MADTFlags {
    /// A one indicates that the system also has a PC-AT-compatible dual-8259 setup.
    ///
    /// The 8259 vectors must be disabled (that is, masked) when enabling the ACPI APIC operation.
    pub const fn pcat_compat(&self) -> bool {
        self.0 & 0b1 != 0
    }
    // JJ here, the rest of the bits are reserved; no need to implement.
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Multiple APIC Description Table
///
/// The ACPI interrupt model describes all interrupts for the entire system in a uniform interrupt model implementation. Supported interrupt models include:
///
/// - The PC-AT-compatible dual 8259 interrupt controller.
/// - **Intel processor-based systems** - The Intel Advanced Programmable Interrupt Controller (APIC) and Intel Streamlined Advanced Programmable Interrupt.
/// - **ARM processor-based systems** - The Generic Interrupt Controller (GIC).
/// - **LoongArch processor-based systems** - the LoongArch Programmable Interrupt Controller (LPIC).
///
/// The choice of interrupt model(s) to support is up to the platform designer.
/// The interrupt model cannot be dynamically changed by system firmware; OSPM will choose which model to use and install support for that model at the time of installation.
/// If a platform supports multiple models, an OS will install support for only one of the models and will not mix models.
/// Multi-boot capability is a feature in many modern operating systems.
/// This means that a system may have multiple operating systems or multiple instances of an OS installed at any one time. Platform designers must allow for this.
///
/// This provides OSPM with information necessary for operation on systems with APIC, SAPIC, GIC, or LPIC implementations.
///
/// ACPI represents all interrupts as "flat" values known as global system interrupts.
/// Therefore to support APICs, SAPICs, GICs, or LPICs on an ACPI-enabled system, each used interrupt input must be mapped to the global system interrupt value used by ACPI. See Global System Interrupts for more details.
///
/// Additional support is required to handle various multi-processor functions that implementations might support (for example, identifying each processor's local interrupt controller ID).
///
/// All addresses in the MADT are processor-relative physical addresses.
///
/// Starting with ACPI Specification 6.3, the use of the Processor() object was deprecated.
/// Only legacy systems should continue with this usage. On the Itanium architecture only, a _UID is provided for the Processor() that is a string object.
/// This usage of _UID is also deprecated since it can preclude an OSPM from being able to match a processor to a non-enumerable device, such as those defined in the MADT.
/// From ACPI Specification 6.3 onward, all processor objects for all architectures except Itanium must now use Device() objects with an _HID of ACPI0007, and use only integer _UID values.
pub struct MultipleAPICDescriptionTable {
    /// - **Signature** - "APIC"
    pub header: SDTHeader,
    /// The 32-bit physical address at which each processor can access its local interrupt controller.
    pub local_interrupt_controller_address: u32,
    /// Multiple APIC flags.
    pub flags: MADTFlags,
}
impl MultipleAPICDescriptionTable {
    /// A list of interrupt controller structures for this implementation.
    ///
    /// This list will contain all of the structures from Interrupt Controller Structure Types needed to support this platform.
    ///
    /// **JJ's Note: I'm not sure what the best way is to implement this.  I've decided for right now to just have this return the buffer reference containing the structures.**
    pub const fn interrupt_controller_structure(&self) -> &[u8] {
        // SAFETY: I sure hope the OEM doesn't frick things up...
        unsafe {
            core::slice::from_raw_parts(
                (self as *const _ as *const u8).add(size_of::<Self>()),
                (self.header.length as usize - size_of::<Self>()) / 8,
            )
        }
    }
}
