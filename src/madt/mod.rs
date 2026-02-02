pub mod interrupt_source_override;
pub mod ioapic;
pub mod local_apic_nmi;
pub mod nmi_source;
pub mod proc_local_apic;
pub mod bridge_io_pic;
pub mod core_pic;
pub mod extend_io_pic;
pub mod gic_distributor;
pub mod gic_interrupt_translation_service;
pub mod gic_msi_frame;
pub mod gic_redistributor;
pub mod giccpu_interface;
pub mod hyper_transport_pic;
pub mod iosapic;
pub mod legacy_io_pic;
pub mod local_api_address_override;
pub mod local_sapic;
pub mod local_x2apic_nmi;
pub mod lpc_pic;
pub mod msi_pic;
pub mod multiprocessor_wakeup;
pub mod platform_interrupt_source;
pub mod processor_local_x2apic;

use crate::SDTHeader;

#[derive(Copy, Clone)]
/// ## Local (S)APIC Flags
pub struct LocalAPICFlags(u32);
impl LocalAPICFlags {
    /// If this bit is set the processor is ready for use. If this bit is clear and the Online Capable bit is set,
    /// system hardware supports enabling this processor during OS runtime.<br>
    /// If this bit is clear and the Online Capable bit is also clear, this processor is unusable,
    /// and OSPM shall ignore the contents of the Processor Local APIC Structure.
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
pub struct MADT {
    /// - **Signature** - "APIC"
    pub header: SDTHeader,
    /// The 32-bit physical address at which each processor can access its local interrupt controller.
    pub local_interrupt_controller_address: u32,
    /// Multiple APIC flags.
    pub flags: MADTFlags,
    /// A list of interrupt controller structures for this implementation.
    ///
    /// This list will contain all of the structures from Interrupt Controller Structure Types needed to support this platform. These structures are described in the following sections.
    pub interrupt_controller_structure: [u8; 0],
}
