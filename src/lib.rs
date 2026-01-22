#![no_std]

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct GenericAddressStructure {
    pub address_space_id: u8,
    pub reg_bit_width: u8,
    pub reg_bit_offset: u8,
    pub access_size: u8,
    pub address: u64,
}

/// The Root System Description Pointer structure.<br>
#[repr(C, packed)]
pub struct RSDP {
    /// "RSD PTR "<br>
    /// Notice that this signature must contain a trailing blank character.
    pub signature: [u8; 8],
    /// This is the checksum of the fields defined in the ACPI 1.0 specification.<br>
    /// This includes only the first 20 bytes of this table, bytes 0 to 19, including the checksum field.<br>
    /// These bytes must sum to zero.
    pub checksum: u8,
    /// An OEM-supplied string that identifies the OEM.
    pub oemid: [u8; 6],
    /// The revision of this structure.<br>
    /// Larger revision numbers are backward compatible to lower revision numbers.<br>
    /// The ACPI version 1.0 revision number of this table is zero.
    /// The ACPI version 1.0 RSDP Structure only includes the first 20 bytes of this table, bytes 0 to 19.
    /// It does not include the Length field and beyond.
    ///
    /// **JJ's note: The UEFI docs continue to say "The current value for this field is 2."  However, since I'd like to expand this OS to run on more devices than usual, value 1 is possible,
    /// in which case, the fields of this struct after rsdt_address are undefined behavior.**
    pub revision: u8,
    /// 32 bit physical address of the RSDT.
    ///
    /// **JJ's note: Assume undefined behavior if revision is 2+.**
    pub rsdt_address: u32,

    /// The length of the table, in bytes, including the header, starting from offset 0x0.
    /// This field is used to record the size of the entire table.
    ///
    /// **This field is not available in the ACPI version 1.0 RSDP Structure.**
    pub length: u32,
    /// 64 bit physical address of the XSDT.
    ///
    /// **This field is not available in the ACPI version 1.0 RSDP Structure.**
    pub xsdt_address: u64,
    /// This is a checksum of the entire table, including both checksum fields.
    ///
    /// **This field is not available in the ACPI version 1.0 RSDP Structure.**
    pub extended_checksum: u8,
    /// Reseved field.
    reserved: [u8; 3],
}
impl RSDP {
    /// Returns true if the rsdt or xsdt address matches the signature for RSDT or XSDT, more for simplicity than anything else.
    pub const fn validate_sdt_signature(&self) -> bool {
        unsafe {
            if self.revision < 2 {
                (&*(self.rsdt_address as usize as *const RSDT))
                    .header
                    .signature[0]
                    == b'R'
                    && (&*(self.rsdt_address as usize as *const RSDT))
                        .header
                        .signature[1]
                        == b'S'
                    && (&*(self.rsdt_address as usize as *const RSDT))
                        .header
                        .signature[2]
                        == b'D'
                    && (&*(self.rsdt_address as usize as *const RSDT))
                        .header
                        .signature[3]
                        == b'T'
            } else {
                (&*(self.xsdt_address as usize as *const XSDT))
                    .header
                    .signature[0]
                    == b'X'
                    && (&*(self.xsdt_address as usize as *const XSDT))
                        .header
                        .signature[1]
                        == b'S'
                    && (&*(self.xsdt_address as usize as *const XSDT))
                        .header
                        .signature[2]
                        == b'D'
                    && (&*(self.xsdt_address as usize as *const XSDT))
                        .header
                        .signature[3]
                        == b'T'
            }
        }
    }
}

/// System Description Table Header structure.
///
/// All system description tables begin with the structure shown in the SDTHeader Fields.
/// The signature field in this table determines the content of the system description table.
#[repr(C)]
pub struct SDTHeader {
    /// The ASCII string representation of the table identifier.
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
    /// The OEM assigns each dissimilar table a new OEM Table ID.
    pub oem_table_id: [u8; 8],
    /// An OEM-supplied revision number. Larger numbers are almost always newer revisions.
    pub oem_revision: u32,
    /// Vendor ID of utility that created the table. For tables containing Definition Blocks, this is the ID for the ASL Compiler.
    pub creator_id: u32,
    /// Revision of utility that created the table. For tables containing Definition Blocks, this is the revision for the ASL Compiler.
    pub creator_revision: u32,
}

/// The Root System Description Table structure.
pub struct RSDT {
    pub header: SDTHeader,
    /// An array of 32-bit physical addresses that point to other System Description Tables.
    pub entry: *const u32,
}

/// The Extended System Description Table structure.
pub struct XSDT {
    pub header: SDTHeader,
    /// An array of 64-bit physical addresses that point to other System Description Tables.
    pub entry: *const u64,
}

pub enum FADTPersistentCPUCacheFeature {
    NotReported,
    NotPersistent,
    Persistent,
}

