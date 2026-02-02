use crate::{rsdt::RootSystemDescriptionTable, xsdt::ExtendedSystemDescriptionTable};

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Root System Description Pointer structure.
///
/// During OS initialization, OSPM must obtain the Root System Description Pointer (RSDP) structure from the platform.
/// When OSPM locates the Root System Description Pointer (RSDP) structure,
/// it then locates the Root System Description Table (RSDT) or the Extended Root System Description Table (XSDT) using the physical system address supplied in the RSDP.
///
/// ## Finding the RSDP on IA-PC Systems
///
/// OSPM finds the Root System Description Pointer (RSDP) structure by searching physical memory ranges on 16-byte boundaries for a valid Root System Description Pointer structure signature and checksum match as follows:
/// - The first 1 KB of the Extended BIOS Data Area (EBDA). For EISA or MCA systems, the EBDA can be found in the two-byte location 40:0Eh on the BIOS data area.
/// - The BIOS read-only memory space between 0E0000h and 0FFFFFh.
///
/// ## Finding the RSDP on UEFI Enabled Systems
///
/// In Unified Extensible Firmware Interface (UEFI) enabled systems, a pointer to the RSDP structure exists within the EFI System Table.
/// The OS loader is provided a pointer to the EFI System Table at invocation.
/// The OS loader must retrieve the pointer to the RSDP structure from the EFI System Table and convey the pointer to OSPM, using an OS dependent data structure, as part of the hand off of control from the OS loader to the OS.
///
/// The OS loader locates the pointer to the RSDP structure by examining the EFI Configuration Table within the EFI System Table.
/// EFI Configuration Table entries consist of Globally Unique Identifier (GUID)/table pointer pairs.
/// The UEFI specification defines two GUIDs for ACPI; one for ACPI 1.0 and the other for ACPI 2.0 or later specification revisions.
///
/// The EFI GUID for a pointer to the ACPI 1.0 specification RSDP structure is:
/// - eb9d2d30-2d88-11d3-9a16-0090273fc14d
///
/// The EFI GUID for a pointer to the ACPI 2.0 or later specification RSDP structure is:
/// - 8868e871-e4f1-11d3-bc22-0080c73c8881
///
/// The OS loader for an ACPI-compatible OS will search for an RSDP structure pointer (RSDP Structure) using the current revision GUID first and if it finds one, will use the corresponding RSDP structure pointer.
/// If the GUID is not found then the OS loader will search for the RSDP structure pointer using the ACPI 1.0 version GUID.
///
/// The OS loader must retrieve the pointer to the RSDP structure from the EFI System Table before assuming platform control via the EFI ExitBootServices interface. See the UEFI Specification for more information.
pub struct RootSystemDescriptionPointer {
    /// "RSD PTR "<br>
    /// Notice that this signature must contain a trailing blank character.
    pub signature: [u8; 8],
    /// This is the checksum of the fields defined in the ACPI 1.0 specification.<br>
    /// This includes only the first 20 bytes of this table, bytes 0 to 19, including the checksum field.<br>
    /// These bytes must sum to zero.
    pub checksum: u8,
    /// An OEM-supplied string that identifies the OEM.
    pub oemid: [u8; 6],
    /// The revision of this structure.
    ///
    /// Larger revision numbers are backward compatible to lower revision numbers.<br>
    /// The ACPI version 1.0 revision number of this table is zero.
    /// The ACPI version 1.0 RSDP Structure only includes the first 20 bytes of this table, bytes 0 to 19.
    /// It does not include the Length field and beyond.
    pub revision: u8,
    /// 32 bit physical address of the RSDT.
    ///
    /// **JJ's note: Assume undefined behavior if revision is 2.0+.**
    pub rsdt_address: u32,

    /// The length of the table, in bytes, including the header, starting from offset 0x0.
    /// This field is used to record the size of the entire table.
    ///
    /// **This field is only available in the ACPI version 2.0+ RSDP Structure.**
    pub length: u32,
    /// 64 bit physical address of the XSDT.
    ///
    /// **This field is only available in the ACPI version 2.0+ RSDP Structure.**
    pub xsdt_address: u64,
    /// This is a checksum of the entire table, including both checksum fields.
    ///
    /// **This field is only available in the ACPI version 2.0+ RSDP Structure.**
    pub extended_checksum: u8,
    /// Reseved field.
    ///
    /// **This field is only available in the ACPI version 2.0+ RSDP Structure.**
    reserved: [u8; 3],
}
impl RootSystemDescriptionPointer {
    /// Returns true if the rsdt or xsdt address matches the signature for RSDT or XSDT, more for simplicity than anything else.
    pub const fn validate_sdt_signature(&self) -> bool {
        unsafe {
            if self.revision < 2 {
                (&*(self.rsdt_address as usize as *const RootSystemDescriptionTable))
                    .header
                    .signature[0]
                    == b'R'
                    && (&*(self.rsdt_address as usize as *const RootSystemDescriptionTable))
                        .header
                        .signature[1]
                        == b'S'
                    && (&*(self.rsdt_address as usize as *const RootSystemDescriptionTable))
                        .header
                        .signature[2]
                        == b'D'
                    && (&*(self.rsdt_address as usize as *const RootSystemDescriptionTable))
                        .header
                        .signature[3]
                        == b'T'
            } else {
                (&*(self.xsdt_address as usize as *const ExtendedSystemDescriptionTable))
                    .header
                    .signature[0]
                    == b'X'
                    && (&*(self.xsdt_address as usize as *const ExtendedSystemDescriptionTable))
                        .header
                        .signature[1]
                        == b'S'
                    && (&*(self.xsdt_address as usize as *const ExtendedSystemDescriptionTable))
                        .header
                        .signature[2]
                        == b'D'
                    && (&*(self.xsdt_address as usize as *const ExtendedSystemDescriptionTable))
                        .header
                        .signature[3]
                        == b'T'
            }
        }
    }
}
