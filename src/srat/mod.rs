pub mod affinity;

use crate::SDTHeader;

#[derive(Copy, Clone)]
pub enum AffinityTypes {
    ProcessorLocalAPIC,
    Memory,
    ProcessorLocalx2APIC,
    GICC,
    GICInterruptTranslationService,
    GenericInitiator,
    GenericPort,
    RINTC,
}
impl AffinityTypes {
    pub fn from(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Self::ProcessorLocalAPIC),
            0x01 => Some(Self::Memory),
            0x02 => Some(Self::ProcessorLocalx2APIC),
            0x03 => Some(Self::GICC),
            0x04 => Some(Self::GICInterruptTranslationService),
            0x05 => Some(Self::GenericInitiator),
            0x06 => Some(Self::GenericPort),
            0x07 => Some(Self::RINTC),

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
    pub fn r#type(&self) -> Option<AffinityTypes> {
        AffinityTypes::from(self.r#type)
    }
    pub fn length(&self) -> u8 {
        self.length
    }
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Device Handle - ACPI
pub struct ACPIDeviceHandle {
    /// The _HID value
    pub acpi_hid: u64,
    /// The _UID value
    pub acpi_uid: u32,
    reserved: u32,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Device Handle - PCI
pub struct PCIDeviceHandle {
    /// PCI segment number.
    ///
    /// For systems with fewer than 255 PCI buses, this number must be 0.
    pub pci_segment: u16,
    /// PCI Bus Number (Bits 7:0 of Byte 2)<br>
    /// PCI Device Number (Bits 7:3 of Byte 3)<br>
    /// PCI Function Number (Bits 2:0 of Byte 3)
    pub pci_bdf_number: u16,
    reserved: [u8; 12],
}

#[derive(Copy, Clone)]
/// ## Device Handle
pub union DeviceHandle {
    pub acpi: ACPIDeviceHandle,
    pub pci: PCIDeviceHandle,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## System Resource Affinity Table (SRAT)
///
/// This optional table provides information that allows OSPM to associate the following types of devices with system locality / proximity domains and clock domains:
///
/// - Processors
/// - Memory ranges (including those provided by hot-added memory devices)
/// - Generic initiators (e.g. heterogeneous processors and accelerators, GPUs, and I/O devices with integrated compute or DMA engines)
/// - Generic ports (e.g. host bridges that can dynamically discover new initiators and instantiate new memory range targets)
///
/// On NUMA platforms, SRAT information enables OSPM to optimally configure the operating system
/// during a point in OS initialization when evaluation of objects in the ACPI Namespace is not yet possible.
///
/// OSPM evaluates the SRAT only during OS initialization.
/// The Local APIC ID / Local SAPIC ID / Local x2APIC ID or the GICC ACPI Processor UID of all processors started at boot time must be present in the SRAT.
/// If the Local APIC ID / Local SAPIC ID / Local x2APIC ID or the GICC ACPI Processor UID of a dynamically added processor is not present in the SRAT,
/// a _PXM object must exist for the processor’s device or one of its ancestors in the ACPI Namespace.
///
/// Note: SRAT is the place where proximity domains are defined, and _PXM provides a mechanism to associate a device object (and its children) to an SRAT-defined proximity domain.
pub struct SystemResourceAffinityTable {
    /// - **Signature** - "SRAT"
    pub header: SDTHeader,
    /// **JJ's Note: History indicates that these were gonna be used for describing NUMA topology, but NUMA turned out to be static, making these fields useless.
    /// The System Locality Information Table (SLIT) was introduced not too long after SRAT, which removed the need for global SRAT-level metadata and runtime interpretation hints.**
    ///
    /// **From that point on, firmware vendors decided never to implement it.**
    ///
    /// **The ACPI spec in uefi.org does state that the 32-bit field is reserved to be 1 for backwards compatibility.**
    reserved0: u32,
    reserved1: u64,
}
impl SystemResourceAffinityTable {
    /// A list of static resource allocation structures for the platform.
    ///
    /// **JJ's Note: I'm not sure what the best way is to implement this.  I've decided for right now to just have this return the buffer reference containing the structures.**
    pub const fn static_resource_allocation_structure(&self) -> &[u8] {
        // SAFETY: I sure hope the OEM doesn't frick things up...
        unsafe {
            core::slice::from_raw_parts(
                (self as *const _ as *const u8).add(size_of::<Self>()),
                (self.header.length as usize - size_of::<Self>()) / 8,
            )
        }
    }
}
