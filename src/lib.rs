//! # ACPI System Description Tables
//!
//! This rust library defines all the ACPI-defined System Description Tables.  Such tables and more information can be found
//! <a href="https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#acpi-system-description-tables">here</a>.

#![no_std]

pub mod bgrt;
pub mod cpep;
pub mod dsdt;
pub mod ecdt;
pub mod facs;
pub mod fadt;
//pub mod hpet; // JJ here, this is a reserved signature from acpi.
pub mod madt;
//pub mod mcfg; // JJ here, this is a reserved signature from acpi.
pub mod msct;
pub mod pcct;
pub mod psdt;
pub mod rasf;
pub mod rsdp;
pub mod rsdt;
pub mod sbst;
pub mod slit;
//pub mod spcr; // JJ here, this is a reserved signature from acpi.
pub mod srat;
pub mod ssdt;
pub mod xsdt;

#[derive(Clone, Copy)]
#[repr(C, packed)]
/// The Generic Address Structure (GAS) provides the platform with a robust means to describe register locations.
pub struct GenericAddressStructure {
    /// The address space where the data structure or register exists. Defined values are:
    /// - 0x00 - System Memory space
    /// - 0x01 - System I/O space
    /// - 0x02 - PCI Configuration space
    /// - 0x03 - Embedded Controller
    /// - 0x04 - SMBus
    /// - 0x05 - SystemCMOS
    /// - 0x06 - PciBarTarget
    /// - 0x07 - IPMI
    /// - 0x08 - General PurposeIO
    /// - 0x09 - GenericSerialBus
    /// - 0x0A - Platform Communications Channel (PCC)
    /// - 0x0B - Platform Runtime Mechanism (PRM)
    /// - 0x0C to 0x7E - Reserved
    /// - 0x7F - Functional Fixed Hardware
    /// - 0x80 to 0xFF - OEM Defined
    pub address_space_id: u8,
    /// The size in bits of the given register. When addressing a data structure, this field must be zero.
    pub reg_bit_width: u8,
    /// The bit offset of the given register at the given address. When addressing a data structure, this field must be zero.
    pub reg_bit_offset: u8,
    /// Specifies access size. Unless otherwise defined by the Address Space ID:
    /// - 0 - Undefined (legacy reasons)
    /// - 1 - Byte access
    /// - 2 - Word access
    /// - 3 - Dword access
    /// - 4 - QWord access
    pub access_size: u8,
    /// The 64-bit address of the data structure or register in the given address space (relative to the processor).
    pub address: u64,
}

pub const SDT_HEADER_SIZE: usize = core::mem::size_of::<SDTHeader>();

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## System Description Table Header structure.
///
/// All system description tables begin with the structure shown in the SDTHeader Fields.
/// The signature field in this table determines the content of the system description table.
pub struct SDTHeader {
    /// The ASCII string representation of the table identifier. Note that if OSPM finds a signature in a table that is not listed in
    /// <a href="https://uefi.org/specs/ACPI/6.5/05_ACPI_Software_Programming_Model.html#description-header-signatures-for-tables-defined-by-acpi">this table</a>, then OSPM ignores the entire table (it is not loaded into ACPI namespace);
    /// OSPM ignores the table even though the values in the Length and Checksum fields are correct.
    pub signature: [u8; 4],
    /// The length of the table, in bytes, including the header, starting from offset 0. This field is used to record the size of the entire table.
    pub length: u32,
    /// The revision of the structure corresponding to the signature field for this table.<br>
    /// Larger revision numbers are backward compatible to lower revision numbers with the same signature.
    pub revision: u8,
    /// The entire table, including the checksum field, must add to zero to be considered valid.
    pub checksum: u8,
    /// An OEM-supplied string that identifies the OEM.
    pub oemid: [u8; 6],
    /// An OEM-supplied string that the OEM uses to identify the particular data table.<br>
    /// This field is particularly useful when defining a definition block to distinguish definition block functions.
    /// The OEM assigns each dissimilar structure a new OEM Table ID.
    pub oem_table_id: [u8; 8],
    /// An OEM-supplied revision number. Larger numbers are assumed to be newer revisions.
    pub oem_revision: u32,
    /// Vendor ID of utility that created the table. For tables containing Definition Blocks, this is the ID for the ASL Compiler.
    pub creator_id: u32,
    /// Revision of utility that created the table. For tables containing Definition Blocks, this is the revision for the ASL Compiler.
    pub creator_revision: u32,
}
