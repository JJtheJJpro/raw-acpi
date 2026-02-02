use crate::SDTHeader;

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// ## Boot Graphics Resource Table (BGRT)
/// 
/// The Boot Graphics Resource Table (BGRT) is an optional table that provides a mechanism
/// to indicate that an image was drawn on the screen during boot, and some information about the image.
/// 
/// The table is written when the image is drawn on the screen. This should be done after it is expected that any firmware components
/// that may write to the screen are done doing so and it is known that the image is the only thing on the screen.
/// If the boot path is interrupted (e.g., by a key press), the Displayed bit within the status field should be changed to 0
/// to indicate to the OS that the current image is invalidated.
/// 
/// This table is only supported on UEFI systems.
pub struct BootGraphicsResourceTable {
    /// - **Signature** - "BGRT"
    pub header: SDTHeader,
    /// 2-bytes (16 bit) version ID. This value must be 1.
    pub version: u16,
    /// - **Bits [[7:3]]** - Reserved
    /// - **Bits [[2:1]]** - Orientation Offset. These bits describe the clockwise degree offset from the image's default orientation.
    ///   - **0b00** - 0, no offset
    ///   - **0b01** - 90
    ///   - **0b10** - 180
    ///   - **0b11** - 270
    /// - **Bit [[0]]** - Displayed. A one indicates the boot image graphic is displayed.
    pub status: u8,
    /// - **0x00** - Bitmap
    /// 
    /// The rest of the values are reserved.
    pub image_type: u8,
    /// 8-byte (64 bit) physical address pointing to the firmware’s in-memory copy of the image bitmap.
    pub image_address: u64,
    /// A 4-byte (32-bit) unsigned long describing the display X-offset of the boot image.
    /// 
    /// (X, Y) display offset of the top left corner of the boot image. The top left corner of the display is at offset (0, 0).
    pub image_offset_x: u32,
    /// A 4-byte (32-bit) unsigned long describing the display Y-offset of the boot image.
    /// 
    /// (X, Y) display offset of the top left corner of the boot image. The top left corner of the display is at offset (0, 0).
    pub image_offset_y: u32,
}