pub struct FADTFeatureFlags(u32);
impl FADTFeatureFlags {
    pub const fn wbinvd(&self) -> bool {
        self.0 & 0b00000000000000000000000000000001 != 0
    }
    pub const fn wbinvd_flush(&self) -> bool {
        self.0 & 0b00000000000000000000000000000010 != 0
    }
    pub const fn proc_c1(&self) -> bool {
        self.0 & 0b00000000000000000000000000000100 != 0
    }
    pub const fn p_lvl2_up(&self) -> bool {
        self.0 & 0b00000000000000000000000000001000 != 0
    }
    pub const fn pwr_button(&self) -> bool {
        self.0 & 0b00000000000000000000000000010000 != 0
    }
    pub const fn slp_button(&self) -> bool {
        self.0 & 0b00000000000000000000000000100000 != 0
    }
    pub const fn fix_rtc(&self) -> bool {
        self.0 & 0b00000000000000000000000001000000 != 0
    }
    pub const fn rtc_s4(&self) -> bool {
        self.0 & 0b00000000000000000000000010000000 != 0
    }
    pub const fn tmr_val_ext(&self) -> bool {
        self.0 & 0b00000000000000000000000100000000 != 0
    }
    pub const fn dck_cap(&self) -> bool {
        self.0 & 0b00000000000000000000001000000000 != 0
    }
    pub const fn reset_reg_sup(&self) -> bool {
        self.0 & 0b00000000000000000000010000000000 != 0
    }
    pub const fn sealed_case(&self) -> bool {
        self.0 & 0b00000000000000000000100000000000 != 0
    }
    pub const fn headless(&self) -> bool {
        self.0 & 0b00000000000000000001000000000000 != 0
    }
    pub const fn cpu_sw_slp(&self) -> bool {
        self.0 & 0b00000000000000000010000000000000 != 0
    }
    pub const fn pci_exp_wak(&self) -> bool {
        self.0 & 0b00000000000000000100000000000000 != 0
    }
    pub const fn use_platform_clock(&self) -> bool {
        self.0 & 0b00000000000000001000000000000000 != 0
    }
    pub const fn s4_rtc_sts_valid(&self) -> bool {
        self.0 & 0b00000000000000010000000000000000 != 0
    }
    pub const fn remote_power_on_capable(&self) -> bool {
        self.0 & 0b00000000000000100000000000000000 != 0
    }
    pub const fn force_apic_cluster_model(&self) -> bool {
        self.0 & 0b00000000000001000000000000000000 != 0
    }
    pub const fn force_apic_physical_destination_mode(&self) -> bool {
        self.0 & 0b00000000000010000000000000000000 != 0
    }
    pub const fn hw_reduced_acpi(&self) -> bool {
        self.0 & 0b00000000000100000000000000000000 != 0
    }
    pub const fn low_power_s0_idle_capable(&self) -> bool {
        self.0 & 0b00000000001000000000000000000000 != 0
    }
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
    // The rest of the bits are reserved; no need to implement.
}

#[repr(C, packed)]
pub struct FADT {
    pub header: SDTHeader, // Note, the signature is "FACP" and revision is FADT's Major Version.

    pub firmware_ctrl: u32,
    pub dsdt: u32,
    pub int_model: u8,
    pub preferred_pm_profile: u8,

    pub sci_int: u16,
    pub smi_cmd: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub s4bios_req: u8,
    pub pstate_cnt: u8,

    pub pm1a_evt_blk: u32,
    pub pm1b_evt_blk: u32,
    pub pm1a_cnt_blk: u32,
    pub pm1b_cnt_blk: u32,
    pub pm2_cnt_blk: u32,
    pub pm_tmr_blk: u32,
    pub gpe0_blk: u32,
    pub gpe1_blk: u32,

    pub pm1_evt_len: u8,
    pub pm1_cnt_len: u8,
    pub pm2_cnt_len: u8,
    pub pm_tmr_len: u8,
    pub gpe0_blk_len: u8,
    pub gpe1_blk_len: u8,
    pub gpe1_base: u8,
    pub cst_cnt: u8,

    pub p_lvl2_lat: u16,
    pub p_lvl3_lat: u16,
    pub flush_size: u16,
    pub flush_stride: u16,

    pub duty_offset: u8,
    pub duty_width: u8,
    pub day_alrm: u8,
    pub mon_alrm: u8,
    pub century: u8,

    pub iapc_boot_arch: u16,
    reserved: u8,
    pub flags: FADTFeatureFlags,
    pub reset_reg: GenericAddressStructure,
    pub reset_value: u8,
    pub arm_boot_arch: u16,
    pub minor_version: u8,

    pub x_firmware_ctrl: u64,
    pub x_dsdt: u64,
    pub x_pm1a_evt_blk: GenericAddressStructure,
    pub x_pm1b_evt_blk: GenericAddressStructure,
    pub x_pm1a_cnt_blk: GenericAddressStructure,
    pub x_pm1b_cnt_blk: GenericAddressStructure,
    pub x_pm2_cnt_blk: GenericAddressStructure,
    pub x_pm_tmr_blk: GenericAddressStructure,
    pub x_gpe0_blk: GenericAddressStructure,
    pub x_gpe1_blk: GenericAddressStructure,

    pub sleep_control_reg: GenericAddressStructure,
    pub sleep_status_reg: GenericAddressStructure,

    pub hypervisor_vendor_identity: u64,
}
